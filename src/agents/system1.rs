//! System 1 - Retriever Agent
//!
//! Recibe queries del usuario, busca en memoria híbrida y retorna contexto relevante.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, warn};

use crate::memory::qmd_memory::QmdMemory;

/// Response del System 1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalResult {
    pub query: String,
    pub documents: Vec<RetrievedDocument>,
    pub search_type: SearchType,
    pub total_results: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedDocument {
    pub id: String,
    pub path: String,
    pub content: String,
    pub relevance_score: f32,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    Hybrid,
    Semantic,
    Keyword,
}

/// Configuración del Retriever
#[derive(Debug, Clone)]
pub struct RetrieverConfig {
    pub max_results: usize,
    pub min_relevance_score: f32,
    pub default_search_type: SearchType,
}

impl Default for RetrieverConfig {
    fn default() -> Self {
        Self {
            max_results: 10,
            min_relevance_score: 0.3,
            default_search_type: SearchType::Hybrid,
        }
    }
}

/// System 1 - Retriever Agent (simplificado para compilación)
pub struct System1Retriever {
    memory: Arc<QmdMemory>,
    config: RetrieverConfig,
}

impl System1Retriever {
    /// Crea un nuevo retriever
    pub fn new(memory: Arc<QmdMemory>, config: RetrieverConfig) -> Self {
        Self { memory, config }
    }

    /// Ejecuta la retrieval de manera asíncrona
    pub async fn run(
        &self,
        query: &str,
        search_type: Option<SearchType>,
    ) -> Result<RetrievalResult> {
        let start = std::time::Instant::now();

        info!("🔍 System1 starting retrieval for query: {}", query);

        let selected_search_type = search_type.unwrap_or(self.config.default_search_type.clone());
        let raw_documents = match selected_search_type {
            SearchType::Keyword => self.memory.search(query, self.config.max_results).await?,
            SearchType::Hybrid => match self
                .memory
                .query(query, vec![0.0; 1536], self.config.max_results)
                .await
            {
                Ok(results) => results,
                Err(error) => {
                    warn!("Hybrid search failed, falling back to BM25: {}", error);
                    self.memory.search(query, self.config.max_results).await?
                }
            },
            SearchType::Semantic => {
                warn!(
                    "Semantic search requested without embeddings provider; falling back to BM25"
                );
                self.memory.search(query, self.config.max_results).await?
            }
        };

        let documents: Vec<RetrievedDocument> = raw_documents
            .into_iter()
            .enumerate()
            .map(|(index, doc)| RetrievedDocument {
                id: doc
                    .id
                    .clone()
                    .unwrap_or_else(|| format!("memory:{}", index)),
                path: doc.path,
                content: doc.content,
                relevance_score: 1.0,
                metadata: doc.metadata,
            })
            .collect();

        let total = documents.len();

        info!(
            "✅ System1 retrieved {} documents in {:?}",
            total,
            start.elapsed()
        );

        Ok(RetrievalResult {
            query: query.to_string(),
            documents,
            search_type: selected_search_type,
            total_results: total,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retriever_config_defaults() {
        let config = RetrieverConfig::default();

        assert_eq!(config.max_results, 10);
        assert_eq!(config.min_relevance_score, 0.3);
        assert!(matches!(config.default_search_type, SearchType::Hybrid));
    }
}
