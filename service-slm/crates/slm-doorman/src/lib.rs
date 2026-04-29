// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Doorman — the single secure boundary between the isolated Totebox
//! Archive and any external Large Language Model.
//!
//! Phase-1 scope (B1): three-tier router skeleton plus append-only audit
//! ledger. Concrete wire calls for Tier B (Yo-Yo) land in B2; for Tier C
//! (External API) in B4. Tier A (local mistral.rs / llama-server) is the
//! reference path and works end-to-end against a local OpenAI-compatible
//! HTTP endpoint.
//!
//! Architectural background: `service-slm/ARCHITECTURE.md` and
//! `~/Foundry/conventions/three-ring-architecture.md` (Ring 3 — Optional
//! Intelligence). Audit ledger discipline is doctrinal — see Doctrine §V.

pub mod apprenticeship;
pub mod audit_proxy;
pub mod brief_cache;
pub mod citations;
pub mod error;
pub mod grammar_validation;
pub mod ledger;
pub mod promotion_ledger;
pub mod redact;
pub mod router;
pub mod tier;
pub mod verdict;

pub use apprenticeship::{
    apprentice_prompt, build_attempt, parse_attempt_content, pick_tier_for_brief,
    shadow_corpus_path, ApprenticeshipConfig, ApprenticeshipDispatcher, ParsedAttempt,
    ShadowOutcome, APPRENTICE_SYSTEM_PROMPT,
};
pub use audit_proxy::{
    AuditProxyClient, AuditProxyConfig, AuditProxyPurposeAllowlist,
    FOUNDRY_DEFAULT_PURPOSE_ALLOWLIST,
};
pub use brief_cache::{BriefCache, CachedBrief};
pub use error::{DoormanError, Result};
pub use grammar_validation::LarkValidator;
pub use ledger::{
    AuditCaptureEntry, AuditEntry, AuditLedger, AuditProxyEntry, AuditProxyStubEntry,
};
pub use promotion_ledger::{PromotionLedger, PromotionOutcome, Stage, StatRow};
pub use redact::sanitize;
pub use router::{Doorman, DoormanConfig};
pub use tier::{
    BearerTokenProvider, ExternalAllowlist, ExternalTierClient, ExternalTierConfig,
    LocalTierClient, PricingConfig, StaticBearer, TierCPricing, TierCProvider, YoYoTierClient,
    FOUNDRY_DEFAULT_ALLOWLIST,
};
pub use verdict::{
    SshKeygenVerifier, VerdictDispatchOutcome, VerdictDispatcher, VerdictVerifier, VerdictWireBody,
};

/// Wire version of the Yo-Yo HTTP API contract this Doorman speaks
/// (`infrastructure/slm-yoyo/CONTRACT.md`). Sent in
/// `X-Foundry-Contract-Version` on every Yo-Yo request.
pub const YOYO_CONTRACT_VERSION: &str = "0.0.1";

/// Doorman's own version, surfaced in `/v1/contract` responses to callers
/// and in the audit ledger.
pub const DOORMAN_VERSION: &str = env!("CARGO_PKG_VERSION");
