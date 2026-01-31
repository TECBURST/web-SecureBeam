//! Direct P2P connection establishment
//!
//! Tries to connect directly to the peer using provided hints.

use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

use super::connection::{perform_handshake, TransitConnection, TransitRole};
use super::hints::DirectHint;
use super::HANDSHAKE_TIMEOUT_SECS;
use crate::{Error, Result};

/// Try to establish a direct connection using the given hints
pub async fn try_direct_connection(
    role: TransitRole,
    hints: &[DirectHint],
    transit_key: &[u8],
) -> Result<TransitConnection> {
    let timeout_duration = Duration::from_secs(HANDSHAKE_TIMEOUT_SECS);

    // Try each hint
    for hint in hints {
        let addr = hint.to_addr_string();
        tracing::debug!("Trying direct connection to {}", addr);

        match timeout(
            timeout_duration,
            try_single_connection(&addr, role, transit_key),
        )
        .await
        {
            Ok(Ok(conn)) => {
                tracing::info!("Direct connection successful to {}", addr);
                return Ok(conn);
            }
            Ok(Err(e)) => {
                tracing::debug!("Direct connection to {} failed: {}", addr, e);
            }
            Err(_) => {
                tracing::debug!("Direct connection to {} timed out", addr);
            }
        }
    }

    Err(Error::Connection(
        "All direct connection attempts failed".to_string(),
    ))
}

/// Try a single direct connection
async fn try_single_connection(
    addr: &str,
    role: TransitRole,
    transit_key: &[u8],
) -> Result<TransitConnection> {
    // Connect
    let mut stream = TcpStream::connect(addr)
        .await
        .map_err(|e| Error::Connection(format!("Connect failed: {}", e)))?;

    // Perform handshake
    perform_handshake(&mut stream, role, transit_key).await?;

    // Create encrypted connection
    TransitConnection::new(stream, transit_key, role)
}

/// Listen for incoming direct connections
#[allow(dead_code)]
pub async fn listen_for_direct(
    port: u16,
    role: TransitRole,
    transit_key: &[u8],
) -> Result<TransitConnection> {
    use tokio::net::TcpListener;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .map_err(|e| Error::Connection(format!("Bind failed: {}", e)))?;

    tracing::debug!("Listening for direct connections on port {}", port);

    let timeout_duration = Duration::from_secs(HANDSHAKE_TIMEOUT_SECS);

    match timeout(timeout_duration, listener.accept()).await {
        Ok(Ok((mut stream, peer_addr))) => {
            tracing::debug!("Incoming connection from {}", peer_addr);

            // Perform handshake
            perform_handshake(&mut stream, role, transit_key).await?;

            // Create encrypted connection
            TransitConnection::new(stream, transit_key, role)
        }
        Ok(Err(e)) => Err(Error::Connection(format!("Accept failed: {}", e))),
        Err(_) => Err(Error::Connection("Listen timed out".to_string())),
    }
}
