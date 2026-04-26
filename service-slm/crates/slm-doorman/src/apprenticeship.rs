// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Apprenticeship Substrate dispatcher (AS-2).
//!
//! `ApprenticeshipDispatcher::dispatch_brief` is the AS-2 implementation
//! of `POST /v1/brief`: assemble the apprentice prompt from a brief
//! (resolved citations, redacted scope.files contents, brief body,
//! acceptance test), route through the existing `Doorman::route` so
//! the audit ledger captures the call, parse the apprentice's
//! YAML-frontmatter / fenced-diff response, return an
//! `ApprenticeshipAttempt`.
//!
//! Per Master's 2026-04-26 inbox brief, AS-3 (verdict) and AS-4
//! (shadow) follow; this module exposes shared helpers
//! (`apprentice_prompt`, `parse_attempt_content`,
//! `pick_tier_for_brief`) so the verdict / shadow paths reuse them.

use std::path::{Path, PathBuf};

use chrono::Utc;
use regex::Regex;
use slm_core::{
    ApprenticeshipAttempt, ApprenticeshipBrief, ChatMessage, Complexity, ComputeRequest, ModuleId,
    RequestId, Tier, APPRENTICE_ESCALATE_THRESHOLD, DEFAULT_BRIEF_TIER_B_THRESHOLD_CHARS,
};
use std::str::FromStr;
use std::sync::OnceLock;
use tracing::{debug, info};
use uuid::Uuid;

use crate::citations::{render_for_prompt, resolve as resolve_citations};
use crate::error::{DoormanError, Result};
use crate::redact::sanitize;
use crate::router::Doorman;

/// Per-deployment config for the apprenticeship dispatcher.
#[derive(Clone, Debug)]
pub struct ApprenticeshipConfig {
    /// Workspace root (e.g. `/srv/foundry`). `scope.files` paths
    /// resolve against this root; the citation registry is read from
    /// `<foundry_root>/citations.yaml` unless overridden.
    pub foundry_root: PathBuf,
    /// Path to the citations YAML registry. Defaults to
    /// `<foundry_root>/citations.yaml`.
    pub citations_path: PathBuf,
    /// Char-budget proxy: briefs whose `body.len() +
    /// acceptance_test.len()` exceeds this threshold dispatch to
    /// Tier B. Design-pass Q6.
    pub brief_tier_b_threshold_chars: usize,
}

impl ApprenticeshipConfig {
    /// Default config rooted at the operator's `FOUNDRY_ROOT` env var
    /// (falling back to `/srv/foundry` per Master's brief). Resolves
    /// `citations.yaml` under that root and uses
    /// `DEFAULT_BRIEF_TIER_B_THRESHOLD_CHARS` for Tier-B routing.
    pub fn from_env() -> Self {
        let foundry_root: PathBuf = std::env::var_os("FOUNDRY_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/srv/foundry"));
        let citations_path = foundry_root.join("citations.yaml");
        let threshold = std::env::var("SLM_BRIEF_TIER_B_THRESHOLD_CHARS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(DEFAULT_BRIEF_TIER_B_THRESHOLD_CHARS);
        Self {
            foundry_root,
            citations_path,
            brief_tier_b_threshold_chars: threshold,
        }
    }
}

pub struct ApprenticeshipDispatcher<'a> {
    doorman: &'a Doorman,
    config: ApprenticeshipConfig,
    /// In-process brief / attempt cache, populated on every successful
    /// dispatch_brief so the AS-3 verdict path can recover the
    /// `(brief, attempt)` pair by `(brief_id, attempt_id)`. Optional —
    /// AS-2 unit tests construct without one when they only care
    /// about the routing path.
    cache: Option<std::sync::Arc<crate::brief_cache::BriefCache>>,
}

impl<'a> ApprenticeshipDispatcher<'a> {
    pub fn new(doorman: &'a Doorman, config: ApprenticeshipConfig) -> Self {
        Self {
            doorman,
            config,
            cache: None,
        }
    }

    /// Like [`new`] but also wires a brief cache so the produced
    /// attempt becomes recoverable by the AS-3 verdict path.
    pub fn with_cache(
        doorman: &'a Doorman,
        config: ApprenticeshipConfig,
        cache: std::sync::Arc<crate::brief_cache::BriefCache>,
    ) -> Self {
        Self {
            doorman,
            config,
            cache: Some(cache),
        }
    }

    /// AS-2 entry point. Compose the apprentice prompt from `brief`,
    /// dispatch through `Doorman::route`, parse the response,
    /// return an `ApprenticeshipAttempt`. Inserts `(brief, attempt)`
    /// into the configured cache so AS-3 can recover them at verdict
    /// time.
    pub async fn dispatch_brief(
        &self,
        brief: &ApprenticeshipBrief,
    ) -> Result<ApprenticeshipAttempt> {
        let prompt = apprentice_prompt(&self.config, brief);
        let tier_hint = pick_tier_for_brief(brief, self.config.brief_tier_b_threshold_chars);

        let module_id = brief
            .scope
            .cluster
            .as_deref()
            .and_then(|c| ModuleId::from_str(c).ok())
            .unwrap_or_else(|| {
                ModuleId::from_str("foundry").expect("compile-time-valid default moduleId")
            });

        let req = ComputeRequest {
            request_id: RequestId::new(),
            module_id,
            model: None,
            messages: vec![
                ChatMessage {
                    role: "system".into(),
                    content: APPRENTICE_SYSTEM_PROMPT.to_string(),
                },
                ChatMessage {
                    role: "user".into(),
                    content: prompt,
                },
            ],
            complexity: match tier_hint {
                Tier::Yoyo => Complexity::High,
                _ => Complexity::Medium,
            },
            tier_hint: Some(tier_hint),
            stream: false,
            max_tokens: None,
            temperature: Some(0.0),
            sanitised_outbound: true,
            tier_c_label: None,
        };

        info!(
            target: "slm_doorman::apprenticeship",
            brief_id = %brief.brief_id,
            task_type = %brief.task_type,
            tier = tier_hint.as_str(),
            "dispatching apprentice brief"
        );

        let resp = self.doorman.route(&req).await?;
        let parsed = parse_attempt_content(&resp.content);
        let attempt = build_attempt(brief, &resp, parsed);
        if let Some(cache) = &self.cache {
            cache.insert(brief.clone(), attempt.clone());
        }
        Ok(attempt)
    }
}

/// System message prepended to every apprentice prompt. Frames the
/// role per Doctrine claim #32 and pins the response shape so the
/// parser has a stable target.
pub const APPRENTICE_SYSTEM_PROMPT: &str = "\
You are the Foundry apprentice. Per Doctrine claim #32 (the Apprenticeship Substrate), \
you are first responder on code-shaped work; a senior reviewer (Master, Root, or Task \
Claude) will sign-off your attempt. Be precise. Cite brief invariants and doctrine \
clauses by ID. Produce a unified diff that makes the acceptance test pass. If you are \
not confident you can satisfy the brief, set escalate=true with self_confidence below \
0.5 and an empty diff — that surfaces the gap to the senior cleanly.";

/// Build the apprentice user-prompt body from a brief.
pub fn apprentice_prompt(cfg: &ApprenticeshipConfig, brief: &ApprenticeshipBrief) -> String {
    let resolved = resolve_citations(&cfg.citations_path, &brief.doctrine_citations);
    let citations_block = render_for_prompt(&resolved);
    let files_block = render_files(cfg.foundry_root.as_path(), &brief.scope.files);

    format!(
        "## Brief — {brief_id}\n\
         task_type: {task_type}\n\
         senior: {senior}\n\n\
         ## Doctrine citations\n\
         {citations}\n\
         ## Files in scope\n\
         {files}\n\
         ## Brief body\n\
         {body}\n\n\
         ## Acceptance test\n\
         {acceptance}\n\n\
         ## Required response shape\n\
         Respond with exactly this YAML frontmatter, then `## Reasoning` and `## Diff`:\n\
         \n\
         ---\n\
         self_confidence: <0.0..=1.0>\n\
         escalate: <true|false>\n\
         ---\n\
         \n\
         ## Reasoning\n\
         <your reasoning here>\n\
         \n\
         ## Diff\n\
         ```diff\n\
         <unified diff, OR empty if escalate=true>\n\
         ```\n",
        brief_id = brief.brief_id,
        task_type = brief.task_type,
        senior = brief.senior_identity,
        citations = citations_block,
        files = files_block,
        body = brief.body,
        acceptance = brief.acceptance_test,
    )
}

/// Read each file in `paths` from `root`, sanitize, fence in markdown.
/// Missing files are reported inline so the apprentice sees the gap
/// rather than receiving silently-empty context.
fn render_files(root: &Path, paths: &[String]) -> String {
    if paths.is_empty() {
        return "(no files in scope)\n".to_string();
    }
    let mut out = String::new();
    for rel in paths {
        let p = root.join(rel);
        out.push_str("### ");
        out.push_str(rel);
        out.push('\n');
        match std::fs::read_to_string(&p) {
            Ok(body) => {
                let red = sanitize(&body);
                out.push_str("```\n");
                out.push_str(&red);
                if !red.ends_with('\n') {
                    out.push('\n');
                }
                out.push_str("```\n");
            }
            Err(e) => {
                debug!(target: "slm_doorman::apprenticeship",
                       path = %p.display(), error = %e,
                       "scope.files entry not readable; surfacing to apprentice");
                out.push_str("(file not readable: ");
                out.push_str(&e.to_string());
                out.push_str(")\n");
            }
        }
    }
    out
}

/// Pick a tier hint for a brief. Char-based proxy per design-pass Q6.
pub fn pick_tier_for_brief(brief: &ApprenticeshipBrief, threshold_chars: usize) -> Tier {
    let size = brief.body.len() + brief.acceptance_test.len();
    if size > threshold_chars {
        Tier::Yoyo
    } else {
        Tier::Local
    }
}

/// Parsed view of the apprentice's response body — what we extract
/// from the OpenAI-compatible `content` string.
#[derive(Clone, Debug)]
pub struct ParsedAttempt {
    pub self_confidence: f32,
    pub escalate: bool,
    pub reasoning: String,
    pub diff: String,
}

impl ParsedAttempt {
    /// Sentinel for "apprentice did not return a parseable response".
    /// We treat that as a self-escalation rather than an error so the
    /// senior gets a clear signal.
    pub fn unparsed() -> Self {
        Self {
            self_confidence: 0.0,
            escalate: true,
            reasoning: String::new(),
            diff: String::new(),
        }
    }
}

/// Parse the apprentice's response body. Robust to minor formatting
/// drift; missing pieces become `unparsed()` defaults rather than
/// errors.
pub fn parse_attempt_content(content: &str) -> ParsedAttempt {
    let frontmatter = extract_frontmatter(content);
    let mut self_confidence = 0.0_f32;
    let mut escalate = true;
    if let Some(fm) = frontmatter {
        for line in fm.lines() {
            let t = line.trim();
            if let Some(v) = t.strip_prefix("self_confidence:") {
                if let Ok(f) = v.trim().parse::<f32>() {
                    self_confidence = f.clamp(0.0, 1.0);
                }
            } else if let Some(v) = t.strip_prefix("escalate:") {
                escalate = matches!(v.trim().to_ascii_lowercase().as_str(), "true");
            }
        }
    }

    // Apply the threshold rule (design-pass Q2 + convention §4).
    if self_confidence < APPRENTICE_ESCALATE_THRESHOLD {
        escalate = true;
    }

    let reasoning = extract_section(content, "## Reasoning");
    let diff = extract_diff_block(content).unwrap_or_default();
    let diff = if escalate { String::new() } else { diff };

    ParsedAttempt {
        self_confidence,
        escalate,
        reasoning,
        diff,
    }
}

/// Compose an `ApprenticeshipAttempt` from the brief, the routed
/// `ComputeResponse`, and the parsed response body.
pub fn build_attempt(
    brief: &ApprenticeshipBrief,
    resp: &slm_core::ComputeResponse,
    parsed: ParsedAttempt,
) -> ApprenticeshipAttempt {
    ApprenticeshipAttempt {
        brief_id: brief.brief_id.clone(),
        attempt_id: Uuid::now_v7().simple().to_string(),
        created: Utc::now(),
        model: resp.model.clone(),
        adapter_composition: Vec::new(),
        self_confidence: parsed.self_confidence,
        escalate: parsed.escalate,
        inference_ms: resp.inference_ms,
        tier: resp.tier_used,
        cost_usd: resp.cost_usd,
        reasoning: parsed.reasoning,
        diff: parsed.diff,
    }
}

fn extract_frontmatter(text: &str) -> Option<String> {
    static FRONTMATTER: OnceLock<Regex> = OnceLock::new();
    let re = FRONTMATTER
        .get_or_init(|| Regex::new(r"(?s)\A\s*---\s*\n(.*?)\n---\s*\n").expect("static regex"));
    re.captures(text).map(|c| c[1].to_string())
}

fn extract_section(text: &str, header: &str) -> String {
    if let Some(start) = text.find(header) {
        let after = &text[start + header.len()..];
        // Section ends at the next `## ` header or end of text.
        if let Some(end_rel) = after.find("\n## ") {
            return after[..end_rel].trim().to_string();
        }
        return after.trim().to_string();
    }
    String::new()
}

fn extract_diff_block(text: &str) -> Option<String> {
    static DIFF_FENCE: OnceLock<Regex> = OnceLock::new();
    let re =
        DIFF_FENCE.get_or_init(|| Regex::new(r"(?s)```diff\s*\n(.*?)\n```").expect("static regex"));
    re.captures(text).map(|c| c[1].to_string())
}

/// Apprentice attempt failed entirely (the apprentice did not parse
/// to anything actionable). Used when callers want to short-circuit
/// without going through the routed response.
pub fn empty_attempt(
    brief: &ApprenticeshipBrief,
    model: &str,
    tier: Tier,
) -> ApprenticeshipAttempt {
    ApprenticeshipAttempt {
        brief_id: brief.brief_id.clone(),
        attempt_id: Uuid::now_v7().simple().to_string(),
        created: Utc::now(),
        model: model.to_string(),
        adapter_composition: Vec::new(),
        self_confidence: 0.0,
        escalate: true,
        inference_ms: 0,
        tier,
        cost_usd: 0.0,
        reasoning: String::new(),
        diff: String::new(),
    }
}

// Currently unused inside this module but exported for AS-3 / AS-4
// reuse (corpus tuple writers will need to surface DoormanError on
// non-routable apprentice responses).
#[allow(dead_code)]
fn _doorman_error_marker() -> Option<DoormanError> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::AuditLedger;
    use crate::router::DoormanConfig;
    use crate::tier::{
        BearerTokenProvider, LocalTierClient, LocalTierConfig, PricingConfig, StaticBearer,
        YoYoTierClient, YoYoTierConfig,
    };
    use chrono::TimeZone;
    use slm_core::{BriefScope, SeniorRole};
    use std::path::PathBuf;
    use std::sync::Arc;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn ts() -> chrono::DateTime<chrono::Utc> {
        Utc.with_ymd_and_hms(2026, 4, 26, 16, 0, 0).unwrap()
    }

    fn tmp_dir(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "slm-doorman-as2-{label}-{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn ledger() -> AuditLedger {
        AuditLedger::new(tmp_dir("ledger")).unwrap()
    }

    fn dispatcher_config(root: PathBuf) -> ApprenticeshipConfig {
        ApprenticeshipConfig {
            citations_path: root.join("citations.yaml"),
            foundry_root: root,
            brief_tier_b_threshold_chars: 100, // small for tests
        }
    }

    fn brief_for(body: &str) -> ApprenticeshipBrief {
        ApprenticeshipBrief {
            brief_id: "01J9TESTBRIEF0000000000000".into(),
            created: ts(),
            senior_role: SeniorRole::Master,
            senior_identity: "ps-administrator".into(),
            task_type: "version-bump-manifest".into(),
            scope: BriefScope {
                cluster: None,
                files: vec![],
            },
            acceptance_test: "TEST".into(),
            doctrine_citations: vec![],
            shadow: false,
            body: body.into(),
        }
    }

    fn ok_completion(content: &str) -> serde_json::Value {
        serde_json::json!({
            "choices": [
                { "message": { "role": "assistant", "content": content } }
            ]
        })
    }

    /// Happy path — apprentice returns parseable response with a diff;
    /// dispatcher returns an attempt with `escalate=false` and
    /// non-empty diff.
    #[tokio::test]
    async fn happy_path_brief_returns_attempt_with_diff() {
        let server = MockServer::start().await;
        let apprentice_response = "\
---
self_confidence: 0.82
escalate: false
---

## Reasoning

Bumping MANIFEST.md per ni-51-102 forward-looking marker.

## Diff

```diff
--- a/MANIFEST.md
+++ b/MANIFEST.md
@@ -1 +1 @@
-source_version: 0.1.0
+source_version: 0.2.0
```
";
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_completion(apprentice_response)),
            )
            .expect(1)
            .mount(&server)
            .await;

        let local = LocalTierClient::new(LocalTierConfig {
            endpoint: server.uri(),
            default_model: "olmo-3-1125-7b-q4".into(),
        });
        let doorman = Doorman::new(
            DoormanConfig {
                local: Some(local),
                yoyo: None,
                external: None,
            },
            ledger(),
        );

        let dir = tmp_dir("foundry");
        let cfg = dispatcher_config(dir);
        let dispatcher = ApprenticeshipDispatcher::new(&doorman, cfg);

        let attempt = dispatcher
            .dispatch_brief(&brief_for("small brief body"))
            .await
            .expect("happy path");
        assert_eq!(attempt.brief_id, "01J9TESTBRIEF0000000000000");
        assert!(
            !attempt.escalate,
            "escalate should be false on confident response"
        );
        assert!((attempt.self_confidence - 0.82).abs() < 1e-3);
        assert!(attempt.diff.contains("source_version: 0.2.0"));
        assert_eq!(attempt.tier, Tier::Local);
        assert!(attempt.reasoning.contains("forward-looking"));
    }

    /// Escalate-on-low-confidence — apprentice reports
    /// `self_confidence < APPRENTICE_ESCALATE_THRESHOLD`; dispatcher
    /// forces `escalate = true` and empties the diff.
    #[tokio::test]
    async fn low_confidence_escalates_with_empty_diff() {
        let server = MockServer::start().await;
        let apprentice_response = "\
---
self_confidence: 0.21
escalate: false
---

## Reasoning

I'm not sure how to apply this safely.

## Diff

```diff
--- a/MANIFEST.md
+++ b/MANIFEST.md
@@ -1 +1 @@
-x
+y
```
";
        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_completion(apprentice_response)),
            )
            .expect(1)
            .mount(&server)
            .await;

        let local = LocalTierClient::new(LocalTierConfig {
            endpoint: server.uri(),
            default_model: "olmo-3-1125-7b-q4".into(),
        });
        let doorman = Doorman::new(
            DoormanConfig {
                local: Some(local),
                yoyo: None,
                external: None,
            },
            ledger(),
        );

        let dir = tmp_dir("foundry");
        let cfg = dispatcher_config(dir);
        let dispatcher = ApprenticeshipDispatcher::new(&doorman, cfg);

        let attempt = dispatcher
            .dispatch_brief(&brief_for("small body"))
            .await
            .expect("dispatch ok even for low-confidence response");
        assert!(
            attempt.escalate,
            "Doorman MUST force escalate=true when self_confidence < {APPRENTICE_ESCALATE_THRESHOLD}"
        );
        assert_eq!(attempt.diff, "", "diff MUST be emptied on escalate");
        assert!((attempt.self_confidence - 0.21).abs() < 1e-3);
    }

    /// Tier-B dispatch — brief whose body+acceptance exceeds the
    /// threshold lands on the Yo-Yo tier; verified by the response's
    /// `tier_used` and by mounting the mock at the Yo-Yo server.
    #[tokio::test]
    async fn large_brief_dispatches_to_tier_b() {
        // Two mock servers — Tier A and Tier B. Only Tier B should
        // receive a request; Tier A is configured but mounted with
        // zero expected calls.
        let server_a = MockServer::start().await;
        let server_b = MockServer::start().await;

        let apprentice_response = "\
---
self_confidence: 0.91
escalate: false
---

## Reasoning

OK.

## Diff

```diff
--- a/foo
+++ b/foo
@@ -1 +1 @@
-a
+b
```
";

        Mock::given(method("POST"))
            .and(path("/v1/chat/completions"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(ok_completion(apprentice_response)),
            )
            .expect(1) // Tier B receives the only call
            .mount(&server_b)
            .await;
        // Tier A: not mounted; received_requests asserted to 0 below.

        let local = LocalTierClient::new(LocalTierConfig {
            endpoint: server_a.uri(),
            default_model: "olmo-3-1125-7b-q4".into(),
        });
        let bearer: Arc<dyn BearerTokenProvider> = Arc::new(StaticBearer::new("test"));
        let yoyo = YoYoTierClient::new(
            YoYoTierConfig {
                endpoint: server_b.uri(),
                default_model: "Olmo-3-1125-32B-Think".into(),
                contract_version: crate::YOYO_CONTRACT_VERSION.into(),
                pricing: PricingConfig::default(),
            },
            bearer,
        );
        let doorman = Doorman::new(
            DoormanConfig {
                local: Some(local),
                yoyo: Some(yoyo),
                external: None,
            },
            ledger(),
        );

        let dir = tmp_dir("foundry");
        let cfg = dispatcher_config(dir); // threshold = 100 chars

        let big_body = "x".repeat(150); // 150 + len("TEST") > 100
        let attempt = ApprenticeshipDispatcher::new(&doorman, cfg)
            .dispatch_brief(&brief_for(&big_body))
            .await
            .expect("tier-B dispatch ok");

        assert_eq!(attempt.tier, Tier::Yoyo);
        // Tier A must NOT have received any request.
        let received_a = server_a.received_requests().await.unwrap_or_default();
        assert_eq!(
            received_a.len(),
            0,
            "Doorman MUST route the large brief to Tier B, not Tier A"
        );
    }

    /// Unit: parse_attempt_content recovers from missing frontmatter
    /// by escalating with empty diff.
    #[test]
    fn parse_unparsable_response_escalates() {
        let p = parse_attempt_content("totally unstructured apprentice output");
        assert!(p.escalate);
        assert_eq!(p.diff, "");
        assert_eq!(p.self_confidence, 0.0);
    }

    /// Unit: pick_tier_for_brief boundary.
    #[test]
    fn pick_tier_at_threshold() {
        let mut b = brief_for("");
        b.body = "x".repeat(50);
        b.acceptance_test = "y".repeat(50);
        // 50 + 50 = 100, NOT exceeding 100 → Tier A
        assert_eq!(pick_tier_for_brief(&b, 100), Tier::Local);
        b.body = "x".repeat(51);
        // 51 + 50 = 101, exceeds 100 → Tier B
        assert_eq!(pick_tier_for_brief(&b, 100), Tier::Yoyo);
    }
}
