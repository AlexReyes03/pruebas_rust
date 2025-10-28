use anyhow::{Context, Result};
use std::sync::Arc;
use chrono::Utc;

use crate::modules::models::bank::{BankTransfer, BankTransferDetails};
use crate::modules::repositories::{
    bank_transfer_repo::BankTransferRepository,
    wallet_repo::WalletRepository,
};
use crate::modules::services::reputation_service::ReputationService;
use crate::error::AppError;

#[derive(Clone)]
pub struct BankService {
    bank_transfer_repo: Arc<BankTransferRepository>,
    wallet_repo: Arc<WalletRepository>,
    reputation_service: Arc<ReputationService>,
}

impl BankService {
    pub fn new(
        bank_transfer_repo: Arc<BankTransferRepository>,
        wallet_repo: Arc<WalletRepository>,
        reputation_service: Arc<ReputationService>,
    ) -> Self {
        Self {
            bank_transfer_repo,
            wallet_repo,
            reputation_service,
        }
    }

    pub async fn create_transfer(
        &self,
        public_key: &str,
        amount: f64,
        currency: &str,
        bank_account: &str,
    ) -> Result<(String, String, Option<BankTransferDetails>), AppError> {
        let wallet = self.wallet_repo.find_by_pubkey(public_key).await
            .map_err(|e| AppError::InternalError(e.to_string()))?
            .ok_or_else(|| AppError::WalletNotFound(public_key.to_string()))?;

        let reputation = self.reputation_service
            .calculate_reputation(public_key, Some(&wallet.id))
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        let threshold = self.reputation_service.get_threshold();
        
        if reputation.trust_score < threshold {
            let transfer = BankTransfer {
                id: uuid::Uuid::new_v4().to_string(),
                wallet_id: wallet.id.clone(),
                public_key: public_key.to_string(),
                amount_fiat: amount,
                currency: currency.to_string(),
                bank_account_masked: Self::mask_account(bank_account),
                status: "rejected".to_string(),
                rejection_reason: Some(format!(
                    "Reputation score too low: {} (required: {})",
                    reputation.trust_score, threshold
                )),
                reputation_score: Some(reputation.trust_score as i64),
                created_at: Utc::now(),
                completed_at: None,
            };

            self.bank_transfer_repo.create(&transfer).await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            tracing::warn!(
                "Bank transfer rejected for {}: reputation {} < threshold {}",
                public_key,
                reputation.trust_score,
                threshold
            );

            return Err(AppError::ReputationTooLow {
                current: reputation.trust_score,
                required: threshold,
            });
        }

        let transfer_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let transfer = BankTransfer {
            id: transfer_id.clone(),
            wallet_id: wallet.id.clone(),
            public_key: public_key.to_string(),
            amount_fiat: amount,
            currency: currency.to_string(),
            bank_account_masked: Self::mask_account(bank_account),
            status: "completed".to_string(),
            rejection_reason: None,
            reputation_score: Some(reputation.trust_score as i64),
            created_at: now,
            completed_at: Some(now),
        };

        self.bank_transfer_repo.create(&transfer).await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tracing::info!(
            "Bank transfer completed for {}: {} {} (reputation: {})",
            public_key,
            amount,
            currency,
            reputation.trust_score
        );

        let details = BankTransferDetails {
            amount,
            currency: currency.to_string(),
            bank_account_masked: Self::mask_account(bank_account),
            reputation_score: reputation.trust_score,
            created_at: now,
        };

        Ok((
            transfer_id,
            "completed".to_string(),
            Some(details),
        ))
    }

    pub async fn list_all_transfers(&self) -> Result<Vec<BankTransfer>> {
        self.bank_transfer_repo.find_all().await
    }

    pub async fn list_transfers_by_pubkey(&self, public_key: &str) -> Result<Vec<BankTransfer>> {
        self.bank_transfer_repo.find_by_public_key(public_key).await
    }

    fn mask_account(account: &str) -> String {
        if account.len() <= 4 {
            return "*".repeat(account.len());
        }
        
        let last_four = &account[account.len() - 4..];
        format!("****{}", last_four)
    }
}