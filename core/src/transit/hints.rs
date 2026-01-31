//! Transit hints for connection establishment
//!
//! Hints tell the other peer how to connect to us.

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

/// Collection of transit hints
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransitHints {
    /// Direct connection hints (IP:port pairs)
    #[serde(default)]
    pub direct_hints: Vec<DirectHint>,
    /// Relay server hints
    #[serde(default)]
    pub relay_hints: Vec<RelayHint>,
}

impl TransitHints {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a direct hint
    pub fn add_direct(&mut self, addr: SocketAddr, priority: i32) {
        self.direct_hints.push(DirectHint {
            hostname: addr.ip().to_string(),
            port: addr.port(),
            priority,
        });
    }

    /// Add a relay hint
    pub fn add_relay(&mut self, url: &str) {
        self.relay_hints.push(RelayHint {
            url: url.to_string(),
        });
    }

    /// Merge hints from another set
    pub fn merge(&mut self, other: TransitHints) {
        self.direct_hints.extend(other.direct_hints);
        self.relay_hints.extend(other.relay_hints);
    }

    /// Sort direct hints by priority (higher = better)
    pub fn sort_by_priority(&mut self) {
        self.direct_hints
            .sort_by(|a, b| b.priority.cmp(&a.priority));
    }
}

/// A direct connection hint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectHint {
    /// Hostname or IP address
    pub hostname: String,
    /// Port number
    pub port: u16,
    /// Priority (higher = prefer)
    #[serde(default)]
    pub priority: i32,
}

impl DirectHint {
    /// Create a new direct hint
    pub fn new(hostname: &str, port: u16) -> Self {
        Self {
            hostname: hostname.to_string(),
            port,
            priority: 0,
        }
    }

    /// Create with priority
    pub fn with_priority(hostname: &str, port: u16, priority: i32) -> Self {
        Self {
            hostname: hostname.to_string(),
            port,
            priority,
        }
    }

    /// Get as socket address string
    pub fn to_addr_string(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }
}

/// A relay server hint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayHint {
    /// Relay URL (e.g., "tcp://relay.example.com:4001")
    pub url: String,
}

impl RelayHint {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    /// Parse the URL to get host and port
    pub fn parse(&self) -> Option<(String, u16)> {
        let url = self.url.strip_prefix("tcp://")?;
        let parts: Vec<&str> = url.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        let port: u16 = parts[1].parse().ok()?;
        Some((parts[0].to_string(), port))
    }
}

/// Gather local hints for direct connection
pub async fn gather_local_hints(listen_port: u16) -> TransitHints {
    let mut hints = TransitHints::new();

    // Get local IP addresses
    if let Ok(addrs) = local_ip_addresses() {
        for addr in addrs {
            let priority = if addr.is_ipv6() { 10 } else { 5 };
            hints.add_direct(SocketAddr::new(addr, listen_port), priority);
        }
    }

    // TODO: Add STUN to get external IP

    hints
}

/// Get local IP addresses
fn local_ip_addresses() -> std::io::Result<Vec<IpAddr>> {
    use std::net::UdpSocket;

    let mut addrs = Vec::new();

    // Try to get the default route IP by connecting to a public address
    // (doesn't actually send any packets)
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(local_addr) = socket.local_addr() {
                addrs.push(local_addr.ip());
            }
        }
    }

    // Try IPv6
    if let Ok(socket) = UdpSocket::bind("[::]:0") {
        if socket.connect("[2001:4860:4860::8888]:80").is_ok() {
            if let Ok(local_addr) = socket.local_addr() {
                addrs.push(local_addr.ip());
            }
        }
    }

    Ok(addrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_hint() {
        let hint = DirectHint::new("192.168.1.1", 8080);
        assert_eq!(hint.to_addr_string(), "192.168.1.1:8080");
    }

    #[test]
    fn test_relay_hint_parse() {
        let hint = RelayHint::new("tcp://relay.example.com:4001");
        let (host, port) = hint.parse().unwrap();
        assert_eq!(host, "relay.example.com");
        assert_eq!(port, 4001);
    }

    #[test]
    fn test_hints_merge() {
        let mut hints1 = TransitHints::new();
        hints1.add_direct("192.168.1.1:8080".parse().unwrap(), 5);

        let mut hints2 = TransitHints::new();
        hints2.add_direct("192.168.1.2:8080".parse().unwrap(), 10);

        hints1.merge(hints2);
        assert_eq!(hints1.direct_hints.len(), 2);
    }
}
