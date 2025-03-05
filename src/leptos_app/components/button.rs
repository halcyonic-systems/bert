use leptos::prelude::*;

#[component]
pub fn Button<T>(#[prop(into)] text: Signal<String>, on_click: T) -> impl IntoView
where
    T: Fn() + Clone + Send + 'static,
{
    view! {
        <button
            on:click=move |_| on_click()
            type="button"
            class="py-2 px-3.5 text-sm font-semibold text-white bg-cyan-600 rounded-full shadow-sm hover:bg-cyan-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-cyan-600"
        >
            {text}
        </button>
    }
}
