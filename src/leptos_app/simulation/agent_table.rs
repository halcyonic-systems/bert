//! Agent comparison table: read-only pre-sim overview of all subsystems.
//! Parses model_json signal — no Bevy ECS dependency.

use leptos::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct ModelJson {
    #[serde(default)]
    systems: Vec<SystemJson>,
}

#[derive(Deserialize)]
struct SystemJson {
    info: InfoJson,
    #[serde(default)]
    archetype: Option<String>,
    #[serde(default)]
    agent: Option<AgentJson>,
    #[serde(default)]
    time_constant: Option<String>,
    complexity: serde_json::Value,
}

#[derive(Deserialize)]
struct AgentJson {
    kind: String,
    agency_capacity: f64,
    #[serde(default)]
    primitives: Vec<String>,
}

#[derive(Deserialize)]
struct InfoJson {
    name: String,
    level: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct AgentRow {
    name: String,
    level: i32,
    archetype: String,
    agent_kind: String,
    agency_capacity: String,
    primitives: Vec<String>,
    time_constant: String,
    has_agent: bool,
}

fn extract_agents(json_str: &str) -> Vec<AgentRow> {
    let model: ModelJson = match serde_json::from_str(json_str) {
        Ok(m) => m,
        Err(_) => return vec![],
    };

    let mut rows: Vec<AgentRow> = model
        .systems
        .iter()
        .filter(|s| s.info.level >= 1)
        .map(|s| {
            let (agent_kind, agency_capacity, primitives, has_agent) =
                if let Some(ref agent) = s.agent {
                    (
                        agent.kind.clone(),
                        format!("{:.2}", agent.agency_capacity),
                        agent.primitives.clone(),
                        true,
                    )
                } else {
                    (
                        "\u{2014}".to_string(),
                        "\u{2014}".to_string(),
                        vec![],
                        false,
                    )
                };

            AgentRow {
                name: s.info.name.clone(),
                level: s.info.level,
                archetype: s
                    .archetype
                    .clone()
                    .unwrap_or_else(|| "Unspecified".to_string()),
                agent_kind,
                agency_capacity,
                primitives,
                time_constant: s
                    .time_constant
                    .clone()
                    .unwrap_or_else(|| "Second".to_string()),
                has_agent,
            }
        })
        .collect();

    rows.sort_by(|a, b| a.level.cmp(&b.level).then(a.name.cmp(&b.name)));
    rows
}

#[component]
pub fn AgentComparisonTable(model_json: Signal<Option<String>>) -> impl IntoView {
    let agents = Memo::new(move |_| {
        model_json
            .get()
            .map(|json| extract_agents(&json))
            .unwrap_or_default()
    });

    let (table_open, set_table_open) = signal(false);

    let has_agents = Memo::new(move |_| !agents.get().is_empty());
    let agent_count = Memo::new(move |_| agents.get().len());

    view! {
        <Show when=move || has_agents.get()>
            <div class="px-4 py-2 border-b border-gray-100">
                <button
                    class="text-xs text-gray-500 font-medium flex items-center gap-1 hover:text-gray-700"
                    on:click=move |_| set_table_open.update(|v| *v = !*v)
                >
                    <span class="text-[10px]">
                        {move || if table_open.get() { "\u{25BC}" } else { "\u{25B6}" }}
                    </span>
                    "Agents"
                    <span class="text-gray-300 ml-1">{move || agent_count.get()}</span>
                </button>
                <Show when=move || table_open.get()>
                    <div class="overflow-x-auto mt-2">
                        <table class="w-full text-xs">
                            <thead>
                                <tr class="text-left text-gray-400 border-b border-gray-100">
                                    <th class="py-1 pr-3 font-medium">{"Name"}</th>
                                    <th class="py-1 pr-3 font-medium">{"Archetype"}</th>
                                    <th class="py-1 pr-3 font-medium">{"Kind"}</th>
                                    <th class="py-1 pr-3 font-medium">{"Agency"}</th>
                                    <th class="py-1 pr-3 font-medium">{"Primitives"}</th>
                                    <th class="py-1 font-medium">{"Time"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || agents.get().into_iter().map(|row| {
                                    let row_class = if row.has_agent {
                                        "border-b border-gray-50"
                                    } else {
                                        "border-b border-gray-50 bg-amber-50/50"
                                    };
                                    let prims = row.primitives;
                                    view! {
                                        <tr class=row_class>
                                            <td class="py-1.5 pr-3 text-gray-700 font-medium">{row.name}</td>
                                            <td class="py-1.5 pr-3 text-gray-500">{row.archetype}</td>
                                            <td class="py-1.5 pr-3 text-gray-500">{row.agent_kind}</td>
                                            <td class="py-1.5 pr-3 text-gray-500 tabular-nums">{row.agency_capacity}</td>
                                            <td class="py-1.5 pr-3">
                                                <div class="flex flex-wrap gap-0.5">
                                                    {prims.into_iter().map(|p| {
                                                        view! {
                                                            <span class="bg-blue-100 text-blue-700 px-1 rounded text-[10px]">{p}</span>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            </td>
                                            <td class="py-1.5 text-gray-400">{row.time_constant}</td>
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
