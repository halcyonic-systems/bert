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
    #[prop(optional, into)] tooltip: Option<String>,
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
            <div class="grid grid-cols-1 mt-2">
                <select
                    prop:disabled=disabled
                    id=id
                    name=label
                    on:change=move |ev| on_change(select_option(event_target_value(&ev)))
                    class="col-start-1 row-start-1 py-1.5 pr-8 pl-3 w-full text-base text-gray-900 bg-white rounded-md appearance-none focus:ring-1 focus:ring-cyan-600 outline outline-1 -outline-offset-1 outline-gray-300 sm:text-sm/6 focus:outline focus:outline-2 focus:-outline-offset-2 focus:outline-cyan-600"
                >
                    <Show when=move || selected_option.get().is_none()>
                        <option disabled selected value>
                            -
                        </option>
                    </Show>
                    {options
                        .clone()
                        .into_iter()
                        .map(|option| {
                            view! {
                                <option prop:selected=move || {
                                    option.to_string()
                                        == selected_option
                                            .get()
                                            .map(|opt| opt.to_string())
                                            .unwrap_or("".to_string())
                                }>{option.to_string()}</option>
                            }
                        })
                        .collect_view()}
                </select>
            </div>
        </div>
    }
}
