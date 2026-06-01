mod workbench;

use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post, put},
    Json, Router,
};
use serde_json::json;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tower_http::cors::CorsLayer;
use axum::response::sse::{Event, KeepAlive, Sse};
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::convert::Infallible;
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt as _};

#[derive(RustEmbed)]
#[folder = "src/assets/"]
pub(crate) struct Assets;

#[derive(Clone, Debug)]
pub(crate) struct WorkbenchRoot {
    pub(crate) url_prefix: String,
    pub(crate) fs_path: PathBuf,
    pub(crate) writable: bool,
}

#[derive(Deserialize, Default)]
struct ProtoConfig {
    #[serde(default)]
    root: Vec<RootConfig>,
    weasyprint: Option<String>,
    #[serde(default = "default_max_bytes")]
    max_bytes: usize,
}

#[derive(Deserialize)]
struct RootConfig {
    url_prefix: String,
    fs_path: String,
    #[serde(default)]
    writable: bool,
}

fn default_max_bytes() -> usize {
    2 * 1024 * 1024
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) workspace_dir: Arc<PathBuf>,
    pub(crate) tokens_path: Arc<PathBuf>,
    pub(crate) events_tx: broadcast::Sender<String>,
    pub(crate) roots: Arc<Vec<WorkbenchRoot>>,
    pub(crate) weasyprint: Arc<Option<PathBuf>>,
    pub(crate) max_bytes: usize,
}

#[tokio::main]
async fn main() {
    let workspace_dir = std::env::var("WORKPLACE_PROTO_WORKSPACE")
        .unwrap_or_else(|_| "/home/jennifer/workbench".to_string());
    let port: u16 = std::env::var("WORKPLACE_PROTO_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(9110);

    let workspace_path = PathBuf::from(&workspace_dir);
    let memo_dir = workspace_path.join("memo");
    if !memo_dir.exists() {
        std::fs::create_dir_all(&memo_dir).expect("failed to create memo/ dir");
    }
    let proforma_dir = workspace_path.join("proforma");
    if !proforma_dir.exists() {
        std::fs::create_dir_all(&proforma_dir).expect("failed to create proforma/ dir");
    }
    let bim_dir = workspace_path.join("bim");
    if !bim_dir.exists() {
        std::fs::create_dir_all(&bim_dir).expect("failed to create bim/ dir");
    }

    let tokens_path = std::env::var("DESIGN_TOKENS_PATH").unwrap_or_else(|_| {
        "/srv/foundry/vendor/pointsav-design-system/tokens/dtcg-bundle.json".to_string()
    });

    // Load optional config.toml for workbench roots
    let config_path = std::env::var("WORKPLACE_PROTO_CONFIG")
        .unwrap_or_else(|_| "config.toml".to_string());
    let config: ProtoConfig = if std::path::Path::new(&config_path).exists() {
        match std::fs::read_to_string(&config_path)
            .ok()
            .and_then(|s| toml::from_str::<ProtoConfig>(&s).ok())
        {
            Some(c) => c,
            None => {
                eprintln!("warning: failed to parse {config_path}; using defaults");
                ProtoConfig::default()
            }
        }
    } else {
        ProtoConfig::default()
    };

    let roots: Vec<WorkbenchRoot> = config
        .root
        .iter()
        .map(|r| WorkbenchRoot {
            url_prefix: r.url_prefix.clone(),
            fs_path: PathBuf::from(&r.fs_path),
            writable: r.writable,
        })
        .collect();

    let weasyprint: Option<PathBuf> = config
        .weasyprint
        .as_deref()
        .map(PathBuf::from)
        .or_else(|| {
            let p = PathBuf::from("/usr/bin/weasyprint");
            p.exists().then_some(p)
        });

    let max_bytes = if config.max_bytes == 0 {
        default_max_bytes()
    } else {
        config.max_bytes
    };

    // SSE broadcast + filesystem watcher
    let (events_tx, _) = broadcast::channel::<String>(64);
    let watcher_tx = events_tx.clone();
    let watch_path = workspace_path.clone();
    // Collect all root (url_prefix, fs_path) pairs for path normalisation.
    let root_mappings: Vec<(String, PathBuf)> = roots
        .iter()
        .map(|r| (r.url_prefix.clone(), r.fs_path.clone()))
        .collect();
    tokio::spawn(async move {
        let (inner_tx, mut inner_rx) = tokio::sync::mpsc::channel::<String>(8);
        let mut watcher = recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                let path_str = event.paths.first()
                    .and_then(|p| p.to_str())
                    .unwrap_or("")
                    .to_string();
                let _ = inner_tx.blocking_send(path_str);
            }
        })
        .expect("failed to create file watcher");
        watcher
            .watch(&watch_path, RecursiveMode::Recursive)
            .expect("failed to watch workspace");
        // Watch ALL configured roots (not just writable ones) so inotify fires
        // for external writes (e.g. Claude Code editing files in _clones/).
        for (_, fs_path) in &root_mappings {
            if let Err(e) = watcher.watch(fs_path, RecursiveMode::Recursive) {
                eprintln!("warning: could not watch {:?}: {}", fs_path, e);
            }
        }
        while let Some(path_str) = inner_rx.recv().await {
            if path_str.is_empty() {
                let _ = watcher_tx.send("changed".to_string());
                continue;
            }
            // Convert absolute filesystem path to root-relative (url_prefix/rel)
            // so the frontend can match it against tab.path directly.
            let rel = root_mappings.iter().find_map(|(prefix, base)| {
                let base_str = base.to_string_lossy();
                let base_str = base_str.trim_end_matches('/');
                if path_str.starts_with(base_str) {
                    let rest = path_str[base_str.len()..].trim_start_matches('/');
                    Some(format!("{}/{}", prefix, rest))
                } else {
                    None
                }
            }).unwrap_or_else(|| path_str.clone());
            let mtime = std::fs::metadata(&path_str)
                .ok()
                .and_then(|m| {
                    m.modified().ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                })
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let msg = format!(r#"{{"event":"changed","path":"{}","mtime":{}}}"#, rel, mtime);
            let _ = watcher_tx.send(msg);
        }
    });

    let state = AppState {
        workspace_dir: Arc::new(workspace_path),
        tokens_path: Arc::new(PathBuf::from(&tokens_path)),
        events_tx,
        roots: Arc::new(roots),
        weasyprint: Arc::new(weasyprint),
        max_bytes,
    };

    println!("app-workplace-http-prototype listening on http://0.0.0.0:{port}");
    println!("workspace: {workspace_dir}");
    println!("tokens:    {tokens_path}");
    println!("config:    {config_path}");
    println!("workbench roots: {}", state.roots.len());

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/memo", get(serve_memo))
        .route("/proforma", get(serve_proforma))
        .route("/tokens", get(serve_tokens_page))
        .route("/style.css", get(serve_css))
        .route("/api/files", get(list_files))
        .route("/api/files/read", get(read_file))
        .route("/api/files/save", put(save_file))
        .route("/api/files/create", post(create_file))
        .route("/api/proforma/files", get(list_proforma_files))
        .route("/api/proforma/create", post(create_proforma_file))
        .route("/bim", get(serve_bim))
        .route("/api/bim/files", get(list_bim_files))
        .route("/api/bim/create", post(create_bim_file))
        .route("/api/tokens", get(get_tokens))
        .route("/api/files/events", get(get_file_events))
        .route("/workbench/", get(|| async { Redirect::permanent("/workbench") }))
        .nest("/workbench", workbench::router())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> impl IntoResponse {
    Redirect::permanent("/workbench")
}

async fn serve_memo() -> impl IntoResponse {
    serve_asset("memo.html", "text/html")
}

async fn serve_tokens_page() -> impl IntoResponse {
    serve_asset("tokens.html", "text/html")
}

async fn get_tokens(State(state): State<AppState>) -> impl IntoResponse {
    match std::fs::read_to_string(&*state.tokens_path) {
        Ok(content) => (
            [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
            content,
        )
            .into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn serve_css() -> impl IntoResponse {
    serve_asset("style.css", "text/css")
}

fn serve_asset(name: &str, content_type: &'static str) -> Response {
    match Assets::get(name) {
        Some(content) => (
            [(header::CONTENT_TYPE, content_type)],
            content.data.into_owned(),
        )
            .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Serialize)]
struct FileEntry {
    name: String,
    path: String,
}

async fn list_files(State(state): State<AppState>) -> impl IntoResponse {
    let memo_dir = state.workspace_dir.join("memo");
    let mut entries: Vec<FileEntry> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&memo_dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("html") {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    entries.push(FileEntry {
                        name: name.to_string(),
                        path: format!("memo/{name}"),
                    });
                }
            }
        }
    }
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Json(entries)
}

#[derive(Deserialize)]
struct ReadParams {
    path: String,
}

async fn read_file(
    State(state): State<AppState>,
    Query(params): Query<ReadParams>,
) -> impl IntoResponse {
    match resolve_workspace_path(&state.workspace_dir, &params.path) {
        Ok(abs) => match std::fs::read_to_string(&abs) {
            Ok(content) => (StatusCode::OK, content).into_response(),
            Err(_) => StatusCode::NOT_FOUND.into_response(),
        },
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

#[derive(Deserialize)]
struct SaveBody {
    path: String,
    content: String,
}

async fn save_file(
    State(state): State<AppState>,
    Json(body): Json<SaveBody>,
) -> impl IntoResponse {
    match resolve_workspace_path(&state.workspace_dir, &body.path) {
        Ok(abs) => {
            if let Some(parent) = abs.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            match std::fs::write(&abs, &body.content) {
                Ok(_) => {
                    let _ = state.events_tx.send("changed".to_string());
                    StatusCode::OK.into_response()
                }
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

#[derive(Deserialize)]
struct CreateBody {
    name: String,
}

#[derive(Serialize)]
struct CreateResponse {
    path: String,
}

async fn create_file(
    State(state): State<AppState>,
    Json(body): Json<CreateBody>,
) -> impl IntoResponse {
    let safe_name = sanitize_filename(&body.name);
    let filename = if safe_name.ends_with(".html") {
        safe_name
    } else {
        format!("{safe_name}.html")
    };
    let rel_path = format!("memo/{filename}");
    match resolve_workspace_path(&state.workspace_dir, &rel_path) {
        Ok(abs) => {
            let skeleton = format!(
                "<h1>{}</h1>\n<p></p>\n",
                html_escape(&body.name)
            );
            match std::fs::write(&abs, skeleton) {
                Ok(_) => {
                    let _ = state.events_tx.send("changed".to_string());
                    (StatusCode::CREATED, Json(CreateResponse { path: rel_path }))
                        .into_response()
                }
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

async fn get_file_events(State(state): State<AppState>) -> impl IntoResponse {
    let rx = state.events_tx.subscribe();
    let stream = BroadcastStream::new(rx)
        .filter_map(|r| r.ok())
        .map(|_| Ok::<_, Infallible>(Event::default().data("changed")));
    Sse::new(stream).keep_alive(KeepAlive::default())
}

fn resolve_workspace_path(workspace: &Path, rel: &str) -> Result<PathBuf, ()> {
    let candidate = workspace.join(rel);
    let abs = candidate.canonicalize().unwrap_or(candidate.clone());
    let ws_abs = workspace
        .canonicalize()
        .unwrap_or_else(|_| workspace.to_path_buf());
    if abs.starts_with(&ws_abs) {
        Ok(abs)
    } else {
        Err(())
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

async fn serve_proforma() -> impl IntoResponse {
    serve_asset("proforma.html", "text/html")
}

async fn list_proforma_files(State(state): State<AppState>) -> impl IntoResponse {
    let proforma_dir = state.workspace_dir.join("proforma");
    let mut entries: Vec<FileEntry> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&proforma_dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    entries.push(FileEntry {
                        name: name.to_string(),
                        path: format!("proforma/{name}"),
                    });
                }
            }
        }
    }
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Json(entries)
}

async fn create_proforma_file(
    State(state): State<AppState>,
    Json(body): Json<CreateBody>,
) -> impl IntoResponse {
    let safe_name = sanitize_filename(&body.name);
    let filename = if safe_name.ends_with(".json") {
        safe_name
    } else {
        format!("{safe_name}.json")
    };
    let rel_path = format!("proforma/{filename}");
    match resolve_workspace_path(&state.workspace_dir, &rel_path) {
        Ok(abs) => {
            let skeleton = json!({
                "schema": "proforma-v1.0",
                "title": body.name,
                "rows": 20,
                "cols": 8,
                "cells": {}
            });
            match std::fs::write(&abs, skeleton.to_string()) {
                Ok(_) => {
                    let _ = state.events_tx.send("changed".to_string());
                    (StatusCode::CREATED, Json(CreateResponse { path: rel_path }))
                        .into_response()
                }
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

async fn serve_bim() -> impl IntoResponse {
    serve_asset("bim.html", "text/html")
}

async fn list_bim_files(State(state): State<AppState>) -> impl IntoResponse {
    let bim_dir = state.workspace_dir.join("bim");
    let mut entries: Vec<FileEntry> = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&bim_dir) {
        for entry in rd.flatten() {
            let path = entry.path();
            if path.to_str().map(|s| s.ends_with(".bim.json")).unwrap_or(false) {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    entries.push(FileEntry {
                        name: name.to_string(),
                        path: format!("bim/{name}"),
                    });
                }
            }
        }
    }
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Json(entries)
}

async fn create_bim_file(
    State(state): State<AppState>,
    Json(body): Json<CreateBody>,
) -> impl IntoResponse {
    let safe_name = sanitize_filename(&body.name);
    let filename = if safe_name.ends_with(".bim.json") {
        safe_name
    } else {
        format!("{safe_name}.bim.json")
    };
    let rel_path = format!("bim/{filename}");
    match resolve_workspace_path(&state.workspace_dir, &rel_path) {
        Ok(abs) => {
            let skeleton = json!({
                "$schema": "bim-workspace-v1.0",
                "project": {
                    "title":    { "$value": body.name, "$type": "string" },
                    "ifc-file": { "$value": "", "$type": "file-reference",
                                  "$description": "path to .ifc file relative to workspace root" }
                },
                "element-styles": {
                    "IfcWall":   { "color": { "$value": "#c8c8c8", "$type": "color" }, "visible": { "$value": true, "$type": "boolean" } },
                    "IfcDoor":   { "color": { "$value": "#8b6914", "$type": "color" }, "visible": { "$value": true, "$type": "boolean" } },
                    "IfcWindow": { "color": { "$value": "#88c8e8", "$type": "color" }, "visible": { "$value": true, "$type": "boolean" } },
                    "IfcSlab":   { "color": { "$value": "#a0a0a0", "$type": "color" }, "visible": { "$value": true, "$type": "boolean" } },
                    "IfcColumn": { "color": { "$value": "#b0b0b0", "$type": "color" }, "visible": { "$value": true, "$type": "boolean" } },
                    "IfcBeam":   { "color": { "$value": "#909090", "$type": "color" }, "visible": { "$value": true, "$type": "boolean" } },
                    "IfcStair":  { "color": { "$value": "#d4c08c", "$type": "color" }, "visible": { "$value": true, "$type": "boolean" } },
                    "IfcRoof":   { "color": { "$value": "#8c6060", "$type": "color" }, "visible": { "$value": true, "$type": "boolean" } }
                }
            });
            match std::fs::write(&abs, serde_json::to_string_pretty(&skeleton).unwrap_or_default()) {
                Ok(_) => {
                    let _ = state.events_tx.send("changed".to_string());
                    (StatusCode::CREATED, Json(CreateResponse { path: rel_path }))
                        .into_response()
                }
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}
