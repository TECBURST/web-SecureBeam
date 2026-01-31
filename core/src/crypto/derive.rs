//! Key derivation using HKDF-SHA256
//!
//! Implements the key derivation as specified in the Magic Wormhole client protocol.
//! Different keys are derived for different purposes using HKDF with specific info strings.
//!
//! Security: Derived keys should be wrapped in Zeroizing when stored long-term.

use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::Zeroize;

use super::KEY_SIZE;
use crate::Result;

/// Purpose for key derivation
#[derive(Debug, Clone)]
pub enum Purpose {
    /// Derive the verifier for MITM detection
    Verifier,
    /// Derive a key for a specific phase
    Phase { side: String, phase: String },
    /// Derive the transit encryption key
    Transit,
    /// Custom purpose string
    Custom(String),
}

impl Purpose {
    /// Convert to the info string used in HKDF
    fn to_info(&self) -> Vec<u8> {
        match self {
            Purpose::Verifier => b"wormhole:verifier".to_vec(),
            Purpose::Phase { side, phase } => {
                // Magic Wormhole uses: wormhole:phase:{sha256(side)}:{sha256(phase)}
                let side_hash = sha256_hex(side.as_bytes());
                let phase_hash = sha256_hex(phase.as_bytes());
                format!("wormhole:phase:{}:{}", side_hash, phase_hash).into_bytes()
            }
            Purpose::Transit => b"transit:key".to_vec(),
            Purpose::Custom(s) => s.as_bytes().to_vec(),
        }
    }
}

/// Derive a key using HKDF-SHA256
///
/// - `shared_key`: The shared secret from SPAKE2
/// - `purpose`: The purpose/context for this key
/// - `length`: Desired output key length (default: 32 bytes)
///
/// Note: The caller is responsible for zeroizing the returned key when done.
pub fn derive_key(shared_key: &[u8], purpose: &Purpose, length: usize) -> Result<Vec<u8>> {
    let hkdf = Hkdf::<Sha256>::new(None, shared_key);
    let mut info = purpose.to_info();

    let mut output = vec![0u8; length];
    let result = hkdf
        .expand(&info, &mut output)
        .map_err(|_| crate::Error::Crypto("HKDF expansion failed".to_string()));

    // Zeroize the info string as it may contain sensitive context
    info.zeroize();

    result?;
    Ok(output)
}

/// Derive a 32-byte key for a specific phase
///
/// This is used to encrypt messages in the Wormhole protocol.
pub fn derive_phase_key(shared_key: &[u8], side: &str, phase: &str) -> Result<[u8; KEY_SIZE]> {
    let key = derive_key(
        shared_key,
        &Purpose::Phase {
            side: side.to_string(),
            phase: phase.to_string(),
        },
        KEY_SIZE,
    )?;

    let mut arr = [0u8; KEY_SIZE];
    arr.copy_from_slice(&key);
    Ok(arr)
}

/// Derive the verifier for MITM detection
///
/// Both sides can compare this value (e.g., by reading it aloud over a phone call)
/// to ensure no man-in-the-middle attack occurred.
pub fn derive_verifier(shared_key: &[u8]) -> Result<[u8; KEY_SIZE]> {
    let key = derive_key(shared_key, &Purpose::Verifier, KEY_SIZE)?;

    let mut arr = [0u8; KEY_SIZE];
    arr.copy_from_slice(&key);
    Ok(arr)
}

/// Compute SHA256 and return as hex string
fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Format verifier as human-readable words or hex
///
/// For display to users for MITM verification.
pub fn format_verifier(verifier: &[u8; KEY_SIZE]) -> String {
    // Use first 16 bytes, format as 4 groups of 8 hex chars
    let hex = hex::encode(&verifier[..16]);
    format!(
        "{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..16],
        &hex[16..24],
        &hex[24..32]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_verifier() {
        let shared_key = [0x42u8; 32];

        let verifier1 = derive_verifier(&shared_key).unwrap();
        let verifier2 = derive_verifier(&shared_key).unwrap();

        // Same input should produce same output
        assert_eq!(verifier1, verifier2);

        // Different input should produce different output
        let different_key = [0x43u8; 32];
        let verifier3 = derive_verifier(&different_key).unwrap();
        assert_ne!(verifier1, verifier3);
    }

    #[test]
    fn test_derive_phase_key() {
        let shared_key = [0x42u8; 32];

        let key1 = derive_phase_key(&shared_key, "side-a", "pake").unwrap();
        let key2 = derive_phase_key(&shared_key, "side-a", "version").unwrap();
        let key3 = derive_phase_key(&shared_key, "side-b", "pake").unwrap();

        // Different phases should produce different keys
        assert_ne!(key1, key2);
        // Different sides should produce different keys
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_format_verifier() {
        let verifier = [0xab; KEY_SIZE];
        let formatted = format_verifier(&verifier);

        // Should be 4 groups of 8 hex chars separated by dashes
        assert_eq!(formatted.len(), 35); // 8+1+8+1+8+1+8
        assert!(formatted.contains('-'));
    }

    #[test]
    fn test_purpose_info_strings() {
        assert_eq!(Purpose::Verifier.to_info(), b"wormhole:verifier".to_vec());

        assert_eq!(Purpose::Transit.to_info(), b"transit:key".to_vec());

        // Phase info should include hashed side and phase
        let phase_info = Purpose::Phase {
            side: "test".to_string(),
            phase: "pake".to_string(),
        }
        .to_info();
        let info_str = String::from_utf8(phase_info).unwrap();
        assert!(info_str.starts_with("wormhole:phase:"));
    }
}
