// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ChassisError>;

#[derive(Debug, Error)]
pub enum ChassisError {
    #[error("unauthenticated — module-id not registered in fleet")]
    Unauthenticated,

    #[error("forbidden — X-Foundry-Module-ID header does not match bearer token module-id")]
    ModuleIdMismatch,

    #[error("payment required — Totebox '{0}' is not commercially subscribed for Tier B")]
    NotSubscribed(String),

    #[error("Yo-Yo upstream error: {0}")]
    YoyoUpstream(String),

    #[error("Yo-Yo endpoint not configured for label '{0}'")]
    YoyoNotConfigured(String),

    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("serialisation error: {0}")]
    Json(#[from] serde_json::Error),
}
