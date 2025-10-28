use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;

#[derive(Clone)]
pub struct AaService {
    signers: Arc<RwLock<HashMap<String, String>>>,
}

impl AaService {
    pub fn new() -> Self {
        Self {
            signers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_signer(&self, pubkey: &str, secret_seed: &str) -> Result<()> {
        let mut signers = self.signers.write().await;
        signers.insert(pubkey.to_string(), secret_seed.to_string());
        Ok(())
    }

    pub async fn get_signer(&self, pubkey: &str) -> Option<String> {
        let signers = self.signers.read().await;
        signers.get(pubkey).cloned()
    }

    pub async fn has_signer(&self, pubkey: &str) -> bool {
        let signers = self.signers.read().await;
        signers.contains_key(pubkey)
    }

    pub async fn remove_signer(&self, pubkey: &str) -> Result<()> {
        let mut signers = self.signers.write().await;
        signers.remove(pubkey);
        Ok(())
    }

    pub async fn relay_transaction(&self, _pubkey: &str, _tx_xdr: &str) -> Result<String> {
        // TODO: Implement transaction signing and submission
        Ok("mock_tx_hash".to_string())
    }
}

impl Default for AaService {
    fn default() -> Self {
        Self::new()
    }
}