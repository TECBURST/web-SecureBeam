<p align="center">
  <img src="https://raw.githubusercontent.com/TECBURST/web-SecureBeam/master/frontend/public/logo.svg" alt="SecureBeam Logo" width="120" height="120">
</p>

<h1 align="center">SecureBeam</h1>

<p align="center">
  <strong>Secure P2P File Transfer</strong>
</p>

<p align="center">
  Transfer files directly between computers with end-to-end encryption.<br>
  No accounts. No cloud storage. No file size limits.
</p>

<p align="center">
  <a href="https://github.com/TECBURST/web-SecureBeam/releases/latest">
    <img src="https://img.shields.io/github/v/release/TECBURST/web-SecureBeam?style=flat-square" alt="Latest Release">
  </a>
  <a href="https://github.com/TECBURST/web-SecureBeam/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/TECBURST/web-SecureBeam/ci.yml?style=flat-square" alt="Build Status">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/github/license/TECBURST/web-SecureBeam?style=flat-square" alt="License">
  </a>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#download">Download</a> •
  <a href="#how-it-works">How It Works</a> •
  <a href="#security">Security</a> •
  <a href="#development">Development</a>
</p>

---

## Features

- **End-to-End Encryption** - Files are encrypted using NaCl SecretBox (XSalsa20-Poly1305)
- **Direct P2P Transfer** - Files go directly between devices, no cloud storage
- **No Account Required** - Just share a code with the recipient
- **Cross-Platform** - Native apps for Windows, macOS, and Linux
- **No File Size Limits** - Transfer files of any size
- **Progress Tracking** - Real-time speed and ETA display
- **Directory Support** - Send entire folders as compressed archives

## Download

### Desktop Applications

| Platform | Download |
|----------|----------|
| **Windows** | [SecureBeam.exe](https://github.com/TECBURST/web-SecureBeam/releases/latest) / [SecureBeam.msi](https://github.com/TECBURST/web-SecureBeam/releases/latest) |
| **macOS** | [SecureBeam.dmg](https://github.com/TECBURST/web-SecureBeam/releases/latest) |
| **Linux** | [SecureBeam.AppImage](https://github.com/TECBURST/web-SecureBeam/releases/latest) / [.deb](https://github.com/TECBURST/web-SecureBeam/releases/latest) / [.rpm](https://github.com/TECBURST/web-SecureBeam/releases/latest) |

Or visit our [website](https://securebeam.io) to download the latest version.

## How It Works

SecureBeam uses the [Magic Wormhole Protocol](https://github.com/magic-wormhole/magic-wormhole-protocols) for secure file transfer.

### Sending a File

1. Open SecureBeam and select a file or folder
2. A unique code is generated (e.g., `123-purple-sausages`)
3. Share this code with the recipient

### Receiving a File

1. Open SecureBeam and enter the code
2. Choose where to save the file
3. Click "Connect & Receive"

### Under the Hood

```
Sender                                                    Receiver
   │                                                          │
   │  1. Generate Code (123-purple-sausages)                 │
   │──────────────────────────────────────────────────────────│
   │                                                          │
   │  2. SPAKE2 Key Exchange                                 │
   │◄────────────────────────────────────────────────────────►│
   │     (Secure key derivation from shared code)            │
   │                                                          │
   │  3. Establish Transit Connection                        │
   │◄────────────────────────────────────────────────────────►│
   │     (Direct P2P or relay fallback)                      │
   │                                                          │
   │  4. Encrypted File Transfer                             │
   │──────────────────────────────────────────────────────────►
   │     (XSalsa20-Poly1305 authenticated encryption)        │
```

## Security

SecureBeam prioritizes security above all else:

### Cryptography

| Component | Algorithm | Purpose |
|-----------|-----------|---------|
| Key Exchange | SPAKE2 | Password-authenticated key exchange |
| Encryption | XSalsa20-Poly1305 | Authenticated encryption |
| Key Derivation | HKDF-SHA256 | Derive keys for different purposes |

### Security Features

- **Zero Knowledge** - Servers never see your files or encryption keys
- **Forward Secrecy** - Each transfer uses unique keys
- **MITM Protection** - SPAKE2 prevents man-in-the-middle attacks
- **Memory Safety** - Written in Rust with automatic memory management
- **Zeroization** - Sensitive data is securely wiped from memory

### What We DON'T Do

- Store your files
- Log file names or contents
- Require accounts or registration
- Access your encryption keys

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     SecureBeam                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   │
│  │   Desktop    │   │   Mailbox    │   │   Transit    │   │
│  │    Client    │   │    Server    │   │    Relay     │   │
│  │   (Tauri)    │   │    (Rust)    │   │    (Rust)    │   │
│  └──────┬───────┘   └──────┬───────┘   └──────┬───────┘   │
│         │                  │                  │            │
│         └──────────────────┴──────────────────┘            │
│                            │                               │
│                   ┌────────┴────────┐                      │
│                   │   Core Library  │                      │
│                   │ (securebeam-core)│                      │
│                   └─────────────────┘                      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.70+
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Setup

```bash
# Clone the repository
git clone https://github.com/TECBURST/web-SecureBeam.git
cd SecureBeam

# Install frontend dependencies
cd frontend
npm install

# Install desktop client dependencies
cd ../client
yarn install
```

### Running

```bash
# Web frontend (development)
cd frontend
npm run dev

# Desktop client (development)
cd client
yarn tauri dev

# Run servers with Docker
docker-compose up -d
```

### Building

```bash
# Build web frontend
cd frontend
npm run build

# Build desktop client
cd client
yarn tauri build
```

### Project Structure

```
SecureBeam/
├── core/               # Shared Rust library (crypto, protocol, transfer)
├── server/             # Mailbox server (signaling)
├── relay-server/       # Transit relay (fallback connection)
├── frontend/           # Web frontend (Vue.js)
├── client/             # Desktop client (Tauri + Vue.js)
├── docker-compose.yml  # Docker configuration
└── agents.md           # Detailed project documentation
```

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting a pull request.

### Reporting Security Issues

If you discover a security vulnerability, please report it responsibly. See our [Security Policy](SECURITY.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Magic Wormhole](https://github.com/magic-wormhole/magic-wormhole) - Protocol inspiration
- [Tauri](https://tauri.app/) - Cross-platform desktop framework
- [Vue.js](https://vuejs.org/) - Frontend framework

---

<p align="center">
  Made with security in mind
</p>
