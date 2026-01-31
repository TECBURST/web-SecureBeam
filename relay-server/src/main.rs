//! SecureBeam Transit Relay Server
//!
//! This server provides a fallback relay for P2P connections when direct
//! connections are not possible (NAT, firewalls, etc.).
//!
//! Protocol:
//! 1. Client connects and sends: "please relay {channel_id} for {side}\n"
//! 2. Server responds: "ok\n"
//! 3. When both sides connect, server pipes data between them

mod relay;

use std::env;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use relay::RelayServer;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "securebeam_relay=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenvy::dotenv().ok();
    let host = env::var("RELAY_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("RELAY_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(4001);

    let addr = format!("{}:{}", host, port);

    tracing::info!(
        "Starting SecureBeam Transit Relay Server v{}",
        env!("CARGO_PKG_VERSION")
    );
    tracing::info!("Listening on {}", addr);

    // Create relay server
    let relay = RelayServer::new();

    // Start listening
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    tracing::info!("Relay server ready");

    // Accept connections
    loop {
        match listener.accept().await {
            Ok((socket, peer_addr)) => {
                tracing::debug!("New connection from {}", peer_addr);
                let relay = relay.clone();
                tokio::spawn(async move {
                    if let Err(e) = relay.handle_connection(socket).await {
                        tracing::warn!("Connection error from {}: {}", peer_addr, e);
                    }
                });
            }
            Err(e) => {
                tracing::error!("Accept error: {}", e);
            }
        }
    }
}
