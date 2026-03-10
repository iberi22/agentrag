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

pub mod daemon;
pub mod local;
pub mod openbao;
pub mod store;
#[cfg(test)]
mod tests;
