// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Health-check handler.

use axum::http::StatusCode;

/// Returns `200 OK` unconditionally.
///
/// Mounted at `GET /health`. Used by load balancers and orchestrators to
/// determine whether the service process is alive. A future readiness probe
/// will live at a separate endpoint and will consult subsystem state before
/// responding.
#[tracing::instrument]
pub async fn health() -> StatusCode {
    StatusCode::OK
}
