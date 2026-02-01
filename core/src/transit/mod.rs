//! Transit module for establishing P2P connections
//!
//! This module implements the Magic Wormhole transit protocol:
//! 1. Try direct P2P connection using local IPs and STUN
//! 2. Fall back to relay server if direct fails
//!
//! The transit is encrypted using a key derived from the wormhole session.

mod connection;
mod direct;
mod hints;
mod relay;

pub use connection::{TransitConnection, TransitRole};
pub use direct::try_direct_connection;
pub use hints::{DirectHint, RelayHint, TransitHints};
pub use relay::connect_via_relay;

use crate::{Error, Result};

/// Default relay server URL
pub const DEFAULT_RELAY: &str = "tcp://relay.securebeam.eu:4001";

/// Transit handshake timeout in seconds
pub const HANDSHAKE_TIMEOUT_SECS: u64 = 30;

/// Establish a transit connection
///
/// Tries direct connection first, falls back to relay if needed.
pub async fn establish_transit(
    role: TransitRole,
    hints: &TransitHints,
    transit_key: &[u8],
) -> Result<TransitConnection> {
    // Try direct connections first
    if !hints.direct_hints.is_empty() {
        tracing::info!(
            "Trying {} direct connection hints",
            hints.direct_hints.len()
        );

        match try_direct_connection(role, &hints.direct_hints, transit_key).await {
            Ok(conn) => {
                tracing::info!("Direct connection established");
                return Ok(conn);
            }
            Err(e) => {
                tracing::warn!("Direct connection failed: {}", e);
            }
        }
    }

    // Fall back to relay
    if !hints.relay_hints.is_empty() {
        tracing::info!("Falling back to relay connection");

        for relay in &hints.relay_hints {
            match connect_via_relay(role, relay, transit_key).await {
                Ok(conn) => {
                    tracing::info!("Relay connection established via {}", relay.url);
                    return Ok(conn);
                }
                Err(e) => {
                    tracing::warn!("Relay {} failed: {}", relay.url, e);
                }
            }
        }
    }

    Err(Error::Connection(
        "All transit connection attempts failed".to_string(),
    ))
}
