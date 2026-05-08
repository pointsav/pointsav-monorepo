//! `ssh-keygen -Y verify` wrapper for witness-record signatures.
//!
//! Per `~/Foundry/CLAUDE.md` §3 + `apprenticeship-substrate.md` §5:
//! the namespace tag for witness signatures is
//! [`WITNESS_NAMESPACE`] = `"capability-witness-v1"`. The signing
//! primitive is the same `ssh-keygen -Y sign / verify` workspace
//! infrastructure that backs commit signing and apprenticeship
//! verdict signing — different namespace tag prevents
//! cross-namespace replay attacks.
//!
//! Implementation: shells out to `/usr/bin/ssh-keygen -Y verify`
//! via `std::process::Command`. Synchronous; the kernel-side
//! verifier blocks on this call. For async consumers the function
//! should be wrapped with `tokio::task::spawn_blocking`.
//!
//! Signed-payload contract (per
//! `system-core::WitnessRecord` doc-comment): the bytes
//! that get signed are `(capability_hash || new_expiry_t.to_be_bytes())`.
//! The caller is responsible for assembling that payload before
//! invoking [`verify_witness_signature`].

use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

/// Namespace tag bound to the `-n` flag of `ssh-keygen -Y sign /
/// verify`. Cross-namespace replay against commit-signing or
/// apprenticeship-verdict signatures is the attack this discipline
/// prevents.
pub const WITNESS_NAMESPACE: &str = "capability-witness-v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WitnessVerifyError {
    /// Failed to write a tempfile (signature or allowed_signers).
    TempFileFailed(String),
    /// `ssh-keygen` not found, didn't start, or crashed.
    ShellOutFailed(String),
    /// Tempfile path was not valid UTF-8 (extremely unlikely on
    /// Foundry VMs but enumerated for completeness).
    NonUtf8Path,
}

/// Verify a detached SSH signature over `signed_payload` under
/// `ssh_pubkey`, asserting namespace = `WITNESS_NAMESPACE` and
/// identity = `signer_identity`.
///
/// Returns `Ok(true)` if `ssh-keygen -Y verify` exits 0 (signature
/// accepts), `Ok(false)` if it exits non-zero (signature refused),
/// `Err(_)` only on infrastructure failure (tempfile / fork /
/// non-utf8 path).
///
/// # Arguments
///
/// - `signature_armored`: the signature in `-----BEGIN SSH
///   SIGNATURE-----` armored form (the format produced by
///   `ssh-keygen -Y sign`); typically the bytes from
///   `WitnessRecord::signature`.
/// - `signed_payload`: the bytes that were signed. Per
///   `WitnessRecord` doc, this is
///   `(capability_hash || new_expiry_t.to_be_bytes())`. Caller
///   assembles.
/// - `ssh_pubkey`: SSH-format public key on a single line, e.g.
///   `"ssh-ed25519 AAAA... [comment]"`. The comment is optional.
/// - `signer_identity`: identity string; matches the `-I` arg of
///   `ssh-keygen -Y verify` and the first column in the
///   `allowed_signers` file generated for this verification.
pub fn verify_witness_signature(
    signature_armored: &[u8],
    signed_payload: &[u8],
    ssh_pubkey: &str,
    signer_identity: &str,
) -> Result<bool, WitnessVerifyError> {
    // Tempfile 1 — signature (.sig); ssh-keygen requires a file
    // path, not stdin.
    let mut sig_file = NamedTempFile::new()
        .map_err(|e| WitnessVerifyError::TempFileFailed(format!("sig tmp: {e}")))?;
    sig_file
        .write_all(signature_armored)
        .map_err(|e| WitnessVerifyError::TempFileFailed(format!("write sig: {e}")))?;
    sig_file
        .flush()
        .map_err(|e| WitnessVerifyError::TempFileFailed(format!("flush sig: {e}")))?;

    // Tempfile 2 — allowed_signers (single line).
    let allowed_line = format!("{} {}\n", signer_identity, ssh_pubkey.trim());
    let mut signers_file = NamedTempFile::new()
        .map_err(|e| WitnessVerifyError::TempFileFailed(format!("signers tmp: {e}")))?;
    signers_file
        .write_all(allowed_line.as_bytes())
        .map_err(|e| WitnessVerifyError::TempFileFailed(format!("write signers: {e}")))?;
    signers_file
        .flush()
        .map_err(|e| WitnessVerifyError::TempFileFailed(format!("flush signers: {e}")))?;

    let sig_path = sig_file.path().to_str().ok_or(WitnessVerifyError::NonUtf8Path)?;
    let signers_path = signers_file
        .path()
        .to_str()
        .ok_or(WitnessVerifyError::NonUtf8Path)?;

    // Spawn ssh-keygen -Y verify.
    let mut child = Command::new("ssh-keygen")
        .args([
            "-Y",
            "verify",
            "-f",
            signers_path,
            "-I",
            signer_identity,
            "-n",
            WITNESS_NAMESPACE,
            "-s",
            sig_path,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| WitnessVerifyError::ShellOutFailed(format!("spawn: {e}")))?;

    // Write payload to stdin.
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(signed_payload)
            .map_err(|e| WitnessVerifyError::ShellOutFailed(format!("stdin write: {e}")))?;
        // Drop closes stdin → signals EOF.
    }

    let output = child
        .wait_with_output()
        .map_err(|e| WitnessVerifyError::ShellOutFailed(format!("wait: {e}")))?;

    Ok(output.status.success())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    /// Generate an ed25519 keypair via ssh-keygen. Returns
    /// `(privkey_path, pubkey_string)` where pubkey_string is the
    /// single-line SSH-format public key.
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
                &format!("{name}@witness-test"),
                "-q",
            ])
            .status()
            .expect("ssh-keygen keygen runs");
        assert!(status.success(), "ssh-keygen keygen exit");
        let pubkey = fs::read_to_string(&pub_path)
            .expect("read .pub file")
            .trim()
            .to_string();
        (priv_path, pubkey)
    }

    /// Sign `payload` under `priv_path` with the given namespace.
    /// Returns the armored signature bytes.
    fn sign_payload(priv_path: &std::path::Path, payload: &[u8], namespace: &str) -> Vec<u8> {
        // ssh-keygen -Y sign reads from stdin and writes the
        // signature to stdout when -O write-stdout is set; without
        // it, it writes to <input>.sig. We use stdin path with `-`
        // and capture stdout.
        let mut child = Command::new("ssh-keygen")
            .args([
                "-Y",
                "sign",
                "-f",
                priv_path.to_str().unwrap(),
                "-n",
                namespace,
                "-q",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("ssh-keygen sign spawns");
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(payload).expect("write payload");
        }
        let output = child.wait_with_output().expect("wait sign");
        assert!(
            output.status.success(),
            "ssh-keygen sign failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        output.stdout
    }

    #[test]
    fn verify_accepts_valid_signature() {
        let dir = TempDir::new().expect("tmpdir");
        let (priv_path, pubkey) = make_keypair(&dir, "k1");

        let payload = b"capability_hash || new_expiry";
        let sig = sign_payload(&priv_path, payload, WITNESS_NAMESPACE);

        let r = verify_witness_signature(&sig, payload, &pubkey, "k1@witness-test")
            .expect("verify call ok");
        assert!(r, "valid signature should verify");
    }

    #[test]
    fn verify_rejects_tampered_payload() {
        let dir = TempDir::new().expect("tmpdir");
        let (priv_path, pubkey) = make_keypair(&dir, "k2");

        let payload = b"capability_hash || new_expiry";
        let sig = sign_payload(&priv_path, payload, WITNESS_NAMESPACE);

        let tampered = b"capability_hash || NEW_EXPIRY_TAMPERED";
        let r = verify_witness_signature(&sig, tampered, &pubkey, "k2@witness-test")
            .expect("verify call ok");
        assert!(!r, "tampered payload should NOT verify");
    }

    #[test]
    fn verify_rejects_wrong_pubkey() {
        let dir = TempDir::new().expect("tmpdir");
        let (priv1, _pub1) = make_keypair(&dir, "k3a");
        let (_priv2, pub2) = make_keypair(&dir, "k3b");

        let payload = b"some bytes";
        let sig = sign_payload(&priv1, payload, WITNESS_NAMESPACE);

        // Use the WRONG pubkey for verification.
        let r = verify_witness_signature(&sig, payload, &pub2, "k3a@witness-test")
            .expect("verify call ok");
        assert!(!r, "wrong pubkey should NOT verify");
    }

    #[test]
    fn verify_rejects_cross_namespace_signature() {
        let dir = TempDir::new().expect("tmpdir");
        let (priv_path, pubkey) = make_keypair(&dir, "k4");

        let payload = b"some bytes";
        // Sign under a DIFFERENT namespace (e.g., commit-signing).
        let sig = sign_payload(&priv_path, payload, "git-commit-signing");

        let r = verify_witness_signature(&sig, payload, &pubkey, "k4@witness-test")
            .expect("verify call ok");
        assert!(
            !r,
            "signature in a different namespace must NOT verify under WITNESS_NAMESPACE"
        );
    }

    #[test]
    fn verify_rejects_truncated_signature() {
        let dir = TempDir::new().expect("tmpdir");
        let (priv_path, pubkey) = make_keypair(&dir, "k5");

        let payload = b"some bytes";
        let mut sig = sign_payload(&priv_path, payload, WITNESS_NAMESPACE);
        // Truncate the signature mid-base64-line; ssh-keygen will
        // reject the malformed armor.
        sig.truncate(sig.len() / 2);

        let r = verify_witness_signature(&sig, payload, &pubkey, "k5@witness-test");
        // Malformed signature: ssh-keygen -Y verify exits non-zero,
        // we return Ok(false). Either Ok(false) or
        // ShellOutFailed acceptable (ssh-keygen behaviour varies on
        // truncation patterns); we assert it's not Ok(true).
        assert!(!matches!(r, Ok(true)), "truncated sig must not verify; got {r:?}");
    }
}
