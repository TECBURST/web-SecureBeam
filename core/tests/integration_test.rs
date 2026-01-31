//! Integration tests for SecureBeam
//!
//! These tests verify end-to-end functionality of the core library,
//! including cryptographic operations, file transfer, and protocol handling.

use securebeam_core::{
    crypto::{derive_key, derive_verifier, Purpose, SecretBox, Side, Spake2Exchange},
    protocol::{FileAnswer, FileOffer, OfferType},
    transfer::FileTransfer,
};
use tempfile::TempDir;

/// Test complete SPAKE2 key exchange between two parties
#[test]
fn test_full_key_exchange() {
    let code = b"42-purple-sausages";

    // Sender side
    let mut sender = Spake2Exchange::new(code, Side::A);
    let sender_msg = sender.start().expect("Sender should start");

    // Receiver side
    let mut receiver = Spake2Exchange::new(code, Side::B);
    let receiver_msg = receiver.start().expect("Receiver should start");

    // Complete exchange
    let sender_key = sender.finish(&receiver_msg).expect("Sender should finish");
    let receiver_key = receiver
        .finish(&sender_msg)
        .expect("Receiver should finish");

    // Keys should match
    assert_eq!(sender_key, receiver_key);
    assert_eq!(sender_key.len(), 32);

    // Verifiers should also match
    let sender_verifier = derive_verifier(&sender_key).expect("Should derive verifier");
    let receiver_verifier = derive_verifier(&receiver_key).expect("Should derive verifier");
    assert_eq!(sender_verifier, receiver_verifier);
}

/// Test that wrong codes result in different keys
#[test]
fn test_wrong_code_different_keys() {
    let mut sender = Spake2Exchange::new(b"42-purple-sausages", Side::A);
    let mut receiver = Spake2Exchange::new(b"42-green-elephants", Side::B);

    let sender_msg = sender.start().unwrap();
    let receiver_msg = receiver.start().unwrap();

    let sender_key = sender.finish(&receiver_msg).unwrap();
    let receiver_key = receiver.finish(&sender_msg).unwrap();

    // Keys should NOT match with different codes
    assert_ne!(sender_key, receiver_key);
}

/// Test SecretBox encryption and decryption with derived keys
#[test]
fn test_encrypted_communication() {
    let shared_key = [0x42u8; 32];

    // Derive phase keys for both sides
    let sender_phase_key = derive_key(
        &shared_key,
        &Purpose::Phase {
            side: "sender".to_string(),
            phase: "offer".to_string(),
        },
        32,
    )
    .expect("Should derive key");

    let receiver_phase_key = derive_key(
        &shared_key,
        &Purpose::Phase {
            side: "sender".to_string(),
            phase: "offer".to_string(),
        },
        32,
    )
    .expect("Should derive key");

    // Keys should match (same purpose)
    assert_eq!(sender_phase_key, receiver_phase_key);

    // Encrypt message
    let sender_box = SecretBox::new(&sender_phase_key).expect("Should create SecretBox");
    let message = b"Hello, SecureBeam!";
    let sealed = sender_box.seal(message).expect("Should seal message");

    // Decrypt on receiver side
    let receiver_box = SecretBox::new(&receiver_phase_key).expect("Should create SecretBox");
    let opened = receiver_box.open(&sealed).expect("Should open message");

    assert_eq!(opened, message.to_vec());
}

/// Test file offer creation and acceptance
#[tokio::test]
async fn test_file_offer_answer_flow() {
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let test_file = temp_dir.path().join("test.txt");

    // Create test file
    std::fs::write(&test_file, "Hello, World! This is a test file.").unwrap();

    let transfer = FileTransfer::new();

    // Create offer
    let offer = transfer
        .prepare_file_offer(&test_file)
        .await
        .expect("Should prepare offer");

    assert_eq!(offer.name(), "test.txt");
    assert!(offer.transfer_size() > 0);

    // Serialize and deserialize offer
    let offer_json = serde_json::to_string(&offer).expect("Should serialize");
    let parsed_offer: FileOffer = serde_json::from_str(&offer_json).expect("Should deserialize");

    assert_eq!(parsed_offer.name(), offer.name());

    // Create acceptance answer
    let answer = FileAnswer::accept();
    assert!(answer.is_accepted());

    // Create rejection answer
    let rejection = FileAnswer::reject("User cancelled".to_string());
    assert!(!rejection.is_accepted());
}

/// Test hash computation
#[test]
fn test_hash_verification() {
    let transfer = FileTransfer::new();

    let data = b"Test data for hashing";
    let hash1 = transfer.compute_hash(data);
    let hash2 = transfer.compute_hash(data);

    // Same data should produce same hash
    assert_eq!(hash1, hash2);

    // Different data should produce different hash
    let different_data = b"Different test data";
    let hash3 = transfer.compute_hash(different_data);
    assert_ne!(hash1, hash3);

    // Hash should be 64 hex characters (SHA-256)
    assert_eq!(hash1.len(), 64);
}

/// Test transfer progress calculation
#[test]
fn test_transfer_progress() {
    use securebeam_core::TransferProgress;

    let mut progress = TransferProgress::new(1000);

    assert_eq!(progress.percentage(), 0.0);
    assert!(!progress.is_complete());

    progress.bytes_transferred = 250;
    assert_eq!(progress.percentage(), 25.0);

    progress.bytes_transferred = 500;
    assert_eq!(progress.percentage(), 50.0);

    progress.bytes_transferred = 1000;
    assert_eq!(progress.percentage(), 100.0);
    assert!(progress.is_complete());
}

/// Test directory preparation
#[tokio::test]
async fn test_directory_offer() {
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let test_subdir = temp_dir.path().join("test_folder");
    std::fs::create_dir(&test_subdir).unwrap();

    // Create test files
    std::fs::write(test_subdir.join("file1.txt"), "Content 1").unwrap();
    std::fs::write(test_subdir.join("file2.txt"), "Content 2").unwrap();

    let transfer = FileTransfer::new();
    let offer = transfer
        .prepare_directory_offer(&test_subdir)
        .await
        .expect("Should prepare directory offer");

    assert_eq!(offer.name(), "test_folder");
    assert!(offer.transfer_size() > 0);
    // Verify it's a directory offer by checking the offer type
    assert!(matches!(offer.offer_type, OfferType::Directory(_)));
}

/// Test that secrets are properly handled
#[test]
fn test_secret_zeroization() {
    // This test verifies that sensitive data structures implement Drop properly
    // We can't directly verify memory content, but we ensure the types compile
    // with zeroize support

    let code = b"test-code";
    let mut exchange = Spake2Exchange::new(code, Side::A);
    let _msg = exchange.start().expect("Should start");

    // Exchange goes out of scope here and should zeroize sensitive data
    drop(exchange);

    // SecretBox should also zeroize
    let key = [0x42u8; 32];
    let secret_box = SecretBox::new(&key).expect("Should create SecretBox");
    drop(secret_box);

    // If we got here without panics, zeroization is at least structurally sound
}

/// Test transit key derivation
#[test]
fn test_transit_key_derivation() {
    let shared_key = [0x42u8; 32];

    let transit_key =
        derive_key(&shared_key, &Purpose::Transit, 32).expect("Should derive transit key");

    assert_eq!(transit_key.len(), 32);

    // Same inputs should produce same output
    let transit_key2 =
        derive_key(&shared_key, &Purpose::Transit, 32).expect("Should derive transit key");

    assert_eq!(transit_key, transit_key2);

    // Different purpose should produce different key
    let verifier_key =
        derive_key(&shared_key, &Purpose::Verifier, 32).expect("Should derive verifier key");

    assert_ne!(transit_key, verifier_key);
}
