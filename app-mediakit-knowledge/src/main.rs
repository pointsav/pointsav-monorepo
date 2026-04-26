//! Wiki engine binary entry.
//!
//! See ARCHITECTURE.md for the build-phase plan.

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;
use tracing_subscriber::EnvFilter;

use app_mediakit_knowledge::search;
use app_mediakit_knowledge::server::{router, AppState};

#[derive(Parser)]
#[command(name = "app-mediakit-knowledge", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the wiki engine HTTP server.
    Serve {
        /// Path to the directory holding Markdown content.
        #[arg(long, env = "WIKI_CONTENT_DIR")]
        content_dir: PathBuf,

        /// Address to bind. Defaults to loopback.
        #[arg(long, env = "WIKI_BIND", default_value = "127.0.0.1:9090")]
        bind: SocketAddr,

        /// Path to the Foundry citation registry YAML file.
        /// Exposed via GET /api/citations for SAA editor autocomplete.
        #[arg(
            long,
            env = "WIKI_CITATIONS_YAML",
            default_value = "/srv/foundry/citations.yaml"
        )]
        citations_yaml: PathBuf,

        /// Path to the persistent state directory (search index, future KV).
        /// Per Track D `guide-provision-node.md`, the canonical production
        /// location is `/var/lib/local-knowledge/state`.
        #[arg(
            long,
            env = "WIKI_STATE_DIR",
            default_value = "/var/lib/local-knowledge/state"
        )]
        state_dir: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Serve {
            content_dir,
            bind,
            citations_yaml,
            state_dir,
        } => serve(content_dir, bind, citations_yaml, state_dir).await,
    }
}

async fn serve(
    content_dir: PathBuf,
    bind: SocketAddr,
    citations_yaml: PathBuf,
    state_dir: PathBuf,
) -> Result<()> {
    if !content_dir.is_dir() {
        bail!(
            "content directory does not exist or is not a directory: {}",
            content_dir.display()
        );
    }
    tracing::info!(
        content_dir = %content_dir.display(),
        state_dir = %state_dir.display(),
        citations_yaml = %citations_yaml.display(),
        %bind,
        "starting wiki engine"
    );

    // Phase 3 Step 3.1+3.2 — build the search index on startup. Tree walk
    // over content_dir; on-disk index at <state_dir>/search/.
    tracing::info!("building search index");
    let search_index = search::build_index(&content_dir, &state_dir).await?;
    tracing::info!("search index ready");

    let state = AppState {
        content_dir,
        citations_yaml,
        search: Arc::new(search_index),
    };
    let app = router(state);
    let listener = tokio::net::TcpListener::bind(bind).await?;
    tracing::info!(addr = %bind, "listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    tracing::info!("shut down cleanly");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("install ctrl-c handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
    tracing::info!("shutdown signal received");
}
