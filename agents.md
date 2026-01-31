# SecureBeam - Projekt Dokumentation

## Projektübersicht

SecureBeam ist eine P2P-Dateiübertragungslösung ähnlich wie [Magic Wormhole](https://github.com/magic-wormhole/magic-wormhole.rs). Das Ziel ist es, eine einfache und sichere Möglichkeit zu schaffen, Dateien direkt von PC zu PC zu übertragen.

### Architektur

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Client A       │     │  Signaling       │     │  Client B       │
│  (Rust/Tauri)   │◄───►│  Server (Rust)   │◄───►│  (Rust/Tauri)   │
└────────┬────────┘     └──────────────────┘     └────────┬────────┘
         │                                                 │
         │              ┌──────────────────┐              │
         └─────────────►│  P2P Verbindung  │◄─────────────┘
                        │  (Direkttransfer)│
                        └──────────────────┘
```

---

## Komponenten

### 1. Signaling Server (Rust Backend)

**Zweck:** Vermittelt die initiale Verbindung zwischen zwei Clients (ähnlich Magic Wormhole Mailbox Server)

**Technologie-Stack:**
- Rust
- Tokio (async runtime)
- Axum oder Actix-Web (Web Framework)
- WebSocket für Echtzeitkommunikation
- Redis (optional für Session-Speicherung)

**Funktionen:**
- [ ] Session-Erstellung mit eindeutigem Code
- [ ] Wormhole-Code Generierung (z.B. "7-crossword-puzzle")
- [ ] WebSocket-basierte Nachrichtenvermittlung
- [ ] PAKE (Password-Authenticated Key Exchange) Unterstützung
- [ ] Session-Timeout und Cleanup
- [ ] Health-Check Endpunkte

### 2. Native Desktop Clients

**Technologie-Stack:**
- Rust (Core-Logik)
- Tauri (Desktop Framework für alle Plattformen)
- React/TypeScript (Frontend UI - bestehendes Frontend nutzen)

**Unterstützte Plattformen:**
- [ ] Windows (x64)
- [ ] Linux (x64, AppImage/deb)
- [ ] macOS (Intel + Apple Silicon)

**Client-Funktionen:**
- [ ] Datei senden (mit generiertem Code)
- [ ] Datei empfangen (mit Code-Eingabe)
- [ ] Fortschrittsanzeige
- [ ] Drag & Drop Support
- [ ] Verschlüsselte Übertragung (E2E)
- [ ] NAT-Traversal (STUN/TURN)
- [ ] Direct Connection wenn möglich

### 3. Shared Library (Rust Crate)

**Zweck:** Gemeinsame Logik für alle Clients

**Module:**
- [ ] `protocol` - Protokolldefinitionen
- [ ] `crypto` - Verschlüsselung, PAKE
- [ ] `transfer` - Dateitransfer-Logik
- [ ] `network` - Netzwerk-Abstraktionen

---

## Projektstruktur

```
SecureBeam/
├── agents.md                 # Diese Datei
├── docker-compose.yml        # Docker Setup
├── frontend/                 # Bestehendes Web-Frontend (Coming Soon)
│
├── server/                   # Signaling Server
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
│       ├── main.rs
│       ├── config.rs
│       ├── handlers/
│       ├── models/
│       └── ws/
│
├── core/                     # Shared Rust Library
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── protocol.rs
│       ├── crypto.rs
│       ├── transfer.rs
│       └── network.rs
│
└── client/                   # Tauri Desktop Client
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── src/                  # Rust Backend
    └── ui/                   # Frontend (React)
```

---

## Entwicklungsplan

### Phase 1: Grundgerüst (Aktuell)

- [x] Projekt-Dokumentation erstellen (agents.md)
- [ ] Docker Compose Setup
- [ ] Signaling Server Grundstruktur
  - [ ] Cargo.toml mit Dependencies
  - [ ] Basic HTTP Server mit Axum
  - [ ] WebSocket Endpoint
  - [ ] Session Management

### Phase 2: Protokoll Implementation

- [ ] Core Library erstellen
- [ ] Wormhole-Code Generierung
- [ ] PAKE Implementation (SPAKE2)
- [ ] Nachrichten-Protokoll definieren

### Phase 3: Signaling Server Fertigstellung

- [ ] Vollständige WebSocket-Kommunikation
- [ ] Session Lifecycle Management
- [ ] Error Handling
- [ ] Logging & Monitoring
- [ ] Tests

### Phase 4: Desktop Client

- [ ] Tauri Projekt Setup
- [ ] UI Design & Implementation
- [ ] Client-Server Kommunikation
- [ ] P2P Verbindungsaufbau
- [ ] Dateitransfer

### Phase 5: Plattform-Builds

- [ ] Windows Build & Installer
- [ ] Linux Build (AppImage, .deb)
- [ ] macOS Build (Universal Binary)

### Phase 6: Testing & Polish

- [ ] Integration Tests
- [ ] Performance Optimierung
- [ ] Security Audit
- [ ] Dokumentation

---

## Referenzen

- [Magic Wormhole Rust](https://github.com/magic-wormhole/magic-wormhole.rs)
- [Magic Wormhole Protocol Docs](https://magic-wormhole.readthedocs.io/)
- [Tauri Framework](https://tauri.app/)
- [SPAKE2 Protocol](https://tools.ietf.org/html/draft-irtf-cfrg-spake2)

---

## Aktuelle Arbeit

**Status:** Phase 1 - Grundgerüst

**Nächste Schritte:**
1. Docker Compose erstellen
2. Signaling Server Grundstruktur aufsetzen
3. WebSocket Handler implementieren

---

## Commit-Historie

Commits werden regelmäßig erstellt um den Fortschritt zu dokumentieren.
