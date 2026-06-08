// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// slm-mcp-server — Foundry MCP server (Sprint 3)
//
// Exposes Foundry capabilities as MCP tools over stdio.
// Requires a running slm-doorman-server (default: http://127.0.0.1:9080).
//
// Usage:
//   slm-mcp-server --doorman http://127.0.0.1:9080
//
// Env:
//   SLM_DOORMAN_URL   doorman base URL  (default: http://127.0.0.1:9080)
//   SLM_MODULE_ID     module-id header  (default: mcp-foundry)
//
// .mcp.json example:
//   { "mcpServers": { "foundry": {
//       "command": "/usr/local/bin/slm-mcp-server",
//       "args": ["--doorman", "http://127.0.0.1:9080"],
//       "env": { "SLM_MODULE_ID": "mcp-foundry" },
//       "type": "stdio"
//   } } }

use rmcp::{ServerHandler, ServiceExt, tool, tool_handler, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Input types ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct QueryDatagraphInput {
    /// Free-text or keyword query forwarded to the DataGraph
    q: String,
    /// Maximum number of entity results to return (default 10)
    limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct MutateDatagraphInput {
    /// Mutation payload — must match service-content mutate schema
    mutation: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct GetEntityContextInput {
    /// Entity name or identifier to look up
    entity: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct SubmitExtractionInput {
    /// Prose document content to extract entities from (ADR-07: no structured data)
    text: String,
    /// JSON Schema constraining the output entity array
    schema: serde_json::Value,
    /// Module-id override (defaults to SLM_MODULE_ID env var)
    module_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct AskLocalInput {
    /// Prompt to send to the local OLMo model via the Doorman.
    prompt: String,
    /// Maximum tokens to generate (default 300; hard cap 400 — ~108 s at 3.7 tok/s).
    max_tokens: Option<u32>,
}

// ── Server struct ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct FoundryServer {
    client: reqwest::Client,
    doorman_url: String,
    module_id: String,
}

impl FoundryServer {
    fn new(doorman_url: String, module_id: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("reqwest client init");
        Self { client, doorman_url, module_id }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.doorman_url, path)
    }
}

// ── Tool implementations ──────────────────────────────────────────────────────

#[tool_router]
impl FoundryServer {
    /// Query the Foundry DataGraph for entity context.
    ///
    /// Forwards to `POST /v1/graph/query` on the Doorman.
    /// Returns a JSON array of entity context objects.
    #[tool(description = "Query the Foundry DataGraph for entity context. Returns matching entities and their attributes.")]
    async fn query_datagraph(
        &self,
        rmcp::handler::server::wrapper::Parameters(p): rmcp::handler::server::wrapper::Parameters<QueryDatagraphInput>,
    ) -> String {
        let limit = p.limit.unwrap_or(10);
        let body = serde_json::json!({ "q": p.q, "limit": limit });
        match self
            .client
            .post(self.url("/v1/graph/query"))
            .header("X-Foundry-Module-ID", &self.module_id)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<serde_json::Value>().await {
                Ok(v) => serde_json::to_string_pretty(&v).unwrap_or_else(|_| "{}".into()),
                Err(e) => format!("[ERROR] failed to parse response: {e}"),
            },
            Err(e) => format!("[ERROR] graph query failed: {e}"),
        }
    }

    /// Mutate the Foundry DataGraph (create / update / delete entities).
    ///
    /// Forwards `mutation` payload to `POST /v1/graph/mutate` on the Doorman.
    #[tool(description = "Mutate the Foundry DataGraph. Provide a mutation object matching the service-content mutate schema.")]
    async fn mutate_datagraph(
        &self,
        rmcp::handler::server::wrapper::Parameters(p): rmcp::handler::server::wrapper::Parameters<MutateDatagraphInput>,
    ) -> String {
        match self
            .client
            .post(self.url("/v1/graph/mutate"))
            .header("X-Foundry-Module-ID", &self.module_id)
            .json(&p.mutation)
            .send()
            .await
        {
            Ok(resp) => {
                let status = resp.status().as_u16();
                match resp.json::<serde_json::Value>().await {
                    Ok(v) => format!("HTTP {status}\n{}", serde_json::to_string_pretty(&v).unwrap_or_else(|_| "{}".into())),
                    Err(e) => format!("[ERROR] HTTP {status} — failed to parse response: {e}"),
                }
            }
            Err(e) => format!("[ERROR] graph mutate failed: {e}"),
        }
    }

    /// Fetch rich entity enrichment context by entity name.
    ///
    /// Convenience wrapper around query_datagraph scoped to a single entity.
    #[tool(description = "Fetch full entity enrichment context from the DataGraph by entity name or identifier.")]
    async fn get_entity_context(
        &self,
        rmcp::handler::server::wrapper::Parameters(p): rmcp::handler::server::wrapper::Parameters<GetEntityContextInput>,
    ) -> String {
        let body = serde_json::json!({ "q": p.entity, "limit": 5 });
        match self
            .client
            .post(self.url("/v1/graph/query"))
            .header("X-Foundry-Module-ID", &self.module_id)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<serde_json::Value>().await {
                Ok(v) => format!("Entity context for '{}':\n{}", p.entity, serde_json::to_string_pretty(&v).unwrap_or_else(|_| "[]".into())),
                Err(e) => format!("[ERROR] failed to parse entity context: {e}"),
            },
            Err(e) => format!("[ERROR] entity context fetch failed: {e}"),
        }
    }

    /// Get training corpus statistics and daily cost summary from Doorman.
    #[tool(description = "Retrieve Foundry corpus statistics and daily inference cost summary from the Doorman.")]
    async fn get_corpus_stats(&self) -> String {
        let health_fut = self.client.get(self.url("/healthz")).send();
        let cost_fut = self.client.get(self.url("/v1/cost/daily")).send();
        let (health_res, cost_res) = tokio::join!(health_fut, cost_fut);

        let health_str = match health_res {
            Ok(r) => r.text().await.unwrap_or_else(|_| "?".into()),
            Err(e) => format!("unreachable: {e}"),
        };
        let cost_str = match cost_res {
            Ok(r) => match r.json::<serde_json::Value>().await {
                Ok(v) => serde_json::to_string_pretty(&v).unwrap_or_else(|_| "{}".into()),
                Err(e) => format!("parse error: {e}"),
            },
            Err(e) => format!("unreachable: {e}"),
        };

        format!("Doorman health: {health_str}\n\nDaily cost:\n{cost_str}")
    }

    /// Submit a prose document to the Foundry entity extraction pipeline.
    ///
    /// Forwards to `POST /v1/extract` on the Doorman.
    /// Requires a JSON Schema that constrains the output entity array.
    /// Returns extracted entities or a deferred status if Yo-Yo is unavailable.
    #[tool(description = "Submit a prose document for entity extraction via the Foundry pipeline. Requires a JSON Schema for the output entity array.")]
    async fn submit_extraction(
        &self,
        rmcp::handler::server::wrapper::Parameters(p): rmcp::handler::server::wrapper::Parameters<SubmitExtractionInput>,
    ) -> String {
        let module_id = p.module_id.as_deref().unwrap_or(&self.module_id);
        let body = serde_json::json!({
            "text": p.text,
            "schema": p.schema,
            "module_id": module_id,
        });
        match self
            .client
            .post(self.url("/v1/extract"))
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => {
                let status = resp.status().as_u16();
                match resp.json::<serde_json::Value>().await {
                    Ok(v) => format!("HTTP {status}\n{}", serde_json::to_string_pretty(&v).unwrap_or_else(|_| "{}".into())),
                    Err(e) => format!("[ERROR] HTTP {status} — parse error: {e}"),
                }
            }
            Err(e) => format!("[ERROR] extraction request failed: {e}"),
        }
    }

    /// Check Doorman health: tier availability (A/B/C) and circuit breaker state.
    #[tool(description = "Check Doorman health including tier availability (A/B/C), readiness, and circuit breaker state.")]
    async fn doorman_health(&self) -> String {
        let healthz_fut = self.client.get(self.url("/healthz")).send();
        let readyz_fut = self.client.get(self.url("/readyz")).send();
        let contract_fut = self.client.get(self.url("/v1/contract")).send();
        let (healthz, readyz, contract) = tokio::join!(healthz_fut, readyz_fut, contract_fut);

        let hz = match healthz {
            Ok(r) => r.text().await.unwrap_or_else(|_| "?".into()),
            Err(e) => format!("unreachable: {e}"),
        };
        let rz = match readyz {
            Ok(r) => {
                let status = r.status().as_u16();
                let body = r.text().await.unwrap_or_else(|_| "?".into());
                format!("HTTP {status} — {body}")
            }
            Err(e) => format!("unreachable: {e}"),
        };
        let ct = match contract {
            Ok(r) => match r.json::<serde_json::Value>().await {
                Ok(v) => serde_json::to_string_pretty(&v).unwrap_or_else(|_| "{}".into()),
                Err(e) => format!("parse error: {e}"),
            },
            Err(e) => format!("unreachable: {e}"),
        };

        format!("/healthz: {hz}\n/readyz: {rz}\n\nContract:\n{ct}")
    }

    /// Submit a prompt to the local OLMo model and return its response.
    ///
    /// Calls `POST /v1/chat/completions` on the Doorman, which routes to Tier A
    /// (local OLMo). Data never leaves the VM (SYS-ADR-07 compliant).
    ///
    /// Known behaviour: graph context injection is currently broken (BRIEF §9c fault log).
    /// Use query_datagraph first and include relevant entity context in your prompt manually.
    /// Output requesting structured JSON is often wrapped in markdown fences — use
    /// submit_extraction with a JSON Schema for parseable structured output.
    #[tool(description = "Submit a prompt to the local OLMo 7B model via the Doorman. \
        Returns the model response plus tier, inference time, and cost. \
        IMPORTANT: graph context injection is currently broken — call query_datagraph first \
        and include entity context in your prompt manually. \
        Output may be wrapped in markdown fences; use submit_extraction with a schema \
        for parseable JSON output. No data leaves the VM (SYS-ADR-07 compliant).")]
    async fn ask_local(
        &self,
        rmcp::handler::server::wrapper::Parameters(p): rmcp::handler::server::wrapper::Parameters<AskLocalInput>,
    ) -> String {
        let max_tokens = p.max_tokens.unwrap_or(300).min(400);
        let body = serde_json::json!({
            "messages": [{"role": "user", "content": p.prompt}],
            "stream": false,
            "max_tokens": max_tokens,
        });
        match self
            .client
            .post(self.url("/v1/chat/completions"))
            .timeout(std::time::Duration::from_secs(180))
            .header("X-Foundry-Module-ID", &self.module_id)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => match resp.json::<serde_json::Value>().await {
                Ok(v) => {
                    let content = v["content"].as_str().unwrap_or("").to_string();
                    let tier = v["tier_used"].as_str().unwrap_or("?");
                    let model = v["model"].as_str().unwrap_or("?");
                    let ms = v["inference_ms"].as_u64().unwrap_or(0);
                    let cost = v["cost_usd"].as_f64().unwrap_or(0.0);
                    format!("{content}\n\n---\ntier={tier} model={model} inference={ms}ms cost=${cost:.6}")
                }
                Err(e) => format!("[ERROR] parse failed: {e}"),
            },
            Err(e) => format!("[ERROR] request failed: {e}"),
        }
    }
}

#[tool_handler(
    name = "slm-mcp-server",
    version = "0.1.0",
    instructions = "Foundry MCP server — DataGraph, corpus, and Doorman tools"
)]
impl ServerHandler for FoundryServer {}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    // Parse --doorman flag
    let args: Vec<String> = std::env::args().collect();
    let doorman_url = args
        .windows(2)
        .find(|w| w[0] == "--doorman")
        .map(|w| w[1].clone())
        .or_else(|| std::env::var("SLM_DOORMAN_URL").ok())
        .unwrap_or_else(|| "http://127.0.0.1:9080".to_string());

    let module_id = std::env::var("SLM_MODULE_ID").unwrap_or_else(|_| "mcp-foundry".to_string());

    tracing::info!(doorman_url = %doorman_url, module_id = %module_id, "slm-mcp-server starting");

    let server = FoundryServer::new(doorman_url, module_id);
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    server.serve(transport).await?.waiting().await?;

    Ok(())
}
