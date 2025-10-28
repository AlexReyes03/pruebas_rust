use axum::{extract::{Path, State}, Json};
use crate::error::AppError;
use crate::modules::models::reputation::ReputationResponse;
use crate::state::AppState;

pub async fn get_reputation(
    State(state): State<AppState>,
    Path(pubkey): Path<String>,
) -> Result<Json<ReputationResponse>, AppError> {
    let wallet = state
        .wallet_service
        .wallet_repo
        .find_by_pubkey(&pubkey)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    let wallet_id = wallet.as_ref().map(|w| w.id.as_str());

    let response = state
        .reputation_service
        .get_reputation_response(&pubkey, wallet_id)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(Json(response))
}