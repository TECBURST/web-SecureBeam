//! Magic Wormhole compatible message types
//!
//! This implements the server protocol as specified in:
//! https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/server-protocol.md

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// All messages from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
    /// Bind to an application ID
    Bind { appid: String, side: String },
    /// List nameplates (optional, for UI)
    List,
    /// Allocate a new nameplate
    Allocate,
    /// Claim an existing nameplate
    Claim { nameplate: String },
    /// Release a nameplate
    Release { nameplate: Option<String> },
    /// Open a mailbox
    Open { mailbox: String },
    /// Add a message to the mailbox
    Add {
        phase: String,
        body: String, // hex-encoded bytes
    },
    /// Close the mailbox
    Close {
        mailbox: Option<String>,
        mood: Option<String>,
    },
    /// Ping to keep connection alive
    Ping { ping: i64 },
}

/// All messages from server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ServerMessage {
    /// Welcome message on connection
    Welcome { welcome: WelcomeInfo },
    /// List of nameplates
    Nameplates { nameplates: Vec<NameplateInfo> },
    /// Nameplate was allocated
    Allocated { nameplate: String },
    /// Nameplate was claimed
    Claimed { mailbox: String },
    /// Nameplate was released
    Released,
    /// A message was received
    Message {
        side: String,
        phase: String,
        body: String, // hex-encoded bytes
        id: u64,
    },
    /// Mailbox was closed
    Closed,
    /// Pong response
    Pong { pong: i64 },
    /// Acknowledgement
    Ack,
    /// Error occurred
    Error {
        error: String,
        orig: Option<serde_json::Value>,
    },
}

/// Welcome information sent on connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeInfo {
    /// Optional message of the day
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motd: Option<String>,
    /// Optional error message (server is unavailable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Server implementation info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
}

impl Default for WelcomeInfo {
    fn default() -> Self {
        Self {
            motd: None,
            error: None,
            server_version: Some("SecureBeam/0.1.0".to_string()),
        }
    }
}

/// Nameplate info for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameplateInfo {
    pub id: String,
}

impl ServerMessage {
    /// Serialize to JSON string
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Create an error response
    pub fn error(msg: &str, orig: Option<&ClientMessage>) -> Self {
        ServerMessage::Error {
            error: msg.to_string(),
            orig: orig.and_then(|m| serde_json::to_value(m).ok()),
        }
    }

    /// Create a welcome message
    pub fn welcome() -> Self {
        ServerMessage::Welcome {
            welcome: WelcomeInfo::default(),
        }
    }
}

/// Mood values for closing mailbox
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mood {
    /// Transfer completed successfully
    Happy,
    /// Transfer was cancelled
    Lonely,
    /// Something went wrong
    Scary,
    /// Lost connection
    Errory,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_message_serialize() {
        let msg = ClientMessage::Bind {
            appid: "test-app".to_string(),
            side: "abc123".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"bind\""));
        assert!(json.contains("\"appid\":\"test-app\""));
    }

    #[test]
    fn test_server_message_serialize() {
        let msg = ServerMessage::Allocated {
            nameplate: "4".to_string(),
        };
        let json = msg.to_json();
        assert!(json.contains("\"type\":\"allocated\""));
        assert!(json.contains("\"nameplate\":\"4\""));
    }

    #[test]
    fn test_welcome_message() {
        let msg = ServerMessage::welcome();
        let json = msg.to_json();
        assert!(json.contains("\"type\":\"welcome\""));
        assert!(json.contains("SecureBeam"));
    }
}
