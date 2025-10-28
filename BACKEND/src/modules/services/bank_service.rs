use anyhow::Result;
use std::sync::Arc;

use crate::modules::repositories::bank_transfer_repo::BankTransferRepository;
use crate::modules::services::reputation_service::ReputationService;

#[derive(Clone)]
pub struct BankService {
    bank_transfer_repo: Arc<BankTransferRepository>,
    reputation_service: Arc<ReputationService>,
}

impl BankService {
    pub fn new(
        bank_transfer_repo: Arc<BankTransferRepository>,
        reputation_service: Arc<ReputationService>,
    ) -> Self {
        Self {
            bank_transfer_repo,
            reputation_service,
        }
    }

    pub async fn create_transfer(
        &self,
        _public_key: &str,
        _amount: f64,
        _currency: &str,
        _bank_account: &str,
    ) -> Result<String> {
        // TODO: Implement transfer creation with reputation check
        Ok("transfer_id".to_string())
    }

    pub async fn list_all_transfers(&self) -> Result<Vec<crate::modules::models::bank::BankTransfer>> {
        self.bank_transfer_repo.find_all().await
    }
}