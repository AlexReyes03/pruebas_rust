use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use crate::{
    error::AppError,
    models::wallet::*,
    state::AppState,
};

pub async fn generate_wallet(
    State(state): State<AppState>,
    Json(payload): Json<GenerateWalletRequest>,
) -> Result<Json<GenerateWalletResponse>, AppError> {
    let response = state.wallet_service
        .generate_wallet(payload)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(Json(response))
}