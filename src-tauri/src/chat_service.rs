use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[cfg(feature = "local-llm")]
use ollama_rs::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage, MessageRole},
    },
    Ollama,
};

#[cfg(feature = "cloud-api")]
use async_openai::{Client as OpenAIClient, types::*};

// #[cfg(feature = "cloud-api")]
// use anthropic::{Client as AnthropicClient, types::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub model_context: String, // Serialized JSON model
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub response: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum LLMProvider {
    #[cfg(feature = "local-llm")]
    Local(Ollama, String), // (ollama_client, model_name)
    // #[cfg(feature = "cloud-api")]
    // Claude(AnthropicClient, String), // (claude_client, model_name)
    #[cfg(feature = "cloud-api")]
    OpenAI(OpenAIClient, String), // (openai_client, model_name)
    Mock, // Fallback when no providers available
}

#[derive(Clone)]
pub struct MinimalLLMService {
    model_path: Option<PathBuf>,
    provider: LLMProvider,
}

impl MinimalLLMService {
    pub fn new() -> Result<Self> {
        // Provider priority: 1) Local Ollama (default), 2) Claude, 3) OpenAI, 4) Mock
        let provider = Self::detect_best_provider();

        Ok(Self {
            model_path: None,
            provider,
        })
    }

    fn detect_best_provider() -> LLMProvider {
        // First try local Ollama - this is the default and preferred option
        #[cfg(feature = "local-llm")]
        {
            // Always try local first - this preserves existing behavior
            let ollama = Ollama::new("http://localhost", 11434);
            return LLMProvider::Local(ollama, "llama3.2:3b".to_string());
        }

        // If local not available, check for cloud API keys
        #[cfg(feature = "cloud-api")]
        {
            // For now, use OpenAI as the primary cloud option
            if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
                if !api_key.is_empty() {
                    let client = OpenAIClient::new();
                    return LLMProvider::OpenAI(client, "gpt-4o-mini".to_string());
                }
            }

            // Claude support temporarily disabled due to API version compatibility
            // TODO: Re-enable once anthropic crate API is updated
            // if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
            //     if !api_key.is_empty() {
            //         let client = AnthropicClient::new(api_key);
            //         return LLMProvider::Claude(client, "claude-3-5-haiku-20241022".to_string());
            //     }
            // }
        }

        // Final fallback to mock responses
        LLMProvider::Mock
    }
    
    pub async fn initialize_model(&mut self, model_name: &str) -> Result<()> {
        match &mut self.provider {
            #[cfg(feature = "local-llm")]
            LLMProvider::Local(_, current_model) => {
                *current_model = model_name.to_string();
            }
            // #[cfg(feature = "cloud-api")]
            // LLMProvider::Claude(_, current_model) => {
            //     *current_model = model_name.to_string();
            // }
            #[cfg(feature = "cloud-api")]
            LLMProvider::OpenAI(_, current_model) => {
                *current_model = model_name.to_string();
            }
            LLMProvider::Mock => {
                // Mock provider doesn't need model initialization
            }
        }
        Ok(())
    }
    
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        match self.generate_llm_response(&request.message, &request.model_context).await {
            Ok(response) => Ok(ChatResponse {
                response,
                error: None,
            }),
            Err(e) => {
                let enhanced_response = Self::create_enhanced_mock_response(&request.message, &request.model_context);
                let provider_info = match &self.provider {
                    #[cfg(feature = "local-llm")]
                    LLMProvider::Local(_, _) => "Local Ollama unavailable. Install with: `curl -fsSL https://ollama.ai/install.sh | sh && ollama pull llama3.2:3b`",
                    // #[cfg(feature = "cloud-api")]
                    // LLMProvider::Claude(_, _) => "Claude API error. Check your ANTHROPIC_API_KEY",
                    #[cfg(feature = "cloud-api")]
                    LLMProvider::OpenAI(_, _) => "OpenAI API error. Check your OPENAI_API_KEY",
                    LLMProvider::Mock => "Using mock responses",
                };
                
                Ok(ChatResponse {
                    response: format!("{}\n\n⚠️ **{}**: {}", enhanced_response, provider_info, e),
                    error: Some(e.to_string()),
                })
            }
        }
    }
    
    async fn generate_llm_response(&self, question: &str, model_context: &str) -> Result<String> {
        match &self.provider {
            #[cfg(feature = "local-llm")]
            LLMProvider::Local(ollama, model_name) => {
                self.chat_with_ollama(question, model_context, ollama, model_name).await
            }
            // #[cfg(feature = "cloud-api")]
            // LLMProvider::Claude(client, model_name) => {
            //     self.chat_with_claude(question, model_context, client, model_name).await
            // }
            #[cfg(feature = "cloud-api")]
            LLMProvider::OpenAI(client, model_name) => {
                self.chat_with_openai(question, model_context, client, model_name).await
            }
            LLMProvider::Mock => {
                Ok(Self::create_enhanced_mock_response(question, model_context))
            }
        }
    }
    
    #[cfg(feature = "local-llm")]
    async fn chat_with_ollama(&self, question: &str, model_context: &str, ollama: &Ollama, model_name: &str) -> Result<String> {
        let context_summary = self.extract_model_summary(model_context);
        
        let system_prompt = self.get_system_prompt();
        let user_prompt = format!(
            "Analyze this BERT system model. Model context: {}\n\nUser question: {}",
            context_summary, question
        );

        let messages = vec![
            ChatMessage::new(MessageRole::System, system_prompt),
            ChatMessage::new(MessageRole::User, user_prompt),
        ];

        let chat_request = ChatMessageRequest::new(model_name.to_string(), messages);
        let response = ollama.send_chat_messages(chat_request).await?;
        
        Ok(response.message.content)
    }
    
    // #[cfg(feature = "cloud-api")]
    // async fn chat_with_claude(&self, question: &str, model_context: &str, client: &AnthropicClient, model_name: &str) -> Result<String> {
    //     let context_summary = self.extract_model_summary(model_context);
    //     let system_prompt = self.get_system_prompt();
    //     let user_prompt = format!(
    //         "Analyze this BERT system model. Model context: {}\n\nUser question: {}",
    //         context_summary, question
    //     );

    //     let request = CreateMessageRequestArgs::default()
    //         .model(model_name)
    //         .system(system_prompt)
    //         .messages(vec![
    //             MessageArgs::default()
    //                 .role(Role::User)
    //                 .content(user_prompt)
    //                 .build()?
    //         ])
    //         .max_tokens(1000)
    //         .build()?;

    //     let response = client.messages().create(request).await?;
        
    //     // Extract text content from response
    //     if let Some(content) = response.content.first() {
    //         if let Content::Text { text } = content {
    //             Ok(text.clone())
    //         } else {
    //             Ok("No text content in response".to_string())
    //         }
    //     } else {
    //         Ok("Empty response from Claude".to_string())
    //     }
    // }
    
    #[cfg(feature = "cloud-api")]
    async fn chat_with_openai(&self, question: &str, model_context: &str, client: &OpenAIClient, model_name: &str) -> Result<String> {
        let context_summary = self.extract_model_summary(model_context);
        let system_prompt = self.get_system_prompt();
        let user_prompt = format!(
            "Analyze this BERT system model. Model context: {}\n\nUser question: {}",
            context_summary, question
        );

        let request = CreateChatCompletionRequestArgs::default()
            .model(model_name)
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(system_prompt)
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_prompt)
                    .build()?
                    .into(),
            ])
            .build()?;

        let response = client.chat().completions().create(request).await?;
        
        Ok(response.choices[0].message.content.clone().unwrap_or_default())
    }
    
    fn get_system_prompt(&self) -> String {
        r#"You are a BERT systems analysis assistant. Report FACTS from JSON data ONLY.

FORBIDDEN WORDS/PHRASES (never use these):
- "appears to be", "seems", "suggests", "likely", "probably", "may be"
- "this system seems to...", "appears to handle...", "looks like..."

REQUIRED FORMAT - Start ALL responses with:
**System Facts:**
• **Name**: [exact string from JSON "name" field]
• **Subsystems**: [exact count] total
• **Interactions**: [exact count] total  
• **Components**: [list exact names from JSON]
• **Flows**: [list exact interaction names/types]

THEN provide factual breakdown:
**Subsystem Analysis:**
[List each subsystem name exactly as written in JSON, with its exact properties]

**Interaction Analysis:**
[List each interaction name exactly as written, with exact source→sink pairs]

RULES:
1. Extract data EXACTLY as written in JSON
2. Count elements precisely 
3. Quote field names and values directly
4. State what IS present, not what it might mean
5. If unsure about a fact, state "Data not available" instead of guessing

Example BAD response: "This appears to be a Bitcoin system that seems to handle..."
Example GOOD response: "This system contains 4 subsystems named: Protocol, Validating, Mining, Network Distribution. The system has 12 interactions including..."

Report the DATA, don't interpret its meaning."#.to_string()
    }
    
    fn extract_model_summary(&self, model_context: &str) -> String {
        // Parse JSON and extract key information for the LLM
        if let Ok(model_data) = serde_json::from_str::<serde_json::Value>(model_context) {
            let mut summary = String::new();
            
            // Extract basic model info
            if let Some(env) = model_data.get("environment") {
                if let Some(info) = env.get("info") {
                    if let Some(name) = info.get("name").and_then(|n| n.as_str()) {
                        summary.push_str(&format!("Model Name: {}\n", name));
                    }
                    if let Some(description) = info.get("description").and_then(|d| d.as_str()) {
                        summary.push_str(&format!("Description: {}\n", description));
                    }
                }
                
                // Extract systems information
                if let Some(systems) = env.get("systems").and_then(|s| s.as_array()) {
                    summary.push_str(&format!("\nSystems ({}): \n", systems.len()));
                    for (i, system) in systems.iter().take(5).enumerate() { // Limit to first 5 for context size
                        if let Some(name) = system.get("name").and_then(|n| n.as_str()) {
                            summary.push_str(&format!("  {}. {}", i + 1, name));
                            if let Some(desc) = system.get("description").and_then(|d| d.as_str()) {
                                summary.push_str(&format!(" - {}", desc));
                            }
                            summary.push('\n');
                        }
                    }
                    if systems.len() > 5 {
                        summary.push_str(&format!("  ... and {} more systems\n", systems.len() - 5));
                    }
                }
                
                // Extract interactions information
                if let Some(interactions) = env.get("interactions").and_then(|i| i.as_array()) {
                    summary.push_str(&format!("\nInteractions ({}): \n", interactions.len()));
                    for (i, interaction) in interactions.iter().take(3).enumerate() { // Limit to first 3
                        if let Some(name) = interaction.get("name").and_then(|n| n.as_str()) {
                            summary.push_str(&format!("  {}. {}", i + 1, name));
                            if let Some(source) = interaction.get("source").and_then(|s| s.as_str()) {
                                if let Some(sink) = interaction.get("sink").and_then(|s| s.as_str()) {
                                    summary.push_str(&format!(" (from {} to {})", source, sink));
                                }
                            }
                            summary.push('\n');
                        }
                    }
                    if interactions.len() > 3 {
                        summary.push_str(&format!("  ... and {} more interactions\n", interactions.len() - 3));
                    }
                }
            }
            
            // Provide the raw JSON as well for complex queries
            summary.push_str(&format!("\nFull Model Data:\n{}", model_context));
            summary
        } else {
            format!("Model data: {}", model_context)
        }
    }
    
    fn create_enhanced_mock_response(message: &str, context: &str) -> String {
        // Parse the JSON context to extract EXACT information
        let model_info = if let Ok(json) = serde_json::from_str::<serde_json::Value>(context) {
            let system_name = json.get("environment")
                .and_then(|env| env.get("info"))
                .and_then(|info| info.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown System");
            
            let description = json.get("environment")
                .and_then(|env| env.get("info"))
                .and_then(|info| info.get("description"))
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let systems_count = json.get("systems")
                .and_then(|v| v.as_array())
                .map(|arr| arr.len())
                .unwrap_or(0);

            let interactions_count = json.get("interactions")
                .and_then(|v| v.as_array())
                .map(|arr| arr.len())
                .unwrap_or(0);

            // Extract exact source names
            let sources: Vec<String> = json.get("environment")
                .and_then(|env| env.get("sources"))
                .and_then(|sources| sources.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| item.get("info")?.get("name")?.as_str())
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default();

            // Extract exact sink names  
            let sinks: Vec<String> = json.get("environment")
                .and_then(|env| env.get("sinks"))
                .and_then(|sinks| sinks.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| item.get("info")?.get("name")?.as_str())
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default();

            // Extract exact subsystem names
            let subsystems: Vec<String> = json.get("systems")
                .and_then(|systems| systems.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| item.get("info")?.get("name")?.as_str())
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default();

            // Extract exact interaction names and details
            let interactions: Vec<String> = json.get("interactions")
                .and_then(|interactions| interactions.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| {
                            let name = item.get("info")?.get("name")?.as_str()?;
                            let from = item.get("source")?.as_str().unwrap_or("Unknown");
                            let to = item.get("sink")?.as_str().unwrap_or("Unknown");
                            Some(format!("{} (from {} to {})", name, from, to))
                        })
                        .collect()
                })
                .unwrap_or_default();

            // STRUCTURED FACTUAL FORMAT (exactly what we want from real LLM)
            let sources_str = if sources.is_empty() { "None found".to_string() } else { sources.join(", ") };
            let sinks_str = if sinks.is_empty() { "None found".to_string() } else { sinks.join(", ") };
            let subsystems_str = if subsystems.is_empty() { "None found".to_string() } else { subsystems.join(", ") };
            let interactions_str = if interactions.is_empty() { "None found".to_string() } else { interactions.join(", ") };
            
            format!(
                "**System Facts:**\n\
                • **System Name**: {}\n\
                • **Description**: {}\n\
                • **Subsystem Count**: {}\n\
                • **Interaction Count**: {}\n\
                • **Sources**: {}\n\
                • **Sinks**: {}\n\
                • **Subsystems**: {}\n\
                • **Interactions**: {}",
                system_name,
                if description.is_empty() { "Not specified" } else { description },
                systems_count,
                interactions_count,
                sources_str,
                sinks_str,
                subsystems_str,
                interactions_str
            )
        } else {
            "**System Facts**: Unable to parse model data - please load a valid JSON model file.".to_string()
        };

        let message_lower = message.to_lowercase();
        
        // Provide FACTUAL responses based on exact data
        if message_lower.contains("what is") && (message_lower.contains("system") || message_lower.contains("this")) {
            format!("{}\n\nThese facts are extracted directly from your loaded JSON model data.", model_info)
        } else if message_lower.contains("sources") || message_lower.contains("source") {
            let sources_data = if let Ok(json) = serde_json::from_str::<serde_json::Value>(context) {
                json.get("environment")
                    .and_then(|env| env.get("sources"))
                    .and_then(|sources| sources.as_array())
                    .map(|arr| {
                        if arr.is_empty() {
                            "No sources found in the model data.".to_string()
                        } else {
                            let source_list: Vec<String> = arr.iter()
                                .filter_map(|item| item.get("info")?.get("name")?.as_str())
                                .map(|s| format!("• {}", s))
                                .collect();
                            format!("**Sources in the model:**\n{}", source_list.join("\n"))
                        }
                    })
                    .unwrap_or("No sources section found in model data.".to_string())
            } else {
                "Unable to parse model data.".to_string()
            };
            format!("{}\n\n{}", model_info, sources_data)
        } else if message_lower.contains("components") || message_lower.contains("subsystems") {
            let components_data = if let Ok(json) = serde_json::from_str::<serde_json::Value>(context) {
                json.get("systems")
                    .and_then(|systems| systems.as_array())
                    .map(|arr| {
                        if arr.is_empty() {
                            "No subsystems found in the model data.".to_string()
                        } else {
                            let component_list: Vec<String> = arr.iter()
                                .filter_map(|item| item.get("info")?.get("name")?.as_str())
                                .enumerate()
                                .map(|(i, s)| format!("{}. {}", i + 1, s))
                                .collect();
                            format!("**Subsystems in the model:**\n{}", component_list.join("\n"))
                        }
                    })
                    .unwrap_or("No systems section found in model data.".to_string())
            } else {
                "Unable to parse model data.".to_string()
            };
            format!("{}\n\n{}", model_info, components_data)
        } else if message_lower.contains("interactions") || message_lower.contains("flows") {
            let interactions_data = if let Ok(json) = serde_json::from_str::<serde_json::Value>(context) {
                json.get("interactions")
                    .and_then(|interactions| interactions.as_array())
                    .map(|arr| {
                        if arr.is_empty() {
                            "No interactions found in the model data.".to_string()
                        } else {
                            let interaction_list: Vec<String> = arr.iter()
                                .enumerate()
                                .filter_map(|(i, item)| {
                                    let name = item.get("info")?.get("name")?.as_str()?;
                                    let from = item.get("source")?.as_str().unwrap_or("Unknown source");
                                    let to = item.get("sink")?.as_str().unwrap_or("Unknown sink");
                                    Some(format!("{}. {} (flows from {} to {})", i + 1, name, from, to))
                                })
                                .collect();
                            format!("**Interactions in the model:**\n{}", interaction_list.join("\n"))
                        }
                    })
                    .unwrap_or("No interactions section found in model data.".to_string())
            } else {
                "Unable to parse model data.".to_string()
            };
            format!("{}\n\n{}", model_info, interactions_data)
        } else {
            format!("{}\n\n💡 **Note**: This factual analysis is from your loaded model data. For enhanced AI analysis, ensure Ollama is running:\n```\nollama pull llama3.2:3b\nollama run llama3.2:3b\n```", model_info)
        }
    }
}

impl Default for MinimalLLMService {
    fn default() -> Self {
        Self::new().unwrap()
    }
} 