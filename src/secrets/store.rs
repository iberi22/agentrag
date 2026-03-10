use async_trait::async_trait;

use crate::secrets::SecretResult;

#[async_trait]
pub trait SecretStore: Send + Sync {
    async fn get(&self, key: &str) -> SecretResult<String>;
    async fn set(&self, key: &str, value: &str) -> SecretResult<()>;
    async fn delete(&self, key: &str) -> SecretResult<()>;
}
