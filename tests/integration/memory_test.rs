//! Memory Module Tests - aligned with the current QmdMemory implementation.

#[cfg(test)]
mod memory_manager_tests {
    use cortex::memory::manager::{MemoryAction, MemoryManager, MemoryMetrics};
    use cortex::memory::qmd_memory::{MemoryDocument, QmdMemory};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    fn empty_memory() -> Arc<QmdMemory> {
        Arc::new(QmdMemory::new(Arc::new(RwLock::new(Vec::new()))))
    }

    #[tokio::test]
    async fn test_memory_manager_creation() {
        let manager = MemoryManager::new(empty_memory());

        assert_eq!(manager.max_documents, 1000);
        assert_eq!(manager.max_age_hours, 24.0 * 7.0);
    }

    #[tokio::test]
    async fn test_get_metrics_empty() {
        let manager = MemoryManager::new(empty_memory());

        let metrics = manager.get_metrics().await.unwrap();
        assert_eq!(metrics.total_documents, 0);
    }

    #[tokio::test]
    async fn test_analyze_and_recommend_empty() {
        let manager = MemoryManager::new(empty_memory());

        let actions = manager.analyze_and_recommend().await.unwrap();
        assert!(actions.is_empty());
    }

    #[tokio::test]
    async fn test_analyze_recommends_consolidation() {
        let docs = Arc::new(RwLock::new(Vec::new()));
        {
            let mut docs_guard = docs.write().await;
            for i in 0..1500 {
                docs_guard.push(MemoryDocument {
                    id: Some(format!("doc_{i}")),
                    path: format!("memory/doc_{i}"),
                    content: format!("content {i}"),
                    metadata: serde_json::json!({}),
                    embedding: vec![],
                });
            }
        }

        let manager = MemoryManager::new(Arc::new(QmdMemory::new(docs)));
        let actions = manager.analyze_and_recommend().await.unwrap();

        assert!(!actions.is_empty());
        assert!(matches!(actions[0], MemoryAction::Consolidate { .. }));
    }
}

#[cfg(test)]
mod memory_metrics_tests {
    use cortex::memory::manager::MemoryMetrics;

    #[test]
    fn test_memory_metrics_creation() {
        let metrics = MemoryMetrics {
            total_documents: 100,
            total_size_bytes: 50000,
            oldest_document_age_hours: 48.0,
            newest_document_age_hours: 2.0,
            average_relevance: 0.75,
        };

        assert_eq!(metrics.total_documents, 100);
        assert_eq!(metrics.total_size_bytes, 50000);
        assert!(metrics.average_relevance > 0.0 && metrics.average_relevance <= 1.0);
    }
}

#[cfg(test)]
mod memory_action_tests {
    use cortex::memory::manager::MemoryAction;

    #[test]
    fn test_memory_action_variants() {
        let keep = MemoryAction::Keep;
        assert!(matches!(keep, MemoryAction::Keep));

        let compress = MemoryAction::Compress {
            doc_id: "doc1".to_string(),
            reason: "redundant".to_string(),
        };
        assert!(matches!(compress, MemoryAction::Compress { .. }));

        let delete = MemoryAction::Delete {
            doc_id: "doc2".to_string(),
            reason: "outdated".to_string(),
        };
        assert!(matches!(delete, MemoryAction::Delete { .. }));

        let update = MemoryAction::Update {
            doc_id: "doc3".to_string(),
            new_content: "new content".to_string(),
        };
        assert!(matches!(update, MemoryAction::Update { .. }));

        let consolidate = MemoryAction::Consolidate {
            doc_ids: vec!["doc4".to_string()],
            reason: "merge".to_string(),
        };
        assert!(matches!(consolidate, MemoryAction::Consolidate { .. }));
    }
}
