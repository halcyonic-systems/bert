use leptos::prelude::*;

#[component]
pub fn TextArea<F>(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into)] text: Signal<String>,
    on_input: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + 'static,
{
    view! {
        <div class="mb-2">
            <label for=id.clone() class="block text-sm/6 font-medium text-gray-900">
                {label.clone()}
            </label>
            <div class="mt-2">
                <textarea
                    rows="4"
                    name=label
                    id=id
                    placeholder=placeholder
                    class="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline focus:outline-2 focus:ring-rose-600 focus:-outline-offset-2 focus:outline-rose-600 sm:text-sm/6"
                    on:input=move |ev| on_input(event_target_value(&ev))
                >
                    {move || text.get()}
                </textarea>
            </div>
        </div>
    }
}
