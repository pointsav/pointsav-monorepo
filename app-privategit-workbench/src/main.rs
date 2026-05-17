use anyhow::{anyhow, Context, Result};
use axum::{
    body::Bytes,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::Write,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
    time::UNIX_EPOCH,
};
use tokio::net::TcpListener;

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

#[derive(Deserialize, Debug, Clone)]
struct RootEntry {
    url_prefix: String,
    fs_path: String,
    #[serde(default)]
    writable: bool,
}

#[derive(Deserialize, Debug)]
struct Config {
    bind: String,
    #[serde(default = "default_max_bytes")]
    max_bytes: usize,
    #[serde(rename = "root")]
    roots: Vec<RootEntry>,
}

fn default_max_bytes() -> usize {
    2 * 1024 * 1024
}

impl Config {
    fn load(path: &str) -> Result<Self> {
        let s = fs::read_to_string(path).context("reading config.toml")?;
        toml::from_str(&s).context("parsing config.toml")
    }
}

// ---------------------------------------------------------------------------
// App state
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct AppState {
    roots: Arc<Vec<RootEntry>>,
    max_bytes: usize,
}

// ---------------------------------------------------------------------------
// Path resolution helper
// ---------------------------------------------------------------------------

/// Resolve a URL-style path (e.g. "_sandbox-jennifer/foo/bar.txt") to a
/// canonical fs path, checking that it stays within the declared root.
/// Returns (fs_path, writable).
fn resolve_path(roots: &[RootEntry], url_path: &str) -> Result<(PathBuf, bool)> {
    let url_path = url_path.trim_start_matches('/');

    for root in roots {
        let prefix = root.url_prefix.trim_end_matches('/');
        let rest = if url_path == prefix {
            ""
        } else if let Some(r) = url_path.strip_prefix(&format!("{}/", prefix)) {
            r
        } else {
            continue;
        };

        let base = PathBuf::from(&root.fs_path);
        let target = if rest.is_empty() {
            base.clone()
        } else {
            base.join(rest)
        };

        // Canonicalize to resolve symlinks and check containment.
        // The target may not exist yet for write; canonicalize the parent instead.
        let canonical = if target.exists() {
            target.canonicalize().context("canonicalize target")?
        } else {
            let parent = target.parent().ok_or_else(|| anyhow!("no parent"))?;
            let cp = parent.canonicalize().context("canonicalize parent")?;
            cp.join(target.file_name().ok_or_else(|| anyhow!("no filename"))?)
        };

        let root_canonical = base.canonicalize().context("canonicalize root")?;
        if !canonical.starts_with(&root_canonical) {
            return Err(anyhow!("path traversal attempt"));
        }

        return Ok((canonical, root.writable));
    }

    Err(anyhow!("no matching root for path: {}", url_path))
}

// ---------------------------------------------------------------------------
// Allowed write extensions
// ---------------------------------------------------------------------------

fn allowed_write_ext(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    matches!(
        ext.to_lowercase().as_str(),
        "md" | "txt" | "html" | "css" | "js" | "ts" | "json" | "toml" | "yaml" | "yml"
            | "sh" | "rs" | "py" | "rb" | "go" | "conf" | "ini" | "env" | "lock" | "svg"
    )
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct FileQuery {
    path: String,
}

#[derive(Serialize)]
struct FileResponse {
    content: String,
    mtime: u64,
    writable: bool,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

fn err(status: StatusCode, msg: impl Into<String>) -> Response {
    (status, Json(ErrorBody { error: msg.into() })).into_response()
}

/// GET /_api/edit/file?path=<url_path>
async fn get_file(
    State(state): State<AppState>,
    Query(q): Query<FileQuery>,
) -> Response {
    let (fs_path, writable) = match resolve_path(&state.roots, &q.path) {
        Ok(v) => v,
        Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()),
    };

    if !fs_path.exists() {
        return err(StatusCode::NOT_FOUND, "file not found");
    }
    if !fs_path.is_file() {
        return err(StatusCode::BAD_REQUEST, "not a file");
    }

    let meta = match fs::metadata(&fs_path) {
        Ok(m) => m,
        Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let content = match fs::read_to_string(&fs_path) {
        Ok(c) => c,
        Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    Json(FileResponse { content, mtime, writable }).into_response()
}

/// PUT /_api/edit/file?path=<url_path>
/// Body: raw UTF-8 text. Header X-Foundry-Editor: 1 required.
/// Optional header X-Foundry-Mtime: <u64> — if provided and mtime differs, returns 409.
async fn put_file(
    State(state): State<AppState>,
    Query(q): Query<FileQuery>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    // CSRF guard
    if headers.get("x-foundry-editor").and_then(|v| v.to_str().ok()) != Some("1") {
        return err(StatusCode::FORBIDDEN, "missing X-Foundry-Editor header");
    }

    let (fs_path, writable) = match resolve_path(&state.roots, &q.path) {
        Ok(v) => v,
        Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()),
    };

    if !writable {
        return err(StatusCode::FORBIDDEN, "path is not writable");
    }

    if !allowed_write_ext(&fs_path) {
        return err(StatusCode::FORBIDDEN, "file extension not allowed for writes");
    }

    if body.len() > state.max_bytes {
        return err(StatusCode::PAYLOAD_TOO_LARGE, "file exceeds max_bytes limit");
    }

    // mtime conflict check
    if let Some(client_mtime_str) = headers.get("x-foundry-mtime").and_then(|v| v.to_str().ok()) {
        if let Ok(client_mtime) = client_mtime_str.parse::<u64>() {
            if fs_path.exists() {
                if let Ok(meta) = fs::metadata(&fs_path) {
                    let server_mtime = meta
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    if server_mtime != client_mtime {
                        return err(StatusCode::CONFLICT, "file modified on disk since last read");
                    }
                }
            }
        }
    }

    let content = match std::str::from_utf8(&body) {
        Ok(s) => s,
        Err(_) => return err(StatusCode::BAD_REQUEST, "body is not valid UTF-8"),
    };

    // Atomic write: write to .tmp, then rename
    let tmp_path = fs_path.with_extension(format!(
        "{}.tmp",
        fs_path.extension().and_then(|e| e.to_str()).unwrap_or("bin")
    ));

    let write_result: Result<()> = (|| {
        let mut f = fs::File::create(&tmp_path)?;
        f.write_all(content.as_bytes())?;
        f.sync_all()?;
        fs::rename(&tmp_path, &fs_path)?;
        Ok(())
    })();

    if let Err(e) = write_result {
        let _ = fs::remove_file(&tmp_path);
        return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
    }

    // Return new mtime
    let new_mtime = fs::metadata(&fs_path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let mut resp_map = HashMap::new();
    resp_map.insert("ok", serde_json::Value::Bool(true));
    resp_map.insert("mtime", serde_json::Value::Number(new_mtime.into()));
    Json(resp_map).into_response()
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());

    let config = Config::load(&config_path)?;
    let state = AppState {
        roots: Arc::new(config.roots),
        max_bytes: config.max_bytes,
    };

    let app = Router::new()
        .route("/file", get(get_file).put(put_file))
        .with_state(state);

    let addr: SocketAddr = config.bind.parse().context("parsing bind address")?;
    println!("local-intranet-editor listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
