use axum::{extract::{Path, State}, Json};
use crate::error::AppError;
use crate::modules::models::reputation::ReputationResponse;
use crate::state::AppState;

pub async fn get_reputation(
    State(_state): State<AppState>,
    Path(_pubkey): Path<String>,
) -> Result<Json<ReputationResponse>, AppError> {
    Err(AppError::NotImplemented("get_reputation endpoint".to_string()))
}