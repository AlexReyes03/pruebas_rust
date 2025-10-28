use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::modules::controllers::{
    admin, bank, convert, health, reputation, wallet,
};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        .route("/health", get(health::health_check))
        
        .route("/wallet/generate", post(wallet::generate_wallet))
        .route("/wallet/fund", post(wallet::fund_wallet))
        .route("/wallet/:pubkey/balance", get(wallet::get_balance))
        .route("/wallet/:pubkey/send", post(wallet::send_transaction))
        
        .route("/reputation/:pubkey", get(reputation::get_reputation))
        
        .route("/convert/to-usdc", post(convert::convert_to_usdc))
        .route("/rates", get(convert::get_rates))
        
        .route("/bank/transfer", post(bank::create_transfer))
        .route("/admin/transfers", get(bank::list_transfers))
        
        .route("/admin/stats", get(admin::get_stats))
        .route("/admin/health-details", get(admin::health_details))
        .route("/admin/aa-accounts", get(admin::list_aa_accounts))
        
        .route("/aa/relayer", post(wallet::aa_relay_transaction))
        
        .with_state(state);

    Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}