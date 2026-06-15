//! Flows edge table: read-only view of every interaction (edge) in the model,
//! colored by substance type and styled by usability. Parses model_json — no Bevy
//! ECS dependency, no backend change. Substance/usability are static model properties,
//! so this renders from model_json alone; live flow values live in the LineCharts. (#61)

use std::collections::HashMap;

use leptos::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct ModelJson {
    #[serde(default)]
    systems: Vec<EntityJson>,
    #[serde(default)]
    environment: Option<EnvJson>,
    #[serde(default)]
    interactions: Vec<InteractionJson>,
}

#[derive(Deserialize)]
struct EnvJson {
    #[serde(default)]
    sources: Vec<EntityJson>,
    #[serde(default)]
    sinks: Vec<EntityJson>,
}

#[derive(Deserialize)]
struct EntityJson {
    info: IdName,
}

#[derive(Deserialize)]
struct IdName {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct InteractionJson {
    info: IdName,
    #[serde(default)]
    substance: SubstanceJson,
    #[serde(rename = "type", default)]
    interaction_type: String,
    #[serde(default)]
    usability: String,
    #[serde(default)]
    source: String,
    #[serde(default)]
    sink: String,
}

#[derive(Deserialize, Default)]
struct SubstanceJson {
    #[serde(rename = "type", default)]
    stype: String,
    #[serde(default)]
    sub_type: String,
}

#[derive(Clone, Debug, PartialEq)]
struct FlowRow {
    name: String,
    source: String,
    sink: String,
    substance: String,
    sub_type: String,
    usability: String,
    is_force: bool,
}

/// Tailwind classes for a substance-type badge.
fn substance_class(substance: &str) -> &'static str {
    match substance {
        "Energy" => "bg-amber-100 text-amber-700",
        "Material" => "bg-emerald-100 text-emerald-700",
        "Message" => "bg-violet-100 text-violet-700",
        _ => "bg-gray-100 text-gray-500",
    }
}

/// Tailwind classes for a usability badge.
fn usability_class(usability: &str) -> &'static str {
    match usability {
        "Resource" => "bg-sky-50 text-sky-600",
        "Product" => "bg-emerald-50 text-emerald-600",
        "Waste" => "bg-gray-100 text-gray-400",
        "Disruption" => "bg-red-50 text-red-600",
        _ => "bg-gray-50 text-gray-400",
    }
}

fn extract_flows(json_str: &str) -> Vec<FlowRow> {
    let model: ModelJson = match serde_json::from_str(json_str) {
        Ok(m) => m,
        Err(_) => return vec![],
    };

    // id -> display name across systems + environment sources/sinks.
    let mut names: HashMap<String, String> = HashMap::new();
    for s in &model.systems {
        names.insert(s.info.id.clone(), s.info.name.clone());
    }
    if let Some(ref env) = model.environment {
        for e in env.sources.iter().chain(env.sinks.iter()) {
            names.insert(e.info.id.clone(), e.info.name.clone());
        }
    }
    let resolve = |id: &str| -> String {
        names.get(id).cloned().unwrap_or_else(|| {
            if id.is_empty() {
                "\u{2014}".to_string()
            } else {
                id.to_string()
            }
        })
    };

    let mut rows: Vec<FlowRow> = model
        .interactions
        .iter()
        .map(|ix| FlowRow {
            name: ix.info.name.clone(),
            source: resolve(&ix.source),
            sink: resolve(&ix.sink),
            substance: ix.substance.stype.clone(),
            sub_type: ix.substance.sub_type.clone(),
            usability: ix.usability.clone(),
            is_force: ix.interaction_type == "Force",
        })
        .collect();

    // Group by substance for a readable order (Energy, Material, Message, other).
    let rank = |s: &str| match s {
        "Energy" => 0,
        "Material" => 1,
        "Message" => 2,
        _ => 3,
    };
    rows.sort_by(|a, b| {
        rank(&a.substance)
            .cmp(&rank(&b.substance))
            .then(a.name.cmp(&b.name))
    });
    rows
}

#[component]
pub fn FlowsTable(model_json: Signal<Option<String>>) -> impl IntoView {
    let flows = Memo::new(move |_| {
        model_json
            .get()
            .map(|json| extract_flows(&json))
            .unwrap_or_default()
    });

    let (table_open, set_table_open) = signal(false);
    let has_flows = Memo::new(move |_| !flows.get().is_empty());
    let flow_count = Memo::new(move |_| flows.get().len());

    view! {
        <Show when=move || has_flows.get()>
            <div class="px-4 py-2 border-b border-gray-100">
                <button
                    class="text-xs text-gray-500 font-medium flex items-center gap-1 hover:text-gray-700"
                    on:click=move |_| set_table_open.update(|v| *v = !*v)
                >
                    <span class="text-[10px]">
                        {move || if table_open.get() { "\u{25BC}" } else { "\u{25B6}" }}
                    </span>
                    "Flows"
                    <span class="text-gray-300 ml-1">{move || flow_count.get()}</span>
                </button>
                <Show when=move || table_open.get()>
                    <div class="overflow-x-auto mt-2">
                        <table class="w-full text-xs">
                            <thead>
                                <tr class="text-left text-gray-400 border-b border-gray-100">
                                    <th class="py-1 pr-3 font-medium">{"Flow"}</th>
                                    <th class="py-1 pr-3 font-medium">{"Source \u{2192} Sink"}</th>
                                    <th class="py-1 pr-3 font-medium">{"Substance"}</th>
                                    <th class="py-1 font-medium">{"Usability"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || flows.get().into_iter().map(|row| {
                                    let subst_cls = format!(
                                        "px-1.5 py-0.5 rounded text-[10px] font-medium {}",
                                        substance_class(&row.substance),
                                    );
                                    let use_cls = format!(
                                        "px-1.5 py-0.5 rounded text-[10px] font-medium {}",
                                        usability_class(&row.usability),
                                    );
                                    let substance_label = if row.sub_type.is_empty() {
                                        row.substance.clone()
                                    } else {
                                        format!("{} \u{00B7} {}", row.substance, row.sub_type)
                                    };
                                    let name_view = if row.is_force {
                                        format!("{} (force)", row.name)
                                    } else {
                                        row.name
                                    };
                                    view! {
                                        <tr class="border-b border-gray-50">
                                            <td class="py-1.5 pr-3 text-gray-700 font-medium">{name_view}</td>
                                            <td class="py-1.5 pr-3 text-gray-500">
                                                {row.source}{" \u{2192} "}{row.sink}
                                            </td>
                                            <td class="py-1.5 pr-3">
                                                <span class=subst_cls>{substance_label}</span>
                                            </td>
                                            <td class="py-1.5">
                                                <span class=use_cls>{row.usability}</span>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()}
                            </tbody>
                        </table>
                    </div>
                </Show>
            </div>
        </Show>
    }
}
