//! Transit connection abstraction
//!
//! Provides a unified interface for encrypted transit connections,
//! whether they are direct or via relay.

use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::{Error, Result};
use crate::crypto::SecretBox;

/// Role in the transit connection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitRole {
    /// The sender initiates and sends files
    Sender,
    /// The receiver accepts and receives files
    Receiver,
}

impl TransitRole {
    /// Get the handshake string for this role
    pub fn handshake_string(&self, transit_key_hash: &str) -> String {
        match self {
            TransitRole::Sender => format!("transit sender {} ready\n\n", transit_key_hash),
            TransitRole::Receiver => format!("transit receiver {} ready\n\n", transit_key_hash),
        }
    }

    /// Get the expected peer handshake string
    pub fn expected_peer_handshake(&self, transit_key_hash: &str) -> String {
        match self {
            TransitRole::Sender => format!("transit receiver {} ready\n\n", transit_key_hash),
            TransitRole::Receiver => format!("transit sender {} ready\n\n", transit_key_hash),
        }
    }
}

/// An encrypted transit connection
pub struct TransitConnection {
    stream: TcpStream,
    secretbox: SecretBox,
    role: TransitRole,
    /// Sequence number for sender
    send_seq: u64,
    /// Sequence number for receiver
    recv_seq: u64,
}

impl TransitConnection {
    /// Create a new transit connection from an established TCP stream
    pub fn new(stream: TcpStream, transit_key: &[u8], role: TransitRole) -> Result<Self> {
        let secretbox = SecretBox::new(transit_key)?;
        Ok(Self {
            stream,
            secretbox,
            role,
            send_seq: 0,
            recv_seq: 0,
        })
    }

    /// Get the role
    pub fn role(&self) -> TransitRole {
        self.role
    }

    /// Send encrypted data
    pub async fn send(&mut self, data: &[u8]) -> Result<()> {
        // Encrypt the data
        let encrypted = self.secretbox.seal(data)?;

        // Send length prefix (4 bytes, big-endian)
        let len = encrypted.len() as u32;
        self.stream.write_all(&len.to_be_bytes()).await?;

        // Send encrypted data
        self.stream.write_all(&encrypted).await?;
        self.stream.flush().await?;

        self.send_seq += 1;
        Ok(())
    }

    /// Receive and decrypt data
    pub async fn receive(&mut self) -> Result<Vec<u8>> {
        // Read length prefix
        let mut len_buf = [0u8; 4];
        self.stream.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf) as usize;

        // Sanity check
        if len > 10 * 1024 * 1024 {
            return Err(Error::Protocol("Message too large".to_string()));
        }

        // Read encrypted data
        let mut encrypted = vec![0u8; len];
        self.stream.read_exact(&mut encrypted).await?;

        // Decrypt
        let decrypted = self.secretbox.open(&encrypted)?;

        self.recv_seq += 1;
        Ok(decrypted)
    }

    /// Send a file in chunks
    pub async fn send_file<R: AsyncRead + Unpin>(
        &mut self,
        reader: &mut R,
        total_size: u64,
        chunk_size: usize,
        mut progress_callback: impl FnMut(u64),
    ) -> Result<()> {
        let mut sent = 0u64;
        let mut buffer = vec![0u8; chunk_size];

        while sent < total_size {
            let to_read = std::cmp::min(chunk_size as u64, total_size - sent) as usize;
            let n = reader.read(&mut buffer[..to_read]).await?;

            if n == 0 {
                break;
            }

            self.send(&buffer[..n]).await?;
            sent += n as u64;
            progress_callback(sent);
        }

        // Send empty chunk to signal end
        self.send(&[]).await?;

        Ok(())
    }

    /// Receive a file in chunks
    pub async fn receive_file<W: AsyncWrite + Unpin>(
        &mut self,
        writer: &mut W,
        expected_size: u64,
        mut progress_callback: impl FnMut(u64),
    ) -> Result<()> {
        let mut received = 0u64;

        loop {
            let chunk = self.receive().await?;

            // Empty chunk signals end
            if chunk.is_empty() {
                break;
            }

            writer.write_all(&chunk).await?;
            received += chunk.len() as u64;
            progress_callback(received);
        }

        writer.flush().await?;

        if received != expected_size {
            return Err(Error::Transfer(format!(
                "Size mismatch: expected {}, got {}",
                expected_size, received
            )));
        }

        Ok(())
    }

    /// Close the connection
    pub async fn close(mut self) -> Result<()> {
        self.stream.shutdown().await?;
        Ok(())
    }
}

/// Perform the transit handshake
pub async fn perform_handshake(
    stream: &mut TcpStream,
    role: TransitRole,
    transit_key: &[u8],
) -> Result<()> {
    use sha2::{Sha256, Digest};

    // Compute transit key hash
    let mut hasher = Sha256::new();
    hasher.update(transit_key);
    let hash = hasher.finalize();
    let key_hash = hex::encode(&hash[..16]);

    // Send our handshake
    let our_handshake = role.handshake_string(&key_hash);
    stream.write_all(our_handshake.as_bytes()).await?;

    // Read peer's handshake
    let expected = role.expected_peer_handshake(&key_hash);
    let mut buf = vec![0u8; expected.len()];
    stream.read_exact(&mut buf).await?;

    let received = String::from_utf8_lossy(&buf);
    if received != expected {
        return Err(Error::Protocol(format!(
            "Invalid handshake: expected '{}', got '{}'",
            expected.trim(),
            received.trim()
        )));
    }

    tracing::debug!("Transit handshake successful");
    Ok(())
}
