// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

use axum::{
    body::{to_bytes, Body},
    extract::{ConnectInfo, Query, Request, State},
    http::{Method, StatusCode},
    middleware::{self, Next},
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::config_http::config_routes;
use crate::entity_filter;
use crate::graph::{GraphEntity, GraphStore, RelatedToEdge};
use crate::pairing::{InterfaceAuditLog, NonceCache, PairingKeypair, PairingRecord, PairingStore};
use crate::write_governance::WriteGovernanceDispatcher;

// ── shared server state ───────────────────────────────────────────────────────

pub struct HttpState {
    pub graph: Arc<dyn GraphStore>,
    pub doorman_endpoint: String,
    pub ontology_dir: String,
    pub corpus_dir: String,
    #[allow(dead_code)]
    pub graph_dir: String,
    pub pairing_store: Mutex<PairingStore>,
    pub nonce_cache: NonceCache,
    pub pairing_key: PairingKeypair,
    pub capability_audit: InterfaceAuditLog,
    /// SYS-ADR-19 write-governance checkpoint (`GET /v1/graph/pending`,
    /// `POST /v1/graph/verdict`). Independent of `SERVICE_CONTENT_WRITE_
    /// GOVERNANCE_ENABLED` — the dispatcher/endpoints always exist; the flag
    /// only controls whether `lib.rs`'s automated writers route through it.
    pub write_governance: WriteGovernanceDispatcher,
    /// When true (`SERVICE_CONTENT_REQUIRE_CAPABILITY=1`), a request without an
    /// `X-Foundry-Capability` header is rejected with 401 by both capability gates
    /// unless it comes from a loopback peer (the co-resident Doorman, which does not
    /// yet send the header). Default off, so current behaviour is unchanged.
    pub require_capability: bool,
}

// ── pairing request / response ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PairRequest {
    /// `<base64url(payload_json)>.<base64url(ed25519_sig)>`
    pub token: String,
    /// Base64url Ed25519 verifying key (32 bytes) of the issuing node.
    pub public_key: String,
    pub node_label: String,
}

#[derive(Debug, Serialize)]
pub struct PairResponse {
    pub status: &'static str,
    pub paired_on: String,
    pub role: String,
    pub archive_scope: Vec<String>,
}

/// Issue a new pairing invite token.
#[derive(Debug, Deserialize)]
pub struct PairTokenQuery {
    pub role: String,
    #[serde(default)]
    pub node_label: String,
    /// Comma-separated archive scope IDs (optional).
    #[serde(default)]
    pub archive_scope: String,
}

#[derive(Debug, Serialize)]
pub struct PairTokenResponse {
    pub token: String,
    pub public_key: String,
}

// ── request / response types ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ContextQuery {
    pub q: String,
    pub module_id: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    /// When >0, follow RelatedTo edges transitively (clamped to 1-4 hops).
    #[serde(default)]
    pub hops: usize,
}

fn default_limit() -> usize {
    20
}

/// Query params for GET /v1/graph/edges — the induced-subgraph edge lookup.
/// Kept as a separate endpoint (rather than folding edges into
/// `/v1/graph/context`'s response) so the existing bare-`Vec<GraphEntity>`
/// array response of that endpoint stays wire-compatible for its other
/// consumer archives; callers that want relation data fetch it here as a
/// second call.
#[derive(Debug, Deserialize)]
pub struct EdgesQuery {
    pub module_id: String,
    /// Comma-separated entity names — the node set to compute the induced
    /// subgraph over. Typically the `entity_name`s from a prior
    /// `/v1/graph/context` response.
    pub entities: String,
}

/// Query params for GET /v1/graph/delta
#[derive(Debug, Deserialize)]
pub struct DeltaQuery {
    /// ISO 8601 timestamp — return entities created at or after this instant.
    /// Example: `2026-06-29T00:00:00Z`
    pub since: String,
    pub module_id: String,
    #[serde(default = "default_delta_limit")]
    pub limit: usize,
}

fn default_delta_limit() -> usize {
    500
}

#[derive(Debug, Deserialize)]
pub struct MutateRequest {
    pub module_id: String,
    pub entities: Vec<GraphEntity>,
}

#[derive(Debug, Serialize)]
pub struct MutateResponse {
    pub upserted: usize,
}

#[derive(Debug, Deserialize)]
pub struct DraftRequest {
    pub module_id: String,
    #[serde(default)]
    pub query_hint: String,
    /// Purpose forwarded to Doorman audit_proxy allowlist check.
    /// Must be one of: initial-graph-build, entity-disambiguation,
    /// citation-grounding, editorial-refinement.
    #[serde(default = "default_purpose")]
    pub purpose: String,
    #[serde(default = "default_max_entities")]
    pub max_entities: usize,
}

fn default_purpose() -> String {
    "initial-graph-build".to_string()
}

fn default_max_entities() -> usize {
    20
}

#[derive(Debug, Serialize)]
pub struct DraftResponse {
    pub draft: String,
    pub entity_count: usize,
    pub audit_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub entity_count: usize,
}

fn default_dry_run() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct CleanupQuery {
    pub module_id: String,
    /// When true (default), report what would be deleted without deleting anything.
    /// Must be explicitly set to false to trigger deletions.
    #[serde(default = "default_dry_run")]
    pub dry_run: bool,
}

#[derive(Debug, Serialize)]
pub struct CleanupSample {
    pub entity_name: String,
    pub classification: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct CleanupReport {
    pub module_id: String,
    pub scanned: usize,
    pub flagged: usize,
    pub deleted: usize,
    pub dry_run: bool,
    /// Up to 100 flagged entity samples for inspection.
    pub samples: Vec<CleanupSample>,
}

#[derive(Debug, Deserialize)]
pub struct IngestRequest {
    pub text: String,
    #[serde(default)]
    pub module_id: String,
    #[serde(default)]
    pub doc_id: String,
    #[serde(default)]
    pub source_type: String,
}

#[derive(Debug, Serialize)]
pub struct IngestResponse {
    pub doc_id: String,
    pub queued: bool,
}

// ── handlers ──────────────────────────────────────────────────────────────────

async fn healthz(State(state): State<Arc<HttpState>>) -> Json<HealthResponse> {
    let (status, entity_count) = match state.graph.count_all() {
        Ok(n) => ("ok", n),
        Err(_) => ("degraded", 0),
    };
    Json(HealthResponse {
        status,
        entity_count,
    })
}

async fn graph_context(
    State(state): State<Arc<HttpState>>,
    Query(params): Query<ContextQuery>,
) -> Result<Json<Vec<GraphEntity>>, (StatusCode, String)> {
    let result = if params.hops > 0 {
        state.graph.query_context_transitive(
            &params.module_id,
            &params.q,
            params.limit,
            params.hops,
        )
    } else {
        state
            .graph
            .query_context(&params.module_id, &params.q, params.limit)
    };
    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// The induced subgraph of `RelatedTo` edges over a caller-supplied entity
/// set — typically the entities a prior `/v1/graph/context` call returned.
/// Surfaces relation data that `upsert_edges` writes but `/v1/graph/context`
/// itself never has (2026-07-06 ontology audit finding: edges were
/// write-only until this endpoint).
async fn graph_edges(
    State(state): State<Arc<HttpState>>,
    Query(params): Query<EdgesQuery>,
) -> Result<Json<Vec<RelatedToEdge>>, (StatusCode, String)> {
    let names: Vec<String> = params
        .entities
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    state
        .graph
        .query_edges_among(&params.module_id, &names)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn graph_mutate(
    State(state): State<Arc<HttpState>>,
    Json(mut body): Json<MutateRequest>,
) -> Result<Json<MutateResponse>, (StatusCode, String)> {
    // Backfill each entity's module_id from the request's top-level value when absent —
    // `upsert_entities` writes `entity.module_id` verbatim as the node's `e.module_id`
    // property (not the outer parameter, which only scopes the node's composite ID), so
    // an empty per-entity value would otherwise land as `module_id: ""` in the graph.
    // A caller is not required to redundantly repeat the same tenant on every entity —
    // discovered via a real integration gap: project-editorial's `graph-committer.py`
    // sends exactly this shape (2026-07-18 live end-to-end test).
    for entity in &mut body.entities {
        if entity.module_id.is_empty() {
            entity.module_id = body.module_id.clone();
        }
    }
    state
        .graph
        .upsert_entities(&body.module_id, &body.entities)
        .map(|upserted| Json(MutateResponse { upserted }))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

// ── SYS-ADR-19 write-governance checkpoint ──────────────────────────────────

/// `GET /v1/graph/pending` — list held-pending automated writes (both
/// outstanding and already-resolved, most-recently-captured first) so a
/// human reviewer can see what's queued and what a `write_id` needs for
/// `POST /v1/graph/verdict`.
async fn graph_pending(
    State(state): State<Arc<HttpState>>,
) -> Result<Json<Vec<crate::write_governance::PendingWrite>>, (StatusCode, String)> {
    crate::write_governance::list_pending(&state.write_governance.pending_dir)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// `POST /v1/graph/verdict` — a human reviewer's signed accept/reject on a
/// held-pending write. Verifies the SSH signature (shells out to
/// `ssh-keygen -Y verify`, potentially slow) via `spawn_blocking` before
/// touching any state — mirrors the apprenticeship substrate's own
/// verify-before-mutate ordering.
async fn graph_verdict(
    State(state): State<Arc<HttpState>>,
    Json(wire): Json<crate::write_governance::WriteVerdictWireBody>,
) -> Result<Json<crate::write_governance::WriteVerdictDispatchOutcome>, (StatusCode, String)> {
    let dispatcher = state.write_governance.clone();
    let graph = state.graph.clone();
    let result = tokio::task::spawn_blocking(move || dispatcher.dispatch(wire, graph.as_ref()))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("join: {e}")))?;

    result.map(Json).map_err(|e| {
        use crate::write_governance::WriteGovernanceError as E;
        let status = match &e {
            E::VerifySignature(_) => StatusCode::FORBIDDEN,
            E::VerdictParse(_) => StatusCode::BAD_REQUEST,
            E::OrphanVerdict { .. } => StatusCode::GONE,
            E::AlreadyResolved { .. } => StatusCode::CONFLICT,
            E::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, e.to_string())
    })
}

/// Drafting pipeline.
///
/// Queries the LadybugDB graph for entities matching the request's module and
/// query_hint, packages them into a ≤2 000-token structured prompt, then sends
/// it to the Doorman's local chat route (`/v1/chat/completions`, model
/// `doorman-tier-a`, low complexity). No external provider is called
/// (operator ruling 2026-10-26, audit row 213 / Q287 A; the fleet has no Claude
/// API key). The handler surfaces the upstream status and message rather than
/// masking it.
///
/// `audit_id` in the response is the Doorman's `request_id` for this call: the
/// chat route does not return a separate `audit_id` (that is `/v1/audit/proxy`
/// only), and the request id is the key of the inference ledger entry.
async fn draft_generate(
    State(state): State<Arc<HttpState>>,
    Json(body): Json<DraftRequest>,
) -> Result<Json<DraftResponse>, (StatusCode, String)> {
    // 1. Fetch graph entities.
    let entities = state
        .graph
        .query_context(&body.module_id, &body.query_hint, body.max_entities)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("graph query failed: {e}"),
            )
        })?;

    let entity_count = entities.len();

    // 1b. Fetch the induced subgraph of edges among these entities — relation
    // data `upsert_edges` writes but, until now, nothing ever read back.
    // Non-fatal: a query failure here degrades to entity-only rendering
    // rather than failing the whole draft request.
    let names: Vec<String> = entities.iter().map(|e| e.entity_name.clone()).collect();
    let edges = state
        .graph
        .query_edges_among(&body.module_id, &names)
        .unwrap_or_default();

    // 2. Package entities + relationships into a structured prompt (≤2 000
    // tokens ≈ 8 000 chars).
    let entity_block = format_entity_block(&entities, &edges);

    let system_prompt = "You are a knowledge graph analyst for a real estate property \
        management archive. Given a list of extracted entities from the Totebox Archive, \
        produce a concise structured summary document. \
        Include: key people and their roles, active projects, notable locations, \
        and company relationships. Write in precise professional prose. \
        Maximum 600 words.";

    let user_message = format!(
        "Module: {module_id}\nQuery context: {hint}\n\n{entity_block}",
        module_id = body.module_id,
        hint = if body.query_hint.is_empty() {
            "(all entities)"
        } else {
            &body.query_hint
        },
        entity_block = entity_block
    );

    // Truncate to keep under 8 000 chars (~2 000 tokens).
    let user_message = if user_message.len() > 8000 {
        format!(
            "{}\n\n[truncated — {entity_count} entities total]",
            truncate_at_char_boundary(&user_message, 7900)
        )
    } else {
        user_message
    };

    // 3. Build the chat-completions JSON. `purpose` is accepted on the request
    // for compatibility but is not sent: the local route has no purpose field.
    let chat_body = serde_json::json!({
        "model": "doorman-tier-a",
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_message}
        ],
        "max_tokens": 1024,
        "temperature": 0.3
    });

    // 4. POST to the Doorman local chat route. The Doorman assigns the
    // request id (it must be a UUID, so none is sent).
    let url = format!("{}/v1/chat/completions", state.doorman_endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .header("X-Foundry-Module-ID", &body.module_id)
        .header("X-Foundry-Complexity", "low")
        .json(&chat_body)
        .timeout(Duration::from_secs(120))
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Doorman unreachable: {e}")))?;

    let status = res.status();
    let resp_json: serde_json::Value = res
        .json()
        .await
        .unwrap_or_else(|_| serde_json::json!({"error": "invalid JSON from Doorman"}));

    if !status.is_success() {
        let msg = resp_json
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("upstream error")
            .to_string();
        return Err((
            StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
            format!("doorman chat {}: {}", status.as_u16(), msg),
        ));
    }

    // 5. Extract the draft text from the completion.
    let audit_id = resp_json
        .get("request_id")
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let draft = resp_json
        .pointer("/choices/0/message/content")
        .or_else(|| resp_json.get("content"))
        .and_then(|v| v.as_str())
        .map(strip_think_blocks)
        .unwrap_or_else(|| "(no content returned)".to_string());

    Ok(Json(DraftResponse {
        draft,
        entity_count,
        audit_id,
    }))
}

/// `POST /v1/ingest` — submit a document for DataGraph enrichment.
///
/// Writes a `CORPUS_<doc_id>_<ts>.json` file to the watched corpus directory.
/// The file watcher picks it up within milliseconds and routes it through the
/// Tier A → Tier B enrichment cascade, writing extracted entities to LadybugDB.
///
/// Returns 202 Accepted with `{ "doc_id": "...", "queued": true }`.
async fn ingest_document(
    State(state): State<Arc<HttpState>>,
    Json(body): Json<IngestRequest>,
) -> Result<(axum::http::StatusCode, Json<IngestResponse>), (axum::http::StatusCode, String)> {
    if body.text.trim().is_empty() {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "text must not be empty".to_string(),
        ));
    }

    let doc_id = if body.doc_id.is_empty() {
        format!(
            "{:x}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos()
        )
    } else {
        // Sanitise: allow only alphanumeric + hyphens/underscores
        body.doc_id
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect()
    };

    let module_id = if body.module_id.is_empty() {
        "woodfine"
    } else {
        body.module_id.as_str()
    };
    let source_type = if body.source_type.is_empty() {
        "ingest-api"
    } else {
        body.source_type.as_str()
    };

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    let worm_id = format!("DOC_{}_{}", doc_id, ts);
    let corpus_json = serde_json::json!({
        "worm_id": worm_id,
        "corpus": body.text,
        "module_id": module_id,
        "source_type": source_type,
    });

    let filename = format!("CORPUS_{}_{}.json", doc_id, ts);
    let path = std::path::Path::new(&state.corpus_dir).join(&filename);

    tokio::fs::write(&path, corpus_json.to_string())
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to write corpus file: {e}"),
            )
        })?;

    Ok((
        axum::http::StatusCode::ACCEPTED,
        Json(IngestResponse {
            doc_id,
            queued: true,
        }),
    ))
}

/// `GET /v1/graph/cleanup` — scan and optionally delete noise entities.
///
/// Default is dry_run=true (safe). Pass dry_run=false to apply deletions.
/// Example: curl 'http://127.0.0.1:9081/v1/graph/cleanup?module_id=jennifer&dry_run=true'
/// GET /v1/graph/delta — federation delta sync.
/// Returns entities created at or after `?since=<ISO-8601>` for a given `module_id`.
/// Consumers (app-orchestration-graph) call this to pull incremental updates.
async fn graph_delta(
    State(state): State<Arc<HttpState>>,
    Query(params): Query<DeltaQuery>,
) -> Result<Json<Vec<GraphEntity>>, (StatusCode, String)> {
    let entities = state
        .graph
        .query_entities_since(&params.module_id, &params.since, params.limit)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entities))
}

///
/// Uses the same noise filters as the ingest gate so cleanup matches exactly what
/// the hardened binary would have rejected on ingestion.
async fn graph_cleanup(
    State(state): State<Arc<HttpState>>,
    Query(params): Query<CleanupQuery>,
) -> Result<Json<CleanupReport>, (StatusCode, String)> {
    let entities = state.graph.list_entities(&params.module_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("list_entities failed: {e}"),
        )
    })?;

    let scanned = entities.len();
    let mut flagged = 0usize;
    let mut deleted = 0usize;
    let mut samples: Vec<CleanupSample> = Vec::new();

    for entity in &entities {
        let reason: Option<&'static str> =
            if entity_filter::is_noise_entity_name(&entity.entity_name) {
                Some("noise-name")
            } else if entity.entity_name.split_whitespace().count() > 8 {
                Some("fragment-wordcount")
            } else if entity_filter::coerce_classification(
                &entity.entity_name,
                &entity.classification,
            )
            .is_none()
            {
                Some("type-incoherent")
            } else if !entity_filter::is_allowed_classification(&entity.classification) {
                Some("oov-classification")
            } else {
                None
            };

        if let Some(r) = reason {
            flagged += 1;
            if samples.len() < 100 {
                samples.push(CleanupSample {
                    entity_name: entity.entity_name.clone(),
                    classification: entity.classification.clone(),
                    reason: r.to_string(),
                });
            }
            if !params.dry_run {
                match state
                    .graph
                    .delete_entity(&params.module_id, &entity.entity_name)
                {
                    Ok(()) => deleted += 1,
                    Err(e) => eprintln!(
                        "[CLEANUP] delete_entity failed for '{}': {e}",
                        entity.entity_name
                    ),
                }
            }
        }
    }

    Ok(Json(CleanupReport {
        module_id: params.module_id,
        scanned,
        flagged,
        deleted,
        dry_run: params.dry_run,
        samples,
    }))
}

// ── enrichment endpoint ───────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EnrichQuery {
    pub module_id: String,
    /// Only enrich entities of this classification (default: Person).
    #[serde(default = "default_enrich_classification")]
    pub classification: String,
    /// Max entities to enrich per call (default: 10). Gate: total ≤ 50.
    #[serde(default = "default_enrich_limit")]
    pub limit: usize,
    /// When true (default), report which entities WOULD be enriched without calling Tier A.
    #[serde(default = "default_dry_run")]
    pub dry_run: bool,
}

fn default_enrich_classification() -> String {
    "Person".to_string()
}

fn default_enrich_limit() -> usize {
    10
}

#[derive(Debug, Serialize)]
pub struct EnrichResponse {
    pub module_id: String,
    pub classification: String,
    pub null_vector_count: usize,
    pub enriched: usize,
    pub unchanged: usize,
    pub dry_run: bool,
    pub samples: Vec<String>,
}

/// `POST /v1/graph/enrich` — retroactively fill NULL role_vector entries via Tier A inference.
///
/// Queries LadybugDB for Person entities (or the requested classification) whose
/// `role_vector` is NULL, then asks Tier A (local OLMo) "what is X's role?" for
/// each entity and writes back any non-empty answer.
///
/// Default: dry_run=true — reports how many entities would be enriched without
/// calling Tier A. Set dry_run=false to trigger live enrichment.
///
/// Note (I4, 2026-06-27): if the source corpus contains only technical text
/// (code diffs, commit messages), Tier A will answer "unknown" for most entities.
/// The real unlock is source diversification — routing CRM/people data through
/// the extraction pipeline. This endpoint provides the infrastructure for when
/// richer sources become available.
///
/// Example: curl -X POST 'http://127.0.0.1:9081/v1/graph/enrich?module_id=jennifer&dry_run=false&limit=5'
async fn graph_enrich(
    State(state): State<Arc<HttpState>>,
    Query(params): Query<EnrichQuery>,
) -> Result<Json<EnrichResponse>, (StatusCode, String)> {
    let limit = params.limit.min(50);

    // Find all entities for this module; filter NULL role_vector + matching classification.
    let all = state.graph.list_entities(&params.module_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("list_entities: {e}"),
        )
    })?;

    let candidates: Vec<_> = all
        .into_iter()
        .filter(|e| e.role_vector.is_none() && e.classification == params.classification)
        .take(limit)
        .collect();

    let null_vector_count = candidates.len();
    let mut samples: Vec<String> = candidates.iter().map(|e| e.entity_name.clone()).collect();
    samples.truncate(20);

    if params.dry_run || candidates.is_empty() {
        return Ok(Json(EnrichResponse {
            module_id: params.module_id,
            classification: params.classification,
            null_vector_count,
            enriched: 0,
            unchanged: 0,
            dry_run: true,
            samples,
        }));
    }

    // Live enrichment: ask Tier A for each entity's role.
    let client = reqwest::Client::new();
    let chat_url = format!("{}/v1/chat/completions", state.doorman_endpoint);
    let mut enriched = 0usize;
    let mut unchanged = 0usize;

    for entity in candidates {
        let prompt = format!(
            "What is the professional role or job title of {}? \
             Answer in 3-7 words (e.g. \"Senior Software Engineer\" or \"CEO, Woodfine Management Corp.\"). \
             If unknown, answer exactly: unknown",
            entity.entity_name
        );
        let body = serde_json::json!({
            "model": "local",
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 30,
            "temperature": 0.0
        });
        let res = client
            .post(&chat_url)
            .header("X-Foundry-Module-ID", &params.module_id)
            .json(&body)
            .timeout(Duration::from_secs(60))
            .send()
            .await;

        let role_text = match res {
            Ok(r) if r.status().is_success() => {
                let j: serde_json::Value = r.json().await.unwrap_or_default();
                j.pointer("/choices/0/message/content")
                    .and_then(|v| v.as_str())
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty() && s.to_lowercase() != "unknown")
            }
            _ => None,
        };

        match role_text {
            Some(role) => {
                let mut updated = entity.clone();
                updated.role_vector = Some(role);
                if state
                    .graph
                    .upsert_entities(&params.module_id, &[updated])
                    .is_ok()
                {
                    enriched += 1;
                } else {
                    unchanged += 1;
                }
            }
            None => {
                unchanged += 1;
            }
        }
    }

    Ok(Json(EnrichResponse {
        module_id: params.module_id,
        classification: params.classification,
        null_vector_count,
        enriched,
        unchanged,
        dry_run: false,
        samples,
    }))
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Truncate a string to at most `max_bytes` bytes, snapped down to the nearest UTF-8 char
/// boundary. A raw byte-index slice (`&s[..max_bytes]`) panics if `max_bytes` lands
/// mid-character — guaranteed eventually given multi-byte content (accented names, em-dashes,
/// the bilingual mandate) landing near the truncation point.
/// Remove `<think>…</think>` reasoning blocks that a think-model may leave in
/// `content` when the upstream did not split them into `reasoning_content`.
fn strip_think_blocks(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(start) = rest.find("<think>") {
        out.push_str(&rest[..start]);
        match rest[start..].find("</think>") {
            Some(end) => rest = &rest[start + end + "</think>".len()..],
            None => {
                rest = "";
                break;
            }
        }
    }
    out.push_str(rest);
    out.trim().to_string()
}

fn truncate_at_char_boundary(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

/// Retrieval-stage guardrail: entities below this confidence are excluded
/// from rendered context entirely (narration and citation both), not just
/// down-weighted. Deliberately well below the write-path grounding floors
/// already in effect (0.65 for OLMo-grounded, 0.75 for GLiNER, 0.9 for
/// taxonomy-seeded) — this is a safety net against genuinely low-quality
/// data reaching a consumer, not a routine filter; under normal operation
/// almost nothing should be excluded by it.
///
/// Deliberately does NOT also require `source_doc` to be present: taxonomy-
/// seeded entities (guides, archetypes, domains) are legitimately sourceless
/// but trustworthy — excluding them would break rendering for entities that
/// were never extraction output in the first place. `source_doc` is still
/// rendered as a citation when present.
const MIN_RENDER_CONFIDENCE: f64 = 0.5;

fn format_entity_block(entities: &[GraphEntity], edges: &[RelatedToEdge]) -> String {
    let rendered: Vec<&GraphEntity> = entities
        .iter()
        .filter(|e| e.confidence >= MIN_RENDER_CONFIDENCE)
        .collect();
    if rendered.is_empty() {
        return "(no entities found in graph)".to_string();
    }
    let mut out = String::from("## Extracted Entities\n\n");
    for e in &rendered {
        out.push_str(&format!("- **{}** ({})", e.entity_name, e.classification));
        if let Some(r) = &e.role_vector {
            out.push_str(&format!("; role: {r}"));
        }
        if let Some(l) = &e.location_vector {
            out.push_str(&format!("; location: {l}"));
        }
        if let Some(c) = &e.contact_vector {
            out.push_str(&format!("; contact: {c}"));
        }
        out.push_str(&format!("; confidence: {:.2}", e.confidence));
        if let Some(src) = &e.source_doc {
            out.push_str(&format!("; source: {src}"));
        }
        out.push('\n');
    }
    if !edges.is_empty() {
        out.push_str("\n## Relationships\n\n");
        for edge in edges {
            out.push_str(&format!(
                "- {} --[{}]--> {}\n",
                edge.src_entity_name, edge.relation_type, edge.tgt_entity_name
            ));
        }
    }
    out
}

// ── pairing handlers ─────────────────────────────────────────────────────────

async fn pair_peer(
    State(state): State<Arc<HttpState>>,
    Json(body): Json<PairRequest>,
) -> Result<Json<PairResponse>, (StatusCode, String)> {
    use crate::pairing::{verify_pair_token, PairError};

    let payload = verify_pair_token(&body.token, &body.public_key).map_err(|e| match e {
        PairError::Malformed => (StatusCode::BAD_REQUEST, e.to_string()),
        PairError::BadSignature => (StatusCode::UNAUTHORIZED, e.to_string()),
        PairError::Expired => (StatusCode::UNAUTHORIZED, "token expired".into()),
        PairError::NonceReused => (StatusCode::CONFLICT, "nonce already used".into()),
    })?;

    // already_paired check before nonce uniqueness — a re-submit of an existing
    // pairing must return already_paired even if the nonce was previously used.
    {
        let store = state.pairing_store.lock().unwrap();
        if let Some(existing) = store.get(&body.public_key) {
            return Ok(Json(PairResponse {
                status: "already_paired",
                paired_on: existing.paired_on.clone(),
                role: existing.role.clone(),
                archive_scope: existing.archive_scope.clone(),
            }));
        }
    }

    if !state.nonce_cache.try_insert(&payload.nonce) {
        return Err((StatusCode::CONFLICT, "nonce already used".into()));
    }

    let paired_on = Utc::now().to_rfc3339();
    let rec = PairingRecord {
        public_key: body.public_key.clone(),
        issuer: payload.issuer.clone(),
        peer_type: payload.peer_type.clone(),
        role: payload.role.clone(),
        archive_scope: payload.archive_scope.clone(),
        node_label: body.node_label.clone(),
        paired_on: paired_on.clone(),
        nonce: payload.nonce.clone(),
    };

    state
        .pairing_store
        .lock()
        .unwrap()
        .insert(rec)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PairResponse {
        status: "paired",
        paired_on,
        role: payload.role,
        archive_scope: payload.archive_scope,
    }))
}

/// List all paired peers — public_key, role, node_label, paired_on.
async fn list_pairs(State(state): State<Arc<HttpState>>) -> Json<Vec<PairingRecord>> {
    Json(state.pairing_store.lock().unwrap().list())
}

/// Issue a new signed invite token (Totebox → caller).
async fn issue_pair_token(
    State(state): State<Arc<HttpState>>,
    Query(q): Query<PairTokenQuery>,
) -> Result<Json<PairTokenResponse>, (StatusCode, String)> {
    let scope: Vec<String> = if q.archive_scope.is_empty() {
        vec![]
    } else {
        q.archive_scope
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };
    let label = if q.node_label.is_empty() {
        "totebox"
    } else {
        &q.node_label
    };
    let token = state.pairing_key.issue_token(&q.role, scope, label);
    Ok(Json(PairTokenResponse {
        token,
        public_key: state.pairing_key.verifying_key_b64.clone(),
    }))
}

// ── capability gate middleware ────────────────────────────────────────────────

/// Pure scope-vs-target + grant-vs-forward policy check for a verified
/// capability, independent of any HTTP/router plumbing so it can be
/// unit-tested directly.
///
/// `["*"]` in `archive_scope` is the operator-override wildcard
/// (BRIEF-datagraph-tenant-isolation.md, Decisions Locked):
/// - Honored ONLY for `user_scope == "ADMIN"` capabilities.
/// - Never honored on the mutate route — the override is read-only even for
///   the operator, full stop.
///
/// A non-wildcard scope must contain the request's target archive exactly.
///
/// **grant-vs-forward** (Session 4): `forwarded_for` is `None` for a direct
/// grant (the signing peer's own capability) or `Some(origin)` when the peer
/// claims to be relaying a capability on behalf of a third instance. A
/// forward claim is only honored for `user_scope == "ADMIN"` signers —
/// otherwise an ordinary paired peer could unilaterally claim to forward on
/// behalf of someone with broader access than itself, defeating the point of
/// distinguishing the two in the first place. This check is independent of
/// (and applies before) the scope-vs-target check above.
fn scope_permits_request(
    archive_scope: &[String],
    user_scope: &str,
    target_module_id: Option<&str>,
    is_mutate_route: bool,
    forwarded_for: Option<&str>,
) -> Result<(), String> {
    if forwarded_for.is_some() && user_scope != "ADMIN" {
        return Err(format!(
            "capability claims forwarded_for={forwarded_for:?} but signer's user_scope \
             {user_scope:?} is not ADMIN — only ADMIN-scoped peers may forward"
        ));
    }

    let is_wildcard = archive_scope.iter().any(|s| s == "*");

    if is_wildcard && is_mutate_route {
        return Err(
            "wildcard archive_scope capability is read-only; not honored on /v1/graph/mutate"
                .to_string(),
        );
    }
    if is_wildcard && user_scope != "ADMIN" {
        return Err("wildcard archive_scope requires an ADMIN-role capability".to_string());
    }
    if is_wildcard {
        return Ok(());
    }

    match target_module_id {
        Some(target) if archive_scope.iter().any(|s| s == target) => Ok(()),
        Some(target) => Err(format!(
            "capability archive_scope {archive_scope:?} does not cover target archive {target:?}"
        )),
        None => Err("could not determine target archive (module_id) for scope check".to_string()),
    }
}

/// Whether a request with no `X-Foundry-Capability` header may pass a capability
/// gate. Always true while `require_capability` is off. When on, only a loopback
/// peer passes; an unknown peer address (no `ConnectInfo`) is treated as not
/// loopback, so the check fails closed.
fn headerless_allowed(state: &HttpState, req: &Request) -> bool {
    if !state.require_capability {
        return true;
    }
    req.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|c| c.0.ip().is_loopback())
        .unwrap_or(false)
}

/// Verifies a forwarded `X-Foundry-Capability` header on WORM-touching routes.
///
/// Requests with no header pass through unchanged — this preserves the current
/// localhost-trusted path (the Doorman calls `/v1/graph/mutate` directly and
/// does not yet send this header). Requests that DO present the header must
/// verify against a registered peer's public key (resolved by `from_instance`
/// via `PairingStore::resolve_public_key`, which checks service-content's own
/// direct `/v1/pair` registrations first, falling back to peers Command has
/// paired — O9 credential-chain wiring, the platform design notes) or are
/// rejected. A verified request is recorded to `interface-audit.jsonl` before
/// continuing, tagged with which trust source resolved it.
/// Parses, signature-verifies, and grant-ceiling-checks an `X-Foundry-Capability`
/// header value, shared by [`capability_gate`] and [`admin_capability_gate`] —
/// everything up through "is this signed payload within what this peer was
/// actually granted at `/v1/pair` time" is identical between the two; only the
/// policy check applied *after* this (archive-scope-vs-target for tenant
/// routes, ADMIN-only for global config routes) differs.
///
/// Returns the verified payload plus the trust-source label for the audit log.
async fn verify_capability_grant(
    state: &Arc<HttpState>,
    header_str: &str,
) -> Result<(crate::pairing::CapabilityPayload, &'static str), (StatusCode, String)> {
    use crate::pairing::{verify_capability, CapabilityError};

    let (payload_b64, _) = header_str.split_once('.').ok_or((
        StatusCode::UNAUTHORIZED,
        "malformed capability token".to_string(),
    ))?;
    let payload_bytes = URL_SAFE_NO_PAD.decode(payload_b64).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "malformed capability token".to_string(),
        )
    })?;
    let unverified: serde_json::Value = serde_json::from_slice(&payload_bytes).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "malformed capability payload".to_string(),
        )
    })?;
    let from_instance = unverified
        .get("from_instance")
        .and_then(|v| v.as_str())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "capability payload missing from_instance".to_string(),
        ))?;

    // Resolve the peer's public key AND (for a direct registration) its full
    // granted `PairingRecord` in the same lock acquisition — the record's
    // `role`/`archive_scope` are the actual grant from `/v1/pair` time, used
    // below as a ceiling on what the peer's own signed payload may claim.
    let (resolved, direct_grant) = {
        let store = state.pairing_store.lock().unwrap();
        let resolved = store.resolve_public_key(from_instance);
        let direct_grant = resolved.as_ref().and_then(|r| {
            if matches!(r, crate::pairing::ResolvedPeer::Direct(_)) {
                store.get(r.public_key()).cloned()
            } else {
                None
            }
        });
        (resolved, direct_grant)
    };
    let resolved = resolved.ok_or((
        StatusCode::FORBIDDEN,
        format!("{from_instance} is not a paired peer"),
    ))?;
    let public_key = resolved.public_key().to_string();
    let trust_source = resolved.source_label();

    let verified = verify_capability(header_str, &public_key).map_err(|e| match e {
        CapabilityError::Malformed => (StatusCode::BAD_REQUEST, e.to_string()),
        CapabilityError::BadSignature => (StatusCode::UNAUTHORIZED, e.to_string()),
        CapabilityError::Expired => (StatusCode::UNAUTHORIZED, e.to_string()),
    })?;

    if !state.nonce_cache.try_insert(&verified.nonce) {
        return Err((
            StatusCode::CONFLICT,
            "capability nonce already used".to_string(),
        ));
    }

    // ── grant ceiling (S1 fix — tenant-isolation bypass) ──────────────────
    //
    // Signature verification only proves the caller holds the private key
    // for a *paired* public key — it does not prove the caller is entitled
    // to whatever `user_scope`/`archive_scope` it put in its own signed
    // payload. Before this check, a peer paired with role USER and a single
    // archive could self-sign a payload claiming `user_scope: "ADMIN"`,
    // `archive_scope: ["*"]` and the gate would honor it outright, silently
    // defeating the tenant-isolation design's "operator override,
    // ADMIN-role only" design. Enforce the payload as a ceiling under the
    // actual `/v1/pair`-time grant, not a self-description trusted as-is.
    //
    // Direct peers: role must match the granted role (ADMIN's grant covers
    // any lower self-asserted role — a peer may legitimately scope itself
    // down for one request); every self-asserted archive must be present in
    // the granted `archive_scope` (or the grant itself contains `"*"`).
    // ViaCommand peers: only the role ceiling applies — Command's minimal
    // pairing record carries no per-archive scope, and that path already
    // represents an operator-level trust source distinct from archive-to-
    // archive `/v1/pair` (see `CommandPairedPeer`'s own doc comment).
    let user_scope_within_grant =
        |granted_role: &str| granted_role == "ADMIN" || granted_role == verified.user_scope;
    match (&resolved, &direct_grant) {
        (crate::pairing::ResolvedPeer::Direct(_), Some(grant)) => {
            if !user_scope_within_grant(&grant.role) {
                return Err((
                    StatusCode::FORBIDDEN,
                    format!(
                        "capability claims user_scope {:?} but this peer was only granted {:?}",
                        verified.user_scope, grant.role
                    ),
                ));
            }
            let grant_is_wildcard = grant.archive_scope.iter().any(|s| s == "*");
            if !grant_is_wildcard {
                if let Some(bad) = verified
                    .archive_scope
                    .iter()
                    .find(|s| *s != "*" && !grant.archive_scope.contains(s))
                {
                    return Err((
                        StatusCode::FORBIDDEN,
                        format!(
                            "capability claims archive_scope entry {bad:?} not covered by this \
                             peer's granted archive_scope {:?}",
                            grant.archive_scope
                        ),
                    ));
                }
                if verified.archive_scope.iter().any(|s| s == "*") {
                    return Err((
                        StatusCode::FORBIDDEN,
                        "capability claims wildcard archive_scope but this peer's grant is not \
                         wildcard"
                            .to_string(),
                    ));
                }
            }
        }
        (crate::pairing::ResolvedPeer::Direct(_), None) => {
            // Resolved via a direct public-key match with no stored record
            // (shouldn't happen in practice — `resolve_public_key`'s Direct
            // path only returns keys found in the store — but fail closed
            // rather than silently skip the grant check if it ever does).
            return Err((
                StatusCode::FORBIDDEN,
                "no pairing record found for this peer's grant".to_string(),
            ));
        }
        (crate::pairing::ResolvedPeer::ViaCommand { role, .. }, _) => {
            if !user_scope_within_grant(role) {
                return Err((
                    StatusCode::FORBIDDEN,
                    format!(
                        "capability claims user_scope {:?} but Command granted this peer {:?}",
                        verified.user_scope, role
                    ),
                ));
            }
        }
    }

    Ok((verified, trust_source))
}

/// Verifies a forwarded `X-Foundry-Capability` header on WORM-touching routes.
///
/// Requests with no header pass through unchanged — this preserves the current
/// localhost-trusted path (the Doorman calls `/v1/graph/mutate` directly and
/// does not yet send this header). Requests that DO present the header must
/// verify against a registered peer's public key (resolved by `from_instance`
/// via `PairingStore::resolve_public_key`, which checks service-content's own
/// direct `/v1/pair` registrations first, falling back to peers Command has
/// paired — O9 credential-chain wiring, the platform design notes) or are
/// rejected. A verified request is recorded to `interface-audit.jsonl` before
/// continuing, tagged with which trust source resolved it.
async fn capability_gate(
    State(state): State<Arc<HttpState>>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let Some(header_value) = req.headers().get("X-Foundry-Capability") else {
        if headerless_allowed(&state, &req) {
            return Ok(next.run(req).await);
        }
        return Err((
            StatusCode::UNAUTHORIZED,
            "X-Foundry-Capability header required".to_string(),
        ));
    };
    let header_str = header_value
        .to_str()
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "malformed X-Foundry-Capability header".to_string(),
            )
        })?
        .to_string();

    let (verified, trust_source) = verify_capability_grant(&state, &header_str).await?;

    // ── scope-vs-target (BRIEF-datagraph-tenant-isolation.md, Decisions Locked) ──
    //
    // A forwarded capability's `archive_scope` must actually cover the archive
    // this request targets. Note what this does NOT implement yet: "grant-vs-
    // forward" (distinguishing a capability granted directly to the caller from
    // one relayed by a third instance) — no wire-format field exists to express
    // that distinction today; this is an explicitly open item (see NEXT.md),
    // not silently assumed safe. See `scope_permits_request` for the pure
    // policy logic (unit-tested independently of the router/HTTP plumbing).
    let is_mutate_route = req.method() == Method::POST && req.uri().path() == "/v1/graph/mutate";
    // /v1/ingest carries its target module_id in the JSON body (`IngestRequest`),
    // not the query string — added to the gates alongside cleanup, enrich and
    // delta; needs the same body-parsing path as mutate, not the query-string
    // fallback below (which would always return None for a JSON POST).
    let is_ingest_route = req.method() == Method::POST && req.uri().path() == "/v1/ingest";

    let (req, target_module_id) = if is_mutate_route || is_ingest_route {
        let (parts, body) = req.into_parts();
        let bytes = to_bytes(body, 10 * 1024 * 1024).await.map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("failed to read request body: {e}"),
            )
        })?;
        let target = serde_json::from_slice::<serde_json::Value>(&bytes)
            .ok()
            .and_then(|v| {
                v.get("module_id")
                    .and_then(|m| m.as_str())
                    .map(String::from)
            });
        (Request::from_parts(parts, Body::from(bytes)), target)
    } else {
        let target = req.uri().query().and_then(|q| {
            q.split('&')
                .find_map(|kv| kv.strip_prefix("module_id=").map(String::from))
        });
        (req, target)
    };

    // /v1/ingest is a write (creates a new corpus document) — the "operator
    // override is read-only, even for ADMIN" rule applies equally to it, not
    // just to /v1/graph/mutate.
    if let Err(msg) = scope_permits_request(
        &verified.archive_scope,
        &verified.user_scope,
        target_module_id.as_deref(),
        is_mutate_route || is_ingest_route,
        verified.forwarded_for.as_deref(),
    ) {
        let status = if msg.starts_with("could not determine") {
            StatusCode::BAD_REQUEST
        } else {
            StatusCode::FORBIDDEN
        };
        return Err((status, msg));
    }

    let endpoint = req.uri().path().to_string();
    if let Err(e) = state
        .capability_audit
        .record(&endpoint, &verified, trust_source)
    {
        eprintln!("[HTTP] interface-audit write failed: {e}");
    }

    Ok(next.run(req).await)
}

/// Global config routes (`/v1/config/*`, taxonomy rewrite) have no
/// natural per-archive `module_id` to scope against (`scope_permits_request`'s
/// archive-scope-vs-target check does not apply to them), so this is a
/// separate, simpler gate rather than reusing [`capability_gate`] outright:
/// same signature/nonce/grant-ceiling verification via
/// [`verify_capability_grant`], then a flat requirement that the *granted*
/// role (not just the self-asserted one — the ceiling check already enforces
/// that) is `ADMIN`. Absent-header requests still pass through unchanged,
/// preserving the same localhost-trusted default as `capability_gate`.
async fn admin_capability_gate(
    State(state): State<Arc<HttpState>>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let Some(header_value) = req.headers().get("X-Foundry-Capability") else {
        if headerless_allowed(&state, &req) {
            return Ok(next.run(req).await);
        }
        return Err((
            StatusCode::UNAUTHORIZED,
            "X-Foundry-Capability header required".to_string(),
        ));
    };
    let header_str = header_value
        .to_str()
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "malformed X-Foundry-Capability header".to_string(),
            )
        })?
        .to_string();

    let (verified, trust_source) = verify_capability_grant(&state, &header_str).await?;

    if verified.user_scope != "ADMIN" {
        return Err((
            StatusCode::FORBIDDEN,
            format!(
                "config routes require an ADMIN capability, got {:?}",
                verified.user_scope
            ),
        ));
    }

    let endpoint = req.uri().path().to_string();
    if let Err(e) = state
        .capability_audit
        .record(&endpoint, &verified, trust_source)
    {
        eprintln!("[HTTP] interface-audit write failed: {e}");
    }

    Ok(next.run(req).await)
}

// ── server entrypoint ─────────────────────────────────────────────────────────

/// Builds the full axum `Router` against a given `HttpState`, with no binding/serving
/// side effects — separated from `run_server` so tests can drive the real router via
/// `tower::ServiceExt::oneshot` instead of only unit-testing individual handlers.
pub fn router(state: Arc<HttpState>) -> Router {
    // Capability-gated: the DataGraph read, write and destructive routes. A
    // request that presents an X-Foundry-Capability header is verified against a
    // paired peer; a headerless request passes only while
    // SERVICE_CONTENT_REQUIRE_CAPABILITY is off, or from a loopback peer when it
    // is on (the co-resident Doorman). `/v1/graph/cleanup` deletes entities in
    // bulk when `dry_run=false`; `/v1/ingest` writes a corpus document and
    // carries its module_id in the JSON body.
    let capability_gated = Router::new()
        .route("/v1/graph/context", get(graph_context))
        .route("/v1/graph/mutate", post(graph_mutate))
        .route("/v1/graph/edges", get(graph_edges))
        .route("/v1/graph/delta", get(graph_delta))
        .route("/v1/graph/cleanup", get(graph_cleanup))
        .route("/v1/graph/enrich", post(graph_enrich))
        .route("/v1/ingest", post(ingest_document))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            capability_gate,
        ));

    // Global routes with no per-archive module_id to scope against: the
    // taxonomy config routes, minting invite tokens and listing pairings. A
    // presented header must carry an ADMIN grant.
    let admin_gated = config_routes()
        .route("/v1/pair/token", get(issue_pair_token))
        .route("/v1/pairs", get(list_pairs))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            admin_capability_gate,
        ));

    Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/draft/generate", post(draft_generate))
        .route("/v1/pair", post(pair_peer))
        .route("/v1/graph/pending", get(graph_pending))
        .route("/v1/graph/verdict", post(graph_verdict))
        .merge(capability_gated)
        .merge(admin_gated)
        .with_state(state)
}

pub async fn run_server(
    store: Arc<dyn GraphStore>,
    bind_addr: String,
    doorman_endpoint: String,
    ontology_dir: String,
    corpus_dir: String,
    graph_dir: String,
) {
    let pairing_store = PairingStore::load(&graph_dir).unwrap_or_else(|e| {
        eprintln!("[HTTP] pairing store load failed: {e}; starting empty");
        PairingStore::load("/tmp").unwrap_or_else(|e2| {
            eprintln!("[FATAL] pairing store fallback load also failed: {e2}");
            std::process::exit(1);
        })
    });
    let pairing_key = PairingKeypair::load_or_generate(&graph_dir).unwrap_or_else(|e| {
        eprintln!("[HTTP] pairing keypair init failed: {e}");
        PairingKeypair::load_or_generate("/tmp").unwrap_or_else(|e2| {
            eprintln!("[FATAL] pairing keypair fallback init also failed: {e2}");
            std::process::exit(1);
        })
    });

    let capability_audit = InterfaceAuditLog::new(&graph_dir);

    let write_governance = WriteGovernanceDispatcher {
        verifier: Arc::new(crate::write_governance::SshKeygenVerifier::new(
            crate::write_governance::allowed_signers_path(),
        )),
        pending_dir: crate::write_governance::pending_writes_dir(),
    };

    let require_capability = std::env::var("SERVICE_CONTENT_REQUIRE_CAPABILITY")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if require_capability {
        println!(
            "[HTTP] SERVICE_CONTENT_REQUIRE_CAPABILITY on: headerless requests allowed from loopback only"
        );
    }
    let state = Arc::new(HttpState {
        graph: store,
        doorman_endpoint,
        ontology_dir,
        corpus_dir,
        graph_dir,
        pairing_store: Mutex::new(pairing_store),
        nonce_cache: NonceCache::new(),
        pairing_key,
        capability_audit,
        write_governance,
        require_capability,
    });

    let app = router(state);

    let listener = match tokio::net::TcpListener::bind(&bind_addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[HTTP] Failed to bind {}: {}", bind_addr, e);
            return;
        }
    };
    println!("[HTTP] Graph API listening on {}", bind_addr);
    if let Err(e) = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    {
        eprintln!("[HTTP] Server error: {}", e);
    }
    println!("[HTTP] Graph API server shut down cleanly.");
}

/// Awaits SIGTERM, then returns — drives `run_server`'s own axum graceful
/// shutdown independently of `lib.rs::run()`'s separate SIGTERM listener for
/// the watcher loop. Multiple independent listeners for the same signal in
/// one process is a safe, supported pattern (see the "SIGTERM handling"
/// comment in `lib.rs::run()` for why); each half of the bundle drains its
/// own in-flight work on the same signal without needing to coordinate a
/// shared flag across the thread boundary.
async fn shutdown_signal() {
    match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
        Ok(mut sigterm) => {
            sigterm.recv().await;
            println!("[HTTP] SIGTERM received — draining in-flight requests, no new connections accepted...");
        }
        Err(e) => {
            eprintln!("[HTTP] failed to install SIGTERM handler: {e} — graceful shutdown unavailable, server will only stop on abrupt termination");
            std::future::pending::<()>().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_think_blocks_removes_reasoning_and_keeps_answer() {
        assert_eq!(strip_think_blocks("<think>a</think>Answer."), "Answer.");
        assert_eq!(strip_think_blocks("A<think>x</think>B<think>y</think>C"), "ABC");
        assert_eq!(strip_think_blocks("no blocks"), "no blocks");
        // An unclosed block drops the rest rather than leaking reasoning.
        assert_eq!(strip_think_blocks("Hi <think>never closed"), "Hi");
    }

    #[test]
    fn truncate_at_char_boundary_never_splits_a_multibyte_char() {
        // Build a string where a 3-byte UTF-8 character (e.g. an accented/CJK-range
        // character) straddles the old fixed byte offset — this is exactly the shape
        // that panicked the previous `&user_message[..7900]` fixed-offset slice.
        let prefix = "x".repeat(7899); // 7899 ASCII bytes, so the multi-byte char starts at 7899
        let s = format!("{prefix}日本語のテキスト以降"); // multi-byte chars starting right at the old cut point
                                                         // Old code: &s[..7900] would panic here (byte 7900 lands mid-character).
        let truncated = truncate_at_char_boundary(&s, 7900);
        assert!(s.is_char_boundary(0)); // sanity
        assert!(truncated.len() <= 7900);
        // Must not panic, and must be valid UTF-8 (guaranteed by the type system once we
        // successfully construct a &str, but assert the boundary landed at or before 7899
        // since byte 7899 is the last ASCII byte and the multi-byte run starts there).
        assert_eq!(truncated, &prefix[..7899]);
    }

    #[test]
    fn truncate_at_char_boundary_no_op_when_under_limit() {
        let s = "short string";
        assert_eq!(truncate_at_char_boundary(s, 8000), s);
    }

    #[test]
    fn truncate_at_char_boundary_exact_ascii_boundary() {
        let s = "a".repeat(100);
        assert_eq!(truncate_at_char_boundary(&s, 50).len(), 50);
    }

    fn entity(name: &str, confidence: f64, source_doc: Option<&str>) -> GraphEntity {
        GraphEntity {
            entity_name: name.to_string(),
            classification: "Company".to_string(),
            role_vector: None,
            location_vector: None,
            contact_vector: None,
            module_id: "test".to_string(),
            confidence,
            source_doc: source_doc.map(str::to_string),
        }
    }

    #[test]
    fn format_entity_block_empty_when_no_entities() {
        assert_eq!(
            format_entity_block(&[], &[]),
            "(no entities found in graph)"
        );
    }

    #[test]
    fn format_entity_block_renders_confidence_and_source_doc() {
        let entities = vec![entity("Woodfine Capital Projects", 0.9, Some("DOC_1"))];
        let block = format_entity_block(&entities, &[]);
        assert!(block.contains("Woodfine Capital Projects"));
        assert!(block.contains("confidence: 0.90"));
        assert!(block.contains("source: DOC_1"));
    }

    #[test]
    fn format_entity_block_filters_low_confidence_entities() {
        // Below MIN_RENDER_CONFIDENCE (0.5) — must not appear in rendered output at all,
        // not just be down-weighted. The one high-confidence entity still renders.
        let entities = vec![
            entity("Untrusted Noise Entity", 0.2, None),
            entity("PointSav Digital Systems", 0.9, None),
        ];
        let block = format_entity_block(&entities, &[]);
        assert!(!block.contains("Untrusted Noise Entity"));
        assert!(block.contains("PointSav Digital Systems"));
    }

    #[test]
    fn format_entity_block_all_entities_below_threshold_returns_empty_message() {
        let entities = vec![entity("Untrusted", 0.1, None)];
        assert_eq!(
            format_entity_block(&entities, &[]),
            "(no entities found in graph)"
        );
    }

    #[test]
    fn format_entity_block_renders_relationships_section() {
        let entities = vec![
            entity("PointSav Digital Systems", 0.9, None),
            entity("Woodfine Capital Projects", 0.9, None),
        ];
        let edges = vec![RelatedToEdge {
            src_entity_name: "PointSav Digital Systems".to_string(),
            tgt_entity_name: "Woodfine Capital Projects".to_string(),
            relation_type: "subsidiary_of".to_string(),
        }];
        let block = format_entity_block(&entities, &edges);
        assert!(block.contains("## Relationships"));
        assert!(block
            .contains("PointSav Digital Systems --[subsidiary_of]--> Woodfine Capital Projects"));
    }

    #[test]
    fn format_entity_block_no_relationships_section_when_no_edges() {
        let entities = vec![entity("Solo Entity", 0.9, None)];
        assert!(!format_entity_block(&entities, &[]).contains("## Relationships"));
    }

    // ── scope_permits_request (capability_gate scope-vs-target policy) ────────

    #[test]
    fn scope_permits_exact_match_on_read() {
        let scope = vec!["project-totebox".to_string()];
        assert!(
            scope_permits_request(&scope, "USER", Some("project-totebox"), false, None).is_ok()
        );
    }

    #[test]
    fn scope_permits_rejects_mismatched_target() {
        let scope = vec!["project-totebox".to_string()];
        let err = scope_permits_request(&scope, "USER", Some("project-editorial"), false, None)
            .expect_err("mismatched target must be rejected");
        assert!(err.contains("does not cover target archive"));
    }

    #[test]
    fn scope_permits_rejects_undeterminable_target() {
        let scope = vec!["project-totebox".to_string()];
        let err = scope_permits_request(&scope, "USER", None, false, None)
            .expect_err("missing target module_id must be rejected");
        assert!(err.contains("could not determine target archive"));
    }

    #[test]
    fn scope_permits_wildcard_admin_on_read() {
        let scope = vec!["*".to_string()];
        assert!(scope_permits_request(&scope, "ADMIN", Some("any-archive"), false, None).is_ok());
    }

    #[test]
    fn scope_permits_wildcard_rejects_non_admin() {
        let scope = vec!["*".to_string()];
        let err = scope_permits_request(&scope, "USER", Some("any-archive"), false, None)
            .expect_err("wildcard scope must require ADMIN role");
        assert!(err.contains("requires an ADMIN-role capability"));
    }

    #[test]
    fn scope_permits_wildcard_admin_rejected_on_mutate() {
        // The operator override is read-only even for the operator — a wildcard
        // scope must never be honored on the mutate route, regardless of role.
        let scope = vec!["*".to_string()];
        let err = scope_permits_request(&scope, "ADMIN", Some("any-archive"), true, None)
            .expect_err("wildcard scope must never be honored on mutate");
        assert!(err.contains("read-only"));
    }

    #[test]
    fn scope_permits_exact_match_scope_still_enforced_on_mutate() {
        // Non-wildcard scopes are unaffected by the mutate-route restriction —
        // only the wildcard override is read-only.
        let scope = vec!["project-totebox".to_string()];
        assert!(scope_permits_request(&scope, "USER", Some("project-totebox"), true, None).is_ok());
    }

    // ── scope_permits_request — grant-vs-forward (Session 4) ──────────────────

    #[test]
    fn scope_permits_direct_grant_unaffected_by_forward_check() {
        // forwarded_for: None is the default (existing) behavior — untouched.
        let scope = vec!["project-totebox".to_string()];
        assert!(
            scope_permits_request(&scope, "USER", Some("project-totebox"), false, None).is_ok()
        );
    }

    #[test]
    fn scope_permits_forward_claim_from_admin_permitted() {
        let scope = vec!["project-totebox".to_string()];
        assert!(scope_permits_request(
            &scope,
            "ADMIN",
            Some("project-totebox"),
            false,
            Some("project-orchestration"),
        )
        .is_ok());
    }

    #[test]
    fn scope_permits_forward_claim_from_non_admin_rejected() {
        // An ordinary paired peer must not be able to unilaterally claim to
        // forward on behalf of a third instance — the whole point of
        // distinguishing grant-vs-forward.
        let scope = vec!["project-totebox".to_string()];
        let err = scope_permits_request(
            &scope,
            "USER",
            Some("project-totebox"),
            false,
            Some("project-orchestration"),
        )
        .expect_err("non-ADMIN forward claim must be rejected");
        assert!(err.contains("only ADMIN-scoped peers may forward"));
    }

    #[test]
    fn scope_permits_forward_check_runs_before_scope_check() {
        // A non-ADMIN forward claim is rejected on the forward check even when
        // archive_scope would otherwise have permitted the request — forward
        // authorization is a precondition, not an alternative path.
        let scope = vec!["project-totebox".to_string()];
        let err = scope_permits_request(
            &scope,
            "USER",
            Some("project-totebox"), // scope-vs-target WOULD pass
            false,
            Some("project-orchestration"), // but forward-vs-grant must fail first
        )
        .expect_err("forward check must run first");
        assert!(err.contains("ADMIN"));
    }
}
