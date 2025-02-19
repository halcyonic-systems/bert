use leptos::prelude::*;

#[component]
pub fn SvgNode(
    label: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    font_size: &'static str,
) -> impl IntoView {
    view! {
        <rect x={x} y={y} fill="none" stroke="steelblue" stroke-width="2" rx="5" ry="5" width={width} height={height}></rect>
        <text x={x + (width / 2.0)} y={y + height / 1.5} fill="#222" font-size={font_size} font-weight="bold" font-family="sans-serif" text-anchor="middle">
            {label}
        </text>
        <line x1={x + width / 2.0} y1={y} x2={x + width / 2.0} y2={y - 15.0} stroke-width="2" stroke="black" />
        <line x1={x + width / 2.0} y1={y + height} x2={x + width / 2.0} y2={y + height + 15.0} stroke-width="2" stroke="black" />
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
    let points = format!("{x},{y} {},{} {},{} {x},{}", x+width, y+1.0, x+width, y+height, y+height+1.0);

    view! {
        <polyline points={points} fill="none" stroke={color} stroke-width="2" stroke-linejoin="round"/>
        <line x1={x + width / 2.0} y1={y + height} x2={x + width / 2.0} y2={y + height + 15.0} stroke-width="2" stroke="black" />
    }
}
