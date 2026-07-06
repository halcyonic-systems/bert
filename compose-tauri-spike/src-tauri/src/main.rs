#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Spike: does a web canvas render a live conservation sim *well* when the sim
//! runs in Rust and streams aggregate state over Tauri events?
//!
//! The real bert-compose ticks at ~4/sec (a discrete conservation circuit), so
//! per-tick IPC is trivially cheap — the open question is whether the *rendering*
//! feels as alive as egui's immediate-mode canvas. This emits one `tick` event
//! per simulation step; the front-end interpolates between ticks at 60fps. The
//! tick-rate is adjustable at runtime so we can push it until something breaks.
//!
//! Layout (node positions) lives in the FRONT-END, not here — deliberately, to
//! show the engine emitting pure state (the real circuit.rs wrongly carries
//! egui::Pos2). The toy circuit is self-contained; no bert-compose dependency.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::Serialize;
use tauri::Emitter;

/// One simulation step's aggregate state — the whole IPC payload.
#[derive(Serialize, Clone)]
struct TickState {
    tick: u32,
    levels: Vec<f32>,
    flows: Vec<f32>,
    emitted: f32,
    delivered: f32,
    stored: f32,
    dissipated: f32,
    balance: f32,
}

/// A tiny tank-and-flow conservation circuit: source → stores → sink.
struct Circuit {
    levels: Vec<f32>,
    wires: Vec<(usize, usize, f32)>, // (from, to, conductance)
    last_flows: Vec<f32>,
    source: usize,
    sink: usize,
    emit_rate: f32,
    tick: u32,
    emitted: f32,
    delivered: f32,
    dissipated: f32,
}

impl Circuit {
    fn new() -> Self {
        // 6 nodes: 0 source, 1/2/4 stores, 3 amplifier(pass), 5 sink.
        let wires = vec![
            (0, 1, 0.50),
            (1, 2, 0.40),
            (1, 3, 0.30),
            (3, 4, 0.50),
            (2, 5, 0.40),
            (4, 5, 0.40),
        ];
        Self {
            levels: vec![0.0; 6],
            last_flows: vec![0.0; wires.len()],
            wires,
            source: 0,
            sink: 5,
            emit_rate: 3.0,
            tick: 0,
            emitted: 0.0,
            delivered: 0.0,
            dissipated: 0.0,
        }
    }

    fn reset(&mut self) {
        self.levels.iter_mut().for_each(|l| *l = 0.0);
        self.last_flows.iter_mut().for_each(|f| *f = 0.0);
        self.tick = 0;
        self.emitted = 0.0;
        self.delivered = 0.0;
        self.dissipated = 0.0;
    }

    /// One discrete conservation step. Sequential per-wire transfer keeps levels
    /// non-negative; a small leak on stores feeds the dissipated ledger.
    fn step(&mut self) {
        self.tick += 1;
        self.levels[self.source] += self.emit_rate;
        self.emitted += self.emit_rate;

        for (i, &(a, b, k)) in self.wires.iter().enumerate() {
            let f = (k * self.levels[a]).min(self.levels[a]);
            self.levels[a] -= f;
            self.levels[b] += f;
            self.last_flows[i] = f;
        }

        // sink delivers everything that reached it
        let drain = self.levels[self.sink];
        self.delivered += drain;
        self.levels[self.sink] = 0.0;

        // 1% leak on interior stores → dissipated (keeps the ledger honest)
        for i in 0..self.levels.len() {
            if i != self.source && i != self.sink {
                let leak = self.levels[i] * 0.01;
                self.levels[i] -= leak;
                self.dissipated += leak;
            }
        }
    }

    fn snapshot(&self) -> TickState {
        let stored: f32 = self.levels.iter().sum();
        let balance = self.emitted - self.delivered - stored - self.dissipated;
        TickState {
            tick: self.tick,
            levels: self.levels.clone(),
            flows: self.last_flows.clone(),
            emitted: self.emitted,
            delivered: self.delivered,
            stored,
            dissipated: self.dissipated,
            balance,
        }
    }
}

/// Shared simulation state — touched by both the tick thread and the commands.
struct Sim {
    circuit: Mutex<Circuit>,
    running: AtomicBool,
    rate: AtomicU32, // ticks per second
}

#[tauri::command]
fn start_sim(sim: tauri::State<'_, Arc<Sim>>) {
    sim.running.store(true, Ordering::Relaxed);
}

#[tauri::command]
fn stop_sim(sim: tauri::State<'_, Arc<Sim>>) {
    sim.running.store(false, Ordering::Relaxed);
}

#[tauri::command]
fn reset_sim(sim: tauri::State<'_, Arc<Sim>>) {
    sim.circuit.lock().expect("circuit lock").reset();
}

#[tauri::command]
fn set_rate(rate: u32, sim: tauri::State<'_, Arc<Sim>>) {
    sim.rate.store(rate.clamp(1, 480), Ordering::Relaxed);
}

fn main() {
    let sim = Arc::new(Sim {
        circuit: Mutex::new(Circuit::new()),
        running: AtomicBool::new(false),
        rate: AtomicU32::new(4), // matches the real bert-compose default
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(sim.clone())
        .setup(move |app| {
            // One long-lived tick thread. When running, step + emit; always sleeps
            // at the current rate so rate changes take effect immediately.
            let handle = app.handle().clone();
            std::thread::spawn(move || loop {
                let rate = sim.rate.load(Ordering::Relaxed).max(1);
                if sim.running.load(Ordering::Relaxed) {
                    let state = {
                        let mut c = sim.circuit.lock().expect("circuit lock");
                        c.step();
                        c.snapshot()
                    };
                    let _ = handle.emit("tick", state);
                }
                std::thread::sleep(Duration::from_millis((1000 / rate) as u64));
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_sim, stop_sim, reset_sim, set_rate])
        .run(tauri::generate_context!())
        .expect("error while running compose-tauri-spike");
}
