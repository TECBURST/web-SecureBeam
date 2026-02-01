//! Cryptographic primitives for SecureBeam
//!
//! This module implements the Magic Wormhole cryptographic protocol:
//! - SPAKE2 for password-authenticated key exchange
//! - NaCl SecretBox (XSalsa20-Poly1305) for authenticated encryption
//! - HKDF-SHA256 for key derivation
//!
//! Security features:
//! - All sensitive keys are zeroized on drop
//! - Constant-time comparisons for cryptographic operations
//! - Secure random number generation

mod derive;
mod key_exchange;
mod secretbox;

pub use derive::{derive_key, derive_phase_key, derive_verifier, format_verifier, Purpose};
pub use key_exchange::{Side, Spake2Exchange, Spake2Message};
pub use secretbox::{constant_time_eq, Nonce, SecretBox, KEY_SIZE, NONCE_SIZE};
pub use zeroize::Zeroizing;

/// Compute SHA256 hash and return as hex string
pub fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// The application ID for SecureBeam (used in key derivation)
pub const APP_ID: &[u8] = b"securebeam.eu/file-transfer";
