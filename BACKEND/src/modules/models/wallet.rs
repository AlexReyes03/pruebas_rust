use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Wallet {
    pub id: String,
    pub public_key: String,
    pub is_aa_wallet: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateWalletRequest {
    pub aa_mode: bool,
    #[serde(default)]
    pub reveal_secret: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateWalletResponse {
    pub id: String,
    pub public_key: String,
    pub secret_key: Option<String>,
    pub aa_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundWalletRequest {
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundWalletResponse {
    pub public_key: String,
    pub status: String,
    pub tx_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub asset_code: String,
    pub balance: String,
    pub asset_issuer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub public_key: String,
    pub balances: Vec<Balance>,
    pub recent_transactions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionRequest {
    pub destination: String,
    pub amount: String,
    pub asset_code: Option<String>,
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionResponse {
    pub tx_hash: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayTransactionRequest {
    pub public_key: String,
    pub tx_xdr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayTransactionResponse {
    pub tx_hash: String,
    pub status: String,
}