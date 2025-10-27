use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Wallet {
    pub id: String,
    pub public_key: String,
    pub created_at: DateTime<Utc>,
    pub is_aa_wallet: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateWalletRequest {
    pub aa_mode: bool,
    pub reveal_secret: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GenerateWalletResponse {
    pub public_key: String,
    pub secret_key: Option<String>, // Only if reveal_secret=true in dev
    pub aa_enabled: bool,
}