use leptos::prelude::*;
use tauri_sys::core::invoke;

use super::types::{LaunchParams, RunInfo};

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

    let is_running = Memo::new(move |_| {
        active_run.get().map(|r| r.status == "Pending" || r.status == "Running").unwrap_or(false)
    });

    let do_launch = move |_| {
        let seed: Option<u64> = seed_text.get_untracked().parse().ok();
        let steps: u64 = steps_text.get_untracked().parse().unwrap_or(200);

        set_launching.set(true);
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
            <div class="absolute top-16 right-4 z-30 w-72 bg-white rounded-lg shadow-lg border border-gray-200">

                <div class="px-4 py-3 border-b border-gray-100 flex justify-between items-center">
                    <span class="text-sm font-semibold text-gray-900">{"Simulation"}</span>
                    <button class="text-gray-400 hover:text-gray-600 text-sm"
                            on:click=move |_| on_close.run(())>
                        {"x"}
                    </button>
                </div>

                <div class="px-4 py-3 space-y-3">
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
                </div>
            </div>
        </Show>
    }
}
