use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct ConvertService {
    coingecko_url: String,
    client: Client,
}

impl ConvertService {
    pub fn new(coingecko_url: String) -> Self {
        Self {
            coingecko_url,
            client: Client::new(),
        }
    }

    pub async fn convert_to_usdc(
        &self,
        from_token: &str,
        amount: &str,
    ) -> Result<(String, String, String)> {
        let amount_f64: f64 = amount.parse().context("Invalid amount format")?;
        
        let from_token_lower = from_token.to_lowercase();
        let coin_id = match from_token_lower.as_str() {
            "xlm" => "stellar",
            "eth" => "ethereum",
            "btc" => "bitcoin",
            _ => from_token_lower.as_str(),
        };

        let xlm_to_usd = self.get_exchange_rate(coin_id, "usd").await?;
        
        let usdc_amount = amount_f64 * xlm_to_usd;
        
        let usd_to_mxn = self.get_exchange_rate("usd", "mxn").await
            .unwrap_or(20.0);
        
        let fiat_amount = usdc_amount * usd_to_mxn;
        
        tracing::info!(
            "Conversion: {} {} = ${:.2} USDC = ${:.2} MXN",
            amount,
            from_token,
            usdc_amount,
            fiat_amount
        );
        
        Ok((
            format!("{:.6}", usdc_amount),
            format!("{:.2}", fiat_amount),
            "CoinGecko".to_string(),
        ))
    }

    pub async fn get_exchange_rate(&self, from: &str, to: &str) -> Result<f64> {
        let url = if from == "usd" {
            format!("{}/simple/price?ids=tether&vs_currencies={}", self.coingecko_url, to)
        } else {
            format!(
                "{}/simple/price?ids={}&vs_currencies={}",
                self.coingecko_url, from, to
            )
        };

        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch exchange rate from CoinGecko")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("CoinGecko API returned error status"));
        }

        let json: Value = response.json().await.context("Failed to parse CoinGecko response")?;
        
        let rate = if from == "usd" {
            json["tether"][to]
                .as_f64()
                .context("Rate not found in response")?
        } else {
            json[from][to]
                .as_f64()
                .context("Rate not found in response")?
        };

        Ok(rate)
    }

    pub fn mock_convert(&self, from_token: &str, amount: &str) -> Result<(String, String)> {
        let amount_f64: f64 = amount.parse().context("Invalid amount format")?;
        
        let mock_rate = match from_token.to_lowercase().as_str() {
            "xlm" => 0.12,
            "eth" => 2000.0,
            "btc" => 40000.0,
            _ => 1.0,
        };
        
        let usdc_amount = amount_f64 * mock_rate;
        let fiat_amount = usdc_amount * 20.0;
        
        Ok((
            format!("{:.6}", usdc_amount),
            format!("{:.2}", fiat_amount),
        ))
    }
}