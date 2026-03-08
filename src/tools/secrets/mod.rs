use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecretError {
    #[error("Secret not found: {0}")]
    NotFound(String),
    #[error("Provider error: {0}")]
    ProviderError(String),
    #[error("Approval denied for operation: {0}")]
    ApprovalDenied(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub type SecretResult<T> = Result<T, SecretError>;

#[async_trait]
pub trait SecretProvider: Send + Sync {
    async fn get(&self, key: &str) -> SecretResult<String>;
    async fn set(&self, key: &str, value: &str) -> SecretResult<()>;
    async fn delete(&self, key: &str) -> SecretResult<()>;
}

pub struct SecretsManager {
    provider: Box<dyn SecretProvider>,
}

impl SecretsManager {
    pub fn new(provider: Box<dyn SecretProvider>) -> Self {
        Self { provider }
    }

    pub async fn get_secret(&self, key: &str) -> SecretResult<String> {
        // Shared logic for "grants" can be added here
        // For now, we delegate to the provider
        self.provider.get(key).await
    }

    pub async fn set_secret(&self, key: &str, value: &str) -> SecretResult<()> {
        // High-stakes approval logic as per SPEC_HUMAN_LAYER_PROTOCOL.md
        // In the full AgentRAG swarm, this would trigger a System 3 veto check
        // or a human approval request.

        log_high_stakes_action("SET_SECRET", key);

        self.provider.set(key, value).await
    }

    pub async fn delete_secret(&self, key: &str) -> SecretResult<()> {
        log_high_stakes_action("DELETE_SECRET", key);
        self.provider.delete(key).await
    }
}

fn log_high_stakes_action(action: &str, key: &str) {
    println!("⚠️  HIGH STAKES ACTION DETECTED");
    println!("Action: {}", action);
    println!("Key: {}", key);
    println!("Reason: Auth/Secrets handling requires explicit oversight.");
}

pub mod local;
pub mod openbao;
