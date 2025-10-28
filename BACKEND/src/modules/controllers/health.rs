use axum::Json;
use serde_json::{json, Value};

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "wallet-backend",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}