//! Nameplate management for the Mailbox Server
//!
//! Nameplates are short numeric identifiers that point to mailboxes.
//! They are the visible part of wormhole codes (e.g., the "4" in "4-purple-sausages").

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use std::collections::HashSet;

/// A nameplate that points to a mailbox
#[derive(Debug, Clone)]
pub struct Nameplate {
    /// The nameplate ID (numeric string like "4", "127")
    pub id: String,
    /// The mailbox ID this nameplate points to
    pub mailbox_id: String,
    /// Sides that have claimed this nameplate
    pub claimed_by: HashSet<String>,
    /// When the nameplate was created
    pub created_at: DateTime<Utc>,
    /// When the nameplate expires
    pub expires_at: DateTime<Utc>,
}

impl Nameplate {
    /// Create a new nameplate
    pub fn new(id: String, mailbox_id: String, timeout_secs: u64) -> Self {
        let now = Utc::now();
        Self {
            id,
            mailbox_id,
            claimed_by: HashSet::new(),
            created_at: now,
            expires_at: now + chrono::Duration::seconds(timeout_secs as i64),
        }
    }

    /// Check if the nameplate has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if a side has claimed this nameplate
    pub fn is_claimed_by(&self, side: &str) -> bool {
        self.claimed_by.contains(side)
    }

    /// Claim the nameplate for a side
    pub fn claim(&mut self, side: &str) -> bool {
        if self.claimed_by.len() >= 2 {
            return false; // Already fully claimed
        }
        self.claimed_by.insert(side.to_string())
    }

    /// Release the nameplate for a side
    pub fn release(&mut self, side: &str) {
        self.claimed_by.remove(side);
    }

    /// Check if the nameplate can be released (no more claims)
    pub fn can_release(&self) -> bool {
        self.claimed_by.is_empty()
    }
}

/// Generate a random nameplate ID
pub fn generate_nameplate_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    // Generate a number between 1 and 999
    rng.gen_range(1..1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nameplate_creation() {
        let np = Nameplate::new("4".to_string(), "mailbox-123".to_string(), 300);
        assert_eq!(np.id, "4");
        assert_eq!(np.mailbox_id, "mailbox-123");
        assert!(np.claimed_by.is_empty());
        assert!(!np.is_expired());
    }

    #[test]
    fn test_nameplate_claim() {
        let mut np = Nameplate::new("4".to_string(), "mailbox-123".to_string(), 300);

        assert!(np.claim("side-a"));
        assert!(np.is_claimed_by("side-a"));
        assert!(!np.is_claimed_by("side-b"));

        assert!(np.claim("side-b"));
        assert!(!np.claim("side-c")); // Third claim should fail

        np.release("side-a");
        assert!(!np.is_claimed_by("side-a"));
    }

    #[test]
    fn test_generate_nameplate_id() {
        let id = generate_nameplate_id();
        let num: u32 = id.parse().unwrap();
        assert!(num >= 1 && num < 1000);
    }
}
