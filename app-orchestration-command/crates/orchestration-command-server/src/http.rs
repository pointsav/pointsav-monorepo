// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Axum HTTP router and handlers for orchestration-command-server.

use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;

use orchestration_command::invite::InviteIssuer;
use orchestration_command::pairing::PairingStore;
use orchestration_command::routing::MessageRouter;
use orchestration_command::LicenseStatus;
use orchestration_command_core::{
    ArchiveEntry, ArchivesResponse, AuditEntry, AuditRollupResponse, ChildStatus,
    InviteRequest, InviteResponse, LicenseLabel, MessageEnvelope, PairRequest, PersonnelEntry,
    ReadyzResponse,
};

pub struct AppState {
    pub archives: Arc<Vec<ArchiveEntry>>,
    pub personnel: Arc<HashMap<String, PersonnelEntry>>,
    pub inviter: Arc<InviteIssuer>,
    pub pairing_store: Arc<PairingStore>,
    pub router: Arc<MessageRouter>,
    pub license: Arc<LicenseStatus>,
    pub child_running: Arc<std::sync::atomic::AtomicBool>,
    pub child_configured: bool,
    pub archives_loaded: usize,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/v1/archives", get(archives))
        .route("/v1/personnel/{user}", get(personnel))
        .route("/v1/invite", post(invite))
        .route("/v1/pair", post(pair))
        .route("/v1/message", post(message))
        .route("/v1/audit/rollup", get(audit_rollup))
        .with_state(state)
}

async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn readyz(State(s): State<Arc<AppState>>) -> impl IntoResponse {
    let license = match s.license.as_ref() {
        LicenseStatus::Valid(_) => LicenseLabel::Valid,
        LicenseStatus::Absent => LicenseLabel::Absent,
        LicenseStatus::Invalid(_) => LicenseLabel::Invalid,
    };
    let slm_child = if !s.child_configured {
        ChildStatus::NotConfigured
    } else if s.child_running.load(Ordering::SeqCst) {
        ChildStatus::Running
    } else {
        ChildStatus::Stopped
    };
    let body = ReadyzResponse {
        version: orchestration_command_core::COMMAND_VERSION,
        license,
        slm_child,
        archives_loaded: s.archives_loaded,
    };
    (StatusCode::OK, Json(body))
}

async fn archives(State(s): State<Arc<AppState>>) -> impl IntoResponse {
    let total = s.archives.len();
    let body = ArchivesResponse {
        archives: s.archives.as_ref().clone(),
        total,
    };
    Json(body)
}

async fn personnel(
    Path(user): Path<String>,
    State(s): State<Arc<AppState>>,
) -> impl IntoResponse {
    match s.personnel.get(&user) {
        Some(entry) => (StatusCode::OK, Json(entry.clone())).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "user not found" })),
        )
            .into_response(),
    }
}

async fn invite(
    State(s): State<Arc<AppState>>,
    Json(req): Json<InviteRequest>,
) -> impl IntoResponse {
    if !s.license.permits_pairing() {
        return (
            StatusCode::PAYMENT_REQUIRED,
            Json(serde_json::json!({
                "error": "valid license required for invite endpoint"
            })),
        )
            .into_response();
    }
    match s.inviter.issue(req.role.clone(), req.archive_scope, 24) {
        Ok(token) => {
            let expires_at = Utc::now() + chrono::Duration::hours(24);
            (
                StatusCode::OK,
                Json(InviteResponse {
                    token,
                    expires_at,
                    role: req.role,
                }),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn pair(
    State(s): State<Arc<AppState>>,
    Json(req): Json<PairRequest>,
) -> impl IntoResponse {
    if !s.license.permits_pairing() {
        return (
            StatusCode::PAYMENT_REQUIRED,
            Json(serde_json::json!({
                "error": "valid license required for pairing endpoint"
            })),
        )
            .into_response();
    }
    let payload = match s.inviter.verify_and_consume(&req.token) {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };
    match s.pairing_store.record(
        req.public_key,
        payload.role,
        payload.archive_scope,
        req.node_label,
    ) {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn message(
    State(s): State<Arc<AppState>>,
    Json(envelope): Json<MessageEnvelope>,
) -> impl IntoResponse {
    match s.router.route(&envelope) {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn audit_rollup(State(s): State<Arc<AppState>>) -> impl IntoResponse {
    let entries: Vec<AuditEntry> = s
        .archives
        .iter()
        .map(|a| AuditEntry {
            module_id: a.module_id.clone(),
            messages_routed: s.router.messages_routed(),
            invites_issued: 0,
            pairings_created: s.pairing_store.pairings_created(),
        })
        .collect();
    Json(AuditRollupResponse {
        entries,
        as_of: Utc::now(),
    })
}
