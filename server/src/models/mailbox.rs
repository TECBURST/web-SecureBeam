//! Mailbox management for the Mailbox Server
//!
//! Mailboxes store messages between peers. Each message has a phase
//! (a string identifier) and a body (binary data as hex).

use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// A message stored in a mailbox
#[derive(Debug, Clone)]
pub struct MailboxMessage {
    /// Unique message ID
    pub id: u64,
    /// The side that sent this message
    pub side: String,
    /// The phase of this message (e.g., "pake", "version", "0", "1")
    pub phase: String,
    /// The message body (hex-encoded bytes)
    pub body: String,
    /// When the message was added
    pub added_at: DateTime<Utc>,
}

/// A mailbox for message exchange between two peers
#[derive(Debug, Clone)]
pub struct Mailbox {
    /// Unique mailbox ID
    pub id: String,
    /// Application ID this mailbox belongs to
    pub appid: String,
    /// Sides that have opened this mailbox
    pub opened_by: HashSet<String>,
    /// Stored messages, indexed by (side, phase)
    messages: Vec<MailboxMessage>,
    /// Next message ID
    next_message_id: u64,
    /// When the mailbox was created
    pub created_at: DateTime<Utc>,
    /// When the mailbox expires
    pub expires_at: DateTime<Utc>,
    /// Whether the mailbox is closed
    pub closed: bool,
}

impl Mailbox {
    /// Create a new mailbox
    pub fn new(appid: String, timeout_secs: u64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            appid,
            opened_by: HashSet::new(),
            messages: Vec::new(),
            next_message_id: 1,
            created_at: now,
            expires_at: now + chrono::Duration::seconds(timeout_secs as i64),
            closed: false,
        }
    }

    /// Check if the mailbox has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Open the mailbox for a side
    pub fn open(&mut self, side: &str) -> bool {
        if self.opened_by.len() >= 2 || self.closed {
            return false;
        }
        self.opened_by.insert(side.to_string())
    }

    /// Check if a side has opened this mailbox
    pub fn is_opened_by(&self, side: &str) -> bool {
        self.opened_by.contains(side)
    }

    /// Add a message to the mailbox
    pub fn add_message(&mut self, side: &str, phase: &str, body: &str) -> MailboxMessage {
        let msg = MailboxMessage {
            id: self.next_message_id,
            side: side.to_string(),
            phase: phase.to_string(),
            body: body.to_string(),
            added_at: Utc::now(),
        };
        self.next_message_id += 1;
        self.messages.push(msg.clone());
        msg
    }

    /// Get all messages for a phase
    pub fn get_messages_for_phase(&self, phase: &str) -> Vec<&MailboxMessage> {
        self.messages.iter().filter(|m| m.phase == phase).collect()
    }

    /// Get all messages not from a specific side
    pub fn get_messages_for_peer(&self, side: &str) -> Vec<&MailboxMessage> {
        self.messages.iter().filter(|m| m.side != side).collect()
    }

    /// Get all messages after a specific ID
    pub fn get_messages_after(&self, after_id: u64) -> Vec<&MailboxMessage> {
        self.messages.iter().filter(|m| m.id > after_id).collect()
    }

    /// Get all messages
    pub fn get_all_messages(&self) -> &[MailboxMessage] {
        &self.messages
    }

    /// Close the mailbox
    pub fn close(&mut self, side: &str) {
        self.opened_by.remove(side);
        if self.opened_by.is_empty() {
            self.closed = true;
        }
    }

    /// Check if mailbox can be deleted
    pub fn can_delete(&self) -> bool {
        self.closed || self.is_expired()
    }

    /// Get the number of connected peers
    pub fn peer_count(&self) -> usize {
        self.opened_by.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mailbox_creation() {
        let mb = Mailbox::new("test-app".to_string(), 300);
        assert!(!mb.id.is_empty());
        assert_eq!(mb.appid, "test-app");
        assert!(mb.opened_by.is_empty());
        assert!(!mb.is_expired());
        assert!(!mb.closed);
    }

    #[test]
    fn test_mailbox_open() {
        let mut mb = Mailbox::new("test-app".to_string(), 300);

        assert!(mb.open("side-a"));
        assert!(mb.is_opened_by("side-a"));

        assert!(mb.open("side-b"));
        assert!(!mb.open("side-c")); // Third open should fail

        assert_eq!(mb.peer_count(), 2);
    }

    #[test]
    fn test_mailbox_messages() {
        let mut mb = Mailbox::new("test-app".to_string(), 300);
        mb.open("side-a");
        mb.open("side-b");

        let msg1 = mb.add_message("side-a", "pake", "deadbeef");
        assert_eq!(msg1.id, 1);
        assert_eq!(msg1.side, "side-a");
        assert_eq!(msg1.phase, "pake");

        let msg2 = mb.add_message("side-b", "pake", "cafebabe");
        assert_eq!(msg2.id, 2);

        let peer_messages = mb.get_messages_for_peer("side-a");
        assert_eq!(peer_messages.len(), 1);
        assert_eq!(peer_messages[0].side, "side-b");

        let after_messages = mb.get_messages_after(1);
        assert_eq!(after_messages.len(), 1);
        assert_eq!(after_messages[0].id, 2);
    }

    #[test]
    fn test_mailbox_close() {
        let mut mb = Mailbox::new("test-app".to_string(), 300);
        mb.open("side-a");
        mb.open("side-b");

        mb.close("side-a");
        assert!(!mb.closed);
        assert_eq!(mb.peer_count(), 1);

        mb.close("side-b");
        assert!(mb.closed);
        assert!(mb.can_delete());
    }
}
