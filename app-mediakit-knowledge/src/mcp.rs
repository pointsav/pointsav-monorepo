// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Claim-record MCP surface (Phase 3.6 of `KNOWLEDGE-PLATFORM-PLAN.md`).
//!
//! Exposes `query_claims(topic, asof?)` over stdio. Run as its own CLI
//! subcommand (`app-mediakit-knowledge mcp --knowledge-toml …`), separate
//! from the HTTP `serve` process — MCP-over-stdio needs the process's
//! stdin/stdout dedicated to the protocol, which an HTTP server's own
//! logging/console use does not leave free.
//!
//! **Provisional, not yet reconciled with `service-slm`'s `slm-mcp-server`**
//! (`KNOWLEDGE-PLATFORM-PLAN.md` Decision 3) — that reconciliation is
//! cross-project and out of this archive's scope for this run; this server
//! stands alone until it happens. Tracked in `BRIEF-knowledge-ng-rewrite.md`.

use std::borrow::Cow;

use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, ContentBlock, ErrorCode, ErrorData as McpError, Implementation,
    InitializeResult, ServerCapabilities,
};
use rmcp::{tool, tool_handler, tool_router, ServerHandler};

use crate::claims_store::ClaimStore;

/// Parameters for `query_claims`. Field docs become the tool's parameter
/// descriptions in its generated schema (schemars picks up doc comments).
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct QueryClaimsParams {
    /// Topic slug to query claims for (the wiki article's slug).
    pub topic: String,
    /// Optional point-in-time filter, ISO date (`YYYY-MM-DD`, `YYYY-MM`, or
    /// `YYYY`). When present, only claims whose `valid_at` is at or before
    /// this date (or have no `valid_at` at all — timeless claims always
    /// match) are returned.
    #[serde(default)]
    pub asof: Option<String>,
}

/// Parameters for `validate_editorial_standards`.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ValidateEditorialParams {
    /// Markdown body text to check against the editorial-standards ruleset.
    pub body_md: String,
}

fn internal_error(msg: impl Into<String>) -> McpError {
    McpError {
        code: ErrorCode(-32603),
        message: Cow::from(msg.into()),
        data: None,
    }
}

/// The claim-query MCP server. Holds a `ClaimStore` handle (cheap to clone —
/// wraps an `Arc`), so it can share the same redb file the HTTP process
/// writes to (opened read-mostly here; writes only ever come from the
/// reindex path and the verification scheduler in the `serve` process).
#[derive(Clone)]
pub struct ClaimsMcpServer {
    store: ClaimStore,
    // Read only by the #[tool_router]/#[tool_handler]-generated trait impls,
    // not by any hand-written code — dead_code's static analysis doesn't see
    // that usage, hence the allow.
    #[allow(dead_code)]
    tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
}

#[tool_router]
impl ClaimsMcpServer {
    pub fn new(store: ClaimStore) -> Self {
        Self {
            store,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Query claims for a topic slug, optionally filtered to a point in time via each claim's valid_at field. Returns the matching claims as a JSON array."
    )]
    async fn query_claims(
        &self,
        Parameters(params): Parameters<QueryClaimsParams>,
    ) -> Result<CallToolResult, McpError> {
        let claims = self
            .store
            .claims_for_topic(&params.topic)
            .map_err(|e| internal_error(format!("claim store read failed: {e}")))?;
        let filtered = filter_by_asof(claims, params.asof.as_deref());
        let json = serde_json::to_string_pretty(&filtered)
            .map_err(|e| internal_error(format!("serialization failed: {e}")))?;
        Ok(CallToolResult::success(vec![ContentBlock::text(json)]))
    }

    #[tool(
        description = "Check markdown body text against the provisional editorial-standards ruleset (Bloomberg-standard language, BCSC forward-looking-claim posture). Returns a JSON array of flagged lines. Provisional starter ruleset — not project-editorial's eventual real one."
    )]
    async fn validate_editorial_standards(
        &self,
        Parameters(params): Parameters<ValidateEditorialParams>,
    ) -> Result<CallToolResult, McpError> {
        let findings = crate::editorial::validate_editorial_standards(&params.body_md);
        let json = serde_json::to_string_pretty(&findings)
            .map_err(|e| internal_error(format!("serialization failed: {e}")))?;
        Ok(CallToolResult::success(vec![ContentBlock::text(json)]))
    }
}

/// Keep only claims whose `valid_at` is at or before `asof` (timeless claims
/// — no `valid_at` — always match). `asof: None` returns every claim
/// unfiltered. Plain string comparison is correct because every `valid_at`
/// form (`YYYY`, `YYYY-MM`, `YYYY-MM-DD`) sorts lexicographically the same
/// as chronologically.
fn filter_by_asof(
    claims: Vec<crate::content::Claim>,
    asof: Option<&str>,
) -> Vec<crate::content::Claim> {
    match asof {
        Some(asof) => claims
            .into_iter()
            .filter(|c| c.valid_at.as_deref().map(|v| v <= asof).unwrap_or(true))
            .collect(),
        None => claims,
    }
}

#[tool_handler]
impl ServerHandler for ClaimsMcpServer {
    fn get_info(&self) -> InitializeResult {
        InitializeResult::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "app-mediakit-knowledge claim-layer + editorial-linter MCP surface \
                 (provisional — see KNOWLEDGE-PLATFORM-PLAN.md Decisions 3 and 8 for \
                 the pending service-slm / project-editorial reconciliations). Tools: \
                 query_claims(topic, asof?), validate_editorial_standards(body_md).",
            )
    }
}

/// Run the MCP server on stdio until the client disconnects.
pub async fn run_stdio(store: ClaimStore) -> anyhow::Result<()> {
    use rmcp::ServiceExt;
    let server = ClaimsMcpServer::new(store);
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{Claim, Confidence};

    fn sample(id: &str, topic: &str, valid_at: Option<&str>) -> Claim {
        Claim {
            id: id.to_string(),
            topic_slug: topic.to_string(),
            cites: vec![],
            confidence: Confidence::Established,
            valid_at: valid_at.map(String::from),
            depends_on: vec![],
            text: "text".to_string(),
            content_hash: "hash".to_string(),
            published_at: None,
            revision_sha: None,
        }
    }

    #[tokio::test]
    async fn query_claims_tool_call_succeeds_end_to_end() {
        // Exercises the real #[tool] entry point (MCP plumbing included);
        // the filtering semantics themselves are covered precisely by the
        // plain-function tests below, which don't depend on CallToolResult's
        // internal content representation.
        let dir = tempfile::tempdir().unwrap();
        let store = ClaimStore::open(&dir.path().join("claims.redb")).unwrap();
        store
            .upsert_topic_claims(&[sample("a", "topic-x", None)])
            .unwrap();
        let server = ClaimsMcpServer::new(store);
        let result = server
            .query_claims(Parameters(QueryClaimsParams {
                topic: "topic-x".to_string(),
                asof: Some("2026".to_string()),
            }))
            .await
            .unwrap();
        assert!(!result.content.is_empty());
    }

    #[test]
    fn filter_by_asof_keeps_timeless_and_past_drops_future() {
        let claims = vec![
            sample("timeless", "t", None),
            sample("early", "t", Some("2020")),
            sample("future", "t", Some("2099")),
        ];
        let filtered = filter_by_asof(claims, Some("2026"));
        let ids: Vec<_> = filtered.iter().map(|c| c.id.as_str()).collect();
        assert!(ids.contains(&"timeless"));
        assert!(ids.contains(&"early"));
        assert!(!ids.contains(&"future"));
    }

    #[test]
    fn filter_by_asof_none_returns_everything() {
        let claims = vec![
            sample("a", "t", Some("2020")),
            sample("b", "t", Some("2099")),
        ];
        assert_eq!(filter_by_asof(claims, None).len(), 2);
    }
}
