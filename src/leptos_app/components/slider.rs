use leptos::prelude::*;
use web_sys::Event;

#[component]
pub fn Slider(
    id: &'static str,
    label: &'static str,
    #[prop(into)] value: Signal<f64>,
    #[prop(into, default = 0.0.into())] min: Signal<f64>,
    #[prop(into, default = 1.0.into())] max: Signal<f64>,
    #[prop(into, default = 1.0.into())] step: Signal<f64>,
    #[prop(optional, into)] tooltip: Option<&'static str>,
    on_input: impl Fn(f64) + 'static + Clone,
) -> impl IntoView {
    let internal_value = RwSignal::new(value.get_untracked());

    let on_input = move |ev: Event| {
        let Ok(v) = event_target_value(&ev).parse::<f64>() else {
            return; // empty field while user is mid-edit — skip
        };
        internal_value.set(v);
        on_input(v);
    };

    view! {
        <div class="mb-2">
            <label for=id class="flex items-center font-medium text-gray-900 text-sm/6">
                <span>{label}</span>
                {tooltip.map(|tip| view! {
                    <span class="ml-1 text-gray-400 hover:text-gray-600 cursor-help text-sm" title=tip>
                        "?"
                    </span>
                })}
            </label>
            <div class="flex relative items-center rounded-md">
                <input
                    id=id
                    type="range"
                    prop:min=min
                    prop:max=max
                    prop:step=step
                    prop:value=value
                    on:input=on_input.clone()
                    class="w-full rounded-2xl appearance-none cursor-pointer outline-none h-[15px] [&::-webkit-slider-runnable-track]:h-[16px] [&::-webkit-slider-runnable-track]:rounded-2xl [&::-webkit-slider-thumb]:h-[15px] [&::-webkit-slider-thumb]:w-[15px] [&::-webkit-slider-thumb]:bg-gray-200 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:border-cyan-600 [&::-webkit-slider-thumb]:border-2 [&::-moz-range-track]:h-[15px] [&::-moz-range-track]:rounded-2xl [&::-moz-range-thumb]:h-[15px] [&::-moz-range-thumb]:w-[15px] [&::-moz-range-thumb]:rounded-full [&::-moz-range-thumb]:border [&::-moz-range-thumb]:border-cyan-600 [&::-moz-range-thumb]:bg-gray-200"
                    style:background=move || {
                        let percent = (internal_value.get() / (max.get() - min.get())) * 100f64;
                        format!(
                            "linear-gradient(to right, #e11d48 0%, #e11d48 {percent}%, #d1d5db {percent}%, #d1d5db 100%)",
                        )
                    }
                />
                <input
                    id="slider-value"
                    step="0.01"
                    prop:value=internal_value
                    on:input=on_input
                    type="number"
                    class="p-1 ml-1 text-center rounded-lg border-2 border-cyan-600 w-[60px] [&::-moz-appearance]: textfield [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:m-0 [&::-webkit-inner-spin-button]:m-0"
                />
            </div>
        </div>
    }
}
