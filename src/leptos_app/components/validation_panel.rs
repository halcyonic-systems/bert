use crate::bevy_app::data_model::validate::{Severity, ValidationIssue};
use leptos::prelude::*;

#[component]
pub fn ValidationPanel(
    #[prop(into)] issues: Signal<Option<Vec<ValidationIssue>>>,
    #[prop(into)] on_continue: Callback<()>,
    #[prop(into)] on_dismiss: Callback<()>,
) -> impl IntoView {
    let has_errors = Memo::new(move |_| {
        issues
            .get()
            .as_ref()
            .map(|v| v.iter().any(|i| i.severity == Severity::Error))
            .unwrap_or(false)
    });

    let error_count = Memo::new(move |_| {
        issues
            .get()
            .as_ref()
            .map(|v| v.iter().filter(|i| i.severity == Severity::Error).count())
            .unwrap_or(0)
    });

    let warning_count = Memo::new(move |_| {
        issues
            .get()
            .as_ref()
            .map(|v| v.iter().filter(|i| i.severity == Severity::Warning).count())
            .unwrap_or(0)
    });

    let copy_all = move |_| {
        if let Some(issues) = issues.get() {
            let text = issues
                .iter()
                .map(|i| {
                    let sev = match i.severity {
                        Severity::Error => "ERROR",
                        Severity::Warning => "WARNING",
                    };
                    let mut line = format!("[{sev}] {}: {}", i.location, i.message);
                    if let Some(ref s) = i.suggestion {
                        line.push_str(&format!("\n  Suggestion: {s}"));
                    }
                    line
                })
                .collect::<Vec<_>>()
                .join("\n\n");

            if let Some(window) = web_sys::window() {
                let _ = window.navigator().clipboard().write_text(&text);
            }
        }
    };

    view! {
        <Show when=move || issues.get().is_some()>
            <div class="fixed inset-0 bg-black bg-opacity-50 z-30 flex items-center justify-center">
                <div class="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[80vh] overflow-hidden m-4 flex flex-col">
                    <div class="p-5 border-b border-gray-200 flex justify-between items-center shrink-0">
                        <div>
                            <h2 class="text-lg font-bold text-gray-900">
                                {move || if has_errors.get() { "Model Validation Errors" } else { "Model Validation Warnings" }}
                            </h2>
                            <p class="text-sm text-gray-500 mt-1">
                                {move || {
                                    let e = error_count.get();
                                    let w = warning_count.get();
                                    match (e, w) {
                                        (0, w) => format!("{w} warning{}", if w != 1 { "s" } else { "" }),
                                        (e, 0) => format!("{e} error{}", if e != 1 { "s" } else { "" }),
                                        (e, w) => format!("{e} error{}, {w} warning{}",
                                            if e != 1 { "s" } else { "" },
                                            if w != 1 { "s" } else { "" }),
                                    }
                                }}
                            </p>
                        </div>
                        <button
                            class="text-gray-500 hover:text-gray-700 bg-gray-100 hover:bg-gray-200 px-3 py-1 rounded text-sm"
                            on:click=copy_all
                        >
                            "Copy All"
                        </button>
                    </div>

                    <div class="overflow-y-auto flex-1 p-4 space-y-3">
                        {move || {
                            issues.get().unwrap_or_default().into_iter().map(|issue| {
                                let is_error = issue.severity == Severity::Error;
                                let badge_class = if is_error {
                                    "bg-red-100 text-red-700 border-red-200"
                                } else {
                                    "bg-amber-100 text-amber-700 border-amber-200"
                                };
                                let badge_text = if is_error { "Error" } else { "Warning" };
                                let suggestion = issue.suggestion.clone();

                                view! {
                                    <div class="border rounded-lg p-3 bg-gray-50">
                                        <div class="flex items-start gap-2">
                                            <span class={format!("inline-block px-2 py-0.5 rounded text-xs font-medium border shrink-0 {badge_class}")}>
                                                {badge_text}
                                            </span>
                                            <div class="min-w-0">
                                                <code class="text-xs font-mono text-gray-600 break-all">
                                                    {issue.location.clone()}
                                                </code>
                                                <p class="text-sm text-gray-800 mt-1">{issue.message.clone()}</p>
                                                {suggestion.map(|s| view! {
                                                    <p class="text-xs italic text-gray-500 mt-1">{s}</p>
                                                })}
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>

                    <div class="p-4 border-t border-gray-200 flex justify-end gap-3 shrink-0">
                        <button
                            class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-md"
                            on:click=move |_| on_dismiss.run(())
                        >
                            "Dismiss"
                        </button>
                        <Show when=move || !has_errors.get()>
                            <button
                                class="px-4 py-2 text-sm font-medium text-white bg-green-600 hover:bg-green-700 rounded-md"
                                on:click=move |_| on_continue.run(())
                            >
                                "Continue Anyway"
                            </button>
                        </Show>
                    </div>
                </div>
            </div>
        </Show>
    }
}
