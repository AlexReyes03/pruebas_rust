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
        tracing::debug!("Registered AA signer for pubkey: {}", pubkey);
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
        tracing::debug!("Removed AA signer for pubkey: {}", pubkey);
        Ok(())
    }

    pub async fn list_signers(&self) -> Vec<String> {
        let signers = self.signers.read().await;
        signers.keys().cloned().collect()
    }

    pub async fn relay_transaction(&self, pubkey: &str, tx_xdr: &str) -> Result<String> {
        let secret = self.get_signer(pubkey).await
            .ok_or_else(|| anyhow::anyhow!("No signer registered for this account"))?;

        tracing::info!("Relaying transaction for AA account: {}", pubkey);
        tracing::debug!("TX XDR: {}", tx_xdr);
        tracing::debug!("Using signer: {}...", &secret[..10]);

        Ok(format!("aa_relayed_{}", uuid::Uuid::new_v4()))
    }
}

impl Default for AaService {
    fn default() -> Self {
        Self::new()
    }
}