use std::sync::Arc;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::models::{AppState, ClientMessage, ServerMessage};

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, code, state))
}

/// Handle WebSocket connection
async fn handle_socket(socket: WebSocket, code: String, state: Arc<AppState>) {
    // Check if session exists and is valid
    match state.get_session(&code).await {
        Some(s) if !s.is_expired() && s.can_connect() => {},
        Some(_) => {
            tracing::warn!("Session {} is expired or full", code);
            return;
        }
        None => {
            tracing::warn!("Session {} not found", code);
            return;
        }
    };

    tracing::info!("WebSocket connection for session: {}", code);

    // Create channel for outgoing messages
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Register client
    let client_id = match state.register_client(&code, tx).await {
        Some(id) => id,
        None => {
            tracing::error!("Failed to register client for session: {}", code);
            return;
        }
    };

    tracing::info!("Client {} connected to session {}", client_id, code);

    // Split socket
    let (mut sender, mut receiver) = socket.split();

    // Send connected message
    let peer_count = state.get_client_count(&code).await as u8;
    let connected_msg = ServerMessage::Connected {
        client_id: client_id.to_string(),
        peer_count,
    };
    if sender
        .send(Message::Text(connected_msg.to_json().into()))
        .await
        .is_err()
    {
        tracing::error!("Failed to send connected message");
        state.unregister_client(&code, client_id).await;
        return;
    }

    // Notify other clients about new peer
    let peer_joined_msg = ServerMessage::PeerJoined { peer_count };
    state.broadcast(&code, client_id, &peer_joined_msg.to_json()).await;

    // Spawn task to forward outgoing messages
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    let state_clone = state.clone();
    let code_clone2 = code.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    handle_message(&state_clone, &code_clone2, client_id, &text).await;
                }
                Ok(Message::Close(_)) => {
                    tracing::info!("Client {} closed connection", client_id);
                    break;
                }
                Err(e) => {
                    tracing::error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => tracing::debug!("Send task ended"),
        _ = recv_task => tracing::debug!("Receive task ended"),
    }

    // Cleanup
    state.unregister_client(&code, client_id).await;

    // Notify remaining clients
    let peer_count = state.get_client_count(&code).await as u8;
    let peer_left_msg = ServerMessage::PeerLeft { peer_count };
    state.broadcast(&code, client_id, &peer_left_msg.to_json()).await;

    tracing::info!("Client {} disconnected from session {}", client_id, code);
}

/// Handle incoming WebSocket message
async fn handle_message(state: &Arc<AppState>, code: &str, client_id: Uuid, text: &str) {
    let message: ClientMessage = match serde_json::from_str(text) {
        Ok(msg) => msg,
        Err(e) => {
            tracing::warn!("Invalid message from client {}: {}", client_id, e);
            let error_msg = ServerMessage::Error {
                code: "invalid_message".to_string(),
                message: "Failed to parse message".to_string(),
            };
            state.send_to_client(code, client_id, &error_msg.to_json()).await;
            return;
        }
    };

    match message {
        ClientMessage::Signal { data } => {
            tracing::debug!("Forwarding signal from client {}", client_id);
            let forward_msg = ServerMessage::Signal { data };
            state.broadcast(code, client_id, &forward_msg.to_json()).await;
        }
        ClientMessage::Pake { message } => {
            tracing::debug!("Forwarding PAKE message from client {}", client_id);
            let forward_msg = ServerMessage::Pake { message };
            state.broadcast(code, client_id, &forward_msg.to_json()).await;
        }
        ClientMessage::Ping => {
            let pong_msg = ServerMessage::Pong;
            state.send_to_client(code, client_id, &pong_msg.to_json()).await;
        }
    }
}
