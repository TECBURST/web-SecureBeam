//! SecureBeam Desktop Client - Tauri Backend
//!
//! Provides commands for the Vue.js frontend to interact with
//! the SecureBeam core library for P2P file transfers.

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Re-export core types we need
use securebeam_core::FileTransfer;

/// Application state
pub struct AppState {
    /// Current wormhole code (if sender)
    pub wormhole_code: Mutex<Option<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            wormhole_code: Mutex::new(None),
        }
    }
}

/// File offer info for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOfferInfo {
    pub name: String,
    pub size: u64,
    pub compressed: bool,
}

/// Generate a new wormhole code for sending
#[tauri::command]
fn generate_code() -> Result<String, String> {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..1000);

    // Word list for human-readable codes (simplified)
    let adjectives = [
        "purple", "green", "blue", "red", "orange", "yellow", "silver", "golden",
    ];
    let nouns = [
        "sausages",
        "elephants",
        "guitars",
        "planets",
        "mountains",
        "rivers",
        "clouds",
        "forests",
    ];

    let adj = adjectives[rng.gen_range(0..adjectives.len())];
    let noun = nouns[rng.gen_range(0..nouns.len())];

    Ok(format!("{}-{}-{}", number, adj, noun))
}

/// Parse a wormhole code into its components
#[tauri::command]
fn parse_code(code: String) -> Result<(String, String), String> {
    let parts: Vec<&str> = code.split('-').collect();
    if parts.len() < 3 {
        return Err("Invalid code format. Expected: number-word-word".to_string());
    }

    let nameplate = parts[0].to_string();
    let password = parts[1..].join("-");

    Ok((nameplate, password))
}

/// Prepare a file for sending
#[tauri::command]
async fn prepare_file(path: String) -> Result<FileOfferInfo, String> {
    let transfer = FileTransfer::new();

    let offer = transfer
        .prepare_file_offer(&path)
        .await
        .map_err(|e| e.to_string())?;

    Ok(FileOfferInfo {
        name: offer.name().to_string(),
        size: offer.transfer_size(),
        compressed: offer.is_compressed(),
    })
}

/// Prepare a directory for sending
#[tauri::command]
async fn prepare_directory(path: String) -> Result<FileOfferInfo, String> {
    let transfer = FileTransfer::new();

    let offer = transfer
        .prepare_directory_offer(&path)
        .await
        .map_err(|e| e.to_string())?;

    Ok(FileOfferInfo {
        name: offer.name().to_string(),
        size: offer.transfer_size(),
        compressed: offer.is_compressed(),
    })
}

/// Format file size for display
#[tauri::command]
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Get the core library version
#[tauri::command]
fn get_version() -> String {
    securebeam_core::VERSION.to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            generate_code,
            parse_code,
            prepare_file,
            prepare_directory,
            format_size,
            get_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
