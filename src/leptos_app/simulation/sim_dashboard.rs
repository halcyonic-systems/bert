use leptos::prelude::*;

use super::state_panel::StatePanel;
use super::types::SimulationResults;

#[component]
pub fn SimDashboard(results: Signal<Option<SimulationResults>>) -> impl IntoView {
    let (expanded, set_expanded) = signal(true);

    let has_data = Memo::new(move |_| results.get().is_some());

    view! {
        <Show when=move || has_data.get()>
            <div
                class="fixed bottom-0 left-0 right-0 z-20 bg-white border-t border-gray-200 shadow-lg transition-all duration-300"
                style=move || {
                    if expanded.get() {
                        "height: 280px;"
                    } else {
                        "height: 36px;"
                    }
                }
            >
                // Tab bar + collapse toggle
                <div class="flex items-center justify-between px-4 h-9 border-b border-gray-100 bg-gray-50">
                    <div class="flex items-center gap-1">
                        <button class="px-3 py-1 text-xs font-medium text-blue-700 bg-blue-50 rounded">
                            {"State"}
                        </button>
                        <button class="px-3 py-1 text-xs font-medium text-gray-400 cursor-not-allowed" disabled>
                            {"History"}
                        </button>
                        <button class="px-3 py-1 text-xs font-medium text-gray-400 cursor-not-allowed" disabled>
                            {"Flows"}
                        </button>
                    </div>
                    <div class="flex items-center gap-2">
                        <span class="text-xs text-gray-400">{"v(t) Dashboard"}</span>
                        <button
                            class="text-gray-400 hover:text-gray-600 text-sm px-1"
                            on:click=move |_| set_expanded.update(|e| *e = !*e)
                        >
                            {move || if expanded.get() { "\u{25BC}" } else { "\u{25B2}" }}
                        </button>
                    </div>
                </div>

                // Panel content
                <Show when=move || expanded.get()>
                    <div style="height: calc(100% - 36px);">
                        <StatePanel results=results />
                    </div>
                </Show>
            </div>
        </Show>
    }
}
