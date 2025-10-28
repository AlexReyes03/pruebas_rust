use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reputation {
    pub public_key: String,
    pub trust_score: u8,
    pub level: String,
    pub tx_count: u32,
    pub total_volume: f64,
    pub last_calculated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationResponse {
    pub public_key: String,
    pub trust_score: u8,
    pub level: String,
    pub details: ReputationDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationDetails {
    pub tx_count: u32,
    pub total_volume: f64,
    pub account_age_days: i64,
    pub last_activity: Option<DateTime<Utc>>,
}