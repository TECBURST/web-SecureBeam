//! Cryptographic operations for SecureBeam
//!
//! Uses SPAKE2 for password-authenticated key exchange (PAKE)

use spake2::{Ed25519Group, Identity, Password, Spake2};
use sha2::{Sha256, Digest};

use crate::{Error, Result};

/// Session key derived from PAKE
#[derive(Clone)]
pub struct SessionKey {
    key: Vec<u8>,
}

impl SessionKey {
    /// Create a session key from raw bytes
    pub fn from_bytes(key: Vec<u8>) -> Self {
        Self { key }
    }

    /// Get the key bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }

    /// Derive an encryption key using HKDF-like construction
    pub fn derive_encryption_key(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.key);
        hasher.update(b"encryption");
        hasher.finalize().into()
    }

    /// Derive a MAC key using HKDF-like construction
    pub fn derive_mac_key(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.key);
        hasher.update(b"mac");
        hasher.finalize().into()
    }
}

/// Key exchange state machine
pub struct KeyExchange {
    state: KeyExchangeState,
}

enum KeyExchangeState {
    /// Initial state, ready to start
    Ready {
        code: String,
        is_initiator: bool,
    },
    /// Waiting for peer's message
    WaitingForPeer {
        spake: Spake2<Ed25519Group>,
        outgoing_message: Vec<u8>,
    },
    /// Key exchange completed
    Completed {
        session_key: SessionKey,
    },
    /// Error occurred
    Failed,
}

impl KeyExchange {
    /// Create a new key exchange as the initiator (sender)
    pub fn new_initiator(code: &str) -> Self {
        Self {
            state: KeyExchangeState::Ready {
                code: code.to_string(),
                is_initiator: true,
            },
        }
    }

    /// Create a new key exchange as the responder (receiver)
    pub fn new_responder(code: &str) -> Self {
        Self {
            state: KeyExchangeState::Ready {
                code: code.to_string(),
                is_initiator: false,
            },
        }
    }

    /// Start the key exchange and return the first message to send
    pub fn start(&mut self) -> Result<Vec<u8>> {
        let (code, is_initiator) = match &self.state {
            KeyExchangeState::Ready { code, is_initiator } => (code.clone(), *is_initiator),
            _ => return Err(Error::Crypto("Invalid state for start".to_string())),
        };

        let password = Password::new(code.as_bytes());
        let identity = if is_initiator {
            Identity::new(b"sender")
        } else {
            Identity::new(b"receiver")
        };

        let (spake, outgoing) = Spake2::<Ed25519Group>::start_symmetric(&password, &identity);

        self.state = KeyExchangeState::WaitingForPeer {
            spake,
            outgoing_message: outgoing.clone(),
        };

        Ok(outgoing)
    }

    /// Process the peer's message and complete the key exchange
    pub fn finish(&mut self, peer_message: &[u8]) -> Result<SessionKey> {
        let spake = match std::mem::replace(&mut self.state, KeyExchangeState::Failed) {
            KeyExchangeState::WaitingForPeer { spake, .. } => spake,
            _ => return Err(Error::Crypto("Invalid state for finish".to_string())),
        };

        let key = spake
            .finish(peer_message)
            .map_err(|_| Error::Crypto("PAKE verification failed".to_string()))?;

        let session_key = SessionKey::from_bytes(key.to_vec());

        self.state = KeyExchangeState::Completed {
            session_key: session_key.clone(),
        };

        Ok(session_key)
    }

    /// Get the outgoing message (if in the appropriate state)
    pub fn get_outgoing_message(&self) -> Option<&[u8]> {
        match &self.state {
            KeyExchangeState::WaitingForPeer { outgoing_message, .. } => Some(outgoing_message),
            _ => None,
        }
    }

    /// Check if the key exchange is complete
    pub fn is_complete(&self) -> bool {
        matches!(self.state, KeyExchangeState::Completed { .. })
    }

    /// Get the session key (if exchange is complete)
    pub fn get_session_key(&self) -> Option<&SessionKey> {
        match &self.state {
            KeyExchangeState::Completed { session_key } => Some(session_key),
            _ => None,
        }
    }
}

/// Compute SHA-256 hash of data
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Compute SHA-256 hash of data and return as hex string
pub fn sha256_hex(data: &[u8]) -> String {
    let hash = sha256(data);
    hex::encode(hash)
}

// Re-export hex for convenience
mod hex {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

    pub fn encode(data: &[u8]) -> String {
        let mut result = String::with_capacity(data.len() * 2);
        for byte in data {
            result.push(HEX_CHARS[(byte >> 4) as usize] as char);
            result.push(HEX_CHARS[(byte & 0x0f) as usize] as char);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_exchange() {
        let code = "7-test-code";

        let mut sender = KeyExchange::new_initiator(code);
        let mut receiver = KeyExchange::new_responder(code);

        let sender_msg = sender.start().unwrap();
        let receiver_msg = receiver.start().unwrap();

        let sender_key = sender.finish(&receiver_msg).unwrap();
        let receiver_key = receiver.finish(&sender_msg).unwrap();

        assert_eq!(sender_key.as_bytes(), receiver_key.as_bytes());
    }
}
