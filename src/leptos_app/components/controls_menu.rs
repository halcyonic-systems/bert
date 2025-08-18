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
                            // Navigation Controls
                            <div class="space-y-4">
                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"Navigation"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Pan" control="Right-click and drag" />
                                    <ControlItem action="Scroll" control="Mouse wheel" />
                                    <ControlItem action="Reset View" control="Ctrl+R" />
                                </div>

                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"Zoom"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Zoom Out" control="- key" />
                                    <ControlItem action="Zoom In" control="= key" />
                                </div>

                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"Selection"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Select Element" control="Left-click" />
                                    <ControlItem action="Multi-select" control="Shift + click" />
                                    <ControlItem action="Deselect All" control="Escape" />
                                </div>
                            </div>

                            // Element Management & File Operations
                            <div class="space-y-4">
                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"Element Management"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Move Elements" control="Click and drag" />
                                    <ControlItem action="Delete Elements" control="Delete / Backspace" />
                                    <ControlItem action="Hide Elements" control="H key" />
                                    <ControlItem action="Unhide Elements" control="U key" />
                                </div>

                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"File Operations"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Open File" control="Ctrl+L" />
                                    <ControlItem action="Save" control="Ctrl+S" />
                                    <ControlItem action="Save As..." control="Ctrl+Shift+S" />
                                </div>

                                <h3 class="text-lg font-semibold text-gray-800 border-b pb-2">"Advanced"</h3>
                                <div class="space-y-2">
                                    <ControlItem action="Sink/Source Equivalence" control="E key" />
                                    <ControlItem action="Toggle Background" control="Ctrl+Alt+B" />
                                </div>
                            </div>
                        </div>

                        <div class="mt-6 pt-4 border-t border-gray-200">
                            <p class="text-sm text-gray-600 text-center">
                                "Double-click on subsystems to enter them. Use toolbar buttons to create new elements."
                            </p>
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
