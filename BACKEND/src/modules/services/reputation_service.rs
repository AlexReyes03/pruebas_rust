use anyhow::Result;
use std::sync::Arc;

use crate::modules::repositories::transaction_repo::TransactionRepository;
use crate::modules::services::stellar_service::StellarService;
use crate::modules::models::reputation::Reputation;

#[derive(Clone)]
pub struct ReputationService {
    transaction_repo: Arc<TransactionRepository>,
    stellar_service: Arc<StellarService>,
    threshold: u8,
}

impl ReputationService {
    pub fn new(
        transaction_repo: Arc<TransactionRepository>,
        stellar_service: Arc<StellarService>,
        threshold: u8,
    ) -> Self {
        Self {
            transaction_repo,
            stellar_service,
            threshold,
        }
    }

    pub fn get_threshold(&self) -> u8 {
        self.threshold
    }

    pub async fn calculate_reputation(&self, _public_key: &str) -> Result<Reputation> {
        // TODO: Implement reputation calculation
        Ok(Reputation {
            public_key: _public_key.to_string(),
            trust_score: 65,
            level: "Verified L1".to_string(),
            tx_count: 0,
            total_volume: 0.0,
            last_calculated: chrono::Utc::now(),
        })
    }

    pub async fn check_threshold(&self, public_key: &str) -> Result<bool> {
        let reputation = self.calculate_reputation(public_key).await?;
        Ok(reputation.trust_score >= self.threshold)
    }
}