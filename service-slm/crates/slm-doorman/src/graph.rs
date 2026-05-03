// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Graph context client — queries `service-content`'s Ring 2 knowledge graph
//! to inject per-tenant entity context ahead of Doorman inference calls.
//!
//! The client is intentionally fault-tolerant: a missing or unavailable
//! `service-content` instance is logged as a warning but never causes an
//! inference call to fail. The Doorman proceeds without graph context rather
//! than surfacing an error to the caller.
//!
//! Architectural background: `service-slm/ARCHITECTURE.md` Ring 2 (Knowledge &
//! Processing). Wire protocol: `GET /v1/graph/context?q=<query>&module_id=<mid>&limit=<n>`.

use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tracing::warn;

/// One entity row returned by the `service-content` graph API.
#[derive(Debug, Deserialize)]
struct GraphEntityRow {
    entity_name: String,
    classification: String,
    role_vector: Option<String>,
    location_vector: Option<String>,
    contact_vector: Option<String>,
}

/// HTTP client for `service-content`'s `GET /v1/graph/context` endpoint.
///
/// Construct once at Doorman startup and share as `Option<GraphContextClient>`
/// in `DoormanConfig`. Thread-safe: `reqwest::Client` is `Clone + Send + Sync`.
pub struct GraphContextClient {
    /// Base URL of the `service-content` HTTP server, e.g.
    /// `http://127.0.0.1:9081`. No trailing slash.
    endpoint: String,
    http: Client,
}

impl GraphContextClient {
    /// Construct a client targeting the given `endpoint`.
    ///
    /// The underlying `reqwest::Client` is built with a 5-second timeout so
    /// a slow or unresponsive `service-content` instance never stalls an
    /// inference call for more than 5 seconds.
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            http: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .expect("failed to build graph HTTP client"),
        }
    }

    /// Query `service-content` for entities matching `query` under `module_id`.
    ///
    /// Returns a formatted multi-line context string when entities are found,
    /// or `None` when:
    /// - `service-content` is unavailable (non-fatal — logged at WARN, returns `None`)
    /// - the endpoint returns a non-2xx status
    /// - the response body cannot be parsed as a JSON array
    /// - no entities match the query
    ///
    /// `limit` is capped at 10 to bound context injection size.
    pub async fn fetch_context(&self, module_id: &str, query: &str, limit: usize) -> Option<String> {
        let limit = limit.min(10);
        let url = format!("{}/v1/graph/context", self.endpoint);

        let resp = self
            .http
            .get(&url)
            .query(&[
                ("q", query),
                ("module_id", module_id),
                ("limit", &limit.to_string()),
            ])
            .send()
            .await;

        match resp {
            Err(e) => {
                warn!(
                    target: "slm_doorman::graph",
                    error = %e,
                    "service-content graph unavailable"
                );
                None
            }
            Ok(r) if !r.status().is_success() => {
                warn!(
                    target: "slm_doorman::graph",
                    status = %r.status(),
                    "service-content graph returned non-2xx status"
                );
                None
            }
            Ok(r) => {
                let entities: Vec<GraphEntityRow> = match r.json().await {
                    Ok(v) => v,
                    Err(e) => {
                        warn!(
                            target: "slm_doorman::graph",
                            error = %e,
                            "graph response parse error"
                        );
                        return None;
                    }
                };
                if entities.is_empty() {
                    return None;
                }
                let ctx = entities
                    .iter()
                    .map(|e| {
                        let mut parts = vec![format!("{} ({})", e.entity_name, e.classification)];
                        if let Some(r) = &e.role_vector {
                            parts.push(format!("role: {}", r));
                        }
                        if let Some(l) = &e.location_vector {
                            parts.push(format!("location: {}", l));
                        }
                        if let Some(c) = &e.contact_vector {
                            parts.push(format!("contact: {}", c));
                        }
                        parts.join("; ")
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                Some(ctx)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// When service-content returns one entity with all vector fields populated,
    /// `fetch_context` must return a `Some(String)` that contains all expected
    /// fields formatted correctly.
    #[tokio::test]
    async fn fetch_context_returns_formatted_string_when_entities_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/graph/context"))
            .and(query_param("q", "john"))
            .and(query_param("module_id", "woodfine"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "entity_name": "John Smith",
                    "classification": "Person",
                    "role_vector": "property manager",
                    "location_vector": null,
                    "contact_vector": "john@example.com",
                    "module_id": "woodfine",
                    "confidence": 0.95
                }
            ])))
            .mount(&server)
            .await;

        let client = GraphContextClient::new(server.uri());
        let result = client.fetch_context("woodfine", "john", 5).await;
        assert!(result.is_some(), "expected Some context string, got None");
        let ctx = result.unwrap();
        assert!(
            ctx.contains("John Smith"),
            "context must contain entity name; got: {ctx}"
        );
        assert!(
            ctx.contains("Person"),
            "context must contain classification; got: {ctx}"
        );
        assert!(
            ctx.contains("property manager"),
            "context must contain role_vector; got: {ctx}"
        );
        assert!(
            ctx.contains("john@example.com"),
            "context must contain contact_vector; got: {ctx}"
        );
    }

    /// When service-content returns an empty array, `fetch_context` must return
    /// `None` (no entities matched — no context to inject).
    #[tokio::test]
    async fn fetch_context_returns_none_when_no_entities() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/graph/context"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let client = GraphContextClient::new(server.uri());
        let result = client.fetch_context("woodfine", "unknown", 5).await;
        assert!(
            result.is_none(),
            "empty entity list must produce None, got Some"
        );
    }

    /// When the `service-content` instance is unreachable, `fetch_context` must
    /// return `None` without panicking. The Doorman must be able to proceed
    /// without graph context.
    #[tokio::test]
    async fn fetch_context_returns_none_when_service_unavailable() {
        // Use a port nothing is listening on — connection will be refused.
        let client = GraphContextClient::new("http://127.0.0.1:19999".to_string());
        let result = client.fetch_context("woodfine", "test", 5).await;
        assert!(
            result.is_none(),
            "unavailable service must produce None (non-fatal); got Some"
        );
    }
}
