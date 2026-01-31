//! NaCl SecretBox (XSalsa20-Poly1305) authenticated encryption
//!
//! This implements the encryption used by Magic Wormhole for all encrypted
//! messages after the PAKE exchange.
//!
//! Security features:
//! - Keys are securely zeroed on drop using zeroize
//! - Constant-time comparisons for sensitive data
//! - Random nonce generation for each encryption

use xsalsa20poly1305::{
    aead::{Aead, KeyInit},
    XSalsa20Poly1305,
};
use rand::RngCore;
use subtle::ConstantTimeEq;
use zeroize::ZeroizeOnDrop;

use crate::{Error, Result};

/// Size of a SecretBox key in bytes (256 bits)
pub const KEY_SIZE: usize = 32;

/// Size of a SecretBox nonce in bytes (192 bits)
pub const NONCE_SIZE: usize = 24;

/// Size of the authentication tag in bytes (128 bits)
pub const TAG_SIZE: usize = 16;

/// A 24-byte nonce for SecretBox
#[derive(Debug, Clone)]
pub struct Nonce(pub [u8; NONCE_SIZE]);

impl Nonce {
    /// Generate a random nonce
    pub fn random() -> Self {
        let mut nonce = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut nonce);
        Nonce(nonce)
    }

    /// Create a nonce from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != NONCE_SIZE {
            return Err(Error::Crypto(format!(
                "Invalid nonce size: expected {}, got {}",
                NONCE_SIZE,
                bytes.len()
            )));
        }
        let mut nonce = [0u8; NONCE_SIZE];
        nonce.copy_from_slice(bytes);
        Ok(Nonce(nonce))
    }
}

/// NaCl SecretBox for authenticated encryption
///
/// The key is automatically zeroed when this struct is dropped.
#[derive(ZeroizeOnDrop)]
pub struct SecretBox {
    #[zeroize(skip)]
    cipher: XSalsa20Poly1305,
    key: [u8; KEY_SIZE],
}

impl SecretBox {
    /// Create a new SecretBox with the given key
    pub fn new(key: &[u8]) -> Result<Self> {
        if key.len() != KEY_SIZE {
            return Err(Error::Crypto(format!(
                "Invalid key size: expected {}, got {}",
                KEY_SIZE,
                key.len()
            )));
        }

        let mut key_arr = [0u8; KEY_SIZE];
        key_arr.copy_from_slice(key);

        let cipher = XSalsa20Poly1305::new((&key_arr).into());

        Ok(Self {
            cipher,
            key: key_arr,
        })
    }

    /// Encrypt a message with a random nonce
    ///
    /// Returns (nonce, ciphertext) where ciphertext includes the auth tag.
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<(Nonce, Vec<u8>)> {
        let nonce = Nonce::random();
        let ciphertext = self.encrypt_with_nonce(plaintext, &nonce)?;
        Ok((nonce, ciphertext))
    }

    /// Encrypt a message with a specific nonce
    pub fn encrypt_with_nonce(&self, plaintext: &[u8], nonce: &Nonce) -> Result<Vec<u8>> {
        self.cipher
            .encrypt((&nonce.0).into(), plaintext)
            .map_err(|_| Error::Crypto("Encryption failed".to_string()))
    }

    /// Decrypt a message
    pub fn decrypt(&self, nonce: &Nonce, ciphertext: &[u8]) -> Result<Vec<u8>> {
        self.cipher
            .decrypt((&nonce.0).into(), ciphertext)
            .map_err(|_| Error::Crypto("Decryption failed - invalid ciphertext or wrong key".to_string()))
    }

    /// Encrypt and prepend the nonce to the ciphertext
    ///
    /// Output format: nonce (24 bytes) || ciphertext (includes 16-byte tag)
    pub fn seal(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let (nonce, ciphertext) = self.encrypt(plaintext)?;
        let mut output = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
        output.extend_from_slice(&nonce.0);
        output.extend_from_slice(&ciphertext);
        Ok(output)
    }

    /// Decrypt a sealed message (nonce prepended to ciphertext)
    pub fn open(&self, sealed: &[u8]) -> Result<Vec<u8>> {
        if sealed.len() < NONCE_SIZE + TAG_SIZE {
            return Err(Error::Crypto("Sealed message too short".to_string()));
        }

        let nonce = Nonce::from_bytes(&sealed[..NONCE_SIZE])?;
        let ciphertext = &sealed[NONCE_SIZE..];

        self.decrypt(&nonce, ciphertext)
    }
}

// Note: Key zeroing is handled automatically by ZeroizeOnDrop derive

/// Constant-time comparison of two byte slices
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0x42u8; KEY_SIZE];
        let sb = SecretBox::new(&key).unwrap();

        let plaintext = b"Hello, SecureBeam!";
        let (nonce, ciphertext) = sb.encrypt(plaintext).unwrap();

        let decrypted = sb.decrypt(&nonce, &ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_seal_open() {
        let key = [0x42u8; KEY_SIZE];
        let sb = SecretBox::new(&key).unwrap();

        let plaintext = b"Secret message";
        let sealed = sb.seal(plaintext).unwrap();

        // Sealed should be: nonce (24) + ciphertext (14) + tag (16) = 54 bytes
        assert_eq!(sealed.len(), NONCE_SIZE + plaintext.len() + TAG_SIZE);

        let opened = sb.open(&sealed).unwrap();
        assert_eq!(opened, plaintext);
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1 = [0x42u8; KEY_SIZE];
        let key2 = [0x43u8; KEY_SIZE];

        let sb1 = SecretBox::new(&key1).unwrap();
        let sb2 = SecretBox::new(&key2).unwrap();

        let plaintext = b"Secret";
        let sealed = sb1.seal(plaintext).unwrap();

        // Decryption with wrong key should fail
        assert!(sb2.open(&sealed).is_err());
    }

    #[test]
    fn test_tampered_ciphertext_fails() {
        let key = [0x42u8; KEY_SIZE];
        let sb = SecretBox::new(&key).unwrap();

        let plaintext = b"Secret";
        let mut sealed = sb.seal(plaintext).unwrap();

        // Tamper with the ciphertext
        sealed[NONCE_SIZE + 5] ^= 0xFF;

        // Decryption should fail due to authentication
        assert!(sb.open(&sealed).is_err());
    }

    #[test]
    fn test_constant_time_eq() {
        let a = b"hello";
        let b = b"hello";
        let c = b"world";
        let d = b"hell";

        assert!(constant_time_eq(a, b));
        assert!(!constant_time_eq(a, c));
        assert!(!constant_time_eq(a, d));
    }
}
