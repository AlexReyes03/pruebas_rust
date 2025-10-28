use axum::{extract::State, Json};
use crate::error::AppError;
use crate::modules::models::bank::*;
use crate::state::AppState;

pub async fn create_transfer(
    State(state): State<AppState>,
    Json(payload): Json<BankTransferRequest>,
) -> Result<Json<BankTransferResponse>, AppError> {
    let (transfer_id, status, details) = state
        .bank_service
        .create_transfer(
            &payload.public_key,
            payload.amount_fiat,
            &payload.currency,
            &payload.bank_account,
        )
        .await?;

    let message = if status == "completed" {
        format!(
            "Bank transfer of {} {} successfully processed",
            payload.amount_fiat, payload.currency
        )
    } else {
        "Bank transfer rejected due to low reputation".to_string()
    };

    Ok(Json(BankTransferResponse {
        id: transfer_id,
        status,
        message,
        transfer_details: details,
    }))
}

pub async fn list_transfers(
    State(state): State<AppState>,
) -> Result<Json<TransferListResponse>, AppError> {
    let transfers = state
        .bank_service
        .list_all_transfers()
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    let total = transfers.len();

    Ok(Json(TransferListResponse { transfers, total }))
}