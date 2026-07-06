import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

/** Aggregate per-tick state streamed from Rust (mirror of the Rust TickState). */
interface TickState {
  tick: number;
  levels: number[];
  flows: number[];
  emitted: number;
  delivered: number;
  stored: number;
  dissipated: number;
  balance: number;
}

// Layout lives in the FRONT-END (the engine emits pure state). Must match the
// Rust circuit's node count and wire order.
const NODES = [
  { x: 80, y: 230, label: "Source", kind: "source" },
  { x: 250, y: 230, label: "Store A", kind: "store" },
  { x: 470, y: 120, label: "Store B", kind: "store" },
  { x: 470, y: 340, label: "Amp", kind: "amp" },
  { x: 660, y: 340, label: "Store C", kind: "store" },
  { x: 850, y: 230, label: "Sink", kind: "sink" },
] as const;
const WIRES: [number, number][] = [
  [0, 1],
  [1, 2],
  [1, 3],
  [3, 4],
  [2, 5],
  [4, 5],
];
const W = 930;
const H = 460;

const C = {
  stage: "#0c1322",
  grid: "#16213c",
  wire: "#2b3b5c",
  flow: "#5eead4",
  store: "#2dd4bf",
  source: "#818cf8",
  sink: "#34d399",
  amp: "#fbbf24",
  ring: "#2b3b5c",
  text: "#9fb2d0",
};

const lerp = (a: number, b: number, t: number) => a + (b - a) * t;
const tankRadius = (level: number) => 9 + 26 * (1 - Math.exp(-level / 16));

export default function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const tick = useRef<{ prev: TickState | null; curr: TickState | null; at: number; interval: number }>({
    prev: null,
    curr: null,
    at: 0,
    interval: 250,
  });
  const frames = useRef(0);
  const events = useRef(0);

  const [running, setRunning] = useState(false);
  const [rate, setRate] = useState(4);
  const [hud, setHud] = useState({ fps: 0, eps: 0, tick: 0, balance: 0, emitted: 0, delivered: 0, stored: 0, dissipated: 0 });

  // Stream of tick events from Rust.
  useEffect(() => {
    let un: (() => void) | undefined;
    listen<TickState>("tick", (e) => {
      const r = tick.current;
      r.prev = r.curr ?? e.payload;
      r.curr = e.payload;
      r.at = performance.now();
      events.current++;
    }).then((u) => (un = u));
    return () => un?.();
  }, []);

  // 60fps render loop — interpolates between the two most recent ticks.
  useEffect(() => {
    const cv = canvasRef.current!;
    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    cv.width = W * dpr;
    cv.height = H * dpr;
    const ctx = cv.getContext("2d")!;
    ctx.scale(dpr, dpr);

    let raf = 0;
    const draw = () => {
      frames.current++;
      render(ctx);
      raf = requestAnimationFrame(draw);
    };
    raf = requestAnimationFrame(draw);
    return () => cancelAnimationFrame(raf);
  }, []);

  // HUD sampling (decoupled from the canvas so React never re-renders per frame).
  useEffect(() => {
    let last = performance.now();
    const id = setInterval(() => {
      const now = performance.now();
      const dt = (now - last) / 1000;
      last = now;
      const fps = Math.round(frames.current / dt);
      const eps = Math.round(events.current / dt);
      frames.current = 0;
      events.current = 0;
      const c = tick.current.curr;
      setHud({
        fps,
        eps,
        tick: c?.tick ?? 0,
        balance: c?.balance ?? 0,
        emitted: c?.emitted ?? 0,
        delivered: c?.delivered ?? 0,
        stored: c?.stored ?? 0,
        dissipated: c?.dissipated ?? 0,
      });
    }, 500);
    return () => clearInterval(id);
  }, []);

  function render(ctx: CanvasRenderingContext2D) {
    const now = performance.now();
    const r = tick.current;
    const a = r.prev ?? r.curr;
    const b = r.curr;
    const t = b && a !== b ? Math.min((now - r.at) / r.interval, 1) : 0;

    const levels = b ? (a ? a.levels.map((v, i) => lerp(v, b.levels[i], t)) : b.levels) : NODES.map(() => 0);
    const flows = b ? b.flows : WIRES.map(() => 0);

    // stage
    ctx.fillStyle = C.stage;
    ctx.fillRect(0, 0, W, H);
    ctx.strokeStyle = C.grid;
    ctx.lineWidth = 1;
    for (let x = 0; x < W; x += 46) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, H);
      ctx.stroke();
    }
    for (let y = 0; y < H; y += 46) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(W, y);
      ctx.stroke();
    }

    // wires + flow particles
    WIRES.forEach(([fa, fb], i) => {
      const A = NODES[fa];
      const B = NODES[fb];
      ctx.strokeStyle = C.wire;
      ctx.lineWidth = 2.5;
      ctx.beginPath();
      ctx.moveTo(A.x, A.y);
      ctx.lineTo(B.x, B.y);
      ctx.stroke();

      const flow = flows[i] ?? 0;
      if (flow > 0.01) {
        const n = 5;
        for (let k = 0; k < n; k++) {
          const p = ((now * 0.00012 * (1 + flow) + k / n) % 1 + 1) % 1;
          const px = lerp(A.x, B.x, p);
          const py = lerp(A.y, B.y, p);
          ctx.beginPath();
          ctx.arc(px, py, 2.6, 0, Math.PI * 2);
          ctx.fillStyle = C.flow;
          ctx.globalAlpha = Math.min(0.25 + flow * 0.5, 0.95);
          ctx.fill();
          ctx.globalAlpha = 1;
        }
      }
    });

    // nodes
    NODES.forEach((node, i) => {
      const col = node.kind === "source" ? C.source : node.kind === "sink" ? C.sink : node.kind === "amp" ? C.amp : C.store;
      const rad = tankRadius(levels[i] ?? 0);
      // glow
      ctx.beginPath();
      ctx.arc(node.x, node.y, rad + 4, 0, Math.PI * 2);
      ctx.fillStyle = col;
      ctx.globalAlpha = 0.14;
      ctx.fill();
      ctx.globalAlpha = 1;
      // fill
      ctx.beginPath();
      ctx.arc(node.x, node.y, rad, 0, Math.PI * 2);
      ctx.fillStyle = col;
      ctx.globalAlpha = 0.9;
      ctx.fill();
      ctx.globalAlpha = 1;
      // ring
      ctx.beginPath();
      ctx.arc(node.x, node.y, 36, 0, Math.PI * 2);
      ctx.strokeStyle = C.ring;
      ctx.lineWidth = 1.5;
      ctx.stroke();
      // label
      ctx.fillStyle = C.text;
      ctx.font = "12px Inter, system-ui, sans-serif";
      ctx.textAlign = "center";
      ctx.fillText(node.label, node.x, node.y + 56);
    });
  }

  const start = () => invoke("start_sim").then(() => setRunning(true));
  const stop = () => invoke("stop_sim").then(() => setRunning(false));
  const reset = () => invoke("reset_sim");
  const changeRate = (v: number) => {
    setRate(v);
    tick.current.interval = 1000 / v;
    void invoke("set_rate", { rate: v });
  };

  const balanced = Math.abs(hud.balance) < 0.01;

  return (
    <div className="mx-auto flex min-h-full max-w-5xl flex-col px-6 py-8">
      <header className="mb-5 flex items-baseline justify-between">
        <div>
          <h1 className="text-2xl font-semibold text-[var(--frost-text)]" style={{ fontFamily: "var(--font-display)" }}>
            bert-compose · canvas spike
          </h1>
          <p className="mt-1 text-[13px] text-[var(--frost-text-muted)]">
            Rust conservation sim → Tauri tick events → web canvas, interpolated at 60fps
          </p>
        </div>
        <div className="flex gap-4 text-right">
          <Metric label="render" value={`${hud.fps} fps`} good={hud.fps >= 55} />
          <Metric label="events" value={`${hud.eps}/s`} good />
          <Metric label="tick" value={`${hud.tick}`} good />
        </div>
      </header>

      <div className="overflow-hidden rounded-[var(--radius)] border border-[var(--frost-border)]" style={{ background: C.stage }}>
        <canvas ref={canvasRef} style={{ width: W, height: H, display: "block", margin: "0 auto" }} />
      </div>

      {/* Controls */}
      <div className="mt-5 flex flex-wrap items-center gap-4 rounded-[var(--radius)] border border-[var(--frost-border)] bg-[var(--frost-surface)] p-4">
        <button
          onClick={running ? stop : start}
          className="rounded-[var(--radius-sm)] bg-[var(--frost-teal)] px-4 py-2 text-[13px] font-semibold text-[var(--frost-on-accent)] transition hover:brightness-110"
        >
          {running ? "Pause" : "Run"}
        </button>
        <button
          onClick={reset}
          className="rounded-[var(--radius-sm)] border border-[var(--frost-border)] px-4 py-2 text-[13px] font-medium text-[var(--frost-text-muted)] transition hover:text-[var(--frost-text)]"
        >
          Reset
        </button>
        <label className="flex flex-1 items-center gap-3 text-[13px] text-[var(--frost-text-muted)]">
          <span className="whitespace-nowrap">tick rate</span>
          <input
            type="range"
            min={1}
            max={240}
            value={rate}
            onChange={(e) => changeRate(Number(e.target.value))}
            className="flex-1 accent-[var(--frost-teal)]"
          />
          <span className="w-20 whitespace-nowrap font-mono text-[var(--frost-text)]">{rate}/s</span>
        </label>
      </div>

      {/* Conservation ledger */}
      <div className="mt-4 grid grid-cols-2 gap-3 sm:grid-cols-5">
        <Ledger label="emitted" value={hud.emitted} />
        <Ledger label="delivered" value={hud.delivered} />
        <Ledger label="stored" value={hud.stored} />
        <Ledger label="dissipated" value={hud.dissipated} />
        <div
          className="rounded-[var(--radius-sm)] border p-3"
          style={{ borderColor: balanced ? "var(--engine-formal)" : "#b4530988", background: balanced ? "color-mix(in srgb, var(--engine-formal) 8%, transparent)" : "#b4530914" }}
        >
          <div className="text-[10px] uppercase tracking-[0.14em] text-[var(--frost-text-faint)]">balance</div>
          <div className="mt-1 font-mono text-[15px]" style={{ color: balanced ? "var(--engine-formal)" : "#d97706" }}>
            {hud.balance.toFixed(3)} {balanced ? "✓" : "≠0"}
          </div>
        </div>
      </div>

      <p className="mt-5 text-[12px] leading-relaxed text-[var(--frost-text-faint)]">
        Experiment: drag tick rate up. The canvas interpolates at 60fps regardless of how sparsely Rust ticks — watch
        whether <span className="text-[var(--frost-text-muted)]">render fps</span> holds at 60 as{" "}
        <span className="text-[var(--frost-text-muted)]">events/s</span> climbs. If fps stays pinned while events scale,
        IPC is not the bottleneck and Bevy is unnecessary for this class of sim.
      </p>
    </div>
  );
}

function Metric({ label, value, good }: { label: string; value: string; good: boolean }) {
  return (
    <div>
      <div className="text-[10px] uppercase tracking-[0.14em] text-[var(--frost-text-faint)]">{label}</div>
      <div className="font-mono text-[15px]" style={{ color: good ? "var(--frost-teal)" : "#d97706" }}>
        {value}
      </div>
    </div>
  );
}

function Ledger({ label, value }: { label: string; value: number }) {
  return (
    <div className="rounded-[var(--radius-sm)] border border-[var(--frost-border-soft)] bg-[var(--frost-surface)]/60 p-3">
      <div className="text-[10px] uppercase tracking-[0.14em] text-[var(--frost-text-faint)]">{label}</div>
      <div className="mt-1 font-mono text-[15px] text-[var(--frost-text)]">{value.toFixed(1)}</div>
    </div>
  );
}
