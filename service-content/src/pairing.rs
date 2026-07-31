// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Peer pairing for service-content — `POST /v1/pair` receiver side.
//!
//! Wire format matches project-orchestration's app-orchestration-command v0.0.1:
//!   token = `<base64url(payload_json)>.<base64url(ed25519_sig_over_payload_b64_bytes)>`
//!   public_key = base64url of the issuing node's Ed25519 verifying key (32 bytes)
//!
//! Totebox persists pairings to `$GRAPH_DIR/pairing-store.jsonl` (append-only)
//! and writes WORM audit entries to `$GRAPH_DIR/pair-audit.jsonl`.
//!
//! Totebox-side token issuance: `PairingKeypair::issue_token()`. The keypair
//! seed is persisted to `$GRAPH_DIR/totebox-pair.seed` (32 bytes, raw) so
//! the public key is stable across restarts.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

// ── token payload ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPayload {
    pub issuer: String,
    pub role: String,
    pub nonce: String,
    pub expiry: String,
    #[serde(default)]
    pub archive_scope: Vec<String>,
    #[serde(default)]
    pub peer_type: String,
}

impl TokenPayload {
    pub fn is_expired(&self) -> bool {
        match self.expiry.parse::<DateTime<Utc>>() {
            Ok(exp) => Utc::now() > exp,
            Err(_) => true,
        }
    }
}

// ── pairing record ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingRecord {
    pub public_key: String,
    pub issuer: String,
    pub peer_type: String,
    pub role: String,
    pub archive_scope: Vec<String>,
    pub node_label: String,
    pub paired_on: String,
    pub nonce: String,
}

// ── error ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum PairError {
    Malformed,
    BadSignature,
    Expired,
    #[allow(dead_code)]
    NonceReused,
}

impl std::fmt::Display for PairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PairError::Malformed => write!(f, "malformed token"),
            PairError::BadSignature => write!(f, "invalid signature"),
            PairError::Expired => write!(f, "token expired"),
            PairError::NonceReused => write!(f, "nonce already used"),
        }
    }
}

// ── token verification ────────────────────────────────────────────────────────

/// Verify a pairing token against the caller-supplied public key.
///
/// Returns the embedded payload on success.
/// The caller must separately check nonce uniqueness.
pub fn verify_pair_token(token: &str, public_key_b64: &str) -> Result<TokenPayload, PairError> {
    let (payload_b64, sig_b64) = token.split_once('.').ok_or(PairError::Malformed)?;

    // Decode public key (32 bytes → VerifyingKey).
    let pk_bytes = URL_SAFE_NO_PAD
        .decode(public_key_b64)
        .map_err(|_| PairError::Malformed)?;
    let pk_arr: [u8; 32] = pk_bytes.try_into().map_err(|_| PairError::Malformed)?;
    let vk = VerifyingKey::from_bytes(&pk_arr).map_err(|_| PairError::Malformed)?;

    // Decode signature (64 bytes).
    let sig_bytes = URL_SAFE_NO_PAD
        .decode(sig_b64)
        .map_err(|_| PairError::Malformed)?;
    let sig_arr: [u8; 64] = sig_bytes.try_into().map_err(|_| PairError::Malformed)?;
    let sig = Signature::from_bytes(&sig_arr);

    // Verify: signature is over the payload_b64 bytes (same convention as membership.rs).
    use ed25519_dalek::Verifier as _;
    vk.verify(payload_b64.as_bytes(), &sig)
        .map_err(|_| PairError::BadSignature)?;

    // Decode and parse payload.
    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| PairError::Malformed)?;
    let payload: TokenPayload =
        serde_json::from_slice(&payload_bytes).map_err(|_| PairError::Malformed)?;

    if payload.is_expired() {
        return Err(PairError::Expired);
    }

    Ok(payload)
}

// ── Command-sourced peers (O9 credential-chain wiring) ─────────────────────────
//
// app-orchestration-command's `PairingStore` (a separate, same-named-but-
// unrelated type in a different, proprietary crate) records its own pairings
// — console/instance ↔ CommandCentre — using a fundamentally different
// protocol: an invite-token ceremony that authorizes enrollment but never
// itself proves the enrolling peer holds the private key for the public_key
// it registers. service-content's own pairing (above) proves live key
// possession on every request instead (`verify_capability`'s Ed25519
// signature check). These are genuinely incompatible protocols, not just
// "shared vocabulary" — reconciling them by relaying Command's raw invite
// token would not work (verify_capability expects a signature over a
// payload, not an invite proof).
//
// The sound bridge: teach service-content to ALSO trust public keys Command
// has recorded (an additional, alternate source for "who is a registered
// peer"), while keeping service-content's own signature verification as the
// thing that actually proves possession at request time. A peer paired via
// Command still has to sign its own `X-Foundry-Capability` header with the
// same key it registered — service-content's existing `verify_capability`
// call is completely unchanged; only the "resolve from_instance → public_key"
// step gains a second, optional source.
//
// Deliberately NOT a new Cargo dependency on `orchestration-command-core`
// (that crate is `LicenseRef-PointSav-Proprietary`; service-content is
// AGPL-3.0-or-later — a dependency in that direction is not legal per this
// workspace's own license-boundary rule, matching the same "duplicate a
// tiny type, don't share a crate" pattern already used for O5/O8). This
// struct only needs to deserialize the same wire shape Command already
// writes to `user-pairings.yaml`, not import Command's actual Rust types.
//
// Both processes currently run on the same host (foundry-workspace) — reading
// the file directly avoids inventing a new network/auth surface between the
// two services for what is, today, a local file both could equally read.
// If the two are ever split across hosts, this becomes an HTTP lookup
// instead; the `find_by_instance`/`resolve_public_key` split below is
// designed so that swap doesn't touch any caller.

/// One entry from Command's `user-pairings.yaml`, deserialized locally rather
/// than via a shared type (see module doc comment above). Only the fields
/// service-content actually needs are captured; unknown fields are ignored.
#[derive(Debug, Clone, Deserialize)]
pub struct CommandPairedPeer {
    pub public_key: String,
    /// Command's `PairingRole` enum ("User" | "Admin" | "Interface"),
    /// deserialized as a plain string — service-content doesn't need to
    /// interpret it (archive_scope for the actual request comes from the
    /// signed `CapabilityPayload` itself, not from this record; see
    /// `capability_gate`'s doc comment), only to know a key exists.
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub node_label: String,
}

/// Which trust source resolved a peer's public key in `resolve_public_key` —
/// carried into the audit trail so a Command-sourced trust decision (a new
/// path as of O9) is visibly distinguishable from service-content's own
/// long-standing direct registrations, not silently indistinguishable.
#[derive(Debug, Clone)]
pub enum ResolvedPeer {
    /// service-content's own direct `/v1/pair` registration.
    Direct(String),
    /// Trusted via Command's pairing record instead. `role` is Command's own
    /// `PairingRole` ("User" | "Admin" | "Interface") as a plain string.
    ViaCommand { public_key: String, role: String },
}

impl ResolvedPeer {
    pub fn public_key(&self) -> &str {
        match self {
            ResolvedPeer::Direct(k) => k,
            ResolvedPeer::ViaCommand { public_key, .. } => public_key,
        }
    }

    /// "direct" or "command" — for the audit trail, not used in any access
    /// decision (verify_capability's signature check is what actually gates
    /// access; this is purely observability).
    pub fn source_label(&self) -> &'static str {
        match self {
            ResolvedPeer::Direct(_) => "direct",
            ResolvedPeer::ViaCommand { .. } => "command",
        }
    }
}

/// Reads Command's `user-pairings.yaml`, tolerant of absence — this is an
/// optional, additional trust source, not a hard requirement. Any read or
/// parse failure returns an empty list with a warning rather than failing
/// service-content's own startup, since Command pairing is orthogonal to
/// service-content's primary function.
fn load_command_pairings(path: &Path) -> Vec<CommandPairedPeer> {
    if !path.exists() {
        return Vec::new();
    }
    match std::fs::read_to_string(path) {
        Ok(content) => match serde_yaml::from_str::<Vec<CommandPairedPeer>>(&content) {
            Ok(peers) => peers,
            Err(e) => {
                eprintln!("[pairing] failed to parse Command pairings at {path:?}: {e} — treating as empty, service-content's own pairings are unaffected");
                Vec::new()
            }
        },
        Err(e) => {
            eprintln!("[pairing] failed to read Command pairings at {path:?}: {e} — treating as empty, service-content's own pairings are unaffected");
            Vec::new()
        }
    }
}

// ── pairing store ─────────────────────────────────────────────────────────────

/// In-memory pairing registry backed by an append-only JSONL file.
///
/// Keyed by `public_key` (base64url). A second pairing attempt with the same
/// public key returns `already_paired`.
pub struct PairingStore {
    store_path: PathBuf,
    audit_path: PathBuf,
    by_pubkey: HashMap<String, PairingRecord>,
    /// Additional peers trusted via Command's own pairing record — see the
    /// module doc comment above `CommandPairedPeer`. Empty unless
    /// `COMMAND_USER_PAIRINGS_PATH` is set and the file exists/parses.
    command_pairings: Vec<CommandPairedPeer>,
}

impl PairingStore {
    /// Load existing pairings from disk. Creates the file if absent.
    pub fn load(graph_dir: &str) -> std::io::Result<Self> {
        let store_path = Path::new(graph_dir).join("pairing-store.jsonl");
        let audit_path = Path::new(graph_dir).join("pair-audit.jsonl");
        let mut by_pubkey = HashMap::new();

        if store_path.exists() {
            let content = std::fs::read_to_string(&store_path)?;
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Ok(rec) = serde_json::from_str::<PairingRecord>(line) {
                    by_pubkey.insert(rec.public_key.clone(), rec);
                }
            }
        }

        let command_pairings = std::env::var("COMMAND_USER_PAIRINGS_PATH")
            .map(|p| load_command_pairings(Path::new(&p)))
            .unwrap_or_default();

        Ok(Self {
            store_path,
            audit_path,
            by_pubkey,
            command_pairings,
        })
    }

    pub fn get(&self, public_key: &str) -> Option<&PairingRecord> {
        self.by_pubkey.get(public_key)
    }

    /// All paired records, for `GET /v1/pairs`.
    pub fn list(&self) -> Vec<PairingRecord> {
        self.by_pubkey.values().cloned().collect()
    }

    /// Resolve a peer's registered public key by the `from_instance` identifier
    /// presented in a forwarded `X-Foundry-Capability` header — matched against
    /// `issuer`, falling back to `node_label` (both populated at `/v1/pair` time).
    pub fn find_by_instance(&self, from_instance: &str) -> Option<&PairingRecord> {
        self.by_pubkey
            .values()
            .find(|r| r.issuer == from_instance || r.node_label == from_instance)
    }

    /// Resolve a peer's public key for `capability_gate`, checking
    /// service-content's own direct `/v1/pair` registrations first, then
    /// falling back to peers Command has paired (O9 credential-chain
    /// wiring — see the module doc comment above `CommandPairedPeer`).
    /// `capability_gate`'s own `verify_capability` call is what actually
    /// proves the caller holds this key's private half; this method only
    /// resolves which key to check against — the returned `ResolvedPeer`
    /// also records which trust source answered, so the audit trail can
    /// tell the two apart (a new, previously-impossible trust path is worth
    /// being able to see, not just silently accept).
    pub fn resolve_public_key(&self, from_instance: &str) -> Option<ResolvedPeer> {
        if let Some(rec) = self.find_by_instance(from_instance) {
            return Some(ResolvedPeer::Direct(rec.public_key.clone()));
        }
        self.command_pairings
            .iter()
            .find(|p| p.node_label == from_instance)
            .map(|p| ResolvedPeer::ViaCommand {
                public_key: p.public_key.clone(),
                role: p.role.clone(),
            })
    }

    /// Persist a new pairing and return it.
    pub fn insert(&mut self, rec: PairingRecord) -> std::io::Result<()> {
        let line = serde_json::to_string(&rec).map_err(std::io::Error::other)?;

        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.store_path)?;
        writeln!(f, "{}", line)?;

        let audit = serde_json::json!({
            "event": "paired",
            "ts": Utc::now().to_rfc3339(),
            "issuer": rec.issuer,
            "peer_type": rec.peer_type,
            "role": rec.role,
            "node_label": rec.node_label,
            "nonce": rec.nonce,
        });
        let mut af = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.audit_path)?;
        writeln!(af, "{}", audit)?;

        self.by_pubkey.insert(rec.public_key.clone(), rec);
        Ok(())
    }
}

// ── totebox keypair (for token issuance) ──────────────────────────────────────

/// Persistent Ed25519 keypair for this Totebox instance.
///
/// The 32-byte seed is stored at `$GRAPH_DIR/totebox-pair.seed` so the public
/// key is stable across restarts (partners can cache it).
pub struct PairingKeypair {
    signing_key: SigningKey,
    pub verifying_key_b64: String,
}

impl PairingKeypair {
    /// Load from disk, or generate + save if not present.
    pub fn load_or_generate(graph_dir: &str) -> std::io::Result<Self> {
        let seed_path = Path::new(graph_dir).join("totebox-pair.seed");
        let seed: [u8; 32] = if seed_path.exists() {
            let bytes = std::fs::read(&seed_path)?;
            bytes.try_into().map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "bad seed length")
            })?
        } else {
            let mut s = [0u8; 32];
            let mut f = std::fs::File::open("/dev/urandom")?;
            use std::io::Read as _;
            f.read_exact(&mut s)?;
            std::fs::write(&seed_path, s)?;
            s
        };

        let signing_key = SigningKey::from_bytes(&seed);
        let vk = signing_key.verifying_key();
        let verifying_key_b64 = URL_SAFE_NO_PAD.encode(vk.as_bytes());

        Ok(Self {
            signing_key,
            verifying_key_b64,
        })
    }

    /// Issue a signed invite token for the given role and archive scope.
    pub fn issue_token(&self, role: &str, archive_scope: Vec<String>, node_label: &str) -> String {
        let nonce = {
            let mut b = [0u8; 16];
            if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
                use std::io::Read as _;
                let _ = f.read_exact(&mut b);
            }
            format!(
                "{:x}{:x}",
                u64::from_le_bytes(b[..8].try_into().unwrap_or_default()),
                u64::from_le_bytes(b[8..].try_into().unwrap_or_default())
            )
        };

        let payload = TokenPayload {
            issuer: node_label.to_string(),
            role: role.to_string(),
            nonce,
            expiry: (Utc::now() + chrono::Duration::hours(24)).to_rfc3339(),
            archive_scope,
            peer_type: "totebox".to_string(),
        };

        let payload_json = serde_json::to_string(&payload).expect("always serializable");
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());

        use ed25519_dalek::Signer as _;
        let sig: Signature = self.signing_key.sign(payload_b64.as_bytes());
        let sig_b64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());

        format!("{}.{}", payload_b64, sig_b64)
    }
}

// ── capability assertions (Model B forwarded trust) ────────────────────────────
//
// Wire format agreed with project-orchestration 2026-06-30 (outbox
// project-totebox-20260630-design-ack-pairing-protocol):
//   X-Foundry-Capability: <base64url(payload_json)>.<base64url(sig)>
//   payload = { from_instance, user_scope, archive_scope, nonce, expiry, peer_type }
// Signed with the forwarding peer's own Ed25519 key — the same key registered
// in PairingStore at `/v1/pair` time. Model B: os-totebox verifies independently
// rather than trusting the forwarding instance's pairing alone (DOCTRINE: holds
// no archive keys).
//
// Extended 2026-07-18 (BRIEF-datagraph-tenant-isolation.md's "grant-vs-forward"
// carry-forward) with `forwarded_for`: `#[serde(default)]`, so existing senders
// that predate this field remain valid (absent = direct grant, unchanged
// behavior). `None` means the signing peer is asserting its OWN capability
// (a direct grant from its own `/v1/pair` registration); `Some(origin_instance)`
// means the peer is relaying a capability on behalf of a third instance it
// talked to. `capability_gate` only honors a forward when the signing peer's
// own `user_scope` is `"ADMIN"` — an ordinary paired peer cannot unilaterally
// claim to forward on behalf of someone with broader access than itself. No
// real sender uses this yet (`app-orchestration-graph`, the one place a real
// forward would occur, sends no capability at all today) — this is the
// receiver-side contract, validated with synthetic peers until a real sender
// exists.

/// Payload of a forwarded `X-Foundry-Capability` header.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityPayload {
    pub from_instance: String,
    pub user_scope: String,
    #[serde(default)]
    pub archive_scope: Vec<String>,
    pub nonce: String,
    pub expiry: String,
    #[serde(default)]
    pub peer_type: String,
    /// `None` = direct grant (signing peer's own capability). `Some(origin)` =
    /// forwarded on behalf of `origin` — only honored for `user_scope: "ADMIN"`
    /// signers (see module doc comment above).
    #[serde(default)]
    pub forwarded_for: Option<String>,
}

impl CapabilityPayload {
    pub fn is_expired(&self) -> bool {
        match self.expiry.parse::<DateTime<Utc>>() {
            Ok(exp) => Utc::now() > exp,
            Err(_) => true,
        }
    }
}

#[derive(Debug)]
pub enum CapabilityError {
    Malformed,
    BadSignature,
    Expired,
}

impl std::fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CapabilityError::Malformed => write!(f, "malformed capability header"),
            CapabilityError::BadSignature => write!(f, "invalid capability signature"),
            CapabilityError::Expired => write!(f, "capability expired"),
        }
    }
}

/// Verify an `X-Foundry-Capability` header value against a known public key.
///
/// Caller resolves `from_instance` → `public_key` via
/// [`PairingStore::find_by_instance`] before calling this, and separately
/// checks nonce uniqueness via [`NonceCache`].
pub fn verify_capability(
    token: &str,
    public_key_b64: &str,
) -> Result<CapabilityPayload, CapabilityError> {
    let (payload_b64, sig_b64) = token.split_once('.').ok_or(CapabilityError::Malformed)?;

    let pk_bytes = URL_SAFE_NO_PAD
        .decode(public_key_b64)
        .map_err(|_| CapabilityError::Malformed)?;
    let pk_arr: [u8; 32] = pk_bytes
        .try_into()
        .map_err(|_| CapabilityError::Malformed)?;
    let vk = VerifyingKey::from_bytes(&pk_arr).map_err(|_| CapabilityError::Malformed)?;

    let sig_bytes = URL_SAFE_NO_PAD
        .decode(sig_b64)
        .map_err(|_| CapabilityError::Malformed)?;
    let sig_arr: [u8; 64] = sig_bytes
        .try_into()
        .map_err(|_| CapabilityError::Malformed)?;
    let sig = Signature::from_bytes(&sig_arr);

    use ed25519_dalek::Verifier as _;
    vk.verify(payload_b64.as_bytes(), &sig)
        .map_err(|_| CapabilityError::BadSignature)?;

    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| CapabilityError::Malformed)?;
    let payload: CapabilityPayload =
        serde_json::from_slice(&payload_bytes).map_err(|_| CapabilityError::Malformed)?;

    if payload.is_expired() {
        return Err(CapabilityError::Expired);
    }

    Ok(payload)
}

/// Append-only WORM audit log for capability-gated INTERFACE-peer requests.
///
/// Interim sidecar until the service-fs WORM drop dir is wired
/// (BRIEF-flow-build-plan.md §INTERFACE middleware).
pub struct InterfaceAuditLog {
    path: PathBuf,
}

impl InterfaceAuditLog {
    pub fn new(graph_dir: &str) -> Self {
        Self {
            path: Path::new(graph_dir).join("interface-audit.jsonl"),
        }
    }

    /// `trust_source` is `"direct"` (service-content's own `/v1/pair`) or
    /// `"command"` (resolved via Command's pairing record — O9 credential-
    /// chain wiring) — see `ResolvedPeer::source_label`. Purely observability;
    /// `verify_capability`'s signature check already gated access before this
    /// is ever called.
    pub fn record(
        &self,
        endpoint: &str,
        payload: &CapabilityPayload,
        trust_source: &str,
    ) -> std::io::Result<()> {
        let entry = serde_json::json!({
            "event": "capability_verified",
            "ts": Utc::now().to_rfc3339(),
            "endpoint": endpoint,
            "from_instance": payload.from_instance,
            "user_scope": payload.user_scope,
            "archive_scope": payload.archive_scope,
            "peer_type": payload.peer_type,
            "nonce": payload.nonce,
            "trust_source": trust_source,
        });
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        writeln!(f, "{}", entry)
    }
}

// ── nonce cache ───────────────────────────────────────────────────────────────

/// In-memory nonce deduplication. Prevents replay within the process lifetime.
///
/// Not persisted — nonces are tied to short-lived tokens (24h default).
/// After restart the window is narrow enough to be acceptable.
pub struct NonceCache(pub Mutex<HashSet<String>>);

impl Default for NonceCache {
    fn default() -> Self {
        Self::new()
    }
}

impl NonceCache {
    pub fn new() -> Self {
        Self(Mutex::new(HashSet::new()))
    }

    /// Returns false if the nonce was already seen.
    pub fn try_insert(&self, nonce: &str) -> bool {
        self.0.lock().unwrap().insert(nonce.to_string())
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_dir() -> tempfile::TempDir {
        tempfile::TempDir::new().expect("tmpdir")
    }

    fn make_keypair(dir: &str) -> PairingKeypair {
        PairingKeypair::load_or_generate(dir).expect("keypair")
    }

    #[test]
    fn issue_and_verify_roundtrip() {
        let d = tmp_dir();
        let kp = make_keypair(d.path().to_str().unwrap());
        let token = kp.issue_token("INTERFACE", vec!["project-totebox".into()], "test-node");
        let payload = verify_pair_token(&token, &kp.verifying_key_b64).expect("valid");
        assert_eq!(payload.role, "INTERFACE");
        assert_eq!(payload.peer_type, "totebox");
        assert!(!payload.is_expired());
    }

    #[test]
    fn tampered_payload_rejected() {
        let d = tmp_dir();
        let kp = make_keypair(d.path().to_str().unwrap());
        let token = kp.issue_token("USER", vec![], "node");
        let tampered = token.replacen('a', "b", 1);
        assert!(verify_pair_token(&tampered, &kp.verifying_key_b64).is_err());
    }

    #[test]
    fn wrong_key_rejected() {
        let d = tmp_dir();
        let kp1 = make_keypair(d.path().to_str().unwrap());
        let d2 = tmp_dir();
        let kp2 = make_keypair(d2.path().to_str().unwrap());
        let token = kp1.issue_token("ADMIN", vec![], "node");
        assert!(verify_pair_token(&token, &kp2.verifying_key_b64).is_err());
    }

    #[test]
    fn expired_token_rejected() {
        let d = tmp_dir();
        let kp = make_keypair(d.path().to_str().unwrap());
        // Manually craft a token with past expiry.
        let payload = TokenPayload {
            issuer: "test".into(),
            role: "USER".into(),
            nonce: "abc".into(),
            expiry: "2020-01-01T00:00:00Z".into(),
            archive_scope: vec![],
            peer_type: "orchestration".into(),
        };
        let pj = serde_json::to_string(&payload).unwrap();
        let pb64 = URL_SAFE_NO_PAD.encode(pj.as_bytes());
        use ed25519_dalek::Signer as _;
        let sig: Signature = kp.signing_key.sign(pb64.as_bytes());
        let sb64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());
        let token = format!("{}.{}", pb64, sb64);
        assert!(matches!(
            verify_pair_token(&token, &kp.verifying_key_b64),
            Err(PairError::Expired)
        ));
    }

    #[test]
    fn nonce_cache_deduplicates() {
        let cache = NonceCache::new();
        assert!(cache.try_insert("nonce-1"));
        assert!(!cache.try_insert("nonce-1"));
        assert!(cache.try_insert("nonce-2"));
    }

    #[test]
    fn pairing_store_roundtrip() {
        let d = tmp_dir();
        let dir = d.path().to_str().unwrap();
        let mut store = PairingStore::load(dir).expect("load");
        assert!(store.get("pk1").is_none());

        let rec = PairingRecord {
            public_key: "pk1".into(),
            issuer: "test-issuer".into(),
            peer_type: "orchestration".into(),
            role: "INTERFACE".into(),
            archive_scope: vec!["project-totebox".into()],
            node_label: "test-node".into(),
            paired_on: Utc::now().to_rfc3339(),
            nonce: "n1".into(),
        };
        store.insert(rec.clone()).expect("insert");
        assert!(store.get("pk1").is_some());

        // Reload from disk — record must survive.
        let store2 = PairingStore::load(dir).expect("reload");
        let loaded = store2.get("pk1").expect("persisted");
        assert_eq!(loaded.role, "INTERFACE");
    }

    #[test]
    fn pairing_store_list_returns_all_records() {
        let d = tmp_dir();
        let dir = d.path().to_str().unwrap();
        let mut store = PairingStore::load(dir).expect("load");
        assert!(store.list().is_empty());

        for i in 0..3 {
            let rec = PairingRecord {
                public_key: format!("pk{i}"),
                issuer: "test-issuer".into(),
                peer_type: "orchestration".into(),
                role: "INTERFACE".into(),
                archive_scope: vec!["project-totebox".into()],
                node_label: format!("node-{i}"),
                paired_on: Utc::now().to_rfc3339(),
                nonce: format!("n{i}"),
            };
            store.insert(rec).expect("insert");
        }

        let listed = store.list();
        assert_eq!(listed.len(), 3);
    }

    fn make_capability_token(
        kp: &PairingKeypair,
        from_instance: &str,
        expiry: DateTime<Utc>,
    ) -> String {
        let payload = CapabilityPayload {
            from_instance: from_instance.into(),
            user_scope: "jennifer".into(),
            archive_scope: vec!["project-orchestration".into()],
            nonce: "cap-nonce-1".into(),
            expiry: expiry.to_rfc3339(),
            peer_type: "orchestration".into(),
            forwarded_for: None,
        };
        let pj = serde_json::to_string(&payload).unwrap();
        let pb64 = URL_SAFE_NO_PAD.encode(pj.as_bytes());
        use ed25519_dalek::Signer as _;
        let sig: Signature = kp.signing_key.sign(pb64.as_bytes());
        let sb64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());
        format!("{}.{}", pb64, sb64)
    }

    #[test]
    fn capability_roundtrip_verifies() {
        let d = tmp_dir();
        let kp = make_keypair(d.path().to_str().unwrap());
        let token = make_capability_token(
            &kp,
            "project-orchestration",
            Utc::now() + chrono::Duration::hours(1),
        );
        let payload = verify_capability(&token, &kp.verifying_key_b64).expect("valid capability");
        assert_eq!(payload.from_instance, "project-orchestration");
        assert_eq!(payload.user_scope, "jennifer");
    }

    #[test]
    fn capability_expired_rejected() {
        let d = tmp_dir();
        let kp = make_keypair(d.path().to_str().unwrap());
        let token = make_capability_token(
            &kp,
            "project-orchestration",
            Utc::now() - chrono::Duration::hours(1),
        );
        assert!(matches!(
            verify_capability(&token, &kp.verifying_key_b64),
            Err(CapabilityError::Expired)
        ));
    }

    #[test]
    fn capability_wrong_key_rejected() {
        let d = tmp_dir();
        let kp1 = make_keypair(d.path().to_str().unwrap());
        let d2 = tmp_dir();
        let kp2 = make_keypair(d2.path().to_str().unwrap());
        let token = make_capability_token(
            &kp1,
            "project-orchestration",
            Utc::now() + chrono::Duration::hours(1),
        );
        assert!(verify_capability(&token, &kp2.verifying_key_b64).is_err());
    }

    #[test]
    fn find_by_instance_matches_issuer_or_label() {
        let d = tmp_dir();
        let dir = d.path().to_str().unwrap();
        let mut store = PairingStore::load(dir).expect("load");
        let rec = PairingRecord {
            public_key: "pk-orch".into(),
            issuer: "project-orchestration".into(),
            peer_type: "orchestration".into(),
            role: "INTERFACE".into(),
            archive_scope: vec![],
            node_label: "orch-node-1".into(),
            paired_on: Utc::now().to_rfc3339(),
            nonce: "n1".into(),
        };
        store.insert(rec).expect("insert");

        assert_eq!(
            store
                .find_by_instance("project-orchestration")
                .unwrap()
                .public_key,
            "pk-orch"
        );
        assert_eq!(
            store.find_by_instance("orch-node-1").unwrap().public_key,
            "pk-orch"
        );
        assert!(store.find_by_instance("unknown-instance").is_none());
    }

    #[test]
    fn interface_audit_log_appends_jsonl() {
        let d = tmp_dir();
        let log = InterfaceAuditLog::new(d.path().to_str().unwrap());
        let payload = CapabilityPayload {
            from_instance: "project-orchestration".into(),
            user_scope: "jennifer".into(),
            archive_scope: vec!["project-totebox".into()],
            nonce: "n1".into(),
            expiry: (Utc::now() + chrono::Duration::hours(1)).to_rfc3339(),
            peer_type: "orchestration".into(),
            forwarded_for: None,
        };
        log.record("/v1/graph/mutate", &payload, "direct").expect("record");
        let content = std::fs::read_to_string(d.path().join("interface-audit.jsonl")).unwrap();
        assert_eq!(content.lines().count(), 1);
        assert!(content.contains("capability_verified"));
        assert!(content.contains("project-orchestration"));
        assert!(content.contains("\"trust_source\":\"direct\""));
    }

    #[test]
    fn keypair_seed_stable_across_reload() {
        let d = tmp_dir();
        let dir = d.path().to_str().unwrap();
        let kp1 = make_keypair(dir);
        let kp2 = PairingKeypair::load_or_generate(dir).expect("reload");
        assert_eq!(kp1.verifying_key_b64, kp2.verifying_key_b64);
    }

    // ── O9 credential-chain wiring ──────────────────────────────────────────

    #[test]
    fn load_command_pairings_missing_file_returns_empty() {
        let d = tmp_dir();
        let missing = d.path().join("does-not-exist.yaml");
        assert!(load_command_pairings(&missing).is_empty());
    }

    #[test]
    fn load_command_pairings_malformed_yaml_returns_empty() {
        let d = tmp_dir();
        let path = d.path().join("user-pairings.yaml");
        std::fs::write(&path, "not: [valid, yaml: structure for Vec<CommandPairedPeer>")
            .expect("write");
        assert!(load_command_pairings(&path).is_empty());
    }

    #[test]
    fn load_command_pairings_parses_valid_entries() {
        let d = tmp_dir();
        let path = d.path().join("user-pairings.yaml");
        std::fs::write(
            &path,
            "- public_key: pk-cmd-1\n  role: ADMIN\n  node_label: console-1\n\
             - public_key: pk-cmd-2\n  role: USER\n  node_label: console-2\n",
        )
        .expect("write");
        let peers = load_command_pairings(&path);
        assert_eq!(peers.len(), 2);
        assert_eq!(peers[0].public_key, "pk-cmd-1");
        assert_eq!(peers[0].role, "ADMIN");
        assert_eq!(peers[0].node_label, "console-1");
        assert_eq!(peers[1].node_label, "console-2");
    }

    #[test]
    fn load_command_pairings_tolerates_unknown_fields() {
        let d = tmp_dir();
        let path = d.path().join("user-pairings.yaml");
        // Command's real record shape carries fields service-content doesn't
        // need (e.g. paired_on, expiry) — must be ignored, not a parse error.
        std::fs::write(
            &path,
            "- public_key: pk-cmd-1\n  role: ADMIN\n  node_label: console-1\n  paired_on: '2026-07-30T00:00:00Z'\n  something_else: 42\n",
        )
        .expect("write");
        let peers = load_command_pairings(&path);
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].public_key, "pk-cmd-1");
    }

    fn store_with_command_pairings(
        dir: &tempfile::TempDir,
        command_pairings: Vec<CommandPairedPeer>,
    ) -> PairingStore {
        let mut store = PairingStore::load(dir.path().to_str().unwrap()).expect("load");
        store.command_pairings = command_pairings;
        store
    }

    #[test]
    fn resolve_public_key_falls_back_to_command_pairing() {
        let d = tmp_dir();
        let store = store_with_command_pairings(
            &d,
            vec![CommandPairedPeer {
                public_key: "pk-cmd".into(),
                role: "ADMIN".into(),
                node_label: "console-1".into(),
            }],
        );
        let resolved = store.resolve_public_key("console-1").expect("resolved via command");
        assert_eq!(resolved.public_key(), "pk-cmd");
        assert_eq!(resolved.source_label(), "command");
        assert!(matches!(resolved, ResolvedPeer::ViaCommand { .. }));
    }

    #[test]
    fn resolve_public_key_prefers_direct_registration_over_command() {
        let d = tmp_dir();
        let mut store = store_with_command_pairings(
            &d,
            vec![CommandPairedPeer {
                public_key: "pk-cmd".into(),
                role: "ADMIN".into(),
                node_label: "same-node".into(),
            }],
        );
        store
            .insert(PairingRecord {
                public_key: "pk-direct".into(),
                issuer: "same-node".into(),
                peer_type: "totebox".into(),
                role: "INTERFACE".into(),
                archive_scope: vec!["project-totebox".into()],
                node_label: "same-node".into(),
                paired_on: Utc::now().to_rfc3339(),
                nonce: "n1".into(),
            })
            .expect("insert");

        let resolved = store.resolve_public_key("same-node").expect("resolved");
        assert_eq!(resolved.public_key(), "pk-direct");
        assert_eq!(resolved.source_label(), "direct");
        assert!(matches!(resolved, ResolvedPeer::Direct(_)));
    }

    #[test]
    fn resolve_public_key_none_when_peer_unknown_to_either_source() {
        let d = tmp_dir();
        let store = store_with_command_pairings(&d, vec![]);
        assert!(store.resolve_public_key("nobody").is_none());
    }

    #[test]
    fn pairing_store_load_reads_command_pairings_path_env_var() {
        let store_dir = tmp_dir();
        let command_dir = tmp_dir();
        let command_path = command_dir.path().join("user-pairings.yaml");
        std::fs::write(
            &command_path,
            "- public_key: pk-env\n  role: ADMIN\n  node_label: env-node\n",
        )
        .expect("write");

        // SAFETY / isolation: no other test in this module reads or writes
        // COMMAND_USER_PAIRINGS_PATH, so this does not race with sibling
        // tests despite cargo test's default multi-threaded execution.
        std::env::set_var("COMMAND_USER_PAIRINGS_PATH", &command_path);
        let store = PairingStore::load(store_dir.path().to_str().unwrap()).expect("load");
        std::env::remove_var("COMMAND_USER_PAIRINGS_PATH");

        let resolved = store.resolve_public_key("env-node").expect("resolved via env-configured path");
        assert_eq!(resolved.public_key(), "pk-env");
        assert_eq!(resolved.source_label(), "command");
    }
}
