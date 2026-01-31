use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::models::AppState;

/// Word lists for generating memorable codes
const WORDS: &[&str] = &[
    "apple", "banana", "cherry", "dragon", "eagle", "falcon", "guitar", "hammer",
    "island", "jungle", "kiwi", "lemon", "mango", "neptune", "orange", "piano",
    "quantum", "rocket", "sunset", "tiger", "umbrella", "violin", "whisper", "xylophone",
    "yellow", "zebra", "anchor", "breeze", "crystal", "dolphin", "ember", "forest",
    "glacier", "horizon", "ivory", "jasmine", "kernel", "lantern", "meadow", "nebula",
    "ocean", "phoenix", "quartz", "rainbow", "shadow", "thunder", "unity", "velvet",
    "wonder", "zenith",
];

/// Generate a human-readable session code
fn generate_code() -> String {
    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen_range(1..=9);
    let word1 = WORDS[rng.gen_range(0..WORDS.len())];
    let word2 = WORDS[rng.gen_range(0..WORDS.len())];
    format!("{}-{}-{}", number, word1, word2)
}

#[derive(Deserialize)]
pub struct CreateSessionRequest {
    /// Optional custom timeout in seconds
    pub timeout_secs: Option<u64>,
}

#[derive(Serialize)]
pub struct CreateSessionResponse {
    pub code: String,
    pub expires_at: String,
}

/// Create a new session
pub async fn create_session(
    State(state): State<Arc<AppState>>,
    Json(request): Json<Option<CreateSessionRequest>>,
) -> Result<Json<CreateSessionResponse>, (StatusCode, String)> {
    let timeout_secs = request
        .and_then(|r| r.timeout_secs)
        .unwrap_or(300); // 5 minutes default

    // Generate unique code
    let code = loop {
        let candidate = generate_code();
        if state.get_session(&candidate).await.is_none() {
            break candidate;
        }
    };

    let session = state.create_session(code.clone(), timeout_secs).await;

    tracing::info!("Created new session: {}", code);

    Ok(Json(CreateSessionResponse {
        code: session.code,
        expires_at: session.expires_at.to_rfc3339(),
    }))
}

#[derive(Serialize)]
pub struct SessionInfoResponse {
    pub code: String,
    pub status: String,
    pub connected_clients: u8,
    pub created_at: String,
    pub expires_at: String,
}

/// Get session information
pub async fn get_session_info(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<Json<SessionInfoResponse>, (StatusCode, String)> {
    let session = state
        .get_session(&code)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Session not found".to_string()))?;

    if session.is_expired() {
        return Err((StatusCode::GONE, "Session expired".to_string()));
    }

    Ok(Json(SessionInfoResponse {
        code: session.code,
        status: format!("{:?}", session.status).to_lowercase(),
        connected_clients: session.connected_clients,
        created_at: session.created_at.to_rfc3339(),
        expires_at: session.expires_at.to_rfc3339(),
    }))
}
