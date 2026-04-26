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
use slm_core::{Complexity, ComputeRequest, ComputeResponse, Tier};
use tracing::{info, warn};

use crate::error::{DoormanError, Result};
use crate::ledger::{AuditEntry, AuditLedger, CompletionStatus};
use crate::tier::{ExternalTierClient, LocalTierClient, YoYoTierClient};

#[derive(Default)]
pub struct DoormanConfig {
    pub local: Option<LocalTierClient>,
    pub yoyo: Option<YoYoTierClient>,
    pub external: Option<ExternalTierClient>,
}

pub struct Doorman {
    local: Option<LocalTierClient>,
    yoyo: Option<YoYoTierClient>,
    external: Option<ExternalTierClient>,
    ledger: AuditLedger,
}

impl Doorman {
    pub fn new(config: DoormanConfig, ledger: AuditLedger) -> Self {
        Self {
            local: config.local,
            yoyo: config.yoyo,
            external: config.external,
            ledger,
        }
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
    pub async fn route(&self, req: &ComputeRequest) -> Result<ComputeResponse> {
        let target = self.select_tier(req)?;
        let result = self.dispatch(target, req).await;
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
        DoormanError::ExternalNotAllowlisted { .. } => CompletionStatus::PolicyDenied,
        DoormanError::Upstream(_)
        | DoormanError::UpstreamShape(_)
        | DoormanError::ContractMajorMismatch { .. }
        | DoormanError::BearerToken(_) => CompletionStatus::UpstreamError,
        DoormanError::LedgerIo(_) | DoormanError::LedgerSerde(_) | DoormanError::HomeUnset => {
            CompletionStatus::UpstreamError
        }
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
            },
            std::sync::Arc::new(crate::tier::StaticBearer::new("unused-in-selection-test")),
        );
        let doorman = Doorman::new(
            DoormanConfig {
                local: None,
                yoyo: Some(yoyo),
                external: None,
            },
            ledger(),
        );
        let picked = doorman
            .select_tier(&req(Complexity::High, None))
            .expect("should pick yoyo");
        assert_eq!(picked, Tier::Yoyo);
    }
}
