//! Checkpoint Module Tests

#[cfg(test)]
mod checkpoint_tests {
    use cortex::checkpoint::{Checkpoint, CheckpointManager};

    #[test]
    fn test_checkpoint_creation() {
        let checkpoint = Checkpoint::new(
            "task_123".to_string(),
            "checkpoint_1".to_string(),
            serde_json::json!({"state": "test"}),
        );
        
        assert_eq!(checkpoint.task_id, "task_123");
        assert_eq!(checkpoint.name, "checkpoint_1");
    }

    #[test]
    fn test_checkpoint_data() {
        let data = serde_json::json!({
            "step": 1,
            "progress": 0.5,
            "data": "test"
        });
        
        let checkpoint = Checkpoint::new(
            "task".to_string(),
            "cp".to_string(),
            data,
        );
        
        assert_eq!(checkpoint.data["step"], 1);
    }

    #[tokio::test]
    async fn test_checkpoint_manager_save() {
        let manager = CheckpointManager::new();
        
        let checkpoint = Checkpoint::new(
            "task_1".to_string(),
            "save_test".to_string(),
            serde_json::json!({"value": 42}),
        );
        
        manager.save(checkpoint).await.unwrap();
        
        let loaded = manager.load("task_1".to_string(), "save_test".to_string()).await.unwrap();
        assert!(loaded.is_some());
    }

    #[tokio::test]
    async fn test_checkpoint_list() {
        let manager = CheckpointManager::new();
        
        // Save multiple checkpoints
        for i in 0..5 {
            let cp = Checkpoint::new(
                "task_list".to_string(),
                format!("cp_{}", i),
                serde_json::json!({"index": i}),
            );
            manager.save(cp).await.unwrap();
        }
        
        let checkpoints = manager.list("task_list".to_string()).await.unwrap();
        assert_eq!(checkpoints.len(), 5);
    }

    #[tokio::test]
    async fn test_checkpoint_delete() {
        let manager = CheckpointManager::new();
        
        let cp = Checkpoint::new(
            "task_del".to_string(),
            "to_delete".to_string(),
            serde_json::json!({"test": true}),
        );
        
        manager.save(cp).await.unwrap();
        manager.delete("task_del".to_string(), "to_delete".to_string()).await.unwrap();
        
        let loaded = manager.load("task_del".to_string(), "to_delete".to_string()).await.unwrap();
        assert!(loaded.is_none());
    }
}

#[cfg(test)]
mod checkpoint_recovery_tests {
    #[tokio::test]
    #[ignore = "recovery integration pending full task-system wiring"]
    async fn test_full_recovery() {
        // Test full task recovery from checkpoints
        todo!("Implement with actual task system");
    }

    #[tokio::test]
    #[ignore = "recovery integration pending full task-system wiring"]
    async fn test_partial_recovery() {
        // Test recovery from specific checkpoint
        todo!("Implement with actual task system");
    }
}
