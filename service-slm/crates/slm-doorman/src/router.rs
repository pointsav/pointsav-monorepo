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
use slm_core::{Complexity, ComputeRequest, ComputeResponse, GrammarConstraint, Tier};
use tracing::{info, warn};

use crate::error::{DoormanError, Result};
use crate::grammar_validation::LarkValidator;
use crate::graph::GraphContextClient;
use crate::ledger::{AuditEntry, AuditLedger, CompletionStatus, ENTRY_TYPE_CHAT_COMPLETION};
use crate::mesh::MeshRegistry;
use crate::tier::{ExternalTierClient, LocalTierClient, YoYoTierClient};

#[derive(Default)]
pub struct DoormanConfig {
    pub local: Option<LocalTierClient>,
    pub yoyo: Option<YoYoTierClient>,
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
    yoyo: Option<YoYoTierClient>,
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
        self.yoyo.is_some()
    }

    pub fn has_external(&self) -> bool {
        self.external.is_some()
    }

    pub fn ledger(&self) -> &AuditLedger {
        &self.ledger
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
        let req_ref: &ComputeRequest = if let Some(ref gc) = self.graph_context_client {
            let query = req
                .messages
                .iter()
                .rev()
                .find(|m| m.role == "user")
                .map(|m| m.content.chars().take(200).collect::<String>())
                .unwrap_or_default();

            if !query.is_empty() {
                if let Some(ctx) = gc
                    .fetch_context(req.module_id.as_str(), &query, 5)
                    .await
                {
                    let mut cloned = req.clone();
                    cloned.messages.insert(
                        0,
                        slm_core::ChatMessage {
                            role: "system".to_string(),
                            content: format!("[ENTITY CONTEXT]\n{}", ctx),
                        },
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
            return self.confirm_tier(hint);
        }
        // Default policy: low / medium → local, high → yoyo if configured
        // else local. Tier C is never a default — callers must hint it
        // explicitly and the label-allowlist check runs in `dispatch`.
        let preferred = match req.complexity {
            Complexity::Low | Complexity::Medium => Tier::Local,
            Complexity::High => {
                if self.yoyo.is_some() {
                    Tier::Yoyo
                } else {
                    Tier::Local
                }
            }
        };
        self.confirm_tier(preferred)
    }

    fn confirm_tier(&self, tier: Tier) -> Result<Tier> {
        let configured = match tier {
            Tier::Local => self.local.is_some(),
            Tier::Yoyo => self.yoyo.is_some(),
            Tier::External => self.external.is_some(),
        };
        if configured {
            Ok(tier)
        } else {
            warn!(
                target: "slm_doorman::router",
                ?tier,
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
                self.yoyo
                    .as_ref()
                    .ok_or(DoormanError::TierUnavailable(Tier::Yoyo))?
                    .complete(req)
                    .await
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
                tier,
                model: resp.model.clone(),
                inference_ms: resp.inference_ms,
                cost_usd: resp.cost_usd,
                sanitised_outbound: req.sanitised_outbound,
                completion_status: CompletionStatus::Ok,
                error_message: None,
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
    }
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slm_core::{ChatMessage, ModuleId, RequestId};
    use std::str::FromStr;

    fn req(complexity: Complexity, hint: Option<Tier>) -> ComputeRequest {
        ComputeRequest {
            request_id: RequestId::new(),
            module_id: ModuleId::from_str("foundry").unwrap(),
            model: None,
            messages: vec![ChatMessage {
                role: "user".into(),
                content: "ping".into(),
            }],
            complexity,
            tier_hint: hint,
            stream: false,
            max_tokens: None,
            temperature: None,
            sanitised_outbound: true,
            tier_c_label: None,
            grammar: None,
            speculation: None,
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
        let result = doorman.route(&req(Complexity::Medium, None)).await;
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
        let doorman = Doorman::new(
            DoormanConfig {
                local: None,
                yoyo: Some(yoyo),
                external: None,
                lark_validator: None,
                graph_context_client: None,
            },
            ledger(),
        );
        let picked = doorman
            .select_tier(&req(Complexity::High, None))
            .expect("should pick yoyo");
        assert_eq!(picked, Tier::Yoyo);
    }
}
