pub mod session;
#[allow(dead_code)]
pub mod state;

use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

pub use session::{SessionCheckpoint, SessionCheckpointInput, MAX_SESSION_CHECKPOINT_BYTES};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub task_id: String,
    pub name: String,
    pub data: serde_json::Value,
}

impl Checkpoint {
    pub fn new(task_id: String, name: String, data: serde_json::Value) -> Self {
        Self {
            task_id,
            name,
            data,
        }
    }
}

#[derive(Default)]
pub struct CheckpointManager {
    checkpoints: RwLock<HashMap<String, Checkpoint>>,
}

impl CheckpointManager {
    pub fn new() -> Self {
        Self::default()
    }

    fn key(task_id: &str, name: &str) -> String {
        format!("{task_id}::{name}")
    }

    pub async fn save(&self, checkpoint: Checkpoint) -> Result<()> {
        self.checkpoints
            .write()
            .await
            .insert(Self::key(&checkpoint.task_id, &checkpoint.name), checkpoint);
        Ok(())
    }

    pub async fn load(&self, task_id: String, name: String) -> Result<Option<Checkpoint>> {
        Ok(self
            .checkpoints
            .read()
            .await
            .get(&Self::key(&task_id, &name))
            .cloned())
    }

    pub async fn list(&self, task_id: String) -> Result<Vec<Checkpoint>> {
        Ok(self
            .checkpoints
            .read()
            .await
            .values()
            .filter(|checkpoint| checkpoint.task_id == task_id)
            .cloned()
            .collect())
    }

    pub async fn delete(&self, task_id: String, name: String) -> Result<()> {
        self.checkpoints
            .write()
            .await
            .remove(&Self::key(&task_id, &name));
        Ok(())
    }
}
