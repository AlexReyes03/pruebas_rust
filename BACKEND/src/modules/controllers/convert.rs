use axum::{extract::{Query, State}, Json};
use crate::error::AppError;
use crate::modules::models::convert::*;
use crate::state::AppState;

pub async fn convert_to_usdc(
    State(state): State<AppState>,
    Json(payload): Json<ConvertRequest>,
) -> Result<Json<ConvertResponse>, AppError> {
    let (usdc_amount, fiat_amount, rate_source) = state
        .convert_service
        .convert_to_usdc(&payload.from_token, &payload.amount)
        .await
        .map_err(|e| AppError::ExternalApiError(e.to_string()))?;

    Ok(Json(ConvertResponse {
        from_token: payload.from_token.clone(),
        from_amount: payload.amount.clone(),
        usdc_amount,
        fiat_amount,
        fiat_currency: "MXN".to_string(),
        rate_source,
    }))
}

pub async fn get_rates(
    State(state): State<AppState>,
    Query(params): Query<RatesQuery>,
) -> Result<Json<RatesResponse>, AppError> {
    let rate = state
        .convert_service
        .get_exchange_rate(&params.from, &params.to)
        .await
        .map_err(|e| AppError::ExternalApiError(e.to_string()))?;

    Ok(Json(RatesResponse {
        from: params.from,
        to: params.to,
        rate,
        source: "CoinGecko".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    }))
}