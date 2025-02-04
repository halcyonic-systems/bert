use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use crate::events::TreeEvent;

struct SvgSystem {
    label: String,
    children: Vec<SvgSystem>,
    level: i32,
}

#[component]
pub fn Tree(
    #[prop(into)] visible: Signal<bool>,
     event_receiver: LeptosEventReceiver<TreeEvent>,
) -> impl IntoView {
    event_receiver.read().as_ref().map(|TreeEvent { world_model }| {
        // TODO : build nested `SvgSystem` from `world_model`

        view! {
            <Show when=move || visible.get()>
                <h2>"Tree"</h2>
                <svg></svg>
            </Show>
        }
    });
}

#[component]
pub fn SystemNode(system: SvgSystem) -> impl IntoView {
    // TODO : compute size from level

    // TODO : recursive over children?
    view! {
        <rect r="5" width="100" height="100">
            <text>{system.label}</text>
        </rect>
    }
}