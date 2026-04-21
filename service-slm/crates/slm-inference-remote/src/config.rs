// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Configuration for the remote inference client.
//!
//! Holds the Cloud Run node's base URL, the `ModuleId` threaded through every
//! ledger row, and the per-request HTTP timeout. Validation runs once at
//! construction time so the client surface cannot be handed a malformed URL.

use std::time::Duration;

use reqwest::Url;
use slm_core::ModuleId;
use thiserror::Error;

/// Default per-request HTTP timeout.
///
/// Conservative starting point for the Cloud Run `/healthz` boot probe. The
/// real inference endpoints will likely want a longer ceiling; callers can
/// override via [`RemoteInferenceConfig::with_request_timeout`].
const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Validated configuration for a [`RemoteInferenceClient`](crate::RemoteInferenceClient).
///
/// Construct via [`RemoteInferenceConfig::new`]; all fields are validated at
/// that point and the resulting value is immutable except through the
/// fluent `with_*` builders.
#[derive(Debug, Clone)]
pub struct RemoteInferenceConfig {
    base_url: Url,
    module_id: ModuleId,
    request_timeout: Duration,
}

impl RemoteInferenceConfig {
    /// Constructs a config from a base URL string and a `ModuleId`.
    ///
    /// The base URL must parse and use `http` or `https`; other schemes are
    /// rejected so the client cannot be pointed at `file://` or `data:` URLs
    /// that would bypass the HTTP stack's TLS and timeout machinery.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::InvalidUrl`] if `base_url` does not parse and
    /// [`ConfigError::UnsupportedScheme`] if the scheme is not `http` or
    /// `https`.
    pub fn new(base_url: &str, module_id: ModuleId) -> Result<Self, ConfigError> {
        let base_url = Url::parse(base_url).map_err(|source| ConfigError::InvalidUrl {
            input: base_url.to_owned(),
            reason: source.to_string(),
        })?;
        match base_url.scheme() {
            "http" | "https" => {}
            other => {
                return Err(ConfigError::UnsupportedScheme {
                    scheme: other.to_owned(),
                });
            }
        }
        Ok(Self {
            base_url,
            module_id,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
        })
    }

    /// Overrides the per-request HTTP timeout.
    #[must_use]
    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Base URL of the remote Cloud Run node.
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// `ModuleId` stamped on every ledger row emitted by the client.
    #[must_use]
    pub fn module_id(&self) -> &ModuleId {
        &self.module_id
    }

    /// Per-request HTTP timeout.
    #[must_use]
    pub fn request_timeout(&self) -> Duration {
        self.request_timeout
    }
}

/// Errors produced by [`RemoteInferenceConfig::new`].
#[derive(Debug, Error)]
pub enum ConfigError {
    /// The supplied base URL did not parse.
    #[error("invalid base URL `{input}`: {reason}")]
    InvalidUrl {
        /// The string that failed to parse.
        input: String,
        /// Human-readable reason the URL was rejected (from the `url` crate).
        reason: String,
    },

    /// The supplied base URL parsed but used an unsupported scheme.
    ///
    /// Only `http` and `https` are accepted.
    #[error("unsupported URL scheme `{scheme}`: expected http or https")]
    UnsupportedScheme {
        /// The scheme that was rejected.
        scheme: String,
    },
}
