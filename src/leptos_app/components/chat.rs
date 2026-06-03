use leptos::ev;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tauri_sys::core::invoke_result;

use super::AppMode;

fn dimension_color(dim: &str) -> &'static str {
    match dim {
        "C" => "bg-blue-100 text-blue-700",
        "N" => "bg-green-100 text-green-700",
        "E" => "bg-amber-100 text-amber-700",
        "G" => "bg-purple-100 text-purple-700",
        "B" => "bg-red-100 text-red-700",
        "T" => "bg-cyan-100 text-cyan-700",
        "H" => "bg-orange-100 text-orange-700",
        "Δt" | "Dt" => "bg-pink-100 text-pink-700",
        _ => "bg-gray-100 text-gray-700",
    }
}

#[derive(Serialize)]
struct ChatArgs {
    request: ChatRequest,
}

#[derive(Serialize)]
struct ChatRequest {
    message: String,
    model_context: String,
    mode: String,
    history: Vec<HistoryEntry>,
}

#[derive(Serialize)]
struct HistoryEntry {
    role: String,
    content: String,
}

#[derive(Deserialize, Clone, Debug)]
struct SourceRef {
    source: String,
    #[serde(default)]
    excerpt: String,
}

#[derive(Deserialize, Clone, Debug)]
struct ChatResponse {
    response: String,
    provider: String,
    #[serde(default)]
    dimensions: Option<Vec<String>>,
    #[serde(default)]
    route: Option<String>,
    #[serde(default)]
    sources: Option<Vec<SourceRef>>,
}

#[derive(Serialize)]
struct GenerateArgs {
    conversation: String,
}

#[derive(Deserialize, Clone, Debug)]
struct GenerateResponse {
    json_data: String,
    #[serde(default)]
    repairs: Vec<String>,
}

#[derive(Clone, Debug)]
struct ChatMessage {
    id: usize,
    content: String,
    is_user: bool,
    provider: Option<String>,
    dimensions: Option<Vec<String>>,
    route: Option<String>,
    sources: Option<Vec<SourceRef>>,
}

#[component]
pub fn ChatPanel(
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] model_context: Signal<Option<String>>,
    #[prop(into)] app_mode: Signal<AppMode>,
    #[prop(into)] on_model_generated: Callback<Vec<u8>>,
) -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<ChatMessage>::new());
    let (input_value, set_input_value) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (is_generating, set_is_generating) = signal(false);
    let (next_id, set_next_id) = signal(0usize);

    let alloc_id = move || {
        let id = next_id.get_untracked();
        set_next_id.set(id + 1);
        id
    };

    let is_creating = Memo::new(move |_| app_mode.get() == AppMode::Creating);
    let has_model = Memo::new(move |_| model_context.get().is_some());

    Effect::new(move |prev_creating: Option<bool>| {
        let creating = is_creating.get();
        if creating && prev_creating != Some(true) {
            set_messages.set(vec![ChatMessage {
                id: 0,
                content: "Name a system and I'll generate a first draft. You can refine it from there.".to_string(),
                is_user: false,
                provider: Some("system".to_string()),
                dimensions: None, route: None, sources: None,
            }]);
            set_next_id.set(1);
        }
        creating
    });

    let send_message = move |_| {
        let message = input_value.get_untracked();
        if message.trim().is_empty() || is_loading.get_untracked() {
            return;
        }

        let uid = alloc_id();
        set_messages.update(|msgs| {
            msgs.push(ChatMessage {
                id: uid,
                content: message.clone(),
                is_user: true,
                provider: None,
                dimensions: None,
                route: None,
                sources: None,
            });
        });
        set_is_loading.set(true);
        set_input_value.set(String::new());

        let creating = is_creating.get_untracked();
        let first_user_msg = creating && !messages.get_untracked().iter().any(|m| m.is_user && m.id != uid);

        if first_user_msg {
            leptos::task::spawn_local(async move {
                let transcript = format!("User: {message}");
                let args = GenerateArgs { conversation: transcript };

                match invoke_result::<GenerateResponse, String>(
                    "generate_model_from_conversation", &args,
                ).await {
                    Ok(resp) => {
                        if !resp.repairs.is_empty() {
                            web_sys::console::log_1(&format!("Model repairs: {:?}", resp.repairs).into());
                        }
                        on_model_generated.run(resp.json_data.into_bytes());
                        let rid = next_id.get_untracked();
                        set_next_id.set(rid + 1);
                        set_messages.update(|msgs| {
                            msgs.push(ChatMessage {
                                id: rid,
                                content: "First draft generated! Describe changes and click **Generate Model** to rebuild.".to_string(),
                                is_user: false,
                                provider: Some("system".to_string()),
                                dimensions: None, route: None, sources: None,
                            });
                        });
                    }
                    Err(e) => {
                        let rid = next_id.get_untracked();
                        set_next_id.set(rid + 1);
                        set_messages.update(|msgs| {
                            msgs.push(ChatMessage {
                                id: rid,
                                content: format!("Generation failed: {e}"),
                                is_user: false,
                                provider: Some("system".to_string()),
                                dimensions: None, route: None, sources: None,
                            });
                        });
                    }
                }
                set_is_loading.set(false);
            });
            return;
        }

        let history: Vec<HistoryEntry> = messages
            .get_untracked()
            .iter()
            .filter(|m| m.id != uid)
            .filter(|m| m.provider.as_deref() != Some("system"))
            .map(|m| HistoryEntry {
                role: if m.is_user {
                    "user".to_string()
                } else {
                    "assistant".to_string()
                },
                content: m.content.clone(),
            })
            .collect();

        leptos::task::spawn_local(async move {
            let (ctx, mode) = if creating {
                (String::new(), "creation".to_string())
            } else {
                (
                    model_context.get_untracked()
                        .unwrap_or_else(|| r#"{"environment":{"info":{"name":"No model loaded"}},"systems":[],"interactions":[]}"#.to_string()),
                    "analysis".to_string(),
                )
            };

            let args = ChatArgs {
                request: ChatRequest {
                    message: message.clone(),
                    model_context: ctx,
                    mode,
                    history,
                },
            };

            let (response_text, provider, dims, route, sources) =
                match invoke_result::<ChatResponse, String>("chat_with_model", &args).await {
                    Ok(resp) => (resp.response, Some(resp.provider), resp.dimensions, resp.route, resp.sources),
                    Err(e) => (format!("Error: {e}"), None, None, None, None),
                };

            let rid = next_id.get_untracked();
            set_next_id.set(rid + 1);

            set_messages.update(|msgs| {
                msgs.push(ChatMessage {
                    id: rid,
                    content: response_text,
                    is_user: false,
                    provider,
                    dimensions: dims,
                    route,
                    sources,
                });
            });
            set_is_loading.set(false);
        });
    };

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        send_message(());
    };

    let generate_model = move |_| {
        if is_generating.get_untracked() {
            return;
        }
        set_is_generating.set(true);

        // Show a status message in chat
        let status_id = alloc_id();
        set_messages.update(|msgs| {
            msgs.push(ChatMessage {
                id: status_id,
                content: "Generating your model... This may take a moment.".to_string(),
                is_user: false,
                provider: Some("system".to_string()),
                dimensions: None, route: None, sources: None,
            });
        });

        let transcript = messages
            .get_untracked()
            .iter()
            .filter(|m| m.id != status_id)
            .map(|m| {
                let role = if m.is_user { "User" } else { "Assistant" };
                format!("{}: {}", role, m.content)
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        leptos::task::spawn_local(async move {
            let args = GenerateArgs {
                conversation: transcript,
            };

            match invoke_result::<GenerateResponse, String>(
                "generate_model_from_conversation",
                &args,
            )
            .await
            {
                Ok(resp) => {
                    if !resp.repairs.is_empty() {
                        web_sys::console::log_1(&format!("Model repairs: {:?}", resp.repairs).into());
                    }
                    on_model_generated.run(resp.json_data.into_bytes());
                }
                Err(e) => {
                    let rid = next_id.get_untracked();
                    set_next_id.set(rid + 1);
                    set_messages.update(|msgs| {
                        msgs.push(ChatMessage {
                            id: rid,
                            content: format!("Generation failed: {e}"),
                            is_user: false,
                            provider: Some("system".to_string()),
                            dimensions: None, route: None, sources: None,
                        });
                    });
                }
            }
            set_is_generating.set(false);
        });
    };

    let has_user_messages = Memo::new(move |_| messages.get().iter().any(|m| m.is_user));

    view! {
        <Show when=move || visible.get()>
            <div class={move || {
                if is_creating.get() {
                    "absolute inset-0 z-30 flex items-center justify-center bg-gray-50/60 backdrop-blur-sm"
                } else {
                    "absolute bottom-4 right-4 z-30"
                }
            }}>
                <div class={move || {
                    if is_creating.get() {
                        "bg-white rounded-2xl shadow-2xl border border-gray-200 w-[36rem] h-[32rem] flex flex-col"
                    } else {
                        "bg-white rounded-lg shadow-xl border border-gray-200 w-96 h-[28rem] flex flex-col"
                    }
                }}>
                    // Header
                    <div class="bg-gray-50 rounded-t-lg p-3 border-b border-gray-200 flex items-center justify-between">
                        <div>
                            <h3 class="font-semibold text-gray-800 text-sm">
                                {move || if is_creating.get() {
                                    "Create New Model"
                                } else if has_model.get() {
                                    "Chat with Model"
                                } else {
                                    "Chat"
                                }}
                            </h3>
                            <span class="text-xs text-gray-400">
                                {move || if is_creating.get() {
                                    "Describe your system to generate a model"
                                } else if has_model.get() {
                                    "Ask about your loaded model"
                                } else {
                                    "Load a model for analysis"
                                }}
                            </span>
                        </div>
                        <div class="flex items-center gap-2">
                            <Show when=move || is_creating.get() && has_user_messages.get()>
                                <button
                                    class="px-3 py-1.5 rounded-lg bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white text-xs font-medium transition-colors"
                                    prop:disabled=move || is_generating.get() || is_loading.get()
                                    on:click=generate_model
                                >
                                    {move || if is_generating.get() { "Generating..." } else { "Generate Model" }}
                                </button>
                            </Show>
                            <button
                                class="text-gray-400 hover:text-gray-600"
                                on:click=move |_| on_close.run(())
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </div>
                    </div>

                    // Messages
                    <div class="flex-1 overflow-y-auto p-3 space-y-2 text-sm">
                        <For
                            each=move || messages.get()
                            key=|msg| msg.id
                            children=move |msg| {
                                let align = if msg.is_user { "justify-end" } else { "justify-start" };
                                let bubble = if msg.is_user {
                                    "bg-blue-500 text-white rounded-br-none"
                                } else {
                                    "bg-gray-100 text-gray-800 rounded-bl-none"
                                };
                                let provider_label = msg.provider.clone();
                                let route_label = msg.route.clone();
                                let dims = msg.dimensions.clone().unwrap_or_default();
                                let srcs = msg.sources.clone().unwrap_or_default();
                                view! {
                                    <div class=format!("flex {align}")>
                                        <div class="max-w-[80%]">
                                            <div class=format!("px-3 py-2 rounded-lg whitespace-pre-wrap {bubble}")>
                                                {msg.content}
                                            </div>
                                            {if !dims.is_empty() {
                                                Some(view! {
                                                    <div class="flex flex-wrap gap-1 mt-1 ml-1">
                                                        {dims.into_iter().map(|d| {
                                                            let color = dimension_color(&d);
                                                            view! {
                                                                <span class=format!("px-1.5 py-0.5 rounded text-[10px] font-medium {color}")>{d}</span>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                })
                                            } else { None }}
                                            <div class="flex items-center gap-2 mt-0.5 ml-1">
                                                {provider_label.map(|p| {
                                                    let label = if let Some(ref r) = route_label {
                                                        format!("via {p} ({r})")
                                                    } else {
                                                        format!("via {p}")
                                                    };
                                                    view! {
                                                        <span class="text-[10px] text-gray-400">{label}</span>
                                                    }
                                                })}
                                            </div>
                                            {if !srcs.is_empty() {
                                                Some(view! {
                                                    <div class="mt-1 ml-1 space-y-0.5">
                                                        {srcs.into_iter().take(3).map(|s| {
                                                            view! {
                                                                <div class="text-[10px] text-gray-400 truncate" title=s.excerpt.clone()>
                                                                    {"\u{1F4D6} "}{s.source}
                                                                </div>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                })
                                            } else { None }}
                                        </div>
                                    </div>
                                }
                            }
                        />

                        <Show when=move || is_loading.get()>
                            <div class="flex justify-start">
                                <div class="bg-gray-100 px-3 py-2 rounded-lg rounded-bl-none">
                                    <div class="flex space-x-1">
                                        <div class="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce"></div>
                                        <div class="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                                        <div class="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                                    </div>
                                </div>
                            </div>
                        </Show>
                    </div>

                    // Input
                    <form class="p-3 border-t border-gray-200" on:submit=handle_submit>
                        <div class="flex space-x-2">
                            <input
                                type="text"
                                placeholder={move || if is_creating.get() {
                                    "Describe your system..."
                                } else {
                                    "Ask about your model..."
                                }}
                                class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
                                prop:value=move || input_value.get()
                                on:input=move |ev| set_input_value.set(event_target_value(&ev))
                                prop:disabled=move || is_loading.get() || is_generating.get()
                            />
                            <button
                                type="submit"
                                class="bg-blue-500 hover:bg-blue-600 disabled:bg-gray-400 text-white px-3 py-2 rounded-lg"
                                prop:disabled=move || is_loading.get() || is_generating.get() || input_value.get().trim().is_empty()
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
                                </svg>
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </Show>
    }
}
