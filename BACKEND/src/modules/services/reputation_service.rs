use anyhow::{Context, Result};
use std::sync::Arc;
use chrono::Utc;

use crate::modules::models::reputation::{Reputation, ReputationDetails, ReputationResponse};
use crate::modules::repositories::transaction_repo::TransactionRepository;
use crate::modules::services::stellar_service::StellarService;

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

    pub async fn calculate_reputation(&self, public_key: &str, wallet_id: Option<&str>) -> Result<Reputation> {
        let account_age_days = self.get_account_age(public_key).await?;
        
        let (tx_count, total_volume) = if let Some(wid) = wallet_id {
            let count = self.transaction_repo.count_by_wallet_id(wid).await? as u32;
            let volume = self.transaction_repo.sum_volume_by_wallet_id(wid).await?;
            (count, volume)
        } else {
            (0, 0.0)
        };

        let trust_score = self.calculate_trust_score(tx_count, total_volume, account_age_days);
        let level = self.get_trust_level(trust_score);

        tracing::debug!(
            "Reputation calculated for {}: score={}, tx_count={}, volume={}, age_days={}",
            public_key,
            trust_score,
            tx_count,
            total_volume,
            account_age_days
        );

        Ok(Reputation {
            public_key: public_key.to_string(),
            trust_score,
            level,
            tx_count,
            total_volume,
            last_calculated: Utc::now(),
        })
    }

    pub async fn check_threshold(&self, public_key: &str, wallet_id: Option<&str>) -> Result<bool> {
        let reputation = self.calculate_reputation(public_key, wallet_id).await?;
        Ok(reputation.trust_score >= self.threshold)
    }

    pub async fn get_reputation_response(
        &self,
        public_key: &str,
        wallet_id: Option<&str>,
    ) -> Result<ReputationResponse> {
        let reputation = self.calculate_reputation(public_key, wallet_id).await?;
        let account_age_days = self.get_account_age(public_key).await?;
        
        let last_activity = if let Some(wid) = wallet_id {
            let recent = self.transaction_repo.find_by_wallet_id(wid, 1).await?;
            recent.first().map(|tx| tx.created_at)
        } else {
            None
        };

        Ok(ReputationResponse {
            public_key: reputation.public_key.clone(),
            trust_score: reputation.trust_score,
            level: reputation.level.clone(),
            details: ReputationDetails {
                tx_count: reputation.tx_count,
                total_volume: reputation.total_volume,
                account_age_days,
                last_activity,
            },
        })
    }

    fn calculate_trust_score(&self, tx_count: u32, total_volume: f64, age_days: i64) -> u8 {
        let base_score: f64 = 10.0;
        
        let tx_bonus = (tx_count as f64 * 2.0).min(40.0);
        
        let volume_bonus = if total_volume > 0.0 {
            (total_volume.log10() * 10.0).min(30.0)
        } else {
            0.0
        };
        
        let age_bonus = (age_days as f64 / 10.0).min(20.0);
        
        let total_score = base_score + tx_bonus + volume_bonus + age_bonus;
        
        total_score.min(100.0) as u8
    }

    fn get_trust_level(&self, score: u8) -> String {
        match score {
            0..=30 => "Unverified".to_string(),
            31..=60 => "Verified L1".to_string(),
            61..=80 => "Verified L2".to_string(),
            81..=100 => "Trusted".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    async fn get_account_age(&self, public_key: &str) -> Result<i64> {
        let exists = self.stellar_service.check_account_exists(public_key).await?;
        
        if !exists {
            return Ok(0);
        }
        
        Ok(30)
    }
}