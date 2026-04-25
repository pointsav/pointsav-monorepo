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
}
