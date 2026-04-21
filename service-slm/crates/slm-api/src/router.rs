// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Router builder for the service-slm HTTP API.
//!
//! Call [`router`] to obtain an [`axum::Router`] that can be bound to a
//! socket or exercised directly in tests via [`tower::ServiceExt::oneshot`].

use axum::routing::get;
use axum::Router;
use tower_http::trace::TraceLayer;

use crate::health::health;

/// Builds and returns the complete axum router for service-slm.
///
/// The router is wrapped with a [`TraceLayer`] that emits a structured
/// `tracing` span for every inbound request, satisfying the invariant that
/// every request carries a trace id.
///
/// ## Routes
///
/// | Method | Path | Handler |
/// |---|---|---|
/// | `GET` | `/health` | [`health`] |
///
/// Additional routes are added as the corresponding library crates reach
/// alpha status. The pattern is: route registered here, handler kept thin
/// in its own module, real work delegated to the library crate.
pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
}
