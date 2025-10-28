use anyhow::Result;
use sqlx::SqlitePool;
use crate::modules::models::transaction::Transaction;

#[derive(Clone)]
pub struct TransactionRepository {
    pool: SqlitePool,
}

impl TransactionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, tx: &Transaction) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO transactions (id, wallet_id, tx_hash, tx_type, from_address, to_address, amount, asset, status, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            tx.id,
            tx.wallet_id,
            tx.tx_hash,
            tx.tx_type,
            tx.from_address,
            tx.to_address,
            tx.amount,
            tx.asset,
            tx.status,
            tx.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_wallet_id(&self, wallet_id: &str, limit: i64) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as!(
            Transaction,
            "SELECT id, wallet_id, tx_hash, tx_type, from_address, to_address, amount, asset, status, created_at 
             FROM transactions 
             WHERE wallet_id = ? 
             ORDER BY created_at DESC 
             LIMIT ?",
            wallet_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(transactions)
    }

    pub async fn count_by_wallet_id(&self, wallet_id: &str) -> Result<i64> {
        let result = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM transactions WHERE wallet_id = ?",
            wallet_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result)
    }
}