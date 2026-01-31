//! SecureBeam Core Library
//!
//! This library provides the core functionality for SecureBeam P2P file transfers.
//!
//! # Modules
//!
//! - `protocol` - Protocol definitions and message types
//! - `crypto` - Cryptographic operations (PAKE, encryption)
//! - `transfer` - File transfer logic
//! - `network` - Network abstractions and WebSocket client

pub mod protocol;
pub mod crypto;
pub mod transfer;
pub mod network;

pub use protocol::{Message, TransferRequest, TransferResponse};
pub use crypto::{KeyExchange, SessionKey};
pub use transfer::{FileTransfer, TransferProgress};
pub use network::SignalingClient;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type for SecureBeam operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for SecureBeam operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Transfer error: {0}")]
    Transfer(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Session not found")]
    SessionNotFound,

    #[error("Session expired")]
    SessionExpired,

    #[error("Peer disconnected")]
    PeerDisconnected,
}
