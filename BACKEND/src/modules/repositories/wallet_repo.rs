use anyhow::Result;
use sqlx::SqlitePool;
use crate::modules::models::wallet::Wallet;

#[derive(Clone)]
pub struct WalletRepository {
    pool: SqlitePool,
}

impl WalletRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, wallet: &Wallet) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO wallets (id, public_key, is_aa_wallet, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
            wallet.id,
            wallet.public_key,
            wallet.is_aa_wallet,
            wallet.created_at,
            wallet.updated_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_pubkey(&self, pubkey: &str) -> Result<Option<Wallet>> {
        let wallet = sqlx::query_as!(
            Wallet,
            "SELECT id, public_key, is_aa_wallet, created_at, updated_at FROM wallets WHERE public_key = ?",
            pubkey
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(wallet)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Wallet>> {
        let wallet = sqlx::query_as!(
            Wallet,
            "SELECT id, public_key, is_aa_wallet, created_at, updated_at FROM wallets WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(wallet)
    }
}