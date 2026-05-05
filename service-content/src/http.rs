use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use crate::graph::{GraphEntity, GraphStore};
use crate::config_http::config_routes;

// ── shared server state ───────────────────────────────────────────────────────

pub struct HttpState {
    pub graph: Arc<dyn GraphStore>,
    pub doorman_endpoint: String,
    pub ontology_dir: String,
}

// ── request / response types ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ContextQuery {
    pub q: String,
    pub module_id: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    20
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
}

// ── handlers ──────────────────────────────────────────────────────────────────

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn graph_context(
    State(state): State<Arc<HttpState>>,
    Query(params): Query<ContextQuery>,
) -> Result<Json<Vec<GraphEntity>>, (StatusCode, String)> {
    state
        .graph
        .query_context(&params.module_id, &params.q, params.limit)
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn graph_mutate(
    State(state): State<Arc<HttpState>>,
    Json(body): Json<MutateRequest>,
) -> Result<Json<MutateResponse>, (StatusCode, String)> {
    state
        .graph
        .upsert_entities(&body.module_id, &body.entities)
        .map(|upserted| Json(MutateResponse { upserted }))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Tier C Drafting Pipeline.
///
/// Queries the LadybugDB graph for entities matching the request's module and
/// query_hint, packages them into a ≤2 000-token structured prompt, then
/// proxies the request to Claude 3.5 Sonnet via Doorman /v1/audit/proxy.
///
/// Pre-D4: the Doorman will return 503 (provider unconfigured). The handler
/// surfaces the upstream status and message rather than masking it.
async fn draft_generate(
    State(state): State<Arc<HttpState>>,
    Json(body): Json<DraftRequest>,
) -> Result<Json<DraftResponse>, (StatusCode, String)> {
    // 1. Fetch graph entities.
    let entities = state
        .graph
        .query_context(&body.module_id, &body.query_hint, body.max_entities)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("graph query failed: {e}")))?;

    let entity_count = entities.len();

    // 2. Package entities into a structured prompt (≤2 000 tokens ≈ 8 000 chars).
    let entity_block = format_entity_block(&entities);

    let system_prompt = "You are a knowledge graph analyst for a real estate property \
        management archive. Given a list of extracted entities from the Totebox Archive, \
        produce a concise structured summary document. \
        Include: key people and their roles, active projects, notable locations, \
        and company relationships. Write in precise professional prose. \
        Maximum 600 words.";

    let user_message = format!(
        "Module: {module_id}\nQuery context: {hint}\n\n{entity_block}",
        module_id = body.module_id,
        hint = if body.query_hint.is_empty() { "(all entities)" } else { &body.query_hint },
        entity_block = entity_block
    );

    // Truncate to keep under 8 000 chars (~2 000 tokens).
    let user_message = if user_message.len() > 8000 {
        format!("{}\n\n[truncated — {entity_count} entities total]", &user_message[..7900])
    } else {
        user_message
    };

    // 3. Build the AuditProxyRequest JSON (matches slm-core wire format).
    let request_id = format!(
        "sc-draft-{}-{}",
        body.module_id,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos()
    );

    let proxy_body = serde_json::json!({
        "provider": "anthropic",
        "model": "anthropic:claude-haiku-4-5-20251001",
        "purpose": body.purpose,
        "caller_request_id": request_id,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_message}
        ],
        "max_tokens": 1024,
        "temperature": 0.3
    });

    // 4. POST to Doorman /v1/audit/proxy.
    let url = format!("{}/v1/audit/proxy", state.doorman_endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .header("X-Foundry-Module-ID", &body.module_id)
        .header("X-Foundry-Request-ID", &request_id)
        .json(&proxy_body)
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
            format!("audit_proxy {}: {}", status.as_u16(), msg),
        ));
    }

    // 5. Extract the draft text from the completion.
    let audit_id = resp_json
        .get("audit_id")
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let draft = resp_json
        .pointer("/choices/0/message/content")
        .or_else(|| resp_json.get("content"))
        .and_then(|v| v.as_str())
        .unwrap_or("(no content returned)")
        .to_string();

    Ok(Json(DraftResponse {
        draft,
        entity_count,
        audit_id,
    }))
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn format_entity_block(entities: &[GraphEntity]) -> String {
    if entities.is_empty() {
        return "(no entities found in graph)".to_string();
    }
    let mut out = String::from("## Extracted Entities\n\n");
    for e in entities {
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
        out.push('\n');
    }
    out
}

// ── server entrypoint ─────────────────────────────────────────────────────────

pub async fn run_server(store: Arc<dyn GraphStore>, bind_addr: String, doorman_endpoint: String, ontology_dir: String) {
    let state = Arc::new(HttpState {
        graph: store,
        doorman_endpoint,
        ontology_dir,
    });

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/graph/context", get(graph_context))
        .route("/v1/graph/mutate", post(graph_mutate))
        .route("/v1/draft/generate", post(draft_generate))
        .merge(config_routes())
        .with_state(state);

    let listener = match tokio::net::TcpListener::bind(&bind_addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[HTTP] Failed to bind {}: {}", bind_addr, e);
            return;
        }
    };
    println!("[HTTP] Graph API listening on {}", bind_addr);
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("[HTTP] Server error: {}", e);
    }
}
