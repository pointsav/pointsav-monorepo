// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

pub mod annotation_log;
pub mod dav_fs;
pub mod event;
pub mod event_store;
pub mod ledger_backing;
pub mod mirror_index;
pub mod mirror_rebuild;
pub mod mirror_view;

pub use annotation_log::AnnotationLog;
pub use dav_fs::WormCalendarFs;
pub use event::CalendarEvent;
pub use event_store::EventStore;
pub use ledger_backing::LedgerBacking;
pub use mirror_index::MirrorIndex;
pub use mirror_rebuild::{rebuild_from_mirror, MailboxRoot};
pub use mirror_view::MirrorView;

use axum::extract::{Request, State};
use axum::response::IntoResponse;
use axum::routing::{any, get};
use axum::Router;
use dav_server::fakels::FakeLs;
use dav_server::DavHandler;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub dav: Arc<DavHandler>,
}

/// The CalDAV router: `GET /healthz` (liveness) plus the full
/// WebDAV/CalDAV method set (PROPFIND, REPORT, PUT, GET, DELETE,
/// MKCALENDAR, ...) mounted at `/` and every path beneath it, handled
/// entirely by `dav_server::DavHandler` against a `WormCalendarFs`.
pub fn router(ledger_backing: Arc<LedgerBacking>, event_store: Arc<EventStore>) -> Router {
    router_for(WormCalendarFs::new(ledger_backing, event_store))
}

/// As [`router`], also serving the mirrored events in `mirror`
/// (read-only) beside the native ones.
pub fn router_with_mirror(
    ledger_backing: Arc<LedgerBacking>,
    event_store: Arc<EventStore>,
    mirror: Arc<MirrorView>,
) -> Router {
    router_for(WormCalendarFs::with_mirror(
        ledger_backing,
        event_store,
        mirror,
    ))
}

fn router_for(fs: Box<WormCalendarFs>) -> Router {
    let dav = DavHandler::builder()
        .filesystem(fs)
        .locksystem(FakeLs::new())
        .build_handler();

    let state = AppState { dav: Arc::new(dav) };

    Router::new()
        .route("/healthz", get(healthz))
        .route("/", any(handle_dav))
        // axum 0.7 uses the older `/*path` catch-all syntax (no curly
        // braces) -- dav-server's own example targets axum 0.8's
        // `/{*path}` syntax, which this crate's axum 0.7 (matching
        // every other service in this workspace) rejects at router-
        // build time with "catch-all parameters are only allowed at
        // the end of a route" -- a version-syntax mismatch, not a
        // real routing conflict.
        .route("/*path", any(handle_dav))
        .with_state(state)
}

async fn healthz() -> impl IntoResponse {
    (axum::http::StatusCode::OK, "ok")
}

async fn handle_dav(State(state): State<AppState>, req: Request) -> impl IntoResponse {
    state.dav.handle(req).await
}
