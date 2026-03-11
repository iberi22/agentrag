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

#[allow(dead_code)]
pub mod daemon;
#[allow(dead_code)]
pub mod store;
#[allow(dead_code)]
pub mod local;
#[allow(dead_code)]
pub mod openbao;
#[cfg(test)]
mod tests;
