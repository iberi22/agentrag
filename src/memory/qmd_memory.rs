//! QMD Memory - In-memory storage for minimal slice
//!
//! Sistema de memoria con búsqueda simple.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Documento en memoria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryDocument {
    pub id: Option<String>,
    pub path: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub embedding: Vec<f32>,
}

/// QMD Memory - In-memory storage
#[derive(Clone)]
pub struct QmdMemory {
    docs: Arc<RwLock<Vec<MemoryDocument>>>,
}

impl QmdMemory {
    pub fn new(docs: Arc<RwLock<Vec<MemoryDocument>>>) -> Self {
        Self { docs }
    }

    pub async fn init(&self) -> Result<()> {
        Ok(())
    }

    /// search: fast keyword match (simple contains)
    pub async fn search(&self, query_text: &str, limit: usize) -> Result<Vec<MemoryDocument>> {
        let docs = self.docs.read().await;
        let query_lower = query_text.to_lowercase();

        let results: Vec<_> = docs
            .iter()
            .filter(|d| d.content.to_lowercase().contains(&query_lower))
            .take(limit)
            .cloned()
            .collect();

        Ok(results)
    }

    /// vsearch: semantic similarity (placeholder - no embeddings)
    #[allow(dead_code)]
    pub async fn vsearch(
        &self,
        _query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<MemoryDocument>> {
        let docs = self.docs.read().await;
        Ok(docs.iter().take(limit).cloned().collect())
    }

    /// query: hybrid search + LLM reranking (placeholder)
    pub async fn query(
        &self,
        query_text: &str,
        _query_vector: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<MemoryDocument>> {
        self.search(query_text, limit).await
    }

    /// get: retrieve specific document by path or ID
    #[allow(dead_code)]
    pub async fn get(&self, path_or_id: &str) -> Result<Option<MemoryDocument>> {
        let docs = self.docs.read().await;
        Ok(docs
            .iter()
            .find(|d| d.path == path_or_id || d.id.as_deref() == Some(path_or_id))
            .cloned())
    }

    /// add: insert a document
    #[allow(dead_code)]
    pub async fn add(&self, doc: MemoryDocument) -> Result<()> {
        let mut docs = self.docs.write().await;
        docs.push(doc);
        Ok(())
    }

    /// add_document: convenience method to add a document with auto-generated ID
    #[allow(dead_code)]
    pub async fn add_document(
        &self,
        path: String,
        content: String,
        metadata: serde_json::Value,
    ) -> Result<()> {
        let doc = MemoryDocument {
            id: Some(uuid::Uuid::new_v4().to_string()),
            path,
            content,
            metadata,
            embedding: vec![],
        };
        self.add(doc).await
    }

    /// delete: remove a document by path or ID
    pub async fn delete(&self, path_or_id: &str) -> Result<Option<MemoryDocument>> {
        let mut docs = self.docs.write().await;

        if let Some(index) = docs
            .iter()
            .position(|d| d.path == path_or_id || d.id.as_deref() == Some(path_or_id))
        {
            return Ok(Some(docs.remove(index)));
        }

        Ok(None)
    }

    /// count: number of documents
    pub async fn count(&self) -> Result<usize> {
        let docs = self.docs.read().await;
        Ok(docs.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_qmd_memory_creation() {
        let docs: Arc<RwLock<Vec<MemoryDocument>>> = Arc::new(RwLock::new(Vec::new()));
        let memory = QmdMemory::new(docs);
        memory.init().await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_by_path_or_id() {
        let docs: Arc<RwLock<Vec<MemoryDocument>>> = Arc::new(RwLock::new(Vec::new()));
        let memory = QmdMemory::new(docs);

        let doc = MemoryDocument {
            id: Some("doc-1".to_string()),
            path: "memory/doc-1".to_string(),
            content: "content".to_string(),
            metadata: serde_json::json!({}),
            embedding: vec![],
        };

        memory.add(doc.clone()).await.unwrap();

        let deleted_by_path = memory.delete("memory/doc-1").await.unwrap();
        assert!(deleted_by_path.is_some());
        assert_eq!(memory.count().await.unwrap(), 0);

        memory.add(doc).await.unwrap();
        let deleted_by_id = memory.delete("doc-1").await.unwrap();
        assert!(deleted_by_id.is_some());
        assert_eq!(memory.count().await.unwrap(), 0);
    }

    #[test]
    fn test_cosine_similarity() {
        let left = vec![1.0, 0.0, 0.0];
        let right = vec![1.0, 0.0, 0.0];

        assert_eq!(cosine_similarity(&left, &right), 1.0);
        assert_eq!(cosine_similarity(&left, &[0.0, 1.0, 0.0]), 0.0);
    }
}

#[allow(dead_code)]
fn cosine_similarity(left: &[f32], right: &[f32]) -> f32 {
    if left.is_empty() || right.is_empty() || left.len() != right.len() {
        return 0.0;
    }

    let dot = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a * b)
        .sum::<f32>();
    let left_mag = left.iter().map(|x| x * x).sum::<f32>().sqrt();
    let right_mag = right.iter().map(|x| x * x).sum::<f32>().sqrt();

    if left_mag == 0.0 || right_mag == 0.0 {
        return 0.0;
    }

    dot / (left_mag * right_mag)
}
