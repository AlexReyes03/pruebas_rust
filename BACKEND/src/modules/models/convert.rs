use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertRequest {
    pub from_token: String,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertResponse {
    pub from_token: String,
    pub from_amount: String,
    pub usdc_amount: String,
    pub fiat_amount: String,
    pub fiat_currency: String,
    pub rate_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatesQuery {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatesResponse {
    pub from: String,
    pub to: String,
    pub rate: f64,
    pub source: String,
    pub timestamp: String,
}