use crate::bevy_app::events::PaletteElementTypeEvent;
use leptos::prelude::*;

/// Fixed left-side palette panel for placing system elements.
///
/// Provides always-visible toolbox with 3 draggable element types:
/// - Subsystem
/// - Interface
/// - Environmental Object (source/sink)
///
/// Replaces Bevy world-space palette sprites with professional fixed UI.
#[component]
pub fn Palette(
    /// Callback when palette item is clicked - triggers placement mode
    on_element_click: Callback<PaletteElementTypeEvent>,
) -> impl IntoView {
    view! {
        <div class="fixed left-0 top-20 bottom-0 w-16 bg-stone-50 shadow-md z-10 flex flex-col items-center py-3 space-y-2">
            <PaletteButton
                element_type=PaletteElementTypeEvent::Subsystem
                icon_path="assets/palette-icons/subsystem-icon.png"
                tooltip="Subsystem: A component with sufficient complexity to warrant further deconstruction. Click to place inside the focused system."
                on_click=on_element_click
            />
            <PaletteButton
                element_type=PaletteElementTypeEvent::Interface
                icon_path="assets/palette-icons/interface-icon.png"
                tooltip="Interface: A boundary crossing point that acts as a pass-way for inputs and outputs. Click to place on the system boundary."
                on_click=on_element_click
            />
            <PaletteButton
                element_type=PaletteElementTypeEvent::EnvironmentalObject
                icon_path="assets/palette-icons/source.png"
                tooltip="External Entity: A source (provides inputs) or sink (receives outputs) in the environment. Unmodeled in terms of internal workings."
                on_click=on_element_click
            />
        </div>
    }
}

#[component]
fn PaletteButton(
    element_type: PaletteElementTypeEvent,
    icon_path: &'static str,
    tooltip: &'static str,
    on_click: Callback<PaletteElementTypeEvent>,
) -> impl IntoView {
    view! {
        <button
            class="w-12 h-12 rounded hover:bg-gray-100 active:bg-gray-200 transition-colors flex items-center justify-center"
            title=tooltip
            on:click=move |_| {
                on_click.run(element_type);
            }
        >
            <img
                src=icon_path
                alt=format!("{:?}", element_type)
                class="w-9 h-9"
            />
        </button>
    }
}
