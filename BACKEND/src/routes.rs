use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::modules::controllers::{
    bank, convert, health, reputation, wallet,
};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    // CORS configuration for frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // API routes
    let api_routes = Router::new()
        // Health check
        .route("/health", get(health::health_check))
        
        // Wallet endpoints
        .route("/wallet/generate", post(wallet::generate_wallet))
        .route("/wallet/fund", post(wallet::fund_wallet))
        .route("/wallet/:pubkey/balance", get(wallet::get_balance))
        .route("/wallet/:pubkey/send", post(wallet::send_transaction))
        
        // Reputation endpoints
        .route("/reputation/:pubkey", get(reputation::get_reputation))
        
        // Convert endpoints
        .route("/convert/to-usdc", post(convert::convert_to_usdc))
        .route("/rates", get(convert::get_rates))
        
        // Bank transfer endpoints
        .route("/bank/transfer", post(bank::create_transfer))
        .route("/admin/transfers", get(bank::list_transfers))
        
        // Account Abstraction simulation endpoints
        .route("/aa/relayer", post(wallet::aa_relay_transaction))
        
        .with_state(state);

    // Main router with /api prefix
    Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}