use crate::bevy_app::events::PaletteElementTypeEvent;
use leptos::prelude::*;

/// Floating palette in lower-left corner for placing system elements.
///
/// Compact horizontal layout with icon + label for each element type:
/// - Subsystem (internal component)
/// - Interface (boundary crossing)
/// - Environment (external entity)
///
/// Includes help button for systems science quick-reference and mode indicator.
#[component]
pub fn Palette(
    /// Callback when palette item is clicked - triggers placement mode
    on_element_click: Callback<PaletteElementTypeEvent>,
    /// Optional mode text to display (e.g., "Connection Mode (F)")
    #[prop(optional)]
    mode_text: Option<Signal<String>>,
) -> impl IntoView {
    let (help_open, set_help_open) = signal(false);

    view! {
        // Main palette container - upper-left quadrant
        <div class="fixed left-4 top-1/4 -translate-y-1/2 z-10 flex flex-col gap-2">
            // Mode indicator (above palette when active)
            {move || {
                mode_text.map(|text| {
                    let current_text = text.get();
                    if !current_text.is_empty() {
                        Some(view! {
                            <div class="px-3 py-1.5 bg-blue-100 text-blue-700 rounded-lg text-sm font-medium shadow-md">
                                {current_text}
                            </div>
                        })
                    } else {
                        None
                    }
                }).flatten()
            }}

            // Element buttons row
            <div class="flex items-center gap-1 bg-stone-50 border border-stone-200 rounded-lg shadow-lg p-1">
                <PaletteButton
                    element_type=PaletteElementTypeEvent::Subsystem
                    icon_path="assets/palette-icons/subsystem-icon.png"
                    label="Subsystem"
                    tooltip="Internal component - click to place inside system"
                    on_click=on_element_click
                />
                <div class="w-px h-8 bg-stone-300"></div>
                <PaletteButton
                    element_type=PaletteElementTypeEvent::Interface
                    icon_path="assets/palette-icons/interface-icon.png"
                    label="Interface"
                    tooltip="Boundary crossing - click to place on system boundary"
                    on_click=on_element_click
                />
                <div class="w-px h-8 bg-stone-300"></div>
                <PaletteButton
                    element_type=PaletteElementTypeEvent::EnvironmentalObject
                    icon_path="assets/palette-icons/source.png"
                    label="Environment"
                    tooltip="External entity - click to place in environment region"
                    on_click=on_element_click
                />
                <div class="w-px h-8 bg-stone-300"></div>
                // Help button
                <button
                    class="flex items-center justify-center w-8 h-8 rounded hover:bg-stone-100 active:bg-stone-200 transition-colors text-stone-500 hover:text-stone-700 font-bold"
                    title="Systems Science Quick Reference"
                    on:click=move |_| set_help_open.update(|v| *v = !*v)
                >
                    {"?"}
                </button>
            </div>
        </div>

        // Quick Reference Panel (slides up when help is open)
        <Show when=move || help_open.get()>
            <QuickReferencePanel on_close=Callback::new(move |_| set_help_open.set(false)) />
        </Show>
    }
}

#[component]
fn PaletteButton(
    element_type: PaletteElementTypeEvent,
    icon_path: &'static str,
    label: &'static str,
    tooltip: &'static str,
    on_click: Callback<PaletteElementTypeEvent>,
) -> impl IntoView {
    view! {
        <button
            class="flex items-center gap-2 px-3 py-2 rounded hover:bg-stone-100 active:bg-stone-200 transition-colors"
            title=tooltip
            on:click=move |_| {
                on_click.run(element_type);
            }
        >
            <img
                src=icon_path
                alt=label
                class="w-7 h-7"
            />
            <span class="text-sm text-stone-700 font-medium">{label}</span>
        </button>
    }
}

/// Quick reference panel showing systems science definitions for each element type.
/// Placeholder content to be refined in future sessions.
#[component]
fn QuickReferencePanel(on_close: Callback<()>) -> impl IntoView {
    view! {
        <div class="fixed left-72 top-1/4 -translate-y-1/2 z-20 w-80 bg-white border border-stone-200 rounded-lg shadow-xl">
            // Header
            <div class="flex items-center justify-between px-4 py-2 border-b border-stone-200 bg-stone-50 rounded-t-lg">
                <h3 class="font-semibold text-stone-800">{"Systems Science Reference"}</h3>
                <button
                    class="text-stone-400 hover:text-stone-600 text-lg font-bold"
                    on:click=move |_| on_close.run(())
                >
                    {"\u{00D7}"}
                </button>
            </div>

            // Content
            <div class="p-4 space-y-4 text-sm">
                // Subsystem
                <div>
                    <div class="flex items-center gap-2 mb-1">
                        <img src="assets/palette-icons/subsystem-icon.png" alt="Subsystem" class="w-5 h-5" />
                        <span class="font-semibold text-stone-800">{"Subsystem"}</span>
                    </div>
                    <p class="text-stone-600 pl-7">
                        {"A component within the system boundary that processes flows. Subsystems can be decomposed into their own internal structure."}
                    </p>
                    // TODO: Add Mobus citation/page reference
                </div>

                // Interface
                <div>
                    <div class="flex items-center gap-2 mb-1">
                        <img src="assets/palette-icons/interface-icon.png" alt="Interface" class="w-5 h-5" />
                        <span class="font-semibold text-stone-800">{"Interface"}</span>
                    </div>
                    <p class="text-stone-600 pl-7">
                        {"A boundary-crossing point where flows enter or exit the system. Interfaces mediate between internal processes and the environment."}
                    </p>
                    // TODO: Add Mobus citation/page reference
                </div>

                // Environment
                <div>
                    <div class="flex items-center gap-2 mb-1">
                        <img src="assets/palette-icons/source.png" alt="Environment" class="w-5 h-5" />
                        <span class="font-semibold text-stone-800">{"External Entity"}</span>
                    </div>
                    <p class="text-stone-600 pl-7">
                        {"Sources and sinks in the environment. Sources provide inputs to the system; sinks receive outputs from the system."}
                    </p>
                    // TODO: Add Mobus citation/page reference
                </div>

                // Footer hint
                <div class="pt-2 border-t border-stone-100 text-xs text-stone-400">
                    {"Based on Mobus & Kalton (2015) systems science framework"}
                </div>
            </div>
        </div>
    }
}
