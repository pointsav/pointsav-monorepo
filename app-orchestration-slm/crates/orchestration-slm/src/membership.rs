// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Signed membership tokens — short-lived Ed25519 credentials issued to
//! registered Totebox archives.
//!
//! Token format (same convention as `license.rs`):
//!   `<base64url(claims_json)>.<base64url(ed25519_signature_over_claims)>`
//!
//! The chassis generates an Ed25519 keypair at startup from OS entropy.
//! `issue()` signs a fresh token valid for one hour. `verify()` checks
//! the signature and expiry; it returns the embedded claims on success.
//!
//! The token is returned in `RegistrationResponseV2::membership_token` and
//! the Doorman supplies it as `Authorization: Bearer <token>` on proxy calls.
//! `authenticate_membership()` on `FleetRegistry` accepts this token as an
//! alternative to the plain bearer module-id.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use chrono::{DateTime, Duration, Utc};
use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::io::Read as _;

use crate::error::ChassisError;

/// How long a membership token remains valid.
const TOKEN_VALIDITY_HOURS: i64 = 1;

/// Claims embedded in a membership token payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipClaims {
    pub module_id: String,
    pub archive_id: String,
    /// UNIX timestamp (seconds UTC) when the token was issued.
    pub issued_at: i64,
    /// UNIX timestamp (seconds UTC) after which the token is invalid.
    pub expires_at: i64,
}

impl MembershipClaims {
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.expires_at
    }

    pub fn issued_at_dt(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.issued_at, 0)
    }

    pub fn expires_at_dt(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.expires_at, 0)
    }
}

/// Ed25519 signing keypair for membership token issuance and verification.
///
/// Generated at chassis startup from `/dev/urandom`. The verifying key is
/// kept alongside the signing key so that `verify()` is self-contained —
/// no env-var public key is needed for membership tokens (unlike the license
/// token, which is issued off-chassis by the marketplace).
pub struct MembershipKey {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl MembershipKey {
    /// Generate a fresh keypair from OS entropy.
    ///
    /// Reads 32 bytes from `/dev/urandom` as the Ed25519 scalar seed.
    pub fn generate() -> std::io::Result<Self> {
        let mut seed = [0u8; 32];
        let mut f = std::fs::File::open("/dev/urandom")?;
        f.read_exact(&mut seed)?;
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();
        Ok(Self { signing_key, verifying_key })
    }

    /// Issue a signed membership token for the given `module_id` and `archive_id`.
    pub fn issue(&self, module_id: &str, archive_id: &str) -> String {
        let now = Utc::now();
        let claims = MembershipClaims {
            module_id: module_id.to_string(),
            archive_id: archive_id.to_string(),
            issued_at: now.timestamp(),
            expires_at: (now + Duration::hours(TOKEN_VALIDITY_HOURS)).timestamp(),
        };

        let claims_json = serde_json::to_string(&claims)
            .expect("MembershipClaims is always serializable");
        let claims_b64 = URL_SAFE_NO_PAD.encode(claims_json.as_bytes());

        use ed25519_dalek::Signer as _;
        let sig: Signature = self.signing_key.sign(claims_b64.as_bytes());
        let sig_b64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());

        format!("{}.{}", claims_b64, sig_b64)
    }

    /// Verify a membership token. Returns the embedded claims on success.
    ///
    /// Errors:
    /// - `ChassisError::Unauthenticated` — malformed, invalid signature, or expired.
    pub fn verify(&self, token: &str) -> Result<MembershipClaims, ChassisError> {
        let (claims_b64, sig_b64) = token
            .split_once('.')
            .ok_or(ChassisError::Unauthenticated)?;

        let sig_bytes = URL_SAFE_NO_PAD
            .decode(sig_b64)
            .map_err(|_| ChassisError::Unauthenticated)?;
        let sig_arr: [u8; 64] = sig_bytes
            .try_into()
            .map_err(|_| ChassisError::Unauthenticated)?;
        let sig = Signature::from_bytes(&sig_arr);

        use ed25519_dalek::Verifier as _;
        self.verifying_key
            .verify(claims_b64.as_bytes(), &sig)
            .map_err(|_| ChassisError::Unauthenticated)?;

        let claims_json = URL_SAFE_NO_PAD
            .decode(claims_b64)
            .map_err(|_| ChassisError::Unauthenticated)?;
        let claims: MembershipClaims = serde_json::from_slice(&claims_json)
            .map_err(|_| ChassisError::Unauthenticated)?;

        if claims.is_expired() {
            return Err(ChassisError::Unauthenticated);
        }

        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_key() -> MembershipKey {
        MembershipKey::generate().expect("dev: /dev/urandom must be available")
    }

    #[test]
    fn issue_and_verify_roundtrip() {
        let key = make_key();
        let token = key.issue("op::a::slm", "project-test");
        let claims = key.verify(&token).expect("valid token must verify");
        assert_eq!(claims.module_id, "op::a::slm");
        assert_eq!(claims.archive_id, "project-test");
        assert!(!claims.is_expired());
    }

    #[test]
    fn tampered_token_is_rejected() {
        let key = make_key();
        let token = key.issue("op::a::slm", "project-test");
        let tampered = token.replacen('a', "b", 1);
        assert!(key.verify(&tampered).is_err());
    }

    #[test]
    fn wrong_key_is_rejected() {
        let key1 = make_key();
        let key2 = make_key();
        let token = key1.issue("op::a::slm", "project-test");
        assert!(key2.verify(&token).is_err());
    }

    #[test]
    fn token_has_two_segments() {
        let key = make_key();
        let token = key.issue("op::a::slm", "project-test");
        assert_eq!(token.matches('.').count(), 1, "must be exactly one dot separator");
    }
}
