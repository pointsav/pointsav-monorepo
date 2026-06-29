// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("license error: {0}")]
    License(String),

    #[error("fleet error: {0}")]
    Fleet(String),

    #[error("invite error: {0}")]
    Invite(String),

    #[error("pairing error: {0}")]
    Pairing(String),

    #[error("routing error: {0}")]
    Routing(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}
