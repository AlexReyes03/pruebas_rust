use anyhow::Result;
use sqlx::SqlitePool;
use crate::modules::models::bank::BankTransfer;

#[derive(Clone)]
pub struct BankTransferRepository {
    pool: SqlitePool,
}

impl BankTransferRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, transfer: &BankTransfer) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO bank_transfers 
            (id, wallet_id, public_key, amount_fiat, currency, bank_account_masked, status, rejection_reason, reputation_score, created_at, completed_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            transfer.id,
            transfer.wallet_id,
            transfer.public_key,
            transfer.amount_fiat,
            transfer.currency,
            transfer.bank_account_masked,
            transfer.status,
            transfer.rejection_reason,
            transfer.reputation_score,
            transfer.created_at,
            transfer.completed_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<BankTransfer>> {
        let transfers = sqlx::query_as!(
            BankTransfer,
            r#"
            SELECT id, wallet_id, public_key, amount_fiat, currency, bank_account_masked, 
                   status, rejection_reason, reputation_score, created_at, completed_at
            FROM bank_transfers 
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(transfers)
    }

    pub async fn find_by_wallet(&self, wallet_id: &str) -> Result<Vec<BankTransfer>> {
        let transfers = sqlx::query_as!(
            BankTransfer,
            r#"
            SELECT id, wallet_id, public_key, amount_fiat, currency, bank_account_masked, 
                   status, rejection_reason, reputation_score, created_at, completed_at
            FROM bank_transfers 
            WHERE wallet_id = ?
            ORDER BY created_at DESC
            "#,
            wallet_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(transfers)
    }
}