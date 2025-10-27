use sqlx::SqlitePool;
use anyhow::Result;
use crate::models::wallet::Wallet;

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
            INSERT INTO wallets (id, public_key, created_at, is_aa_wallet)
            VALUES (?, ?, ?, ?)
            "#,
            wallet.id,
            wallet.public_key,
            wallet.created_at,
            wallet.is_aa_wallet
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_pubkey(&self, pubkey: &str) -> Result<Option<Wallet>> {
        let wallet = sqlx::query_as!(
            Wallet,
            "SELECT * FROM wallets WHERE public_key = ?",
            pubkey
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(wallet)
    }
}