//! SecureBeam Mailbox Server
//!
//! Implements the Magic Wormhole server protocol for P2P file transfer signaling.

mod config;
mod handlers;
mod models;
mod ws;

use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::models::AppState;
use crate::handlers::health_check;
use crate::ws::ws_handler;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "securebeam_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env();

    tracing::info!("Starting SecureBeam Mailbox Server v{}", env!("CARGO_PKG_VERSION"));
    tracing::info!("Listening on {}:{}", config.host, config.port);

    // Create shared state
    let state = Arc::new(AppState::new(config.session_timeout_secs));

    // Spawn cleanup task for expired nameplates and mailboxes
    let cleanup_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            cleanup_state.cleanup_expired().await;
        }
    });

    // Build router
    let app = Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        // WebSocket endpoint for mailbox protocol
        .route("/v1", get(ws_handler))
        // Add middleware
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    // Create listener
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Mailbox server ready at ws://{}/v1", addr);

    // Run server
    axum::serve(listener, app).await.unwrap();
}
