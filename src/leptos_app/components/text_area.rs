use leptos::prelude::*;

#[component]
pub fn TextArea<F>(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(into, default = String::new())] placeholder: String,
    #[prop(into)] text: Signal<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    /// Tooltip for the label (shown as ? icon)
    #[prop(optional, into)]
    tooltip: Option<String>,
    on_input: F,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + 'static,
{
    view! {
        <div class="mb-2">
            <label for=id.clone() class="flex items-center font-medium text-gray-900 text-sm/6">
                <span>{label.clone()}</span>
                {tooltip.as_ref().map(|tip| view! {
                    <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" title=tip.clone()>
                        "?"
                    </span>
                })}
            </label>
            <div class="mt-2">
                <textarea
                    rows="4"
                    name=label
                    id=id
                    prop:disabled=disabled
                    placeholder=placeholder
                    class="block py-1.5 px-3 w-full text-base text-gray-900 bg-white rounded-md focus:ring-cyan-600 outline outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 sm:text-sm/6 focus:outline focus:outline-2 focus:-outline-offset-2 focus:outline-cyan-600"
                    on:input=move |ev| on_input(event_target_value(&ev))
                >
                    {move || text.get()}
                </textarea>
            </div>
        </div>
    }
}
