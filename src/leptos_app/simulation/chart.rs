use leptos::prelude::*;

#[component]
pub fn LineChart(
    label: String,
    ticks: Vec<u64>,
    values: Vec<f64>,
    #[prop(default = "rgb(59,130,246)".to_string())] color: String,
    #[prop(default = 260)] width: u32,
    #[prop(default = 120)] height: u32,
) -> impl IntoView {
    if ticks.is_empty() || values.is_empty() {
        return view! { <div class="text-xs text-gray-400">{"No data"}</div> }.into_any();
    }

    let pad_l = 40u32;
    let pad_r = 8u32;
    let pad_t = 20u32;
    let pad_b = 24u32;
    let plot_w = width.saturating_sub(pad_l + pad_r) as f64;
    let plot_h = height.saturating_sub(pad_t + pad_b) as f64;

    let x_min = *ticks.iter().min().unwrap() as f64;
    let x_max = *ticks.iter().max().unwrap() as f64;
    let y_min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let x_range = if (x_max - x_min).abs() < 1e-10 { 1.0 } else { x_max - x_min };
    let y_range = if (y_max - y_min).abs() < 1e-10 { 1.0 } else { y_max - y_min };

    let to_x = move |t: f64| pad_l as f64 + (t - x_min) / x_range * plot_w;
    let to_y = move |v: f64| pad_t as f64 + plot_h - (v - y_min) / y_range * plot_h;

    let points: String = ticks.iter().zip(values.iter())
        .map(|(t, v)| format!("{:.1},{:.1}", to_x(*t as f64), to_y(*v)))
        .collect::<Vec<_>>()
        .join(" ");

    let y_mid = y_min + y_range / 2.0;
    let fmt = |v: f64| -> String {
        if v.abs() >= 1000.0 { format!("{:.0}", v) }
        else if v.abs() >= 1.0 { format!("{:.1}", v) }
        else { format!("{:.3}", v) }
    };

    let y_min_str = fmt(y_min);
    let y_max_str = fmt(y_max);
    let y_mid_str = fmt(y_mid);
    let x_min_str = format!("{}", ticks.first().unwrap());
    let x_max_str = format!("{}", ticks.last().unwrap());

    let y_min_y = to_y(y_min);
    let y_max_y = to_y(y_max);
    let y_mid_y = to_y(y_mid);

    let vb = format!("0 0 {} {}", width, height);
    let color2 = color.clone();

    view! {
        <div class="mb-2">
            <div class="text-xs font-medium text-gray-600 mb-0.5">{label}</div>
            <svg viewBox={vb} class="w-full" style={format!("max-height: {}px", height)}>
                // Y axis grid lines
                <line x1={pad_l.to_string()} y1={y_min_y.to_string()} x2={(width - pad_r).to_string()} y2={y_min_y.to_string()} stroke="#e5e7eb" stroke-width="0.5"/>
                <line x1={pad_l.to_string()} y1={y_mid_y.to_string()} x2={(width - pad_r).to_string()} y2={y_mid_y.to_string()} stroke="#e5e7eb" stroke-width="0.5" stroke-dasharray="3,3"/>
                <line x1={pad_l.to_string()} y1={y_max_y.to_string()} x2={(width - pad_r).to_string()} y2={y_max_y.to_string()} stroke="#e5e7eb" stroke-width="0.5"/>

                // Y axis labels
                <text x={(pad_l - 4).to_string()} y={y_max_y.to_string()} text-anchor="end" font-size="8" fill="#9ca3af" dominant-baseline="middle">{y_max_str}</text>
                <text x={(pad_l - 4).to_string()} y={y_mid_y.to_string()} text-anchor="end" font-size="8" fill="#9ca3af" dominant-baseline="middle">{y_mid_str}</text>
                <text x={(pad_l - 4).to_string()} y={y_min_y.to_string()} text-anchor="end" font-size="8" fill="#9ca3af" dominant-baseline="middle">{y_min_str}</text>

                // X axis labels
                <text x={pad_l.to_string()} y={(height - 4).to_string()} text-anchor="start" font-size="8" fill="#9ca3af">{x_min_str}</text>
                <text x={(width - pad_r).to_string()} y={(height - 4).to_string()} text-anchor="end" font-size="8" fill="#9ca3af">{x_max_str}</text>

                // Data line
                <polyline points={points} fill="none" stroke={color} stroke-width="1.5" stroke-linejoin="round"/>

                // Last value dot
                {ticks.last().zip(values.last()).map(|(t, v)| {
                    let cx = to_x(*t as f64);
                    let cy = to_y(*v);
                    view! {
                        <circle cx={cx.to_string()} cy={cy.to_string()} r="2.5" fill={color2.clone()}/>
                    }
                })}
            </svg>
        </div>
    }.into_any()
}
