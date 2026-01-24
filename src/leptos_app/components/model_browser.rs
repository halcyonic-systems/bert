//! # Model Browser Component
//!
//! Provides a modal interface for browsing and loading pre-built BERT models.
//! This component displays example models across different domains (biological,
//! organizational, technical) to help users understand BERT's capabilities.

use crate::LoadFileEvent;
use leptos::prelude::*;

// Embedded model data - These JSON files contain complete BERT model definitions
// with boundaries, interfaces, flows, and system elements properly configured.
const LLM_MODEL_JSON: &str = include_str!("../../../assets/models/examples/llm.json");
const BITCOIN_MODEL_JSON: &str = include_str!("../../../assets/models/examples/bitcoin.json");
const ETHEREUM_MODEL_JSON: &str = include_str!("../../../assets/models/examples/ethereum.json");
const COSMOS_HUB_MODEL_JSON: &str = include_str!("../../../assets/models/examples/cosmos-hub.json");
const SOLANA_MODEL_JSON: &str = include_str!("../../../assets/models/examples/solana.json");

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
                            <p class="text-sm text-gray-600 mt-1">"Deep systems analysis of Bitcoin's validation, mining, protocol, and development subsystems"</p>
                        </button>

                        // Ethereum Network Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading Ethereum model, data length: {}", ETHEREUM_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:ethereum.json".to_string(),
                                    data: ETHEREUM_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"Ethereum Network"</h3>
                            <p class="text-sm text-gray-600 mt-1">"Smart contract platform with EVM, consensus, staking, and dApp ecosystem"</p>
                        </button>

                        // Cosmos Hub Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading Cosmos Hub model, data length: {}", COSMOS_HUB_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:cosmos-hub.json".to_string(),
                                    data: COSMOS_HUB_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"Cosmos Hub"</h3>
                            <p class="text-sm text-gray-600 mt-1">"Internet of Blockchains with IBC protocol, Tendermint consensus, and ATOM staking"</p>
                        </button>

                        // Solana Model
                        <button
                            class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                            on:click=move |_| {
                                leptos::logging::log!("Loading Solana model, data length: {}", SOLANA_MODEL_JSON.len());
                                on_load.run(LoadFileEvent {
                                    file_path: "template:solana.json".to_string(),
                                    data: SOLANA_MODEL_JSON.as_bytes().to_vec(),
                                });
                                on_close.run(());
                            }
                        >
                            <h3 class="font-semibold text-gray-800">"Solana Network"</h3>
                            <p class="text-sm text-gray-600 mt-1">"High-performance blockchain with Proof of History, Tower BFT, and parallel execution"</p>
                        </button>

                    </div>

                    <div class="mt-6 text-center text-sm text-gray-600">
                        "5 example models available. Press Ctrl+L to load your own files."
                    </div>
                </div>
            </div>
        </Show>
    }
}
