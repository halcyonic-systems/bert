use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AppMode {
    Landing,
    Creating,
    Describing,
    Editing,
}

#[component]
pub fn LandingScreen(
    #[prop(into)] on_describe: Callback<String>,
    #[prop(into)] on_scratch: Callback<()>,
    #[prop(into)] on_browse: Callback<()>,
    #[prop(into)] on_open_file: Callback<()>,
    #[prop(into)] on_open_recent: Callback<String>,
    #[prop(into)] tauri_available: bool,
    #[prop(into)] ollama_available: Signal<bool>,
    #[prop(into)] recent_models: Signal<Vec<(String, String)>>,
) -> impl IntoView {
    let (description, set_description) = signal(String::new());
    let (generating, set_generating) = signal(false);

    let on_generate = move |_| {
        let desc = description.get_untracked();
        if !desc.trim().is_empty() && !generating.get_untracked() {
            set_generating.set(true);
            on_describe.run(desc);
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();
            let desc = description.get_untracked();
            if !desc.trim().is_empty() && !generating.get_untracked() {
                set_generating.set(true);
                on_describe.run(desc);
            }
        }
    };

    view! {
        <div class="absolute inset-0 z-40 flex items-center justify-center bg-gray-50/80 backdrop-blur-sm">
            <div class="bg-white rounded-2xl shadow-2xl p-10 max-w-2xl w-full mx-4">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold text-gray-900 mb-1">"BERT"</h1>
                    <p class="text-sm text-gray-400">"Bounded Entity Reasoning Toolkit"</p>
                </div>

                <div class="grid grid-cols-2 gap-4 mb-6">
                    // Describe a System
                    <div class="col-span-2 p-5 rounded-xl border-2 border-gray-200 hover:border-blue-300 transition-colors">
                        <div class="flex items-center gap-2 mb-3">
                            <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
                            </svg>
                            <span class="font-medium text-gray-900">"Describe a System"</span>
                            {move || {
                                if !tauri_available {
                                    view! { <span class="text-xs text-gray-400 ml-auto">"Desktop only"</span> }.into_any()
                                } else if ollama_available.get() {
                                    view! { <span class="ml-auto w-2 h-2 rounded-full bg-green-400"></span> }.into_any()
                                } else {
                                    view! { <span class="text-xs text-amber-500 ml-auto">"Start Ollama to enable"</span> }.into_any()
                                }
                            }}
                        </div>
                        <div class="flex gap-2">
                            <input
                                type="text"
                                class="flex-1 px-4 py-2.5 rounded-lg border border-gray-300 focus:border-blue-400 focus:ring-1 focus:ring-blue-400 outline-none text-sm"
                                placeholder="e.g. \"a coffee shop\" or \"the United Nations\""
                                disabled=move || !tauri_available || !ollama_available.get() || generating.get()
                                prop:value=move || description.get()
                                on:input=move |ev| set_description.set(event_target_value(&ev))
                                on:keydown=on_keydown
                            />
                            <button
                                class="px-5 py-2.5 rounded-lg bg-blue-600 hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed text-white font-medium text-sm transition-colors"
                                disabled=move || !tauri_available || !ollama_available.get() || description.get().trim().is_empty() || generating.get()
                                on:click=on_generate
                            >
                                {move || if generating.get() { "Generating..." } else { "Generate" }}
                            </button>
                        </div>
                    </div>

                    // Start from Scratch
                    <button
                        class="p-5 rounded-xl border-2 border-gray-200 hover:border-blue-300 hover:bg-blue-50 transition-colors text-left"
                        on:click=move |_| on_scratch.run(())
                    >
                        <div class="flex items-center gap-2 mb-2">
                            <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                            </svg>
                            <span class="font-medium text-gray-900">"Start from Scratch"</span>
                        </div>
                        <p class="text-xs text-gray-500">"Blank canvas — build manually"</p>
                    </button>

                    // Browse Models
                    <button
                        class="p-5 rounded-xl border-2 border-gray-200 hover:border-blue-300 hover:bg-blue-50 transition-colors text-left"
                        on:click=move |_| on_browse.run(())
                    >
                        <div class="flex items-center gap-2 mb-2">
                            <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                            </svg>
                            <span class="font-medium text-gray-900">"Browse Models"</span>
                        </div>
                        <p class="text-xs text-gray-500">"Examples and saved models"</p>
                    </button>
                </div>

                // Open File link
                <div class="flex items-center justify-between text-sm text-gray-500 border-t border-gray-100 pt-4">
                    <button
                        class="hover:text-blue-600 transition-colors flex items-center gap-1.5"
                        on:click=move |_| on_open_file.run(())
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 19a2 2 0 01-2-2V7a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1M5 19h14a2 2 0 002-2v-5a2 2 0 00-2-2H9a2 2 0 00-2 2v5a2 2 0 01-2 2z" />
                        </svg>
                        "Open file..."
                    </button>

                    // Recent models
                    <div class="flex items-center gap-3">
                        {move || {
                            let models = recent_models.get();
                            if models.is_empty() {
                                None
                            } else {
                                Some(view! {
                                    <span class="text-xs text-gray-400">"Recent:"</span>
                                    {models.into_iter().take(3).map(|(name, path)| {
                                        let path_clone = path.clone();
                                        view! {
                                            <button
                                                class="text-xs text-blue-500 hover:text-blue-700 transition-colors truncate max-w-[120px]"
                                                title=path.clone()
                                                on:click=move |_| on_open_recent.run(path_clone.clone())
                                            >
                                                {name}
                                            </button>
                                        }
                                    }).collect_view()}
                                })
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
