use axum::{
    extract::{Path, State},
    Json,
};
use crate::error::AppError;
use crate::modules::models::wallet::*;
use crate::state::AppState;

pub async fn generate_wallet(
    State(_state): State<AppState>,
    Json(_payload): Json<GenerateWalletRequest>,
) -> Result<Json<GenerateWalletResponse>, AppError> {
    Err(AppError::NotImplemented("generate_wallet endpoint".to_string()))
}

pub async fn fund_wallet(
    State(_state): State<AppState>,
    Json(_payload): Json<FundWalletRequest>,
) -> Result<Json<FundWalletResponse>, AppError> {
    Err(AppError::NotImplemented("fund_wallet endpoint".to_string()))
}

pub async fn get_balance(
    State(_state): State<AppState>,
    Path(_pubkey): Path<String>,
) -> Result<Json<BalanceResponse>, AppError> {
    Err(AppError::NotImplemented("get_balance endpoint".to_string()))
}

pub async fn send_transaction(
    State(_state): State<AppState>,
    Path(_pubkey): Path<String>,
    Json(_payload): Json<SendTransactionRequest>,
) -> Result<Json<SendTransactionResponse>, AppError> {
    Err(AppError::NotImplemented("send_transaction endpoint".to_string()))
}

pub async fn aa_relay_transaction(
    State(_state): State<AppState>,
    Json(_payload): Json<RelayTransactionRequest>,
) -> Result<Json<RelayTransactionResponse>, AppError> {
    Err(AppError::NotImplemented("aa_relay_transaction endpoint".to_string()))
}