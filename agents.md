# SecureBeam - Project Documentation

## Project Overview

SecureBeam is a P2P file transfer solution based on the [Magic Wormhole Protocol](https://github.com/magic-wormhole/magic-wormhole-protocols). The goal is to provide a simple and **cryptographically secure** way to transfer files directly from PC to PC.

**Security is the highest priority** - we strictly follow the proven Magic Wormhole protocol.

---

## Architecture (Magic Wormhole Style)

```
┌─────────────────┐                                    ┌─────────────────┐
│  Client A       │                                    │  Client B       │
│  (Sender)       │                                    │  (Receiver)     │
└────────┬────────┘                                    └────────┬────────┘
         │                                                      │
         │  1. Allocate Nameplate                              │
         │  2. Open Mailbox                                    │
         │  3. PAKE (SPAKE2)          ┌──────────────┐        │
         │◄────────────────────────────│   Mailbox    │────────►│
         │                             │   Server     │         │
         │  4. Exchange Version        │   (Rust)     │         │
         │  5. Transit Hints           └──────────────┘         │
         │                                                      │
         │                                                      │
         │  6. Direct P2P (if possible)                        │
         │◄────────────────────────────────────────────────────►│
         │                                                      │
         │  7. Relay Fallback          ┌──────────────┐        │
         │◄────────────────────────────│   Transit    │────────►│
         │                             │   Relay      │         │
         │                             └──────────────┘         │
         │                                                      │
         │  8. Encrypted File Transfer (NaCl SecretBox)        │
         └──────────────────────────────────────────────────────┘
```

---

## Project Components

### 1. Core Library (`core/`)

The shared Rust library implementing the Magic Wormhole protocol.

```
core/
├── Cargo.toml
└── src/
    ├── lib.rs           # Public API exports
    ├── network.rs       # SignalingClient for mailbox communication
    ├── protocol.rs      # Message types (Offer, Answer, Ack)
    ├── transfer.rs      # FileTransfer with progress callbacks
    ├── crypto/
    │   ├── mod.rs
    │   ├── derive.rs    # HKDF key derivation
    │   └── key_exchange.rs  # SPAKE2 implementation
    └── transit/
        ├── mod.rs       # Transit connection establishment
        └── connection.rs # Encrypted transit connection
```

**Features:**
- [x] SPAKE2 key exchange
- [x] NaCl SecretBox encryption (XSalsa20-Poly1305)
- [x] HKDF-SHA256 key derivation
- [x] File transfer with progress callbacks
- [x] Directory transfer (TAR archive)
- [x] GZIP compression (automatic for text files)
- [x] Zeroize for sensitive data

### 2. Mailbox Server (`server/`)

Handles signaling between clients for connection establishment.

```
server/
├── Cargo.toml
├── Dockerfile
└── src/
    ├── main.rs
    ├── config.rs
    ├── handlers/
    ├── models/
    └── ws/
```

**Features:**
- [x] WebSocket transport
- [x] Session management
- [x] Message relay between peers
- [x] Automatic cleanup

### 3. Transit Relay (`relay-server/`)

Fallback relay when direct P2P connection fails.

```
relay-server/
├── Cargo.toml
├── Dockerfile
└── src/
    ├── main.rs
    └── relay.rs
```

**Features:**
- [x] TCP/WebSocket relay
- [x] Channel-based routing
- [x] No decryption (pass-through only)

### 4. Web Frontend (`frontend/`)

Landing page and download portal.

```
frontend/
├── package.json
└── src/
    ├── App.vue
    ├── views/
    │   └── HomeView.vue    # Download page with OS detection
    ├── components/
    └── i18n/               # Internationalization
```

**Features:**
- [x] Vue.js 3 + TypeScript
- [x] Tailwind CSS
- [x] Dynamic download fetching from GitHub releases
- [x] OS-specific download buttons (Windows, macOS, Linux)
- [x] i18n support (EN, DE)

### 5. Desktop Client (`client/`)

Native cross-platform application built with Tauri 2.0.

```
client/
├── package.json
├── src/
│   ├── App.vue
│   ├── views/
│   │   ├── HomeView.vue      # Main menu
│   │   ├── SendView.vue      # File sending with progress
│   │   └── ReceiveView.vue   # File receiving with progress
│   └── components/
└── src-tauri/
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── capabilities/
    │   └── default.json      # Tauri 2 permissions
    └── src/
        └── lib.rs            # Rust backend with full transfer logic
```

**Features:**
- [x] Full P2P file transfer (not demo mode)
- [x] Native file picker dialogs
- [x] Real-time progress display
- [x] Transfer speed (MB/s)
- [x] ETA (estimated time remaining)
- [x] SPAKE2 key exchange via mailbox
- [x] Transit connection (relay fallback)
- [x] File and directory support

---

## Security Implementation

### Cryptography Stack

| Component | Algorithm | Purpose |
|-----------|-----------|---------|
| **PAKE** | SPAKE2 (Ed25519) | Key exchange with wormhole code as password |
| **KDF** | HKDF-SHA256 | Key derivation for different purposes |
| **Encryption** | NaCl SecretBox (XSalsa20-Poly1305) | Authenticated encryption |
| **Verifier** | SHA256 Subkey | Man-in-the-Middle detection |

### Key Derivation

```
Shared PAKE Key
      │
      ├──► HKDF("wormhole:verifier") ──► Verifier (for MITM check)
      │
      ├──► HKDF("wormhole:phase:{side_hash}:{phase_hash}") ──► Phase Key
      │
      └──► HKDF("transit:key") ──► Transit Encryption Key
```

### Security Measures

- [x] Zeroize for sensitive data (keys, passwords)
- [x] Constant-time comparisons for crypto operations
- [x] Path traversal protection (TAR extraction)
- [x] Input validation with size limits
- [x] Error message sanitization
- [x] No logging of secrets

---

## Development Status

### Completed Phases

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Protocol Foundation | ✅ |
| 2 | Cryptography (SPAKE2, NaCl, HKDF) | ✅ |
| 3 | Mailbox Server | ✅ |
| 4 | Transit Protocol | ✅ |
| 5 | File Transfer | ✅ |
| 6 | Desktop Client (Tauri 2.0) | ✅ |
| 7 | Security Hardening | ✅ |
| 8 | CI/CD Pipeline | ✅ |
| 9 | Production Deployment | ✅ |

### Platform Support

| Platform | Format | Status |
|----------|--------|--------|
| Windows | .exe, .msi | ✅ |
| macOS | .dmg | ✅ |
| Linux | .AppImage, .deb, .rpm | ✅ |

---

## Infrastructure

### Docker Services

```yaml
services:
  frontend:      # Web frontend (Nginx)
  server:        # Mailbox server (Rust)
  relay:         # Transit relay (Rust)
  nginx:         # Reverse proxy
```

### Deployment

- **Frontend**: GitHub Pages (automatic deploy on push to master)
- **Servers**: Docker Compose on VPS
- **Releases**: GitHub Releases (automatic builds via CI/CD)

---

## References

### Protocol Documentation
- [Magic Wormhole Protocols](https://github.com/magic-wormhole/magic-wormhole-protocols)
- [Server Protocol](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/server-protocol.md)
- [Client Protocol](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/client-protocol.md)
- [Transit Protocol](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/transit.md)
- [File Transfer Protocol](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/file-transfer-protocol.md)

### Rust Implementation (Reference)
- [magic-wormhole.rs](https://github.com/magic-wormhole/magic-wormhole.rs)

### Cryptography
- [SPAKE2 RFC Draft](https://tools.ietf.org/html/draft-irtf-cfrg-spake2)
- [NaCl Crypto Library](https://nacl.cr.yp.to/)
- [spake2 Crate](https://docs.rs/spake2)

### Frameworks
- [Tauri Framework](https://tauri.app/)
- [Vue.js](https://vuejs.org/)

---

## Quick Start

### Development

```bash
# Web Frontend
cd frontend
npm install
npm run dev

# Desktop Client
cd client
yarn install
yarn tauri dev

# Servers (Docker)
docker-compose up -d
```

### Building

```bash
# Frontend
cd frontend
npm run build

# Desktop Client (all platforms)
cd client
yarn tauri build
```

---

## License

MIT
