//! # Model Browser Component
//!
//! Provides a modal interface for browsing and loading pre-built BERT models.
//! This component displays example models across different domains (biological,
//! organizational, technical) to help users understand BERT's capabilities.

use leptos::prelude::*;
use crate::LoadFileEvent;

// Embedded model data - These JSON files contain complete BERT model definitions
// with boundaries, interfaces, flows, and system elements properly configured.
const CELL_MODEL_JSON: &str = include_str!("../../../assets/models/cell.json");
const ORGANIZATION_MODEL_JSON: &str = include_str!("../../../assets/models/organization.json");
const SOLAR_PANEL_MODEL_JSON: &str = include_str!("../../../assets/models/solar-panel.json");
const LLM_MODEL_JSON: &str = include_str!("../../../assets/models/llm.json");
const ECOSYSTEM_MODEL_JSON: &str = include_str!("../../../assets/models/ecosystem.json");
const SYSTEM_MODEL_JSON: &str = include_str!("../../../assets/models/system.json");
const BERT_MODEL_JSON: &str = include_str!("../../../assets/models/bert.json");
const BITCOIN_MODEL_JSON: &str = include_str!("../../../assets/models/bitcoin.json");

/// Modal component for browsing and loading example BERT models.
///
/// Displays a grid of available example models with descriptions, allowing users
/// to quickly load pre-built models for exploration or as starting points for
/// their own system analysis.
///
/// # Parameters
///
/// - `visible`: Signal controlling modal visibility
/// - `on_close`: Callback triggered when modal should be closed
/// - `on_load`: Callback triggered when a model is selected for loading
///
/// # Examples
///
/// ```rust,ignore
/// // Usage in parent component
/// let (show_browser, set_show_browser) = signal(false);
/// 
/// view! {
///     <ModelBrowser
///         visible=show_browser
///         on_close=move |_| set_show_browser.set(false)
///         on_load=handle_model_load
///     />
/// }
/// ```
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
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 max-h-96 overflow-y-auto">
                        // Simple Cell Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading cell model, data length: {}", CELL_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:cell.json".to_string(),
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
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading organization model, data length: {}", ORGANIZATION_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:organization.json".to_string(),
                                    data: ORGANIZATION_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"Organization"</h3>
                            <p class="text-sm text-gray-600 mt-1">"Organizational structure with human capital, financial flows, and waste"</p>
                        </button>
                        
                        // Solar Panel Model  
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading solar panel model, data length: {}", SOLAR_PANEL_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:solar-panel.json".to_string(),
                                    data: SOLAR_PANEL_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"Solar Panel"</h3>
                            <p class="text-sm text-gray-600 mt-1">"Photovoltaic system converting sunlight to electricity with thermal waste"</p>
                        </button>
                        
                        // LLM Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading LLM model, data length: {}", LLM_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:llm.json".to_string(),
                                    data: LLM_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"Large Language Model"</h3>
                            <p class="text-sm text-gray-600 mt-1">"AI system with transformer architecture, attention mechanisms, and knowledge processing"</p>
                        </button>
                        
                        // Ecosystem Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading ecosystem model, data length: {}", ECOSYSTEM_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:ecosystem.json".to_string(),
                                    data: ECOSYSTEM_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"Ecosystem"</h3>
                            <p class="text-sm text-gray-600 mt-1">"Natural ecosystem with predator-prey dynamics, nutrient cycling, and energy flows"</p>
                        </button>
                        
                        // Generic System Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading system model, data length: {}", SYSTEM_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:system.json".to_string(),
                                    data: SYSTEM_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"Generic System"</h3>
                            <p class="text-sm text-gray-600 mt-1">"Pure systems theory template with Mobus 7-tuple framework and theoretical grounding"</p>
                        </button>
                        
                        // BERT Self-Referential Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading BERT model, data length: {}", BERT_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:bert.json".to_string(),
                                    data: BERT_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"BERT Tool"</h3>
                            <p class="text-sm text-gray-600 mt-1">"Self-referential analysis of BERT itself as a bounded entity reasoning system"</p>
                        </button>
                        
                        // Bitcoin Network Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading Bitcoin model, data length: {}", BITCOIN_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:bitcoin.json".to_string(),
                                    data: BITCOIN_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"Bitcoin Network"</h3>
                            <p class="text-sm text-gray-600 mt-1">"Deep Systems Analysis of Bitcoin's validation, mining, protocol, and development subsystems"</p>
                        </button>
                    </div>
                    
                    <div class="mt-6 text-center text-sm text-gray-600">
                        "8 enhanced educational models available. Press Ctrl+L to load your own files."
                    </div>
                </div>
            </div>
        </Show>
    }
}