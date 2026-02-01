//! SecureBeam Desktop Client - Tauri Backend
//!
//! Provides commands for the Vue.js frontend to interact with
//! the SecureBeam core library for P2P file transfers.

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tauri::{Emitter, State};
use tokio::sync::Mutex;

use securebeam_core::{
    crypto::{derive_key, Purpose, Side, Spake2Exchange},
    establish_transit, FileAnswer, FileOffer, FileTransfer, Message, SignalingClient, TransitHints,
    TransitRole, DEFAULT_MAILBOX, DEFAULT_RELAY,
};

/// Application state
pub struct AppState {
    /// Current transfer in progress
    pub transfer: Mutex<Option<TransferState>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            transfer: Mutex::new(None),
        }
    }
}

/// Current transfer state
pub struct TransferState {
    pub code: String,
    pub path: String,
    pub offer: Option<FileOffer>,
    pub is_directory: bool,
}

/// Transfer progress info for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferProgressInfo {
    pub bytes_transferred: u64,
    pub total_bytes: u64,
    pub percentage: f64,
    pub speed_mbps: f64,
    pub eta_seconds: Option<f64>,
    pub status: String,
}

/// File offer info for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOfferInfo {
    pub name: String,
    pub size: u64,
    pub compressed: bool,
    pub is_directory: bool,
}

/// Generate word lists for codes
const ADJECTIVES: &[&str] = &[
    "purple", "green", "blue", "red", "orange", "yellow", "silver", "golden", "crimson", "azure",
    "amber", "coral", "ivory", "jade", "lime", "navy",
];

const NOUNS: &[&str] = &[
    "sausages",
    "elephants",
    "guitars",
    "planets",
    "mountains",
    "rivers",
    "clouds",
    "forests",
    "dragons",
    "unicorns",
    "wizards",
    "knights",
    "castles",
    "oceans",
    "comets",
    "crystals",
];

/// Generate a new wormhole code for sending
#[tauri::command]
fn generate_code() -> Result<String, String> {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..1000);
    let adj = ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())];
    let noun = NOUNS[rng.gen_range(0..NOUNS.len())];
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
        is_directory: false,
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
        is_directory: true,
    })
}

/// Start sending a file
#[tauri::command]
async fn start_send(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    path: String,
    code: String,
    is_directory: bool,
) -> Result<(), String> {
    let transfer_state = TransferState {
        code: code.clone(),
        path: path.clone(),
        offer: None,
        is_directory,
    };

    *state.transfer.lock().await = Some(transfer_state);

    // Spawn the transfer task
    let app_handle = app.clone();
    tokio::spawn(async move {
        if let Err(e) = run_sender(app_handle, path, code, is_directory).await {
            eprintln!("Transfer error: {}", e);
        }
    });

    Ok(())
}

/// Run the sender transfer
async fn run_sender(
    app: tauri::AppHandle,
    path: String,
    code: String,
    is_directory: bool,
) -> Result<(), String> {
    // Emit status
    let _ = app.emit("transfer-status", "Connecting to server...");

    // Prepare the offer
    let file_transfer = FileTransfer::new();
    let offer = if is_directory {
        file_transfer
            .prepare_directory_offer(&path)
            .await
            .map_err(|e| e.to_string())?
    } else {
        file_transfer
            .prepare_file_offer(&path)
            .await
            .map_err(|e| e.to_string())?
    };

    // Connect to mailbox server
    let client = SignalingClient::new(DEFAULT_MAILBOX);
    let mut session = client.connect(&code).await.map_err(|e| e.to_string())?;

    let _ = app.emit("transfer-status", "Waiting for receiver...");

    // SPAKE2 key exchange
    let mut exchange = Spake2Exchange::new(code.as_bytes(), Side::A);
    let our_pake_msg = exchange.start().map_err(|e| e.to_string())?;

    // Send our PAKE message
    session
        .send(&serde_json::json!({"pake": our_pake_msg.to_hex()}).to_string())
        .await
        .map_err(|e| e.to_string())?;

    // Wait for peer's PAKE message
    let peer_msg_text = session
        .receive()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Connection closed")?;

    let peer_json: serde_json::Value =
        serde_json::from_str(&peer_msg_text).map_err(|e| e.to_string())?;
    let peer_pake_hex = peer_json["pake"].as_str().ok_or("Missing pake field")?;
    let peer_pake_msg = securebeam_core::crypto::Spake2Message::from_hex(peer_pake_hex)
        .map_err(|e| e.to_string())?;

    // Complete key exchange
    let shared_key = exchange.finish(&peer_pake_msg).map_err(|e| e.to_string())?;

    let _ = app.emit(
        "transfer-status",
        "Key exchange complete. Establishing connection...",
    );

    // Derive transit key
    let transit_key = derive_key(&shared_key, &Purpose::Transit, 32).map_err(|e| e.to_string())?;

    // Exchange transit hints
    let our_hints = TransitHints {
        direct_hints: vec![],
        relay_hints: vec![securebeam_core::transit::RelayHint {
            url: DEFAULT_RELAY.to_string(),
        }],
    };

    session
        .send(&serde_json::to_string(&our_hints).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;

    let peer_hints_text = session
        .receive()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Connection closed")?;

    let peer_hints: TransitHints =
        serde_json::from_str(&peer_hints_text).map_err(|e| e.to_string())?;

    // Merge hints
    let mut combined_hints = our_hints;
    combined_hints.direct_hints.extend(peer_hints.direct_hints);
    combined_hints.relay_hints.extend(peer_hints.relay_hints);

    // Establish transit connection
    let _ = app.emit("transfer-status", "Establishing P2P connection...");

    let mut transit = establish_transit(TransitRole::Sender, &combined_hints, &transit_key)
        .await
        .map_err(|e| e.to_string())?;

    // Send offer
    let offer_msg = Message::offer(offer.clone());
    transit
        .send(&offer_msg.to_bytes().map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;

    let _ = app.emit("transfer-status", "Waiting for acceptance...");

    // Wait for answer
    let answer_bytes = transit.receive().await.map_err(|e| e.to_string())?;
    let answer_msg = Message::from_bytes(&answer_bytes).map_err(|e| e.to_string())?;

    match answer_msg {
        Message::Answer(answer) => {
            if !answer.is_accepted() {
                return Err("Transfer rejected by receiver".to_string());
            }
        }
        _ => return Err("Unexpected message type".to_string()),
    }

    let _ = app.emit("transfer-status", "Transferring...");

    // Send file with progress
    let total_size = offer.transfer_size();
    let start_time = Instant::now();
    let app_clone = app.clone();

    let progress_callback = move |progress: securebeam_core::TransferProgress| {
        let elapsed = start_time.elapsed().as_secs_f64();
        let speed_bps = if elapsed > 0.0 {
            progress.bytes_transferred as f64 / elapsed
        } else {
            0.0
        };

        let eta = if speed_bps > 0.0 && progress.bytes_transferred < total_size {
            Some((total_size - progress.bytes_transferred) as f64 / speed_bps)
        } else {
            None
        };

        let info = TransferProgressInfo {
            bytes_transferred: progress.bytes_transferred,
            total_bytes: total_size,
            percentage: progress.percentage(),
            speed_mbps: speed_bps / (1024.0 * 1024.0),
            eta_seconds: eta,
            status: "Transferring...".to_string(),
        };

        let _ = app_clone.emit("transfer-progress", info);
    };

    if is_directory {
        file_transfer
            .send_directory(&mut transit, &path, &offer, progress_callback)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        file_transfer
            .send_file(&mut transit, &path, &offer, progress_callback)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Wait for ACK
    let ack_bytes = transit.receive().await.map_err(|e| e.to_string())?;
    let _ack_msg = Message::from_bytes(&ack_bytes).map_err(|e| e.to_string())?;

    let _ = app.emit("transfer-status", "Transfer complete!");
    let _ = app.emit("transfer-complete", ());

    Ok(())
}

/// Start receiving a file
#[tauri::command]
async fn start_receive(
    app: tauri::AppHandle,
    code: String,
    save_path: String,
) -> Result<(), String> {
    let app_handle = app.clone();
    tokio::spawn(async move {
        if let Err(e) = run_receiver(app_handle, code, save_path).await {
            eprintln!("Receive error: {}", e);
        }
    });

    Ok(())
}

/// Run the receiver transfer
async fn run_receiver(
    app: tauri::AppHandle,
    code: String,
    save_path: String,
) -> Result<(), String> {
    let _ = app.emit("transfer-status", "Connecting to server...");

    // Connect to mailbox server
    let client = SignalingClient::new(DEFAULT_MAILBOX);
    let mut session = client.connect(&code).await.map_err(|e| e.to_string())?;

    let _ = app.emit("transfer-status", "Exchanging keys...");

    // SPAKE2 key exchange
    let mut exchange = Spake2Exchange::new(code.as_bytes(), Side::B);
    let our_pake_msg = exchange.start().map_err(|e| e.to_string())?;

    // Send our PAKE message
    session
        .send(&serde_json::json!({"pake": our_pake_msg.to_hex()}).to_string())
        .await
        .map_err(|e| e.to_string())?;

    // Wait for peer's PAKE message
    let peer_msg_text = session
        .receive()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Connection closed")?;

    let peer_json: serde_json::Value =
        serde_json::from_str(&peer_msg_text).map_err(|e| e.to_string())?;
    let peer_pake_hex = peer_json["pake"].as_str().ok_or("Missing pake field")?;
    let peer_pake_msg = securebeam_core::crypto::Spake2Message::from_hex(peer_pake_hex)
        .map_err(|e| e.to_string())?;

    // Complete key exchange
    let shared_key = exchange.finish(&peer_pake_msg).map_err(|e| e.to_string())?;

    let _ = app.emit(
        "transfer-status",
        "Key exchange complete. Establishing connection...",
    );

    // Derive transit key
    let transit_key = derive_key(&shared_key, &Purpose::Transit, 32).map_err(|e| e.to_string())?;

    // Exchange transit hints
    let our_hints = TransitHints {
        direct_hints: vec![],
        relay_hints: vec![securebeam_core::transit::RelayHint {
            url: DEFAULT_RELAY.to_string(),
        }],
    };

    session
        .send(&serde_json::to_string(&our_hints).map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;

    let peer_hints_text = session
        .receive()
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Connection closed")?;

    let peer_hints: TransitHints =
        serde_json::from_str(&peer_hints_text).map_err(|e| e.to_string())?;

    // Merge hints
    let mut combined_hints = our_hints;
    combined_hints.direct_hints.extend(peer_hints.direct_hints);
    combined_hints.relay_hints.extend(peer_hints.relay_hints);

    // Establish transit connection
    let _ = app.emit("transfer-status", "Establishing P2P connection...");

    let mut transit = establish_transit(TransitRole::Receiver, &combined_hints, &transit_key)
        .await
        .map_err(|e| e.to_string())?;

    // Receive offer
    let offer_bytes = transit.receive().await.map_err(|e| e.to_string())?;
    let offer_msg = Message::from_bytes(&offer_bytes).map_err(|e| e.to_string())?;

    let offer = match offer_msg {
        Message::Offer(o) => o,
        _ => return Err("Expected offer message".to_string()),
    };

    // Emit offer info for user confirmation (simplified - auto-accept for now)
    let offer_info = FileOfferInfo {
        name: offer.name().to_string(),
        size: offer.transfer_size(),
        compressed: offer.is_compressed(),
        is_directory: matches!(offer.offer_type, securebeam_core::OfferType::Directory(_)),
    };
    let _ = app.emit("file-offer", offer_info.clone());

    // Accept the transfer
    let answer = FileAnswer::accept();
    let answer_msg = Message::answer(answer);
    transit
        .send(&answer_msg.to_bytes().map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;

    let _ = app.emit("transfer-status", "Receiving...");

    // Receive file with progress
    let file_transfer = FileTransfer::new();
    let total_size = offer.transfer_size();
    let start_time = Instant::now();
    let app_clone = app.clone();

    let progress_callback = move |progress: securebeam_core::TransferProgress| {
        let elapsed = start_time.elapsed().as_secs_f64();
        let speed_bps = if elapsed > 0.0 {
            progress.bytes_transferred as f64 / elapsed
        } else {
            0.0
        };

        let eta = if speed_bps > 0.0 && progress.bytes_transferred < total_size {
            Some((total_size - progress.bytes_transferred) as f64 / speed_bps)
        } else {
            None
        };

        let info = TransferProgressInfo {
            bytes_transferred: progress.bytes_transferred,
            total_bytes: total_size,
            percentage: progress.percentage(),
            speed_mbps: speed_bps / (1024.0 * 1024.0),
            eta_seconds: eta,
            status: "Receiving...".to_string(),
        };

        let _ = app_clone.emit("transfer-progress", info);
    };

    let dest_path = format!("{}/{}", save_path, offer.name());

    if offer_info.is_directory {
        file_transfer
            .receive_directory(&mut transit, &dest_path, &offer, progress_callback)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        file_transfer
            .receive_file(&mut transit, &dest_path, &offer, progress_callback)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Send ACK
    transit
        .send(&Message::Ack.to_bytes().map_err(|e| e.to_string())?)
        .await
        .map_err(|e| e.to_string())?;

    let _ = app.emit("transfer-status", "Transfer complete!");
    let _ = app.emit("transfer-complete", ());

    Ok(())
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

/// Get the saved download path
#[tauri::command]
fn get_download_path() -> Option<String> {
    // Try to read from a config file or use platform defaults
    if let Some(config_dir) = dirs::config_dir() {
        let config_file = config_dir.join("securebeam").join("config.json");
        if config_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_file) {
                if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(path) = config["download_path"].as_str() {
                        return Some(path.to_string());
                    }
                }
            }
        }
    }
    None
}

/// Get the default download path
#[tauri::command]
fn get_default_download_path() -> String {
    dirs::download_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| {
            dirs::home_dir()
                .map(|p| p.join("Downloads").to_string_lossy().to_string())
                .unwrap_or_else(|| "~/Downloads".to_string())
        })
}

/// Set the download path
#[tauri::command]
fn set_download_path(path: String) -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("securebeam");

    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    let config_file = config_dir.join("config.json");

    // Read existing config or create new
    let mut config: serde_json::Value = if config_file.exists() {
        let content = std::fs::read_to_string(&config_file).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    config["download_path"] = serde_json::Value::String(path);

    std::fs::write(
        &config_file,
        serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Test connection to the signaling server
#[tauri::command]
async fn test_signaling_connection() -> bool {
    use tokio::time::{timeout, Duration};

    let result = timeout(Duration::from_secs(5), async {
        // Try to establish a WebSocket connection to the signaling server
        let url = format!("{}/ws/test-ping", DEFAULT_MAILBOX);
        match tokio_tungstenite::connect_async(&url).await {
            Ok(_) => {
                // Successfully connected - connection will be dropped automatically
                true
            }
            Err(_) => false,
        }
    })
    .await;

    result.unwrap_or(false)
}

/// Test connection to the relay server
#[tauri::command]
async fn test_relay_connection() -> bool {
    use tokio::time::{timeout, Duration};

    let result = timeout(Duration::from_secs(5), async {
        // Try TCP connection to relay server
        let relay_url = DEFAULT_RELAY.trim_start_matches("tcp://");
        match tokio::net::TcpStream::connect(relay_url).await {
            Ok(_) => true,
            Err(_) => false,
        }
    })
    .await;

    result.unwrap_or(false)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            generate_code,
            parse_code,
            prepare_file,
            prepare_directory,
            start_send,
            start_receive,
            format_size,
            get_version,
            get_download_path,
            get_default_download_path,
            set_download_path,
            test_signaling_connection,
            test_relay_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
