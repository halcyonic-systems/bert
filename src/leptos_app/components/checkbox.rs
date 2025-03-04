use leptos::prelude::*;

#[component]
pub fn Checkbox<T>(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(into)] checked: Signal<bool>,
    on_toggle: T,
) -> impl IntoView
where
    T: Fn(bool) + Clone + Send + 'static,
{
    view! {
        <div class="flex relative items-center my-2 rounded-md">
            <input
                prop:checked=move || checked.get()
                class="col-start-1 row-start-1 bg-white rounded border border-gray-300 ring-cyan-600 appearance-none checked:bg-cyan-600 checked:border-cyan-600 focus-visible:bg-cyan-600 disabled:bg-gray-100 disabled:border-gray-300 indeterminate:border-cyan-600 indeterminate:bg-cyan-600 forced-colors:appearance-auto hover:checked:bg-cyan-600 focus:checked:bg-cyan-600 focus:outline focus:outline-2 focus:outline-offset-2 focus:outline-cyan-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-cyan-600 disabled:checked:bg-gray-100"
                id=id.clone()
                type="checkbox"
                on:input=move |_| on_toggle(!checked.get_untracked())
            />
            <label for=id class="block pl-2 font-medium text-gray-900 text-sm/6">
                {label}
            </label>

        </div>
    }
}
