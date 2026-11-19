// SPDX-License-Identifier: Apache-2.0

//! `/v1/jobs*` and `/v1/engines` routes. Merged into the main router only
//! when `SLM_JOBS_ENABLED=true`; every route requires a bearer credential and
//! takes module id, engine allow-list and priority from the credential alone
//! (`X-Foundry-Module-ID` / `X-Foundry-Priority` are ignored here).
// An axum `Response` is the error type of these helpers by design; boxing it
// would only add noise at every call site.
#![allow(clippy::result_large_err)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{bail, Context};
use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, Extension, Path, Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;
use slm_doorman::jobs::engines::{
    AdmissionGate, JobEngine, LlmLocalEngine, OcrOpenaiVisionEngine, SpanGlinerEngine,
};
use slm_doorman::jobs::types::{JobRecord, SubmitRequest};
use slm_doorman::jobs::{
    Credential, CredentialSet, EngineKind, JobId, JobStore, JobsConfig, JobsService, Registry,
    SubmitError,
};

use crate::http::AppState;

#[derive(Clone)]
struct Ctx {
    svc: Arc<JobsService>,
    creds: Arc<CredentialSet>,
}

fn err(status: StatusCode, msg: &str) -> Response {
    (status, Json(json!({"error": {"message": msg}}))).into_response()
}

fn submit_err(e: SubmitError) -> Response {
    match e {
        SubmitError::EngineNotAllowed => err(
            StatusCode::FORBIDDEN,
            "engine not allowed for this credential",
        ),
        SubmitError::BadRequest(m) => err(StatusCode::BAD_REQUEST, &m),
        SubmitError::InputNotFound => err(StatusCode::NOT_FOUND, "input not found"),
        SubmitError::TooLarge(m) => err(StatusCode::PAYLOAD_TOO_LARGE, &m),
        SubmitError::BufferFull => err(
            StatusCode::INSUFFICIENT_STORAGE,
            "job buffer is full; ack finished jobs",
        ),
        SubmitError::Storage(m) => {
            tracing::error!(error = %m, "job storage error");
            err(StatusCode::INTERNAL_SERVER_ERROR, "storage error")
        }
    }
}

async fn auth(State(ctx): State<Ctx>, mut req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    match token.and_then(|t| ctx.creds.authenticate(t)) {
        Some(c) => {
            req.extensions_mut().insert(c.clone());
            next.run(req).await
        }
        None => err(StatusCode::UNAUTHORIZED, "unauthorized"),
    }
}

/// Public view of a job: no local paths, no credential ids.
fn view(r: &JobRecord) -> serde_json::Value {
    json!({
        "job_id": r.job_id,
        "engine_id": r.engine_id,
        "kind": r.kind,
        "status": r.status,
        "defer_reason": r.defer_reason,
        "created_at": r.created_at,
        "finished_at": r.finished_at,
        "record": r.record,
        "buffer_deleted_at": r.buffer_deleted_at,
        "replay_of": r.replay_of,
        "replay_match": r.replay_match,
    })
}

fn parse_id(s: &str) -> Result<JobId, Response> {
    JobId::from_str(s).map_err(|_| err(StatusCode::BAD_REQUEST, "invalid job id"))
}

async fn put_input(
    State(ctx): State<Ctx>,
    Extension(cred): Extension<Credential>,
    headers: axum::http::HeaderMap,
    body: Bytes,
) -> Response {
    let media = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    match ctx.svc.stage_input(&cred, media, &body) {
        Ok((r, sha)) => (
            StatusCode::CREATED,
            Json(json!({"staged_ref": r, "sha256": sha, "size": body.len()})),
        )
            .into_response(),
        Err(e) => submit_err(e),
    }
}

async fn submit(
    State(ctx): State<Ctx>,
    Extension(cred): Extension<Credential>,
    body: Bytes,
) -> Response {
    let req: SubmitRequest = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => return err(StatusCode::BAD_REQUEST, &format!("invalid request: {e}")),
    };
    match ctx.svc.submit(&cred, req) {
        Ok(id) => (StatusCode::ACCEPTED, Json(json!({"job_id": id}))).into_response(),
        Err(e) => submit_err(e),
    }
}

async fn get_job(
    State(ctx): State<Ctx>,
    Extension(cred): Extension<Credential>,
    Path(id): Path<String>,
) -> Response {
    let id = match parse_id(&id) {
        Ok(i) => i,
        Err(r) => return r,
    };
    match ctx.svc.status(&cred, id) {
        Some(rec) => Json(view(&rec)).into_response(),
        None => err(StatusCode::NOT_FOUND, "job not found"),
    }
}

async fn get_result(
    State(ctx): State<Ctx>,
    Extension(cred): Extension<Credential>,
    Path(id): Path<String>,
) -> Response {
    let id = match parse_id(&id) {
        Ok(i) => i,
        Err(r) => return r,
    };
    let Some(rec) = ctx.svc.status(&cred, id) else {
        return err(StatusCode::NOT_FOUND, "job not found");
    };
    if !rec.status.is_terminal() {
        return err(StatusCode::CONFLICT, "job not finished");
    }
    match ctx.svc.result(&cred, id) {
        Some(res) => Json(json!({
            "job": view(&rec),
            "text": res.text,
            "entities": res.entities,
            "rejected": res.rejected,
        }))
        .into_response(),
        None => err(
            StatusCode::NOT_FOUND,
            "no result (none produced, or already acked)",
        ),
    }
}

async fn ack(
    State(ctx): State<Ctx>,
    Extension(cred): Extension<Credential>,
    Path(id): Path<String>,
) -> Response {
    let id = match parse_id(&id) {
        Ok(i) => i,
        Err(r) => return r,
    };
    match ctx.svc.ack(&cred, id) {
        None => err(StatusCode::NOT_FOUND, "job not found"),
        Some(false) => err(StatusCode::CONFLICT, "job not finished"),
        Some(true) => Json(json!({"job_id": id, "buffer_deleted": true})).into_response(),
    }
}

#[derive(serde::Deserialize)]
struct CompareQuery {
    a: String,
    b: String,
}

/// `GET /v1/jobs/compare?a=<job>&b=<job>`: agreement between two engines'
/// validated entities for the human review gate.
async fn compare(
    State(ctx): State<Ctx>,
    Extension(cred): Extension<Credential>,
    axum::extract::Query(q): axum::extract::Query<CompareQuery>,
) -> Response {
    let (a, b) = match (parse_id(&q.a), parse_id(&q.b)) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(r), _) | (_, Err(r)) => return r,
    };
    match ctx.svc.compare(&cred, a, b) {
        Some(c) => Json(c).into_response(),
        None => err(
            StatusCode::NOT_FOUND,
            "both jobs need a finished entity result",
        ),
    }
}

async fn replay(
    State(ctx): State<Ctx>,
    Extension(cred): Extension<Credential>,
    Path(id): Path<String>,
) -> Response {
    let id = match parse_id(&id) {
        Ok(i) => i,
        Err(r) => return r,
    };
    match ctx.svc.replay(&cred, id) {
        Ok(new_id) => (
            StatusCode::ACCEPTED,
            Json(json!({"job_id": new_id, "replay_of": id})),
        )
            .into_response(),
        Err(e) => submit_err(e),
    }
}

async fn list_engines(State(ctx): State<Ctx>, Extension(cred): Extension<Credential>) -> Response {
    let engines: Vec<_> = ctx
        .svc
        .registry()
        .all()
        .iter()
        .filter(|e| cred.allows_engine(&e.engine_id))
        .map(|e| {
            json!({
                "engine_id": e.engine_id,
                "kind": e.kind,
                "model": e.model,
                "weights_sha256": e.weights_sha256,
                "licence": e.licence,
                "permits": e.permits,
                "cpu_set": e.cpu_set,
                "nice": e.nice,
                "max_job_secs": e.max_job_secs,
                "max_input_bytes": e.max_input_bytes,
            })
        })
        .collect();
    Json(json!({"engines": engines})).into_response()
}

pub fn router(svc: Arc<JobsService>, creds: Arc<CredentialSet>) -> Router {
    let max_upload = svc.config().max_upload_bytes as usize;
    let ctx = Ctx { svc, creds };
    Router::new()
        .route(
            "/v1/jobs/inputs",
            post(put_input).layer(DefaultBodyLimit::max(max_upload)),
        )
        .route(
            "/v1/jobs",
            post(submit).layer(DefaultBodyLimit::max(64 * 1024)),
        )
        .route("/v1/jobs/compare", get(compare))
        .route("/v1/jobs/{id}", get(get_job))
        .route("/v1/jobs/{id}/result", get(get_result))
        .route("/v1/jobs/{id}/ack", post(ack))
        .route("/v1/jobs/{id}/replay", post(replay))
        .route("/v1/engines", get(list_engines))
        .route_layer(middleware::from_fn_with_state(ctx.clone(), auth))
        .with_state(ctx)
}

/// Idle-only admission to Tier A, shared with interactive traffic.
struct TierAGate(Arc<AppState>);

impl AdmissionGate for TierAGate {
    fn try_admit(&self) -> Option<Box<dyn Send>> {
        self.0
            .doorman
            .try_admit_local_p2()
            .map(|p| Box::new(p) as Box<dyn Send>)
    }
}

#[cfg(unix)]
fn require_private(path: &std::path::Path) -> anyhow::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mode = std::fs::metadata(path)
        .with_context(|| format!("cannot stat {}", path.display()))?
        .permissions()
        .mode();
    if mode & 0o077 != 0 {
        bail!(
            "{} must not be group/world accessible (mode {:o})",
            path.display(),
            mode & 0o777
        );
    }
    Ok(())
}
#[cfg(not(unix))]
fn require_private(_: &std::path::Path) -> anyhow::Result<()> {
    Ok(())
}

/// Build the jobs router from the environment, or `None` when
/// `SLM_JOBS_ENABLED` is not `true`. Any misconfiguration is a boot error:
/// jobs never come up half-configured or without credentials.
pub fn build_from_env(state: Arc<AppState>) -> anyhow::Result<Option<Router>> {
    if std::env::var("SLM_JOBS_ENABLED").as_deref() != Ok("true") {
        return Ok(None);
    }
    let reg_path = PathBuf::from(
        std::env::var("SLM_ENGINE_REGISTRY_FILE")
            .context("SLM_ENGINE_REGISTRY_FILE is required when jobs are enabled")?,
    );
    let cred_path = PathBuf::from(
        std::env::var("SLM_JOBS_CREDENTIALS_FILE")
            .context("SLM_JOBS_CREDENTIALS_FILE is required when jobs are enabled")?,
    );
    require_private(&cred_path)?;
    let registry = Registry::load(&reg_path).map_err(anyhow::Error::msg)?;
    let creds = CredentialSet::load(&cred_path).map_err(anyhow::Error::msg)?;
    creds
        .validate_engines(|id| registry.get(id).is_some())
        .map_err(anyhow::Error::msg)?;

    let base = match std::env::var("SLM_JOBS_DIR") {
        Ok(d) => PathBuf::from(d),
        Err(_) => match std::env::var("FOUNDRY_ROOT") {
            Ok(r) => PathBuf::from(r).join("data/jobs"),
            Err(_) => PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
                .join(".service-slm/jobs"),
        },
    };
    let store = JobStore::new(base.clone())
        .with_context(|| format!("cannot open jobs dir {}", base.display()))?;

    let mut engines: HashMap<String, Arc<dyn JobEngine>> = HashMap::new();
    for spec in registry.all() {
        let engine: Arc<dyn JobEngine> = match spec.kind {
            EngineKind::LlmLocal => Arc::new(LlmLocalEngine::new(
                spec.endpoint.clone(),
                spec.model.clone(),
                Arc::new(TierAGate(state.clone())),
            )),
            EngineKind::SpanGliner => Arc::new(SpanGlinerEngine::new(
                spec.endpoint.clone(),
                spec.domain_id.clone(),
                std::time::Duration::from_secs(spec.max_job_secs),
            )),
            EngineKind::OcrOpenaiVision => Arc::new(OcrOpenaiVisionEngine::new(
                spec.endpoint.clone(),
                spec.model.clone(),
                std::time::Duration::from_secs(spec.max_job_secs),
            )),
        };
        engines.insert(spec.engine_id.clone(), engine);
    }

    let mut cfg = JobsConfig::default();
    if let Some(n) = std::env::var("SLM_JOBS_MAX_BUFFER_BYTES")
        .ok()
        .and_then(|v| v.parse().ok())
    {
        cfg.max_buffer_bytes = n;
    }
    if let Some(n) = std::env::var("SLM_JOBS_BUFFER_TTL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
    {
        cfg.buffer_ttl_secs = n;
    }
    if let Some(n) = std::env::var("SLM_JOBS_MAX_UPLOAD_BYTES")
        .ok()
        .and_then(|v| v.parse().ok())
    {
        cfg.max_upload_bytes = n;
    }
    let audit_state = state.clone();
    let sink: slm_doorman::jobs::service::AuditSink = Arc::new(move |e| {
        if let Err(err) = audit_state.doorman.ledger().append_job_entry(&e) {
            tracing::warn!(error = %err, "job audit write failed (non-fatal)");
        }
    });
    let svc = JobsService::new(registry, store, engines, sink, cfg).map_err(anyhow::Error::msg)?;
    svc.start();
    tracing::info!(dir = %base.display(), "jobs enabled: /v1/jobs, /v1/engines");
    Ok(Some(router(svc, Arc::new(creds))))
}
