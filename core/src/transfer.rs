//! File transfer logic

use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

use crate::{Error, Result};
use crate::protocol::{FileChunk, TransferRequest};
use crate::crypto::sha256_hex;

/// Default chunk size for file transfers (64 KB)
pub const DEFAULT_CHUNK_SIZE: usize = 64 * 1024;

/// Transfer progress information
#[derive(Debug, Clone)]
pub struct TransferProgress {
    /// Bytes transferred so far
    pub bytes_transferred: u64,
    /// Total bytes to transfer
    pub total_bytes: u64,
    /// Current transfer speed in bytes per second
    pub speed_bps: f64,
}

impl TransferProgress {
    pub fn new(total_bytes: u64) -> Self {
        Self {
            bytes_transferred: 0,
            total_bytes,
            speed_bps: 0.0,
        }
    }

    /// Get progress as a percentage
    pub fn percentage(&self) -> f64 {
        if self.total_bytes == 0 {
            100.0
        } else {
            (self.bytes_transferred as f64 / self.total_bytes as f64) * 100.0
        }
    }
}

/// File transfer operations
pub struct FileTransfer {
    chunk_size: usize,
}

impl FileTransfer {
    /// Create a new file transfer with default chunk size
    pub fn new() -> Self {
        Self {
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }

    /// Create a new file transfer with custom chunk size
    pub fn with_chunk_size(chunk_size: usize) -> Self {
        Self { chunk_size }
    }

    /// Prepare a file for sending
    pub async fn prepare_send<P: AsRef<Path>>(&self, path: P) -> Result<TransferRequest> {
        let path = path.as_ref();

        let metadata = tokio::fs::metadata(path).await?;
        let size = metadata.len();

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::Transfer("Invalid filename".to_string()))?
            .to_string();

        // Compute file hash
        let hash = self.compute_file_hash(path).await?;

        Ok(TransferRequest::new(filename, size, hash))
    }

    /// Compute SHA-256 hash of a file
    pub async fn compute_file_hash<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let file = File::open(path).await?;
        let mut reader = BufReader::new(file);
        let mut hasher_data = Vec::new();

        let mut buffer = vec![0u8; self.chunk_size];
        loop {
            let bytes_read = reader.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            hasher_data.extend_from_slice(&buffer[..bytes_read]);
        }

        Ok(sha256_hex(&hasher_data))
    }

    /// Read file chunks for sending
    pub async fn read_chunks<P, F>(
        &self,
        path: P,
        transfer_id: String,
        mut on_chunk: F,
    ) -> Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(FileChunk) -> Result<()>,
    {
        let file = File::open(path).await?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0u8; self.chunk_size];
        let mut sequence = 0u64;

        loop {
            let bytes_read = reader.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }

            let data = base64_encode(&buffer[..bytes_read]);
            let is_last = bytes_read < self.chunk_size;

            let chunk = FileChunk {
                id: transfer_id.clone(),
                sequence,
                data,
                is_last,
            };

            on_chunk(chunk)?;
            sequence += 1;

            if is_last {
                break;
            }
        }

        Ok(())
    }

    /// Write received chunks to file
    pub async fn write_chunk<P: AsRef<Path>>(
        &self,
        path: P,
        chunk: &FileChunk,
        append: bool,
    ) -> Result<usize> {
        let data = base64_decode(&chunk.data)?;

        let file = if append {
            tokio::fs::OpenOptions::new()
                .append(true)
                .open(path)
                .await?
        } else {
            File::create(path).await?
        };

        let mut writer = BufWriter::new(file);
        writer.write_all(&data).await?;
        writer.flush().await?;

        Ok(data.len())
    }

    /// Verify received file hash
    pub async fn verify_hash<P: AsRef<Path>>(&self, path: P, expected_hash: &str) -> Result<bool> {
        let actual_hash = self.compute_file_hash(path).await?;
        Ok(actual_hash == expected_hash)
    }
}

impl Default for FileTransfer {
    fn default() -> Self {
        Self::new()
    }
}

// Simple base64 encoding/decoding
fn base64_encode(data: &[u8]) -> String {
    use base64_impl::*;
    encode(data)
}

fn base64_decode(data: &str) -> Result<Vec<u8>> {
    use base64_impl::*;
    decode(data).map_err(|e| Error::Transfer(format!("Base64 decode error: {}", e)))
}

mod base64_impl {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    pub fn encode(data: &[u8]) -> String {
        let mut result = String::new();
        let mut i = 0;

        while i < data.len() {
            let b0 = data[i] as u32;
            let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
            let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };

            let triple = (b0 << 16) | (b1 << 8) | b2;

            result.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
            result.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);

            if i + 1 < data.len() {
                result.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }

            if i + 2 < data.len() {
                result.push(ALPHABET[(triple & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }

            i += 3;
        }

        result
    }

    pub fn decode(data: &str) -> std::result::Result<Vec<u8>, &'static str> {
        let data = data.trim_end_matches('=');
        let mut result = Vec::new();

        let mut buffer = 0u32;
        let mut bits = 0;

        for c in data.chars() {
            let value = match c {
                'A'..='Z' => c as u32 - 'A' as u32,
                'a'..='z' => c as u32 - 'a' as u32 + 26,
                '0'..='9' => c as u32 - '0' as u32 + 52,
                '+' => 62,
                '/' => 63,
                _ => return Err("Invalid base64 character"),
            };

            buffer = (buffer << 6) | value;
            bits += 6;

            if bits >= 8 {
                bits -= 8;
                result.push((buffer >> bits) as u8);
                buffer &= (1 << bits) - 1;
            }
        }

        Ok(result)
    }
}
