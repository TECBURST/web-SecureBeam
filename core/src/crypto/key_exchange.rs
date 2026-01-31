//! SPAKE2 Password-Authenticated Key Exchange
//!
//! Implements the key exchange protocol as specified in the Magic Wormhole client protocol.
//! Both sides use the wormhole code as the password to derive a shared secret.

use spake2::{Ed25519Group, Identity, Password, Spake2};
use crate::{Error, Result};

/// SPAKE2 message to be exchanged between peers
#[derive(Debug, Clone)]
pub struct Spake2Message(pub Vec<u8>);

impl Spake2Message {
    /// Encode as hex string for transmission
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }

    /// Decode from hex string
    pub fn from_hex(s: &str) -> Result<Self> {
        hex::decode(s)
            .map(Spake2Message)
            .map_err(|e| Error::Crypto(format!("Invalid hex: {}", e)))
    }
}

/// SPAKE2 key exchange state machine
pub struct Spake2Exchange {
    state: ExchangeState,
}

enum ExchangeState {
    /// Ready to start exchange
    Ready {
        password: Vec<u8>,
        side: Side,
    },
    /// Waiting for peer's SPAKE2 message
    WaitingForPeer {
        spake: Spake2<Ed25519Group>,
        outgoing: Spake2Message,
    },
    /// Exchange completed successfully
    Completed {
        shared_key: Vec<u8>,
    },
    /// Exchange failed
    Failed,
}

/// Which side of the exchange we are
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// The side that initiates (sender)
    A,
    /// The side that responds (receiver)
    B,
}

impl Spake2Exchange {
    /// Create a new SPAKE2 exchange
    ///
    /// The password should be the full wormhole code (e.g., "4-purple-sausages").
    /// The side determines the identity used in SPAKE2.
    pub fn new(password: &[u8], side: Side) -> Self {
        Self {
            state: ExchangeState::Ready {
                password: password.to_vec(),
                side,
            },
        }
    }

    /// Start the key exchange, returning the message to send to the peer
    ///
    /// This uses symmetric SPAKE2 with a shared identity,
    /// matching the Magic Wormhole protocol.
    pub fn start(&mut self) -> Result<Spake2Message> {
        let (password, _side) = match std::mem::replace(&mut self.state, ExchangeState::Failed) {
            ExchangeState::Ready { password, side } => (password, side),
            _ => return Err(Error::Crypto("Invalid state: not ready".to_string())),
        };

        // Magic Wormhole uses symmetric SPAKE2 with shared identity
        // The side is used later for key derivation, not for SPAKE2 itself
        let identity = Identity::new(b"");

        let (spake, outgoing) = Spake2::<Ed25519Group>::start_symmetric(
            &Password::new(&password),
            &identity,
        );

        let msg = Spake2Message(outgoing);

        self.state = ExchangeState::WaitingForPeer {
            spake,
            outgoing: msg.clone(),
        };

        Ok(msg)
    }

    /// Get the outgoing message if we're in the waiting state
    pub fn outgoing_message(&self) -> Option<&Spake2Message> {
        match &self.state {
            ExchangeState::WaitingForPeer { outgoing, .. } => Some(outgoing),
            _ => None,
        }
    }

    /// Complete the key exchange with the peer's message
    ///
    /// Returns the shared key on success.
    pub fn finish(&mut self, peer_message: &Spake2Message) -> Result<Vec<u8>> {
        let spake = match std::mem::replace(&mut self.state, ExchangeState::Failed) {
            ExchangeState::WaitingForPeer { spake, .. } => spake,
            _ => return Err(Error::Crypto("Invalid state: not waiting for peer".to_string())),
        };

        let shared_key = spake
            .finish(&peer_message.0)
            .map_err(|_| Error::Crypto("SPAKE2 verification failed - wrong code?".to_string()))?;

        let key = shared_key.to_vec();

        self.state = ExchangeState::Completed {
            shared_key: key.clone(),
        };

        Ok(key)
    }

    /// Check if the exchange is complete
    pub fn is_complete(&self) -> bool {
        matches!(self.state, ExchangeState::Completed { .. })
    }

    /// Get the shared key if exchange is complete
    pub fn shared_key(&self) -> Option<&[u8]> {
        match &self.state {
            ExchangeState::Completed { shared_key } => Some(shared_key),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spake2_exchange() {
        let password = b"4-purple-sausages";

        // Side A (sender)
        let mut exchange_a = Spake2Exchange::new(password, Side::A);
        let msg_a = exchange_a.start().unwrap();

        // Side B (receiver)
        let mut exchange_b = Spake2Exchange::new(password, Side::B);
        let msg_b = exchange_b.start().unwrap();

        // Complete exchange
        let key_a = exchange_a.finish(&msg_b).unwrap();
        let key_b = exchange_b.finish(&msg_a).unwrap();

        // Both sides should derive the same key
        assert_eq!(key_a, key_b);
        assert!(!key_a.is_empty());
    }

    #[test]
    fn test_spake2_wrong_password() {
        let mut exchange_a = Spake2Exchange::new(b"4-purple-sausages", Side::A);
        let mut exchange_b = Spake2Exchange::new(b"5-wrong-password", Side::B);

        let msg_a = exchange_a.start().unwrap();
        let msg_b = exchange_b.start().unwrap();

        // With wrong passwords, keys won't match (but no error during exchange)
        let key_a = exchange_a.finish(&msg_b).unwrap();
        let key_b = exchange_b.finish(&msg_a).unwrap();

        // Keys should be different with wrong password
        assert_ne!(key_a, key_b);
    }

    #[test]
    fn test_hex_encoding() {
        let msg = Spake2Message(vec![0xde, 0xad, 0xbe, 0xef]);
        let hex = msg.to_hex();
        assert_eq!(hex, "deadbeef");

        let decoded = Spake2Message::from_hex(&hex).unwrap();
        assert_eq!(decoded.0, msg.0);
    }
}
