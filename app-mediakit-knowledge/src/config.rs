use anyhow::{Context, Result};
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    /// Path to local clone of content-wiki-documentation. Required.
    pub content_path: PathBuf,
    /// Git remote name. Default: "origin"
    pub git_remote: String,
    /// Seconds between git pull attempts. Default: 60
    pub sync_interval_secs: u64,
    /// LRU page cache capacity. Default: 256
    pub cache_size: usize,
    /// MBA authentication endpoint. Required when editor is enabled.
    pub editor_auth_url: Option<String>,
    /// Enable browser editor. Default: false
    pub editor_enabled: bool,
    /// HTTP bind address. Default: 0.0.0.0:3000
    pub bind_addr: SocketAddr,
    /// Site title in header and <title>. Default: "PointSav Documentation"
    pub site_title: String,
    /// Base URL for canonical links. Default: "http://localhost:3000"
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            content_path: std::env::var("CONTENT_PATH")
                .context("CONTENT_PATH is required")?.into(),
            git_remote: std::env::var("GIT_REMOTE")
                .unwrap_or_else(|_| "origin".into()),
            sync_interval_secs: std::env::var("SYNC_INTERVAL")
                .unwrap_or_else(|_| "60".into())
                .parse().context("SYNC_INTERVAL must be a positive integer")?,
            cache_size: std::env::var("CACHE_SIZE")
                .unwrap_or_else(|_| "256".into())
                .parse().context("CACHE_SIZE must be a positive integer")?,
            editor_auth_url: std::env::var("EDITOR_AUTH").ok(),
            editor_enabled: std::env::var("EDITOR_ENABLED")
                .unwrap_or_else(|_| "false".into())
                .parse().context("EDITOR_ENABLED must be 'true' or 'false'")?,
            bind_addr: std::env::var("BIND_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:3000".into())
                .parse().context("BIND_ADDR must be a valid socket address")?,
            site_title: std::env::var("SITE_TITLE")
                .unwrap_or_else(|_| "PointSav Documentation".into()),
            base_url: std::env::var("BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".into()),
        })
    }
}
