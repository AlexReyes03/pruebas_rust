use axum::{extract::State, Json};
use crate::error::AppError;
use crate::modules::models::bank::*;
use crate::state::AppState;

pub async fn create_transfer(
    State(_state): State<AppState>,
    Json(_payload): Json<BankTransferRequest>,
) -> Result<Json<BankTransferResponse>, AppError> {
    Err(AppError::NotImplemented("create_transfer endpoint".to_string()))
}

pub async fn list_transfers(
    State(_state): State<AppState>,
) -> Result<Json<TransferListResponse>, AppError> {
    Err(AppError::NotImplemented("list_transfers endpoint".to_string()))
}