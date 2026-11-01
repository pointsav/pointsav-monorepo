// SPDX-License-Identifier: Apache-2.0

//! `ssh-keygen -Y sign`/`verify` wrapper for Envelope signatures.
//!
//! Reuses the exact workspace-wide SSHSIG signing convention already
//! proven in `system-ledger::witness` (which documents: "the signing
//! primitive is the same ssh-keygen -Y sign / verify workspace
//! infrastructure that backs commit signing and apprenticeship
//! verdict signing"). Not a dependency on that crate -- `system-ledger`
//! deliberately declares its own nested `[workspace]` (excluded from
//! this repo's root workspace specifically to avoid multi-workspace-
//! root conflicts when path-depended-on from a root-workspace member,
//! per the root `Cargo.toml`'s own `exclude` comment), so depending on
//! it directly from `envelope` -- a root-workspace member -- would
//! reintroduce that exact conflict. This module is a focused,
//! self-contained re-implementation of the same std-path approach
//! (shell out to `ssh-keygen`), not a divergent scheme: it exists
//! precisely because `witness.rs`'s own doc comment says callers must
//! pick a distinct namespace tag per signing purpose, never reuse
//! `WITNESS_NAMESPACE` for a different kind of record.

use std::io::Write;
use std::process::{Command, Stdio};

/// Namespace tag bound to the `-n` flag of `ssh-keygen -Y sign/verify`.
/// Distinct from `system-ledger::witness::WITNESS_NAMESPACE`
/// (`"capability-witness-v1"`) and the apprenticeship substrate's own
/// tag -- cross-namespace replay against either is exactly the attack
/// a distinct namespace per signing purpose prevents.
pub const ENVELOPE_NAMESPACE: &str = "totebox-envelope-v1";

#[derive(Debug)]
pub enum SigningError {
    TempFileFailed(String),
    ShellOutFailed(String),
    NonUtf8Path,
    SignFailed(String),
}

impl std::fmt::Display for SigningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SigningError::TempFileFailed(e) => write!(f, "tempfile error: {e}"),
            SigningError::ShellOutFailed(e) => write!(f, "ssh-keygen shell-out failed: {e}"),
            SigningError::NonUtf8Path => write!(f, "tempfile path was not valid UTF-8"),
            SigningError::SignFailed(e) => write!(f, "ssh-keygen -Y sign failed: {e}"),
        }
    }
}

impl std::error::Error for SigningError {}

/// Sign `payload` (the envelope's canonical-CBOR bytes, minus the
/// `signature` field itself) under the Ed25519 private key at
/// `signing_key_path`, using [`ENVELOPE_NAMESPACE`]. Returns the
/// PEM-armored SSHSIG signature bytes (`-----BEGIN SSH SIGNATURE-----`
/// form) -- store these verbatim as `Envelope::signature`.
pub fn sign_envelope(
    signing_key_path: &std::path::Path,
    payload: &[u8],
) -> Result<Vec<u8>, SigningError> {
    let key_path = signing_key_path
        .to_str()
        .ok_or(SigningError::NonUtf8Path)?;
    let mut child = Command::new("ssh-keygen")
        .args(["-Y", "sign", "-f", key_path, "-n", ENVELOPE_NAMESPACE, "-q"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| SigningError::ShellOutFailed(format!("spawn: {e}")))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(payload)
            .map_err(|e| SigningError::ShellOutFailed(format!("stdin write: {e}")))?;
        // Drop closes stdin -> signals EOF.
    }

    let output = child
        .wait_with_output()
        .map_err(|e| SigningError::ShellOutFailed(format!("wait: {e}")))?;

    if !output.status.success() {
        return Err(SigningError::SignFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    Ok(output.stdout)
}

#[derive(Debug)]
pub enum VerifyError {
    TempFileFailed(String),
    ShellOutFailed(String),
    NonUtf8Path,
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::TempFileFailed(e) => write!(f, "tempfile error: {e}"),
            VerifyError::ShellOutFailed(e) => write!(f, "ssh-keygen shell-out failed: {e}"),
            VerifyError::NonUtf8Path => write!(f, "tempfile path was not valid UTF-8"),
        }
    }
}

impl std::error::Error for VerifyError {}

/// Verify a detached SSH signature over `payload` under `ssh_pubkey`,
/// asserting namespace = [`ENVELOPE_NAMESPACE`] and identity =
/// `signer_identity`. Returns `Ok(true)` when valid, `Ok(false)` when
/// not (wrong key, wrong payload, wrong namespace, truncated),
/// `Err(_)` only on infrastructure failure (ssh-keygen missing,
/// tempfile write failure, etc.).
pub fn verify_envelope_signature(
    signature_armored: &[u8],
    payload: &[u8],
    ssh_pubkey: &str,
    signer_identity: &str,
) -> Result<bool, VerifyError> {
    use tempfile::NamedTempFile;

    let mut sig_file = NamedTempFile::new()
        .map_err(|e| VerifyError::TempFileFailed(format!("sig tmp: {e}")))?;
    sig_file
        .write_all(signature_armored)
        .map_err(|e| VerifyError::TempFileFailed(format!("write sig: {e}")))?;
    sig_file
        .flush()
        .map_err(|e| VerifyError::TempFileFailed(format!("flush sig: {e}")))?;

    let allowed_line = format!("{} {}\n", signer_identity, ssh_pubkey.trim());
    let mut signers_file = NamedTempFile::new()
        .map_err(|e| VerifyError::TempFileFailed(format!("signers tmp: {e}")))?;
    signers_file
        .write_all(allowed_line.as_bytes())
        .map_err(|e| VerifyError::TempFileFailed(format!("write signers: {e}")))?;
    signers_file
        .flush()
        .map_err(|e| VerifyError::TempFileFailed(format!("flush signers: {e}")))?;

    let sig_path = sig_file.path().to_str().ok_or(VerifyError::NonUtf8Path)?;
    let signers_path = signers_file
        .path()
        .to_str()
        .ok_or(VerifyError::NonUtf8Path)?;

    let mut child = Command::new("ssh-keygen")
        .args([
            "-Y",
            "verify",
            "-f",
            signers_path,
            "-I",
            signer_identity,
            "-n",
            ENVELOPE_NAMESPACE,
            "-s",
            sig_path,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| VerifyError::ShellOutFailed(format!("spawn: {e}")))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(payload)
            .map_err(|e| VerifyError::ShellOutFailed(format!("stdin write: {e}")))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| VerifyError::ShellOutFailed(format!("wait: {e}")))?;

    Ok(output.status.success())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_keypair(dir: &TempDir, name: &str) -> (std::path::PathBuf, String) {
        let priv_path = dir.path().join(name);
        let pub_path = dir.path().join(format!("{name}.pub"));
        let status = Command::new("ssh-keygen")
            .args([
                "-t",
                "ed25519",
                "-f",
                priv_path.to_str().unwrap(),
                "-N",
                "",
                "-C",
                &format!("{name}@envelope-test"),
                "-q",
            ])
            .status()
            .expect("ssh-keygen keygen runs");
        assert!(status.success());
        let pubkey = fs::read_to_string(&pub_path).unwrap().trim().to_string();
        (priv_path, pubkey)
    }

    #[test]
    fn sign_then_verify_round_trips() {
        let dir = TempDir::new().unwrap();
        let (priv_path, pubkey) = make_keypair(&dir, "k1");
        let payload = b"envelope canonical bytes go here";

        let sig = sign_envelope(&priv_path, payload).expect("sign succeeds");
        let ok = verify_envelope_signature(&sig, payload, &pubkey, "k1@envelope-test")
            .expect("verify call ok");
        assert!(ok, "valid signature should verify");
    }

    #[test]
    fn tampered_payload_fails_verification() {
        let dir = TempDir::new().unwrap();
        let (priv_path, pubkey) = make_keypair(&dir, "k2");
        let payload = b"original bytes";

        let sig = sign_envelope(&priv_path, payload).expect("sign succeeds");
        let ok = verify_envelope_signature(&sig, b"tampered bytes", &pubkey, "k2@envelope-test")
            .expect("verify call ok");
        assert!(!ok, "tampered payload must NOT verify");
    }

    #[test]
    fn wrong_pubkey_fails_verification() {
        let dir = TempDir::new().unwrap();
        let (priv1, _pub1) = make_keypair(&dir, "k3a");
        let (_priv2, pub2) = make_keypair(&dir, "k3b");
        let payload = b"some bytes";

        let sig = sign_envelope(&priv1, payload).expect("sign succeeds");
        let ok = verify_envelope_signature(&sig, payload, &pub2, "k3a@envelope-test")
            .expect("verify call ok");
        assert!(!ok, "wrong pubkey must NOT verify");
    }

    #[test]
    fn cross_namespace_signature_fails_verification() {
        // Sign under a DIFFERENT namespace (simulating a witness-record
        // or apprenticeship-verdict signature) and confirm it does not
        // verify as an envelope signature -- the whole point of a
        // distinct namespace tag per signing purpose.
        let dir = TempDir::new().unwrap();
        let (priv_path, pubkey) = make_keypair(&dir, "k4");
        let payload = b"some bytes";

        let mut child = Command::new("ssh-keygen")
            .args([
                "-Y",
                "sign",
                "-f",
                priv_path.to_str().unwrap(),
                "-n",
                "capability-witness-v1",
                "-q",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(payload).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success());

        let ok = verify_envelope_signature(&output.stdout, payload, &pubkey, "k4@envelope-test")
            .expect("verify call ok");
        assert!(!ok, "a witness-namespace signature must NOT verify as an envelope signature");
    }
}
