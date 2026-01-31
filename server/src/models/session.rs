use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a transfer session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// Waiting for peer to connect
    Waiting,
    /// Both peers connected, ready for transfer
    Connected,
    /// Transfer in progress
    Transferring,
    /// Transfer completed
    Completed,
    /// Session expired or closed
    Closed,
}

/// A transfer session between two peers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session ID
    pub id: Uuid,
    /// Human-readable session code (e.g., "7-crossword-puzzle")
    pub code: String,
    /// Current status
    pub status: SessionStatus,
    /// Number of connected clients (0, 1, or 2)
    pub connected_clients: u8,
    /// When the session was created
    pub created_at: DateTime<Utc>,
    /// When the session expires
    pub expires_at: DateTime<Utc>,
}

impl Session {
    /// Create a new session with the given code
    pub fn new(code: String, timeout_secs: u64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            code,
            status: SessionStatus::Waiting,
            connected_clients: 0,
            created_at: now,
            expires_at: now + chrono::Duration::seconds(timeout_secs as i64),
        }
    }

    /// Check if the session has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if the session can accept more connections
    pub fn can_connect(&self) -> bool {
        self.connected_clients < 2 && !self.is_expired() && self.status != SessionStatus::Closed
    }

    /// Add a client to the session
    pub fn add_client(&mut self) {
        if self.connected_clients < 2 {
            self.connected_clients += 1;
            if self.connected_clients == 2 {
                self.status = SessionStatus::Connected;
            }
        }
    }

    /// Remove a client from the session
    pub fn remove_client(&mut self) {
        if self.connected_clients > 0 {
            self.connected_clients -= 1;
            if self.connected_clients < 2 {
                self.status = SessionStatus::Waiting;
            }
        }
    }
}
