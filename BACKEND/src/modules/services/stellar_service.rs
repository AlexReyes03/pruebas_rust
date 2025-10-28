use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct StellarService {
    horizon_url: String,
    friendbot_url: String,
    client: Client,
}

impl StellarService {
    pub fn new(horizon_url: String, friendbot_url: String) -> Self {
        Self {
            horizon_url,
            friendbot_url,
            client: Client::new(),
        }
    }

    pub async fn fund_account(&self, public_key: &str) -> Result<String> {
        let url = format!("{}?addr={}", self.friendbot_url, public_key);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to call Friendbot")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Friendbot failed with status {}: {}",
                status,
                text
            ));
        }

        let json: Value = response.json().await.context("Failed to parse Friendbot response")?;
        
        let tx_hash = json["hash"]
            .as_str()
            .context("No hash in Friendbot response")?
            .to_string();

        tracing::info!("Account {} funded successfully: {}", public_key, tx_hash);
        Ok(tx_hash)
    }

    pub async fn get_account_balance(&self, public_key: &str) -> Result<Vec<(String, String)>> {
        let url = format!("{}/accounts/{}", self.horizon_url, public_key);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch account from Horizon")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Horizon API failed with status {}: {}",
                status,
                text
            ));
        }

        let json: Value = response.json().await.context("Failed to parse Horizon response")?;
        
        let balances = json["balances"]
            .as_array()
            .context("No balances in response")?;

        let mut result = Vec::new();
        for balance in balances {
            let asset_code = if balance["asset_type"].as_str() == Some("native") {
                "XLM".to_string()
            } else {
                balance["asset_code"].as_str().unwrap_or("UNKNOWN").to_string()
            };
            
            let amount = balance["balance"].as_str().unwrap_or("0").to_string();
            
            result.push((asset_code, amount));
        }

        Ok(result)
    }

    pub async fn submit_transaction(&self, tx_xdr: &str) -> Result<String> {
        let url = format!("{}/transactions", self.horizon_url);
        
        let params = [("tx", tx_xdr)];
        
        let response = self.client
            .post(&url)
            .form(&params)
            .send()
            .await
            .context("Failed to submit transaction to Horizon")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Transaction submission failed with status {}: {}",
                status,
                text
            ));
        }

        let json: Value = response.json().await.context("Failed to parse submit response")?;
        
        let tx_hash = json["hash"]
            .as_str()
            .context("No hash in submit response")?
            .to_string();

        tracing::info!("Transaction submitted successfully: {}", tx_hash);
        Ok(tx_hash)
    }

    pub async fn get_account_transactions(&self, public_key: &str, limit: u32) -> Result<Vec<String>> {
        let url = format!(
            "{}/accounts/{}/transactions?limit={}&order=desc",
            self.horizon_url, public_key, limit
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch transactions from Horizon")?;

        if !response.status().is_success() {
            return Ok(vec![]);
        }

        let json: Value = response.json().await.context("Failed to parse transactions response")?;
        
        let records = json["_embedded"]["records"]
            .as_array()
            .context("No records in response")?;

        let mut result = Vec::new();
        for record in records {
            if let Some(hash) = record["hash"].as_str() {
                result.push(hash.to_string());
            }
        }

        Ok(result)
    }

    pub async fn check_account_exists(&self, public_key: &str) -> Result<bool> {
        let url = format!("{}/accounts/{}", self.horizon_url, public_key);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to check account existence")?;

        Ok(response.status().is_success())
    }
}