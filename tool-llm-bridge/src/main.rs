// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::collections::HashSet;

use tool_llm_bridge::{router, AppState, BridgeConfig};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let bind_addr =
        std::env::var("LLM_BRIDGE_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:9210".to_string());
    let access_token = std::env::var("LLM_BRIDGE_ACCESS_TOKEN")
        .expect("LLM_BRIDGE_ACCESS_TOKEN is required — no default local-access token");
    let provider_endpoint = std::env::var("LLM_BRIDGE_PROVIDER_ENDPOINT").unwrap_or_default();
    let provider_api_key = std::env::var("LLM_BRIDGE_PROVIDER_API_KEY").unwrap_or_default();
    let allowed_labels: HashSet<String> = std::env::var("LLM_BRIDGE_ALLOWED_LABELS")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if provider_endpoint.is_empty() || provider_api_key.is_empty() {
        tracing::warn!(
            target: "tool_llm_bridge",
            "LLM_BRIDGE_PROVIDER_ENDPOINT / LLM_BRIDGE_PROVIDER_API_KEY not set — \
             bridge will refuse every request with upstream_not_configured until \
             explicitly enabled by the operator"
        );
    }
    if allowed_labels.is_empty() {
        tracing::warn!(
            target: "tool_llm_bridge",
            "LLM_BRIDGE_ALLOWED_LABELS is empty — no label will ever be permitted \
             until the operator configures at least one"
        );
    }

    let config = BridgeConfig {
        access_token,
        provider_endpoint,
        provider_api_key,
        allowed_labels,
    };

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    tracing::info!(target: "tool_llm_bridge", %bind_addr, "tool-llm-bridge listening");
    axum::serve(listener, router(AppState::new(config))).await?;
    Ok(())
}
