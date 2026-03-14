//! QMD Memory - lightweight in-memory document store with cached search.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering as AtomicOrdering},
        Arc,
    },
};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryDocument {
    pub id: Option<String>,
    pub path: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CacheMetrics {
    pub hits: usize,
    pub misses: usize,
    pub entries: usize,
}

#[derive(Debug, Clone)]
pub struct CachedSearchResult {
    pub documents: Vec<MemoryDocument>,
    pub cache_hit: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SearchCacheKey {
    query: String,
    limit: usize,
}

#[derive(Default)]
struct CacheCounters {
    hits: AtomicUsize,
    misses: AtomicUsize,
}

#[derive(Clone)]
pub struct QmdMemory {
    docs: Arc<RwLock<Vec<MemoryDocument>>>,
    search_cache: Arc<RwLock<HashMap<SearchCacheKey, Vec<MemoryDocument>>>>,
    cache_counters: Arc<CacheCounters>,
}

impl QmdMemory {
    pub fn new(docs: Arc<RwLock<Vec<MemoryDocument>>>) -> Self {
        Self {
            docs,
            search_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_counters: Arc::new(CacheCounters::default()),
        }
    }

    pub async fn init(&self) -> Result<()> {
        Ok(())
    }

    pub async fn search(&self, query_text: &str, limit: usize) -> Result<Vec<MemoryDocument>> {
        Ok(self.search_with_cache(query_text, limit).await?.documents)
    }

    pub async fn search_with_cache(
        &self,
        query_text: &str,
        limit: usize,
    ) -> Result<CachedSearchResult> {
        let normalized_query = normalize_query(query_text);
        let cache_key = SearchCacheKey {
            query: normalized_query.clone(),
            limit,
        };

        if let Some(cached) = self.search_cache.read().await.get(&cache_key).cloned() {
            self.cache_counters
                .hits
                .fetch_add(1, AtomicOrdering::Relaxed);
            return Ok(CachedSearchResult {
                documents: cached,
                cache_hit: true,
            });
        }

        let docs = self.docs.read().await;
        let mut scored: Vec<(f32, MemoryDocument)> = docs
            .iter()
            .filter_map(|doc| {
                let score = lexical_score(doc, &normalized_query);
                (score > 0.0).then(|| (score, doc.clone()))
            })
            .collect();
        drop(docs);

        scored.sort_by(|left, right| {
            right
                .0
                .partial_cmp(&left.0)
                .unwrap_or(Ordering::Equal)
                .then_with(|| left.1.path.cmp(&right.1.path))
        });

        let documents: Vec<MemoryDocument> =
            scored.into_iter().map(|(_, doc)| doc).take(limit).collect();

        self.search_cache
            .write()
            .await
            .insert(cache_key, documents.clone());
        self.cache_counters
            .misses
            .fetch_add(1, AtomicOrdering::Relaxed);

        Ok(CachedSearchResult {
            documents,
            cache_hit: false,
        })
    }

    pub async fn vsearch(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<MemoryDocument>> {
        if query_vector.is_empty() {
            return Ok(Vec::new());
        }

        let docs = self.docs.read().await;
        let mut scored: Vec<(f32, MemoryDocument)> = docs
            .iter()
            .filter_map(|doc| {
                let score = cosine_similarity(&query_vector, &doc.embedding);
                (score > 0.0).then(|| (score, doc.clone()))
            })
            .collect();

        scored.sort_by(|left, right| {
            right
                .0
                .partial_cmp(&left.0)
                .unwrap_or(Ordering::Equal)
                .then_with(|| left.1.path.cmp(&right.1.path))
        });

        Ok(scored.into_iter().map(|(_, doc)| doc).take(limit).collect())
    }

    pub async fn query(
        &self,
        query_text: &str,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<MemoryDocument>> {
        let keyword_results = self.search(query_text, limit).await?;
        if query_vector.is_empty() {
            return Ok(keyword_results);
        }

        let vector_results = self.vsearch(query_vector, limit).await?;
        let mut combined = Vec::with_capacity(limit);
        let mut seen = std::collections::HashSet::new();

        for doc in keyword_results.into_iter().chain(vector_results.into_iter()) {
            let key = doc
                .id
                .clone()
                .unwrap_or_else(|| format!("path:{}", doc.path));
            if seen.insert(key) {
                combined.push(doc);
            }
            if combined.len() >= limit {
                break;
            }
        }

        Ok(combined)
    }

    pub async fn get(&self, path_or_id: &str) -> Result<Option<MemoryDocument>> {
        let docs = self.docs.read().await;
        Ok(docs
            .iter()
            .find(|doc| doc.path == path_or_id || doc.id.as_deref() == Some(path_or_id))
            .cloned())
    }

    pub async fn add(&self, doc: MemoryDocument) -> Result<()> {
        self.docs.write().await.push(doc);
        self.invalidate_cache().await;
        Ok(())
    }

    pub async fn add_document(
        &self,
        path: String,
        content: String,
        metadata: serde_json::Value,
    ) -> Result<()> {
        // Generate embedding using pplx-embed service
        let embedding = generate_embedding(&content).await.unwrap_or_else(|_| Vec::new());
        
        self.add(MemoryDocument {
            id: Some(uuid::Uuid::new_v4().to_string()),
            path,
            content,
            metadata,
            embedding,
        })
        .await
    }

    pub async fn delete(&self, path_or_id: &str) -> Result<Option<MemoryDocument>> {
        let mut docs = self.docs.write().await;
        let removed = docs
            .iter()
            .position(|doc| doc.path == path_or_id || doc.id.as_deref() == Some(path_or_id))
            .map(|index| docs.remove(index));
        drop(docs);

        if removed.is_some() {
            self.invalidate_cache().await;
        }

        Ok(removed)
    }

    pub async fn clear(&self) -> Result<usize> {
        let mut docs = self.docs.write().await;
        let removed = docs.len();
        docs.clear();
        drop(docs);
        self.invalidate_cache().await;
        Ok(removed)
    }

    pub async fn count(&self) -> Result<usize> {
        Ok(self.docs.read().await.len())
    }

    pub async fn cache_metrics(&self) -> CacheMetrics {
        CacheMetrics {
            hits: self.cache_counters.hits.load(AtomicOrdering::Relaxed),
            misses: self.cache_counters.misses.load(AtomicOrdering::Relaxed),
            entries: self.search_cache.read().await.len(),
        }
    }

    async fn invalidate_cache(&self) {
        self.search_cache.write().await.clear();
    }
}

fn normalize_query(query_text: &str) -> String {
    query_text
        .split_whitespace()
        .map(normalize_token)
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_token(token: &str) -> String {
    token
        .chars()
        .filter(|char| char.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

fn lexical_score(doc: &MemoryDocument, normalized_query: &str) -> f32 {
    if normalized_query.is_empty() {
        return 0.0;
    }

    let content = doc.content.to_lowercase();
    let path = doc.path.to_lowercase();

    normalized_query
        .split_whitespace()
        .filter(|term| !term.is_empty())
        .map(|term| {
            let content_hits = content.matches(term).count() as f32;
            let path_hits = path.matches(term).count() as f32 * 2.0;
            content_hits + path_hits
        })
        .sum()
}

pub(crate) fn cosine_similarity(left: &[f32], right: &[f32]) -> f32 {
    if left.is_empty() || right.is_empty() || left.len() != right.len() {
        return 0.0;
    }

    let dot = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a * b)
        .sum::<f32>();
    let left_magnitude = left.iter().map(|value| value * value).sum::<f32>().sqrt();
    let right_magnitude = right.iter().map(|value| value * value).sum::<f32>().sqrt();

    if left_magnitude == 0.0 || right_magnitude == 0.0 {
        return 0.0;
    }

    dot / (left_magnitude * right_magnitude)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn repeated_searches_hit_cache() {
        let memory = QmdMemory::new(Arc::new(RwLock::new(Vec::new())));
        memory
            .add_document(
                "docs/cache".to_string(),
                "cache acceleration for repeated searches".to_string(),
                serde_json::json!({}),
            )
            .await
            .unwrap();

        let first = memory.search_with_cache("cache acceleration", 5).await.unwrap();
        let second = memory.search_with_cache("cache acceleration", 5).await.unwrap();
        let metrics = memory.cache_metrics().await;

        assert!(!first.cache_hit);
        assert!(second.cache_hit);
        assert_eq!(metrics.misses, 1);
        assert_eq!(metrics.hits, 1);
        assert_eq!(metrics.entries, 1);
    }

    #[tokio::test]
    async fn mutating_memory_invalidates_cache() {
        let memory = QmdMemory::new(Arc::new(RwLock::new(Vec::new())));
        memory
            .add_document(
                "docs/original".to_string(),
                "performance tuning for cortex".to_string(),
                serde_json::json!({}),
            )
            .await
            .unwrap();

        let _ = memory.search_with_cache("performance", 5).await.unwrap();
        assert_eq!(memory.cache_metrics().await.entries, 1);

        memory
            .add_document(
                "docs/new".to_string(),
                "new performance tuning guide".to_string(),
                serde_json::json!({}),
            )
            .await
            .unwrap();

        assert_eq!(memory.cache_metrics().await.entries, 0);
    }
}

// ============================================
// pplx-embed Integration for Semantic Search
// ============================================

/// Generate embedding using pplx-embed service
async fn generate_embedding(text: &str) -> Result<Vec<f32>> {
    use std::time::Duration;
    
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;
    
    let response = client
        .post("http://localhost:8002/embed")
        .json(&serde_json::json!({ "text": text }))
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Ok(Vec::new());
    }
    
    let result: serde_json::Value = response.json().await?;
    
    if let Some(embeddings) = result.get("embeddings").and_then(|e| e.as_array()) {
        if let Some(first) = embeddings.first() {
            if let Some(embedding) = first.as_array() {
                return Ok(embedding.iter()
                    .filter_map(|v| v.as_f64().map(|f| f as f32))
                    .collect());
            }
        }
    }
    
    Ok(Vec::new())
}

/// Query with semantic search using embeddings
pub async fn query_with_embedding(memory: &QmdMemory, query_text: &str, limit: usize) -> Result<Vec<MemoryDocument>> {
    let query_vector = generate_embedding(query_text).await?;
    
    if query_vector.is_empty() {
        // Fallback to keyword search
        return memory.search(query_text, limit).await;
    }
    
    memory.query(query_text, query_vector, limit).await
}
