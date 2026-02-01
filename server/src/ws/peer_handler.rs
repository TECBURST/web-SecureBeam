//! Simple peer-pairing WebSocket handler
//!
//! Pairs two clients by code and relays messages between them.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};

/// Shared state for peer connections
pub struct PeerState {
    /// Active peers waiting for a partner, keyed by code
    waiting: RwLock<HashMap<String, mpsc::UnboundedSender<String>>>,
    /// Connected peer pairs, keyed by code
    pairs: RwLock<HashMap<String, (mpsc::UnboundedSender<String>, mpsc::UnboundedSender<String>)>>,
}

impl PeerState {
    pub fn new() -> Self {
        Self {
            waiting: RwLock::new(HashMap::new()),
            pairs: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for PeerState {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket upgrade handler for peer pairing
pub async fn peer_ws_handler(
    ws: WebSocketUpgrade,
    Path(code): Path<String>,
    State(state): State<Arc<PeerState>>,
) -> impl IntoResponse {
    tracing::info!("Peer connection request for code: {}", code);
    ws.on_upgrade(move |socket| handle_peer_socket(socket, code, state))
}

/// Handle a peer WebSocket connection
async fn handle_peer_socket(socket: WebSocket, code: String, state: Arc<PeerState>) {
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Create channel for outgoing messages to this peer
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Check if there's already a peer waiting for this code
    let mut waiting = state.waiting.write().await;

    if let Some(partner_tx) = waiting.remove(&code) {
        // Partner found - we're the second peer
        tracing::info!("Peer joined, pairing complete for code: {}", code);

        // Store the pair
        let mut pairs = state.pairs.write().await;
        pairs.insert(code.clone(), (partner_tx.clone(), tx.clone()));
        drop(pairs);
        drop(waiting);

        // Spawn task to forward messages from channel to WebSocket
        let send_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if ws_sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });

        // Handle incoming messages - forward to partner
        while let Some(result) = ws_receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    tracing::debug!("Relaying message for code {}: {} bytes", code, text.len());
                    if partner_tx.send(text).is_err() {
                        tracing::warn!("Partner disconnected for code: {}", code);
                        break;
                    }
                }
                Ok(Message::Close(_)) => {
                    tracing::info!("Peer closed connection for code: {}", code);
                    break;
                }
                Err(e) => {
                    tracing::error!("WebSocket error for code {}: {}", code, e);
                    break;
                }
                _ => {}
            }
        }

        // Cleanup
        send_task.abort();
        let mut pairs = state.pairs.write().await;
        pairs.remove(&code);
    } else {
        // We're the first peer - wait for partner
        tracing::info!("First peer waiting for code: {}", code);
        waiting.insert(code.clone(), tx.clone());
        drop(waiting);

        // Spawn task to forward messages from channel to WebSocket
        let code_clone = code.clone();
        let send_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if ws_sender.send(Message::Text(msg)).await.is_err() {
                    tracing::warn!("Failed to send to first peer for code: {}", code_clone);
                    break;
                }
            }
        });

        // Handle incoming messages
        let state_clone = state.clone();
        while let Some(result) = ws_receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    // Try to forward to partner if connected
                    let pairs = state_clone.pairs.read().await;
                    if let Some((_peer1_tx, peer2_tx)) = pairs.get(&code) {
                        // Determine which peer we are and send to the other
                        // Since we're the first peer, send to peer2
                        if peer2_tx.send(text).is_err() {
                            tracing::warn!("Partner disconnected for code: {}", code);
                            break;
                        }
                    }
                    drop(pairs);
                }
                Ok(Message::Close(_)) => {
                    tracing::info!("First peer closed connection for code: {}", code);
                    break;
                }
                Err(e) => {
                    tracing::error!("WebSocket error for code {}: {}", code, e);
                    break;
                }
                _ => {}
            }
        }

        // Cleanup
        send_task.abort();
        let mut waiting = state.waiting.write().await;
        waiting.remove(&code);
        let mut pairs = state.pairs.write().await;
        pairs.remove(&code);
    }

    tracing::info!("Peer disconnected for code: {}", code);
}
