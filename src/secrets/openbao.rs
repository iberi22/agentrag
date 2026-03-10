use async_trait::async_trait;

use crate::secrets::{store::SecretStore, SecretError, SecretResult};

pub struct OpenBaoSecretStore {
    address: String,
    token: String,
}

impl OpenBaoSecretStore {
    pub fn new(address: &str, token: &str) -> Self {
        Self {
            address: address.to_string(),
            token: token.to_string(),
        }
    }
}

#[async_trait]
impl SecretStore for OpenBaoSecretStore {
    async fn get(&self, _key: &str) -> SecretResult<String> {
        Err(SecretError::ProviderError(format!(
            "OpenBao provider not yet fully implemented for {}",
            self.address
        )))
    }

    async fn set(&self, _key: &str, _value: &str) -> SecretResult<()> {
        let _ = &self.token;
        Err(SecretError::ProviderError(format!(
            "OpenBao provider not yet fully implemented for {}",
            self.address
        )))
    }

    async fn delete(&self, _key: &str) -> SecretResult<()> {
        Err(SecretError::ProviderError(format!(
            "OpenBao provider not yet fully implemented for {}",
            self.address
        )))
    }
}
