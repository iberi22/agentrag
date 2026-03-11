use async_trait::async_trait;
use crate::secrets::SecretResult;
use crate::secrets::store::SecretStore;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct LocalSecretStore {
    storage: Mutex<HashMap<String, String>>,
}

impl LocalSecretStore {
    pub fn new() -> Self {
        Self {
            storage: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl SecretStore for LocalSecretStore {
    async fn get(&self, key: &str) -> SecretResult<String> {
        let storage = self.storage.lock().map_err(|e| crate::secrets::SecretError::DatabaseError(e.to_string()))?;
        storage.get(key).cloned().ok_or_else(|| crate::secrets::SecretError::NotFound(key.to_string()))
    }

    async fn set(&self, key: &str, value: &str) -> SecretResult<()> {
        let mut storage = self.storage.lock().map_err(|e| crate::secrets::SecretError::DatabaseError(e.to_string()))?;
        storage.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn delete(&self, key: &str) -> SecretResult<()> {
        let mut storage = self.storage.lock().map_err(|e| crate::secrets::SecretError::DatabaseError(e.to_string()))?;
        storage.remove(key);
        Ok(())
    }
}
