use leptos::prelude::*;
use std::fmt::Display;

#[component]
pub fn SelectGroup<Opt, F>(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(into)] options: Vec<Opt>,
    #[prop(into)] selected_option: Signal<Option<Opt>>,
    on_change: F,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView
where
    Opt: Display + Sync + Send + Clone + PartialEq + Eq + 'static,
    F: Fn(Option<Opt>) + Clone + 'static,
{
    let select_option = {
        let options = options.clone();

        move |option: String| {
            options
                .clone()
                .into_iter()
                .find(|opt| *opt.to_string() == option)
        }
    };

    let is_selected = Signal::derive({
        let options = options.clone();
        move || {
            options
                .iter()
                .find(|opt| {
                    selected_option
                        .get()
                        .is_some_and(|selected| **opt == selected)
                })
                .is_some()
        }
    });

    view! {
        <div class="mb-2">
            <label for=id.clone() class="block text-sm/6 font-medium text-gray-900">
                {label.clone()}
            </label>
            <div class="mt-2 grid grid-cols-1">
                <select
                    prop:disabled=disabled
                    id=id
                    name=label
                    on:change=move |ev| on_change(select_option(event_target_value(&ev)))
                    class="col-start-1 row-start-1 w-full appearance-none rounded-md bg-white py-1.5 pl-3 pr-8 text-base text-gray-900 outline outline-1 -outline-offset-1 outline-gray-300 focus:ring-1 focus:ring-rose-600 focus:outline focus:outline-2 focus:-outline-offset-2 focus:outline-rose-600 sm:text-sm/6"
                >
                    <Show when=move || selected_option.get().is_none()>
                        <option disabled selected value>
                            -
                        </option>
                    </Show>
                    {options
                        .iter()
                        .map(|option| {
                            view! { <option selected=is_selected>{option.to_string()}</option> }
                        })
                        .collect_view()}
                </select>
            </div>
        </div>
    }
}
