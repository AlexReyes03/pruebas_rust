use anyhow::{Context, Result};
use crate::{
    models::wallet::*,
    repositories::wallet_repo::WalletRepository,
    services::aa_service::AaService,
};

pub struct WalletService {
    wallet_repo: WalletRepository,
    aa_service: AaService,
}

impl WalletService {
    pub fn new(wallet_repo: WalletRepository, aa_service: AaService) -> Self {
        Self { wallet_repo, aa_service }
    }

    pub async fn generate_wallet(&self, req: GenerateWalletRequest) -> Result<GenerateWalletResponse> {
        // Generate keypair using ed25519-dalek or stellar-sdk
        let keypair = self.generate_keypair()?;
        
        let wallet = Wallet {
            id: uuid::Uuid::new_v4().to_string(),
            public_key: keypair.public_key.clone(),
            created_at: chrono::Utc::now(),
            is_aa_wallet: req.aa_mode,
        };

        self.wallet_repo.create(&wallet).await
            .context("Failed to save wallet to database")?;

        if req.aa_mode {
            self.aa_service.register_signer(&keypair.public_key, &keypair.secret_key).await?;
        }

        Ok(GenerateWalletResponse {
            public_key: keypair.public_key,
            secret_key: if req.reveal_secret.unwrap_or(false) {
                Some(keypair.secret_key)
            } else {
                None
            },
            aa_enabled: req.aa_mode,
        })
    }

    fn generate_keypair(&self) -> Result<KeyPair> {
        // Implementation using stellar-sdk or ed25519-dalek
        todo!("Generate Stellar keypair")
    }
}