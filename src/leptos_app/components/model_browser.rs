use crate::LoadFileEvent;
use leptos::prelude::*;

const LLM_MODEL_JSON: &str = include_str!("../../../assets/models/examples/llm.json");
const BITCOIN_MODEL_JSON: &str = include_str!("../../../assets/models/examples/bitcoin.json");
const ETHEREUM_MODEL_JSON: &str = include_str!("../../../assets/models/examples/ethereum.json");
const COSMOS_HUB_MODEL_JSON: &str =
    include_str!("../../../assets/models/examples/cosmos-hub.json");
const SOLANA_MODEL_JSON: &str = include_str!("../../../assets/models/examples/solana.json");

struct ExampleModel {
    name: &'static str,
    description: &'static str,
    file_path: &'static str,
    data: &'static str,
}

const EXAMPLES: &[ExampleModel] = &[
    ExampleModel {
        name: "Large Language Model",
        description: "AI system with transformer architecture, attention mechanisms, and knowledge processing",
        file_path: "template:llm.json",
        data: "", // handled separately due to const limitations
    },
    ExampleModel {
        name: "Bitcoin Network",
        description: "Deep systems analysis of Bitcoin's validation, mining, protocol, and development subsystems",
        file_path: "template:bitcoin.json",
        data: "",
    },
    ExampleModel {
        name: "Ethereum Network",
        description: "Smart contract platform with EVM, consensus, staking, and dApp ecosystem",
        file_path: "template:ethereum.json",
        data: "",
    },
    ExampleModel {
        name: "Cosmos Hub",
        description: "Internet of Blockchains with IBC protocol, Tendermint consensus, and ATOM staking",
        file_path: "template:cosmos-hub.json",
        data: "",
    },
    ExampleModel {
        name: "Solana Network",
        description: "High-performance blockchain with Proof of History, Tower BFT, and parallel execution",
        file_path: "template:solana.json",
        data: "",
    },
];

fn get_example_data(file_path: &str) -> &'static str {
    match file_path {
        "template:llm.json" => LLM_MODEL_JSON,
        "template:bitcoin.json" => BITCOIN_MODEL_JSON,
        "template:ethereum.json" => ETHEREUM_MODEL_JSON,
        "template:cosmos-hub.json" => COSMOS_HUB_MODEL_JSON,
        "template:solana.json" => SOLANA_MODEL_JSON,
        _ => "",
    }
}

#[component]
pub fn ModelBrowser(
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] on_load: Callback<LoadFileEvent>,
) -> impl IntoView {
    let (local_models, set_local_models) = signal(Vec::<(String, String)>::new());

    Effect::new(move |_| {
        if visible.get() {
            let tauri_exists = leptos_use::js! { "__TAURI__" in &window() };
            if tauri_exists {
                leptos::task::spawn_local({
                    let set_local = set_local_models;
                    async move {
                        #[derive(serde::Deserialize)]
                        struct LocalModelInfo {
                            name: String,
                            path: String,
                            #[allow(dead_code)]
                            modified: u64,
                        }
                        let models = tauri_sys::core::invoke::<Vec<LocalModelInfo>>(
                            "list_local_models",
                            (),
                        )
                        .await;
                        set_local.set(
                            models.into_iter().map(|m| (m.name, m.path)).collect(),
                        );
                    }
                });
            }
        }
    });

    view! {
        <Show when=move || visible.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 z-30 flex items-center justify-center">
                <div class="bg-white rounded-lg shadow-xl max-w-3xl max-h-[80vh] m-4 p-6 overflow-y-auto">
                    <div class="flex justify-between items-center mb-6">
                        <h2 class="text-2xl font-bold text-gray-900">"Model Browser"</h2>
                        <button
                            class="text-gray-400 hover:text-gray-600 text-2xl font-bold"
                            on:click=move |_| on_close.run(())
                        >
                            "×"
                        </button>
                    </div>

                    // Examples section
                    <div class="mb-6">
                        <h3 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-3">"Examples"</h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                            {EXAMPLES.iter().map(|example| {
                                let fp = example.file_path;
                                let name = example.name;
                                let desc = example.description;
                                view! {
                                    <button
                                        class="p-4 border rounded-lg hover:bg-gray-50 text-left transition-colors"
                                        on:click=move |_| {
                                            let data = get_example_data(fp);
                                            on_load.run(LoadFileEvent {
                                                file_path: fp.to_string(),
                                                data: data.as_bytes().to_vec(),
                                            });
                                            on_close.run(());
                                        }
                                    >
                                        <h3 class="font-semibold text-gray-800">{name}</h3>
                                        <p class="text-sm text-gray-600 mt-1">{desc}</p>
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    // Your Models section
                    {move || {
                        let models = local_models.get();
                        if models.is_empty() {
                            None
                        } else {
                            Some(view! {
                                <div class="mb-6">
                                    <h3 class="text-sm font-medium text-gray-500 uppercase tracking-wider mb-3">"Your Models"</h3>
                                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                                        {models.into_iter().map(|(name, path)| {
                                            let path_for_click = path.clone();
                                            let path_for_display = path.clone();
                                            view! {
                                                <button
                                                    class="p-4 border border-blue-200 bg-blue-50/50 rounded-lg hover:bg-blue-100 text-left transition-colors"
                                                    title=path_for_display
                                                    on:click=move |_| {
                                                        let p = path_for_click.clone();
                                                        let on_load = on_load;
                                                        let on_close = on_close;
                                                        leptos::task::spawn_local(async move {
                                                            #[derive(serde::Serialize)]
                                                            struct Args { pb: std::path::PathBuf }
                                                            #[derive(serde::Deserialize)]
                                                            struct FileData { data: Vec<u8>, path: String }
                                                            let file_data = tauri_sys::core::invoke::<FileData>(
                                                                "load_file",
                                                                &Args { pb: std::path::PathBuf::from(&p) },
                                                            ).await;
                                                            on_load.run(LoadFileEvent {
                                                                file_path: file_data.path,
                                                                data: file_data.data,
                                                            });
                                                            on_close.run(());
                                                        });
                                                    }
                                                >
                                                    <h3 class="font-semibold text-gray-800 capitalize">{name}</h3>
                                                    <p class="text-xs text-gray-500 mt-1">"Saved locally"</p>
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            })
                        }
                    }}

                </div>
            </div>
        </Show>
    }
}
