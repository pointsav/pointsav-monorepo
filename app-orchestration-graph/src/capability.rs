// SPDX-License-Identifier: LicenseRef-PointSav-ARR
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//
// This file is proprietary material of Woodfine Capital Projects Inc.
// See the LICENSE file in this repository for the full terms.
// Unauthorized use, reproduction, or distribution is prohibited.

//! This instance's own Ed25519 identity, `/v1/pair` registration, and
//! `X-Foundry-Capability` header construction for fan-out calls to target
//! `service-content` instances.
//!
//! Wire formats match `service-content/src/pairing.rs` exactly (independently
//! reimplemented here, not a shared dependency — this crate is deliberately
//! standalone, matching its existing architectural separation from
//! `service-content`). Ships as a **direct grant** only
//! (`forwarded_for: None`) — this instance doesn't yet authenticate its own
//! inbound callers, so it has no real "on behalf of X" identity to relay; see
//! `BRIEF-datagraph-tenant-isolation.md` Session 4 for the scope reasoning.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use chrono::Utc;
use ed25519_dalek::{Signature, Signer as _, SigningKey, VerifyingKey};
use serde::Serialize;
use std::path::{Path, PathBuf};

/// `/v1/pair` token payload — matches `service-content::pairing::TokenPayload`.
#[derive(Debug, Serialize)]
struct TokenPayload {
    issuer: String,
    role: String,
    nonce: String,
    expiry: String,
    archive_scope: Vec<String>,
    peer_type: String,
}

/// `X-Foundry-Capability` header payload — matches
/// `service-content::pairing::CapabilityPayload`.
#[derive(Debug, Serialize)]
struct CapabilityPayload {
    from_instance: String,
    user_scope: String,
    archive_scope: Vec<String>,
    nonce: String,
    expiry: String,
    peer_type: String,
    forwarded_for: Option<String>,
}

/// `POST /v1/pair` request body — matches `service-content::http::PairRequest`.
#[derive(Debug, Serialize)]
pub struct PairRequest {
    pub token: String,
    pub public_key: String,
    pub node_label: String,
}

pub const INSTANCE_LABEL: &str = "app-orchestration-graph";

pub struct Identity {
    signing_key: SigningKey,
    pub verifying_key_b64: String,
}

impl Identity {
    /// Load a persisted seed from `seed_path`, or generate + save a fresh one.
    pub fn load_or_generate(seed_path: &Path) -> std::io::Result<Self> {
        let seed: [u8; 32] = if seed_path.exists() {
            let bytes = std::fs::read(seed_path)?;
            bytes
                .try_into()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "bad seed length"))?
        } else {
            let mut s = [0u8; 32];
            let mut f = std::fs::File::open("/dev/urandom")?;
            use std::io::Read as _;
            f.read_exact(&mut s)?;
            if let Some(parent) = seed_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(seed_path, s)?;
            s
        };
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key_b64 = URL_SAFE_NO_PAD.encode(signing_key.verifying_key().as_bytes());
        Ok(Self {
            signing_key,
            verifying_key_b64,
        })
    }

    #[cfg(test)]
    pub fn from_seed_byte(b: u8) -> Self {
        let signing_key = SigningKey::from_bytes(&[b; 32]);
        let verifying_key_b64 = URL_SAFE_NO_PAD.encode(signing_key.verifying_key().as_bytes());
        Self {
            signing_key,
            verifying_key_b64,
        }
    }

    #[cfg(test)]
    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    fn sign_and_encode(&self, payload_json: &str) -> String {
        let pb64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());
        let sig: Signature = self.signing_key.sign(pb64.as_bytes());
        let sb64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());
        format!("{pb64}.{sb64}")
    }

    /// Build a `POST /v1/pair` request registering this instance with a
    /// target `service-content` as an ADMIN-role peer scoped to `module_id`.
    pub fn build_pair_request(&self, module_id: &str, nonce: &str) -> PairRequest {
        let payload = TokenPayload {
            issuer: INSTANCE_LABEL.to_string(),
            role: "ADMIN".to_string(),
            nonce: nonce.to_string(),
            expiry: (Utc::now() + chrono::Duration::days(365)).to_rfc3339(),
            archive_scope: vec![module_id.to_string()],
            peer_type: "orchestration".to_string(),
        };
        let payload_json = serde_json::to_string(&payload).expect("always serializable");
        PairRequest {
            token: self.sign_and_encode(&payload_json),
            public_key: self.verifying_key_b64.clone(),
            node_label: INSTANCE_LABEL.to_string(),
        }
    }

    /// Build a signed `X-Foundry-Capability` header value for a fan-out call
    /// targeting `module_id`. Always a direct grant (`forwarded_for: None`)
    /// — see module doc comment for why.
    pub fn make_capability_header(&self, module_id: &str, nonce: &str) -> String {
        let payload = CapabilityPayload {
            from_instance: INSTANCE_LABEL.to_string(),
            user_scope: "ADMIN".to_string(),
            archive_scope: vec![module_id.to_string()],
            nonce: nonce.to_string(),
            expiry: (Utc::now() + chrono::Duration::minutes(5)).to_rfc3339(),
            peer_type: "orchestration".to_string(),
            forwarded_for: None,
        };
        let payload_json = serde_json::to_string(&payload).expect("always serializable");
        self.sign_and_encode(&payload_json)
    }
}

/// Nanosecond-timestamp-based nonce — no `rand`/`uuid` dependency needed;
/// uniqueness only needs to hold within this process's own request stream.
pub fn fresh_nonce() -> String {
    format!(
        "og-{}",
        Utc::now()
            .timestamp_nanos_opt()
            .unwrap_or(0)
    )
}

/// Default identity seed path: `$ORCHESTRATION_GRAPH_STATE_DIR/identity.seed`,
/// else `/var/lib/app-orchestration-graph/identity.seed`.
pub fn default_seed_path() -> PathBuf {
    let base = std::env::var("ORCHESTRATION_GRAPH_STATE_DIR")
        .unwrap_or_else(|_| "/var/lib/app-orchestration-graph".to_string());
    Path::new(&base).join("identity.seed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_from_seed_byte_is_deterministic() {
        let a = Identity::from_seed_byte(7);
        let b = Identity::from_seed_byte(7);
        assert_eq!(a.verifying_key_b64, b.verifying_key_b64);
    }

    #[test]
    fn pair_request_is_signed_and_well_formed() {
        let id = Identity::from_seed_byte(1);
        let req = id.build_pair_request("pointsav", "nonce-1");
        assert_eq!(req.public_key, id.verifying_key_b64);
        assert_eq!(req.node_label, INSTANCE_LABEL);
        assert!(req.token.contains('.'), "token must be payload.sig format");

        // Verify the signature actually verifies against the claimed key.
        let (payload_b64, sig_b64) = req.token.split_once('.').unwrap();
        let sig_bytes = URL_SAFE_NO_PAD.decode(sig_b64).unwrap();
        let sig_arr: [u8; 64] = sig_bytes.try_into().unwrap();
        let sig = Signature::from_bytes(&sig_arr);
        use ed25519_dalek::Verifier as _;
        assert!(id.verifying_key().verify(payload_b64.as_bytes(), &sig).is_ok());
    }

    #[test]
    fn capability_header_is_signed_direct_grant_not_forward() {
        let id = Identity::from_seed_byte(2);
        let header = id.make_capability_header("pointsav", "nonce-2");
        let (payload_b64, _) = header.split_once('.').unwrap();
        let payload_bytes = URL_SAFE_NO_PAD.decode(payload_b64).unwrap();
        let payload: serde_json::Value = serde_json::from_slice(&payload_bytes).unwrap();
        assert_eq!(payload["from_instance"], INSTANCE_LABEL);
        assert_eq!(payload["user_scope"], "ADMIN");
        assert_eq!(payload["archive_scope"], serde_json::json!(["pointsav"]));
        assert!(
            payload["forwarded_for"].is_null(),
            "must ship as a direct grant, not a forward claim, per Session 4 scope decision"
        );
    }

    #[test]
    fn fresh_nonce_is_unique_across_calls() {
        let a = fresh_nonce();
        let b = fresh_nonce();
        assert_ne!(a, b);
    }
}
