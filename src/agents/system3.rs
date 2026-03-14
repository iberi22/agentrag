//! System 3 - Actor Agent con LLM
//!
//! Ejecuta acciones y genera respuestas usando LLM.

use anyhow::Result;
use chrono::{Datelike, Duration, NaiveDate};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
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
        System3Actor::heuristic_answer(query, docs)
    }
}

fn clean_date(text: &str) -> String {
    let trimmed = text.trim();

    if let Some((_, after_on)) = trimmed.rsplit_once(" on ") {
        let year = trimmed
            .split(',')
            .nth(1)
            .map(str::trim)
            .filter(|part| !part.is_empty());

        return match year {
            Some(year) if !after_on.contains(year) => format!("{} {}", after_on.trim(), year),
            _ => after_on.trim().to_string(),
        };
    }

    if let Some((before_comma, after_comma)) = trimmed.split_once(',') {
        let before = before_comma.trim();
        let after = after_comma.trim();
        if before.chars().any(|ch| ch.is_ascii_digit())
            && after.contains(':')
            && after
                .chars()
                .all(|ch| ch.is_ascii_digit() || ch == ':' || ch.is_whitespace())
        {
            return before.to_string();
        }
        if before.chars().all(|ch| ch.is_ascii_digit()) || before.chars().all(|ch| ch.is_alphabetic())
        {
            return format!("{before}, {after}");
        }
    }

    trimmed.to_string()
}

fn date_patterns() -> &'static [Regex] {
    static DATE_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    DATE_PATTERNS
        .get_or_init(|| {
            vec![
                Regex::new(r"(?i)\b\d{1,2}\s+[A-Za-z]+\s+\d{4}\b").expect("day month year regex"),
                Regex::new(r"(?i)\b[A-Za-z]+\s+\d{1,2},\s+\d{4}\b")
                    .expect("month day year regex"),
                Regex::new(r"\b(19|20)\d{2}\b").expect("year regex"),
            ]
        })
        .as_slice()
}

fn snippet(text: &str, max_chars: usize) -> String {
    text.chars().take(max_chars).collect::<String>().trim().to_string()
}

fn top_non_empty_contents(docs: &[RetrievedDocument], limit: usize) -> Vec<String> {
    docs.iter()
        .filter_map(|doc| {
            let text = doc.content.trim();
            (!text.is_empty()).then(|| text.to_string())
        })
        .take(limit)
        .collect()
}

fn query_lower(query: &str) -> String {
    query.to_lowercase()
}

fn query_terms(query: &str) -> Vec<String> {
    query
        .to_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .filter(|term| {
            let term = *term;
            term.len() > 2
                && !matches!(
                    term,
                    "when"
                        | "what"
                        | "have"
                        | "that"
                        | "with"
                        | "from"
                        | "into"
                        | "this"
                        | "your"
                        | "about"
                        | "did"
                        | "does"
                        | "the"
                        | "and"
                        | "for"
                        | "who"
                        | "why"
                        | "how"
                        | "where"
                        | "was"
                        | "were"
                        | "after"
                        | "before"
                        | "they"
                        | "them"
                        | "went"
                )
        })
        .map(|term| term.to_string())
        .collect()
}

fn query_phrases(terms: &[String]) -> Vec<String> {
    if terms.len() < 2 {
        return Vec::new();
    }

    terms.windows(2).map(|window| window.join(" ")).collect()
}

fn extract_date_answer(text: &str) -> Option<String> {
    for pattern in date_patterns() {
        if let Some(found) = pattern.find(text) {
            return Some(clean_date(found.as_str()));
        }
    }
    None
}

fn parse_session_date(session_time: &str) -> Option<NaiveDate> {
    let date_text = session_time
        .rsplit_once(" on ")
        .map(|(_, date_text)| date_text.trim())
        .unwrap_or_else(|| session_time.trim());

    NaiveDate::parse_from_str(date_text, "%e %B, %Y").ok()
}

fn format_date(date: NaiveDate) -> String {
    date.format("%-d %B %Y").to_string()
}

fn extract_relative_date_answer(text: &str, session_time: &str) -> Option<String> {
    let lowered = text.to_lowercase();
    let session_date = parse_session_date(session_time)?;

    if lowered.contains("yesterday") {
        return Some(format_date(session_date - Duration::days(1)));
    }

    if lowered.contains("last year") {
        return Some((session_date.year() - 1).to_string());
    }

    None
}

fn has_temporal_signal(text: &str) -> bool {
    let lowered = text.to_lowercase();
    extract_date_answer(text).is_some()
        || lowered.contains("yesterday")
        || lowered.contains("last year")
        || lowered.contains("last month")
        || lowered.contains("last week")
}

fn doc_category(doc: &RetrievedDocument) -> &str {
    doc.metadata
        .get("category")
        .and_then(|value| value.as_str())
        .unwrap_or_default()
}

fn doc_text_for_scoring(doc: &RetrievedDocument) -> String {
    let mut parts = vec![doc.path.clone(), doc.content.clone()];

    if let Some(map) = doc.metadata.as_object() {
        for value in map.values() {
            match value {
                serde_json::Value::String(text) => parts.push(text.clone()),
                serde_json::Value::Array(items) => {
                    for item in items {
                        if let Some(text) = item.as_str() {
                            parts.push(text.to_string());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    parts.join(" ")
}

fn doc_answer_text(doc: &RetrievedDocument) -> String {
    let mut parts = vec![doc.content.clone()];

    if let Some(map) = doc.metadata.as_object() {
        for (key, value) in map {
            if key == "session_time" {
                continue;
            }

            match value {
                serde_json::Value::String(text) => parts.push(text.clone()),
                serde_json::Value::Array(items) => {
                    for item in items {
                        if let Some(text) = item.as_str() {
                            parts.push(text.to_string());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    parts.join(" ")
}

fn score_doc_for_query(doc: &RetrievedDocument, terms: &[String]) -> usize {
    if terms.is_empty() {
        return 0;
    }

    let searchable_text = doc_text_for_scoring(doc);
    let searchable_lower = searchable_text.to_lowercase();
    let content_lower = doc.content.to_lowercase();
    let speaker_lower = doc
        .metadata
        .get("speaker")
        .and_then(|value| value.as_str())
        .unwrap_or_default()
        .to_lowercase();
    let category = doc_category(doc);

    let mut score = 0usize;
    for term in terms {
        if speaker_lower == *term {
            score += 4;
        }
        if searchable_lower.contains(term) {
            score += 2;
        }
        if content_lower.contains(term) {
            score += 2;
        }
    }

    for phrase in query_phrases(terms) {
        if content_lower.contains(&phrase) {
            score += 5;
        } else if searchable_lower.contains(&phrase) {
            score += 2;
        }
    }

    if has_temporal_signal(&searchable_text) {
        score += 3;
    }

    if has_temporal_signal(&doc.content) {
        score += 6;
    }

    match category {
        "conversation" => score += 5,
        "observation" => score += 1,
        "session_summary" => score = score.saturating_sub(4),
        _ => {}
    }

    score
}

fn term_overlap_in_content(doc: &RetrievedDocument, terms: &[String]) -> usize {
    let content_lower = doc.content.to_lowercase();
    terms.iter().filter(|term| content_lower.contains(term.as_str())).count()
}

fn best_date_answer(query: &str, docs: &[RetrievedDocument]) -> Option<String> {
    let terms = query_terms(query);
    if let Some((_, answer)) = docs
        .iter()
        .filter_map(|doc| {
            let session_time = doc
                .metadata
                .get("session_time")
                .and_then(|value| value.as_str())?;
            let answer = extract_relative_date_answer(&doc.content, session_time)?;
            let category_priority = match doc_category(doc) {
                "conversation" => 2usize,
                "observation" => 1usize,
                _ => 0usize,
            };

            Some((
                (
                    category_priority,
                    term_overlap_in_content(doc, &terms),
                    score_doc_for_query(doc, &terms),
                ),
                answer,
            ))
        })
        .max_by_key(|(score, _)| *score)
    {
        return Some(answer);
    }

    let best_doc = docs
        .iter()
        .max_by_key(|doc| {
            let answer_text = doc_answer_text(doc);
            let explicit = extract_date_answer(&answer_text).is_some();
            let category_priority = match doc_category(doc) {
                "conversation" => 2usize,
                "observation" => 1usize,
                _ => 0usize,
            };

            (
                category_priority,
                usize::from(explicit),
                term_overlap_in_content(doc, &terms),
                score_doc_for_query(doc, &terms),
            )
        })
        .or_else(|| docs.first())?;

    let answer_text = doc_answer_text(best_doc);
    extract_date_answer(&answer_text)
        .or_else(|| {
            best_doc
                .metadata
                .get("session_time")
                .and_then(|value| value.as_str())
                .and_then(|session_time| extract_relative_date_answer(&answer_text, session_time))
        })
        .or_else(|| {
            best_doc
                .metadata
                .get("session_time")
                .and_then(|value| value.as_str())
                .map(clean_date)
        })
}

impl System3Actor {
    fn heuristic_answer(query: &str, docs: &[RetrievedDocument]) -> String {
        if docs.is_empty() {
            return format!(
                "I couldn't find sufficient information to answer your query about '{}'.",
                query
            );
        }

        let lowered = query_lower(query);

        if lowered.contains("when") {
            if let Some(answer) = best_date_answer(query, docs) {
                return answer;
            }
        }

        if lowered.contains("what do") && lowered.contains("both")
            || lowered.contains("what do") && lowered.contains("have in common")
            || lowered.contains("how do") && lowered.contains("both")
        {
            let joined = top_non_empty_contents(docs, 2).join(" ");
            if !joined.is_empty() {
                return snippet(&joined, 220);
            }
        }

        if let Some(first) = docs.first() {
            let text = first.content.trim();
            if !text.is_empty() {
                return snippet(text, 220);
            }
        }

        "I found relevant memory, but the best answer could not be synthesized yet.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{best_date_answer, clean_date, extract_date_answer, extract_relative_date_answer};
    use crate::agents::system1::RetrievedDocument;

    fn doc(content: &str, session_time: Option<&str>, speaker: Option<&str>) -> RetrievedDocument {
        RetrievedDocument {
            id: "doc-1".to_string(),
            path: "locomo/conv-1/session_1/D1:1".to_string(),
            content: content.to_string(),
            relevance_score: 1.0,
            metadata: serde_json::json!({
                "session_time": session_time,
                "speaker": speaker,
            }),
        }
    }

    #[test]
    fn clean_date_keeps_readable_format() {
        assert_eq!(clean_date("Monday on 7 May 2023"), "7 May 2023");
        assert_eq!(clean_date("8 May, 2023"), "8 May, 2023");
        assert_eq!(clean_date("7 May 2023, 18:00"), "7 May 2023");
    }

    #[test]
    fn extract_date_answer_prefers_explicit_years_and_dates() {
        assert_eq!(
            extract_date_answer("Melanie painted a sunrise in 2022 for a school mural."),
            Some("2022".to_string())
        );
        assert_eq!(
            extract_date_answer("The event happened on 7 May 2023 after work."),
            Some("7 May 2023".to_string())
        );
    }

    #[test]
    fn extract_relative_date_answer_resolves_against_session_time() {
        assert_eq!(
            extract_relative_date_answer(
                "Caroline: I went to a LGBTQ support group yesterday and it was so powerful.",
                "1:56 pm on 8 May, 2023"
            ),
            Some("7 May 2023".to_string())
        );
        assert_eq!(
            extract_relative_date_answer(
                "Yeah, I painted that lake sunrise last year! It's special to me.",
                "1:56 pm on 8 May, 2023"
            ),
            Some("2022".to_string())
        );
    }

    #[test]
    fn best_date_answer_uses_matching_document_before_other_session_times() {
        let docs = vec![
            doc(
                "Melanie: Yeah, I painted that lake sunrise last year! It's special to me.",
                Some("15 July, 2023"),
                Some("Melanie"),
            ),
            doc(
                "Caroline: I went to a LGBTQ support group yesterday and it was so powerful.",
                Some("1:56 pm on 8 May, 2023"),
                Some("Caroline"),
            ),
        ];

        assert_eq!(
            best_date_answer("When did Caroline go to the LGBTQ support group?", &docs),
            Some("7 May 2023".to_string())
        );
        assert_eq!(
            best_date_answer("When did Melanie paint a sunrise?", &docs),
            Some("2022".to_string())
        );
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
        Self::heuristic_answer(query, docs)
    }
}
