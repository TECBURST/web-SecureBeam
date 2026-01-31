//! SecureBeam Core Library
//!
//! This library provides the core functionality for SecureBeam P2P file transfers,
//! implementing the Magic Wormhole protocol for secure, authenticated file sharing.
//!
//! # Modules
//!
//! - `crypto` - Cryptographic operations (SPAKE2, NaCl SecretBox, HKDF)
//! - `protocol` - Protocol definitions and message types
//! - `transfer` - File transfer logic with compression
//! - `transit` - P2P connection establishment (direct + relay)
//! - `network` - Network abstractions and WebSocket client
//!
//! # Security
//!
//! This library implements the Magic Wormhole protocol:
//! - SPAKE2 for password-authenticated key exchange
//! - NaCl SecretBox (XSalsa20-Poly1305) for authenticated encryption
//! - HKDF-SHA256 for key derivation

pub mod crypto;
pub mod protocol;
pub mod transfer;
pub mod transit;
pub mod network;

// Re-export commonly used types
pub use crypto::{
    Spake2Exchange, Spake2Message,
    SecretBox, Nonce,
    derive_key, derive_phase_key, derive_verifier, Purpose,
};
pub use protocol::{Message, TransferRequest, TransferResponse};
pub use transfer::{FileTransfer, TransferProgress};
pub use transit::{TransitConnection, TransitRole, TransitHints, establish_transit};
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

    #[error("Wrong wormhole code")]
    WrongCode,

    #[error("MITM attack detected")]
    MitmDetected,
}
