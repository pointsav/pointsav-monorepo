// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Doorman — three-tier router skeleton.
//!
//! Routes one `ComputeRequest` to a configured tier and writes one audit
//! entry per call. Per the Optional Intelligence principle
//! (`conventions/three-ring-architecture.md`) every tier is optional;
//! the Doorman boots and serves /healthz with no tiers configured at
//! all. A request to an unconfigured tier returns
//! `DoormanError::TierUnavailable`, which a caller (Ring 2 service or
//! the inbound HTTP server) MAY translate to a "fall back to
//! deterministic processing" decision.

use chrono::Utc;
use slm_core::{CanonicalMessage, Complexity, ComputeRequest, ComputeResponse, GrammarConstraint, Role, Tier};
use tracing::{info, warn};

use crate::error::{DoormanError, Result};
use crate::grammar_validation::LarkValidator;
use crate::graph::GraphContextClient;
use crate::ledger::{
    AuditEntry, AuditLedger, CompletionStatus, ExtractionAuditEntry, ENTRY_TYPE_CHAT_COMPLETION,
    ENTRY_TYPE_EXTRACT,
};
use crate::mesh::MeshRegistry;
use crate::tier::{ExternalTierClient, LocalTierClient, YoYoTierClient};

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct DoormanConfig {
    pub local: Option<LocalTierClient>,
    pub yoyo: HashMap<String, YoYoTierClient>,
    pub external: Option<ExternalTierClient>,
    /// Optional Lark grammar pre-validator (PS.3 step 5). When `Some`,
    /// the router validates any Lark grammar before dispatching to Tier B.
    /// `None` disables the validation step (opt-out for callers that do not
    /// have the llguidance init overhead or for testing). Set via
    /// `LarkValidator::new()` at Doorman startup.
    pub lark_validator: Option<LarkValidator>,
    /// Optional graph context client (Brief E). When `Some`, the router
    /// queries `service-content`'s `/v1/graph/context` endpoint before
    /// dispatching to any tier, injecting matching entity rows as a
    /// `[ENTITY CONTEXT]` system message prepended to the message list.
    /// Non-fatal: if the client is absent or the query returns no results,
    /// the request proceeds without context injection.
    /// Set via `SERVICE_CONTENT_ENDPOINT` env var at Doorman startup.
    pub graph_context_client: Option<GraphContextClient>,
}

pub struct Orchestrator {
    pub registry: Box<dyn MeshRegistry>,
}

pub struct Doorman {
    local: Option<LocalTierClient>,
    yoyo: HashMap<String, YoYoTierClient>,
    external: Option<ExternalTierClient>,
    ledger: AuditLedger,
    lark_validator: Option<LarkValidator>,
    graph_context_client: Option<GraphContextClient>,
    pub orchestrator: Option<Orchestrator>,
}

impl Doorman {
    pub fn new(config: DoormanConfig, ledger: AuditLedger) -> Self {
        Self {
            local: config.local,
            yoyo: config.yoyo,
            external: config.external,
            ledger,
            lark_validator: config.lark_validator,
            graph_context_client: config.graph_context_client,
            orchestrator: None,
        }
    }

    pub async fn route_async(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        if let Some(ref orch) = self.orchestrator {
            info!(target: "slm_doorman::router", request_id = %req.request_id, "dispatching via orchestrator");
            
            if let Some(node) = orch.registry.select_optimal(req).await {
                info!(target: "slm_doorman::router", node_id = %node.id.0, "selected node");
                // Here we would dispatch to node.endpoint
                // For Phase 1, we continue to delegate to existing dispatch path 
                // but node-aware dispatch logic goes here in future steps.
            }
        }
        self.route(req).await
    }

    pub fn has_local(&self) -> bool {
        self.local.is_some()
    }

    pub fn has_yoyo(&self) -> bool {
        !self.yoyo.is_empty()
    }

    pub fn has_external(&self) -> bool {
        self.external.is_some()
    }

    pub fn has_lark_validator(&self) -> bool {
        self.lark_validator.is_some()
    }

    /// Returns the circuit breaker state string of the default (first)
    /// Yo-Yo client, or `"unconfigured"` when no Yo-Yo clients exist.
    pub fn default_yoyo_circuit_state(&self) -> &'static str {
        self.yoyo
            .values()
            .next()
            .map(|c| c.circuit_state())
            .unwrap_or("unconfigured")
    }

    pub fn ledger(&self) -> &AuditLedger {
        &self.ledger
    }

    /// Direct access to the graph context client — needed by callers that
    /// want to capture graph_context separately from the prompt injection
    /// path. Phase 2 (P2-2.5) of learning-loop-master-plan-2026-05-18.md:
    /// apprenticeship.rs::dispatch_shadow embeds the queried context in
    /// the JSONL tuple so LoRA training learns citation-grounded prose
    /// (Doctrine claim #44 — co-evolution loop).
    ///
    /// Returns `None` when no client is configured (service-content not
    /// wired). Callers MUST handle the None case as "no graph context
    /// available" rather than failing.
    pub fn graph_context_client(&self) -> Option<&GraphContextClient> {
        self.graph_context_client.as_ref()
    }

    /// Pick a tier from the request and dispatch. The caller's `tier_hint`
    /// is honoured when the named tier is configured; otherwise the
    /// router maps `complexity` to a default tier and probes for the
    /// best configured option.
    ///
    /// When a `graph_context_client` is configured, the router queries
    /// `service-content` for entity context matching the last user message
    /// (first 200 chars) before dispatching. Matching entities are prepended
    /// as a `[ENTITY CONTEXT]` system message. The query is non-fatal: if
    /// service-content is unavailable or returns no results, the request
    /// proceeds without modification.
    pub async fn route(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        let target = self.select_tier(req)?;

        // Graph context injection (Brief E).
        //
        // When service-content is configured and the request contains a user
        // message, fetch matching entity rows and prepend them as a system
        // message. The owned `effective_req` must be declared before the
        // conditional block so its lifetime covers the dispatch call below.
        let effective_req: ComputeRequest;
        let req_ref: &ComputeRequest = if req.graph_context_enabled == Some(false) {
            req
        } else if let Some(ref gc) = self.graph_context_client {
            let query = req
                .messages
                .iter()
                .rev()
                .find(|m| m.role == Role::User)
                .map(|m| m.text_content().chars().take(200).collect::<String>())
                .unwrap_or_default();

            if !query.is_empty() {
                if let Some(ctx) = gc
                    .fetch_context(req.module_id.as_str(), &query, 5)
                    .await
                {
                    let mut cloned = req.clone();
                    cloned.messages.insert(
                        0,
                        CanonicalMessage::text("system", format!("[ENTITY CONTEXT]\n{}", ctx)),
                    );
                    effective_req = cloned;
                    &effective_req
                } else {
                    req
                }
            } else {
                req
            }
        } else {
            req
        };

        let result = self.dispatch(target, req_ref).await;
        self.write_audit(req, target, &result);
        result
    }

    fn select_tier(&self, req: &ComputeRequest) -> Result<Tier> {
        if let Some(hint) = req.tier_hint {
            return self.confirm_tier_with_req(hint, req);
        }
        // Default policy: low / medium → local, high → yoyo if configured
        // else local. Tier C is never a default — callers must hint it
        // explicitly and the label-allowlist check runs in `dispatch`.
        let preferred = match req.complexity {
            Complexity::Low | Complexity::Medium => Tier::Local,
            Complexity::High => {
                if !self.yoyo.is_empty() {
                    Tier::Yoyo
                } else {
                    Tier::Local
                }
            }
        };
        self.confirm_tier_with_req(preferred, req)
    }

    fn confirm_tier_with_req(&self, tier: Tier, req: &ComputeRequest) -> Result<Tier> {
        let configured = match tier {
            Tier::Local => self.local.is_some(),
            Tier::Yoyo => {
                if let Some(ref label) = req.yoyo_label {
                    self.yoyo.contains_key(label)
                } else {
                    !self.yoyo.is_empty()
                }
            }
            Tier::External => self.external.is_some(),
        };
        if configured {
            Ok(tier)
        } else {
            warn!(
                target: "slm_doorman::router",
                ?tier,
                yoyo_label = ?req.yoyo_label,
                "tier not configured — community-tier mode may be active"
            );
            Err(DoormanError::TierUnavailable(tier))
        }
    }

    async fn dispatch(&self, tier: Tier, req: &ComputeRequest) -> Result<ComputeResponse> {
        info!(
            target: "slm_doorman::router",
            request_id = %req.request_id,
            module_id = %req.module_id,
            tier = tier.as_str(),
            "dispatching"
        );

        // PS.3 step 5 — pre-validate Lark grammar before Tier B dispatch.
        //
        // Tier B (vLLM ≥0.12) accepts Lark grammars. A malformed Lark string
        // relayed to vLLM produces an opaque upstream 400/500 with no useful
        // error message. We validate at the boundary so callers get a typed
        // 400 MalformedLarkGrammar with parse-error location instead.
        //
        // Only runs when:
        //   (a) a LarkValidator is configured (lark_validator.is_some()), AND
        //   (b) the selected tier is Tier B (Yoyo), AND
        //   (c) the request carries GrammarConstraint::Lark.
        //
        // Tier A rejects Lark in its own grammar-policy check (steps 2-4).
        // Tier C rejects all grammars (steps 2-4). Neither path reaches here
        // with a Lark grammar under normal routing. The guard on Tier::Yoyo
        // is defensive; it makes the semantics explicit rather than relying on
        // routing invariants.
        if tier == Tier::Yoyo {
            if let (Some(validator), Some(GrammarConstraint::Lark(lark_src))) =
                (&self.lark_validator, &req.grammar)
            {
                if let Err(reason) = validator.validate(lark_src) {
                    return Err(DoormanError::MalformedLarkGrammar { reason });
                }
            }
        }

        match tier {
            Tier::Local => {
                self.local
                    .as_ref()
                    .ok_or(DoormanError::TierUnavailable(Tier::Local))?
                    .complete(req)
                    .await
            }
            Tier::Yoyo => {
                let client = if let Some(ref label) = req.yoyo_label {
                    self.yoyo.get(label).ok_or_else(|| {
                        warn!(target: "slm_doorman::router", label, "requested Yo-Yo label not configured");
                        DoormanError::TierUnavailable(Tier::Yoyo)
                    })?
                } else {
                    // Default to the first configured Yo-Yo if no label provided.
                    // If multiple are configured and no label is provided, the
                    // behavior is non-deterministic (HashMap order) but safe.
                    // Operator should always use labels for Multi-Yo-Yo.
                    self.yoyo.values().next().ok_or_else(|| {
                        DoormanError::TierUnavailable(Tier::Yoyo)
                    })?
                };

                // B4: fast-path health + circuit check before making any HTTP call.
                // B2: when SLM_YOYO_AUTO_START=true and the backend is down,
                // invoke start-yoyo.sh (path from SLM_YOYO_START_SCRIPT) and
                // wait up to 90 s for the health probe to recover before falling
                // back to Tier A.
                if !client.allow_request() {
                    let auto_start = std::env::var("SLM_YOYO_AUTO_START")
                        .map(|v| v == "true")
                        .unwrap_or(false);
                    let came_up = if auto_start {
                        info!(
                            target: "slm_doorman::router",
                            request_id = %req.request_id,
                            "Tier B down; SLM_YOYO_AUTO_START=true — triggering start-yoyo.sh"
                        );
                        try_auto_start_yoyo(client).await
                    } else {
                        false
                    };
                    if !came_up {
                        warn!(
                            target: "slm_doorman::router",
                            request_id = %req.request_id,
                            auto_start,
                            "Tier B unavailable; falling back to Tier A"
                        );
                        return self.try_local_fallback(req).await;
                    }
                    info!(
                        target: "slm_doorman::router",
                        request_id = %req.request_id,
                        "Yo-Yo health probe up after auto-start; proceeding with Tier B"
                    );
                }

                match client.complete(req).await {
                    Ok(resp) => Ok(resp),
                    Err(e) if is_transient_tier_b_failure(&e) => {
                        warn!(
                            target: "slm_doorman::router",
                            request_id = %req.request_id,
                            reason = %e,
                            "Tier B transient failure; falling back to Tier A"
                        );
                        self.try_local_fallback(req).await
                    }
                    Err(e) => Err(e),
                }
            }
            Tier::External => {
                self.external
                    .as_ref()
                    .ok_or(DoormanError::TierUnavailable(Tier::External))?
                    .complete(req)
                    .await
            }
        }
    }

    fn write_audit(&self, req: &ComputeRequest, tier: Tier, result: &Result<ComputeResponse>) {
        let entry = match result {
            Ok(resp) => AuditEntry {
                entry_type: ENTRY_TYPE_CHAT_COMPLETION.to_string(),
                timestamp_utc: Utc::now(),
                request_id: req.request_id,
                module_id: req.module_id.clone(),
                tier: resp.tier_used,
                model: resp.model.clone(),
                inference_ms: resp.inference_ms,
                cost_usd: resp.cost_usd,
                sanitised_outbound: req.sanitised_outbound,
                completion_status: CompletionStatus::Ok,
                error_message: None,
                adapter_version: resp.adapter_version.clone(),
            },
            Err(e) => AuditEntry {
                entry_type: ENTRY_TYPE_CHAT_COMPLETION.to_string(),
                timestamp_utc: Utc::now(),
                request_id: req.request_id,
                module_id: req.module_id.clone(),
                tier,
                model: req.model.clone().unwrap_or_default(),
                inference_ms: 0,
                cost_usd: 0.0,
                sanitised_outbound: req.sanitised_outbound,
                completion_status: classify_error(e),
                error_message: Some(e.to_string()),
                adapter_version: None,
            },
        };
        if let Err(write_err) = self.ledger.append(&entry) {
            // Audit failure must never silently drop. Surface to logs;
            // upstream observability picks it up.
            warn!(
                target: "slm_doorman::ledger",
                error = %write_err,
                request_id = %req.request_id,
                "failed to append audit entry"
            );
        }

        // Prometheus metric emit (P3-3.1). The recorder is installed by
        // slm-doorman-server::metrics::init() at startup; if absent, the
        // `metrics` facade no-ops. Labels: tier + model + adapter_version
        // + completion_status. adapter_version is "none" when no LoRA is
        // loaded so the label dimension stays bounded.
        let adapter_label = entry
            .adapter_version
            .clone()
            .unwrap_or_else(|| "none".to_string());
        let status_label = match entry.completion_status {
            CompletionStatus::Ok => "ok",
            CompletionStatus::UpstreamError => "upstream_error",
            CompletionStatus::PolicyDenied => "policy_denied",
            CompletionStatus::TierUnavailable => "tier_unavailable",
        };
        metrics::counter!(
            "slm_requests_total",
            "tier" => entry.tier.as_str().to_string(),
            "model" => entry.model.clone(),
            "adapter_version" => adapter_label.clone(),
            "completion_status" => status_label,
        )
        .increment(1);
        metrics::counter!(
            "slm_cost_usd_total",
            "tier" => entry.tier.as_str().to_string(),
            "model" => entry.model.clone(),
        )
        .increment(entry.cost_usd as u64);
        metrics::histogram!(
            "slm_latency_ms",
            "tier" => entry.tier.as_str().to_string(),
            "model" => entry.model.clone(),
        )
        .record(entry.inference_ms as f64);
        metrics::counter!(
            "slm_audit_writes_total",
            "entry_type" => "chat-completion",
        )
        .increment(1);
    }
}

impl Doorman {
    /// Route a request to a specific named Yo-Yo backend without Tier A fallback.
    ///
    /// Unlike `route()`, this method does NOT fall back to Tier A on
    /// circuit-open or transient failure. Returns
    /// `Err(DoormanError::TierUnavailable(Tier::Yoyo))` immediately so the
    /// caller can defer the request rather than routing to an inappropriate
    /// backend.
    ///
    /// Used by `POST /v1/extract`: entity extraction requires the "trainer"
    /// Yo-Yo node (OLMo 3 32B-Think). OLMo 7B (Tier A) cannot produce
    /// structured JSON arrays reliably and must never serve as a fallback for
    /// extraction.
    ///
    /// Writes one `ExtractionAuditEntry` (entry_type = "extract") per call so
    /// extraction traffic is fully traceable in the audit ledger.
    pub async fn route_yoyo_only(
        &self,
        req: &ComputeRequest,
        label: &str,
    ) -> Result<ComputeResponse> {
        let started = std::time::Instant::now();

        let client = self.yoyo.get(label).ok_or_else(|| {
            warn!(
                target: "slm_doorman::router",
                label,
                "route_yoyo_only: Yo-Yo label not configured"
            );
            DoormanError::TierUnavailable(Tier::Yoyo)
        })?;

        if !client.allow_request() {
            warn!(
                target: "slm_doorman::router",
                request_id = %req.request_id,
                label,
                "route_yoyo_only: circuit not allowing request (open or health-probe down)"
            );
            let latency_ms = started.elapsed().as_millis() as u64;
            self.append_extract_audit(
                req, label, latency_ms, None, true,
                Some("yoyo-circuit-open".to_string()), None,
            );
            return Err(DoormanError::TierUnavailable(Tier::Yoyo));
        }

        // PS.3: validate Lark grammar at boundary if validator configured.
        // Extraction uses JsonSchema; this branch is defensive only.
        if let (Some(validator), Some(GrammarConstraint::Lark(lark_src))) =
            (&self.lark_validator, &req.grammar)
        {
            if let Err(reason) = validator.validate(lark_src) {
                return Err(DoormanError::MalformedLarkGrammar { reason });
            }
        }

        let result = client.complete(req).await;
        let latency_ms = started.elapsed().as_millis() as u64;
        match &result {
            Ok(resp) => self.append_extract_audit(
                req, label, latency_ms, Some(resp), false, None, None,
            ),
            Err(e) => self.append_extract_audit(
                req, label, latency_ms, None, false, None, Some(e.to_string()),
            ),
        }
        result
    }

    #[allow(clippy::too_many_arguments)]
    fn append_extract_audit(
        &self,
        req: &ComputeRequest,
        label: &str,
        latency_ms: u64,
        resp: Option<&ComputeResponse>,
        deferred: bool,
        defer_reason: Option<String>,
        error_message: Option<String>,
    ) {
        let entry = ExtractionAuditEntry {
            entry_type: ENTRY_TYPE_EXTRACT.to_string(),
            timestamp_utc: Utc::now(),
            request_id: req.request_id,
            module_id: req.module_id.clone(),
            extraction_ok: resp.is_some() && !deferred,
            deferred,
            entities_count: 0,  // entity count available only in the HTTP handler
            tier_used: if deferred {
                "deferred".to_string()
            } else {
                format!("yoyo_{}", label)
            },
            latency_ms,
            model: resp.map(|r| r.model.clone()).unwrap_or_default(),
            cost_usd: resp.map(|r| r.cost_usd).unwrap_or(0.0),
            sanitised_outbound: req.sanitised_outbound,
            defer_reason,
            error_message,
            adapter_version: resp.and_then(|r| r.adapter_version.clone()),
        };
        if let Err(e) = self.ledger.append_extract_entry(&entry) {
            warn!(
                target: "slm_doorman::ledger",
                error = %e,
                request_id = %req.request_id,
                "route_yoyo_only: failed to append extraction audit entry"
            );
        }
    }

    /// Begin a streaming Tier-B request. Returns the raw vLLM HTTP response on
    /// success; the caller translates the SSE body to the target wire format.
    ///
    /// Returns `Err(TierUnavailable(Tier::Yoyo))` if no Yo-Yo clients are
    /// configured. Returns `Err(TierBCircuitOpen)` if the circuit is open.
    /// Does NOT fall back to Tier A — streaming callers handle fallback
    /// themselves.
    pub async fn yoyo_stream(&self, req: &ComputeRequest) -> Result<reqwest::Response> {
        let client = if let Some(ref label) = req.yoyo_label {
            self.yoyo.get(label.as_str()).ok_or_else(|| {
                warn!(
                    target: "slm_doorman::router",
                    label = label.as_str(),
                    "yoyo_stream: requested label not configured"
                );
                DoormanError::TierUnavailable(Tier::Yoyo)
            })?
        } else {
            self.yoyo
                .values()
                .next()
                .ok_or_else(|| DoormanError::TierUnavailable(Tier::Yoyo))?
        };
        client.start_stream(req).await
    }
}

impl Doorman {
    async fn try_local_fallback(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        match &self.local {
            Some(local) => {
                info!(
                    target: "slm_doorman::router",
                    request_id = %req.request_id,
                    "Tier A fallback active"
                );
                local.complete(req).await
            }
            None => Err(DoormanError::TierUnavailable(Tier::Local)),
        }
    }
}

fn is_transient_tier_b_failure(e: &DoormanError) -> bool {
    matches!(
        e,
        DoormanError::TierBTimeout
            | DoormanError::TierBCircuitOpen
            | DoormanError::Upstream(_)
            | DoormanError::UpstreamShape(_)
    )
}

fn classify_error(e: &DoormanError) -> CompletionStatus {
    match e {
        DoormanError::TierUnavailable(_) | DoormanError::NotImplemented { .. } => {
            CompletionStatus::TierUnavailable
        }
        DoormanError::ExternalNotAllowlisted { .. }
        | DoormanError::VerifySignature(_)
        // Caller submitted a grammar dialect unsupported on the selected tier.
        // Both Tier A (e.g. Lark) and Tier C (any grammar) violations are
        // classified as PolicyDenied — the request violated the per-tier
        // input policy.
        | DoormanError::TierAGrammarUnsupported { .. }
        | DoormanError::TierCGrammarUnsupported { .. }
        // Caller submitted a syntactically malformed Lark grammar. The Doorman
        // rejected it at the boundary (PS.3 step 5) before any upstream call.
        // Classified as PolicyDenied: the error is entirely on the caller's side.
        | DoormanError::MalformedLarkGrammar { .. }
        // Caller supplied an unrecognised provider string to audit_proxy (PS.4).
        // The request violated input policy — classified as PolicyDenied.
        | DoormanError::AuditProxyInvalidProvider { .. }
        // Caller supplied an un-allowlisted purpose to audit_proxy (PS.4 step 3).
        // The request violated input policy — classified as PolicyDenied.
        | DoormanError::AuditProxyPurposeNotAllowlisted { .. }
        // audit_capture caller-side policy violations (PS.4 step 4): unknown
        // event_type, oversized payload, or unparseable timestamp. All are
        // caller errors — classified as PolicyDenied.
        | DoormanError::AuditCaptureUnknownEventType { .. }
        | DoormanError::AuditCapturePayloadTooLarge { .. }
        | DoormanError::AuditCaptureInvalidTimestamp { .. }
        // audit_proxy oversized request body — caller must reduce the request
        // size. Classified as PolicyDenied (same classification as the
        // audit_capture payload cap variant above).
        | DoormanError::AuditProxyPayloadTooLarge { .. }
        // Per-tenant (moduleId) concurrency cap hit on the audit endpoints.
        // The caller may retry; classified as PolicyDenied because the
        // Doorman is enforcing a per-tenant resource policy.
        | DoormanError::AuditTenantConcurrencyExhausted { .. } => {
            CompletionStatus::PolicyDenied
        }
        // The audit_proxy targeted a provider that is not configured at startup
        // (PS.4 step 2). Server-side configuration gap — classified as UpstreamError
        // (not PolicyDenied; the caller did nothing wrong). HTTP 503.
        DoormanError::AuditProxyProviderUnavailable { .. } => CompletionStatus::UpstreamError,
        // Tier B resilience errors — the circuit or deadline fired. Classified
        // as UpstreamError so the audit ledger reflects upstream unavailability.
        // In the normal routing path these are caught by is_transient_tier_b_failure
        // and trigger Tier A fallback; they reach classify_error only when
        // local is also unavailable.
        DoormanError::TierBTimeout | DoormanError::TierBCircuitOpen => {
            CompletionStatus::UpstreamError
        }
        DoormanError::Upstream(_)
        | DoormanError::UpstreamShape(_)
        | DoormanError::ContractMajorMismatch { .. }
        | DoormanError::BearerToken(_) => CompletionStatus::UpstreamError,
        DoormanError::LedgerIo(_)
        | DoormanError::LedgerSerde(_)
        | DoormanError::HomeUnset
        | DoormanError::LedgerLock(_)
        | DoormanError::CorpusWrite { .. }
        | DoormanError::VerdictParse(_)
        | DoormanError::BriefCacheMiss => CompletionStatus::UpstreamError,
        // Orphan verdict: no shadow corpus tuple exists for this brief_id.
        // Per §7B, no corpus row is created. Classified as PolicyDenied
        // — the caller should verify the shadow brief was dispatched
        // before signing a verdict. HTTP 410 in the HTTP layer.
        DoormanError::OrphanVerdictNoCorpusTuple { .. } => CompletionStatus::PolicyDenied,
        // Brief Queue Substrate (apprenticeship-substrate.md §7C):
        //   QueueIo       — file-system I/O failure; UpstreamError (server-side)
        //   QueueLockFailed — transient lock contention; UpstreamError (server-side)
        //   QueueMalformedBrief — caller-side malformed content; PolicyDenied
        DoormanError::QueueIo { .. } | DoormanError::QueueLockFailed { .. } => {
            CompletionStatus::UpstreamError
        }
        DoormanError::QueueMalformedBrief { .. } => CompletionStatus::PolicyDenied,
        // Graph proxy errors: caller-side for missing header (PolicyDenied),
        // server-side for unreachable service-content (UpstreamError).
        DoormanError::GraphProxyMissingModuleId => CompletionStatus::PolicyDenied,
        DoormanError::GraphProxyServiceUnavailable => CompletionStatus::UpstreamError,
        DoormanError::QueueQualityGateRejected { .. } => CompletionStatus::PolicyDenied,
        DoormanError::CorpusGateRejected { .. } => CompletionStatus::PolicyDenied,
    }
}

/// On-demand Yo-Yo auto-start. Invokes the script at `SLM_YOYO_START_SCRIPT`
/// and polls `client.is_healthy()` every 5 s for up to 90 s.
/// Returns `true` if the backend becomes healthy within the budget.
/// Non-fatal: script spawn failures are logged and `false` is returned.
async fn try_auto_start_yoyo(client: &YoYoTierClient) -> bool {
    let script = match std::env::var("SLM_YOYO_START_SCRIPT") {
        Ok(s) => s,
        Err(_) => {
            warn!(
                target: "slm_doorman::router",
                "SLM_YOYO_AUTO_START=true but SLM_YOYO_START_SCRIPT not set — cannot auto-start"
            );
            return false;
        }
    };

    match tokio::process::Command::new(&script).spawn() {
        Ok(_child) => {
            info!(target: "slm_doorman::router", script = %script, "auto-start: start-yoyo.sh spawned");
        }
        Err(e) => {
            warn!(
                target: "slm_doorman::router",
                script = %script, error = %e,
                "auto-start: failed to spawn start-yoyo.sh"
            );
            return false;
        }
    }

    let deadline = Instant::now() + Duration::from_secs(90);
    while Instant::now() < deadline {
        tokio::time::sleep(Duration::from_secs(5)).await;
        if client.is_healthy() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use slm_core::{CanonicalMessage, ModuleId, RequestId};
    use std::str::FromStr;

    fn req(complexity: Complexity, hint: Option<Tier>, yoyo_label: Option<String>) -> ComputeRequest {
        ComputeRequest {
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("foundry").unwrap(),
            model: None,
            messages: vec![CanonicalMessage::text("user", "ping")],
            complexity,
            tier_hint: hint,
            stream: false,
            max_tokens: None,
            temperature: None,
            sanitised_outbound: true,
            tier_c_label: None,
            yoyo_label,
            grammar: None,
            speculation: None,
            graph_context_enabled: None,
            adapter_version: None,
        }
    }

    fn ledger() -> AuditLedger {
        let dir = std::env::temp_dir().join(format!(
            "slm-doorman-router-test-{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        AuditLedger::new(dir).unwrap()
    }

    #[tokio::test]
    async fn unconfigured_router_refuses_with_tier_unavailable() {
        let doorman = Doorman::new(DoormanConfig::default(), ledger());
        let result = doorman.route(&req(Complexity::Medium, None, None)).await;
        match result {
            Err(DoormanError::TierUnavailable(Tier::Local)) => {}
            other => panic!("expected TierUnavailable(Local), got {other:?}"),
        }
    }

    #[test]
    fn high_complexity_prefers_yoyo_when_configured() {
        // Pure selection logic — no network. We construct a Doorman with
        // a Yo-Yo config that points at a bogus endpoint; select_tier
        // does not hit the network.
        let yoyo = YoYoTierClient::new(
            crate::tier::YoYoTierConfig {
                endpoint: "http://invalid.example".into(),
                default_model: "Olmo-3-1125-32B-Think".into(),
                contract_version: crate::YOYO_CONTRACT_VERSION.into(),
                pricing: crate::tier::PricingConfig::default(),
            },
            std::sync::Arc::new(crate::tier::StaticBearer::new("unused-in-selection-test")),
        );
        let mut yoyo_map = HashMap::new();
        yoyo_map.insert("default".to_string(), yoyo);
        let doorman = Doorman::new(
            DoormanConfig {
                local: None,
                yoyo: yoyo_map,
                external: None,
                lark_validator: None,
                graph_context_client: None,
            },
            ledger(),
        );
        let picked = doorman
            .select_tier(&req(Complexity::High, None, None))
            .expect("should pick yoyo");
        assert_eq!(picked, Tier::Yoyo);
    }
}
