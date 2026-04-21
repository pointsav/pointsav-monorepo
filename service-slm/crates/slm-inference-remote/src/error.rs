// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Error type for the remote inference client.
//!
//! Every variant maps to a stable string code written into the
//! `error_code` column of the ledger row that records the failure. The
//! mapping is intentionally coarse: the ledger is a SOC3 audit artefact,
//! not a debugger, so fine-grained transport details stay in logs.

use thiserror::Error;

use crate::config::ConfigError;

/// Errors produced by [`RemoteInferenceClient`](crate::RemoteInferenceClient).
///
/// Use [`RemoteInferenceError::ledger_code`] to obtain the stable
/// `error_code` string that is written to the ledger alongside a
/// `FAILED` completion status.
#[derive(Debug, Error)]
pub enum RemoteInferenceError {
    /// The HTTP transport failed before a response was received (DNS
    /// failure, connect failure, TLS failure, timeout, etc.).
    #[error("remote inference HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// The remote node responded with a non-success HTTP status.
    #[error("remote inference returned HTTP {status}: {body}")]
    RemoteStatus {
        /// The HTTP status code returned by the remote node.
        status: u16,
        /// The response body, truncated by the caller if large.
        body: String,
    },

    /// Writing the phase-transition row to the ledger failed.
    #[error("remote inference ledger write failed: {0}")]
    Ledger(#[from] slm_ledger::LedgerError),

    /// The client was handed a malformed [`RemoteInferenceConfig`](crate::RemoteInferenceConfig).
    #[error("remote inference configuration error: {0}")]
    Config(#[from] ConfigError),
}

impl RemoteInferenceError {
    /// Returns the stable string written to the ledger row's `error_code`
    /// column when this error is recorded.
    ///
    /// Keep these codes short and table-like; external audit tooling may
    /// group rows by this value.
    #[must_use]
    pub fn ledger_code(&self) -> String {
        match self {
            Self::Http(_) => "HTTP_TRANSPORT".to_owned(),
            Self::RemoteStatus { status, .. } => format!("HTTP_{status}"),
            Self::Ledger(_) => "LEDGER_FAILURE".to_owned(),
            Self::Config(_) => "CONFIG_ERROR".to_owned(),
        }
    }
}
