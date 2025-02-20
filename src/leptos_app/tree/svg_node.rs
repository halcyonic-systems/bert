use crate::data_model::Complexity;
use leptos::prelude::*;

#[component]
pub fn SvgNode(
    type_: Complexity,
    label: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    font_size: &'static str,
) -> impl IntoView {
    let is_complex = move || matches!(type_, Complexity::Complex { .. });

    view! {
        <SvgLine x1={x + width * 0.5} y1={y} x2={x + width * 0.5} y2={y - 15.0} />

        <Show when=move || type_ == Complexity::Atomic clone:label>
            <circle r={height * 0.4} cx={x + width * 0.5} cy={y + height * 0.4} fill="steelblue" stroke="darkblue" stroke-width="1" />
            <SvgText text={label.clone()} font_size={font_size} x={x + (width * 0.5)} y={y + height * 1.25} />
        </Show>

        <Show when=is_complex clone:label>
            <rect x={x} y={y} fill="none" stroke="steelblue" stroke-width="2" rx="5" ry="5" width={width} height={height}></rect>
            <SvgText text={label.clone()} font_size={font_size} x={x + (width * 0.5)} y={y + height / 1.5} />
            <SvgLine x1={x + width * 0.5} y1={y + height} x2={x + width * 0.5} y2={y + height + 15.0} />
        </Show>
    }
}

#[component]
pub fn SvgSinkOrSource(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: &'static str,
) -> impl IntoView {
    let points = format!(
        "{x},{y} {},{} {},{} {x},{}",
        x + width,
        y + 1.0,
        x + width,
        y + height,
        y + height + 1.0
    );

    view! {
        <polyline points={points} fill="none" stroke={color} stroke-width="2" stroke-linejoin="round"/>
        <SvgLine x1={x + width * 0.5} y1={y + height} x2={x + width * 0.5} y2={y + height + 15.0} />
    }
}

#[component]
pub fn SvgLine(x1: f64, y1: f64, x2: f64, y2: f64) -> impl IntoView {
    view! {
        <line x1={x1} y1={y1} x2={x2} y2={y2} stroke-width="2" stroke="black" />
    }
}

#[component]
pub fn SvgText(text: String, font_size: &'static str, x: f64, y: f64) -> impl IntoView {
    view! {
        <text x={x} y={y} font-size={font_size} font-weight="bold" font-family="sans-serif" text-anchor="middle" fill="#222">
            {text}
        </text>
    }
}
