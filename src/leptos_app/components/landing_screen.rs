use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AppMode {
    Landing,
    Creating,
    Editing,
}

#[component]
pub fn LandingScreen(
    #[prop(into)] on_load_model: Callback<()>,
    #[prop(into)] on_create_model: Callback<()>,
    #[prop(into)] tauri_available: bool,
) -> impl IntoView {
    view! {
        <div class="absolute inset-0 z-40 flex items-center justify-center bg-gray-50/80 backdrop-blur-sm">
            <div class="bg-white rounded-2xl shadow-2xl p-12 max-w-md w-full mx-4 text-center">
                <h1 class="text-3xl font-bold text-gray-900 mb-1">
                    "BERT"
                </h1>
                <p class="text-sm text-gray-400 mb-8">
                    "Bounded Entity Reasoning Toolkit"
                </p>

                <div class="space-y-3">
                    {if tauri_available { Some(view! {
                        <button
                            class="w-full px-6 py-4 rounded-xl bg-blue-600 hover:bg-blue-700 text-white font-medium transition-colors flex items-center justify-center gap-3"
                            on:click=move |_| on_create_model.run(())
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                            </svg>
                            "Create New Model"
                        </button>
                    }) } else { None }}

                    <button
                        class="w-full px-6 py-4 rounded-xl bg-white border-2 border-gray-200 hover:border-blue-300 hover:bg-blue-50 text-gray-700 font-medium transition-colors flex items-center justify-center gap-3"
                        on:click=move |_| on_load_model.run(())
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 19a2 2 0 01-2-2V7a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1M5 19h14a2 2 0 002-2v-5a2 2 0 00-2-2H9a2 2 0 00-2 2v5a2 2 0 01-2 2z" />
                        </svg>
                        "Load Existing Model"
                    </button>
                </div>
            </div>
        </div>
    }
}
