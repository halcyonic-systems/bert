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
        <div class="flex items-center relative my-2 rounded-md">
            <input
                prop:checked=move || checked.get()
                class="col-start-1 row-start-1 appearance-none rounded border border-gray-300 bg-white hover:checked:bg-rose-600 checked:border-rose-600 checked:bg-rose-600 indeterminate:border-rose-600 indeterminate:bg-rose-600 focus-visible:bg-rose-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-rose-600 disabled:border-gray-300 disabled:bg-gray-100 disabled:checked:bg-gray-100 ring-rose-600 forced-colors:appearance-auto focus:checked:bg-rose-600 focus:outline focus:outline-2 focus:outline-offset-2 focus:outline-rose-600 "
                id=id.clone()
                type="checkbox"
                on:input=move |_| on_toggle(!checked.get_untracked())
            />
            <label for=id class="block pl-2  text-sm/6 font-medium text-gray-900">
                {label}
            </label>

        </div>
    }
}
