use leptos::prelude::*;

#[component]
pub fn Divider(#[prop(into, optional)] name: Option<String>) -> impl IntoView {
    let show_name = move || {
        name.clone().map(|n| {
            view! {
                <div class="relative flex justify-center">
                    <span class="bg-white px-2 text-sm text-gray-500">{n}</span>
                </div>
            }
        })
    };

    view! {
        <div class="relative my-4">
            <div class="absolute inset-0 flex items-center" aria-hidden="true">
                <div class="w-full border-t border-1 border-gray-300"></div>
            </div>
            {show_name}
        </div>
    }
}
