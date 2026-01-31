//! Relay connection establishment
//!
//! Connects to a relay server when direct connections fail.

use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use sha2::{Sha256, Digest};

use crate::{Error, Result};
use super::hints::RelayHint;
use super::connection::{TransitConnection, TransitRole, perform_handshake};
use super::HANDSHAKE_TIMEOUT_SECS;

/// Read a line from the stream (until newline)
async fn read_line(stream: &mut TcpStream) -> Result<String> {
    let mut buffer = Vec::with_capacity(64);
    let mut byte = [0u8; 1];

    loop {
        let n = stream.read(&mut byte).await
            .map_err(|e| Error::Connection(format!("Read failed: {}", e)))?;
        if n == 0 {
            break; // EOF
        }
        buffer.push(byte[0]);
        if byte[0] == b'\n' {
            break;
        }
        if buffer.len() > 256 {
            return Err(Error::Connection("Response too long".to_string()));
        }
    }

    String::from_utf8(buffer)
        .map_err(|_| Error::Connection("Invalid UTF-8 in response".to_string()))
}

/// Connect to a relay server
pub async fn connect_via_relay(
    role: TransitRole,
    relay: &RelayHint,
    transit_key: &[u8],
) -> Result<TransitConnection> {
    let (host, port) = relay.parse()
        .ok_or_else(|| Error::Connection(format!("Invalid relay URL: {}", relay.url)))?;

    let addr = format!("{}:{}", host, port);
    tracing::debug!("Connecting to relay at {}", addr);

    let timeout_duration = Duration::from_secs(HANDSHAKE_TIMEOUT_SECS);

    // Connect to relay
    let mut stream = timeout(timeout_duration, TcpStream::connect(&addr)).await
        .map_err(|_| Error::Connection("Relay connect timed out".to_string()))?
        .map_err(|e| Error::Connection(format!("Relay connect failed: {}", e)))?;

    // Compute channel ID from transit key
    let mut hasher = Sha256::new();
    hasher.update(transit_key);
    hasher.update(b"transit-relay-channel");
    let hash = hasher.finalize();
    let channel_id = hex::encode(&hash);

    // Determine side string
    let side = match role {
        TransitRole::Sender => "sender",
        TransitRole::Receiver => "receiver",
    };

    // Send relay handshake
    let handshake = format!("please relay {} for {}\n", channel_id, side);

    stream.write_all(handshake.as_bytes()).await
        .map_err(|e| Error::Connection(format!("Relay handshake write failed: {}", e)))?;

    // Read relay response (read byte by byte to avoid borrowing issues)
    let response = timeout(timeout_duration, read_line(&mut stream)).await
        .map_err(|_| Error::Connection("Relay response timed out".to_string()))??;

    if response.trim() != "ok" {
        return Err(Error::Connection(format!("Relay rejected: {}", response.trim())));
    }

    tracing::debug!("Relay accepted, performing transit handshake");

    // Now perform the transit handshake over the relay
    perform_handshake(&mut stream, role, transit_key).await?;

    // Create encrypted connection
    TransitConnection::new(stream, transit_key, role)
}
