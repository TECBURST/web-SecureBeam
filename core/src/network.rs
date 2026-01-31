//! Network abstractions for SecureBeam

use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{Error, Result};

/// Signaling client for connecting to the SecureBeam server
pub struct SignalingClient {
    server_url: String,
}

impl SignalingClient {
    /// Create a new signaling client
    pub fn new(server_url: &str) -> Self {
        Self {
            server_url: server_url.to_string(),
        }
    }

    /// Create a new session and return the code
    pub async fn create_session(&self) -> Result<SessionInfo> {
        let url = format!("{}/api/sessions", self.server_url);

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        if !response.status().is_success() {
            return Err(Error::Connection("Failed to create session".to_string()));
        }

        let info: SessionInfo = response
            .json()
            .await
            .map_err(|e| Error::Protocol(e.to_string()))?;

        Ok(info)
    }

    /// Connect to a session using WebSocket
    pub async fn connect(&self, code: &str) -> Result<SessionConnection> {
        let ws_url = self
            .server_url
            .replace("http://", "ws://")
            .replace("https://", "wss://");
        let url = format!("{}/ws/{}", ws_url, code);

        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;

        let (write, read) = ws_stream.split();

        let (tx, rx) = mpsc::unbounded_channel();

        Ok(SessionConnection {
            write: Box::new(write),
            read: Box::new(read),
            sender: tx,
            receiver: rx,
        })
    }
}

/// Session information returned by the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub code: String,
    pub expires_at: String,
}

/// Active connection to a session
pub struct SessionConnection {
    write: Box<
        dyn futures::Sink<Message, Error = tokio_tungstenite::tungstenite::Error> + Unpin + Send,
    >,
    read: Box<
        dyn futures::Stream<
                Item = std::result::Result<Message, tokio_tungstenite::tungstenite::Error>,
            > + Unpin
            + Send,
    >,
    sender: mpsc::UnboundedSender<String>,
    receiver: mpsc::UnboundedReceiver<String>,
}

impl SessionConnection {
    /// Send a message to the session
    pub async fn send(&mut self, message: &str) -> Result<()> {
        self.write
            .send(Message::Text(message.to_string().into()))
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(())
    }

    /// Receive a message from the session
    pub async fn receive(&mut self) -> Result<Option<String>> {
        match self.read.next().await {
            Some(Ok(Message::Text(text))) => Ok(Some(text.to_string())),
            Some(Ok(Message::Close(_))) => Ok(None),
            Some(Err(e)) => Err(Error::Connection(e.to_string())),
            _ => Ok(None),
        }
    }

    /// Close the connection
    pub async fn close(&mut self) -> Result<()> {
        self.write
            .send(Message::Close(None))
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(())
    }
}
