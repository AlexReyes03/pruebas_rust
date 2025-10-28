use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminStatsResponse {
    pub total_wallets: i64,
    pub total_transactions: i64,
    pub total_bank_transfers: i64,
    pub aa_wallets_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthDetailsResponse {
    pub status: String,
    pub version: String,
    pub database_connected: bool,
    pub stellar_horizon_url: String,
    pub reputation_threshold: u8,
}

pub async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<AdminStatsResponse>, AppError> {
    let total_wallets = sqlx::query_scalar!(
        "SELECT COUNT(*) as count FROM wallets"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let total_transactions = sqlx::query_scalar!(
        "SELECT COUNT(*) as count FROM transactions"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let total_bank_transfers = sqlx::query_scalar!(
        "SELECT COUNT(*) as count FROM bank_transfers"
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let aa_signers = state.aa_service.list_signers().await;

    Ok(Json(AdminStatsResponse {
        total_wallets,
        total_transactions,
        total_bank_transfers,
        aa_wallets_count: aa_signers.len(),
    }))
}

pub async fn health_details(
    State(state): State<AppState>,
) -> Result<Json<HealthDetailsResponse>, AppError> {
    let db_connected = sqlx::query("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .is_ok();

    Ok(Json(HealthDetailsResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database_connected: db_connected,
        stellar_horizon_url: state.config.stellar.horizon_url.clone(),
        reputation_threshold: state.config.reputation.threshold,
    }))
}

pub async fn list_aa_accounts(
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, AppError> {
    let signers = state.aa_service.list_signers().await;
    Ok(Json(signers))
}