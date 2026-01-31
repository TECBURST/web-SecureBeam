//! File transfer logic with compression and directory support
//!
//! Implements the Magic Wormhole file-transfer-protocol:
//! - Single file transfers (optionally GZIP compressed)
//! - Directory transfers (TAR archive, optionally GZIP compressed)
//! - Hash verification
//! - Progress reporting
//!
//! Security features:
//! - Path traversal protection in TAR extraction
//! - Constant-time hash comparison
//! - Size limits to prevent DoS

use std::io::{Read, Write};
use std::path::Path;
use sha2::{Sha256, Digest};
use subtle::ConstantTimeEq;

use crate::{Error, Result};
use crate::protocol::FileOffer;
use crate::transit::TransitConnection;

/// Default chunk size for file transfers (64 KB)
pub const DEFAULT_CHUNK_SIZE: usize = 64 * 1024;

/// Maximum file size for transfers (10 GB)
pub const MAX_TRANSFER_SIZE: u64 = 10 * 1024 * 1024 * 1024;

/// Maximum number of files in a directory transfer
pub const MAX_DIRECTORY_FILES: u64 = 100_000;

/// File extensions that are already compressed (don't GZIP these)
const COMPRESSED_EXTENSIONS: &[&str] = &[
    "zip", "gz", "bz2", "xz", "7z", "rar",
    "jpg", "jpeg", "png", "gif", "webp", "avif",
    "mp3", "mp4", "mkv", "avi", "mov", "webm",
    "pdf", "docx", "xlsx", "pptx",
];

/// Transfer progress information
#[derive(Debug, Clone)]
pub struct TransferProgress {
    /// Bytes transferred so far
    pub bytes_transferred: u64,
    /// Total bytes to transfer
    pub total_bytes: u64,
    /// Current transfer speed in bytes per second
    pub speed_bps: f64,
    /// Estimated time remaining in seconds
    pub eta_seconds: Option<f64>,
}

impl TransferProgress {
    pub fn new(total_bytes: u64) -> Self {
        Self {
            bytes_transferred: 0,
            total_bytes,
            speed_bps: 0.0,
            eta_seconds: None,
        }
    }

    /// Get progress as a percentage (0-100)
    pub fn percentage(&self) -> f64 {
        if self.total_bytes == 0 {
            100.0
        } else {
            (self.bytes_transferred as f64 / self.total_bytes as f64) * 100.0
        }
    }

    /// Check if transfer is complete
    pub fn is_complete(&self) -> bool {
        self.bytes_transferred >= self.total_bytes
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

    // ==================== SENDER SIDE ====================

    /// Prepare an offer for a file
    pub async fn prepare_file_offer<P: AsRef<Path>>(&self, path: P) -> Result<FileOffer> {
        let path = path.as_ref();

        if !path.is_file() {
            return Err(Error::Transfer("Path is not a file".to_string()));
        }

        let metadata = tokio::fs::metadata(path).await?;
        let original_size = metadata.len();

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::Transfer("Invalid filename".to_string()))?
            .to_string();

        // Decide whether to compress
        let should_compress = self.should_compress_file(path);

        // Compute hash of original file
        let hash = self.compute_file_hash(path).await?;

        // If compressing, we need to get the compressed size
        let (file_size, compressed) = if should_compress {
            let compressed_data = self.compress_file(path).await?;
            (compressed_data.len() as u64, true)
        } else {
            (original_size, false)
        };

        let original_size_opt = if compressed {
            Some(original_size)
        } else {
            None
        };

        Ok(FileOffer::file(filename, file_size, Some(hash), compressed, original_size_opt))
    }

    /// Prepare an offer for a directory
    pub async fn prepare_directory_offer<P: AsRef<Path>>(&self, path: P) -> Result<FileOffer> {
        let path = path.as_ref();

        if !path.is_dir() {
            return Err(Error::Transfer("Path is not a directory".to_string()));
        }

        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::Transfer("Invalid directory name".to_string()))?
            .to_string();

        // Count files and total size
        let (num_files, num_bytes) = self.count_directory_contents(path).await?;

        // Create TAR archive (compressed)
        let archive_data = self.create_tar_archive(path, true).await?;
        let archive_size = archive_data.len() as u64;

        Ok(FileOffer::directory(dir_name, num_files, num_bytes, archive_size, true))
    }

    /// Send a file over a transit connection
    pub async fn send_file<P, F>(
        &self,
        conn: &mut TransitConnection,
        path: P,
        offer: &FileOffer,
        mut progress_callback: F,
    ) -> Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(TransferProgress),
    {
        let path = path.as_ref();
        let total_size = offer.transfer_size();
        let mut progress = TransferProgress::new(total_size);

        // Get the data to send (possibly compressed)
        let data = if offer.is_compressed() {
            self.compress_file(path).await?
        } else {
            tokio::fs::read(path).await?
        };

        // Send in chunks
        let mut sent = 0usize;
        for chunk in data.chunks(self.chunk_size) {
            conn.send(chunk).await?;
            sent += chunk.len();
            progress.bytes_transferred = sent as u64;
            progress_callback(progress.clone());
        }

        // Send empty chunk to signal end
        conn.send(&[]).await?;

        Ok(())
    }

    /// Send a directory (as TAR archive) over a transit connection
    pub async fn send_directory<P, F>(
        &self,
        conn: &mut TransitConnection,
        path: P,
        offer: &FileOffer,
        mut progress_callback: F,
    ) -> Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(TransferProgress),
    {
        let path = path.as_ref();
        let total_size = offer.transfer_size();
        let mut progress = TransferProgress::new(total_size);

        // Create TAR archive
        let archive_data = self.create_tar_archive(path, offer.is_compressed()).await?;

        // Send in chunks
        let mut sent = 0usize;
        for chunk in archive_data.chunks(self.chunk_size) {
            conn.send(chunk).await?;
            sent += chunk.len();
            progress.bytes_transferred = sent as u64;
            progress_callback(progress.clone());
        }

        // Send empty chunk to signal end
        conn.send(&[]).await?;

        Ok(())
    }

    // ==================== RECEIVER SIDE ====================

    /// Receive a file from a transit connection
    ///
    /// Security: Validates file size against offer and maximum limits.
    pub async fn receive_file<P, F>(
        &self,
        conn: &mut TransitConnection,
        path: P,
        offer: &FileOffer,
        mut progress_callback: F,
    ) -> Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(TransferProgress),
    {
        let path = path.as_ref();
        let total_size = offer.transfer_size();

        // Security: Validate transfer size
        if total_size > MAX_TRANSFER_SIZE {
            return Err(Error::Transfer("File size exceeds maximum allowed".to_string()));
        }

        let mut progress = TransferProgress::new(total_size);

        // Pre-allocate with capacity, but with a reasonable limit to prevent OOM
        let initial_capacity = std::cmp::min(total_size as usize, 100 * 1024 * 1024);
        let mut data = Vec::with_capacity(initial_capacity);

        loop {
            let chunk = conn.receive().await?;
            if chunk.is_empty() {
                break;
            }

            // Security: Check we don't exceed the declared size
            if data.len() + chunk.len() > (total_size as usize) + self.chunk_size {
                return Err(Error::Transfer("Received more data than declared".to_string()));
            }

            data.extend_from_slice(&chunk);
            progress.bytes_transferred = data.len() as u64;
            progress_callback(progress.clone());
        }

        // Decompress if needed
        let file_data = if offer.is_compressed() {
            self.decompress_data(&data)?
        } else {
            data
        };

        // Write to file
        tokio::fs::write(path, file_data).await?;

        Ok(())
    }

    /// Receive a directory (TAR archive) from a transit connection
    ///
    /// Security: Validates archive size and extracts with path traversal protection.
    pub async fn receive_directory<P, F>(
        &self,
        conn: &mut TransitConnection,
        path: P,
        offer: &FileOffer,
        mut progress_callback: F,
    ) -> Result<()>
    where
        P: AsRef<Path>,
        F: FnMut(TransferProgress),
    {
        let path = path.as_ref();
        let total_size = offer.transfer_size();

        // Security: Validate transfer size
        if total_size > MAX_TRANSFER_SIZE {
            return Err(Error::Transfer("Archive size exceeds maximum allowed".to_string()));
        }

        let mut progress = TransferProgress::new(total_size);

        // Pre-allocate with capacity limit
        let initial_capacity = std::cmp::min(total_size as usize, 100 * 1024 * 1024);
        let mut data = Vec::with_capacity(initial_capacity);

        loop {
            let chunk = conn.receive().await?;
            if chunk.is_empty() {
                break;
            }

            // Security: Check we don't exceed the declared size
            if data.len() + chunk.len() > (total_size as usize) + self.chunk_size {
                return Err(Error::Transfer("Received more data than declared".to_string()));
            }

            data.extend_from_slice(&chunk);
            progress.bytes_transferred = data.len() as u64;
            progress_callback(progress.clone());
        }

        // Decompress if needed
        let archive_data = if offer.is_compressed() {
            self.decompress_data(&data)?
        } else {
            data
        };

        // Extract TAR archive
        self.extract_tar_archive(&archive_data, path)?;

        Ok(())
    }

    // ==================== COMPRESSION ====================

    /// Check if a file should be compressed based on its extension
    fn should_compress_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext_lower = ext.to_lowercase();
            !COMPRESSED_EXTENSIONS.contains(&ext_lower.as_str())
        } else {
            // No extension - compress by default
            true
        }
    }

    /// Compress a file using GZIP
    async fn compress_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        let data = tokio::fs::read(path).await?;
        self.compress_data(&data)
    }

    /// Compress data using GZIP
    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::write::GzEncoder;
        use flate2::Compression;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)
            .map_err(|e| Error::Transfer(format!("Compression error: {}", e)))?;
        encoder.finish()
            .map_err(|e| Error::Transfer(format!("Compression finish error: {}", e)))
    }

    /// Decompress GZIP data
    fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        use flate2::read::GzDecoder;

        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| Error::Transfer(format!("Decompression error: {}", e)))?;
        Ok(decompressed)
    }

    // ==================== TAR ARCHIVE ====================

    /// Create a TAR archive from a directory
    async fn create_tar_archive<P: AsRef<Path>>(&self, path: P, compress: bool) -> Result<Vec<u8>> {
        let path = path.as_ref().to_path_buf();

        // TAR creation is blocking, so run in a blocking thread
        let archive_data = tokio::task::spawn_blocking(move || {
            Self::create_tar_blocking(&path)
        }).await
            .map_err(|e| Error::Transfer(format!("TAR task error: {}", e)))??;

        if compress {
            self.compress_data(&archive_data)
        } else {
            Ok(archive_data)
        }
    }

    /// Blocking TAR archive creation
    fn create_tar_blocking(path: &Path) -> Result<Vec<u8>> {
        let mut builder = tar::Builder::new(Vec::new());

        let dir_name = path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::Transfer("Invalid directory name".to_string()))?;

        builder.append_dir_all(dir_name, path)
            .map_err(|e| Error::Transfer(format!("TAR append error: {}", e)))?;

        builder.into_inner()
            .map_err(|e| Error::Transfer(format!("TAR finish error: {}", e)))
    }

    /// Extract a TAR archive to a directory
    ///
    /// Security: This function validates each entry path to prevent path traversal
    /// attacks (Zip Slip vulnerability).
    fn extract_tar_archive(&self, data: &[u8], dest_path: &Path) -> Result<()> {
        let mut archive = tar::Archive::new(data);

        // Create the destination directory if it doesn't exist
        let dest_canonical = dest_path.canonicalize()
            .or_else(|_| {
                std::fs::create_dir_all(dest_path)?;
                dest_path.canonicalize()
            })
            .map_err(|e| Error::Transfer(format!("Invalid destination path: {}", e)))?;

        for entry in archive.entries()
            .map_err(|e| Error::Transfer(format!("TAR read error: {}", e)))? {

            let mut entry = entry
                .map_err(|e| Error::Transfer(format!("TAR entry error: {}", e)))?;

            let entry_path = entry.path()
                .map_err(|e| Error::Transfer(format!("TAR path error: {}", e)))?;

            // Security: Validate the entry path to prevent path traversal
            let full_path = dest_canonical.join(&entry_path);
            let full_path_canonical = if full_path.exists() {
                full_path.canonicalize()
                    .map_err(|e| Error::Transfer(format!("Path error: {}", e)))?
            } else {
                // For new files, check that all parent components are safe
                let mut safe_path = dest_canonical.clone();
                for component in entry_path.components() {
                    match component {
                        std::path::Component::Normal(name) => {
                            safe_path.push(name);
                        }
                        std::path::Component::ParentDir => {
                            // SECURITY: Reject any path with ".." components
                            return Err(Error::Transfer(
                                "Path traversal attempt detected in archive".to_string()
                            ));
                        }
                        _ => continue,
                    }
                }
                safe_path
            };

            // Verify the resolved path is inside the destination directory
            if !full_path_canonical.starts_with(&dest_canonical) {
                return Err(Error::Transfer(
                    "Path traversal attempt detected in archive".to_string()
                ));
            }

            // Create parent directories if needed
            if let Some(parent) = full_path_canonical.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| Error::Transfer(format!("Create dir error: {}", e)))?;
            }

            // Extract the entry
            entry.unpack(&full_path_canonical)
                .map_err(|e| Error::Transfer(format!("TAR unpack error: {}", e)))?;
        }

        Ok(())
    }

    // ==================== HASH VERIFICATION ====================

    /// Compute SHA-256 hash of a file
    pub async fn compute_file_hash<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let data = tokio::fs::read(path).await?;
        Ok(self.compute_hash(&data))
    }

    /// Compute SHA-256 hash of data
    pub fn compute_hash(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Verify file hash using constant-time comparison
    ///
    /// This prevents timing attacks where an attacker could learn about the
    /// expected hash by measuring comparison time.
    pub async fn verify_file_hash<P: AsRef<Path>>(&self, path: P, expected_hash: &str) -> Result<bool> {
        let actual_hash = self.compute_file_hash(path).await?;

        // Use constant-time comparison to prevent timing attacks
        let actual_bytes = actual_hash.as_bytes();
        let expected_bytes = expected_hash.as_bytes();

        if actual_bytes.len() != expected_bytes.len() {
            return Ok(false);
        }

        Ok(actual_bytes.ct_eq(expected_bytes).into())
    }

    // ==================== DIRECTORY UTILS ====================

    /// Count files and total bytes in a directory
    async fn count_directory_contents<P: AsRef<Path>>(&self, path: P) -> Result<(u64, u64)> {
        let path = path.as_ref().to_path_buf();

        tokio::task::spawn_blocking(move || {
            Self::count_directory_blocking(&path)
        }).await
            .map_err(|e| Error::Transfer(format!("Count task error: {}", e)))?
    }

    fn count_directory_blocking(path: &Path) -> Result<(u64, u64)> {
        let mut num_files = 0u64;
        let mut num_bytes = 0u64;

        fn walk_dir(path: &Path, num_files: &mut u64, num_bytes: &mut u64) -> Result<()> {
            for entry in std::fs::read_dir(path)
                .map_err(|e| Error::Transfer(format!("Read dir error: {}", e)))? {
                let entry = entry.map_err(|e| Error::Transfer(format!("Dir entry error: {}", e)))?;
                let entry_path = entry.path();

                if entry_path.is_file() {
                    *num_files += 1;
                    let metadata = std::fs::metadata(&entry_path)
                        .map_err(|e| Error::Transfer(format!("Metadata error: {}", e)))?;
                    *num_bytes += metadata.len();
                } else if entry_path.is_dir() {
                    walk_dir(&entry_path, num_files, num_bytes)?;
                }
            }
            Ok(())
        }

        walk_dir(path, &mut num_files, &mut num_bytes)?;
        Ok((num_files, num_bytes))
    }
}

impl Default for FileTransfer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_compress() {
        let transfer = FileTransfer::new();

        // Should compress text files
        assert!(transfer.should_compress_file(Path::new("test.txt")));
        assert!(transfer.should_compress_file(Path::new("data.json")));
        assert!(transfer.should_compress_file(Path::new("script.rs")));

        // Should NOT compress already compressed files
        assert!(!transfer.should_compress_file(Path::new("image.jpg")));
        assert!(!transfer.should_compress_file(Path::new("archive.zip")));
        assert!(!transfer.should_compress_file(Path::new("video.mp4")));
    }

    #[test]
    fn test_compress_decompress() {
        let transfer = FileTransfer::new();
        let original = b"Hello, World! This is a test of GZIP compression.".repeat(100);

        let compressed = transfer.compress_data(&original).unwrap();
        let decompressed = transfer.decompress_data(&compressed).unwrap();

        assert_eq!(original.to_vec(), decompressed);
        // Compressed should be smaller (for repeated data)
        assert!(compressed.len() < original.len());
    }

    #[test]
    fn test_hash_computation() {
        let transfer = FileTransfer::new();
        let data = b"Hello, World!";
        let hash = transfer.compute_hash(data);

        // Known SHA-256 hash for "Hello, World!"
        assert_eq!(hash, "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f");
    }

    #[test]
    fn test_progress() {
        let mut progress = TransferProgress::new(100);
        assert_eq!(progress.percentage(), 0.0);
        assert!(!progress.is_complete());

        progress.bytes_transferred = 50;
        assert_eq!(progress.percentage(), 50.0);

        progress.bytes_transferred = 100;
        assert!(progress.is_complete());
    }
}
