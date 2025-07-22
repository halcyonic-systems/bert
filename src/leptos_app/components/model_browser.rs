use leptos::prelude::*;
use crate::LoadFileEvent;

// Embedded model data
const CELL_MODEL_JSON: &str = include_str!("../../../assets/models/cell.json");

#[component]
pub fn ModelBrowser(
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_load: Callback<LoadFileEvent>,
) -> impl IntoView {
    view! {
        <Show when=move || visible.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 z-30 flex items-center justify-center">
                <div class="bg-white rounded-lg shadow-xl max-w-3xl max-h-[80vh] m-4 p-6">
                    <div class="flex justify-between items-center mb-6">
                        <h2 class="text-2xl font-bold text-gray-900">"Model Browser"</h2>
                        <button
                            class="text-gray-400 hover:text-gray-600 text-2xl font-bold"
                            on:click=move |_| on_close.run(())
                        >
                            "×"
                        </button>
                    </div>
                    
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        // Simple Cell Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading cell model, data length: {}", CELL_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "cell.json".to_string(),
                                    data: CELL_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"Simple Cell"</h3>
                            <p class="text-sm text-gray-600 mt-1">"A biological cell with ATP production and CO2 waste pathways"</p>
                        </button>
                        
                        // Organization Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors disabled:opacity-50 cursor-not-allowed"
                            disabled=true
                        >
                            <h3 class="font-semibold text-gray-800">"Organization"</h3>
                            <p class="text-sm text-gray-600 mt-1">"Basic organization structure model (coming soon)"</p>
                        </button>
                        
                        // Circuit Model  
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors disabled:opacity-50 cursor-not-allowed"
                            disabled=true
                        >
                            <h3 class="font-semibold text-gray-800">"Circuit"</h3>
                            <p class="text-sm text-gray-600 mt-1">"Simple electrical circuit example (coming soon)"</p>
                        </button>
                    </div>
                    
                    <div class="mt-6 text-center text-sm text-gray-600">
                        "More models coming soon. Press Ctrl+L to load your own files."
                    </div>
                </div>
            </div>
        </Show>
    }
}