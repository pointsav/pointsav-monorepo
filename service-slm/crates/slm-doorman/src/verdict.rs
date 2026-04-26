// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Apprenticeship verdict pipeline (AS-3).
//!
//! `VerdictDispatcher::dispatch` is the AS-3 implementation of
//! `POST /v1/verdict`:
//!
//! 1. Verify the senior's SSH signature over the verdict body
//!    (`ssh-keygen -Y verify -n apprenticeship-verdict-v1` against
//!    `~/Foundry/identity/allowed_signers`). Failure → 403.
//! 2. Parse the verdict frontmatter to recover brief_id, attempt_id,
//!    outcome, etc.
//! 3. Look up the original (brief, attempt) from the in-process
//!    `BriefCache`. Cache miss → `BriefCacheMiss` (HTTP 410-ish; the
//!    senior must reissue the brief).
//! 4. Write the apprenticeship corpus tuple to
//!    `${FOUNDRY_ROOT}/data/training-corpus/apprenticeship/<task-type>/<ulid>.jsonl`.
//!    The redaction filter (`crate::redact::sanitize`) runs over every
//!    body field per convention §9.
//! 5. Append a signed verdict event row to
//!    `data/apprenticeship/ledger.md` under `flock(2)`. Recompute
//!    rolling stats; on threshold cross, append a `promotion` event.
//! 6. On `verdict in [refine, reject]`: also write a DPO pair to
//!    `data/training-corpus/feedback/apprenticeship-<task-type>-<ulid>.jsonl`
//!    per convention §8 + `trajectory-substrate.md` §6.
//!
//! Verdict signature verification is encapsulated in the
//! `VerdictVerifier` trait so tests can inject a `MockVerifier` and the
//! production binary uses `SshKeygenVerifier` (shells out per design-
//! pass Q1; native Rust SSH-key verify is a v0.5+ follow-up).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine as _;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slm_core::{
    ApprenticeshipAttempt, ApprenticeshipBrief, ApprenticeshipVerdict, VerdictOutcome,
    VERDICT_NAMESPACE,
};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::brief_cache::BriefCache;
use crate::error::{DoormanError, Result};
use crate::promotion_ledger::{format_verdict_event, PromotionLedger, PromotionOutcome, StatRow};
use crate::redact::sanitize;

/// Verifies a senior signature over a verdict body. Production uses
/// `SshKeygenVerifier`; tests use `MockVerifier`.
#[async_trait]
pub trait VerdictVerifier: Send + Sync + std::fmt::Debug {
    /// Returns `Ok(())` on accept, `Err(DoormanError::VerifySignature)`
    /// on any failure (bad signature, unknown principal, namespace
    /// mismatch, ssh-keygen exited non-zero).
    async fn verify(
        &self,
        body: &str,
        signature_pem: &str,
        senior_identity: &str,
        namespace: &str,
    ) -> Result<()>;
}

/// Real verifier — shells out to `ssh-keygen -Y verify`. Per design-
/// pass Q1.
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

#[async_trait]
impl VerdictVerifier for SshKeygenVerifier {
    async fn verify(
        &self,
        body: &str,
        signature_pem: &str,
        senior_identity: &str,
        namespace: &str,
    ) -> Result<()> {
        // ssh-keygen -Y verify takes the signature on -s and the body
        // on stdin. Write the signature to a temp file, pipe the body.
        let sig_path = std::env::temp_dir().join(format!(
            "slm-doorman-verify-{}.sig",
            Uuid::now_v7().simple()
        ));
        std::fs::write(&sig_path, signature_pem.as_bytes())
            .map_err(|e| DoormanError::VerifySignature(format!("write sig file: {e}")))?;
        let principal = format!("{senior_identity}@users.noreply.github.com");

        let allowed = self.allowed_signers.clone();
        let sig_for_close = sig_path.clone();
        let body_owned = body.to_string();
        let namespace_owned = namespace.to_string();

        let result = tokio::task::spawn_blocking(move || {
            use std::process::{Command, Stdio};
            let mut child = Command::new("ssh-keygen")
                .arg("-Y")
                .arg("verify")
                .arg("-f")
                .arg(&allowed)
                .arg("-I")
                .arg(&principal)
                .arg("-n")
                .arg(&namespace_owned)
                .arg("-s")
                .arg(&sig_for_close)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| DoormanError::VerifySignature(format!("spawn ssh-keygen: {e}")))?;
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                stdin
                    .write_all(body_owned.as_bytes())
                    .map_err(|e| DoormanError::VerifySignature(format!("write stdin: {e}")))?;
            }
            child
                .wait_with_output()
                .map_err(|e| DoormanError::VerifySignature(format!("wait ssh-keygen: {e}")))
        })
        .await
        .map_err(|e| DoormanError::VerifySignature(format!("join blocking: {e}")))??;

        let _ = std::fs::remove_file(&sig_path);

        if result.status.success() {
            debug!(target: "slm_doorman::verdict", %senior_identity, "ssh-keygen -Y verify OK");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&result.stderr).into_owned();
            warn!(target: "slm_doorman::verdict", %senior_identity, %stderr,
                  "ssh-keygen -Y verify rejected");
            Err(DoormanError::VerifySignature(format!(
                "ssh-keygen -Y verify exited {}: {}",
                result.status, stderr
            )))
        }
    }
}

/// Verdict dispatcher (AS-3). Holds the verifier, brief cache, ledger,
/// and corpus root. One instance per Doorman process; cheap to clone
/// (everything is `Arc`-shared).
#[derive(Clone)]
pub struct VerdictDispatcher {
    pub verifier: Arc<dyn VerdictVerifier>,
    pub cache: Arc<BriefCache>,
    pub ledger: PromotionLedger,
    pub corpus_root: PathBuf,
    pub doctrine_version: String,
    pub tenant: String,
}

/// Wire shape of `POST /v1/verdict` per design-pass Q5. Base64 for the
/// signature blob keeps the JSON body printable.
#[derive(Clone, Debug, Deserialize)]
pub struct VerdictWireBody {
    /// Verbatim verdict-file content (frontmatter + prose) as signed.
    pub body: String,
    /// Base64-encoded SSH signature blob over `body`, namespace
    /// `apprenticeship-verdict-v1` (or `..-batch-v1`).
    pub signature: String,
    /// Senior identity claim — must match the `senior_identity:`
    /// frontmatter line. Cross-check at parse time.
    pub senior_identity: String,
}

/// Outcome of the verdict POST. Returned in the HTTP response body so
/// the senior knows whether their verdict landed and whether the task-
/// type promoted.
#[derive(Clone, Debug, Serialize)]
pub struct VerdictDispatchOutcome {
    pub verdict: VerdictOutcome,
    pub brief_id: String,
    pub attempt_id: String,
    pub corpus_path: String,
    pub promotion: PromotionOutcome,
    pub dpo_pair_path: Option<String>,
}

impl VerdictDispatcher {
    /// AS-3 entry point. Returns a `DoormanError` whose `to_string()`
    /// the HTTP layer surfaces; signature failures map to 403.
    pub async fn dispatch(&self, wire: VerdictWireBody) -> Result<VerdictDispatchOutcome> {
        // Decode signature (the SSH signature is itself an
        // ASCII-armoured PEM-ish block; design-pass Q5 base64-encodes
        // it for JSON transport, so decode back to text).
        let signature_pem = decode_signature(&wire.signature)?;

        // Verify before any state mutation.
        self.verifier
            .verify(
                &wire.body,
                &signature_pem,
                &wire.senior_identity,
                VERDICT_NAMESPACE,
            )
            .await?;

        // Parse verdict frontmatter — recovers brief_id, attempt_id,
        // outcome, senior_identity (cross-checked), final_diff_sha,
        // notes.
        let parsed = parse_verdict_body(&wire.body)?;
        if parsed.senior_identity != wire.senior_identity {
            return Err(DoormanError::VerdictParse(format!(
                "verdict frontmatter senior_identity {:?} does not match wire field {:?}",
                parsed.senior_identity, wire.senior_identity
            )));
        }

        // Look up the brief / attempt that produced this verdict.
        let cached = self
            .cache
            .get(&parsed.brief_id, &parsed.attempt_id)
            .ok_or(DoormanError::BriefCacheMiss)?;

        // Build the verdict struct (sanitised body + the signature).
        let verdict = ApprenticeshipVerdict {
            brief_id: parsed.brief_id.clone(),
            attempt_id: parsed.attempt_id.clone(),
            verdict: parsed.verdict,
            created: parsed.created,
            senior_identity: parsed.senior_identity.clone(),
            final_diff_sha: parsed.final_diff_sha.clone(),
            notes: parsed.notes.clone(),
            body: sanitize(&wire.body),
            signature: wire.signature.clone(),
        };

        // Write the apprenticeship corpus tuple.
        let stage_before = self.ledger.current_stage(&cached.brief.task_type);
        let corpus_path = write_apprenticeship_tuple(
            &self.corpus_root,
            &cached.brief,
            &cached.attempt,
            Some(&verdict),
            parsed.final_diff.as_deref(),
            stage_before.as_str(),
            &self.doctrine_version,
            &self.tenant,
        )?;

        // Append the verdict event to the ledger + recompute stats /
        // promotion.
        let row = StatRow {
            ts: parsed.created,
            task_type: cached.brief.task_type.clone(),
            verdict: parsed.verdict,
            brief_id: parsed.brief_id.clone(),
            attempt_id: parsed.attempt_id.clone(),
            self_confidence: cached.attempt.self_confidence,
            senior_identity: parsed.senior_identity.clone(),
        };
        let body_summary = format!(
            "verdict={} brief_id={} attempt_id={} self_confidence={:.3}",
            parsed.verdict.as_str(),
            parsed.brief_id,
            parsed.attempt_id,
            cached.attempt.self_confidence,
        );
        let event_block = format_verdict_event(
            parsed.created,
            &cached.brief.task_type,
            &parsed.senior_identity,
            &body_summary,
            &signature_pem,
        );
        let promotion = self.ledger.append_verdict(row, &event_block)?;

        // DPO pair on refine / reject.
        let dpo_pair_path = if parsed.verdict.produces_dpo_pair() {
            Some(write_dpo_pair(
                &self.corpus_root,
                &cached.brief.task_type,
                &cached.attempt,
                parsed.final_diff.as_deref().unwrap_or_default(),
                parsed.notes.as_deref().unwrap_or(""),
                &cached.brief.brief_id,
                &cached.attempt.attempt_id,
            )?)
        } else {
            None
        };

        info!(
            target: "slm_doorman::verdict",
            brief_id = %parsed.brief_id,
            attempt_id = %parsed.attempt_id,
            verdict = parsed.verdict.as_str(),
            promoted = promotion.promoted,
            "verdict applied"
        );

        Ok(VerdictDispatchOutcome {
            verdict: parsed.verdict,
            brief_id: parsed.brief_id,
            attempt_id: parsed.attempt_id,
            corpus_path: corpus_path.display().to_string(),
            promotion,
            dpo_pair_path: dpo_pair_path.map(|p| p.display().to_string()),
        })
    }
}

/// Frontmatter shape parsed from a verdict body.
#[derive(Clone, Debug)]
pub struct ParsedVerdict {
    pub brief_id: String,
    pub attempt_id: String,
    pub verdict: VerdictOutcome,
    pub created: DateTime<Utc>,
    pub senior_identity: String,
    pub final_diff_sha: Option<String>,
    pub notes: Option<String>,
    /// Free-form prose body extracted post-frontmatter; carries the
    /// senior's corrected diff or commentary. Used to fill the corpus
    /// tuple's `final_diff` field on accept verdicts.
    pub final_diff: Option<String>,
}

/// Parse the verdict frontmatter shape from `templates/apprenticeship-
/// verdict.md.tmpl`. Required keys: `brief_id`, `attempt_id`,
/// `verdict`, `created`, `senior_identity`. Optional: `final_diff_sha`,
/// `notes`. Robust to comment lines, blank lines, and minor whitespace
/// drift.
pub fn parse_verdict_body(body: &str) -> Result<ParsedVerdict> {
    let frontmatter = extract_frontmatter(body)
        .ok_or_else(|| DoormanError::VerdictParse("missing YAML frontmatter".into()))?;

    let mut brief_id = None;
    let mut attempt_id = None;
    let mut verdict = None;
    let mut created = None;
    let mut senior_identity = None;
    let mut final_diff_sha: Option<String> = None;
    let mut notes: Option<String> = None;

    for line in frontmatter.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let (k, v) = match t.split_once(':') {
            Some(p) => p,
            None => continue,
        };
        let v = v.trim();
        let v_trim = v.trim_matches(&['"', '\''][..]);
        match k.trim() {
            "brief_id" => brief_id = Some(v_trim.to_string()),
            "attempt_id" => attempt_id = Some(v_trim.to_string()),
            "verdict" => {
                verdict = Some(match v_trim {
                    "accept" => VerdictOutcome::Accept,
                    "refine" => VerdictOutcome::Refine,
                    "reject" => VerdictOutcome::Reject,
                    "defer-tier-c" => VerdictOutcome::DeferTierC,
                    other => {
                        return Err(DoormanError::VerdictParse(format!(
                            "unknown verdict outcome {other:?}"
                        )))
                    }
                });
            }
            "created" => {
                created = Some(
                    DateTime::parse_from_rfc3339(v_trim)
                        .map_err(|e| DoormanError::VerdictParse(format!("created: {e}")))?
                        .with_timezone(&Utc),
                );
            }
            "senior_identity" => senior_identity = Some(v_trim.to_string()),
            "final_diff_sha" if !v_trim.is_empty() && v_trim != "null" => {
                final_diff_sha = Some(v_trim.to_string());
            }
            // Single-line notes only at the frontmatter layer; the
            // template uses `notes: |` block scalars but we only need
            // a one-line summary for the audit row.
            "notes" if !v_trim.is_empty() && v_trim != "|" => {
                notes = Some(v_trim.to_string());
            }
            _ => {}
        }
    }

    Ok(ParsedVerdict {
        brief_id: brief_id.ok_or_else(|| DoormanError::VerdictParse("missing brief_id".into()))?,
        attempt_id: attempt_id
            .ok_or_else(|| DoormanError::VerdictParse("missing attempt_id".into()))?,
        verdict: verdict.ok_or_else(|| DoormanError::VerdictParse("missing verdict".into()))?,
        created: created.ok_or_else(|| DoormanError::VerdictParse("missing created".into()))?,
        senior_identity: senior_identity
            .ok_or_else(|| DoormanError::VerdictParse("missing senior_identity".into()))?,
        final_diff_sha,
        notes,
        final_diff: extract_post_frontmatter(body),
    })
}

fn extract_frontmatter(text: &str) -> Option<String> {
    let stripped = text.trim_start_matches([' ', '\t', '\n']);
    let after_open = stripped.strip_prefix("---")?;
    let after_open = after_open.strip_prefix('\n').unwrap_or(after_open);
    let close = after_open.find("\n---")?;
    Some(after_open[..close].to_string())
}

fn extract_post_frontmatter(text: &str) -> Option<String> {
    let stripped = text.trim_start_matches([' ', '\t', '\n']);
    let after_open = stripped.strip_prefix("---\n")?;
    let close_rel = after_open.find("\n---")?;
    let rest = &after_open[close_rel..];
    let rest = rest.strip_prefix("\n---").unwrap_or(rest);
    let rest = rest.strip_prefix('\n').unwrap_or(rest);
    let trimmed = rest.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn decode_signature(b64: &str) -> Result<String> {
    let bytes = B64
        .decode(b64.trim().as_bytes())
        .map_err(|e| DoormanError::VerifySignature(format!("base64 decode: {e}")))?;
    String::from_utf8(bytes)
        .map_err(|e| DoormanError::VerifySignature(format!("utf-8 decode: {e}")))
}

#[allow(clippy::too_many_arguments)]
fn write_apprenticeship_tuple(
    corpus_root: &Path,
    brief: &ApprenticeshipBrief,
    attempt: &ApprenticeshipAttempt,
    verdict: Option<&ApprenticeshipVerdict>,
    final_diff: Option<&str>,
    stage_at_capture: &str,
    doctrine_version: &str,
    tenant: &str,
) -> Result<PathBuf> {
    let dir = corpus_root
        .join("data")
        .join("training-corpus")
        .join("apprenticeship")
        .join(&brief.task_type);
    std::fs::create_dir_all(&dir).map_err(|e| DoormanError::CorpusWrite {
        path: dir.display().to_string(),
        reason: e.to_string(),
    })?;
    let filename = format!("{}.jsonl", Uuid::now_v7().simple());
    let path = dir.join(&filename);

    // Build the corpus tuple. `apprenticeship-substrate.md` §8 schema.
    // Sanitize body fields per convention §9 (briefs, attempts,
    // verdicts all pass through the redaction filter).
    let sanitized_brief = sanitize_brief(brief);
    let sanitized_attempt = sanitize_attempt(attempt);
    let sanitized_verdict = verdict.map(sanitize_verdict);
    let sanitized_final_diff = final_diff.map(sanitize);

    let record = serde_json::json!({
        "tuple_type": "apprenticeship",
        "doctrine_version": doctrine_version,
        "task_type": brief.task_type,
        "stage_at_capture": stage_at_capture,
        "brief": sanitized_brief,
        "attempt": sanitized_attempt,
        "verdict": sanitized_verdict,
        "final_diff": sanitized_final_diff,
        "redaction_class": "internal",
        "evidence_class": "primary",
        "tenant": tenant,
        "cluster": brief.scope.cluster,
        "session_id": null,
        "created": Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    });

    let line = serde_json::to_string(&record).map_err(|e| DoormanError::CorpusWrite {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    std::fs::write(&path, format!("{line}\n")).map_err(|e| DoormanError::CorpusWrite {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    Ok(path)
}

fn sanitize_brief(b: &ApprenticeshipBrief) -> ApprenticeshipBrief {
    let mut clone = b.clone();
    clone.body = sanitize(&clone.body);
    clone.acceptance_test = sanitize(&clone.acceptance_test);
    clone
}

fn sanitize_attempt(a: &ApprenticeshipAttempt) -> ApprenticeshipAttempt {
    let mut clone = a.clone();
    clone.reasoning = sanitize(&clone.reasoning);
    clone.diff = sanitize(&clone.diff);
    clone
}

fn sanitize_verdict(v: &ApprenticeshipVerdict) -> ApprenticeshipVerdict {
    let mut clone = v.clone();
    clone.body = sanitize(&clone.body);
    if let Some(n) = clone.notes.as_ref() {
        clone.notes = Some(sanitize(n));
    }
    clone
}

fn write_dpo_pair(
    corpus_root: &Path,
    task_type: &str,
    rejected: &ApprenticeshipAttempt,
    corrected_diff: &str,
    doctrine_violation_tag: &str,
    brief_id: &str,
    attempt_id: &str,
) -> Result<PathBuf> {
    let dir = corpus_root
        .join("data")
        .join("training-corpus")
        .join("feedback");
    std::fs::create_dir_all(&dir).map_err(|e| DoormanError::CorpusWrite {
        path: dir.display().to_string(),
        reason: e.to_string(),
    })?;
    let filename = format!(
        "apprenticeship-{}-{}.jsonl",
        task_type,
        Uuid::now_v7().simple()
    );
    let path = dir.join(&filename);
    let record = serde_json::json!({
        "tuple_type": "apprenticeship-feedback",
        "task_type": task_type,
        "brief_id": brief_id,
        "attempt_id": attempt_id,
        "rejected_attempt": sanitize_attempt(rejected),
        "corrected_diff": sanitize(corrected_diff),
        "doctrine_violation_tag": doctrine_violation_tag,
        "created": Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    });
    let line = serde_json::to_string(&record).map_err(|e| DoormanError::CorpusWrite {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    std::fs::write(&path, format!("{line}\n")).map_err(|e| DoormanError::CorpusWrite {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use slm_core::{BriefScope, SeniorRole, Tier};
    use std::path::PathBuf;

    /// Test verifier — accepts only when `senior_identity` matches a
    /// preconfigured value AND signature equals a known marker.
    /// Lets us exercise both happy path and rejection without
    /// shelling out to ssh-keygen.
    #[derive(Debug)]
    struct MockVerifier {
        accept_signature: String,
        accept_identity: String,
        accept_namespace: String,
    }

    #[async_trait]
    impl VerdictVerifier for MockVerifier {
        async fn verify(
            &self,
            _body: &str,
            signature_pem: &str,
            senior_identity: &str,
            namespace: &str,
        ) -> Result<()> {
            if signature_pem == self.accept_signature
                && senior_identity == self.accept_identity
                && namespace == self.accept_namespace
            {
                Ok(())
            } else {
                Err(DoormanError::VerifySignature(
                    "mock verifier rejected".into(),
                ))
            }
        }
    }

    fn tmp_dir(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "slm-doorman-verdict-{label}-{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn ts() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 4, 26, 16, 30, 0).unwrap()
    }

    fn brief(brief_id: &str) -> ApprenticeshipBrief {
        ApprenticeshipBrief {
            brief_id: brief_id.into(),
            created: ts(),
            senior_role: SeniorRole::Master,
            senior_identity: "ps-administrator".into(),
            task_type: "version-bump-manifest".into(),
            scope: BriefScope::default(),
            acceptance_test: "T".into(),
            doctrine_citations: vec![],
            shadow: false,
            body: "B".into(),
        }
    }

    fn attempt(brief_id: &str, attempt_id: &str) -> ApprenticeshipAttempt {
        ApprenticeshipAttempt {
            brief_id: brief_id.into(),
            attempt_id: attempt_id.into(),
            created: ts(),
            model: "olmo-3-1125-7b-q4".into(),
            adapter_composition: vec![],
            self_confidence: 0.9,
            escalate: false,
            inference_ms: 100,
            tier: Tier::Local,
            cost_usd: 0.0,
            reasoning: String::new(),
            diff: "+ stub".into(),
        }
    }

    fn verdict_body_text(brief_id: &str, attempt_id: &str, outcome: &str) -> String {
        format!(
            "---\n\
             schema: foundry-apprentice-verdict-v1\n\
             brief_id: {brief_id}\n\
             attempt_id: {attempt_id}\n\
             verdict: {outcome}\n\
             created: 2026-04-26T16:30:00Z\n\
             senior_identity: ps-administrator\n\
             final_diff_sha: 0123456789abcdef0123456789abcdef01234567\n\
             notes: LGTM\n\
             ---\n\
             \n\
             # Verdict\n\
             \n\
             LGTM.\n"
        )
    }

    fn dispatcher(
        corpus: PathBuf,
        ledger_dir: PathBuf,
        cache: Arc<BriefCache>,
    ) -> VerdictDispatcher {
        let verifier: Arc<dyn VerdictVerifier> = Arc::new(MockVerifier {
            accept_signature: "TRUSTED-SIGNATURE-BLOB".into(),
            accept_identity: "ps-administrator".into(),
            accept_namespace: VERDICT_NAMESPACE.into(),
        });
        VerdictDispatcher {
            verifier,
            cache,
            ledger: PromotionLedger::new(ledger_dir).unwrap(),
            corpus_root: corpus,
            doctrine_version: "0.0.7".into(),
            tenant: "pointsav".into(),
        }
    }

    /// Test 1 — happy-path signed verdict accepted; corpus tuple
    /// written to the expected path with the expected schema.
    #[tokio::test]
    async fn happy_path_signed_verdict_writes_corpus_tuple() {
        let corpus = tmp_dir("corpus-1");
        let ledger_dir = tmp_dir("ledger-1");
        let cache = Arc::new(BriefCache::new(8));
        cache.insert(brief("b1"), attempt("b1", "a1"));

        let dispatcher = dispatcher(corpus.clone(), ledger_dir, cache);
        let body = verdict_body_text("b1", "a1", "accept");
        let signature_b64 = B64.encode("TRUSTED-SIGNATURE-BLOB".as_bytes());

        let outcome = dispatcher
            .dispatch(VerdictWireBody {
                body: body.clone(),
                signature: signature_b64,
                senior_identity: "ps-administrator".into(),
            })
            .await
            .expect("happy path verdict accepted");
        assert_eq!(outcome.verdict, VerdictOutcome::Accept);
        assert!(
            outcome.dpo_pair_path.is_none(),
            "accept produces no DPO pair"
        );

        // Corpus tuple landed in the expected directory.
        let expected_dir = corpus
            .join("data")
            .join("training-corpus")
            .join("apprenticeship")
            .join("version-bump-manifest");
        let entries: Vec<_> = std::fs::read_dir(&expected_dir).unwrap().collect();
        assert_eq!(entries.len(), 1, "exactly one corpus row");

        let written = entries.into_iter().next().unwrap().unwrap().path();
        let content = std::fs::read_to_string(&written).unwrap();
        let row: serde_json::Value = serde_json::from_str(content.trim()).unwrap();
        assert_eq!(row["tuple_type"], "apprenticeship");
        assert_eq!(row["doctrine_version"], "0.0.7");
        assert_eq!(row["task_type"], "version-bump-manifest");
        assert_eq!(row["stage_at_capture"], "review");
        assert_eq!(row["redaction_class"], "internal");
        assert_eq!(row["evidence_class"], "primary");
        assert_eq!(row["tenant"], "pointsav");
        assert_eq!(row["brief"]["brief_id"], "b1");
        assert_eq!(row["attempt"]["attempt_id"], "a1");
        assert_eq!(row["verdict"]["verdict"], "accept");
    }

    /// Test 2 — bad signature is rejected; no corpus write, no ledger
    /// row.
    #[tokio::test]
    async fn bad_signature_rejected_no_state_mutation() {
        let corpus = tmp_dir("corpus-2");
        let ledger_dir = tmp_dir("ledger-2");
        let cache = Arc::new(BriefCache::new(8));
        cache.insert(brief("b1"), attempt("b1", "a1"));
        let dispatcher = dispatcher(corpus.clone(), ledger_dir.clone(), cache);

        let body = verdict_body_text("b1", "a1", "accept");
        let bogus_b64 = B64.encode("WRONG-SIG".as_bytes());

        let err = dispatcher
            .dispatch(VerdictWireBody {
                body,
                signature: bogus_b64,
                senior_identity: "ps-administrator".into(),
            })
            .await
            .expect_err("bad signature must fail");
        assert!(matches!(err, DoormanError::VerifySignature(_)));

        // No corpus directory created.
        let expected_dir = corpus
            .join("data")
            .join("training-corpus")
            .join("apprenticeship")
            .join("version-bump-manifest");
        assert!(
            !expected_dir.exists(),
            "rejected verdict MUST NOT write corpus"
        );
        // No ledger.md created.
        assert!(
            !ledger_dir.join("ledger.md").exists(),
            "rejected verdict MUST NOT append ledger row"
        );
    }

    /// Test 3 — synthesise 50 accept verdicts; the 50th must trigger a
    /// `promotion` event and the dispatcher's outcome carries
    /// `promoted=true` with stage_after=spot-check.
    #[tokio::test]
    async fn ledger_promotion_fires_on_50_accepts() {
        let corpus = tmp_dir("corpus-3");
        let ledger_dir = tmp_dir("ledger-3");
        let cache = Arc::new(BriefCache::new(64));
        let dispatcher = dispatcher(corpus.clone(), ledger_dir.clone(), cache.clone());

        for i in 0..49 {
            let bid = format!("b{i}");
            let aid = format!("a{i}");
            cache.insert(brief(&bid), attempt(&bid, &aid));
            let body = verdict_body_text(&bid, &aid, "accept");
            let sig_b64 = B64.encode("TRUSTED-SIGNATURE-BLOB".as_bytes());
            let outcome = dispatcher
                .dispatch(VerdictWireBody {
                    body,
                    signature: sig_b64,
                    senior_identity: "ps-administrator".into(),
                })
                .await
                .unwrap();
            assert!(!outcome.promotion.promoted, "below threshold at i={i}");
        }
        cache.insert(brief("b50"), attempt("b50", "a50"));
        let body = verdict_body_text("b50", "a50", "accept");
        let sig_b64 = B64.encode("TRUSTED-SIGNATURE-BLOB".as_bytes());
        let outcome = dispatcher
            .dispatch(VerdictWireBody {
                body,
                signature: sig_b64,
                senior_identity: "ps-administrator".into(),
            })
            .await
            .unwrap();
        assert!(outcome.promotion.promoted, "50th accept must promote");
        assert_eq!(
            outcome.promotion.stage_after,
            crate::promotion_ledger::Stage::SpotCheck
        );

        let md = std::fs::read_to_string(ledger_dir.join("ledger.md")).unwrap();
        assert!(md.contains("promotion  version-bump-manifest"));
    }

    /// Refine verdict produces a DPO pair under data/training-corpus/feedback/.
    #[tokio::test]
    async fn refine_verdict_writes_dpo_pair() {
        let corpus = tmp_dir("corpus-dpo");
        let ledger_dir = tmp_dir("ledger-dpo");
        let cache = Arc::new(BriefCache::new(8));
        cache.insert(brief("b1"), attempt("b1", "a1"));
        let dispatcher = dispatcher(corpus.clone(), ledger_dir, cache);

        let body = verdict_body_text("b1", "a1", "refine");
        let sig_b64 = B64.encode("TRUSTED-SIGNATURE-BLOB".as_bytes());
        let outcome = dispatcher
            .dispatch(VerdictWireBody {
                body,
                signature: sig_b64,
                senior_identity: "ps-administrator".into(),
            })
            .await
            .unwrap();
        assert_eq!(outcome.verdict, VerdictOutcome::Refine);
        let dpo_path = outcome.dpo_pair_path.expect("refine writes DPO pair");
        let content = std::fs::read_to_string(&dpo_path).unwrap();
        let row: serde_json::Value = serde_json::from_str(content.trim()).unwrap();
        assert_eq!(row["tuple_type"], "apprenticeship-feedback");
        assert_eq!(row["task_type"], "version-bump-manifest");
        assert_eq!(row["doctrine_violation_tag"], "LGTM");
    }

    /// Frontmatter parser handles all required keys.
    #[test]
    fn parse_verdict_body_extracts_required_fields() {
        let body = verdict_body_text("b1", "a1", "accept");
        let p = parse_verdict_body(&body).unwrap();
        assert_eq!(p.brief_id, "b1");
        assert_eq!(p.attempt_id, "a1");
        assert_eq!(p.verdict, VerdictOutcome::Accept);
        assert_eq!(p.senior_identity, "ps-administrator");
        assert_eq!(
            p.final_diff_sha.as_deref(),
            Some("0123456789abcdef0123456789abcdef01234567")
        );
        assert_eq!(p.notes.as_deref(), Some("LGTM"));
    }

    /// Brief-cache miss surfaces as `BriefCacheMiss`.
    #[tokio::test]
    async fn cache_miss_surfaces_briefly() {
        let corpus = tmp_dir("corpus-miss");
        let ledger_dir = tmp_dir("ledger-miss");
        let cache = Arc::new(BriefCache::new(8));
        // Don't insert anything — cache is empty.
        let dispatcher = dispatcher(corpus, ledger_dir, cache);
        let body = verdict_body_text("b1", "a1", "accept");
        let sig_b64 = B64.encode("TRUSTED-SIGNATURE-BLOB".as_bytes());
        let err = dispatcher
            .dispatch(VerdictWireBody {
                body,
                signature: sig_b64,
                senior_identity: "ps-administrator".into(),
            })
            .await
            .expect_err("cache miss must fail");
        assert!(matches!(err, DoormanError::BriefCacheMiss));
    }
}
