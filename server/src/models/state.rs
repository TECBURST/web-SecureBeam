use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use super::Session;

/// Sender for WebSocket messages
pub type WsSender = mpsc::UnboundedSender<String>;

/// Connected client information
#[derive(Debug)]
pub struct ConnectedClient {
    pub id: Uuid,
    pub sender: WsSender,
}

/// Shared application state
#[derive(Debug)]
pub struct AppState {
    /// Active sessions indexed by code
    pub sessions: RwLock<HashMap<String, Session>>,
    /// Connected clients per session code
    pub clients: RwLock<HashMap<String, Vec<ConnectedClient>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            clients: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new session and return its code
    pub async fn create_session(&self, code: String, timeout_secs: u64) -> Session {
        let session = Session::new(code.clone(), timeout_secs);
        let mut sessions = self.sessions.write().await;
        sessions.insert(code.clone(), session.clone());

        let mut clients = self.clients.write().await;
        clients.insert(code, Vec::new());

        session
    }

    /// Get a session by code
    pub async fn get_session(&self, code: &str) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(code).cloned()
    }

    /// Register a client connection
    pub async fn register_client(&self, code: &str, sender: WsSender) -> Option<Uuid> {
        let client_id = Uuid::new_v4();

        // Update session
        {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(code) {
                if !session.can_connect() {
                    return None;
                }
                session.add_client();
            } else {
                return None;
            }
        }

        // Add client
        {
            let mut clients = self.clients.write().await;
            if let Some(client_list) = clients.get_mut(code) {
                client_list.push(ConnectedClient {
                    id: client_id,
                    sender,
                });
            }
        }

        Some(client_id)
    }

    /// Unregister a client connection
    pub async fn unregister_client(&self, code: &str, client_id: Uuid) {
        // Remove client
        {
            let mut clients = self.clients.write().await;
            if let Some(client_list) = clients.get_mut(code) {
                client_list.retain(|c| c.id != client_id);
            }
        }

        // Update session
        {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(code) {
                session.remove_client();
            }
        }
    }

    /// Broadcast a message to all clients in a session except the sender
    pub async fn broadcast(&self, code: &str, sender_id: Uuid, message: &str) {
        let clients = self.clients.read().await;
        if let Some(client_list) = clients.get(code) {
            for client in client_list {
                if client.id != sender_id {
                    let _ = client.sender.send(message.to_string());
                }
            }
        }
    }

    /// Send a message to a specific client
    pub async fn send_to_client(&self, code: &str, client_id: Uuid, message: &str) {
        let clients = self.clients.read().await;
        if let Some(client_list) = clients.get(code) {
            for client in client_list {
                if client.id == client_id {
                    let _ = client.sender.send(message.to_string());
                    break;
                }
            }
        }
    }

    /// Get the number of connected clients for a session
    pub async fn get_client_count(&self, code: &str) -> usize {
        let clients = self.clients.read().await;
        clients.get(code).map(|c| c.len()).unwrap_or(0)
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired(&self) {
        let mut sessions = self.sessions.write().await;
        let mut clients = self.clients.write().await;

        let expired: Vec<String> = sessions
            .iter()
            .filter(|(_, s)| s.is_expired())
            .map(|(code, _)| code.clone())
            .collect();

        for code in expired {
            sessions.remove(&code);
            clients.remove(&code);
            tracing::info!("Cleaned up expired session: {}", code);
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
