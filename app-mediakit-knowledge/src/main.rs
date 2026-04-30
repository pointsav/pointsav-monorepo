use anyhow::Result;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::info;

mod config;
mod editor;
mod renderer;
mod search;
mod server;
mod sync;

use config::Config;
use search::SearchIndex;
use sync::SyncEvent;

/// Shared application state passed to every Axum handler.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub search: Arc<SearchIndex>,
    pub render_cache: Arc<renderer::Cache>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("app_mediakit_knowledge=info".parse()?),
        )
        .init();

    let config = Arc::new(Config::from_env()?);
    info!(content_path = %config.content_path.display(), "starting app-mediakit-knowledge");

    let search = Arc::new(SearchIndex::build(&config.content_path)?);
    info!("search index built");

    let render_cache = Arc::new(renderer::Cache::new(config.cache_size));

    let (sync_tx, _sync_rx) = broadcast::channel::<SyncEvent>(16);

    let sync_config = Arc::clone(&config);
    let sync_search = Arc::clone(&search);
    let sync_cache  = Arc::clone(&render_cache);
    let sync_tx2    = sync_tx.clone();
    tokio::spawn(async move {
        sync::run(sync_config, sync_search, sync_cache, sync_tx2).await;
    });

    let state = AppState {
        config: Arc::clone(&config),
        search: Arc::clone(&search),
        render_cache: Arc::clone(&render_cache),
    };

    let app  = server::build_router(state);
    let addr = config.bind_addr;
    info!(%addr, "listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
