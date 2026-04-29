use leptos::prelude::*;
use leptos::ev;
use serde::{Deserialize, Serialize};
use tauri_sys::core::invoke_result;

#[derive(Serialize)]
struct ChatArgs {
    request: ChatRequest,
}

#[derive(Serialize)]
struct ChatRequest {
    message: String,
    model_context: String,
}

#[derive(Deserialize, Clone, Debug)]
struct ChatResponse {
    response: String,
    provider: String,
}

#[derive(Clone, Debug)]
struct ChatMessage {
    id: usize,
    content: String,
    is_user: bool,
}

#[component]
pub fn ChatPanel(
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] model_context: Signal<Option<String>>,
) -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<ChatMessage>::new());
    let (input_value, set_input_value) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (next_id, set_next_id) = signal(0usize);

    let alloc_id = move || {
        let id = next_id.get_untracked();
        set_next_id.set(id + 1);
        id
    };

    let send_message = move |_| {
        let message = input_value.get_untracked();
        if message.trim().is_empty() || is_loading.get_untracked() {
            return;
        }

        let uid = alloc_id();
        set_messages.update(|msgs| {
            msgs.push(ChatMessage { id: uid, content: message.clone(), is_user: true });
        });
        set_is_loading.set(true);
        set_input_value.set(String::new());

        leptos::task::spawn_local(async move {
            let ctx = model_context.get_untracked()
                .unwrap_or_else(|| r#"{"environment":{"info":{"name":"No model loaded"}},"systems":[],"interactions":[]}"#.to_string());

            let args = ChatArgs {
                request: ChatRequest {
                    message: message.clone(),
                    model_context: ctx,
                },
            };

            let response_text = match invoke_result::<ChatResponse, String>("chat_with_model", &args).await {
                Ok(resp) => resp.response,
                Err(e) => format!("Error: {e}"),
            };

            let rid = next_id.get_untracked();
            set_next_id.set(rid + 1);

            set_messages.update(|msgs| {
                msgs.push(ChatMessage { id: rid, content: response_text, is_user: false });
            });
            set_is_loading.set(false);
        });
    };

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        send_message(());
    };

    let has_model = Memo::new(move |_| model_context.get().is_some());

    view! {
        <Show when=move || visible.get()>
            <div class="absolute bottom-4 right-4 z-30 bg-white rounded-lg shadow-xl border border-gray-200 w-96 h-[28rem] flex flex-col">
                // Header
                <div class="bg-gray-50 rounded-t-lg p-3 border-b border-gray-200 flex items-center justify-between">
                    <div>
                        <h3 class="font-semibold text-gray-800 text-sm">
                            {move || if has_model.get() { "Chat with Model" } else { "Chat" }}
                        </h3>
                        <span class="text-xs text-gray-400">
                            {move || if has_model.get() { "Ask about your loaded model" } else { "Load a model for analysis" }}
                        </span>
                    </div>
                    <button
                        class="text-gray-400 hover:text-gray-600"
                        on:click=move |_| on_close.run(())
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
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
                            view! {
                                <div class=format!("flex {align}")>
                                    <div class=format!("max-w-[80%] px-3 py-2 rounded-lg whitespace-pre-wrap {bubble}")>
                                        {msg.content}
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
                            placeholder="Ask about your model..."
                            class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
                            prop:value=move || input_value.get()
                            on:input=move |ev| set_input_value.set(event_target_value(&ev))
                            prop:disabled=move || is_loading.get()
                        />
                        <button
                            type="submit"
                            class="bg-blue-500 hover:bg-blue-600 disabled:bg-gray-400 text-white px-3 py-2 rounded-lg"
                            prop:disabled=move || is_loading.get() || input_value.get().trim().is_empty()
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
                            </svg>
                        </button>
                    </div>
                </form>
            </div>
        </Show>
    }
}
