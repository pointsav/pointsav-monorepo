// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! SYS-ADR-19 write-governance checkpoint for service-content's automated
//! DataGraph write path.
//!
//! Modeled directly on the apprenticeship/verdict-signing substrate
//! (`service-slm/crates/slm-doorman/src/{apprenticeship,verdict}.rs`) — the
//! same **capture-then-promote**, **verify-before-mutate** shape, simplified
//! for this domain (no brief/attempt distinction, no DPO pairs, no
//! promotion-ledger threshold tracking):
//!
//! 1. **Capture**: when `SERVICE_CONTENT_WRITE_GOVERNANCE_ENABLED` is set, an
//!    automated write (Tier A/Tier B extraction) is captured to a durable
//!    JSONL artifact under `pending_dir` instead of calling `upsert_entities`
//!    directly — `verdict: null`. Survives process restart; a human can
//!    review it via `GET /v1/graph/pending` at any time.
//! 2. **Verify-before-mutate**: `POST /v1/graph/verdict` takes a reviewer's
//!    SSH-signed verdict. The signature (over a canonical text body built
//!    from the verdict fields) is checked via `ssh-keygen -Y verify` against
//!    `${FOUNDRY_ROOT}/identity/allowed_signers`, namespace-bound to
//!    `graph-write-verdict-v1` — the same mechanism, same allowed_signers
//!    file, as the apprenticeship substrate's `apprenticeship-verdict-v1`,
//!    but its own namespace tag so a verdict signature can never be
//!    repurposed across the two systems.
//! 3. **Promote-in-place on accept**: the pending artifact is rewritten
//!    (atomic write-to-temp + rename, same as `promote_corpus_tuple`) with
//!    the verdict and `promoted_at` set, and the entities are written to the
//!    graph for real via `GraphStore::upsert_entities`.
//! 4. **Discard on reject**: the artifact is rewritten with the verdict set
//!    but no graph write happens — retained for audit, never deleted.
//!
//! Deliberately does NOT gate `POST /v1/graph/mutate` (`http.rs`'s
//! `graph_mutate` handler) — that path is already human-gated by its own
//! callers (e.g. project-editorial's `graph-committer.py`, which requires
//! `--confirm` and a prior human approval step of its own). This checkpoint
//! only covers the automated `process_corpus` call sites in `lib.rs`, which
//! previously wrote directly with no checkpoint at all.
//!
//! **Rollout**: ships behind `SERVICE_CONTENT_WRITE_GOVERNANCE_ENABLED`,
//! default unset/off — automated writes behave exactly as before until an
//! operator deliberately turns this on. Real production volume (tens of
//! thousands of queued documents) would otherwise silently stall the moment
//! this binary deploys.

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::graph::{GraphEntity, GraphStore};

/// SSH signature namespace for graph-write verdicts — deliberately distinct
/// from the apprenticeship substrate's `apprenticeship-verdict-v1` so a
/// signature can never be replayed across the two systems.
pub const WRITE_GOVERNANCE_NAMESPACE: &str = "graph-write-verdict-v1";

/// `true` iff `SERVICE_CONTENT_WRITE_GOVERNANCE_ENABLED` is set to a
/// truthy value. Read fresh each call (cheap; not a hot-loop check — this
/// gates once per document processed, not per request).
pub fn write_governance_enabled() -> bool {
    matches!(
        std::env::var("SERVICE_CONTENT_WRITE_GOVERNANCE_ENABLED").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes")
    )
}

/// Directory holding pending-write JSONL artifacts. Defaults to
/// `<SERVICE_CONTENT_GRAPH_DIR>/pending-writes`. Reads env vars directly
/// (matching this codebase's existing ad-hoc-config convention, e.g.
/// `default_module_id()` in `slm-doorman-server`) rather than threading a
/// new parameter through `process_corpus`'s several call sites in `lib.rs`.
pub fn pending_writes_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("SERVICE_CONTENT_PENDING_WRITES_DIR") {
        return PathBuf::from(dir);
    }
    let graph_dir = std::env::var("SERVICE_CONTENT_GRAPH_DIR")
        .unwrap_or_else(|_| "/var/lib/local-content/graph".to_string());
    Path::new(&graph_dir).join("pending-writes")
}

/// Resolve `allowed_signers`, matching `slm-doorman-server`'s own
/// convention exactly (`FOUNDRY_ALLOWED_SIGNERS`, else
/// `${FOUNDRY_ROOT}/identity/allowed_signers`, else `/srv/foundry`).
pub fn allowed_signers_path() -> PathBuf {
    std::env::var_os("FOUNDRY_ALLOWED_SIGNERS")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let foundry_root = std::env::var_os("FOUNDRY_ROOT")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("/srv/foundry"));
            foundry_root.join("identity").join("allowed_signers")
        })
}

// ── errors ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum WriteGovernanceError {
    VerifySignature(String),
    VerdictParse(String),
    OrphanVerdict { write_id: String, path: String },
    AlreadyResolved { write_id: String },
    Io(String),
}

impl fmt::Display for WriteGovernanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VerifySignature(m) => write!(f, "signature verification failed: {m}"),
            Self::VerdictParse(m) => write!(f, "verdict parse error: {m}"),
            Self::OrphanVerdict { write_id, path } => write!(
                f,
                "no pending write found for write_id {write_id:?} (expected at {path})"
            ),
            Self::AlreadyResolved { write_id } => {
                write!(f, "write_id {write_id:?} already has a verdict recorded")
            }
            Self::Io(m) => write!(f, "io error: {m}"),
        }
    }
}

impl std::error::Error for WriteGovernanceError {}

// ── wire / domain types ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WriteVerdictOutcome {
    Accept,
    Reject,
}

impl WriteVerdictOutcome {
    fn parse(s: &str) -> Result<Self, WriteGovernanceError> {
        match s {
            "accept" => Ok(Self::Accept),
            "reject" => Ok(Self::Reject),
            other => Err(WriteGovernanceError::VerdictParse(format!(
                "unknown outcome {other:?}, expected \"accept\" or \"reject\""
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteVerdict {
    pub outcome: WriteVerdictOutcome,
    pub reviewer_identity: String,
    pub created: String,
    #[serde(default)]
    pub notes: Option<String>,
}

/// A held-pending automated write, captured before it would otherwise have
/// gone straight to `GraphStore::upsert_entities`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingWrite {
    pub write_id: String,
    pub module_id: String,
    pub entities: Vec<GraphEntity>,
    #[serde(default)]
    pub source_doc: Option<String>,
    pub tier_used: String,
    pub captured_at: String,
    #[serde(default)]
    pub verdict: Option<WriteVerdict>,
    #[serde(default)]
    pub promoted_at: Option<String>,
}

/// `POST /v1/graph/verdict` wire body. The reviewer signs a canonical text
/// body built by `verdict_signable_body` (same shape they saw when
/// reviewing via `GET /v1/graph/pending`) with `ssh-keygen -Y sign -n
/// graph-write-verdict-v1`.
#[derive(Debug, Clone, Deserialize)]
pub struct WriteVerdictWireBody {
    pub write_id: String,
    pub outcome: String,
    pub reviewer_identity: String,
    #[serde(default)]
    pub notes: Option<String>,
    /// Base64-encoded SSH signature blob over `verdict_signable_body(..)`.
    pub signature: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WriteVerdictDispatchOutcome {
    pub write_id: String,
    pub outcome: WriteVerdictOutcome,
    pub promoted: bool,
    pub upserted: usize,
}

/// The exact text a reviewer signs — deterministic from the wire fields
/// so the server can reconstruct and verify it without trusting a
/// caller-supplied body string.
pub fn verdict_signable_body(
    write_id: &str,
    outcome: &str,
    reviewer_identity: &str,
    notes: Option<&str>,
) -> String {
    format!(
        "write_id: {write_id}\noutcome: {outcome}\nreviewer_identity: {reviewer_identity}\nnotes: {}\n",
        notes.unwrap_or("")
    )
}

// ── signature verification ──────────────────────────────────────────────

/// Verifies a reviewer's SSH signature over a verdict body. Production
/// uses `SshKeygenVerifier`; tests use a mock. Deliberately synchronous —
/// callers wrap the (blocking, shells-out) call in `tokio::task::
/// spawn_blocking` themselves, avoiding a dependency on `async-trait`.
pub trait WriteVerdictVerifier: Send + Sync + fmt::Debug {
    fn verify(
        &self,
        body: &str,
        signature_pem: &str,
        reviewer_identity: &str,
        namespace: &str,
    ) -> Result<(), WriteGovernanceError>;
}

/// Real verifier — shells out to `ssh-keygen -Y verify`, same invocation
/// shape as `slm_doorman::verdict::SshKeygenVerifier`.
#[derive(Clone, Debug)]
pub struct SshKeygenVerifier {
    pub allowed_signers: PathBuf,
}

impl SshKeygenVerifier {
    pub fn new(allowed_signers: impl Into<PathBuf>) -> Self {
        Self {
            allowed_signers: allowed_signers.into(),
        }
    }
}

impl WriteVerdictVerifier for SshKeygenVerifier {
    fn verify(
        &self,
        body: &str,
        signature_pem: &str,
        reviewer_identity: &str,
        namespace: &str,
    ) -> Result<(), WriteGovernanceError> {
        use std::io::Write as _;
        use std::process::{Command, Stdio};

        let sig_path = std::env::temp_dir().join(format!(
            "service-content-write-verdict-{}.sig",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        std::fs::write(&sig_path, signature_pem.as_bytes())
            .map_err(|e| WriteGovernanceError::VerifySignature(format!("write sig file: {e}")))?;
        let principal = format!("{reviewer_identity}@users.noreply.github.com");

        let result = (|| -> Result<std::process::Output, WriteGovernanceError> {
            let mut child = Command::new("ssh-keygen")
                .arg("-Y")
                .arg("verify")
                .arg("-f")
                .arg(&self.allowed_signers)
                .arg("-I")
                .arg(&principal)
                .arg("-n")
                .arg(namespace)
                .arg("-s")
                .arg(&sig_path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| WriteGovernanceError::VerifySignature(format!("spawn: {e}")))?;
            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(body.as_bytes())
                    .map_err(|e| WriteGovernanceError::VerifySignature(format!("stdin: {e}")))?;
            }
            child
                .wait_with_output()
                .map_err(|e| WriteGovernanceError::VerifySignature(format!("wait: {e}")))
        })();

        let _ = std::fs::remove_file(&sig_path);
        let output = result?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            Err(WriteGovernanceError::VerifySignature(format!(
                "ssh-keygen -Y verify exited {}: {stderr}",
                output.status
            )))
        }
    }
}

fn decode_signature(b64: &str) -> Result<String, WriteGovernanceError> {
    let bytes = B64
        .decode(b64.trim().as_bytes())
        .map_err(|e| WriteGovernanceError::VerifySignature(format!("base64 decode: {e}")))?;
    String::from_utf8(bytes)
        .map_err(|e| WriteGovernanceError::VerifySignature(format!("utf-8 decode: {e}")))
}

// ── capture ──────────────────────────────────────────────────────────────

fn pending_write_path(pending_dir: &Path, write_id: &str) -> PathBuf {
    pending_dir.join(format!("pending-{write_id}.jsonl"))
}

/// Capture an automated write as a held-pending artifact instead of
/// writing it directly. Returns the generated `write_id`.
pub fn capture_pending_write(
    pending_dir: &Path,
    module_id: &str,
    entities: &[GraphEntity],
    source_doc: Option<&str>,
    tier_used: &str,
) -> Result<String, WriteGovernanceError> {
    std::fs::create_dir_all(pending_dir)
        .map_err(|e| WriteGovernanceError::Io(format!("create pending dir: {e}")))?;

    let write_id = format!(
        "{}-{}",
        module_id,
        Utc::now().timestamp_nanos_opt().unwrap_or(0)
    );
    let pending = PendingWrite {
        write_id: write_id.clone(),
        module_id: module_id.to_string(),
        entities: entities.to_vec(),
        source_doc: source_doc.map(str::to_string),
        tier_used: tier_used.to_string(),
        captured_at: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        verdict: None,
        promoted_at: None,
    };
    let line = serde_json::to_string(&pending)
        .map_err(|e| WriteGovernanceError::Io(format!("serialize pending write: {e}")))?;
    let path = pending_write_path(pending_dir, &write_id);
    std::fs::write(&path, format!("{line}\n"))
        .map_err(|e| WriteGovernanceError::Io(format!("write {}: {e}", path.display())))?;
    Ok(write_id)
}

/// List every pending write under `pending_dir`, most-recently-captured
/// first. Includes resolved (verdict-set) entries too — callers filter
/// for `verdict.is_none()` if they only want the outstanding queue.
pub fn list_pending(pending_dir: &Path) -> Result<Vec<PendingWrite>, WriteGovernanceError> {
    let mut out = Vec::new();
    let entries = match std::fs::read_dir(pending_dir) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(out),
        Err(e) => return Err(WriteGovernanceError::Io(format!("read_dir: {e}"))),
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
            continue;
        }
        let content = std::fs::read_to_string(&path)
            .map_err(|e| WriteGovernanceError::Io(format!("read {}: {e}", path.display())))?;
        if let Ok(pw) = serde_json::from_str::<PendingWrite>(content.trim()) {
            out.push(pw);
        }
    }
    out.sort_by(|a, b| b.captured_at.cmp(&a.captured_at));
    Ok(out)
}

// ── dispatcher ───────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct WriteGovernanceDispatcher {
    pub verifier: Arc<dyn WriteVerdictVerifier>,
    pub pending_dir: PathBuf,
}

impl WriteGovernanceDispatcher {
    /// AS-3-equivalent entry point for graph-write verdicts. Verify before
    /// any state mutation; on accept, promote the pending write into the
    /// real graph via `graph.upsert_entities`; on reject, discard (mark
    /// resolved, no graph write).
    pub fn dispatch(
        &self,
        wire: WriteVerdictWireBody,
        graph: &dyn GraphStore,
    ) -> Result<WriteVerdictDispatchOutcome, WriteGovernanceError> {
        let signature_pem = decode_signature(&wire.signature)?;
        let body = verdict_signable_body(
            &wire.write_id,
            &wire.outcome,
            &wire.reviewer_identity,
            wire.notes.as_deref(),
        );

        // Verify before any state mutation.
        self.verifier.verify(
            &body,
            &signature_pem,
            &wire.reviewer_identity,
            WRITE_GOVERNANCE_NAMESPACE,
        )?;

        let outcome = WriteVerdictOutcome::parse(&wire.outcome)?;
        let path = pending_write_path(&self.pending_dir, &wire.write_id);
        if !path.exists() {
            return Err(WriteGovernanceError::OrphanVerdict {
                write_id: wire.write_id,
                path: path.display().to_string(),
            });
        }
        let content = std::fs::read_to_string(&path)
            .map_err(|e| WriteGovernanceError::Io(format!("read {}: {e}", path.display())))?;
        let mut pending: PendingWrite = serde_json::from_str(content.trim())
            .map_err(|e| WriteGovernanceError::Io(format!("parse {}: {e}", path.display())))?;

        if pending.verdict.is_some() {
            return Err(WriteGovernanceError::AlreadyResolved {
                write_id: wire.write_id,
            });
        }

        let mut upserted = 0usize;
        if outcome == WriteVerdictOutcome::Accept {
            upserted = graph
                .upsert_entities(&pending.module_id, &pending.entities)
                .map_err(|e| WriteGovernanceError::Io(format!("upsert_entities: {e}")))?;
            pending.promoted_at =
                Some(Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
        }
        pending.verdict = Some(WriteVerdict {
            outcome,
            reviewer_identity: wire.reviewer_identity.clone(),
            created: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            notes: wire.notes.clone(),
        });

        // Atomic overwrite: write to temp, then rename — never lose the
        // pending artifact even if the process dies mid-write.
        let line = serde_json::to_string(&pending)
            .map_err(|e| WriteGovernanceError::Io(format!("serialize promoted write: {e}")))?;
        let tmp_path = path.with_extension("jsonl.tmp");
        std::fs::write(&tmp_path, format!("{line}\n"))
            .map_err(|e| WriteGovernanceError::Io(format!("write temp: {e}")))?;
        std::fs::rename(&tmp_path, &path)
            .map_err(|e| WriteGovernanceError::Io(format!("rename temp to final: {e}")))?;

        Ok(WriteVerdictDispatchOutcome {
            write_id: pending.write_id,
            outcome,
            promoted: outcome == WriteVerdictOutcome::Accept,
            upserted,
        })
    }
}

// ── write-or-hold: the call-site helper used from lib.rs ───────────────

/// What happened to an automated write attempt: written straight to the
/// graph (write-governance disabled — today's exact behavior), or held
/// pending a human verdict (enabled).
#[derive(Debug, Clone)]
pub enum WriteOutcome {
    WrittenDirect(usize),
    HeldPending(String),
}

/// The single call-site helper `lib.rs`'s 3 automated-write sites route
/// through. When write-governance is disabled (the shipped default),
/// behaves identically to calling `graph_store.upsert_entities` directly —
/// zero behavior change. When enabled, captures a pending write instead.
/// `pending_writes_dir()` resolves the target directory internally (env-var
/// based, see its own doc comment) — no new parameter needed at call sites.
pub fn write_or_hold(
    graph_store: &Arc<dyn GraphStore>,
    module_id: &str,
    entities: &[GraphEntity],
    source_doc: Option<&str>,
    tier_used: &str,
) -> anyhow::Result<WriteOutcome> {
    if !write_governance_enabled() {
        let n = graph_store.upsert_entities(module_id, entities)?;
        return Ok(WriteOutcome::WrittenDirect(n));
    }
    let write_id = capture_pending_write(
        &pending_writes_dir(),
        module_id,
        entities,
        source_doc,
        tier_used,
    )
    .map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(WriteOutcome::HeldPending(write_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::LbugGraphStore;

    fn tmp_dir(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "service-content-write-governance-{label}-{}",
            Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn entity(name: &str, module_id: &str) -> GraphEntity {
        GraphEntity {
            entity_name: name.to_string(),
            classification: "Company".to_string(),
            role_vector: None,
            location_vector: None,
            contact_vector: None,
            module_id: module_id.to_string(),
            confidence: 0.95,
            source_doc: None,
        }
    }

    #[derive(Debug)]
    struct MockVerifier {
        accept_signature: String,
        accept_identity: String,
        accept_namespace: String,
    }

    impl WriteVerdictVerifier for MockVerifier {
        fn verify(
            &self,
            _body: &str,
            signature_pem: &str,
            reviewer_identity: &str,
            namespace: &str,
        ) -> Result<(), WriteGovernanceError> {
            if signature_pem == self.accept_signature
                && reviewer_identity == self.accept_identity
                && namespace == self.accept_namespace
            {
                Ok(())
            } else {
                Err(WriteGovernanceError::VerifySignature(
                    "mock verifier rejected".into(),
                ))
            }
        }
    }

    fn dispatcher(pending_dir: PathBuf) -> WriteGovernanceDispatcher {
        WriteGovernanceDispatcher {
            verifier: Arc::new(MockVerifier {
                accept_signature: "TRUSTED-SIGNATURE-BLOB".into(),
                accept_identity: "ps-administrator".into(),
                accept_namespace: WRITE_GOVERNANCE_NAMESPACE.into(),
            }),
            pending_dir,
        }
    }

    fn graph_store(dir: &Path) -> LbugGraphStore {
        let store = LbugGraphStore::new(dir.join("lbug").to_str().unwrap()).unwrap();
        store.init_schema().unwrap();
        store
    }

    fn wire(write_id: &str, outcome: &str, notes: Option<&str>) -> WriteVerdictWireBody {
        WriteVerdictWireBody {
            write_id: write_id.to_string(),
            outcome: outcome.to_string(),
            reviewer_identity: "ps-administrator".to_string(),
            notes: notes.map(str::to_string),
            signature: B64.encode("TRUSTED-SIGNATURE-BLOB".as_bytes()),
        }
    }

    #[test]
    fn write_or_hold_writes_direct_when_disabled() {
        std::env::remove_var("SERVICE_CONTENT_WRITE_GOVERNANCE_ENABLED");
        let dir = tmp_dir("disabled");
        let store: Arc<dyn GraphStore> = Arc::new(graph_store(&dir));
        let outcome = write_or_hold(
            &store,
            "pointsav",
            &[entity("X", "pointsav")],
            None,
            "tier-b",
        )
        .unwrap();
        assert!(matches!(outcome, WriteOutcome::WrittenDirect(1)));
        assert_eq!(store.list_entities("pointsav").unwrap().len(), 1);
    }

    #[test]
    fn capture_pending_write_holds_without_touching_graph() {
        let dir = tmp_dir("capture");
        let pending_dir = dir.join("pending");
        let write_id = capture_pending_write(
            &pending_dir,
            "pointsav",
            &[entity("X", "pointsav")],
            None,
            "tier-b",
        )
        .unwrap();
        let pending = list_pending(&pending_dir).unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].write_id, write_id);
        assert!(pending[0].verdict.is_none());
    }

    #[test]
    fn accept_verdict_promotes_entities_into_graph() {
        let dir = tmp_dir("accept");
        let pending_dir = dir.join("pending");
        let write_id = capture_pending_write(
            &pending_dir,
            "pointsav",
            &[entity("X", "pointsav")],
            None,
            "tier-b",
        )
        .unwrap();
        let store = graph_store(&dir);
        let d = dispatcher(pending_dir.clone());

        let outcome = d
            .dispatch(wire(&write_id, "accept", Some("LGTM")), &store)
            .unwrap();
        assert!(outcome.promoted);
        assert_eq!(outcome.upserted, 1);
        assert_eq!(store.list_entities("pointsav").unwrap().len(), 1);

        // Pending artifact retained, now resolved.
        let pending = list_pending(&pending_dir).unwrap();
        assert_eq!(
            pending.len(),
            1,
            "pending artifact retained for audit, not deleted"
        );
        assert!(pending[0].verdict.is_some());
        assert!(pending[0].promoted_at.is_some());
    }

    #[test]
    fn reject_verdict_writes_nothing() {
        let dir = tmp_dir("reject");
        let pending_dir = dir.join("pending");
        let write_id = capture_pending_write(
            &pending_dir,
            "pointsav",
            &[entity("X", "pointsav")],
            None,
            "tier-b",
        )
        .unwrap();
        let store = graph_store(&dir);
        let d = dispatcher(pending_dir.clone());

        let outcome = d.dispatch(wire(&write_id, "reject", None), &store).unwrap();
        assert!(!outcome.promoted);
        assert_eq!(outcome.upserted, 0);
        assert_eq!(
            store.list_entities("pointsav").unwrap().len(),
            0,
            "reject must not write to the graph"
        );

        let pending = list_pending(&pending_dir).unwrap();
        assert!(
            pending[0].verdict.is_some(),
            "reject still records a verdict"
        );
        assert!(pending[0].promoted_at.is_none());
    }

    #[test]
    fn bad_signature_causes_zero_state_mutation() {
        let dir = tmp_dir("bad-sig");
        let pending_dir = dir.join("pending");
        let write_id = capture_pending_write(
            &pending_dir,
            "pointsav",
            &[entity("X", "pointsav")],
            None,
            "tier-b",
        )
        .unwrap();
        let store = graph_store(&dir);
        let d = dispatcher(pending_dir.clone());

        let mut bad = wire(&write_id, "accept", None);
        bad.signature = B64.encode(b"WRONG-SIGNATURE");
        let err = d.dispatch(bad, &store).unwrap_err();
        assert!(matches!(err, WriteGovernanceError::VerifySignature(_)));

        assert_eq!(store.list_entities("pointsav").unwrap().len(), 0);
        let pending = list_pending(&pending_dir).unwrap();
        assert!(
            pending[0].verdict.is_none(),
            "bad signature must not record any verdict"
        );
    }

    #[test]
    fn orphan_verdict_no_pending_write_returns_specific_error() {
        let dir = tmp_dir("orphan");
        let pending_dir = dir.join("pending");
        let store = graph_store(&dir);
        let d = dispatcher(pending_dir);

        let err = d
            .dispatch(wire("nonexistent-write-id", "accept", None), &store)
            .unwrap_err();
        assert!(matches!(err, WriteGovernanceError::OrphanVerdict { .. }));
    }

    #[test]
    fn already_resolved_verdict_rejected_no_double_write() {
        let dir = tmp_dir("double");
        let pending_dir = dir.join("pending");
        let write_id = capture_pending_write(
            &pending_dir,
            "pointsav",
            &[entity("X", "pointsav")],
            None,
            "tier-b",
        )
        .unwrap();
        let store = graph_store(&dir);
        let d = dispatcher(pending_dir.clone());

        d.dispatch(wire(&write_id, "accept", None), &store).unwrap();
        let err = d
            .dispatch(wire(&write_id, "accept", None), &store)
            .unwrap_err();
        assert!(matches!(err, WriteGovernanceError::AlreadyResolved { .. }));

        // Still exactly one upsert happened (not two).
        assert_eq!(store.list_entities("pointsav").unwrap().len(), 1);
    }
}
