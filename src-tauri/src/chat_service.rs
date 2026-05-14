use std::time::Duration;

use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, MessageRole},
    Ollama,
};
use serde::{Deserialize, Serialize};

const OLLAMA_MODEL: &str = "gemma4:e2b";

const BERT_RAG_URL: &str = "http://localhost:5010/ask";
const BERT_RAG_GENERATE_URL: &str = "http://localhost:5010/generate-from-description";

const ANALYSIS_PROMPT: &str = r#"You are a BERT systems analysis assistant. You analyze system models described in JSON.

RULES:
1. Extract data EXACTLY as written in JSON — quote field names and values directly.
2. Count elements precisely.
3. State what IS present, not what it might mean.
4. If unsure about a fact, state "Data not available" instead of guessing.

FORBIDDEN: "appears to be", "seems", "suggests", "likely", "probably"

Start responses with **System Facts:** when describing the overall system.
Use **bold** for entity names and `code` for IDs.
Be concise."#;

const CREATION_PROMPT: &str = r#"You are helping a user design a systems model using Mobus's systems science framework. Your job is to help them identify the key structural elements of their system through natural conversation.

Guide them through these elements (but conversationally, not as a checklist):
1. **System name and purpose** — what is the system and what does it do?
2. **Subsystems** — what are the main internal components? (2-5 is ideal)
3. **Sources** — what enters the system from outside? (energy, materials, information)
4. **Sinks** — what leaves the system to the outside? (products, waste, signals)
5. **Internal flows** — how do subsystems connect to each other?

Keep responses SHORT (2-4 sentences). Ask ONE clarifying question at a time. Use the user's language, not jargon. When you have enough information for a basic model (name + 2 subsystems + 1 source + 1 sink), tell them: "I think we have enough to generate a first draft! Click **Generate Model** when you're ready, or keep describing to add more detail."

Do NOT output JSON. Just have a conversation."#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub model_context: String,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub history: Vec<HistoryMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRef {
    pub source: String,
    #[serde(default)]
    pub excerpt: String,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default, rename = "type")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub response: String,
    pub provider: String,
    #[serde(default)]
    pub dimensions: Option<Vec<String>>,
    #[serde(default)]
    pub route: Option<String>,
    #[serde(default)]
    pub confidence: Option<f64>,
    #[serde(default)]
    pub intensity: Option<String>,
    #[serde(default)]
    pub sources: Option<Vec<SourceRef>>,
}

#[tauri::command]
pub async fn chat_with_model(request: ChatRequest) -> Result<ChatResponse, String> {
    if request.mode == "creation" {
        return chat_creation_mode(&request.message, &request.history).await;
    }

    // Engine gets full model JSON for structural reasoning.
    // Ollama fallback gets compressed summary to fit smaller context windows.
    if let Ok(resp) = try_bert_rag(&request.message, &request.model_context, &request.history).await {
        return Ok(resp);
    }

    let summary = extract_model_summary(&request.model_context);
    if let Ok(response) = try_ollama(&request.message, &summary).await {
        return Ok(ChatResponse {
            response,
            provider: "ollama".to_string(),
            dimensions: None,
            route: None,
            confidence: None,
            intensity: None,
            sources: None,
        });
    }

    Ok(ChatResponse {
        response: mock_response(&request.message, &request.model_context),
        provider: "mock".to_string(),
        dimensions: None,
        route: None,
        confidence: None,
        intensity: None,
        sources: None,
    })
}

async fn chat_creation_mode(
    message: &str,
    history: &[HistoryMessage],
) -> Result<ChatResponse, String> {
    let ollama = Ollama::default();

    let mut messages = vec![ChatMessage::new(
        MessageRole::System,
        CREATION_PROMPT.to_string(),
    )];

    for h in history {
        let role = match h.role.as_str() {
            "user" => MessageRole::User,
            _ => MessageRole::Assistant,
        };
        messages.push(ChatMessage::new(role, h.content.clone()));
    }

    messages.push(ChatMessage::new(MessageRole::User, message.to_string()));

    let request = ChatMessageRequest::new(OLLAMA_MODEL.to_string(), messages);

    match ollama.send_chat_messages(request).await {
        Ok(response) => Ok(ChatResponse {
            response: response.message.content,
            provider: "ollama".to_string(),
            dimensions: None, route: None, confidence: None, intensity: None, sources: None,
        }),
        Err(e) => Ok(ChatResponse {
            response: format!("I'd love to help you design this system! Tell me more about the main components and what flows in and out.\n\n*(LLM unavailable: {})*", e),
            provider: "fallback".to_string(),
            dimensions: None, route: None, confidence: None, intensity: None, sources: None,
        }),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub conversation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateModelResponse {
    pub json_data: String,
}

#[tauri::command]
pub async fn generate_model_from_conversation(
    conversation: String,
) -> Result<GenerateModelResponse, String> {
    // Try the engine first (full pipeline: description → extraction → compile)
    if let Ok(resp) = try_engine_generate(&conversation).await {
        return Ok(resp);
    }

    // Fallback: local Ollama extraction + local Rust compile
    let mut intermediate = extract_intermediate_from_conversation(&conversation).await?;
    repair_intermediate(&mut intermediate);
    let bert_json = compile_intermediate(&intermediate)?;
    Ok(GenerateModelResponse {
        json_data: bert_json,
    })
}

/// Primary generation path: delegates to bert-rag engine which handles
/// LLM extraction, intermediate validation, and deterministic compilation.
async fn try_engine_generate(description: &str) -> Result<GenerateModelResponse, Box<dyn std::error::Error>> {
    let body = serde_json::json!({ "description": description });

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(120))
        .build()?;

    let resp = client.post(BERT_RAG_GENERATE_URL).json(&body).send().await?;

    if !resp.status().is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("Engine returned {text}").into());
    }

    let json: serde_json::Value = resp.json().await?;
    let model = json.get("model").ok_or("no model field in engine response")?;
    let json_data = serde_json::to_string(model)?;

    Ok(GenerateModelResponse { json_data })
}

/// Fallback-only: patches LLM output when engine is unavailable and local Ollama
/// extraction produces incomplete intermediate specs (empty names, missing sinks, etc).
fn repair_intermediate(spec: &mut serde_json::Value) {
    // Fill empty names
    for (section, prefix) in [
        ("sources", "Source"),
        ("sinks", "Sink"),
        ("subsystems", "Subsystem"),
        ("external_flows", "Flow"),
        ("internal_flows", "Flow"),
    ] {
        if let Some(arr) = spec.get_mut(section).and_then(|v| v.as_array_mut()) {
            for (i, item) in arr.iter_mut().enumerate() {
                if let Some(name) = item.get_mut("name") {
                    if name.as_str().map(|s| s.trim().is_empty()).unwrap_or(true) {
                        *name = serde_json::Value::String(format!("{prefix} {}", i + 1));
                    }
                } else if let Some(obj) = item.as_object_mut() {
                    obj.insert("name".to_string(), serde_json::Value::String(format!("{prefix} {}", i + 1)));
                }
            }
        }
    }

    // Ensure at least 1 sink exists
    let has_sinks = spec.get("sinks")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    if !has_sinks {
        let sys_name = spec.pointer("/system/name")
            .and_then(|v| v.as_str())
            .unwrap_or("System");
        let sink_name = format!("{sys_name} Output");
        let first_sub = spec.get("subsystems")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .and_then(|s| s.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("Subsystem 1")
            .to_string();

        spec.as_object_mut().map(|obj| {
            obj.insert("sinks".to_string(), serde_json::json!([
                {"name": sink_name, "description": "Primary output"}
            ]));
        });

        // Add routing_table entry for the new sink
        if let Some(rt) = spec.get_mut("routing_table").and_then(|v| v.as_array_mut()) {
            rt.push(serde_json::json!({
                "interface": format!("{sink_name} Port"),
                "type": "Export",
                "connected_to": sink_name,
                "has_processor": true,
                "target_subsystem": first_sub,
            }));
        }

        // Add external flow for the new sink
        if let Some(ef) = spec.get_mut("external_flows").and_then(|v| v.as_array_mut()) {
            ef.push(serde_json::json!({
                "name": format!("{sink_name} Flow"),
                "interface": format!("{sink_name} Port"),
                "substance": {"type": "Message", "sub_type": "Output"},
                "usability": "Product",
            }));
        }
    }

    // Fix routing_table type mismatches: Import must connect to a source, Export to a sink
    let source_names: Vec<String> = spec.get("sources")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|s| s.get("name").and_then(|v| v.as_str()).map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let sink_names: Vec<String> = spec.get("sinks")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|s| s.get("name").and_then(|v| v.as_str()).map(|s| s.to_string())).collect())
        .unwrap_or_default();

    if let Some(rt) = spec.get_mut("routing_table").and_then(|v| v.as_array_mut()) {
        for entry in rt.iter_mut() {
            let connected = entry.get("connected_to").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if source_names.contains(&connected) {
                entry.as_object_mut().map(|obj| obj.insert("type".to_string(), serde_json::Value::String("Import".to_string())));
            } else if sink_names.contains(&connected) {
                entry.as_object_mut().map(|obj| obj.insert("type".to_string(), serde_json::Value::String("Export".to_string())));
            }
        }
    }

    // Fill empty substance sub_types
    for section in ["external_flows", "internal_flows"] {
        if let Some(arr) = spec.get_mut(section).and_then(|v| v.as_array_mut()) {
            for item in arr.iter_mut() {
                if let Some(sub) = item.get_mut("substance").and_then(|v| v.as_object_mut()) {
                    let has_subtype = sub.get("sub_type")
                        .and_then(|v| v.as_str())
                        .map(|s| !s.trim().is_empty())
                        .unwrap_or(false);
                    if !has_subtype {
                        let default = match sub.get("type").and_then(|v| v.as_str()).unwrap_or("Message") {
                            "Energy" => "Kinetic",
                            "Material" => "Solid",
                            _ => "Data",
                        };
                        sub.insert("sub_type".to_string(), serde_json::Value::String(default.to_string()));
                    }
                }
            }
        }
    }
}

async fn extract_intermediate_from_conversation(
    conversation: &str,
) -> Result<serde_json::Value, String> {
    let extraction_prompt = format!(
        r#"You are a systems model compiler. Extract a minimal system model from the user's description.

Output ONLY valid JSON (no markdown, no explanation, no thinking):
{{
  "system": {{ "name": "...", "description": "..." }},
  "sources": [{{ "name": "...", "description": "..." }}],
  "sinks": [{{ "name": "...", "description": "..." }}],
  "subsystems": [{{ "name": "...", "description": "..." }}],
  "routing_table": [
    {{ "interface": "unique_interface_name", "type": "Import", "connected_to": "source_name", "has_processor": true, "target_subsystem": "subsystem_name" }},
    {{ "interface": "unique_interface_name", "type": "Export", "connected_to": "sink_name", "has_processor": true, "target_subsystem": "subsystem_name" }}
  ],
  "external_flows": [{{ "name": "descriptive_flow_name", "interface": "matches_routing_table_interface", "substance": {{ "type": "Energy|Material|Message", "sub_type": "descriptive_label" }}, "usability": "Resource|Product|Waste" }}],
  "internal_flows": [{{ "name": "descriptive_flow_name", "source": "subsystem_name", "sink": "subsystem_name", "substance": {{ "type": "Message", "sub_type": "descriptive_label" }}, "usability": "Resource" }}]
}}

RULES:
1. Keep it minimal: 2-3 subsystems, 1-2 sources, 1 sink. Simple first draft.
2. If the user only gave a name, infer the most obvious components.
3. MUST have at least 1 source AND at least 1 sink. Every system has outputs.
4. Every "name" field must be a non-empty descriptive string.
5. Every source gets ONE Import interface in routing_table. Every sink gets ONE Export interface.
6. Each source/sink connects through its OWN unique interface — never route two flows through the same interface.
7. routing_table "type" must be "Import" when connected_to is a source, "Export" when connected_to is a sink.
8. Each external_flow "interface" must EXACTLY match an interface name from routing_table.
9. "sub_type" describes what flows (e.g., "Electricity", "Data", "Funds", "Regulations", "Heat").
10. internal_flows connect subsystems to each other (not to interfaces).

Conversation:
{conversation}"#
    );

    let ollama = Ollama::default();
    let messages = vec![ChatMessage::new(MessageRole::User, extraction_prompt)];
    let request = ChatMessageRequest::new(OLLAMA_MODEL.to_string(), messages);

    let response = ollama
        .send_chat_messages(request)
        .await
        .map_err(|e| format!("LLM extraction failed: {e}"))?;

    let raw = response.message.content.trim().to_string();
    // Strip markdown code fences if present
    let json_str = raw
        .strip_prefix("```json")
        .or_else(|| raw.strip_prefix("```"))
        .unwrap_or(&raw);
    let json_str = json_str.strip_suffix("```").unwrap_or(json_str).trim();

    serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse extracted JSON: {e}\n\nRaw output:\n{raw}"))
}

fn compile_intermediate(intermediate: &serde_json::Value) -> Result<String, String> {
    // Validate the intermediate format
    let spec: crate::intermediate::IntermediateSpec = serde_json::from_value(intermediate.clone())
        .map_err(|e| format!("Failed to parse intermediate format: {e}"))?;

    let errors = crate::intermediate::validate_intermediate(&spec);
    if !errors.is_empty() {
        return Err(format!("Validation failed: {}", errors.join("; ")));
    }

    // Generate BERT JSON
    let mut generator = crate::generator::BertModelGenerator::new(intermediate.clone());
    let model = generator.generate();

    serde_json::to_string(&model).map_err(|e| format!("Failed to serialize model: {e}"))
}

#[derive(Deserialize)]
struct EngineResponse {
    answer: String,
    #[serde(default)]
    dimensions: Option<Vec<String>>,
    #[serde(default)]
    route: Option<String>,
    #[serde(default)]
    confidence: Option<f64>,
    #[serde(default)]
    intensity: Option<String>,
    #[serde(default)]
    sources: Option<Vec<SourceRef>>,
}

/// Query the local bert-rag engine for analysis. Sends conversation history
/// for multi-turn context. Returns full metadata (dimensions, route, sources).
async fn try_bert_rag(
    message: &str,
    model_summary: &str,
    history: &[HistoryMessage],
) -> Result<ChatResponse, Box<dyn std::error::Error>> {
    let hist: Vec<serde_json::Value> = history
        .iter()
        .map(|h| serde_json::json!({"role": h.role, "content": h.content}))
        .collect();

    let body = serde_json::json!({
        "question": message,
        "model_context": model_summary,
        "history": hist,
    });

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(60))
        .build()?;

    let resp = client.post(BERT_RAG_URL).json(&body).send().await?;
    let engine: EngineResponse = resp.json().await?;

    Ok(ChatResponse {
        response: engine.answer,
        provider: "bert-rag".to_string(),
        dimensions: engine.dimensions,
        route: engine.route,
        confidence: engine.confidence,
        intensity: engine.intensity,
        sources: engine.sources,
    })
}

async fn try_ollama(
    message: &str,
    model_summary: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let ollama = Ollama::default();

    let user_prompt = format!("Model context:\n{model_summary}\n\nUser question: {message}");

    let messages = vec![
        ChatMessage::new(MessageRole::System, ANALYSIS_PROMPT.to_string()),
        ChatMessage::new(MessageRole::User, user_prompt),
    ];

    let request = ChatMessageRequest::new(OLLAMA_MODEL.to_string(), messages);
    let response = ollama.send_chat_messages(request).await?;

    Ok(response.message.content)
}

fn extract_model_summary(context: &str) -> String {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(context) else {
        return format!("Raw model data:\n{context}");
    };

    let mut summary = String::new();

    let s0 = find_s0(&json);
    if let Some(name) = s0
        .and_then(|s| s.pointer("/info/name"))
        .and_then(|v| v.as_str())
    {
        summary.push_str(&format!("System of Interest: {name}\n"));
    }
    if let Some(desc) = s0
        .and_then(|s| s.pointer("/info/description"))
        .and_then(|v| v.as_str())
    {
        if !desc.is_empty() {
            summary.push_str(&format!("Description: {desc}\n"));
        }
    }

    if let Some(systems) = json.get("systems").and_then(|v| v.as_array()) {
        summary.push_str(&format!("\nSystems ({}):\n", systems.len()));
        for s in systems.iter().take(10) {
            if let Some(name) = s.pointer("/info/name").and_then(|v| v.as_str()) {
                let id = s
                    .pointer("/info/id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("?");
                let arch = s.get("archetype").and_then(|v| v.as_str()).unwrap_or("-");
                let pi = s
                    .pointer("/boundary/parent_interface")
                    .and_then(|v| v.as_str());
                let role = if pi.is_some() {
                    "processor"
                } else {
                    "subsystem"
                };
                summary.push_str(&format!("  {id}: {name} [{arch}, {role}]\n"));
            }
        }
        if systems.len() > 10 {
            summary.push_str(&format!("  ... and {} more\n", systems.len() - 10));
        }
    }

    if let Some(interactions) = json.get("interactions").and_then(|v| v.as_array()) {
        summary.push_str(&format!("\nInteractions ({}):\n", interactions.len()));
        for ix in interactions.iter().take(8) {
            if let Some(name) = ix.pointer("/info/name").and_then(|v| v.as_str()) {
                let src = ix.get("source").and_then(|v| v.as_str()).unwrap_or("?");
                let snk = ix.get("sink").and_then(|v| v.as_str()).unwrap_or("?");
                let ty = ix.get("type").and_then(|v| v.as_str()).unwrap_or("?");
                summary.push_str(&format!("  {name}: {src} → {snk} [{ty}]\n"));
            }
        }
        if interactions.len() > 8 {
            summary.push_str(&format!("  ... and {} more\n", interactions.len() - 8));
        }
    }

    if let Some(sources) = json
        .pointer("/environment/sources")
        .and_then(|v| v.as_array())
    {
        let names: Vec<&str> = sources
            .iter()
            .filter_map(|s| s.pointer("/info/name").and_then(|v| v.as_str()))
            .collect();
        summary.push_str(&format!("\nSources: {}\n", names.join(", ")));
    }

    if let Some(sinks) = json
        .pointer("/environment/sinks")
        .and_then(|v| v.as_array())
    {
        let names: Vec<&str> = sinks
            .iter()
            .filter_map(|s| s.pointer("/info/name").and_then(|v| v.as_str()))
            .collect();
        summary.push_str(&format!("Sinks: {}\n", names.join(", ")));
    }

    summary
}

// --- Mock fallback (used when neither bert-rag nor Ollama is running) ---

fn mock_response(message: &str, context: &str) -> String {
    let model_info = parse_model_facts(context);
    let message_lower = message.to_lowercase();

    if message_lower.contains("what is")
        && (message_lower.contains("system") || message_lower.contains("this"))
    {
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
        format!(
            "{}\n\n**Interface Processors:**\n{}",
            model_info, processors
        )
    } else {
        format!(
            "{}\n\n*Mock mode — no LLM backend detected. Start bert-rag with `launch start facets` or Ollama with `ollama run {OLLAMA_MODEL}`*",
            model_info
        )
    }
}

fn find_s0(json: &serde_json::Value) -> Option<&serde_json::Value> {
    json.get("systems")?
        .as_array()?
        .iter()
        .find(|s| s.pointer("/info/level").and_then(|v| v.as_i64()) == Some(0))
}

fn parse_model_facts(context: &str) -> String {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(context) else {
        return "Unable to parse model data.".to_string();
    };

    let s0 = find_s0(&json);
    let name = s0
        .and_then(|s| s.pointer("/info/name"))
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| {
            json.pointer("/environment/info/name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown")
        });
    let desc = s0
        .and_then(|s| s.pointer("/info/description"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let systems_count = json
        .get("systems")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let interactions_count = json
        .get("interactions")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let sources_count = json
        .pointer("/environment/sources")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let sinks_count = json
        .pointer("/environment/sinks")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    let subsystem_names: Vec<&str> = json
        .get("systems")
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
        desc_line = if desc.is_empty() {
            String::new()
        } else {
            format!("• **Description**: {desc}\n")
        },
        subs = if subsystem_names.is_empty() {
            "None".to_string()
        } else {
            subsystem_names.join(", ")
        },
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
                    let archetype = s
                        .get("archetype")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unspecified");
                    let parent_iface = s
                        .pointer("/boundary/parent_interface")
                        .and_then(|v| v.as_str());
                    let role = if parent_iface.is_some() {
                        "processor"
                    } else {
                        "independent"
                    };
                    Some(format!(
                        "{}. **{}** ({}) — {} [{}]",
                        i + 1,
                        name,
                        id,
                        archetype,
                        role
                    ))
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
                    Some(format!(
                        "{}. **{}** — {} → {} [{}]",
                        i + 1,
                        name,
                        src,
                        snk,
                        ty
                    ))
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
            let procs: Vec<String> = arr
                .iter()
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
        items
            .iter()
            .map(|s| format!("• {s}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
