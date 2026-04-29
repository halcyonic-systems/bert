use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub model_context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub response: String,
    pub provider: String,
}

#[tauri::command]
pub async fn chat_with_model(request: ChatRequest) -> Result<ChatResponse, String> {
    let response = mock_response(&request.message, &request.model_context);
    Ok(ChatResponse {
        response,
        provider: "mock".to_string(),
    })
}

fn mock_response(message: &str, context: &str) -> String {
    let model_info = parse_model_facts(context);
    let message_lower = message.to_lowercase();

    if message_lower.contains("what is") && (message_lower.contains("system") || message_lower.contains("this")) {
        model_info
    } else if message_lower.contains("source") {
        let sources = extract_list(context, "environment.sources", "info.name");
        format!("{}\n\n**Sources:**\n{}", model_info, bullet_list(&sources))
    } else if message_lower.contains("sink") {
        let sinks = extract_list(context, "environment.sinks", "info.name");
        format!("{}\n\n**Sinks:**\n{}", model_info, bullet_list(&sinks))
    } else if message_lower.contains("subsystem") || message_lower.contains("component") {
        let systems = extract_systems_detail(context);
        format!("{}\n\n**Subsystems:**\n{}", model_info, systems)
    } else if message_lower.contains("interaction") || message_lower.contains("flow") {
        let flows = extract_interactions_detail(context);
        format!("{}\n\n**Interactions:**\n{}", model_info, flows)
    } else if message_lower.contains("interface") {
        let interfaces = extract_interfaces_detail(context);
        format!("{}\n\n**Interfaces:**\n{}", model_info, interfaces)
    } else if message_lower.contains("processor") {
        let processors = extract_processors(context);
        format!("{}\n\n**Interface Processors:**\n{}", model_info, processors)
    } else {
        format!(
            "{}\n\n*Mock mode — ask about subsystems, flows, interfaces, sources, or sinks.*\n\
             *For LLM analysis, start Ollama: `ollama run llama3.2:3b`*",
            model_info
        )
    }
}

fn find_s0(json: &serde_json::Value) -> Option<&serde_json::Value> {
    json.get("systems")?.as_array()?.iter()
        .find(|s| s.pointer("/info/level").and_then(|v| v.as_i64()) == Some(0))
}

fn parse_model_facts(context: &str) -> String {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(context) else {
        return "Unable to parse model data.".to_string();
    };

    let s0 = find_s0(&json);
    let name = s0.and_then(|s| s.pointer("/info/name")).and_then(|v| v.as_str())
        .unwrap_or_else(|| json.pointer("/environment/info/name").and_then(|v| v.as_str()).unwrap_or("Unknown"));
    let desc = s0.and_then(|s| s.pointer("/info/description")).and_then(|v| v.as_str())
        .unwrap_or("");

    let systems_count = json.get("systems")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let interactions_count = json.get("interactions")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let sources_count = json.pointer("/environment/sources")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let sinks_count = json.pointer("/environment/sinks")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    let subsystem_names: Vec<&str> = json.get("systems")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|s| s.pointer("/info/name").and_then(|v| v.as_str()))
                .collect()
        })
        .unwrap_or_default();

    format!(
        "**System Facts:**\n\
         • **Name**: {name}\n\
         {desc_line}\
         • **Systems**: {systems_count}\n\
         • **Interactions**: {interactions_count}\n\
         • **Sources**: {sources_count}\n\
         • **Sinks**: {sinks_count}\n\
         • **Subsystems**: {subs}",
        desc_line = if desc.is_empty() { String::new() } else { format!("• **Description**: {desc}\n") },
        subs = if subsystem_names.is_empty() { "None".to_string() } else { subsystem_names.join(", ") },
    )
}

fn extract_list(context: &str, path: &str, name_path: &str) -> Vec<String> {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(context) else {
        return vec![];
    };

    let pointer = format!("/{}", path.replace('.', "/"));
    json.pointer(&pointer)
        .and_then(|v| v.as_array())
        .map(|arr| {
            let name_pointer = format!("/{}", name_path.replace('.', "/"));
            arr.iter()
                .filter_map(|item| item.pointer(&name_pointer).and_then(|v| v.as_str()))
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default()
}

fn extract_systems_detail(context: &str) -> String {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(context) else {
        return "Unable to parse.".to_string();
    };

    json.get("systems")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .enumerate()
                .filter_map(|(i, s)| {
                    let name = s.pointer("/info/name")?.as_str()?;
                    let id = s.pointer("/info/id")?.as_str().unwrap_or("?");
                    let archetype = s.get("archetype").and_then(|v| v.as_str()).unwrap_or("unspecified");
                    let parent_iface = s.pointer("/boundary/parent_interface").and_then(|v| v.as_str());
                    let role = if parent_iface.is_some() { "processor" } else { "independent" };
                    Some(format!("{}. **{}** ({}) — {} [{}]", i + 1, name, id, archetype, role))
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_else(|| "No systems found.".to_string())
}

fn extract_interactions_detail(context: &str) -> String {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(context) else {
        return "Unable to parse.".to_string();
    };

    json.get("interactions")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .enumerate()
                .filter_map(|(i, ix)| {
                    let name = ix.pointer("/info/name")?.as_str()?;
                    let src = ix.get("source")?.as_str().unwrap_or("?");
                    let snk = ix.get("sink")?.as_str().unwrap_or("?");
                    let ty = ix.get("type")?.as_str().unwrap_or("?");
                    Some(format!("{}. **{}** — {} → {} [{}]", i + 1, name, src, snk, ty))
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_else(|| "No interactions found.".to_string())
}

fn extract_interfaces_detail(context: &str) -> String {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(context) else {
        return "Unable to parse.".to_string();
    };

    json.get("systems")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .flat_map(|s| {
                    s.pointer("/boundary/interfaces")
                        .and_then(|v| v.as_array())
                        .into_iter()
                        .flatten()
                        .filter_map(|iface| {
                            let name = iface.pointer("/info/name")?.as_str()?;
                            let id = iface.pointer("/info/id")?.as_str().unwrap_or("?");
                            let ty = iface.get("type")?.as_str().unwrap_or("?");
                            Some(format!("• **{}** ({}) — {}", name, id, ty))
                        })
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_else(|| "No interfaces found.".to_string())
}

fn extract_processors(context: &str) -> String {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(context) else {
        return "Unable to parse.".to_string();
    };

    json.get("systems")
        .and_then(|v| v.as_array())
        .map(|arr| {
            let procs: Vec<String> = arr.iter()
                .filter_map(|s| {
                    let pi = s.pointer("/boundary/parent_interface")?.as_str()?;
                    let name = s.pointer("/info/name")?.as_str()?;
                    let id = s.pointer("/info/id")?.as_str().unwrap_or("?");
                    Some(format!("• **{}** ({}) → attached to {}", name, id, pi))
                })
                .collect();
            if procs.is_empty() {
                "No interface processors found.".to_string()
            } else {
                procs.join("\n")
            }
        })
        .unwrap_or_else(|| "No systems found.".to_string())
}

fn bullet_list(items: &[String]) -> String {
    if items.is_empty() {
        "None found.".to_string()
    } else {
        items.iter().map(|s| format!("• {s}")).collect::<Vec<_>>().join("\n")
    }
}
