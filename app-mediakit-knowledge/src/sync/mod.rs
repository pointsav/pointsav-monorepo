pub mod git;

use crate::{renderer, search::SearchIndex};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::time::{interval, Duration};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub enum SyncEvent {
    ContentUpdated { new_sha: String },
}

/// Background task: pull from git remote on a regular interval.
/// On HEAD advance: invalidate render cache, signal listeners.
pub async fn run(
    config: Arc<crate::config::Config>,
    _search: Arc<SearchIndex>,
    cache: Arc<renderer::Cache>,
    tx: broadcast::Sender<SyncEvent>,
) {
    let mut ticker = interval(Duration::from_secs(config.sync_interval_secs));
    loop {
        ticker.tick().await;
        match git::pull(&config.content_path, &config.git_remote) {
            Ok(git::PullResult::Advanced { new_sha }) => {
                info!(sha = %new_sha, "git sync advanced HEAD");
                cache.invalidate_all();
                // TODO: rebuild SearchIndex and swap via Arc<RwLock<SearchIndex>>
                tx.send(SyncEvent::ContentUpdated { new_sha }).ok();
            }
            Ok(git::PullResult::UpToDate) => {}
            Err(e) => error!(error = %e, "git sync failed"),
        }
    }
}
