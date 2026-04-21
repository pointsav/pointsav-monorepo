// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! HTTP client for the Cloud Run yo-yo inference node.
//!
//! This is the first real implementation in `slm-inference-remote`. It wires
//! up the `/healthz` boot probe and the two ledger rows that straddle it —
//! `BOOT_REQUEST` before the probe and `BOOT_COMPLETE` after. Later tasks
//! will extend the client with `JOB_*`, `TEARDOWN_*`, and `PREEMPTION`
//! events per YOYO-COMPUTE §5.

use reqwest::{Client, Url};
use serde::Deserialize;
use slm_ledger::{Event, EventType, LedgerWriter};
use tracing::{debug, warn};

use crate::config::RemoteInferenceConfig;
use crate::error::RemoteInferenceError;

/// Handle returned by a successful [`RemoteInferenceClient::boot`] call.
///
/// Carries the base URL of the node and the node identifier reported by
/// the `/healthz` probe. Later work units will accept this handle as the
/// first argument to `job_start`, `teardown`, etc., to prevent calls
/// against a node the caller has not booted.
#[derive(Debug, Clone)]
pub struct NodeHandle {
    base_url: Url,
    node_id: String,
}

impl NodeHandle {
    /// Base URL of the booted node.
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Identifier reported by the node's `/healthz` endpoint.
    #[must_use]
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}

/// HTTP client that drives a single remote yo-yo inference node.
///
/// One instance per node; the [`RemoteInferenceConfig`] it holds pins the
/// base URL and the `ModuleId` threaded through every ledger row.
pub struct RemoteInferenceClient {
    http: Client,
    cfg: RemoteInferenceConfig,
}

/// Wire-format payload returned by the node's `/healthz` endpoint.
///
/// Deliberately minimal. Additional diagnostic fields are allowed on the
/// wire but ignored here — the client only depends on `node_id`.
#[derive(Debug, Deserialize)]
struct HealthResponse {
    node_id: String,
}

impl RemoteInferenceClient {
    /// Builds a client from a validated [`RemoteInferenceConfig`].
    ///
    /// The underlying `reqwest::Client` is configured with the per-request
    /// timeout from `cfg`. Construction can fail if the `reqwest` builder
    /// cannot initialise (for example, if the system's native TLS store
    /// cannot be loaded), which surfaces as
    /// [`RemoteInferenceError::Http`].
    ///
    /// # Errors
    ///
    /// Returns [`RemoteInferenceError::Http`] if the HTTP client cannot be
    /// built.
    pub fn new(cfg: RemoteInferenceConfig) -> Result<Self, RemoteInferenceError> {
        let http = Client::builder().timeout(cfg.request_timeout()).build()?;
        Ok(Self { http, cfg })
    }

    /// Returns the configuration the client was built with.
    #[must_use]
    pub fn config(&self) -> &RemoteInferenceConfig {
        &self.cfg
    }

    /// Probes the remote node's `/healthz` endpoint and records both
    /// phase-transition rows to the ledger.
    ///
    /// A `BOOT_REQUEST` row is written before the probe. On success, a
    /// `BOOT_COMPLETE` row is written with `completion_status = SUCCESS`
    /// and the reported `node_id`. On failure, a `BOOT_COMPLETE` row is
    /// written with `completion_status = FAILED` and a stable error code
    /// from [`RemoteInferenceError::ledger_code`]; the original error is
    /// then returned to the caller.
    ///
    /// Writing the `BOOT_REQUEST` row itself is a hard failure: without
    /// that row the `BOOT_COMPLETE` on its own would be meaningless, so
    /// the method returns early and no HTTP call is made. This mirrors
    /// the YOYO-COMPUTE §5 discipline that phase transitions come in
    /// pairs.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteInferenceError::Ledger`] if appending to the
    /// ledger fails, [`RemoteInferenceError::Http`] if the HTTP transport
    /// fails, or [`RemoteInferenceError::RemoteStatus`] if the node
    /// responds with a non-success status code.
    pub async fn boot(
        &self,
        writer: &mut LedgerWriter,
    ) -> Result<NodeHandle, RemoteInferenceError> {
        let request_event = Event::new(self.cfg.module_id().clone(), EventType::BootRequest);
        writer.append(&request_event)?;
        debug!(
            module_id = %self.cfg.module_id(),
            base_url = %self.cfg.base_url(),
            "BOOT_REQUEST recorded; probing /healthz",
        );

        match self.send_healthcheck().await {
            Ok(health) => {
                let mut complete =
                    Event::new(self.cfg.module_id().clone(), EventType::BootComplete);
                complete.node_id = Some(health.node_id.clone());
                complete.completion_status = Some("SUCCESS".to_owned());
                writer.append(&complete)?;
                debug!(node_id = %health.node_id, "BOOT_COMPLETE (SUCCESS) recorded");
                Ok(NodeHandle {
                    base_url: self.cfg.base_url().clone(),
                    node_id: health.node_id,
                })
            }
            Err(err) => {
                let mut failed = Event::new(self.cfg.module_id().clone(), EventType::BootComplete);
                failed.completion_status = Some("FAILED".to_owned());
                failed.error_code = Some(err.ledger_code());
                // If recording the failure itself fails, the original error
                // is the one the caller cares about; the ledger failure is
                // logged but swallowed to preserve the root cause.
                if let Err(ledger_err) = writer.append(&failed) {
                    warn!(
                        ledger_error = %ledger_err,
                        original_error = %err,
                        "failed to record BOOT_COMPLETE (FAILED) row; returning original error",
                    );
                } else {
                    debug!(error_code = %err.ledger_code(), "BOOT_COMPLETE (FAILED) recorded");
                }
                Err(err)
            }
        }
    }

    /// Performs the `/healthz` GET against the configured base URL.
    ///
    /// Pulled out of `boot` so that future work can reuse it for a
    /// periodic keep-alive probe without duplicating the URL-joining and
    /// status-handling logic.
    async fn send_healthcheck(&self) -> Result<HealthResponse, RemoteInferenceError> {
        // `Url::join` resolves against the base URL; when the base has no
        // trailing slash the join replaces the last segment, so the
        // canonical form of a base URL for this client ends in `/`. The
        // config layer does not enforce that today; treat this as a
        // follow-up (see TASKS.md).
        let url = self.cfg.base_url().join("healthz").map_err(|source| {
            // Surface join failures as transport errors — the only way
            // this fires is if the base URL was constructed bypassing
            // the config validator.
            RemoteInferenceError::RemoteStatus {
                status: 0,
                body: format!("failed to build healthz URL: {source}"),
            }
        })?;

        let response = self.http.get(url).send().await?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(RemoteInferenceError::RemoteStatus {
                status: status.as_u16(),
                body,
            });
        }
        let health = response.json::<HealthResponse>().await?;
        Ok(health)
    }
}
