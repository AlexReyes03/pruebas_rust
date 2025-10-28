use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::Value;

pub struct StellarClient {
    client: Client,
    horizon_url: String,
}

impl StellarClient {
    pub fn new(horizon_url: String) -> Self {
        Self {
            client: Client::new(),
            horizon_url,
        }
    }

    pub async fn get_account(&self, public_key: &str) -> Result<Value> {
        let url = format!("{}/accounts/{}", self.horizon_url, public_key);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch account")?;

        if !response.status().is_success() {
            let status = response.status();
            return Err(anyhow::anyhow!("Horizon API error: {}", status));
        }

        let json = response.json().await.context("Failed to parse response")?;
        Ok(json)
    }

    pub async fn get_transactions(&self, public_key: &str, limit: u32) -> Result<Vec<Value>> {
        let url = format!(
            "{}/accounts/{}/transactions?limit={}&order=desc",
            self.horizon_url, public_key, limit
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch transactions")?;

        if !response.status().is_success() {
            return Ok(vec![]);
        }

        let json: Value = response.json().await.context("Failed to parse response")?;
        
        let records = json["_embedded"]["records"]
            .as_array()
            .unwrap_or(&vec![])
            .clone();

        Ok(records)
    }

    pub async fn get_sequence_number(&self, public_key: &str) -> Result<String> {
        let account = self.get_account(public_key).await?;
        
        let sequence = account["sequence"]
            .as_str()
            .context("No sequence number in response")?
            .to_string();

        Ok(sequence)
    }

    pub async fn submit_transaction(&self, tx_envelope_xdr: &str) -> Result<String> {
        let url = format!("{}/transactions", self.horizon_url);
        
        let params = [("tx", tx_envelope_xdr)];
        
        let response = self.client
            .post(&url)
            .form(&params)
            .send()
            .await
            .context("Failed to submit transaction")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Transaction submission failed: {} - {}",
                status,
                text
            ));
        }

        let json: Value = response.json().await.context("Failed to parse response")?;
        
        let tx_hash = json["hash"]
            .as_str()
            .context("No hash in response")?
            .to_string();

        Ok(tx_hash)
    }
}

pub fn extract_balances(account_json: &Value) -> Vec<(String, String)> {
    let balances = account_json["balances"]
        .as_array()
        .unwrap_or(&vec![]);

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

    result
}