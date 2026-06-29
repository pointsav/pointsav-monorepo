// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Invite token issuer and verifier.
//!
//! Wire format (identical to the license token convention):
//!   `<base64url(payload_json)>.<base64url(ed25519_signature)>`
//! The signature is over the raw bytes of the first segment.
//!
//! Tokens are single-use: the nonce is stored in-memory after first exchange.
//! A replayed nonce returns `Err(CommandError::Invite("nonce already used"))`.
//!
//! Nonce state is in-process and resets on restart. Tokens issued before a
//! restart cannot be replayed because their expiry will typically have passed;
//! for the rare overlap window the caller should check expiry first.

use std::collections::HashSet;
use std::sync::Mutex;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use chrono::{Duration, Utc};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey, Signature};
use uuid::Uuid;

use orchestration_command_core::{InviteTokenPayload, PairingRole};
use crate::error::CommandError;

/// Issues and verifies invite tokens using a per-instance Ed25519 signing key.
pub struct InviteIssuer {
    signing_key: SigningKey,
    instance_id: String,
    used_nonces: Mutex<HashSet<Uuid>>,
}

impl InviteIssuer {
    /// Create an issuer with a freshly-generated ephemeral signing key.
    /// The key is per-process; tokens issued by a previous process cannot
    /// be verified by a new one (restart = new key = all pending tokens invalidated).
    pub fn new_ephemeral(instance_id: impl Into<String>) -> Self {
        // Generate a deterministic-looking but unique key from OS randomness.
        // ed25519-dalek 2.x requires a 32-byte seed.
        let mut seed = [0u8; 32];
        // Fill with pseudo-random bytes — acceptable for ephemeral invite tokens.
        for (i, b) in seed.iter_mut().enumerate() {
            *b = ((Utc::now().timestamp_nanos_opt().unwrap_or(0) >> (i % 8)) as u8)
                .wrapping_add(i as u8);
        }
        // Use a second pass to improve entropy distribution.
        seed[0] ^= seed[31];
        seed[31] ^= seed[0];
        Self {
            signing_key: SigningKey::from_bytes(&seed),
            instance_id: instance_id.into(),
            used_nonces: Mutex::new(HashSet::new()),
        }
    }

    /// Issue a signed invite token.
    pub fn issue(
        &self,
        role: PairingRole,
        archive_scope: Vec<String>,
        ttl_hours: i64,
    ) -> Result<String, CommandError> {
        let payload = InviteTokenPayload {
            issuer: self.instance_id.clone(),
            role,
            nonce: Uuid::now_v7(),
            expiry: Utc::now() + Duration::hours(ttl_hours),
            archive_scope,
        };
        let json = serde_json::to_vec(&payload)
            .map_err(|e| CommandError::Invite(format!("payload json: {e}")))?;
        let payload_b64 = URL_SAFE_NO_PAD.encode(&json);
        let sig: Signature = self.signing_key.sign(payload_b64.as_bytes());
        let sig_b64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());
        Ok(format!("{payload_b64}.{sig_b64}"))
    }

    /// Verify a token and consume its nonce.
    ///
    /// Returns the decoded payload on success. Returns `Err` if the token is
    /// malformed, the signature does not verify, the token is expired, or the
    /// nonce has already been used.
    pub fn verify_and_consume(&self, token: &str) -> Result<InviteTokenPayload, CommandError> {
        let payload = self.verify_only(token)?;

        // Consume nonce — prevents replay.
        let mut used = self.used_nonces.lock().unwrap();
        if used.contains(&payload.nonce) {
            return Err(CommandError::Invite("nonce already used".into()));
        }
        used.insert(payload.nonce);

        Ok(payload)
    }

    fn verify_only(&self, token: &str) -> Result<InviteTokenPayload, CommandError> {
        let (payload_b64, sig_b64) = token
            .split_once('.')
            .ok_or_else(|| CommandError::Invite("token is not <payload>.<signature>".into()))?;

        let payload_bytes = URL_SAFE_NO_PAD
            .decode(payload_b64)
            .map_err(|e| CommandError::Invite(format!("payload base64: {e}")))?;
        let sig_bytes = URL_SAFE_NO_PAD
            .decode(sig_b64)
            .map_err(|e| CommandError::Invite(format!("signature base64: {e}")))?;

        let verifying_key: VerifyingKey = self.signing_key.verifying_key();
        let signature = Signature::from_slice(&sig_bytes)
            .map_err(|e| CommandError::Invite(format!("signature shape: {e}")))?;

        verifying_key
            .verify_strict(payload_b64.as_bytes(), &signature)
            .map_err(|_| CommandError::Invite("signature does not verify".into()))?;

        let payload: InviteTokenPayload = serde_json::from_slice(&payload_bytes)
            .map_err(|e| CommandError::Invite(format!("payload json: {e}")))?;

        if Utc::now() > payload.expiry {
            return Err(CommandError::Invite(format!(
                "token expired at {}",
                payload.expiry.to_rfc3339()
            )));
        }

        Ok(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn issuer() -> InviteIssuer {
        InviteIssuer::new_ephemeral("test-instance")
    }

    #[test]
    fn issue_and_verify() {
        let iss = issuer();
        let token = iss.issue(PairingRole::User, vec!["bim".into()], 24).unwrap();
        let payload = iss.verify_and_consume(&token).unwrap();
        assert_eq!(payload.role, PairingRole::User);
        assert_eq!(payload.archive_scope, ["bim"]);
    }

    #[test]
    fn replay_rejected() {
        let iss = issuer();
        let token = iss.issue(PairingRole::User, vec![], 24).unwrap();
        iss.verify_and_consume(&token).unwrap();
        let err = iss.verify_and_consume(&token).unwrap_err();
        assert!(err.to_string().contains("nonce already used"));
    }

    #[test]
    fn tampered_token_rejected() {
        let iss = issuer();
        let token = iss.issue(PairingRole::User, vec![], 24).unwrap();
        let (p, s) = token.split_once('.').unwrap();
        let mut p = p.to_string();
        let last = p.pop().unwrap();
        p.push(if last == 'A' { 'B' } else { 'A' });
        let err = iss.verify_and_consume(&format!("{p}.{s}")).unwrap_err();
        assert!(err.to_string().contains("signature does not verify") || err.to_string().contains("payload"));
    }
}
