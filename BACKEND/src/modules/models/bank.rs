use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BankTransfer {
    pub id: String,
    pub wallet_id: String,
    pub public_key: String,
    pub amount_fiat: f64,
    pub currency: String,
    pub bank_account_masked: String,
    pub status: String,
    pub rejection_reason: Option<String>,
    pub reputation_score: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferRequest {
    pub public_key: String,
    pub amount_fiat: f64,
    pub currency: String,
    pub bank_account: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferResponse {
    pub id: String,
    pub status: String,
    pub message: String,
    pub transfer_details: Option<BankTransferDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferDetails {
    pub amount: f64,
    pub currency: String,
    pub bank_account_masked: String,
    pub reputation_score: u8,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferListResponse {
    pub transfers: Vec<BankTransfer>,
    pub total: usize,
}