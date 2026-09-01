// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! app-mediakit-knowledge binary — CLI entry point.
//!
//! Subcommands:
//!   serve   — bind and serve an instance from a knowledge.toml
//!   check   — validate content without serving (CI gate; grows in P1)
//!   mcp     — run the claim-query MCP server on stdio (Phase 3.6)

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use app_mediakit_knowledge::app::{router, AppState};
use app_mediakit_knowledge::config::Config;

#[derive(Parser)]
#[command(
    name = "app-mediakit-knowledge",
    version,
    about = "Knowledge wiki engine"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Serve an instance.
    Serve {
        /// Path to the instance knowledge.toml.
        #[arg(long, env = "WIKI_KNOWLEDGE_TOML")]
        knowledge_toml: PathBuf,
    },
    /// Validate content and configuration, then exit.
    Check {
        #[arg(long, env = "WIKI_KNOWLEDGE_TOML")]
        knowledge_toml: PathBuf,
    },
    /// Run the claim-query MCP server on stdio. Provisional — see
    /// `KNOWLEDGE-PLATFORM-PLAN.md` Decision 3 (pending reconciliation with
    /// `service-slm`'s `slm-mcp-server`).
    Mcp {
        #[arg(long, env = "WIKI_KNOWLEDGE_TOML")]
        knowledge_toml: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Serve { knowledge_toml } => serve(knowledge_toml).await,
        Command::Check { knowledge_toml } => check(knowledge_toml),
        Command::Mcp { knowledge_toml } => mcp_serve(knowledge_toml).await,
    }
}

async fn serve(toml_path: PathBuf) -> anyhow::Result<()> {
    let config = Config::load(&toml_path)?;
    let addr = config.socket_addr()?;
    let title = config.site.title.clone();

    let state = AppState::build(config);
    let claims = state.claims.clone();
    let citations = (*state.citations).clone();
    app_mediakit_knowledge::verification::spawn_scheduler(claims, citations);

    let app = router(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("{title} — listening on http://{addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

fn check(toml_path: PathBuf) -> anyhow::Result<()> {
    let config = Config::load(&toml_path)?;
    config.socket_addr()?;
    println!(
        "ok: {} ({} mount(s))",
        config.site.title,
        config.mounts.len()
    );
    Ok(())
}

async fn mcp_serve(toml_path: PathBuf) -> anyhow::Result<()> {
    let config = Config::load(&toml_path)?;
    // AppState::build indexes content and extracts claims into the same
    // redb file the `serve` process uses — the MCP server queries that
    // store directly, it doesn't duplicate extraction logic.
    let state = AppState::build(config);
    app_mediakit_knowledge::mcp::run_stdio(state.claims.clone()).await
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}
