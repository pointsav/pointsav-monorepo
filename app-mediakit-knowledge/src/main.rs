//! Wiki engine binary entry.
//!
//! See ARCHITECTURE.md for the build-phase plan.

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::signal;
use tracing_subscriber::EnvFilter;

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
        Command::Serve { content_dir, bind } => serve(content_dir, bind).await,
    }
}

async fn serve(content_dir: PathBuf, bind: SocketAddr) -> Result<()> {
    if !content_dir.is_dir() {
        bail!(
            "content directory does not exist or is not a directory: {}",
            content_dir.display()
        );
    }
    tracing::info!(content_dir = %content_dir.display(), %bind, "starting wiki engine");

    let state = AppState { content_dir };
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
