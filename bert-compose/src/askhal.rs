//! Sovereign in-app analysis: send the run digest to the local hal stack
//! (LiteLLM proxy at :4000) and get a plain-language read back. Local model
//! by default — no data leaves the machine. Non-blocking: a worker thread
//! posts the request and returns the answer over a channel.

use std::sync::mpsc::{channel, Receiver};

const PROXY: &str = "http://localhost:4000/v1/chat/completions";
const KEY: &str = "sk-litellm-local-dev";

const SYSTEM: &str = "You are a systems scientist reading the output of a process-primitive \
circuit (Mobus atomic work processes: buffers store, sensing reads, modulating gates, \
inverting flips a signal, etc; matter/energy conserve, information copies). Given a run \
summary, explain in plain language what the system DID and WHY — name the dynamic \
(e.g. negative feedback, regulation, oscillation, decay, conservation), point to the \
evidence, and suggest one experiment. Be concrete and brief (≤150 words). Write for a \
curious non-engineer.";

/// Kicks off the request; poll the returned receiver each frame.
pub fn ask(summary: String, model: String) -> Receiver<Result<String, String>> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let body = serde_json::json!({
            "model": model,
            "messages": [
                { "role": "system", "content": SYSTEM },
                { "role": "user", "content": summary },
            ],
            "max_tokens": 400,
        });
        let result = ureq::post(PROXY)
            .timeout(std::time::Duration::from_secs(120))
            .set("Authorization", &format!("Bearer {KEY}"))
            .send_json(body)
            .map_err(|e| {
                format!("hal unreachable: {e}\n\nIs the proxy up? `launch start litellm-proxy`")
            })
            .and_then(|resp| {
                resp.into_json::<serde_json::Value>()
                    .map_err(|e| format!("parse: {e}"))
            })
            .map(|j| {
                j["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or("(empty response)")
                    .trim()
                    .to_string()
            });
        let _ = tx.send(result);
    });
    rx
}

/// Local-first model menu (sovereign by default; cloud options last).
pub const MODELS: &[&str] = &[
    "llama3",
    "mistral-small",
    "gemma4",
    "claude-haiku",
    "claude-sonnet",
];

pub fn is_local(model: &str) -> bool {
    !(model.starts_with("claude") || model.starts_with("gemini") || model.starts_with("gpt"))
}
