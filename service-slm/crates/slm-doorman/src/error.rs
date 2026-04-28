// SPDX-License-Identifier: Apache-2.0 OR MIT

use slm_core::Tier;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DoormanError>;

#[derive(Debug, Error)]
pub enum DoormanError {
    #[error("requested tier {0:?} is not configured")]
    TierUnavailable(Tier),

    #[error("tier {tier:?} is not yet implemented (filled in {filled_in_by})")]
    NotImplemented {
        tier: Tier,
        filled_in_by: &'static str,
    },

    #[error(
        "tier-C task label {label:?} is not on the allowlist; \
         see DoormanConfig::external_allowlist"
    )]
    ExternalNotAllowlisted { label: String },

    #[error("upstream HTTP error: {0}")]
    Upstream(#[from] reqwest::Error),

    #[error("audit ledger I/O error: {0}")]
    LedgerIo(#[from] std::io::Error),

    #[error("audit ledger serialisation error: {0}")]
    LedgerSerde(#[from] serde_json::Error),

    #[error("home directory not resolvable; HOME env var is unset")]
    HomeUnset,

    #[error("upstream response could not be parsed as OpenAI-compatible: {0}")]
    UpstreamShape(String),

    #[error(
        "Yo-Yo contract MAJOR-version mismatch: remote returned {remote_status} \
         (Doorman speaks contract {doorman_version}); refusing to retry"
    )]
    ContractMajorMismatch {
        remote_status: u16,
        doorman_version: &'static str,
    },

    #[error("Yo-Yo bearer-token provider returned no token: {0}")]
    BearerToken(String),

    #[error("verdict signature verification failed: {0}")]
    VerifySignature(String),

    #[error("apprenticeship ledger lock or write failed: {0}")]
    LedgerLock(String),

    #[error("apprenticeship corpus write failed at {path}: {reason}")]
    CorpusWrite { path: String, reason: String },

    #[error("verdict body could not be parsed: {0}")]
    VerdictParse(String),

    #[error(
        "verdict POST referenced an unknown (brief_id, attempt_id) — \
             brief cache miss; senior must reissue the brief"
    )]
    BriefCacheMiss,

    #[error(
        "Tier A (llama-server) does not accept {dialect} grammars; \
         {advice}"
    )]
    TierAGrammarUnsupported {
        /// The grammar dialect that was rejected, e.g. `"Lark"`.
        dialect: &'static str,
        /// Human-readable advice for the caller.
        advice: &'static str,
    },

    #[error(
        "Tier C (external API) does not accept {dialect} grammars; \
         {advice}"
    )]
    TierCGrammarUnsupported {
        /// The grammar dialect that was rejected, e.g. `"Lark"`, `"GBNF"`,
        /// or `"JsonSchema"`.
        dialect: &'static str,
        /// Human-readable advice for the caller.
        advice: &'static str,
    },
}
