use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// A simple toast notification component that displays temporarily and auto-hides
#[component]
pub fn Toast(
    #[prop(into)] message: Signal<String>,
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] on_hide: Callback<()>,
) -> impl IntoView {
    // Auto-hide after 4 seconds when visible becomes true
    Effect::new(move |_| {
        if visible.get() {
            let callback = on_hide.clone();
            // Use setTimeout via web_sys to auto-hide after 4 seconds
            let window = web_sys::window().unwrap();
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                callback.run(());
            }) as Box<dyn FnMut()>);
            
            window.set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(), 
                4000  // 4 seconds
            ).unwrap();
            
            closure.forget(); // Keep closure alive for the timeout duration
        }
    });

    view! {
        <Show when=move || visible.get()>
            <div class="fixed top-4 right-4 z-50 bg-green-500 text-white px-6 py-3 rounded-lg shadow-lg transition-all duration-300 ease-in-out">
                <div class="flex items-center justify-between gap-4">
                    <div class="flex items-center gap-2">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                        </svg>
                        <span>{move || message.get()}</span>
                    </div>
                    <button
                        class="text-white hover:text-gray-200 ml-2"
                        on:click=move |_| on_hide.run(())
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                    </button>
                </div>
            </div>
        </Show>
    }
}