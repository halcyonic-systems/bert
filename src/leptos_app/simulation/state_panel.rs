use leptos::prelude::*;

use super::types::SimulationResults;

#[component]
pub fn StatePanel(results: Signal<Option<SimulationResults>>) -> impl IntoView {
    let rows = Memo::new(move |_| {
        results
            .get()
            .map(|res| {
                let series = &res.system_timeseries;
                if series.is_empty() {
                    return vec![];
                }

                let max_val = series
                    .iter()
                    .filter_map(|s| s.values.last().copied())
                    .fold(0.0_f64, f64::max)
                    .max(0.001);

                series
                    .iter()
                    .map(|s| {
                        let current = s.values.last().copied().unwrap_or(0.0);
                        let prev = if s.values.len() >= 2 {
                            s.values[s.values.len() - 2]
                        } else {
                            0.0
                        };
                        let direction = if (current - prev).abs() < max_val * 0.01 {
                            0 // stable
                        } else if current > prev {
                            1 // growing
                        } else {
                            -1 // shrinking
                        };
                        let bar_pct = (current / max_val * 100.0).min(100.0);
                        (
                            format!("{} — {}", s.name, s.key),
                            current,
                            bar_pct,
                            direction,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    });

    view! {
        <div class="h-full overflow-y-auto px-4 py-2">
            {move || {
                let r = rows.get();
                if r.is_empty() {
                    view! {
                        <div class="text-xs text-gray-400 italic py-4 text-center">
                            {"No simulation data"}
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="space-y-1">
                            <For
                                each=move || rows.get().into_iter().enumerate()
                                key=|(i, _)| *i
                                children=move |(_, (label, value, bar_pct, direction))| {
                                    let bar_color = match direction {
                                        1 => "#22c55e",
                                        -1 => "#ef4444",
                                        _ => "#9ca3af",
                                    };
                                    let dir_icon = match direction {
                                        1 => "\u{25B2}",
                                        -1 => "\u{25BC}",
                                        _ => "\u{25C6}",
                                    };
                                    let label_title = label.clone();
                                    view! {
                                        <div class="flex items-center gap-2 py-0.5">
                                            <span class="text-xs text-gray-600 w-40 truncate flex-shrink-0"
                                                  title=label_title>
                                                {label}
                                            </span>
                                            <div class="flex-1 bg-gray-100 rounded h-4 relative overflow-hidden">
                                                <div
                                                    class="h-full rounded transition-all duration-300"
                                                    style=format!(
                                                        "width: {}%; background-color: {};",
                                                        bar_pct, bar_color
                                                    )
                                                />
                                            </div>
                                            <span class="text-xs font-mono text-gray-500 w-16 text-right flex-shrink-0">
                                                {format!("{value:.2}")}
                                            </span>
                                            <span class="text-xs w-3 flex-shrink-0"
                                                  style=format!("color: {};", bar_color)>
                                                {dir_icon}
                                            </span>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
