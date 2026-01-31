//! Application state for the Mailbox Server
//!
//! Manages nameplates, mailboxes, and client connections.

#![allow(dead_code)]

use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use super::{generate_nameplate_id, Mailbox, MailboxMessage, Nameplate};

/// Sender for WebSocket messages
pub type WsSender = mpsc::UnboundedSender<String>;

/// Information about a connected client
#[derive(Debug)]
pub struct ClientConnection {
    pub id: Uuid,
    pub sender: WsSender,
    pub appid: Option<String>,
    pub side: Option<String>,
    pub mailbox_id: Option<String>,
    pub last_seen_message_id: u64,
}

impl ClientConnection {
    pub fn new(id: Uuid, sender: WsSender) -> Self {
        Self {
            id,
            sender,
            appid: None,
            side: None,
            mailbox_id: None,
            last_seen_message_id: 0,
        }
    }

    /// Check if client is bound (has appid and side)
    pub fn is_bound(&self) -> bool {
        self.appid.is_some() && self.side.is_some()
    }

    /// Bind the client to an app and side
    pub fn bind(&mut self, appid: String, side: String) {
        self.appid = Some(appid);
        self.side = Some(side);
    }
}

/// Shared application state
pub struct AppState {
    /// Nameplates indexed by ID
    pub nameplates: RwLock<HashMap<String, Nameplate>>,
    /// Mailboxes indexed by ID
    pub mailboxes: RwLock<HashMap<String, Mailbox>>,
    /// Connected clients indexed by UUID
    pub clients: RwLock<HashMap<Uuid, ClientConnection>>,
    /// Default timeout in seconds
    pub timeout_secs: u64,
}

impl AppState {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            nameplates: RwLock::new(HashMap::new()),
            mailboxes: RwLock::new(HashMap::new()),
            clients: RwLock::new(HashMap::new()),
            timeout_secs,
        }
    }

    // === Client Management ===

    /// Register a new client connection
    pub async fn register_client(&self, sender: WsSender) -> Uuid {
        let id = Uuid::new_v4();
        let conn = ClientConnection::new(id, sender);

        let mut clients = self.clients.write().await;
        clients.insert(id, conn);

        tracing::debug!("Client {} registered", id);
        id
    }

    /// Unregister a client connection
    pub async fn unregister_client(&self, client_id: Uuid) {
        let mut clients = self.clients.write().await;
        if let Some(conn) = clients.remove(&client_id) {
            tracing::debug!("Client {} unregistered", client_id);

            // Close mailbox if client had one open
            if let (Some(side), Some(mailbox_id)) = (&conn.side, &conn.mailbox_id) {
                drop(clients); // Release lock before calling close_mailbox
                self.close_mailbox(mailbox_id, side).await;
            }
        }
    }

    /// Get a client's sender
    pub async fn get_client_sender(&self, client_id: Uuid) -> Option<WsSender> {
        let clients = self.clients.read().await;
        clients.get(&client_id).map(|c| c.sender.clone())
    }

    /// Bind a client to an app and side
    pub async fn bind_client(&self, client_id: Uuid, appid: String, side: String) -> bool {
        let mut clients = self.clients.write().await;
        if let Some(conn) = clients.get_mut(&client_id) {
            conn.bind(appid, side);
            true
        } else {
            false
        }
    }

    /// Get client's side
    pub async fn get_client_side(&self, client_id: Uuid) -> Option<String> {
        let clients = self.clients.read().await;
        clients.get(&client_id).and_then(|c| c.side.clone())
    }

    /// Get client's appid
    pub async fn get_client_appid(&self, client_id: Uuid) -> Option<String> {
        let clients = self.clients.read().await;
        clients.get(&client_id).and_then(|c| c.appid.clone())
    }

    /// Set client's mailbox
    pub async fn set_client_mailbox(&self, client_id: Uuid, mailbox_id: String) {
        let mut clients = self.clients.write().await;
        if let Some(conn) = clients.get_mut(&client_id) {
            conn.mailbox_id = Some(mailbox_id);
        }
    }

    /// Update client's last seen message ID
    pub async fn update_client_last_seen(&self, client_id: Uuid, message_id: u64) {
        let mut clients = self.clients.write().await;
        if let Some(conn) = clients.get_mut(&client_id) {
            conn.last_seen_message_id = message_id;
        }
    }

    // === Nameplate Management ===

    /// Allocate a new nameplate
    pub async fn allocate_nameplate(&self, appid: &str) -> String {
        let mut nameplates = self.nameplates.write().await;
        let mut mailboxes = self.mailboxes.write().await;

        // Generate unique nameplate ID
        let nameplate_id = loop {
            let candidate = generate_nameplate_id();
            if !nameplates.contains_key(&candidate) {
                break candidate;
            }
        };

        // Create mailbox
        let mailbox = Mailbox::new(appid.to_string(), self.timeout_secs);
        let mailbox_id = mailbox.id.clone();

        // Create nameplate
        let nameplate = Nameplate::new(nameplate_id.clone(), mailbox_id.clone(), self.timeout_secs);

        mailboxes.insert(mailbox_id, mailbox);
        nameplates.insert(nameplate_id.clone(), nameplate);

        tracing::info!("Allocated nameplate: {}", nameplate_id);
        nameplate_id
    }

    /// Claim a nameplate
    pub async fn claim_nameplate(
        &self,
        nameplate_id: &str,
        side: &str,
        appid: &str,
    ) -> Option<String> {
        let mut nameplates = self.nameplates.write().await;

        // Check if nameplate exists
        if let Some(np) = nameplates.get_mut(nameplate_id) {
            if np.is_expired() {
                return None;
            }
            np.claim(side);
            return Some(np.mailbox_id.clone());
        }

        // Nameplate doesn't exist - create it (for receiver who enters code)
        let mut mailboxes = self.mailboxes.write().await;

        let mailbox = Mailbox::new(appid.to_string(), self.timeout_secs);
        let mailbox_id = mailbox.id.clone();

        let mut nameplate = Nameplate::new(
            nameplate_id.to_string(),
            mailbox_id.clone(),
            self.timeout_secs,
        );
        nameplate.claim(side);

        mailboxes.insert(mailbox_id.clone(), mailbox);
        nameplates.insert(nameplate_id.to_string(), nameplate);

        tracing::info!("Created and claimed nameplate: {}", nameplate_id);
        Some(mailbox_id)
    }

    /// Release a nameplate
    pub async fn release_nameplate(&self, nameplate_id: &str, side: &str) -> bool {
        let mut nameplates = self.nameplates.write().await;

        if let Some(np) = nameplates.get_mut(nameplate_id) {
            np.release(side);
            if np.can_release() {
                nameplates.remove(nameplate_id);
                tracing::info!("Released and removed nameplate: {}", nameplate_id);
            }
            return true;
        }
        false
    }

    /// List all nameplates for an app
    pub async fn list_nameplates(&self, _appid: &str) -> Vec<String> {
        let nameplates = self.nameplates.read().await;
        nameplates.keys().cloned().collect()
    }

    // === Mailbox Management ===

    /// Open a mailbox
    pub async fn open_mailbox(&self, mailbox_id: &str, side: &str) -> bool {
        let mut mailboxes = self.mailboxes.write().await;

        if let Some(mb) = mailboxes.get_mut(mailbox_id) {
            if mb.is_expired() || mb.closed {
                return false;
            }
            return mb.open(side);
        }
        false
    }

    /// Add a message to a mailbox
    pub async fn add_message(
        &self,
        mailbox_id: &str,
        side: &str,
        phase: &str,
        body: &str,
    ) -> Option<MailboxMessage> {
        let mut mailboxes = self.mailboxes.write().await;

        if let Some(mb) = mailboxes.get_mut(mailbox_id) {
            if mb.closed {
                return None;
            }
            return Some(mb.add_message(side, phase, body));
        }
        None
    }

    /// Get messages from a mailbox for a specific side
    pub async fn get_messages(
        &self,
        mailbox_id: &str,
        for_side: &str,
        after_id: u64,
    ) -> Vec<MailboxMessage> {
        let mailboxes = self.mailboxes.read().await;

        if let Some(mb) = mailboxes.get(mailbox_id) {
            return mb
                .get_messages_after(after_id)
                .into_iter()
                .filter(|m| m.side != for_side)
                .cloned()
                .collect();
        }
        Vec::new()
    }

    /// Get all messages from a mailbox
    pub async fn get_all_messages(&self, mailbox_id: &str) -> Vec<MailboxMessage> {
        let mailboxes = self.mailboxes.read().await;

        if let Some(mb) = mailboxes.get(mailbox_id) {
            return mb.get_all_messages().to_vec();
        }
        Vec::new()
    }

    /// Close a mailbox
    pub async fn close_mailbox(&self, mailbox_id: &str, side: &str) -> bool {
        let mut mailboxes = self.mailboxes.write().await;

        if let Some(mb) = mailboxes.get_mut(mailbox_id) {
            mb.close(side);
            if mb.can_delete() {
                mailboxes.remove(mailbox_id);
                tracing::info!("Closed and removed mailbox: {}", mailbox_id);
            }
            return true;
        }
        false
    }

    /// Broadcast a message to all clients in a mailbox except the sender
    pub async fn broadcast_to_mailbox(&self, mailbox_id: &str, sender_side: &str, message: &str) {
        let clients = self.clients.read().await;

        for conn in clients.values() {
            if conn.mailbox_id.as_deref() == Some(mailbox_id)
                && conn.side.as_deref() != Some(sender_side)
            {
                let _ = conn.sender.send(message.to_string());
            }
        }
    }

    // === Cleanup ===

    /// Clean up expired nameplates and mailboxes
    pub async fn cleanup_expired(&self) {
        // Clean up nameplates
        {
            let mut nameplates = self.nameplates.write().await;
            let expired: Vec<String> = nameplates
                .iter()
                .filter(|(_, np)| np.is_expired())
                .map(|(id, _)| id.clone())
                .collect();

            for id in expired {
                nameplates.remove(&id);
                tracing::debug!("Cleaned up expired nameplate: {}", id);
            }
        }

        // Clean up mailboxes
        {
            let mut mailboxes = self.mailboxes.write().await;
            let expired: Vec<String> = mailboxes
                .iter()
                .filter(|(_, mb)| mb.can_delete())
                .map(|(id, _)| id.clone())
                .collect();

            for id in expired {
                mailboxes.remove(&id);
                tracing::debug!("Cleaned up expired mailbox: {}", id);
            }
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(300) // 5 minutes default
    }
}
