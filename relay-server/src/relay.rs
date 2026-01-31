//! Transit Relay implementation
//!
//! Implements the Magic Wormhole transit relay protocol.
//! The relay simply connects two clients and pipes data between them.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

/// Error type for relay operations
#[derive(Debug, thiserror::Error)]
pub enum RelayError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid handshake: {0}")]
    InvalidHandshake(String),
    #[error("Channel not found")]
    ChannelNotFound,
    #[error("Peer disconnected")]
    PeerDisconnected,
}

/// A pending connection waiting for its peer
struct PendingConnection {
    /// The TCP stream
    stream: TcpStream,
    /// The side identifier
    side: String,
}

/// Transit Relay Server
#[derive(Clone)]
pub struct RelayServer {
    /// Pending connections indexed by channel ID
    pending: Arc<RwLock<HashMap<String, PendingConnection>>>,
}

impl RelayServer {
    pub fn new() -> Self {
        Self {
            pending: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Handle a new connection
    pub async fn handle_connection(&self, mut stream: TcpStream) -> Result<(), RelayError> {
        // Read the handshake line (read byte by byte until newline)
        let line = self.read_line(&mut stream).await?;

        // Parse handshake: "please relay {channel_id} for {side}\n"
        let (channel_id, side) = self.parse_handshake(&line)?;

        tracing::info!(
            "Relay request for channel {} from side {}",
            channel_id,
            side
        );

        // Send OK response
        stream.write_all(b"ok\n").await?;

        // Check if peer is already waiting
        let peer = {
            let mut pending = self.pending.write().await;
            pending.remove(&channel_id)
        };

        if let Some(peer_conn) = peer {
            // Peer is waiting - connect them
            tracing::info!(
                "Connecting channel {} ({} <-> {})",
                channel_id,
                side,
                peer_conn.side
            );
            self.relay_streams(stream, peer_conn.stream).await?;
        } else {
            // No peer yet - wait for them
            tracing::debug!("Waiting for peer on channel {}", channel_id);

            let mut pending = self.pending.write().await;
            pending.insert(channel_id.clone(), PendingConnection { stream, side });

            // TODO: Add timeout for pending connections
        }

        Ok(())
    }

    /// Read a line from the stream (until newline)
    async fn read_line(&self, stream: &mut TcpStream) -> Result<String, RelayError> {
        use tokio::io::AsyncReadExt;

        let mut buffer = Vec::with_capacity(256);
        let mut byte = [0u8; 1];

        loop {
            let n = stream.read(&mut byte).await?;
            if n == 0 {
                break; // EOF
            }
            buffer.push(byte[0]);
            if byte[0] == b'\n' {
                break;
            }
            if buffer.len() > 1024 {
                return Err(RelayError::InvalidHandshake("Line too long".to_string()));
            }
        }

        String::from_utf8(buffer)
            .map_err(|_| RelayError::InvalidHandshake("Invalid UTF-8".to_string()))
    }

    /// Parse the handshake message
    fn parse_handshake(&self, line: &str) -> Result<(String, String), RelayError> {
        // Format: "please relay {channel_id} for {side}\n"
        let line = line.trim();

        if !line.starts_with("please relay ") {
            return Err(RelayError::InvalidHandshake(
                "Expected 'please relay ...'".to_string(),
            ));
        }

        let rest = &line[13..]; // Skip "please relay "

        let parts: Vec<&str> = rest.split(" for ").collect();
        if parts.len() != 2 {
            return Err(RelayError::InvalidHandshake(
                "Expected 'please relay {channel} for {side}'".to_string(),
            ));
        }

        let channel_id = parts[0].to_string();
        let side = parts[1].to_string();

        // Validate channel ID (should be hex)
        if channel_id.len() < 16 {
            return Err(RelayError::InvalidHandshake(
                "Channel ID too short".to_string(),
            ));
        }

        Ok((channel_id, side))
    }

    /// Relay data between two streams
    async fn relay_streams(
        &self,
        stream1: TcpStream,
        stream2: TcpStream,
    ) -> Result<(), RelayError> {
        // Use into_split() to get owned halves that can be moved into spawned tasks
        let (mut read1, mut write1) = stream1.into_split();
        let (mut read2, mut write2) = stream2.into_split();

        // Spawn two tasks to copy data in both directions
        let task1 = tokio::spawn(async move { tokio::io::copy(&mut read1, &mut write2).await });

        let task2 = tokio::spawn(async move { tokio::io::copy(&mut read2, &mut write1).await });

        // Wait for either direction to finish
        tokio::select! {
            result = task1 => {
                match result {
                    Ok(Ok(bytes)) => tracing::debug!("Direction 1 finished, {} bytes", bytes),
                    Ok(Err(e)) => tracing::debug!("Direction 1 error: {}", e),
                    Err(e) => tracing::debug!("Direction 1 task error: {}", e),
                }
            }
            result = task2 => {
                match result {
                    Ok(Ok(bytes)) => tracing::debug!("Direction 2 finished, {} bytes", bytes),
                    Ok(Err(e)) => tracing::debug!("Direction 2 error: {}", e),
                    Err(e) => tracing::debug!("Direction 2 task error: {}", e),
                }
            }
        }

        tracing::info!("Relay connection closed");
        Ok(())
    }
}

impl Default for RelayServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_handshake() {
        let relay = RelayServer::new();

        // Valid handshake
        let (channel, side) = relay
            .parse_handshake("please relay 0123456789abcdef0123456789abcdef for sender\n")
            .unwrap();
        assert_eq!(channel, "0123456789abcdef0123456789abcdef");
        assert_eq!(side, "sender");

        // Invalid format
        assert!(relay.parse_handshake("invalid").is_err());
        assert!(relay
            .parse_handshake("please relay short for sender")
            .is_err());
    }
}
