use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::time::Instant;

pub async fn log_request(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = Instant::now();

    tracing::info!("--> {} {}", method, uri);

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    let log_level = match status.as_u16() {
        200..=299 => tracing::Level::INFO,
        400..=499 => tracing::Level::WARN,
        500..=599 => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    tracing::event!(
        log_level,
        "<-- {} {} {} ({:.2}ms)",
        method,
        uri,
        status.as_u16(),
        duration.as_millis()
    );

    response
}

pub async fn handle_timeout_error(err: std::io::Error) -> impl IntoResponse {
    tracing::error!("Request timeout: {}", err);
    (
        StatusCode::REQUEST_TIMEOUT,
        "Request took too long to process",
    )
}