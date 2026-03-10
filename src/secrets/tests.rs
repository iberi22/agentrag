#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use proptest::prelude::*;

    use crate::secrets::{daemon::SecretDaemon, local::LocalSecretStore};

    proptest! {
        #[test]
        fn test_daemon_grant_enforcement(
            key in "[a-z0-9]{5,20}",
            value in "[a-zA-Z0-9]{10,100}",
            grant_id_noise in "[a-zA-Z0-9]{10,20}"
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let store = Box::new(LocalSecretStore::new());
                let mut daemon = SecretDaemon::new(store);

                daemon.set_secret(&key, &value).await.unwrap();

                let mut permissions = HashSet::new();
                permissions.insert("read".to_string());
                let grant_id = daemon.issue_grant(&key, permissions, 3600);

                let retrieved = daemon.get_secret(&key, &grant_id).await.unwrap();
                assert_eq!(retrieved, value);

                let noise_res = daemon.get_secret(&key, &grant_id_noise).await;
                assert!(noise_res.is_err());
            });
        }
    }
}
