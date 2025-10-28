use sqlx::SqlitePool;
use std::sync::Arc;

use crate::config::Config;
use crate::modules::services::{
    aa_service::AaService,
    bank_service::BankService,
    convert_service::ConvertService,
    reputation_service::ReputationService,
    stellar_service::StellarService,
    wallet_service::WalletService,
};
use crate::modules::repositories::{
    bank_transfer_repo::BankTransferRepository,
    transaction_repo::TransactionRepository,
    wallet_repo::WalletRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db_pool: SqlitePool,
    
    // Services
    pub wallet_service: Arc<WalletService>,
    pub aa_service: Arc<AaService>,
    pub stellar_service: Arc<StellarService>,
    pub reputation_service: Arc<ReputationService>,
    pub convert_service: Arc<ConvertService>,
    pub bank_service: Arc<BankService>,
}

impl AppState {
    pub async fn new(config: Config, db_pool: SqlitePool) -> Self {
        let config_arc = Arc::new(config.clone());
        
        // Initialize repositories
        let wallet_repo = Arc::new(WalletRepository::new(db_pool.clone()));
        let transaction_repo = Arc::new(TransactionRepository::new(db_pool.clone()));
        let bank_transfer_repo = Arc::new(BankTransferRepository::new(db_pool.clone()));

        // Initialize services
        let aa_service = Arc::new(AaService::new());
        let stellar_service = Arc::new(StellarService::new(
            config.stellar.horizon_url.clone(),
            config.stellar.friendbot_url.clone(),
        ));
        
        let wallet_service = Arc::new(WalletService::new(
            wallet_repo.clone(),
            transaction_repo.clone(),
            aa_service.clone(),
            stellar_service.clone(),
        ));

        let reputation_service = Arc::new(ReputationService::new(
            transaction_repo.clone(),
            stellar_service.clone(),
            config.reputation.threshold,
        ));

        let convert_service = Arc::new(ConvertService::new(
            config.external_apis.coingecko_api_url.clone(),
        ));

        let bank_service = Arc::new(BankService::new(
            bank_transfer_repo.clone(),
            reputation_service.clone(),
        ));

        Self {
            config: config_arc,
            db_pool,
            wallet_service,
            aa_service,
            stellar_service,
            reputation_service,
            convert_service,
            bank_service,
        }
    }
}