use serde::{Deserialize, Serialize};

/// WebSocket message wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum WsMessage {
    Client(ClientMessage),
    Server(ServerMessage),
}

/// Messages from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "payload")]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
    /// Request to send data to peer
    Signal {
        /// Signaling data (SDP, ICE candidates, etc.)
        data: serde_json::Value,
    },
    /// PAKE message for key exchange
    Pake {
        /// PAKE message data
        message: String,
    },
    /// Ping to keep connection alive
    Ping,
}

/// Messages from server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "payload")]
#[serde(rename_all = "snake_case")]
pub enum ServerMessage {
    /// Connection established
    Connected {
        client_id: String,
        peer_count: u8,
    },
    /// Peer joined the session
    PeerJoined {
        peer_count: u8,
    },
    /// Peer left the session
    PeerLeft {
        peer_count: u8,
    },
    /// Forwarded signal from peer
    Signal {
        data: serde_json::Value,
    },
    /// Forwarded PAKE message from peer
    Pake {
        message: String,
    },
    /// Pong response
    Pong,
    /// Error message
    Error {
        code: String,
        message: String,
    },
    /// Session expired
    SessionExpired,
}

impl ServerMessage {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
