use leptos::prelude::*;
use tauri_sys::core::invoke;

use super::chart::LineChart;
use super::types::{LaunchParams, ResultsParams, RunInfo, SimulationResults};

const COLORS: &[&str] = &[
    "rgb(59,130,246)", "rgb(16,185,129)", "rgb(245,158,11)",
    "rgb(239,68,68)", "rgb(139,92,246)", "rgb(236,72,153)",
];

#[component]
pub fn SimPanel(
    visible: ReadSignal<bool>,
    on_close: Callback<()>,
    on_launch: Callback<RunInfo>,
    active_run: Signal<Option<RunInfo>>,
) -> impl IntoView {
    let (seed_text, set_seed_text) = signal("42".to_string());
    let (steps_text, set_steps_text) = signal("200".to_string());
    let (launching, set_launching) = signal(false);
    let (results, set_results) = signal(None::<SimulationResults>);

    let is_running = Memo::new(move |_| {
        active_run.get().map(|r| r.status == "Pending" || r.status == "Running").unwrap_or(false)
    });

    let is_complete = Memo::new(move |_| {
        active_run.get().map(|r| r.status == "Complete").unwrap_or(false)
    });

    // Fetch results when run completes
    Effect::new(move |_| {
        if is_complete.get() {
            if let Some(run) = active_run.get() {
                let run_id = run.run_id.clone();
                use leptos::task::spawn_local;
                spawn_local(async move {
                    let params = ResultsParams {
                        db: "bert-models".to_string(),
                        run_id,
                    };
                    let res = invoke::<SimulationResults>("get_run_results", &params).await;
                    set_results.set(Some(res));
                });
            }
        }
    });

    let do_launch = move |_| {
        let seed: Option<u64> = seed_text.get_untracked().parse().ok();
        let steps: u64 = steps_text.get_untracked().parse().unwrap_or(200);

        set_launching.set(true);
        set_results.set(None);
        use leptos::task::spawn_local;
        spawn_local(async move {
            let params = LaunchParams {
                seed,
                steps,
                db: "bert-models".to_string(),
                model_name: "bitcoin".to_string(),
            };
            let run_info = invoke::<RunInfo>("launch_simulation", &params).await;
            set_launching.set(false);
            on_launch.run(run_info);
        });
    };

    view! {
        <Show when=move || visible.get()>
            <div class="absolute top-16 right-4 z-30 w-80 bg-white rounded-lg shadow-lg border border-gray-200 max-h-[calc(100vh-5rem)] overflow-y-auto">

                <div class="px-4 py-3 border-b border-gray-100 flex justify-between items-center sticky top-0 bg-white rounded-t-lg">
                    <span class="text-sm font-semibold text-gray-900">{"Simulation"}</span>
                    <button class="text-gray-400 hover:text-gray-600 text-sm"
                            on:click=move |_| on_close.run(())>
                        {"x"}
                    </button>
                </div>

                <div class="px-4 py-3 space-y-3">
                    // --- Controls ---
                    <div class="grid grid-cols-2 gap-3">
                        <div>
                            <label class="block text-xs font-medium text-gray-500 mb-1">{"Seed"}</label>
                            <input
                                type="number"
                                class="w-full px-2 py-1.5 border border-gray-200 rounded text-sm font-mono"
                                prop:value=move || seed_text.get()
                                on:input=move |ev| set_seed_text.set(event_target_value(&ev))
                            />
                        </div>
                        <div>
                            <label class="block text-xs font-medium text-gray-500 mb-1">{"Steps"}</label>
                            <input
                                type="number"
                                class="w-full px-2 py-1.5 border border-gray-200 rounded text-sm font-mono"
                                prop:value=move || steps_text.get()
                                on:input=move |ev| set_steps_text.set(event_target_value(&ev))
                            />
                        </div>
                    </div>

                    <button
                        class="w-full px-3 py-2 text-sm text-white bg-blue-600 hover:bg-blue-700 rounded transition disabled:opacity-50"
                        disabled=move || launching.get() || is_running.get()
                        on:click=do_launch
                    >
                        {move || {
                            if launching.get() { "Launching..." }
                            else if is_running.get() { "Running..." }
                            else { "Run" }
                        }}
                    </button>

                    // --- Run status ---
                    {move || active_run.get().map(|r| {
                        let badge = match r.status.as_str() {
                            "Pending" => "bg-gray-100 text-gray-700",
                            "Running" => "bg-blue-100 text-blue-700",
                            "Complete" => "bg-green-100 text-green-700",
                            "Failed" => "bg-red-100 text-red-700",
                            _ => "bg-gray-100 text-gray-700",
                        };
                        view! {
                            <div class="pt-2 border-t border-gray-100">
                                <div class="flex items-center gap-2">
                                    <span class={format!("px-2 py-0.5 rounded-full text-xs font-medium {badge}")}>
                                        {r.status.clone()}
                                    </span>
                                    <span class="text-xs text-gray-500 font-mono truncate">
                                        {r.run_id[..8.min(r.run_id.len())].to_string()}
                                    </span>
                                </div>
                                <div class="text-xs text-gray-500 mt-1 font-mono">
                                    {"Tick "}{r.tick_count}
                                </div>
                            </div>
                        }
                    })}

                    // --- Results: time series charts ---
                    {move || results.get().map(|res| {
                        let sys_series = res.system_timeseries.clone();
                        let flow_series = res.flow_timeseries.clone();
                        let has_data = !sys_series.is_empty() || !flow_series.is_empty();

                        view! {
                            <div class="pt-3 border-t border-gray-100 space-y-1">
                                <div class="text-xs font-semibold text-gray-700 mb-2">{"Results"}</div>

                                {if !sys_series.is_empty() {
                                    Some(view! {
                                        <div class="space-y-1">
                                            <div class="text-xs text-gray-500 font-medium">{"System metrics"}</div>
                                            <For
                                                each=move || sys_series.clone().into_iter().enumerate()
                                                key=|(i, _)| *i
                                                children=move |(i, s)| {
                                                    let label = format!("{} — {}", s.name, s.key);
                                                    let color = COLORS[i % COLORS.len()].to_string();
                                                    view! {
                                                        <LineChart
                                                            label=label
                                                            ticks=s.ticks.clone()
                                                            values=s.values.clone()
                                                            color=color
                                                        />
                                                    }
                                                }
                                            />
                                        </div>
                                    })
                                } else { None }}

                                {if !flow_series.is_empty() {
                                    Some(view! {
                                        <div class="space-y-1">
                                            <div class="text-xs text-gray-500 font-medium mt-2">{"Flow observations"}</div>
                                            <For
                                                each=move || flow_series.clone().into_iter().enumerate()
                                                key=|(i, _)| *i
                                                children=move |(i, s)| {
                                                    let color = COLORS[(i + 3) % COLORS.len()].to_string();
                                                    view! {
                                                        <LineChart
                                                            label=s.name.clone()
                                                            ticks=s.ticks.clone()
                                                            values=s.values.clone()
                                                            color=color
                                                        />
                                                    }
                                                }
                                            />
                                        </div>
                                    })
                                } else { None }}

                                {if !has_data {
                                    Some(view! {
                                        <div class="text-xs text-gray-400 italic">{"No observations recorded"}</div>
                                    })
                                } else { None }}
                            </div>
                        }
                    })}
                </div>
            </div>
        </Show>
    }
}
