use leptos::prelude::*;
use std::fmt::Display;

/// Radio button group with optional per-option descriptions.
///
/// Use `option_descriptions` to add tooltips explaining each option.
/// The descriptions must be in the same order as the options.
#[component]
pub fn RadioGroup<Opt, F>(
    #[prop(into)] id: String,
    #[prop(into)] label: String,
    #[prop(into)] options: Vec<Opt>,
    #[prop(into)] selected: Signal<Option<Opt>>,
    on_change: F,
    /// Tooltip for the overall label (shown as ? icon)
    #[prop(optional, into)] tooltip: Option<String>,
    /// Per-option descriptions shown as tooltips on each radio label
    #[prop(optional)] option_descriptions: Option<Vec<&'static str>>,
) -> impl IntoView
where
    Opt: Display + Sync + Send + Clone + PartialEq + Eq + 'static,
    F: Fn(Opt) + Clone + 'static,
{
    let descriptions = option_descriptions.unwrap_or_default();

    view! {
        <div class="mb-2">
            <label class="flex items-center font-medium text-gray-900 text-sm/6">
                <span>{label}</span>
                {tooltip.as_ref().map(|tip| view! {
                    <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" title=tip.clone()>
                        "?"
                    </span>
                })}
            </label>
            <div class="flex flex-wrap justify-evenly gap-y-2 mt-2">
                {options
                    .into_iter()
                    .enumerate()
                    .map(|(i, option)| {
                        let option_clone = option.clone();
                        let on_change = on_change.clone();
                        let option_id = format!("{}-{}", id, option.to_string().to_lowercase().replace(' ', "-"));
                        let description = descriptions.get(i).copied();
                        view! {
                            <div class="flex items-center" title=description.unwrap_or("")>
                                <input
                                    type="radio"
                                    id=option_id.clone()
                                    name=id.clone()
                                    prop:checked=move || selected.get().as_ref() == Some(&option_clone)
                                    on:change={
                                        let option = option.clone();
                                        let on_change = on_change.clone();
                                        move |_| on_change(option.clone())
                                    }
                                    class="w-4 h-4 text-cyan-600 bg-white border-gray-300 focus:ring-cyan-600 focus:ring-2"
                                />
                                <label for=option_id class="pl-2 font-medium text-gray-900 text-sm/6">
                                    {option.to_string()}
                                </label>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
