//! WebSocket handler for the Mailbox Server
//!
//! Implements the Magic Wormhole server protocol over WebSocket.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::models::{AppState, ClientMessage, NameplateInfo, ServerMessage};

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Handle a WebSocket connection
async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    // Create channel for outgoing messages
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Register client
    let client_id = state.register_client(tx.clone()).await;
    tracing::info!("Client {} connected", client_id);

    // Send welcome message
    let welcome = ServerMessage::welcome();
    if sender.send(Message::Text(welcome.to_json())).await.is_err() {
        tracing::error!("Failed to send welcome message");
        state.unregister_client(client_id).await;
        return;
    }

    // Spawn task to forward outgoing messages
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    let state_clone = state.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    if let Err(e) = handle_message(&state_clone, client_id, &text).await {
                        tracing::warn!("Error handling message: {}", e);
                        // Send error response
                        if let Some(sender) = state_clone.get_client_sender(client_id).await {
                            let error = ServerMessage::error(&e, None);
                            let _ = sender.send(error.to_json());
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    tracing::info!("Client {} closed connection", client_id);
                    break;
                }
                Err(e) => {
                    tracing::error!("WebSocket error for client {}: {}", client_id, e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => tracing::debug!("Send task ended for client {}", client_id),
        _ = recv_task => tracing::debug!("Receive task ended for client {}", client_id),
    }

    // Cleanup
    state.unregister_client(client_id).await;
    tracing::info!("Client {} disconnected", client_id);
}

/// Handle an incoming message
async fn handle_message(state: &Arc<AppState>, client_id: Uuid, text: &str) -> Result<(), String> {
    let message: ClientMessage =
        serde_json::from_str(text).map_err(|e| format!("Invalid message format: {}", e))?;

    let sender = state
        .get_client_sender(client_id)
        .await
        .ok_or("Client not found")?;

    match message {
        ClientMessage::Bind { appid, side } => {
            state
                .bind_client(client_id, appid.clone(), side.clone())
                .await;
            tracing::debug!(
                "Client {} bound to app {} as side {}",
                client_id,
                appid,
                side
            );

            // Send ack
            let ack = ServerMessage::Ack;
            let _ = sender.send(ack.to_json());
        }

        ClientMessage::List => {
            let appid = state.get_client_appid(client_id).await.ok_or("Not bound")?;

            let nameplates = state.list_nameplates(&appid).await;
            let response = ServerMessage::Nameplates {
                nameplates: nameplates
                    .into_iter()
                    .map(|id| NameplateInfo { id })
                    .collect(),
            };
            let _ = sender.send(response.to_json());
        }

        ClientMessage::Allocate => {
            let appid = state.get_client_appid(client_id).await.ok_or("Not bound")?;

            let nameplate = state.allocate_nameplate(&appid).await;
            let response = ServerMessage::Allocated { nameplate };
            let _ = sender.send(response.to_json());
        }

        ClientMessage::Claim { nameplate } => {
            let appid = state.get_client_appid(client_id).await.ok_or("Not bound")?;
            let side = state.get_client_side(client_id).await.ok_or("Not bound")?;

            let mailbox_id = state
                .claim_nameplate(&nameplate, &side, &appid)
                .await
                .ok_or("Failed to claim nameplate")?;

            let response = ServerMessage::Claimed {
                mailbox: mailbox_id,
            };
            let _ = sender.send(response.to_json());
        }

        ClientMessage::Release { nameplate } => {
            let side = state.get_client_side(client_id).await.ok_or("Not bound")?;

            if let Some(np) = nameplate {
                state.release_nameplate(&np, &side).await;
            }

            let response = ServerMessage::Released;
            let _ = sender.send(response.to_json());
        }

        ClientMessage::Open { mailbox } => {
            let side = state.get_client_side(client_id).await.ok_or("Not bound")?;

            if !state.open_mailbox(&mailbox, &side).await {
                return Err("Failed to open mailbox".to_string());
            }

            state.set_client_mailbox(client_id, mailbox.clone()).await;

            // Send ack
            let ack = ServerMessage::Ack;
            let _ = sender.send(ack.to_json());

            // Send any existing messages
            let messages = state.get_all_messages(&mailbox).await;
            for msg in messages {
                if msg.side != side {
                    let response = ServerMessage::Message {
                        side: msg.side.clone(),
                        phase: msg.phase.clone(),
                        body: msg.body.clone(),
                        id: msg.id,
                    };
                    let _ = sender.send(response.to_json());
                }
            }
        }

        ClientMessage::Add { phase, body } => {
            let side = state.get_client_side(client_id).await.ok_or("Not bound")?;

            // Get mailbox ID from client connection
            let clients = state.clients.read().await;
            let mailbox_id = clients
                .get(&client_id)
                .and_then(|c| c.mailbox_id.clone())
                .ok_or("No mailbox open")?;
            drop(clients);

            let msg = state
                .add_message(&mailbox_id, &side, &phase, &body)
                .await
                .ok_or("Failed to add message")?;

            // Send ack to sender
            let ack = ServerMessage::Ack;
            let _ = sender.send(ack.to_json());

            // Broadcast message to other clients in the mailbox
            let broadcast = ServerMessage::Message {
                side: msg.side,
                phase: msg.phase,
                body: msg.body,
                id: msg.id,
            };
            state
                .broadcast_to_mailbox(&mailbox_id, &side, &broadcast.to_json())
                .await;
        }

        ClientMessage::Close { mailbox, mood: _ } => {
            let side = state.get_client_side(client_id).await.ok_or("Not bound")?;

            // Get mailbox ID
            let mailbox_id = if let Some(mb) = mailbox {
                mb
            } else {
                let clients = state.clients.read().await;
                clients
                    .get(&client_id)
                    .and_then(|c| c.mailbox_id.clone())
                    .ok_or("No mailbox to close")?
            };

            state.close_mailbox(&mailbox_id, &side).await;

            let response = ServerMessage::Closed;
            let _ = sender.send(response.to_json());
        }

        ClientMessage::Ping { ping } => {
            let response = ServerMessage::Pong { pong: ping };
            let _ = sender.send(response.to_json());
        }
    }

    Ok(())
}
