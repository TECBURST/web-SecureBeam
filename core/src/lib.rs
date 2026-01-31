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
//!
//! Additional security measures:
//! - Sensitive data (keys, passwords) are zeroized on drop
//! - Constant-time comparisons for cryptographic operations
//! - Path traversal protection for archive extraction
//! - Input validation with size limits

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
pub use protocol::{Message, FileOffer, FileAnswer, OfferType};
pub use transfer::{FileTransfer, TransferProgress};
pub use transit::{TransitConnection, TransitRole, TransitHints, establish_transit};
pub use network::SignalingClient;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type for SecureBeam operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for SecureBeam operations
///
/// Note: Error messages are intentionally generic to avoid leaking
/// sensitive information to potential attackers.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Connection error")]
    Connection(String),

    #[error("Protocol error")]
    Protocol(String),

    #[error("Cryptographic operation failed")]
    Crypto(String),

    #[error("Transfer failed")]
    Transfer(String),

    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Session not found")]
    SessionNotFound,

    #[error("Session expired")]
    SessionExpired,

    #[error("Peer disconnected")]
    PeerDisconnected,

    #[error("Authentication failed")]
    WrongCode,

    #[error("Security verification failed")]
    MitmDetected,
}

impl Error {
    /// Get detailed error information for logging (not for display to users)
    ///
    /// This method returns the internal error details which should only be
    /// used for debugging/logging, not shown to end users.
    pub fn details(&self) -> Option<&str> {
        match self {
            Error::Connection(s) => Some(s),
            Error::Protocol(s) => Some(s),
            Error::Crypto(s) => Some(s),
            Error::Transfer(s) => Some(s),
            _ => None,
        }
    }
}
