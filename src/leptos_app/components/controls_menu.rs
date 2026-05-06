use leptos::prelude::*;

#[component]
pub fn ControlsMenu(
    #[prop(into)] visible: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    view! {
        <Show when=move || visible.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 z-30 flex items-center justify-center">
                <div class="bg-white rounded-lg shadow-xl max-w-4xl max-h-[90vh] overflow-y-auto m-4">
                    <div class="p-6">
                        <div class="flex justify-between items-center mb-6">
                            <h2 class="text-2xl font-bold text-gray-900">"BERT Controls"</h2>
                            <button
                                class="text-gray-400 hover:text-gray-600 text-2xl font-bold"
                                on:click=move |_| on_close.run(())
                            >
                                "×"
                            </button>
                        </div>

                        <div class="grid md:grid-cols-2 gap-6">
                            <div class="space-y-4">
                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"Navigation"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Pan" control="Right-click drag" />
                                    <ControlItem action="Scroll" control="Mouse wheel (⌘/Ctrl+scroll to zoom)" />
                                    <ControlItem action="Zoom In / Out" control="= / -" />
                                    <ControlItem action="Reset View" control="⌘R" />
                                </div>

                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"Selection"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Select" control="Left-click" />
                                    <ControlItem action="Multi-select" control="Shift+click" />
                                    <ControlItem action="Deselect All" control="Escape" />
                                </div>

                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"File"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Open" control="⌘L" />
                                    <ControlItem action="Save" control="⌘S" />
                                    <ControlItem action="Save As" control="⌘⇧S" />
                                    <ControlItem action="Screenshot" control="⌘P" />
                                </div>
                            </div>

                            <div class="space-y-4">
                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"Modeling"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Add Subsystem / Interface" control="Drag from palette" />
                                    <ControlItem action="Create Interface Processor" control="I (select interface first)" />
                                    <ControlItem action="Connect (Flow mode)" control="F (click source, then target)" />
                                    <ControlItem action="Set Equivalence" control="E" />
                                    <ControlItem action="Enter Subsystem" control="Double-click" />
                                </div>

                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"Editing"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Move" control="Click and drag" />
                                    <ControlItem action="Delete" control="Delete / Backspace" />
                                    <ControlItem action="Hide / Unhide" control="H / U" />
                                    <ControlItem action="Undo / Redo" control="⌘Z / ⌘⇧Z" />
                                    <ControlItem action="Toggle Background" control="⌘⌥B" />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[component]
fn ControlItem(action: &'static str, control: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center py-1">
            <span class="text-gray-700 text-sm">{action}</span>
            <code class="bg-gray-100 px-2 py-1 rounded text-xs font-mono text-gray-800">
                {control}
            </code>
        </div>
    }
}
