# SecureBeam - Projekt Dokumentation

## Projektübersicht

SecureBeam ist eine P2P-Dateiübertragungslösung basierend auf dem [Magic Wormhole Protokoll](https://github.com/magic-wormhole/magic-wormhole-protocols). Das Ziel ist es, eine einfache und **kryptographisch sichere** Möglichkeit zu schaffen, Dateien direkt von PC zu PC zu übertragen.

**Sicherheit hat höchste Priorität** - wir orientieren uns streng am bewährten Magic Wormhole Protokoll.

---

## Architektur (Magic Wormhole Style)

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
         │  6. Direct P2P (wenn möglich)                       │
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

## Magic Wormhole Protokoll - Unsere Ziele

### Mailbox Server Protokoll (Server)

Quelle: [server-protocol.md](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/server-protocol.md)

**Konzepte die wir implementieren müssen:**

| Konzept | Beschreibung | Status |
|---------|--------------|--------|
| **AppID** | Eindeutige Anwendungs-ID (z.B. `securebeam.io/file-transfer`) | [ ] |
| **Nameplates** | Kurze numerische Codes (die "4" in "4-purple-sausages") | [ ] |
| **Mailboxen** | Speichern Nachrichten mit Phase + Body | [ ] |
| **Sides** | Hex-String um eigene vs. Partner-Nachrichten zu unterscheiden | [ ] |
| **Phasen** | Strukturierter Nachrichtenfluss | [ ] |

**Nachrichtenfluss:**
```
Client                          Server
   │                               │
   │──── bind (appid, side) ──────►│
   │◄─── welcome ─────────────────│
   │                               │
   │──── allocate ────────────────►│  (Sender: bekommt Nameplate)
   │◄─── allocated ───────────────│
   │                               │
   │──── claim (nameplate) ───────►│  (Receiver: claimed Nameplate)
   │◄─── claimed ─────────────────│
   │                               │
   │──── open (mailbox) ──────────►│
   │◄─── opened ──────────────────│
   │                               │
   │──── add (phase, body) ───────►│  (Nachrichten austauschen)
   │◄─── message ─────────────────│
   │                               │
   │──── close ───────────────────►│
   │◄─── closed ──────────────────│
```

### Client Protokoll (PAKE & Verschlüsselung)

Quelle: [client-protocol.md](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/client-protocol.md)

**Kryptographie-Stack:**

| Komponente | Algorithmus | Zweck |
|------------|-------------|-------|
| **PAKE** | SPAKE2 (Ed25519) | Schlüsselaustausch mit Wormhole-Code als Passwort |
| **KDF** | HKDF-SHA256 | Schlüsselableitung für verschiedene Zwecke |
| **Verschlüsselung** | NaCl SecretBox (XSalsa20-Poly1305) | Authentifizierte Verschlüsselung |
| **Verifier** | SHA256 Subkey | Man-in-the-Middle Erkennung |

**Phasen-Ablauf:**
```
1. PAKE Phase
   - Beide Seiten senden SPAKE2 Nachricht
   - Berechnen gemeinsamen Schlüssel aus Wormhole-Code

2. Version Phase
   - Erste verschlüsselte Nachricht
   - Enthält App-Versions-Info
   - Erfolgreiche Dekryptierung = Vertrauen etabliert

3. Application Phase
   - App-spezifische verschlüsselte Nachrichten
   - Für uns: Transit Hints, File Metadata
```

**Schlüsselableitung:**
```
Shared PAKE Key
      │
      ├──► HKDF("wormhole:verifier") ──► Verifier (zur MITM-Prüfung)
      │
      ├──► HKDF("wormhole:phase:{side_hash}:{phase_hash}") ──► Phase Key
      │
      └──► HKDF("transit:key") ──► Transit Encryption Key
```

### Transit Protokoll (P2P Transfer)

Quelle: [transit.md](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/transit.md)

**Verbindungsaufbau (Priorität):**

1. **Direkte Verbindung versuchen**
   - Lokale IP-Adressen sammeln
   - STUN für externe IP
   - Beide Seiten verbinden gleichzeitig (Hole Punching)
   - `SO_REUSEADDR` für NAT-Traversal

2. **Relay als Fallback**
   - Wenn direkt nicht möglich
   - Server leitet verschlüsselte Daten durch
   - Kein Zugriff auf Klartext

**Handshake:**
```
Sender:   "transit sender {hash} ready\n\n"
Receiver: "transit receiver {hash} ready\n\n"
```

### Dateitransfer & Compression

Quelle: [file-transfer-protocol.md](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/file-transfer-protocol.md)

**Transfer-Ablauf:**

| Schritt | Beschreibung |
|---------|--------------|
| 1. Offer | Sender schickt File-Metadata (Name, Größe, Hash) |
| 2. Answer | Receiver akzeptiert oder lehnt ab |
| 3. Transfer | Verschlüsselte Chunks über Transit |
| 4. Verify | Hash-Verifikation am Ende |

**Compression (wichtig!):**
- **Einzelne Dateien**: Optional GZIP komprimiert
- **Verzeichnisse**: Als TAR-Archiv (optional GZIP)
- Compression-Flag in Offer-Nachricht
- Client entscheidet basierend auf Dateityp

```rust
// Compression Logik
if file.is_already_compressed() {
    // ZIP, MP4, JPG, etc. → keine Compression
    send_raw(file)
} else {
    // TXT, JSON, LOG, etc. → GZIP
    send_compressed(gzip(file))
}
```

---

## Komponenten (Überarbeitet)

### 1. Mailbox Server (Rust)

**Ersetzt unseren bisherigen "Signaling Server"**

**Funktionen:**
- [ ] AppID Registrierung und Validierung
- [ ] Nameplate Allocation (numerisch, kurz)
- [ ] Mailbox Management (Phasen-basiert)
- [ ] Side-Tracking für Nachrichtenzuordnung
- [ ] Nachrichtenspeicherung bis Abruf
- [ ] Automatische Cleanup abgelaufener Sessions
- [ ] WebSocket Transport mit JSON Messages

### 2. Transit Relay (Rust)

**Neuer Service für Relay-Fallback**

**Funktionen:**
- [ ] TCP/WebSocket Relay
- [ ] Channel-basiertes Routing
- [ ] Keine Dekryptierung (nur Durchleitung)
- [ ] Bandbreiten-Limits
- [ ] Connection Timeout

### 3. Core Library (securebeam-core)

**Module:**

```
securebeam-core/
├── crypto/
│   ├── spake2.rs      # SPAKE2 Implementation
│   ├── secretbox.rs   # NaCl SecretBox Wrapper
│   ├── hkdf.rs        # Schlüsselableitung
│   └── verifier.rs    # MITM-Detection
│
├── protocol/
│   ├── mailbox.rs     # Mailbox Client-Protokoll
│   ├── transit.rs     # Transit Protokoll
│   └── messages.rs    # Nachrichtentypen
│
├── transfer/
│   ├── file.rs        # Einzeldatei-Transfer
│   ├── directory.rs   # Verzeichnis als TAR
│   ├── compression.rs # GZIP Compression
│   └── chunking.rs    # Chunk-basierter Transfer
│
└── network/
    ├── direct.rs      # Direkte P2P Verbindung
    ├── relay.rs       # Relay Fallback
    ├── stun.rs        # STUN Client
    └── hints.rs       # Transit Hints Exchange
```

### 4. Native Clients (Tauri)

**Plattformen:**
- [ ] Windows (x64)
- [ ] Linux (x64, AppImage/deb)
- [ ] macOS (Intel + Apple Silicon)

**Features:**
- [ ] Wormhole-Code Generierung & Eingabe
- [ ] Drag & Drop
- [ ] Fortschrittsanzeige
- [ ] Verifier-Anzeige (für paranoid mode)
- [ ] Auto-Compression basierend auf Dateityp

---

## Sicherheitsanforderungen

### Kryptographie

| Anforderung | Implementation |
|-------------|----------------|
| Schlüsselaustausch | SPAKE2 (nicht selbst implementieren, Crate nutzen!) |
| Verschlüsselung | NaCl SecretBox via `sodiumoxide` oder `crypto_box` Crate |
| KDF | HKDF-SHA256 via `hkdf` Crate |
| Zufallszahlen | `rand` mit `OsRng` |

### Wichtige Sicherheitsregeln

1. **Keine eigene Crypto** - Nur bewährte Crates verwenden
2. **Constant-Time Vergleiche** - Für alle kryptographischen Vergleiche
3. **Secure Memory** - Keys nach Verwendung überschreiben
4. **No Logging of Secrets** - Niemals Schlüssel oder Codes loggen
5. **Verifier anzeigen** - User können MITM erkennen

---

## Projektstruktur (Aktualisiert)

```
SecureBeam/
├── agents.md                 # Diese Datei
├── docker-compose.yml        # Docker Setup
├── frontend/                 # Web-Frontend (Coming Soon)
│
├── mailbox-server/           # Mailbox Server (neu)
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
│       ├── main.rs
│       ├── protocol.rs       # Mailbox Protokoll
│       ├── nameplate.rs      # Nameplate Management
│       ├── mailbox.rs        # Mailbox Storage
│       └── ws.rs             # WebSocket Handler
│
├── relay-server/             # Transit Relay (neu)
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
│       ├── main.rs
│       └── relay.rs
│
├── core/                     # Shared Library
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── crypto/
│       ├── protocol/
│       ├── transfer/
│       └── network/
│
└── client/                   # Tauri Desktop Client
    ├── Cargo.toml
    ├── tauri.conf.json
    ├── src-tauri/
    └── src/                  # React Frontend
```

---

## Entwicklungsplan (Überarbeitet)

### Phase 1: Protokoll-Fundament ✓
- [x] Projekt-Dokumentation (agents.md)
- [x] Grundstruktur Server
- [x] Docker Compose Setup
- [x] Magic Wormhole Protokoll studiert

### Phase 2: Kryptographie ✓
- [x] SPAKE2 Integration (`spake2` Crate)
- [x] NaCl SecretBox (`xsalsa20poly1305` Crate)
- [x] HKDF Implementation (`hkdf` Crate)
- [x] Unit Tests für Crypto (12 Tests)

### Phase 3: Mailbox Server ✓
- [x] Nameplate Allocation
- [x] Mailbox mit Phasen
- [x] Side-Tracking
- [x] Vollständiger Message-Flow (bind, allocate, claim, open, add, close)
- [x] Unit Tests (10 Tests)

### Phase 4: Transit ✓
- [x] Direct Connection (TCP mit Hints)
- [x] Relay Server (Transit Relay implementiert)
- [x] Handshake Protokoll
- [x] Verbindungsauswahl (Direct first, Relay fallback)
- [x] Transit Connection mit NaCl Verschlüsselung
- [ ] STUN für externe IP (optional)
- [ ] Hole-Punching (optional)

### Phase 5: File Transfer
- [ ] Offer/Answer Protokoll
- [ ] Chunked Transfer
- [ ] GZIP Compression
- [ ] TAR für Verzeichnisse
- [ ] Hash-Verifikation

### Phase 6: Desktop Client
- [ ] Tauri Setup
- [ ] UI Implementation
- [ ] Integration mit Core Library
- [ ] Plattform-Builds

### Phase 7: Security Hardening
- [ ] Security Audit
- [ ] Fuzzing
- [ ] Memory Safety Review
- [ ] Dependency Audit

---

## Referenzen

### Protokoll-Dokumentation
- [Magic Wormhole Protocols](https://github.com/magic-wormhole/magic-wormhole-protocols)
- [Server Protocol](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/server-protocol.md)
- [Client Protocol](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/client-protocol.md)
- [Transit Protocol](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/transit.md)
- [File Transfer Protocol](https://github.com/magic-wormhole/magic-wormhole-protocols/blob/main/file-transfer-protocol.md)

### Rust Implementation (Referenz)
- [magic-wormhole.rs](https://github.com/magic-wormhole/magic-wormhole.rs)

### Kryptographie
- [SPAKE2 RFC Draft](https://tools.ietf.org/html/draft-irtf-cfrg-spake2)
- [NaCl Crypto Library](https://nacl.cr.yp.to/)
- [sodiumoxide Crate](https://docs.rs/sodiumoxide)
- [spake2 Crate](https://docs.rs/spake2)

### Tools
- [Tauri Framework](https://tauri.app/)

---

## Aktuelle Arbeit

**Status:** Phase 4 abgeschlossen → Phase 5 (File Transfer)

**Erledigte Meilensteine:**
- ✅ Crypto Module mit SPAKE2, NaCl SecretBox, HKDF
- ✅ Mailbox Server mit vollständigem Protokoll
- ✅ Transit Relay Server implementiert
- ✅ Core Library Transit Module (direct + relay)
- ✅ Encrypted Transit Connection
- ✅ 15 Unit Tests bestanden

**Nächste Schritte:**
1. File Transfer Protokoll (Offer/Answer)
2. Chunked Transfer mit Progress
3. GZIP Compression für Textdateien
4. TAR für Verzeichnisse

---

## Commit-Historie

| Commit | Beschreibung |
|--------|--------------|
| `014b7d5` | Initial infrastructure (simplified) |
| `b1c1055` | Clean up compiler warnings |
| `7570b10` | Update agents.md with protocol details |
| `17ab5c5` | Implement Magic Wormhole compatible crypto module |
| `67ceac0` | Implement Magic Wormhole compatible Mailbox Server |
