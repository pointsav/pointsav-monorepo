use anyhow::{anyhow, Context, Result};
use axum::{
    body::Bytes,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
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

const SPA_HTML: &str = include_str!("assets/index.html");

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
    #[serde(default = "default_module_id")]
    module_id: String,
    #[serde(rename = "root")]
    roots: Vec<RootEntry>,
}

fn default_max_bytes() -> usize {
    2 * 1024 * 1024
}

fn default_module_id() -> String {
    "workbench".to_string()
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
    spa_html: Arc<String>,
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
        "md" | "txt"
            | "html"
            | "css"
            | "js"
            | "ts"
            | "json"
            | "toml"
            | "yaml"
            | "yml"
            | "sh"
            | "rs"
            | "py"
            | "rb"
            | "go"
            | "conf"
            | "ini"
            | "env"
            | "lock"
            | "svg"
    )
}

/// Join the parent of a url_path with a new filename.
/// e.g. ("_clones/foo/bar/baz.md", "qux.md") -> "_clones/foo/bar/qux.md"
/// e.g. ("baz.md", "qux.md") -> "qux.md"
fn join_parent_url(url_path: &str, new_name: &str) -> String {
    let trimmed = url_path.trim_start_matches('/');
    match trimmed.rsplit_once('/') {
        Some((parent, _)) => format!("{}/{}", parent, new_name),
        None => new_name.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn get_spa(State(state): State<AppState>) -> Html<String> {
    Html((*state.spa_html).clone())
}

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

/// GET /file?path=<url_path>
async fn get_file(State(state): State<AppState>, Query(q): Query<FileQuery>) -> Response {
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

    Json(FileResponse {
        content,
        mtime,
        writable,
    })
    .into_response()
}

/// PUT /file?path=<url_path>
/// Body: raw UTF-8 text. Header X-Foundry-Editor: 1 required.
/// Optional header X-Foundry-Mtime: <u64> — if provided and mtime differs, returns 409.
async fn put_file(
    State(state): State<AppState>,
    Query(q): Query<FileQuery>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    // CSRF guard
    if headers
        .get("x-foundry-editor")
        .and_then(|v| v.to_str().ok())
        != Some("1")
    {
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
        return err(
            StatusCode::FORBIDDEN,
            "file extension not allowed for writes",
        );
    }

    if body.len() > state.max_bytes {
        return err(
            StatusCode::PAYLOAD_TOO_LARGE,
            "file exceeds max_bytes limit",
        );
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
                        return err(
                            StatusCode::CONFLICT,
                            "file modified on disk since last read",
                        );
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
        fs_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("bin")
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
// Rename
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct RenameQuery {
    from: String,
    to: String,
}

#[derive(Serialize)]
struct RenameResponse {
    ok: bool,
    new_path: String,
    new_name: String,
}

/// POST /rename?from=<url_path>&to=<new_filename>
/// Renames the file within its current directory. `to` must be a bare
/// filename (no slashes). Source must resolve to a writable root.
async fn rename_file(State(state): State<AppState>, Query(q): Query<RenameQuery>) -> Response {
    let new_name = q.to.trim();
    if new_name.is_empty() {
        return err(StatusCode::BAD_REQUEST, "new name is empty");
    }
    if new_name.contains('/') || new_name.contains('\\') {
        return err(
            StatusCode::BAD_REQUEST,
            "new name must not contain slashes",
        );
    }

    let (fs_path, writable) = match resolve_path(&state.roots, &q.from) {
        Ok(v) => v,
        Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()),
    };

    if !fs_path.exists() {
        return err(StatusCode::NOT_FOUND, "source file not found");
    }
    if !writable {
        return err(StatusCode::FORBIDDEN, "path is not writable");
    }

    // Same-name check (before any disk activity)
    let old_name = match fs_path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return err(StatusCode::BAD_REQUEST, "source has no filename"),
    };
    if old_name == new_name {
        return err(StatusCode::BAD_REQUEST, "new name is the same as old name");
    }

    let parent = match fs_path.parent() {
        Some(p) => p,
        None => return err(StatusCode::BAD_REQUEST, "source has no parent directory"),
    };
    let new_fs_path = parent.join(new_name);

    if new_fs_path.exists() {
        return err(StatusCode::CONFLICT, "destination already exists");
    }

    if let Err(e) = fs::rename(&fs_path, &new_fs_path) {
        return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
    }

    let new_url_path = join_parent_url(&q.from, new_name);
    Json(RenameResponse {
        ok: true,
        new_path: new_url_path,
        new_name: new_name.to_string(),
    })
    .into_response()
}

// ---------------------------------------------------------------------------
// Duplicate
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct DuplicateResponse {
    ok: bool,
    new_path: String,
    new_name: String,
}

/// Insert "-copy" (or "-copy-N") before the extension. Returns a filename
/// that does not currently exist in `parent`. Tries N=2..=99.
fn generate_copy_name(parent: &Path, original: &str) -> Option<String> {
    let (stem, ext) = match original.rsplit_once('.') {
        Some((s, e)) if !s.is_empty() => (s.to_string(), Some(e.to_string())),
        _ => (original.to_string(), None),
    };

    let make = |suffix: &str| -> String {
        match &ext {
            Some(e) => format!("{}{}.{}", stem, suffix, e),
            None => format!("{}{}", stem, suffix),
        }
    };

    // First try "-copy"
    let first = make("-copy");
    if !parent.join(&first).exists() {
        return Some(first);
    }

    // Then "-copy-2".."-copy-99"
    for n in 2..=99 {
        let candidate = make(&format!("-copy-{}", n));
        if !parent.join(&candidate).exists() {
            return Some(candidate);
        }
    }

    None
}

/// POST /duplicate?path=<url_path>
/// Copies the file in place with "-copy" inserted before the extension.
/// Source must resolve to a writable root.
async fn duplicate_file(State(state): State<AppState>, Query(q): Query<FileQuery>) -> Response {
    let (fs_path, writable) = match resolve_path(&state.roots, &q.path) {
        Ok(v) => v,
        Err(e) => return err(StatusCode::BAD_REQUEST, e.to_string()),
    };

    if !fs_path.exists() {
        return err(StatusCode::NOT_FOUND, "source file not found");
    }
    if !fs_path.is_file() {
        return err(StatusCode::BAD_REQUEST, "source is not a file");
    }
    if !writable {
        return err(StatusCode::FORBIDDEN, "path is not writable");
    }

    let original_name = match fs_path.file_name().and_then(|n| n.to_str()) {
        Some(n) => n,
        None => return err(StatusCode::BAD_REQUEST, "source has no filename"),
    };
    let parent = match fs_path.parent() {
        Some(p) => p,
        None => return err(StatusCode::BAD_REQUEST, "source has no parent directory"),
    };

    let new_name = match generate_copy_name(parent, original_name) {
        Some(n) => n,
        None => return err(
            StatusCode::CONFLICT,
            "could not find an available copy name (tried -copy through -copy-99)",
        ),
    };
    let new_fs_path = parent.join(&new_name);

    if let Err(e) = fs::copy(&fs_path, &new_fs_path) {
        return err(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
    }

    let new_url_path = join_parent_url(&q.path, &new_name);
    Json(DuplicateResponse {
        ok: true,
        new_path: new_url_path,
        new_name,
    })
    .into_response()
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

    // Inject module_id into SPA HTML as a bootstrap <meta> tag
    let spa_html = SPA_HTML.replacen(
        "<head>",
        &format!(
            "<head>\n<meta name=\"workbench-module-id\" content=\"{}\">",
            config.module_id
        ),
        1,
    );

    let state = AppState {
        roots: Arc::new(config.roots),
        max_bytes: config.max_bytes,
        spa_html: Arc::new(spa_html),
    };

    let app = Router::new()
        .route("/", get(get_spa))
        .route("/file", get(get_file).put(put_file))
        .route("/rename", post(rename_file))
        .route("/duplicate", post(duplicate_file))
        .with_state(state);

    let addr: SocketAddr = config.bind.parse().context("parsing bind address")?;
    println!("app-privategit-workbench listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
