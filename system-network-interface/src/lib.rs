#![no_std]

/// Result of a peer-discovery scan on the local broadcast domain.
/// Step 3 (Genesis Protocol) implements mDNS/DNS-SD (RFC 6762/6763) discovery.
pub enum DiscoveryResult {
    /// A PPN peer was found via mDNS zero-config discovery.
    MdnsFound { addr: [u8; 4] },
    /// A peer address was supplied by the operator (fallback for cloud-VM / cross-VLAN).
    OperatorSupplied { addr: [u8; 4] },
    /// No peers found; this node is the genesis seed.
    NotFound,
}

/// Scan for existing PPN peers via mDNS (RFC 6762).
/// Returns the first peer address found, or NotFound after the discovery window.
/// Step 3 implements the real mDNS responder and UDP multicast scan.
pub fn scan_for_peers() -> DiscoveryResult {
    DiscoveryResult::NotFound
}

/// Send a Genesis Protocol handshake frame to the pairing server at `peer_addr`.
/// `short_code` is the 8-character Crockford base32 SAS confirmation string.
/// Returns true if the handshake was accepted; false if rejected or timed out.
/// Step 3 implements the real CPace PAKE (RFC 9382) key exchange over UDP.
pub fn send_genesis_handshake(_peer_addr: [u8; 4], _short_code: &[u8]) -> bool {
    false
}

pub fn system_status() -> &'static str {
    "system-network-interface: scaffold (Step 3 pending)"
}
