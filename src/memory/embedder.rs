use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_EMBEDDING_URL: &str = "http://localhost:8002";
const DEFAULT_EMBEDDING_MODEL: &str = "perplexity-ai/pplx-embed-v1-0.6b";

#[derive(Debug, Clone)]
pub struct EmbeddingClient {
    client: Client,
    base_url: String,
    model: String,
}

impl EmbeddingClient {
    pub fn from_env() -> Result<Self> {
        let base_url = std::env::var("CORTEX_EMBEDDING_URL")
            .unwrap_or_else(|_| DEFAULT_EMBEDDING_URL.to_string());
        let model = std::env::var("CORTEX_EMBEDDING_MODEL")
            .unwrap_or_else(|_| DEFAULT_EMBEDDING_MODEL.to_string());

        Self::new(base_url, model)
    }

    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .context("failed to build embedding HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
            model: model.into(),
        })
    }

    pub async fn embed(&self, input: &str) -> Result<Vec<f32>> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        let response = self
            .client
            .post(format!("{}/v1/embeddings", self.base_url))
            .json(&EmbeddingRequest {
                input: trimmed,
                model: &self.model,
            })
            .send()
            .await
            .with_context(|| format!("failed to call embeddings service at {}", self.base_url))?
            .error_for_status()
            .context("embeddings service returned an error")?;

        let payload: EmbeddingResponse = response
            .json()
            .await
            .context("failed to decode embeddings response")?;

        payload
            .first_embedding()
            .ok_or_else(|| anyhow!("embeddings response did not contain a vector"))
    }
}

#[derive(Debug, Serialize)]
struct EmbeddingRequest<'a> {
    input: &'a str,
    model: &'a str,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    #[serde(default)]
    data: Vec<EmbeddingDatum>,
    embedding: Option<Vec<f32>>,
}

impl EmbeddingResponse {
    fn first_embedding(self) -> Option<Vec<f32>> {
        self.embedding
            .or_else(|| self.data.into_iter().next().map(|datum| datum.embedding))
    }
}

#[derive(Debug, Deserialize)]
struct EmbeddingDatum {
    embedding: Vec<f32>,
}
