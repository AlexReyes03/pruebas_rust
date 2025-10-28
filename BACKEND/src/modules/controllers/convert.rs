use axum::{extract::{Query, State}, Json};
use crate::error::AppError;
use crate::modules::models::convert::*;
use crate::state::AppState;

pub async fn convert_to_usdc(
    State(_state): State<AppState>,
    Json(_payload): Json<ConvertRequest>,
) -> Result<Json<ConvertResponse>, AppError> {
    Err(AppError::NotImplemented("convert_to_usdc endpoint".to_string()))
}

pub async fn get_rates(
    State(_state): State<AppState>,
    Query(_params): Query<RatesQuery>,
) -> Result<Json<RatesResponse>, AppError> {
    Err(AppError::NotImplemented("get_rates endpoint".to_string()))
}