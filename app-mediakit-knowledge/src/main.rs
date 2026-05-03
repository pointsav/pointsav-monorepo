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

        /// Optional extra directory of GUIDE-* Markdown files (e.g. a
        /// fleet-deployment repo). When set, the engine walks this directory
        /// alongside `content_dir` and serves files at `/wiki/<slug>` URLs,
        /// appearing in categories on the home page just like TOPICs.
        #[arg(long, env = "WIKI_GUIDE_DIR")]
        guide_dir: Option<PathBuf>,

        /// Optional second guide directory (e.g. woodfine-fleet-deployment). When
        /// set, the engine walks this alongside `guide_dir` and `content_dir`.
        #[arg(long, env = "WIKI_GUIDE_DIR_2")]
        guide_dir_2: Option<PathBuf>,

        /// Phase 2 Step 7: enable real-time collaborative editing via
        /// y-codemirror.next + a tokio broadcast WebSocket relay at
        /// `/ws/collab/{slug}`. Default off; the route is only mounted
        /// when this flag is set, and `cm-collab.bundle.js` is only
        /// loaded by the editor when `window.WIKI_COLLAB_ENABLED` is
        /// templated by the server. Two operators editing the same
        /// TOPIC see each other's cursors.
        #[arg(long, env = "WIKI_ENABLE_COLLAB")]
        enable_collab: bool,

        /// Display name shown in the browser tab, site header, and home-page
        /// H1 fallback. Allows the same binary to serve multiple wiki
        /// instances with different branding.
        #[arg(
            long,
            env = "WIKI_SITE_TITLE",
            default_value = "PointSav Documentation Wiki"
        )]
        site_title: String,
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
            guide_dir,
            guide_dir_2,
            enable_collab,
            site_title,
        } => serve(content_dir, guide_dir, guide_dir_2, bind, citations_yaml, state_dir, enable_collab, site_title).await,
    }
}

async fn serve(
    content_dir: PathBuf,
    guide_dir: Option<PathBuf>,
    guide_dir_2: Option<PathBuf>,
    bind: SocketAddr,
    citations_yaml: PathBuf,
    state_dir: PathBuf,
    enable_collab: bool,
    site_title: String,
) -> Result<()> {
    if !content_dir.is_dir() {
        bail!(
            "content directory does not exist or is not a directory: {}",
            content_dir.display()
        );
    }
    if let Some(ref gd) = guide_dir {
        if !gd.is_dir() {
            bail!("guide directory does not exist or is not a directory: {}", gd.display());
        }
        tracing::info!(guide_dir = %gd.display(), "guide directory enabled");
    }
    if let Some(ref gd2) = guide_dir_2 {
        if !gd2.is_dir() {
            bail!("guide directory 2 does not exist or is not a directory: {}", gd2.display());
        }
        tracing::info!(guide_dir_2 = %gd2.display(), "guide directory 2 enabled");
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

    // Phase 4 Step 4.1: open or init git repo. Fail fast if broken.
    tracing::info!("opening git repository");
    let _ = std::process::Command::new("git")
        .args(["config", "--global", "--add", "safe.directory", "*"])
        .status();
    let git_repo = app_mediakit_knowledge::git::open_or_init(&content_dir)?;
    tracing::info!("git repository ready");

    if enable_collab {
        tracing::info!("collab WebSocket relay enabled at /ws/collab/{{slug}}");
    }
    let state = AppState {
        content_dir,
        guide_dir,
        guide_dir_2,
        citations_yaml,
        search: Arc::new(search_index),
        git: Arc::new(std::sync::Mutex::new(git_repo)),
        collab: Arc::new(app_mediakit_knowledge::collab::CollabRooms::new()),
        enable_collab,
        site_title,
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
