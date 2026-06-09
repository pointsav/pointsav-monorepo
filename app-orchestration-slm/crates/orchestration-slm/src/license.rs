// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! License verification — the commercial gate for the proprietary chassis.
//!
//! `service-slm` (the per-archive Doorman) is free and open. This chassis —
//! which brokers a shared GPU fleet across multiple archives — is the paid
//! tier. A customer obtains a signed license token from the software
//! marketplace and supplies it via the `ORCHESTRATION_LICENSE_TOKEN`
//! environment variable. The chassis verifies the token's Ed25519 signature
//! against an embedded public key at startup; an invalid or expired token
//! disables Tier B brokering (the chassis still boots and serves `/healthz`,
//! but proxy routes return "license required").
//!
//! ## Token format
//!
//! A token is two base64url segments joined by a dot:
//!
//! ```text
//! <base64url(payload_json)>.<base64url(ed25519_signature)>
//! ```
//!
//! The signature is over the raw bytes of the first segment (the encoded
//! payload), matching the detached-signature convention used elsewhere in the
//! software-distribution stack. The payload is:
//!
//! ```json
//! {
//!   "product": "soft-slm-orchestration",
//!   "issued_to": "Woodfine Management Corp.",
//!   "expiry": "2027-06-04T00:00:00Z",
//!   "entitlements": ["tier-b-orchestration", "managed-fleet"]
//! }
//! ```
//!
//! ## Offline grace
//!
//! There is no network call. Verification is fully offline against the
//! embedded public key. The token carries its own expiry; a 30-day grace
//! window past `expiry` is permitted ([`GRACE_DAYS`]) so a brief lapse in
//! renewal does not hard-stop a production deployment — after the grace window
//! the license is treated as expired.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use chrono::{DateTime, Duration, Utc};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};

use crate::error::ChassisError;

/// Product identifier this chassis requires in a valid license.
pub const REQUIRED_PRODUCT: &str = "soft-slm-orchestration";

/// Days past `expiry` during which a license is still honored (renewal grace).
pub const GRACE_DAYS: i64 = 30;

/// The decoded, signature-verified payload of a license token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePayload {
    /// Product the license grants. Must equal [`REQUIRED_PRODUCT`].
    pub product: String,
    /// Human-readable licensee name (for logs and the `/readyz` panel).
    #[serde(default)]
    pub issued_to: String,
    /// Expiry timestamp (RFC 3339). A 30-day grace window applies past this.
    pub expiry: DateTime<Utc>,
    /// Capability flags granted by this license.
    #[serde(default)]
    pub entitlements: Vec<String>,
}

impl LicensePayload {
    /// True if `entitlement` is granted by this license.
    pub fn has(&self, entitlement: &str) -> bool {
        self.entitlements.iter().any(|e| e == entitlement)
    }

    /// True when the license is within its validity window (including grace).
    pub fn is_current(&self, now: DateTime<Utc>) -> bool {
        now <= self.expiry + Duration::days(GRACE_DAYS)
    }

    /// True when past expiry but still inside the grace window.
    pub fn in_grace(&self, now: DateTime<Utc>) -> bool {
        now > self.expiry && self.is_current(now)
    }
}

/// The result of a license check, used to gate Tier B brokering.
#[derive(Debug, Clone)]
pub enum LicenseStatus {
    /// A valid, current license. Tier B brokering is enabled.
    Valid(LicensePayload),
    /// No `ORCHESTRATION_LICENSE_TOKEN` was supplied. Tier B disabled; the
    /// chassis runs in community-observation mode (fleet listing + health
    /// only).
    Absent,
    /// A token was supplied but failed verification (bad signature, wrong
    /// product, expired beyond grace, malformed). Tier B disabled. The reason
    /// is carried for the operator log.
    Invalid(String),
}

impl LicenseStatus {
    /// True when Tier B brokering should be permitted.
    pub fn permits_tier_b(&self) -> bool {
        matches!(self, LicenseStatus::Valid(_))
    }

    /// A short label for the `/readyz` panel.
    pub fn label(&self) -> &'static str {
        match self {
            LicenseStatus::Valid(_) => "valid",
            LicenseStatus::Absent => "absent",
            LicenseStatus::Invalid(_) => "invalid",
        }
    }
}

/// Verify a license token string against the given Ed25519 public key (32
/// raw bytes). Returns the payload on success.
///
/// Verification steps: split on `.`; base64url-decode both segments; verify
/// the signature over the *encoded payload bytes*; decode the payload JSON;
/// check the product matches and the license is current (within grace).
pub fn verify_token(token: &str, public_key: &[u8; 32]) -> Result<LicensePayload, ChassisError> {
    verify_token_at(token, public_key, Utc::now())
}

/// Verification with an explicit `now` (for deterministic tests).
pub fn verify_token_at(
    token: &str,
    public_key: &[u8; 32],
    now: DateTime<Utc>,
) -> Result<LicensePayload, ChassisError> {
    let (payload_b64, sig_b64) = token
        .split_once('.')
        .ok_or_else(|| ChassisError::License("token is not <payload>.<signature>".into()))?;

    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|e| ChassisError::License(format!("payload base64: {e}")))?;
    let sig_bytes = URL_SAFE_NO_PAD
        .decode(sig_b64)
        .map_err(|e| ChassisError::License(format!("signature base64: {e}")))?;

    let verifying_key = VerifyingKey::from_bytes(public_key)
        .map_err(|e| ChassisError::License(format!("public key: {e}")))?;
    let signature = Signature::from_slice(&sig_bytes)
        .map_err(|e| ChassisError::License(format!("signature shape: {e}")))?;

    // The signature is over the base64 payload segment's raw bytes.
    verifying_key
        .verify_strict(payload_b64.as_bytes(), &signature)
        .map_err(|_| ChassisError::License("signature does not verify".into()))?;

    let payload: LicensePayload = serde_json::from_slice(&payload_bytes)
        .map_err(|e| ChassisError::License(format!("payload json: {e}")))?;

    if payload.product != REQUIRED_PRODUCT {
        return Err(ChassisError::License(format!(
            "license is for product '{}', not '{}'",
            payload.product, REQUIRED_PRODUCT
        )));
    }
    if !payload.is_current(now) {
        return Err(ChassisError::License(format!(
            "license expired {} (beyond {}-day grace)",
            payload.expiry.to_rfc3339(),
            GRACE_DAYS
        )));
    }

    Ok(payload)
}

/// Resolve the license at startup from the environment.
///
/// Reads `ORCHESTRATION_LICENSE_TOKEN`. The public key comes from
/// `ORCHESTRATION_LICENSE_PUBKEY_HEX` (64 hex chars) if set, else the embedded
/// default. Absent token → [`LicenseStatus::Absent`]; present-but-bad →
/// [`LicenseStatus::Invalid`]; valid → [`LicenseStatus::Valid`]. Never errors —
/// the chassis always boots.
pub fn resolve_from_env(embedded_pubkey: &[u8; 32]) -> LicenseStatus {
    let token = match std::env::var("ORCHESTRATION_LICENSE_TOKEN") {
        Ok(t) if !t.trim().is_empty() => t,
        _ => return LicenseStatus::Absent,
    };
    let pubkey = match std::env::var("ORCHESTRATION_LICENSE_PUBKEY_HEX") {
        Ok(hex) if !hex.trim().is_empty() => match decode_pubkey_hex(hex.trim()) {
            Ok(k) => k,
            Err(e) => return LicenseStatus::Invalid(format!("pubkey hex: {e}")),
        },
        _ => *embedded_pubkey,
    };
    match verify_token(&token, &pubkey) {
        Ok(payload) => LicenseStatus::Valid(payload),
        Err(ChassisError::License(reason)) => LicenseStatus::Invalid(reason),
        Err(e) => LicenseStatus::Invalid(e.to_string()),
    }
}

/// Decode a 64-char hex string into a 32-byte Ed25519 public key.
pub fn decode_pubkey_hex(hex: &str) -> Result<[u8; 32], String> {
    if hex.len() != 64 {
        return Err(format!("expected 64 hex chars, got {}", hex.len()));
    }
    let mut out = [0u8; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        let pair = &hex[i * 2..i * 2 + 2];
        *byte = u8::from_str_radix(pair, 16).map_err(|e| format!("hex pair '{pair}': {e}"))?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    /// Mint a signed token for tests using a freshly-generated key pair.
    fn mint(payload: &LicensePayload, signing: &SigningKey) -> String {
        let json = serde_json::to_vec(payload).unwrap();
        let payload_b64 = URL_SAFE_NO_PAD.encode(json);
        let sig = signing.sign(payload_b64.as_bytes());
        let sig_b64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());
        format!("{payload_b64}.{sig_b64}")
    }

    fn keypair() -> (SigningKey, [u8; 32]) {
        let mut secret = [0u8; 32];
        // Deterministic but non-trivial seed — fine for unit tests.
        for (i, b) in secret.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(7).wrapping_add(13);
        }
        let signing = SigningKey::from_bytes(&secret);
        let pubkey = signing.verifying_key().to_bytes();
        (signing, pubkey)
    }

    fn valid_payload() -> LicensePayload {
        LicensePayload {
            product: REQUIRED_PRODUCT.to_string(),
            issued_to: "Test Corp".into(),
            expiry: Utc::now() + Duration::days(365),
            entitlements: vec!["tier-b-orchestration".into(), "managed-fleet".into()],
        }
    }

    #[test]
    fn valid_token_verifies() {
        let (signing, pubkey) = keypair();
        let token = mint(&valid_payload(), &signing);
        let payload = verify_token(&token, &pubkey).unwrap();
        assert_eq!(payload.product, REQUIRED_PRODUCT);
        assert!(payload.has("tier-b-orchestration"));
    }

    #[test]
    fn tampered_payload_fails() {
        let (signing, pubkey) = keypair();
        let token = mint(&valid_payload(), &signing);
        // Flip a character in the payload segment.
        let (p, s) = token.split_once('.').unwrap();
        let mut p = p.to_string();
        let last = p.pop().unwrap();
        p.push(if last == 'A' { 'B' } else { 'A' });
        let tampered = format!("{p}.{s}");
        assert!(verify_token(&tampered, &pubkey).is_err());
    }

    #[test]
    fn wrong_key_fails() {
        let (signing, _pubkey) = keypair();
        let token = mint(&valid_payload(), &signing);
        // A different key must not verify.
        let other_secret = [9u8; 32];
        let other_pub = SigningKey::from_bytes(&other_secret)
            .verifying_key()
            .to_bytes();
        assert!(verify_token(&token, &other_pub).is_err());
    }

    #[test]
    fn wrong_product_rejected() {
        let (signing, pubkey) = keypair();
        let mut p = valid_payload();
        p.product = "some-other-product".into();
        let token = mint(&p, &signing);
        assert!(verify_token(&token, &pubkey).is_err());
    }

    #[test]
    fn expired_beyond_grace_rejected() {
        let (signing, pubkey) = keypair();
        let mut p = valid_payload();
        p.expiry = Utc::now() - Duration::days(GRACE_DAYS + 5);
        let token = mint(&p, &signing);
        assert!(verify_token(&token, &pubkey).is_err());
    }

    #[test]
    fn within_grace_accepted() {
        let (signing, pubkey) = keypair();
        let mut p = valid_payload();
        let now = Utc::now();
        p.expiry = now - Duration::days(5); // expired, but inside 30-day grace
        let token = mint(&p, &signing);
        let verified = verify_token_at(&token, &pubkey, now).unwrap();
        assert!(verified.in_grace(now));
        assert!(verified.is_current(now));
    }

    #[test]
    fn malformed_token_rejected() {
        let (_signing, pubkey) = keypair();
        assert!(verify_token("not-a-valid-token", &pubkey).is_err());
        assert!(verify_token("only-one-segment-no-dot", &pubkey).is_err());
    }

    #[test]
    fn status_permits_tier_b_only_when_valid() {
        let (signing, pubkey) = keypair();
        let token = mint(&valid_payload(), &signing);
        let payload = verify_token(&token, &pubkey).unwrap();
        assert!(LicenseStatus::Valid(payload).permits_tier_b());
        assert!(!LicenseStatus::Absent.permits_tier_b());
        assert!(!LicenseStatus::Invalid("x".into()).permits_tier_b());
    }

    #[test]
    fn pubkey_hex_round_trips() {
        let (_signing, pubkey) = keypair();
        let hex: String = pubkey.iter().map(|b| format!("{b:02x}")).collect();
        let decoded = decode_pubkey_hex(&hex).unwrap();
        assert_eq!(decoded, pubkey);
    }

    #[test]
    fn pubkey_hex_wrong_length_rejected() {
        assert!(decode_pubkey_hex("abcd").is_err());
    }
}
