use leptos::children::Children;
use leptos::prelude::*;
use leptos::tachys::html::property::IntoProperty;
use leptos::web_sys::Event;
use leptos::{component, slot, view, IntoView};
use std::fmt::Debug;
use std::str::FromStr;

const BASE_LABEL_CLASS: &str = "block text-sm font-medium leading-6 text-gray-900";
const BASE_INPUT_CLASS: &str = "block w-full rounded-md border-0 py-1.5 text-gray-900 ring-1 \
                                ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 \
                                focus:ring-inset focus:ring-cyan-600 sm:text-sm sm:leading-6 \
                                disabled:cursor-not-allowed disabled:bg-gray-50 disabled:text-gray-500";
#[slot]
pub struct InputGroupIcon {
    children: Children,
}

// TODO: `on_input` -> return Option<I> instead and let client write error msg
#[component]
pub fn InputGroup<F, I>(
    id: &'static str,
    #[prop(optional)] label: Option<&'static str>,
    on_input: F,
    #[prop(into)] value: Signal<I>,
    #[prop(optional)] input_class: Option<&'static str>,
    #[prop(optional)] type_: Option<&'static str>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] label_class: Option<&'static str>,
    #[prop(into, optional)] error: Signal<String>,
    #[prop(optional)] input_group_icon: Option<InputGroupIcon>,
    #[prop(optional)] max_length: Option<usize>,
) -> impl IntoView
where
    F: Fn(I) + Clone + Send + 'static,
    I: FromStr + Clone + Default + Send + Sync + IntoProperty + 'static,
    Signal<I>: IntoProperty,
    <I as FromStr>::Err: Debug,
{
    let name = name.unwrap_or(id.to_string());
    let placeholder = placeholder.unwrap_or(label.unwrap_or(""));
    let label_class = format!("{} {}", BASE_LABEL_CLASS, label_class.unwrap_or(""));

    let has_icon = input_group_icon.is_some();
    let icon = input_group_icon.map(|icon| (icon.children)().into_view());
    let input_class = format!(
        "{} {} {}",
        BASE_INPUT_CLASS,
        input_class.unwrap_or(""),
        if has_icon { "pl-10" } else { "" }
    );

    let type_ = type_.unwrap_or("text");

    let label = label.map(|label| {
        view! {
            <label for=id class=label_class>
                {label}
            </label>
        }
    });

    view! {
        <div class="mb-2">
            {label} <div class="relative mt-2 rounded-md">
                <div
                    class="flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none"
                    class:hidden=!has_icon
                >
                    {icon}
                </div>
                <input
                    id=id
                    type=type_
                    step="any"
                    disabled=disabled
                    name=name
                    prop:value=value
                    class=input_class
                    placeholder=placeholder
                    on:input=move |ev: Event| {
                        ev.prevent_default();
                        if let Ok(v) = I::from_str(&event_target_value(&ev)) {
                            on_input(v);
                        } else {
                            on_input(value.get_untracked());
                        };
                    }
                    maxlength=max_length
                />
                <div>
                    <p class="mt-2 text-xs italic text-red-500">{error}</p>
                </div>
            </div>
        </div>
    }
}
