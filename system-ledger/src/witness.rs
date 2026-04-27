//! `ssh-keygen -Y verify` wrapper for witness-record signatures.
//!
//! Per `~/Foundry/CLAUDE.md` §3 + `apprenticeship-substrate.md` §5:
//! the namespace tag for witness signatures is `capability-witness-v1`;
//! pubkey load follows the `allowed_signers` format.
//!
//! Skeleton — implementation lands per task #12. Calling out to
//! `ssh-keygen` will likely use `std::process::Command` (or
//! `tokio::process::Command` if the consumer is already async).

/// Namespace tag bound to the `-n` flag of `ssh-keygen -Y sign /
/// verify`. Cross-namespace replay is the attack this discipline
/// prevents.
pub const WITNESS_NAMESPACE: &str = "capability-witness-v1";

/// Verifies a detached `ssh-keygen -Y` signature over `(capability_hash
/// || new_expiry_t.to_be_bytes())` under a given SSH-format public
/// key. Returns `Ok(true)` on valid signature, `Ok(false)` on signature
/// mismatch, `Err(_)` on shell-out failure or malformed input.
///
/// Skeleton — currently always returns `Err`. Real implementation
/// lands per task #12.
pub fn verify_witness_signature(
    _signature: &[u8],
    _signed_payload: &[u8],
    _ssh_pubkey: &str,
) -> Result<bool, WitnessVerifyError> {
    Err(WitnessVerifyError::NotImplemented)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessVerifyError {
    NotImplemented,
    ShellOutFailed(String),
    MalformedPubkey,
    MalformedSignature,
}
