use std::collections::HashMap;

use leptos::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct ModelJson {
    #[serde(default)]
    interactions: Vec<InteractionJson>,
}

#[derive(Deserialize)]
struct InteractionJson {
    info: InfoJson,
    source: String,
    substance: SubstanceJson,
    #[serde(default = "default_amount")]
    amount: String,
    #[serde(default)]
    r#type: String,
}

fn default_amount() -> String {
    "1".to_string()
}

#[derive(Deserialize)]
struct InfoJson {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct SubstanceJson {
    #[serde(rename = "type")]
    substance_type: String,
}

#[derive(Clone, Debug, PartialEq)]
struct SourceFlow {
    flow_id: String,
    label: String,
    substance: String,
    initial: f64,
}

fn extract_source_flows(json_str: &str) -> Vec<SourceFlow> {
    let model: ModelJson = match serde_json::from_str(json_str) {
        Ok(m) => m,
        Err(_) => return vec![],
    };

    model
        .interactions
        .iter()
        .filter(|ix| ix.source.starts_with("Src-") && ix.r#type != "Force")
        .map(|ix| {
            let label = format!("{} ({})", ix.info.name, ix.substance.substance_type);
            let initial = ix.amount.parse::<f64>().unwrap_or(1.0);
            SourceFlow {
                flow_id: ix.info.id.clone(),
                label,
                substance: ix.substance.substance_type.clone(),
                initial,
            }
        })
        .collect()
}

#[component]
pub fn InputsPanel(
    model_json: Signal<Option<String>>,
    on_change: Callback<HashMap<String, f64>>,
) -> impl IntoView {
    let source_flows = Memo::new(move |_| {
        model_json
            .get()
            .map(|json| extract_source_flows(&json))
            .unwrap_or_default()
    });

    let slider_values = RwSignal::new(HashMap::<String, f64>::new());

    Effect::new(move |_| {
        let flows = source_flows.get();
        let mut vals = HashMap::new();
        for sf in &flows {
            vals.insert(sf.flow_id.clone(), sf.initial);
        }
        slider_values.set(vals.clone());
        on_change.run(vals);
    });

    view! {
        <Show when=move || !source_flows.get().is_empty()>
            <div class="flex items-center gap-4 px-4 py-1.5 bg-gray-50/60 border-b border-gray-100 overflow-x-auto">
                <span class="text-xs text-gray-400 font-medium shrink-0">{"Inputs"}</span>
                <For
                    each=move || source_flows.get()
                    key=|sf| sf.flow_id.clone()
                    let:sf
                >
                    {
                        let flow_id = sf.flow_id.clone();
                        let label = sf.label.clone();
                        let initial = sf.initial;
                        let max_val = if initial <= 1.0 { 100.0 } else { initial * 10.0 };
                        let step = if max_val <= 10.0 { 0.1 } else { 1.0 };

                        let (val, set_val) = signal(initial);

                        let fid = flow_id.clone();
                        view! {
                            <div class="flex items-center gap-1.5 shrink-0">
                                <label class="text-xs text-gray-500 whitespace-nowrap">{label}</label>
                                <input
                                    type="range"
                                    min="0"
                                    max=max_val.to_string()
                                    step=step.to_string()
                                    prop:value=move || val.get().to_string()
                                    class="w-20 h-1 accent-blue-500"
                                    on:input={
                                        let fid = fid.clone();
                                        move |ev| {
                                            let v: f64 = event_target_value(&ev).parse().unwrap_or(initial);
                                            set_val.set(v);
                                            slider_values.update(|m| { m.insert(fid.clone(), v); });
                                            on_change.run(slider_values.get_untracked());
                                        }
                                    }
                                />
                                <span class="text-xs text-gray-400 font-mono w-8 text-right">{move || format!("{:.1}", val.get())}</span>
                            </div>
                        }
                    }
                </For>
            </div>
        </Show>
    }
}
