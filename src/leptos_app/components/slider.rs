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
    // #[prop(into, optional)] show_steps: Signal<bool>,
    on_input: impl Fn(f64) + 'static + Clone,
) -> impl IntoView {

    let internal_value = RwSignal::new(value.get_untracked());
    // let range = Signal::derive(move || {
    //     if !show_steps.get() {
    //         return vec![];
    //     }
    //
    //     let min = min.get();
    //     let max = max.get();
    //     let step = step.get();
    //     let num_steps = ((max - min) / step).ceil() as usize;
    //     (0..num_steps)
    //         .map(|i| min + (i as f64 * step))
    //         .collect::<Vec<_>>()
    // });

    let on_input = move |ev: Event| {
        let value = event_target_value(&ev).parse::<f64>().unwrap();
        internal_value.set(value);
        on_input(value);
    };

    // let internal_value = move || {
    //     let mut val = value.get().to_string();
    //     let i_comma = val.find(|c| c == '.');
    //     if let Some(i) = i_comma {
    //         let v = val.clone();
    //         val.remove_matches(&v[i+2..]);
    //     }
    //     val
    // };

    view! {
        <div class="mb-2">
            <label for=id class="block text-sm/6 font-medium text-gray-900">
                {label}
            </label>
            <div class="flex items-center relative rounded-md">
                <input
                    id=id
                    type="range"
                    prop:min=min
                    prop:max=max
                    prop:step=step
                    prop:value=value
                    on:input=on_input.clone()
                    class="w-full appearance-none h-[15px] rounded-2xl cursor-pointer outline-none
                    [&::-webkit-slider-runnable-track]:h-[16px]
                    [&::-webkit-slider-runnable-track]:rounded-2xl
                    [&::-webkit-slider-thumb]:h-[15px]
                    [&::-webkit-slider-thumb]:w-[15px]
                    [&::-webkit-slider-thumb]:bg-gray-200
                    [&::-webkit-slider-thumb]:rounded-full
                    [&::-webkit-slider-thumb]:border-rose-600
                    [&::-webkit-slider-thumb]:border-2
                    
                    [&::-moz-range-track]:h-[15px]
                    [&::-moz-range-track]:rounded-2xl
                    [&::-moz-range-thumb]:h-[15px]
                    [&::-moz-range-thumb]:w-[15px]
                    [&::-moz-range-thumb]:rounded-full
                    [&::-moz-range-thumb]:border
                    [&::-moz-range-thumb]:border-rose-600
                    [&::-moz-range-thumb]:bg-gray-200
                    "
                    style:background=move || {
                        let percent = (value.get() / (max.get() - min.get())) * 100f64;
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
                    class="ml-1 p-1 text-center border-rose-600 border-2 rounded-lg w-[60px]
                    [&::-moz-appearance]: textfield
                    [&::-webkit-outer-spin-button]:appearance-none
                    [&::-webkit-inner-spin-button]:appearance-none
                    [&::-webkit-outer-spin-button]:m-0
                    [&::-webkit-inner-spin-button]:m-0
                    "
                />
            // <div
            // class="absolute w-full h-full bg-gray-400 rounded-lg"
            // style=move || {
            // format!(
            // "left: {}%; width: {}%",
            // (value.get() - min.get()) / (max.get() - min.get()) * 100.0,
            // (value.get() - min.get()) / (max.get() - min.get()) * 100.0,
            // )
            // }
            // ></div>
            // <Show when=move || show_steps.get() {..} class="flex justify-between text-xs">
            // <For
            // each=move || range.get()
            // key=move |i| i.to_string()
            // children=move |i| {
            // view! {
            // <div class="flex items-center">
            // <span class="block w-2 h-2 bg-gray-400 rounded-full"></span>
            // <span class="ml-1">{i}</span>
            // </div>
            // }
            // }
            // />
            // </Show>
            </div>
        </div>
    }
}
