//! System 3 - Actor Agent con LLM
//!
//! Ejecuta acciones y genera respuestas usando LLM.

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::agents::system1::{RetrievalResult, RetrievedDocument};
use crate::agents::system2::ReasoningResult;

/// Cliente LLM para generar respuestas
pub struct LlmClient {
    client: Client,
    api_key: String,
    model: String,
    endpoint: String,
}

impl LlmClient {
    pub fn new() -> Self {
        let api_key = std::env::var("MINIMAX_API_KEY")
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .unwrap_or_else(|_| "demo-key".to_string());

        let model = "MiniMax-Text-01".to_string();
        let endpoint = "https://api.minimax.chat/v1/text/chatcompletion_pro".to_string();

        Self {
            client: Client::new(),
            api_key,
            model,
            endpoint,
        }
    }

    pub async fn generate_response(
        &self,
        query: &str,
        context: &[RetrievedDocument],
    ) -> Result<String> {
        // Build context from retrieved documents
        let context_text = context
            .iter()
            .map(|d| format!("- {}\n  Source: {}", d.content, d.path))
            .collect::<Vec<_>>()
            .join("\n\n");

        let system_prompt = r#"You are a helpful AI assistant part of the Cortex memory system. 
You have access to relevant documents from the memory store. Use this context to answer the user's question accurately.

If you find relevant information in the context, use it to form your answer.
If you don't find enough information, say so honestly.
Be concise but informative."#;

        let user_prompt = format!(
            "Context from memory:\n{}\n\nUser question: {}",
            context_text, query
        );

        // If no API key, use fallback
        if self.api_key == "demo-key" || self.api_key.is_empty() {
            return Ok(Self::fallback_response(query, context));
        }

        // Make API call
        let request_body = serde_json::json!({
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_prompt}
            ],
            "model": self.model,
            "temperature": 0.7,
            "max_tokens": 500
        });

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json: serde_json::Value = resp.json().await?;
                    // Parse MiniMax response format
                    if let Some(choices) = json["choices"].as_array() {
                        if let Some(choice) = choices.first() {
                            if let Some(message) = choice.get("message") {
                                if let Some(text) = message.get("content").and_then(|c| c.as_str())
                                {
                                    return Ok(text.to_string());
                                }
                            }
                        }
                    }
                    Ok(Self::fallback_response(query, context))
                } else {
                    warn!("LLM API error: {}", resp.status());
                    Ok(Self::fallback_response(query, context))
                }
            }
            Err(e) => {
                error!("LLM request failed: {}", e);
                Ok(Self::fallback_response(query, context))
            }
        }
    }

    fn fallback_response(query: &str, docs: &[RetrievedDocument]) -> String {
        if docs.is_empty() {
            return format!(
                "I couldn't find sufficient information to answer your query about '{}'.",
                query
            );
        }

        let doc_count = docs.len();
        let first_doc = docs.first().map(|d| d.content.as_str()).unwrap_or("");

        format!(
            "Based on my analysis: Found {} relevant documents for query '{}'. \n\nFirst result: {}...\n\nI can provide more details if needed.",
            doc_count,
            query,
            &first_doc[..first_doc.char_indices().nth(100).map(|(i, _)| i).unwrap_or(first_doc.len())]
        )
    }
}

/// Response del System 3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub query: String,
    pub response: String,
    pub actions_taken: Vec<Action>,
    pub memory_updates: Vec<MemoryUpdate>,
    pub tool_calls: Vec<ToolCall>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: ActionType,
    pub description: String,
    pub target: Option<String>,
    pub result: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Response,
    MemoryStore,
    ToolExecution,
    BeliefUpdate,
    NoOp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUpdate {
    pub path: String,
    pub content: String,
    pub operation: MemoryOperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryOperation {
    Create,
    Update,
    Delete,
    Compress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool_name: String,
    pub arguments: serde_json::Value,
    pub result: Option<String>,
}

/// Config del Actor
#[derive(Debug, Clone)]
pub struct ActorConfig {
    pub use_llm: bool,
    pub max_actions: usize,
}

impl Default for ActorConfig {
    fn default() -> Self {
        Self {
            use_llm: true,
            max_actions: 5,
        }
    }
}

/// System 3 - Actor Agent
pub struct System3Actor {
    config: ActorConfig,
    llm_client: LlmClient,
}

impl System3Actor {
    pub fn new(config: ActorConfig) -> Self {
        Self {
            config,
            llm_client: LlmClient::new(),
        }
    }

    pub async fn run(
        &self,
        query: &str,
        retrieval_result: &RetrievalResult,
        _reasoning_result: &ReasoningResult,
    ) -> Result<ActionResult> {
        info!("🎬 System3 executing for query: {}", query);

        // Generate response using LLM with context
        let response = if self.config.use_llm {
            self.llm_client
                .generate_response(query, &retrieval_result.documents)
                .await
                .unwrap_or_else(|e| {
                    warn!("LLM generation failed: {}", e);
                    Self::simple_response(query, &retrieval_result.documents)
                })
        } else {
            Self::simple_response(query, &retrieval_result.documents)
        };

        Ok(ActionResult {
            query: query.to_string(),
            response,
            actions_taken: vec![],
            memory_updates: vec![],
            tool_calls: vec![],
            success: true,
        })
    }

    fn simple_response(query: &str, docs: &[RetrievedDocument]) -> String {
        if docs.is_empty() {
            return format!(
                "I couldn't find sufficient information to answer your query about '{}'.",
                query
            );
        }

        let count = docs.len();
        format!(
            "Found {} relevant documents for '{}':\n\n{}",
            count,
            query,
            docs.iter()
                .take(3)
                .map(|d| format!(
                    "• {} (relevance: {:.2})",
                    d.content.chars().take(80).collect::<String>(),
                    d.relevance_score
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
