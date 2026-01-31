//! Protocol definitions for SecureBeam communication

use serde::{Deserialize, Serialize};

/// Message types for client-server and peer-to-peer communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Message {
    /// Request to initiate a transfer
    TransferRequest(TransferRequest),
    /// Response to a transfer request
    TransferResponse(TransferResponse),
    /// File chunk data
    FileChunk(FileChunk),
    /// Transfer progress update
    Progress(ProgressUpdate),
    /// Transfer completed
    Complete,
    /// Error occurred
    Error { message: String },
}

/// Request to transfer a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    /// Unique transfer ID
    pub id: String,
    /// File name
    pub filename: String,
    /// File size in bytes
    pub size: u64,
    /// File MIME type (if known)
    pub mime_type: Option<String>,
    /// SHA-256 hash of the file
    pub hash: String,
}

/// Response to a transfer request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResponse {
    /// Transfer ID from the request
    pub id: String,
    /// Whether the transfer is accepted
    pub accepted: bool,
    /// Reason for rejection (if not accepted)
    pub reason: Option<String>,
}

/// File chunk for transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunk {
    /// Transfer ID
    pub id: String,
    /// Chunk sequence number
    pub sequence: u64,
    /// Chunk data (base64 encoded)
    pub data: String,
    /// Whether this is the last chunk
    pub is_last: bool,
}

/// Progress update during transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdate {
    /// Transfer ID
    pub id: String,
    /// Bytes transferred so far
    pub bytes_transferred: u64,
    /// Total bytes
    pub total_bytes: u64,
}

impl TransferRequest {
    /// Create a new transfer request
    pub fn new(filename: String, size: u64, hash: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            filename,
            size,
            mime_type: None,
            hash,
        }
    }

    /// Set the MIME type
    pub fn with_mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = Some(mime_type);
        self
    }
}
