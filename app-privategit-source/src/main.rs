use anyhow::Result;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::{fs, path::PathBuf, sync::Arc};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

// ── State ─────────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    releases_dir: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn release_path(releases_dir: &str, parts: &[&str]) -> PathBuf {
    let mut p = PathBuf::from(releases_dir);
    for part in parts {
        p.push(part);
    }
    p
}

async fn stream_file(path: PathBuf, content_type: &'static str) -> Response {
    match File::open(&path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);
            (StatusCode::OK, [(header::CONTENT_TYPE, content_type)], body).into_response()
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "not found", "path": path.display().to_string()})),
        )
            .into_response(),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

async fn healthz() -> Json<Value> {
    Json(json!({"status": "ok", "service": "app-privategit-source"}))
}

async fn releases_index(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    let base = PathBuf::from(&state.releases_dir);
    let products: Vec<String> = fs::read_dir(&base)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    (StatusCode::OK, Json(json!({"products": products})))
}

async fn product_index(
    State(state): State<Arc<AppState>>,
    Path(product): Path<String>,
) -> (StatusCode, Json<Value>) {
    let base = release_path(&state.releases_dir, &[&product]);
    if !base.exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "product not found"})),
        );
    }
    let versions: Vec<String> = fs::read_dir(&base)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    (
        StatusCode::OK,
        Json(json!({"product": product, "versions": versions})),
    )
}

async fn manifest(
    State(state): State<Arc<AppState>>,
    Path((product, version)): Path<(String, String)>,
) -> Response {
    let path = release_path(&state.releases_dir, &[&product, &version, "MANIFEST.json"]);
    stream_file(path, "application/json").await
}

async fn binary(
    State(state): State<Arc<AppState>>,
    Path((product, version, platform)): Path<(String, String, String)>,
) -> Response {
    // Strip .sig suffix to detect signature requests
    if let Some(base_platform) = platform.strip_suffix(".sig") {
        let path = release_path(
            &state.releases_dir,
            &[&product, &version, &format!("{base_platform}.sig")],
        );
        return stream_file(path, "application/octet-stream").await;
    }

    let path = release_path(&state.releases_dir, &[&product, &version, &platform]);
    let filename = format!("{product}-{version}-{platform}");
    match File::open(&path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = Body::from_stream(stream);
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "application/octet-stream"),
                    (
                        header::CONTENT_DISPOSITION,
                        Box::leak(format!("attachment; filename=\"{filename}\"").into_boxed_str()),
                    ),
                ],
                body,
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "binary not found",
                "note": "Real OS binaries ship with the build pipeline. Check back soon."})),
        )
            .into_response(),
    }
}

async fn git_stub() -> (StatusCode, Json<Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({
            "error": "smart-HTTP Git not yet enabled",
            "see": "https://github.com/pointsav/pointsav-monorepo",
            "arriving": "v0.0.2"
        })),
    )
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    let bind_addr = std::env::var("SOURCE_BIND").unwrap_or_else(|_| "127.0.0.1:9201".into());
    let releases_dir =
        std::env::var("RELEASES_DIR").unwrap_or_else(|_| "/var/lib/local-software/releases".into());

    let state = Arc::new(AppState { releases_dir });

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/releases/", get(releases_index))
        .route("/releases/:product/", get(product_index))
        .route("/releases/:product/:version/MANIFEST", get(manifest))
        .route("/releases/:product/:version/:platform", get(binary))
        .route("/git/*path", get(git_stub).post(git_stub))
        .with_state(state);

    tracing::info!("app-privategit-source listening on {bind_addr}");
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
