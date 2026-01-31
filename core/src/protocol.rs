//! Protocol definitions for SecureBeam communication
//!
//! Implements the Magic Wormhole file-transfer-protocol:
//! https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/file-transfer-protocol.md

use serde::{Deserialize, Serialize};

/// Message types for peer-to-peer communication over transit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Message {
    /// Offer to send a file or directory
    Offer(FileOffer),
    /// Answer to accept or reject an offer
    Answer(FileAnswer),
    /// Transfer completed acknowledgment
    Ack,
    /// Error occurred
    Error { message: String },
}

/// File offer message (sent by sender)
///
/// Based on Magic Wormhole file-transfer-protocol offer format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOffer {
    /// Type of transfer: "file" or "directory"
    #[serde(rename = "offer")]
    pub offer_type: OfferType,
}

/// Type of offer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfferType {
    /// Single file offer
    File(FileMetadata),
    /// Directory offer (sent as TAR)
    Directory(DirectoryMetadata),
}

/// Metadata for a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// File name
    pub filename: String,
    /// File size in bytes (after compression if compressed)
    #[serde(rename = "filesize")]
    pub file_size: u64,
    /// Original file size (before compression)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_size: Option<u64>,
    /// SHA-256 hash of the file contents (after compression if compressed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// Whether the file is GZIP compressed
    #[serde(default)]
    pub compressed: bool,
    /// MIME type of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// Metadata for a directory (transferred as TAR archive)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryMetadata {
    /// Directory name
    #[serde(rename = "dirname")]
    pub dir_name: String,
    /// Number of files in the directory
    #[serde(rename = "numfiles")]
    pub num_files: u64,
    /// Number of bytes (total uncompressed)
    #[serde(rename = "numbytes")]
    pub num_bytes: u64,
    /// TAR archive size (possibly compressed)
    #[serde(rename = "zipsize")]
    pub archive_size: u64,
    /// Whether the archive is GZIP compressed
    #[serde(default)]
    pub compressed: bool,
}

/// Answer to a file/directory offer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnswer {
    /// Answer type
    #[serde(rename = "answer")]
    pub answer_type: AnswerType,
}

/// Type of answer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerType {
    /// Accept the transfer
    #[serde(rename = "file_ack")]
    FileAck(String), // "ok"
    /// Reject the transfer
    #[serde(rename = "error")]
    Error(String),
}

impl FileOffer {
    /// Create a new file offer
    pub fn file(
        filename: String,
        file_size: u64,
        hash: Option<String>,
        compressed: bool,
        original_size: Option<u64>,
    ) -> Self {
        Self {
            offer_type: OfferType::File(FileMetadata {
                filename,
                file_size,
                original_size,
                hash,
                compressed,
                mime_type: None,
            }),
        }
    }

    /// Create a new directory offer
    pub fn directory(
        dir_name: String,
        num_files: u64,
        num_bytes: u64,
        archive_size: u64,
        compressed: bool,
    ) -> Self {
        Self {
            offer_type: OfferType::Directory(DirectoryMetadata {
                dir_name,
                num_files,
                num_bytes,
                archive_size,
                compressed,
            }),
        }
    }

    /// Get the filename or directory name
    pub fn name(&self) -> &str {
        match &self.offer_type {
            OfferType::File(f) => &f.filename,
            OfferType::Directory(d) => &d.dir_name,
        }
    }

    /// Get the transfer size (what will be sent over the wire)
    pub fn transfer_size(&self) -> u64 {
        match &self.offer_type {
            OfferType::File(f) => f.file_size,
            OfferType::Directory(d) => d.archive_size,
        }
    }

    /// Check if the transfer is compressed
    pub fn is_compressed(&self) -> bool {
        match &self.offer_type {
            OfferType::File(f) => f.compressed,
            OfferType::Directory(d) => d.compressed,
        }
    }
}

impl FileAnswer {
    /// Create an accept answer
    pub fn accept() -> Self {
        Self {
            answer_type: AnswerType::FileAck("ok".to_string()),
        }
    }

    /// Create a reject answer
    pub fn reject(reason: String) -> Self {
        Self {
            answer_type: AnswerType::Error(reason),
        }
    }

    /// Check if the answer is accepted
    pub fn is_accepted(&self) -> bool {
        matches!(&self.answer_type, AnswerType::FileAck(_))
    }
}

impl Message {
    /// Create an offer message
    pub fn offer(offer: FileOffer) -> Self {
        Message::Offer(offer)
    }

    /// Create an answer message
    pub fn answer(answer: FileAnswer) -> Self {
        Message::Answer(answer)
    }

    /// Create an error message
    pub fn error(message: String) -> Self {
        Message::Error { message }
    }

    /// Serialize to JSON bytes for transit
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| crate::Error::Protocol(format!("Serialize error: {}", e)))
    }

    /// Deserialize from JSON bytes
    pub fn from_bytes(data: &[u8]) -> crate::Result<Self> {
        serde_json::from_slice(data)
            .map_err(|e| crate::Error::Protocol(format!("Deserialize error: {}", e)))
    }
}

// Legacy types for backwards compatibility
pub type TransferRequest = FileOffer;
pub type TransferResponse = FileAnswer;

/// File chunk for transfer (legacy, kept for compatibility)
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

/// Progress update during transfer (legacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdate {
    /// Transfer ID
    pub id: String,
    /// Bytes transferred so far
    pub bytes_transferred: u64,
    /// Total bytes
    pub total_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_offer_serialization() {
        let offer = FileOffer::file(
            "test.txt".to_string(),
            1024,
            Some("abc123".to_string()),
            false,
            None,
        );

        let msg = Message::offer(offer);
        let bytes = msg.to_bytes().unwrap();
        let parsed = Message::from_bytes(&bytes).unwrap();

        match parsed {
            Message::Offer(o) => {
                assert_eq!(o.name(), "test.txt");
                assert_eq!(o.transfer_size(), 1024);
            }
            _ => panic!("Expected Offer message"),
        }
    }

    #[test]
    fn test_file_answer_accept() {
        let answer = FileAnswer::accept();
        assert!(answer.is_accepted());
    }

    #[test]
    fn test_file_answer_reject() {
        let answer = FileAnswer::reject("No space".to_string());
        assert!(!answer.is_accepted());
    }
}
