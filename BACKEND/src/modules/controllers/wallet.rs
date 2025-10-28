use axum::{
    extract::{Path, State},
    Json,
};
use crate::error::AppError;
use crate::modules::models::wallet::*;
use crate::state::AppState;

pub async fn generate_wallet(
    State(state): State<AppState>,
    Json(payload): Json<GenerateWalletRequest>,
) -> Result<Json<GenerateWalletResponse>, AppError> {
    let response = state
        .wallet_service
        .generate_wallet(payload.aa_mode, payload.reveal_secret)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(Json(response))
}

pub async fn fund_wallet(
    State(state): State<AppState>,
    Json(payload): Json<FundWalletRequest>,
) -> Result<Json<FundWalletResponse>, AppError> {
    let tx_hash = state
        .wallet_service
        .fund_wallet(&payload.public_key)
        .await
        .map_err(|e| {
            if e.to_string().contains("account already exists") {
                AppError::BadRequest("Account already funded or exists".to_string())
            } else {
                AppError::StellarNetworkError(e.to_string())
            }
        })?;

    Ok(Json(FundWalletResponse {
        public_key: payload.public_key,
        status: "funded".to_string(),
        tx_hash: Some(tx_hash),
    }))
}

pub async fn get_balance(
    State(state): State<AppState>,
    Path(pubkey): Path<String>,
) -> Result<Json<BalanceResponse>, AppError> {
    let (balances, recent_txs) = state
        .wallet_service
        .get_balance(&pubkey)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                AppError::WalletNotFound(pubkey.clone())
            } else {
                AppError::InternalError(e.to_string())
            }
        })?;

    let balance_list: Vec<Balance> = balances
        .into_iter()
        .map(|(asset_code, balance)| Balance {
            asset_code,
            balance,
            asset_issuer: None,
        })
        .collect();

    Ok(Json(BalanceResponse {
        public_key: pubkey,
        balances: balance_list,
        recent_transactions: recent_txs,
    }))
}

pub async fn send_transaction(
    State(state): State<AppState>,
    Path(pubkey): Path<String>,
    Json(payload): Json<SendTransactionRequest>,
) -> Result<Json<SendTransactionResponse>, AppError> {
    let tx_hash = state
        .wallet_service
        .send_transaction(
            &pubkey,
            &payload.destination,
            &payload.amount,
            payload.asset_code.as_deref(),
        )
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(Json(SendTransactionResponse {
        tx_hash,
        status: "completed".to_string(),
    }))
}

pub async fn aa_relay_transaction(
    State(state): State<AppState>,
    Json(payload): Json<RelayTransactionRequest>,
) -> Result<Json<RelayTransactionResponse>, AppError> {
    let has_signer = state.aa_service.has_signer(&payload.public_key).await;
    
    if !has_signer {
        return Err(AppError::AccountAbstractionError(
            "No AA signer registered for this account".to_string(),
        ));
    }

    let tx_hash = state
        .aa_service
        .relay_transaction(&payload.public_key, &payload.tx_xdr)
        .await
        .map_err(|e| AppError::AccountAbstractionError(e.to_string()))?;

    Ok(Json(RelayTransactionResponse {
        tx_hash,
        status: "relayed".to_string(),
    }))
}