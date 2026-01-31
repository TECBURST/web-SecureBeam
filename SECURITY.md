# SecureBeam Security

This document describes the security measures implemented in SecureBeam.

## Cryptographic Protocol

SecureBeam implements the Magic Wormhole protocol for secure file transfer:

### Key Exchange (SPAKE2)
- Uses SPAKE2 (Password-Authenticated Key Exchange) with Ed25519 group
- Symmetric mode with shared identity
- Passwords are stored in zeroizing memory

### Encryption (NaCl SecretBox)
- XSalsa20-Poly1305 authenticated encryption
- 256-bit keys (32 bytes)
- 192-bit random nonces (24 bytes)
- 128-bit authentication tags (16 bytes)

### Key Derivation (HKDF)
- HKDF-SHA256 for deriving purpose-specific keys
- Separate keys for:
  - Verifier (MITM detection)
  - Phase messages
  - Transit encryption

## Security Measures

### Memory Safety
- All cryptographic keys are zeroized on drop using the `zeroize` crate
- Passwords stored in `Zeroizing<Vec<u8>>` wrapper
- No sensitive data in error messages

### Timing Attack Prevention
- Constant-time comparisons for:
  - Cryptographic verification
  - Hash comparison
  - Authentication checks
- Uses the `subtle` crate for constant-time operations

### Input Validation
- Maximum file size: 10 GB
- Maximum directory files: 100,000
- Path traversal protection in TAR extraction
- Size verification during transfers

### Path Traversal Protection
- TAR extraction validates all entry paths
- Rejects entries with `..` components
- Verifies resolved paths stay within destination

## Error Handling

Error messages are intentionally generic to avoid information leakage:
- "Cryptographic operation failed" (instead of specific crypto errors)
- "Authentication failed" (instead of "wrong password")
- "Security verification failed" (instead of "MITM detected")

Detailed error information is available via `Error::details()` for logging.

## Dependencies

Security-critical dependencies:
- `spake2` - SPAKE2 key exchange
- `xsalsa20poly1305` - NaCl SecretBox
- `hkdf` - Key derivation
- `sha2` - SHA-256 hashing
- `subtle` - Constant-time operations
- `zeroize` - Secure memory clearing
- `rand` - Cryptographic random numbers

## Threat Model

### Protected Against
- Passive network observers (encryption)
- Active MITM attacks (PAKE + verifier)
- Password guessing (SPAKE2 rate limiting by server)
- Timing attacks (constant-time operations)
- Memory disclosure (zeroization)
- Path traversal (archive validation)

### Not Protected Against
- Compromised endpoints
- Weak/guessable wormhole codes
- Physical access to devices
- Side-channel attacks (beyond timing)

## Reporting Vulnerabilities

Please report security vulnerabilities to security@securebeam.io
