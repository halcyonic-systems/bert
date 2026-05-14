use std::collections::HashMap;

use leptos::prelude::*;
use serde::Serialize;
use tauri_sys::core::invoke_result;
use wasm_bindgen_futures::JsFuture;

use super::agent_table::AgentComparisonTable;
use super::chart::LineChart;
use super::inputs_panel::InputsPanel;
use super::types::{
    JsonPollParams, JsonResultsParams, LaunchParams, PollParams, ResultsParams, RunInfo, RunStatus,
    SimulationResults,
};

#[derive(Serialize)]
struct LaunchArgs {
    params: LaunchParams,
}
#[derive(Serialize)]
struct PollArgs {
    params: PollParams,
}
#[derive(Serialize)]
struct ResultsArgs {
    params: ResultsParams,
}
#[derive(Serialize)]
struct JsonPollArgs {
    params: JsonPollParams,
}
#[derive(Serialize)]
struct JsonResultsArgs {
    params: JsonResultsParams,
}

const COLORS: &[&str] = &[
    "rgb(59,130,246)",
    "rgb(16,185,129)",
    "rgb(245,158,11)",
    "rgb(239,68,68)",
    "rgb(139,92,246)",
    "rgb(236,72,153)",
];

async fn sleep_ms(ms: u32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms as i32);
    });
    let _ = JsFuture::from(promise).await;
}

#[component]
pub fn SimPanel(
    visible: ReadSignal<bool>,
    on_close: Callback<()>,
    on_launch: Callback<RunInfo>,
    on_results: Callback<SimulationResults>,
    active_run: Signal<Option<RunInfo>>,
    model_name: Signal<String>,
    json_path: Signal<Option<String>>,
    model_json: Signal<Option<String>>,
) -> impl IntoView {
    let (seed_text, set_seed_text) = signal("42".to_string());
    let (steps_text, set_steps_text) = signal("200".to_string());
    let (launching, set_launching) = signal(false);
    let (results, set_results) = signal(None::<SimulationResults>);
    let (poll_status, set_poll_status) = signal(None::<RunStatus>);
    let (error_msg, set_error_msg) = signal(None::<String>);
    let (input_params, set_input_params) = signal(HashMap::<String, f64>::new());

    let is_running = Memo::new(move |_| {
        if let Some(ps) = poll_status.get() {
            ps.status == "Pending" || ps.status == "Running"
        } else {
            active_run
                .get()
                .map(|r| r.status == "Pending" || r.status == "Running")
                .unwrap_or(false)
        }
    });

    let display_status = Memo::new(move |_| {
        if let Some(ps) = poll_status.get() {
            Some((
                ps.status.clone(),
                ps.tick_count,
                ps.run_id[..8.min(ps.run_id.len())].to_string(),
            ))
        } else {
            active_run.get().map(|r| {
                (
                    r.status.clone(),
                    r.tick_count,
                    r.run_id[..8.min(r.run_id.len())].to_string(),
                )
            })
        }
    });

    let do_launch = move |_| {
        let seed: Option<u64> = seed_text.get_untracked().parse().ok();
        let steps: u64 = steps_text.get_untracked().parse().unwrap_or(200);
        let mut mn = model_name.get_untracked();
        let jp = json_path.get_untracked();
        let mj = model_json.get_untracked();

        if jp.is_none() && mn.is_empty() && mj.is_none() {
            set_error_msg.set(Some("No model loaded".to_string()));
            return;
        }

        if mn.is_empty() {
            if let Some(ref json) = mj {
                mn = serde_json::from_str::<serde_json::Value>(json)
                    .ok()
                    .and_then(|v| v["systems"].as_array()
                        .and_then(|sys| sys.iter().find(|s| s["info"]["level"].as_i64() == Some(0)))
                        .and_then(|s| s["info"]["name"].as_str())
                        .map(|s| s.to_lowercase().replace(' ', "-")))
                    .unwrap_or_else(|| "model".to_string());
            }
        }

        set_launching.set(true);
        set_results.set(None);
        set_poll_status.set(None);
        set_error_msg.set(None);

        let json_path_val = jp.clone();

        use leptos::task::spawn_local;
        spawn_local(async move {
            leptos::logging::log!("Launching simulation for model '{}'...", mn);
            let ip = input_params.get_untracked();
            let has_model_name = !mn.is_empty();
            let params = LaunchParams {
                seed,
                steps,
                db: "bert-models".to_string(),
                model_name: mn,
                json_path: jp,
                params: if ip.is_empty() { None } else { Some(ip) },
            };

            let launch_result =
                invoke_result::<RunInfo, String>("launch_simulation", &LaunchArgs { params }).await;
            match launch_result {
                Ok(run_info) => {
                    leptos::logging::log!("Launched: run_id={}", run_info.run_id);
                    let run_id = run_info.run_id.clone();
                    set_launching.set(false);
                    on_launch.run(run_info);

                    let use_json_poll = json_path_val.is_some() || has_model_name;

                    loop {
                        sleep_ms(1_500).await;

                        if use_json_poll {
                            let poll_params = JsonPollParams {
                                run_id: run_id.clone(),
                            };
                            match invoke_result::<RunStatus, String>(
                                "poll_json_run_status",
                                &JsonPollArgs {
                                    params: poll_params,
                                },
                            )
                            .await
                            {
                                Ok(status) => {
                                    let done =
                                        status.status == "Complete" || status.status == "Failed";
                                    let completed = status.status == "Complete";
                                    set_poll_status.set(Some(status));

                                    if done {
                                        if completed {
                                            let res_params = JsonResultsParams {
                                                run_id: run_id.clone(),
                                            };
                                            match invoke_result::<SimulationResults, String>(
                                                "get_json_run_results",
                                                &JsonResultsArgs { params: res_params },
                                            )
                                            .await
                                            {
                                                Ok(res) => {
                                                    leptos::logging::log!(
                                                        "Got {} system series, {} flow series",
                                                        res.system_timeseries.len(),
                                                        res.flow_timeseries.len()
                                                    );
                                                    on_results.run(res.clone());
                                                    set_results.set(Some(res));
                                                }
                                                Err(e) => set_error_msg.set(Some(format!(
                                                    "Results fetch failed: {e}"
                                                ))),
                                            }
                                        }
                                        break;
                                    }
                                }
                                Err(e) => {
                                    leptos::logging::log!("Poll error: {}", e);
                                    set_error_msg.set(Some(format!("Poll failed: {e}")));
                                    break;
                                }
                            }
                        } else {
                            let poll_params = PollParams {
                                db: "bert-models".to_string(),
                                run_id: run_id.clone(),
                            };
                            match invoke_result::<RunStatus, String>(
                                "poll_run_status",
                                &PollArgs {
                                    params: poll_params,
                                },
                            )
                            .await
                            {
                                Ok(status) => {
                                    let done =
                                        status.status == "Complete" || status.status == "Failed";
                                    let completed = status.status == "Complete";
                                    set_poll_status.set(Some(status));

                                    if done {
                                        if completed {
                                            let res_params = ResultsParams {
                                                db: "bert-models".to_string(),
                                                run_id: run_id.clone(),
                                            };
                                            match invoke_result::<SimulationResults, String>(
                                                "get_run_results",
                                                &ResultsArgs { params: res_params },
                                            )
                                            .await
                                            {
                                                Ok(res) => {
                                                    leptos::logging::log!(
                                                        "Got {} system series, {} flow series",
                                                        res.system_timeseries.len(),
                                                        res.flow_timeseries.len()
                                                    );
                                                    on_results.run(res.clone());
                                                    set_results.set(Some(res));
                                                }
                                                Err(e) => set_error_msg.set(Some(format!(
                                                    "Results fetch failed: {e}"
                                                ))),
                                            }
                                        }
                                        break;
                                    }
                                }
                                Err(e) => {
                                    leptos::logging::log!("Poll error: {}", e);
                                    set_error_msg.set(Some(format!("Poll failed: {e}")));
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    leptos::logging::log!("Launch failed: {}", e);
                    set_launching.set(false);
                    set_error_msg.set(Some(e));
                }
            }
        });
    };

    let (expanded, set_expanded) = signal(true);
    let (maximized, set_maximized) = signal(false);

    view! {
        <Show when=move || visible.get()>
            <div
                class="fixed bottom-0 left-0 right-0 z-30 bg-white border-t border-gray-200 shadow-[0_-4px_12px_rgba(0,0,0,0.08)] transition-all duration-200"
                style=move || {
                    if maximized.get() { "height: calc(100vh - 20px);" }
                    else if expanded.get() { "height: 340px;" }
                    else { "height: 44px;" }
                }
            >
                // --- Header bar (always visible) ---
                <div class="flex items-center justify-between px-4 h-11 border-b border-gray-100 bg-gray-50/80">
                    <div class="flex items-center gap-3">
                        <span class="text-xs font-semibold text-gray-700">{"Simulation"}</span>
                        <span class="text-xs text-gray-400 font-mono">{move || model_name.get()}</span>

                        {move || display_status.get().map(|(status, tick_count, short_id)| {
                            let badge = match status.as_str() {
                                "Pending" => "bg-gray-100 text-gray-600",
                                "Running" => "bg-blue-50 text-blue-600",
                                "Complete" => "bg-emerald-50 text-emerald-600",
                                "Failed" => "bg-red-50 text-red-600",
                                _ => "bg-gray-100 text-gray-600",
                            };
                            view! {
                                <span class={format!("px-2 py-0.5 rounded text-xs font-medium {badge}")}>
                                    {status}{" · "}{tick_count}{" steps"}
                                </span>
                                <span class="text-xs text-gray-300 font-mono">{short_id}</span>
                            }
                        })}

                        {move || error_msg.get().map(|e| view! {
                            <span class="text-xs text-red-500 truncate max-w-xs">{e}</span>
                        })}
                    </div>

                    <div class="flex items-center gap-2">
                        // --- Inline controls ---
                        <div class="flex items-center gap-1.5">
                            <label class="text-xs text-gray-400">{"Seed"}</label>
                            <input
                                type="number"
                                class="w-16 px-1.5 py-1 border border-gray-200 rounded text-xs font-mono bg-white"
                                prop:value=move || seed_text.get()
                                on:input=move |ev| set_seed_text.set(event_target_value(&ev))
                            />
                            <label class="text-xs text-gray-400 ml-1">{"Steps"}</label>
                            <input
                                type="number"
                                class="w-16 px-1.5 py-1 border border-gray-200 rounded text-xs font-mono bg-white"
                                prop:value=move || steps_text.get()
                                on:input=move |ev| set_steps_text.set(event_target_value(&ev))
                            />
                        </div>

                        <button
                            class="px-3 py-1 text-xs font-medium text-white bg-blue-600 hover:bg-blue-700 rounded transition disabled:opacity-50"
                            disabled=move || launching.get() || is_running.get()
                            on:click=do_launch
                        >
                            {move || {
                                if launching.get() { "Launching..." }
                                else if is_running.get() { "Running..." }
                                else { "Run" }
                            }}
                        </button>

                        <button
                            class="px-2 py-1 text-xs font-medium text-gray-600 bg-gray-100 hover:bg-gray-200 rounded transition disabled:opacity-30"
                            disabled=move || results.get().is_none()
                            on:click={
                                let model_name = model_name;
                                move |_| {
                                    if let Some(res) = results.get() {
                                        if let Ok(json) = serde_json::to_string(&res) {
                                            #[derive(serde::Serialize)]
                                            struct ExportArgs { params: ExportParams }
                                            #[derive(serde::Serialize)]
                                            struct ExportParams { results_json: String, model_name: String }
                                            let args = ExportArgs { params: ExportParams {
                                                results_json: json,
                                                model_name: model_name.get_untracked(),
                                            }};
                                            use leptos::task::spawn_local;
                                            spawn_local(async move {
                                                let _ = invoke_result::<(), String>("export_simulation_csv", &args).await;
                                            });
                                        }
                                    }
                                }
                            }
                        >
                            {"Export"}
                        </button>

                        <button
                            class="text-gray-400 hover:text-gray-600 text-sm px-1 ml-1"
                            on:click=move |_| {
                                if maximized.get() {
                                    set_maximized.set(false);
                                } else {
                                    set_expanded.update(|e| *e = !*e);
                                }
                            }
                        >
                            {move || if maximized.get() { "\u{25BC}" } else if expanded.get() { "\u{25BC}" } else { "\u{25B2}" }}
                        </button>
                        <button
                            class="text-gray-400 hover:text-gray-600 text-sm px-1"
                            title="Maximize"
                            on:click=move |_| set_maximized.update(|m| *m = !*m)
                        >
                            {move || if maximized.get() { "\u{2913}" } else { "\u{2912}" }}
                        </button>
                        <button
                            class="text-gray-300 hover:text-gray-500 text-xs px-1"
                            on:click=move |_| on_close.run(())
                        >
                            {"\u{2715}"}
                        </button>
                    </div>
                </div>

                // --- Input sliders ---
                <InputsPanel
                    model_json=model_json
                    on_change=Callback::new(move |vals: HashMap<String, f64>| {
                        set_input_params.set(vals);
                    })
                />

                <AgentComparisonTable model_json=model_json />

                // --- Expanded content: chart grid ---
                <Show when=move || expanded.get()>
                    <div class="h-[calc(100%-44px)] overflow-y-auto px-4 py-3">
                        {move || {
                            let res = results.get();
                            if let Some(res) = res {
                                let sys_series: Vec<_> = res.system_timeseries.iter()
                                    .filter(|s| s.key == "activity" || s.key == "storage" || s.key == "throughput")
                                    .cloned()
                                    .collect();
                                let flow_series: Vec<_> = res.flow_timeseries.iter()
                                    .filter(|f| f.sink_id.starts_with("Snk"))
                                    .cloned().collect();

                                if sys_series.is_empty() && flow_series.is_empty() {
                                    view! {
                                        <div class="text-sm text-gray-400 italic py-8 text-center">
                                            {"No observations recorded"}
                                        </div>
                                    }.into_any()
                                } else {
                                    let sys_views: Vec<_> = sys_series.iter().enumerate().map(|(i, s)| {
                                        let label = format!("{} \u{2014} {}", s.name, s.key);
                                        let color = COLORS[i % COLORS.len()].to_string();
                                        let ticks = s.ticks.clone();
                                        let values = s.values.clone();
                                        view! {
                                            <div class="bg-gray-50/50 rounded-lg p-2 border border-gray-100">
                                                <LineChart label=label ticks=ticks values=values color=color />
                                            </div>
                                        }
                                    }).collect();

                                    let flow_views: Vec<_> = flow_series.iter().enumerate().map(|(i, s)| {
                                        let color = COLORS[(i + 3) % COLORS.len()].to_string();
                                        let name = s.name.clone();
                                        let ticks = s.ticks.clone();
                                        let values = s.values.clone();
                                        view! {
                                            <div class="bg-gray-50/50 rounded-lg p-2 border border-gray-100">
                                                <LineChart label=name ticks=ticks values=values color=color />
                                            </div>
                                        }
                                    }).collect();

                                    view! {
                                        <div>
                                            <div class="grid grid-cols-2 xl:grid-cols-3 gap-3">
                                                {sys_views}
                                            </div>
                                            {if !flow_views.is_empty() {
                                                Some(view! {
                                                    <div class="mt-3">
                                                        <div class="text-xs text-gray-400 font-medium mb-2">{"Flows"}</div>
                                                        <div class="grid grid-cols-2 xl:grid-cols-3 gap-3">
                                                            {flow_views}
                                                        </div>
                                                    </div>
                                                })
                                            } else { None }}
                                        </div>
                                    }.into_any()
                                }
                            } else {
                                view! {
                                    <div class="text-sm text-gray-400 italic py-8 text-center">
                                        {"Run a simulation to see results"}
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </Show>
            </div>
        </Show>
    }
}
