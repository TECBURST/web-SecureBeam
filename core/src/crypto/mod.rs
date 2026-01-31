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

mod key_exchange;
mod secretbox;
mod derive;

pub use key_exchange::{Spake2Exchange, Spake2Message, Side};
pub use secretbox::{SecretBox, Nonce, NONCE_SIZE, KEY_SIZE, constant_time_eq};
pub use derive::{derive_key, derive_phase_key, derive_verifier, Purpose, format_verifier};
pub use zeroize::Zeroizing;

/// Compute SHA256 hash and return as hex string
pub fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// The application ID for SecureBeam (used in key derivation)
pub const APP_ID: &[u8] = b"securebeam.io/file-transfer";
