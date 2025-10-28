use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),

    #[error("Invalid public key format: {0}")]
    InvalidPublicKey(String),

    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: String, available: String },

    #[error("Reputation score too low: current {current}, required {required}")]
    ReputationTooLow { current: u8, required: u8 },

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Account abstraction error: {0}")]
    AccountAbstractionError(String),

    #[error("Stellar network error: {0}")]
    StellarNetworkError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("External API error: {0}")]
    ExternalApiError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal server error")]
    InternalError(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            AppError::WalletNotFound(_) => {
                (StatusCode::NOT_FOUND, "WALLET_NOT_FOUND", self.to_string())
            }
            AppError::InvalidPublicKey(_) => (
                StatusCode::BAD_REQUEST,
                "INVALID_PUBLIC_KEY",
                self.to_string(),
            ),
            AppError::InsufficientBalance { .. } => (
                StatusCode::BAD_REQUEST,
                "INSUFFICIENT_BALANCE",
                self.to_string(),
            ),
            AppError::ReputationTooLow { .. } => (
                StatusCode::FORBIDDEN,
                "REPUTATION_TOO_LOW",
                self.to_string(),
            ),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", self.to_string()),
            AppError::AccountAbstractionError(_) => {
                (StatusCode::BAD_REQUEST, "AA_ERROR", self.to_string())
            }
            AppError::StellarNetworkError(_) => {
                (StatusCode::BAD_GATEWAY, "STELLAR_ERROR", self.to_string())
            }
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                "Database operation failed".to_string(),
            ),
            AppError::ExternalApiError(_) => (
                StatusCode::BAD_GATEWAY,
                "EXTERNAL_API_ERROR",
                self.to_string(),
            ),
            AppError::ConfigError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "CONFIG_ERROR",
                "Configuration error".to_string(),
            ),
            AppError::InternalError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Internal server error".to_string(),
            ),
            AppError::NotImplemented(_) => (
                StatusCode::NOT_IMPLEMENTED,
                "NOT_IMPLEMENTED",
                self.to_string(),
            ),
        };

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": message,
            }
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::ExternalApiError(err.to_string())
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::ConfigError(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}
