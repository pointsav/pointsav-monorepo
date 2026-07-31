// SPDX-License-Identifier: LicenseRef-PointSav-ARR
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//
// This file is proprietary material of Woodfine Capital Projects Inc.
// See the LICENSE file in this repository for the full terms.
// Unauthorized use, reproduction, or distribution is prohibited.

//! app-orchestration-graph — Cross-archive DataGraph federation gateway
//!
//! Listens on port 9181 (env: ORCHESTRATION_GRAPH_BIND, default 127.0.0.1:9181).
//! Fans out GET /v1/graph/context queries to all Totebox archives listed in
//! ORCHESTRATION_GRAPH_TARGETS, deduplicates entities by normalized name, and
//! returns the merged result.
//!
//! Activation: not deployed yet — scaffold only. Activated by Command Session
//! when the fleet has ≥2 Totebox archives with DataGraph endpoints.
//!
//! **Target model** (reworked `BRIEF-datagraph-tenant-isolation.md` Session 4):
//! `ORCHESTRATION_GRAPH_TARGETS` is now a comma-separated list of
//! `archive_name|endpoint|module_id` triples (previously a flat list of bare
//! URLs) — e.g. `project-editorial|http://127.0.0.1:9081|pointsav`. Carrying
//! the archive name and module_id explicitly (not just a URL) is what makes
//! the eventual real per-archive-instance split a config change, not a
//! rework, and is required for capability-forwarding below (a signed
//! capability's `archive_scope` names a `module_id`, not a URL).
//!
//! **Capability-forwarding** (also Session 4): each fan-out call to a target
//! now carries a signed `X-Foundry-Capability` header (see `capability.rs`)
//! — this instance's own Ed25519 identity, paired with each target via
//! `POST /v1/pair` at startup, `archive_scope` set to that target's own
//! `module_id`. Ships as a **direct grant only** (not a "forward" claim) —
//! this instance doesn't yet authenticate its own inbound callers, so it has
//! no real third-party identity to relay on behalf of; see `capability.rs`'s
//! doc comment for the full scope reasoning.
//!
//! Routes:
//!   GET /healthz                    → liveness
//!   GET /v1/graph/context?q=&module_id= → federated entity query
//!   GET /v1/health                  → target list + per-archive status

mod capability;

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use capability::Identity;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// ── types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphEntity {
    pub entity_name: String,
    pub classification: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_vector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_vector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_vector: Option<String>,
    pub module_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_doc: Option<String>,
    /// Which archive this entity was returned from (injected by federation layer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_source: Option<String>,
}

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

#[derive(Debug, Serialize)]
pub struct FederatedContextResponse {
    pub entities: Vec<GraphEntity>,
    pub warnings: Vec<String>,
    pub archives_queried: usize,
    pub archives_responding: usize,
}

#[derive(Debug, Serialize)]
pub struct TargetStatus {
    pub url: String,
    pub reachable: bool,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub targets: Vec<TargetStatus>,
}

/// A federation target: `{archive_name, endpoint, module_id}`, not a bare
/// URL — see module doc comment for why this tuple shape matters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetSpec {
    pub archive_name: String,
    pub endpoint: String,
    pub module_id: String,
}

/// Parse `ORCHESTRATION_GRAPH_TARGETS`: comma-separated
/// `archive_name|endpoint|module_id` triples. Malformed entries (wrong field
/// count) are skipped with a warning printed to stderr, not silently dropped
/// or fatal — one bad entry must not take down the whole target list.
pub fn parse_targets(raw: &str) -> Vec<TargetSpec> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|entry| {
            let parts: Vec<&str> = entry.split('|').map(str::trim).collect();
            match parts.as_slice() {
                [archive_name, endpoint, module_id]
                    if !archive_name.is_empty() && !endpoint.is_empty() && !module_id.is_empty() =>
                {
                    Some(TargetSpec {
                        archive_name: archive_name.to_string(),
                        endpoint: endpoint.to_string(),
                        module_id: module_id.to_string(),
                    })
                }
                _ => {
                    eprintln!(
                        "[orchestration-graph] WARNING: skipping malformed ORCHESTRATION_GRAPH_TARGETS \
                         entry {entry:?} — expected \"archive_name|endpoint|module_id\""
                    );
                    None
                }
            }
        })
        .collect()
}

// ── server state ─────────────────────────────────────────────────────────────

pub struct AppState {
    /// One reqwest::Client with a 30-second per-request timeout.
    /// A single client reuses connection pools across fan-out requests.
    client: Client,
    /// `{archive_name, endpoint, module_id}` tuples parsed from
    /// ORCHESTRATION_GRAPH_TARGETS env var at startup.
    targets: Vec<TargetSpec>,
    /// This instance's own Ed25519 identity — signs the `X-Foundry-
    /// Capability` header attached to every fan-out call.
    identity: Identity,
}

// ── handlers ─────────────────────────────────────────────────────────────────

async fn healthz() -> &'static str {
    "ok"
}

/// GET /v1/graph/context — federated entity query.
/// Fans out to all targets, merges, deduplicates by normalized entity name.
///
/// Each target is queried using ITS OWN configured `module_id` (`TargetSpec.
/// module_id`), not the caller's request-level `params.module_id` — with
/// tenant-isolation enforced server-side at each target
/// (`BRIEF-datagraph-tenant-isolation.md` Session 2/3), a single caller-
/// supplied `module_id` string isn't meaningful across archives that each
/// scope their own tenant differently; `target.module_id` is also what the
/// signed capability's `archive_scope` asserts, so query and capability stay
/// consistent (a mismatch would be rejected by the target's own
/// `capability_gate`). `params.q`/`params.limit` still apply per target.
async fn graph_context(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ContextQuery>,
) -> Result<Json<FederatedContextResponse>, (StatusCode, String)> {
    let targets = &state.targets;
    if targets.is_empty() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "ORCHESTRATION_GRAPH_TARGETS is empty — no archives configured".to_string(),
        ));
    }

    // Fan-out: query all targets concurrently.
    let futs: Vec<_> = targets
        .iter()
        .map(|target| {
            let client = state.client.clone();
            let url = format!(
                "{}/v1/graph/context?q={}&module_id={}&limit={}",
                target.endpoint,
                urlencoding_basic(&params.q),
                urlencoding_basic(&target.module_id),
                params.limit,
            );
            let capability = state
                .identity
                .make_capability_header(&target.module_id, &capability::fresh_nonce());
            let base_url = target.endpoint.clone();
            async move {
                match client
                    .get(&url)
                    .header("X-Foundry-Capability", capability)
                    .send()
                    .await
                {
                    Ok(resp) if resp.status().is_success() => {
                        match resp.json::<Vec<GraphEntity>>().await {
                            Ok(mut entities) => {
                                for e in &mut entities {
                                    e.federation_source = Some(base_url.clone());
                                }
                                Ok(entities)
                            }
                            Err(e) => Err(format!("{}: parse error — {}", base_url, e)),
                        }
                    }
                    Ok(resp) => Err(format!("{}: HTTP {}", base_url, resp.status())),
                    Err(e) => Err(format!("{}: {}", base_url, e)),
                }
            }
        })
        .collect();

    let results = futures_join_all(futs).await;

    let mut merged: Vec<GraphEntity> = Vec::new();
    let mut seen_keys: HashSet<String> = HashSet::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut responding = 0usize;

    for result in results {
        match result {
            Ok(entities) => {
                responding += 1;
                for entity in entities {
                    let key = normalize_entity_key(&entity.entity_name);
                    if seen_keys.insert(key) {
                        merged.push(entity);
                    }
                }
            }
            Err(w) => warnings.push(w),
        }
    }

    // Sort by confidence descending; stable so order within equal confidence is archive order.
    merged.sort_by(|a, b| {
        b.confidence
            .unwrap_or(0.0)
            .partial_cmp(&a.confidence.unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(Json(FederatedContextResponse {
        entities: merged.into_iter().take(params.limit).collect(),
        warnings,
        archives_queried: targets.len(),
        archives_responding: responding,
    }))
}

/// GET /v1/health — list target archives and their reachability.
async fn health_targets(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let futs: Vec<_> = state
        .targets
        .iter()
        .map(|target| {
            let client = state.client.clone();
            let url = format!("{}/healthz", target.endpoint);
            let base_url = target.endpoint.clone();
            async move {
                let reachable = client
                    .get(&url)
                    .send()
                    .await
                    .map(|r| r.status().is_success())
                    .unwrap_or(false);
                TargetStatus {
                    url: base_url,
                    reachable,
                }
            }
        })
        .collect();

    let statuses = futures_join_all(futs).await;
    let all_ok = statuses.iter().all(|s| s.reachable);
    Json(HealthResponse {
        status: if all_ok { "ok" } else { "degraded" },
        targets: statuses,
    })
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Minimal percent-encoding for query string values (encodes space and & only).
fn urlencoding_basic(s: &str) -> String {
    s.replace('%', "%25")
        .replace('&', "%26")
        .replace(' ', "%20")
        .replace('+', "%2B")
}

/// Normalize an entity name for deduplication: lowercase, strip punctuation.
fn normalize_entity_key(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Run a Vec of futures concurrently and collect results.
/// Equivalent to `futures::future::join_all` without the `futures` dependency.
async fn futures_join_all<T: Send + 'static>(
    futs: Vec<impl std::future::Future<Output = T> + Send + 'static>,
) -> Vec<T> {
    let handles: Vec<_> = futs.into_iter().map(|f| tokio::spawn(f)).collect();
    let mut results = Vec::with_capacity(handles.len());
    for h in handles {
        // JoinError only on panic — treat as a warning-worthy absent result.
        if let Ok(v) = h.await {
            results.push(v);
        }
    }
    results
}

// ── main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let bind_addr =
        std::env::var("ORCHESTRATION_GRAPH_BIND").unwrap_or_else(|_| "127.0.0.1:9181".to_string());

    let targets = parse_targets(&std::env::var("ORCHESTRATION_GRAPH_TARGETS").unwrap_or_default());

    if targets.is_empty() {
        eprintln!(
            "[orchestration-graph] WARNING: ORCHESTRATION_GRAPH_TARGETS is not set (or fully \
             malformed). Set to comma-separated \"archive_name|endpoint|module_id\" triples \
             (e.g. project-editorial|http://127.0.0.1:9081|pointsav,project-bim|http://host2:9081|woodfine)."
        );
    } else {
        println!(
            "[orchestration-graph] Federating {} archive(s): {}",
            targets.len(),
            targets
                .iter()
                .map(|t| format!("{} ({})", t.archive_name, t.endpoint))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let identity = Identity::load_or_generate(&capability::default_seed_path())
        .unwrap_or_else(|e| panic!("[orchestration-graph] failed to load/generate identity: {e}"));
    println!(
        "[orchestration-graph] identity public key: {}",
        identity.verifying_key_b64
    );

    // 30-second per-request timeout: one unreachable archive must not stall all queries.
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("build reqwest client");

    // Pair with each target at startup — best-effort. A target that's
    // unreachable or already paired is logged and skipped, not fatal: this
    // crate is explicitly "not deployed yet — scaffold only" and targets may
    // not exist yet during development.
    for target in &targets {
        let pair_url = format!("{}/v1/pair", target.endpoint);
        let req = identity.build_pair_request(&target.module_id, &capability::fresh_nonce());
        match client.post(&pair_url).json(&req).send().await {
            Ok(resp) if resp.status().is_success() => {
                println!(
                    "[orchestration-graph] paired with {} ({})",
                    target.archive_name, target.endpoint
                );
            }
            Ok(resp) => {
                eprintln!(
                    "[orchestration-graph] WARNING: pairing with {} ({}) returned HTTP {} — \
                     fan-out to this target will fail capability_gate until paired",
                    target.archive_name,
                    target.endpoint,
                    resp.status()
                );
            }
            Err(e) => {
                eprintln!(
                    "[orchestration-graph] WARNING: could not reach {} ({}) to pair: {e} — \
                     fan-out to this target will fail capability_gate until paired",
                    target.archive_name, target.endpoint
                );
            }
        }
    }

    let state = Arc::new(AppState {
        client,
        targets,
        identity,
    });

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/graph/context", get(graph_context))
        .route("/v1/health", get(health_targets))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|e| panic!("[orchestration-graph] Failed to bind {}: {}", bind_addr, e));

    println!("[orchestration-graph] Listening on {}", bind_addr);
    axum::serve(listener, app)
        .await
        .expect("[orchestration-graph] Server error");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_targets_single_valid_triple() {
        let targets = parse_targets("project-editorial|http://127.0.0.1:9081|pointsav");
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].archive_name, "project-editorial");
        assert_eq!(targets[0].endpoint, "http://127.0.0.1:9081");
        assert_eq!(targets[0].module_id, "pointsav");
    }

    #[test]
    fn parse_targets_multiple_triples() {
        let targets = parse_targets(
            "project-editorial|http://host1:9081|pointsav,project-bim|http://host2:9081|woodfine",
        );
        assert_eq!(targets.len(), 2);
        assert_eq!(targets[1].archive_name, "project-bim");
        assert_eq!(targets[1].module_id, "woodfine");
    }

    #[test]
    fn parse_targets_trims_whitespace() {
        let targets = parse_targets(" project-editorial | http://127.0.0.1:9081 | pointsav ");
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].archive_name, "project-editorial");
        assert_eq!(targets[0].endpoint, "http://127.0.0.1:9081");
    }

    #[test]
    fn parse_targets_skips_malformed_entries_not_fatal() {
        // Old flat-URL-only format (no pipes) — malformed under the new
        // tuple shape, must be skipped, not panic or silently produce a
        // garbage TargetSpec.
        let targets = parse_targets(
            "http://old-flat-format:9081,project-editorial|http://host1:9081|pointsav",
        );
        assert_eq!(targets.len(), 1, "malformed entry skipped, valid one kept");
        assert_eq!(targets[0].archive_name, "project-editorial");
    }

    #[test]
    fn parse_targets_empty_string_returns_empty_vec() {
        assert!(parse_targets("").is_empty());
        assert!(parse_targets("   ").is_empty());
    }

    #[test]
    fn parse_targets_rejects_empty_field_within_triple() {
        // Right shape (3 pipe-separated fields) but one field is empty —
        // must still be rejected, not accepted with a blank module_id.
        let targets = parse_targets("project-editorial||pointsav");
        assert!(targets.is_empty());
    }
}
