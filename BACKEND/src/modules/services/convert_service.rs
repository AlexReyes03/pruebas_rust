use anyhow::Result;

#[derive(Clone)]
pub struct ConvertService {
    coingecko_url: String,
}

impl ConvertService {
    pub fn new(coingecko_url: String) -> Self {
        Self { coingecko_url }
    }

    pub async fn convert_to_usdc(&self, _from_token: &str, _amount: &str) -> Result<(String, String, String)> {
        // TODO: Implement conversion logic
        Ok((
            "100.00".to_string(),
            "100.00".to_string(),
            "CoinGecko".to_string(),
        ))
    }

    pub async fn get_exchange_rate(&self, _from: &str, _to: &str) -> Result<f64> {
        // TODO: Implement rate fetching from CoinGecko
        Ok(1.0)
    }
}