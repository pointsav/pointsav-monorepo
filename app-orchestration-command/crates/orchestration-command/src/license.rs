// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! License verification — the commercial gate for CommandCentre.
//!
//! Follows the same wire format as `orchestration-slm/src/license.rs`:
//!   `<base64url(payload_json)>.<base64url(ed25519_signature)>`
//!
//! The signature is over the raw bytes of the first segment (encoded payload).
//! Verification is fully offline; no network call is made.
//!
//! An absent or invalid license does NOT prevent the server from starting.
//! The server runs in observation mode (read-only fleet/personnel endpoints
//! only); invite and pairing endpoints return 402 until a valid license is
//! supplied via `COMMAND_LICENSE_TOKEN`.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use chrono::{DateTime, Duration, Utc};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};

use crate::error::CommandError;
use orchestration_command_core::REQUIRED_PRODUCT;

/// Days past `expiry` during which the license is still honored.
pub const GRACE_DAYS: i64 = 30;

/// The decoded, signature-verified payload of a CommandCentre license token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePayload {
    pub product: String,
    #[serde(default)]
    pub issued_to: String,
    pub expiry: DateTime<Utc>,
    #[serde(default)]
    pub entitlements: Vec<String>,
}

impl LicensePayload {
    pub fn has(&self, entitlement: &str) -> bool {
        self.entitlements.iter().any(|e| e == entitlement)
    }

    pub fn is_current(&self, now: DateTime<Utc>) -> bool {
        now <= self.expiry + Duration::days(GRACE_DAYS)
    }

    pub fn in_grace(&self, now: DateTime<Utc>) -> bool {
        now > self.expiry && self.is_current(now)
    }
}

#[derive(Debug, Clone)]
pub enum LicenseStatus {
    Valid(LicensePayload),
    Absent,
    Invalid(String),
}

impl LicenseStatus {
    /// Invite and pairing endpoints require a valid license.
    pub fn permits_pairing(&self) -> bool {
        matches!(self, LicenseStatus::Valid(_))
    }

    pub fn label(&self) -> &'static str {
        match self {
            LicenseStatus::Valid(_) => "valid",
            LicenseStatus::Absent => "absent",
            LicenseStatus::Invalid(_) => "invalid",
        }
    }
}

pub fn verify_token(token: &str, public_key: &[u8; 32]) -> Result<LicensePayload, CommandError> {
    verify_token_at(token, public_key, Utc::now())
}

pub fn verify_token_at(
    token: &str,
    public_key: &[u8; 32],
    now: DateTime<Utc>,
) -> Result<LicensePayload, CommandError> {
    let (payload_b64, sig_b64) = token
        .split_once('.')
        .ok_or_else(|| CommandError::License("token is not <payload>.<signature>".into()))?;

    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|e| CommandError::License(format!("payload base64: {e}")))?;
    let sig_bytes = URL_SAFE_NO_PAD
        .decode(sig_b64)
        .map_err(|e| CommandError::License(format!("signature base64: {e}")))?;

    let verifying_key = VerifyingKey::from_bytes(public_key)
        .map_err(|e| CommandError::License(format!("public key: {e}")))?;
    let signature = Signature::from_slice(&sig_bytes)
        .map_err(|e| CommandError::License(format!("signature shape: {e}")))?;

    verifying_key
        .verify_strict(payload_b64.as_bytes(), &signature)
        .map_err(|_| CommandError::License("signature does not verify".into()))?;

    let payload: LicensePayload = serde_json::from_slice(&payload_bytes)
        .map_err(|e| CommandError::License(format!("payload json: {e}")))?;

    if payload.product != REQUIRED_PRODUCT {
        return Err(CommandError::License(format!(
            "license is for product '{}', not '{REQUIRED_PRODUCT}'",
            payload.product
        )));
    }
    if !payload.is_current(now) {
        return Err(CommandError::License(format!(
            "license expired {} (beyond {GRACE_DAYS}-day grace)",
            payload.expiry.to_rfc3339()
        )));
    }

    Ok(payload)
}

pub fn resolve_from_env(embedded_pubkey: &[u8; 32]) -> LicenseStatus {
    let token = match std::env::var("COMMAND_LICENSE_TOKEN") {
        Ok(t) if !t.trim().is_empty() => t,
        _ => return LicenseStatus::Absent,
    };
    let pubkey = match std::env::var("COMMAND_LICENSE_PUBKEY_HEX") {
        Ok(hex) if !hex.trim().is_empty() => match decode_pubkey_hex(hex.trim()) {
            Ok(k) => k,
            Err(e) => return LicenseStatus::Invalid(format!("pubkey hex: {e}")),
        },
        _ => *embedded_pubkey,
    };
    match verify_token(&token, &pubkey) {
        Ok(payload) => LicenseStatus::Valid(payload),
        Err(CommandError::License(reason)) => LicenseStatus::Invalid(reason),
        Err(e) => LicenseStatus::Invalid(e.to_string()),
    }
}

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
    use chrono::Duration;
    use ed25519_dalek::{Signer, SigningKey};

    fn mint(payload: &LicensePayload, signing: &SigningKey) -> String {
        let json = serde_json::to_vec(payload).unwrap();
        let payload_b64 = URL_SAFE_NO_PAD.encode(json);
        let sig = signing.sign(payload_b64.as_bytes());
        let sig_b64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());
        format!("{payload_b64}.{sig_b64}")
    }

    fn keypair() -> (SigningKey, [u8; 32]) {
        let mut secret = [0u8; 32];
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
            issued_to: "Woodfine Management Corp.".into(),
            expiry: Utc::now() + Duration::days(365),
            entitlements: vec!["command-pairing".into()],
        }
    }

    #[test]
    fn valid_token_verifies() {
        let (signing, pubkey) = keypair();
        let token = mint(&valid_payload(), &signing);
        let payload = verify_token(&token, &pubkey).unwrap();
        assert_eq!(payload.product, REQUIRED_PRODUCT);
        assert!(payload.has("command-pairing"));
    }

    #[test]
    fn tampered_payload_fails() {
        let (signing, pubkey) = keypair();
        let token = mint(&valid_payload(), &signing);
        let (p, s) = token.split_once('.').unwrap();
        let mut p = p.to_string();
        let last = p.pop().unwrap();
        p.push(if last == 'A' { 'B' } else { 'A' });
        assert!(verify_token(&format!("{p}.{s}"), &pubkey).is_err());
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
        p.expiry = now - Duration::days(5);
        let token = mint(&p, &signing);
        assert!(verify_token_at(&token, &pubkey, now).is_ok());
    }
}
