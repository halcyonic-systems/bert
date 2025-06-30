use leptos::*;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use js_sys;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Check if we're in a Tauri environment
fn is_tauri_environment() -> bool {
    // Simple check: if window.__TAURI__ exists, we're in Tauri
    web_sys::window()
        .map(|window| {
            js_sys::Reflect::has(&window, &"__TAURI__".into()).unwrap_or(false)
        })
        .unwrap_or(false)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ChatRequest {
    message: String,
    #[serde(rename = "modelData")]
    model_data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ChatResponse {
    response: String,
    error: Option<String>,
}

#[derive(Clone, Debug)]
struct ChatMessage {
    content: String,
    is_user: bool,
}

#[component]
pub fn ChatPanel() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<ChatMessage>::new());
    let (input_value, set_input_value) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (is_open, set_is_open) = signal(false);

    let send_message = move |message: String| {
        if message.trim().is_empty() || is_loading.get() {
            return;
        }

        // Immediately add user message and set loading state
        set_messages.update(|msgs| {
            msgs.push(ChatMessage {
                content: message.clone(),
                is_user: true,
            });
        });
        
        set_is_loading.set(true);
        set_input_value.set(String::new());

        // Spawn the async task
        spawn_local(async move {
            // Get current model data - try to get from Tauri first, fallback to placeholder
            let model_data = if is_tauri_environment() {
                // Try to get current model data from Tauri
                let result = invoke("get_current_model", serde_wasm_bindgen::to_value(&()).unwrap()).await;
                match serde_wasm_bindgen::from_value::<Option<String>>(result) {
                    Ok(Some(data)) => data,
                    _ => r#"{"environment":{"info":{"name":"Sample System"},"systems":[],"interactions":[]}}"#.to_string()
                }
            } else {
                // Fallback to placeholder if not in Tauri environment
                r#"{"environment":{"info":{"name":"Sample System"},"systems":[],"interactions":[]}}"#.to_string()
            };
            
            let response_text = if is_tauri_environment() {
                // Desktop app: Use Tauri backend
                let chat_request = ChatRequest {
                    message: message.clone(),
                    model_data,
                };
                
                match serde_wasm_bindgen::to_value(&chat_request) {
                    Ok(args) => {
                        let response_result = invoke("chat_with_model", args).await;
                        match serde_wasm_bindgen::from_value::<ChatResponse>(response_result) {
                            Ok(chat_response) => {
                                web_sys::console::log_1(&format!("Chat response received: {}", chat_response.response).into());
                                chat_response.response
                            },
                            Err(e) => {
                                web_sys::console::error_1(&format!("Failed to parse chat response: {:?}", e).into());
                                "Sorry, there was an error parsing the response.".to_string()
                            }
                        }
                    },
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to serialize request: {:?}", e).into());
                        "Sorry, there was an error preparing the request.".to_string()
                    }
                }
            } else {
                // Web browser: Use mock responses
                generate_web_mock_response(&message, &model_data)
            };
            
            // Add bot response and clear loading state
            set_messages.update(|msgs| {
                msgs.push(ChatMessage {
                    content: response_text,
                    is_user: false,
                });
            });
            
            set_is_loading.set(false);
            web_sys::console::log_1(&"Chat response completed".into());
        });
    };
    
    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let message = input_value.get();
        if !message.trim().is_empty() && !is_loading.get() {
            send_message(message);
        }
    };
    
    let toggle_chat = move |_| {
        set_is_open.update(|open| *open = !*open);
    };

    view! {
        <div class="fixed bottom-4 right-4 z-50">
            // Chat toggle button
            <button
                class="bg-blue-500 hover:bg-blue-600 text-white rounded-full p-3 shadow-lg mb-2"
                on:click=toggle_chat
            >
                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                </svg>
            </button>
            
            // Chat panel
            <div class=move || format!("bg-white rounded-lg shadow-xl border border-gray-200 w-80 h-96 flex flex-col transition-all duration-300 {}", 
                if is_open.get() { "opacity-100 scale-100" } else { "opacity-0 scale-95 pointer-events-none" }
            )>
                // Chat header
                <div class="bg-gray-50 rounded-t-lg p-3 border-b border-gray-200">
                    <div class="flex items-center justify-between">
                        <h3 class="font-semibold text-gray-800">
                            {if is_tauri_environment() { "Chat with Model" } else { "Chat Demo (Web)" }}
                        </h3>
                        <button 
                            class="text-gray-500 hover:text-gray-700"
                            on:click=toggle_chat
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                </div>
                
                // Messages area
                <div class="flex-1 overflow-y-auto p-3 space-y-2">
                    <For
                        each=move || messages.get()
                        key=|message| format!("{}-{}", message.content, message.is_user)
                        children=move |message| {
                            view! {
                                <div class=format!("flex {}", if message.is_user { "justify-end" } else { "justify-start" })>
                                    <div class=format!("max-w-xs px-3 py-2 rounded-lg text-sm {}", 
                                        if message.is_user { 
                                            "bg-blue-500 text-white rounded-br-none" 
                                        } else { 
                                            "bg-gray-100 text-gray-800 rounded-bl-none" 
                                        }
                                    )>
                                        {message.content}
                                    </div>
                                </div>
                            }
                        }
                    />
                    
                    // Loading indicator
                    <Show when=move || is_loading.get()>
                        <div class="flex justify-start">
                            <div class="bg-gray-100 text-gray-800 px-3 py-2 rounded-lg rounded-bl-none text-sm">
                                <div class="flex space-x-1">
                                    <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                                    <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                                    <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                                </div>
                            </div>
                        </div>
                    </Show>
                </div>
                
                // Input area
                <form class="p-3 border-t border-gray-200" on:submit=handle_submit>
                    <div class="flex space-x-2">
                        <input
                            type="text"
                            placeholder=if is_tauri_environment() { "Ask about your model..." } else { "Try the demo (web version)..." }
                            class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent text-sm"
                            prop:value=move || input_value.get()
                            on:input=move |ev| set_input_value.set(event_target_value(&ev))
                            prop:disabled=move || is_loading.get()
                        />
                        <button
                            type="submit"
                            class="bg-blue-500 hover:bg-blue-600 disabled:bg-gray-400 text-white px-3 py-2 rounded-lg transition-colors"
                            prop:disabled=move || is_loading.get() || input_value.get().trim().is_empty()
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
                            </svg>
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}

// Mock responses for web version
fn generate_web_mock_response(question: &str, _model_context: &str) -> String {
    let question_lower = question.to_lowercase();
    
    if question_lower.contains("what") && question_lower.contains("system") {
        "This is a demo system model for the web version. The chat feature works fully in the desktop app with real model data.".to_string()
    } else if question_lower.contains("how many") {
        "In the web demo, we show placeholder data. Download the desktop app for full functionality with your actual models.".to_string()
    } else if question_lower.contains("overview") || question_lower.contains("summary") {
        "This is the web demo of BERT's chat feature. For full model analysis, please use the desktop application which can access your local model files.".to_string()
    } else if question_lower.contains("demo") || question_lower.contains("test") {
        "Great! The chat interface is working. This is a simple demo for the web version. The desktop app provides full model analysis with your real BERT models.".to_string()
    } else {
        format!(
            "I received your message: '{}'. This is the web demo version. For full chat functionality with your actual system models, please use the desktop app!",
            question
        )
    }
} 