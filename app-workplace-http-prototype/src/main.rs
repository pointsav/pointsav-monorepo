use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tower_http::cors::CorsLayer;

#[derive(RustEmbed)]
#[folder = "src/assets/"]
struct Assets;

#[derive(Clone)]
struct AppState {
    workspace_dir: Arc<PathBuf>,
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

    let state = AppState {
        workspace_dir: Arc::new(workspace_path),
    };

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/memo", get(serve_memo))
        .route("/style.css", get(serve_css))
        .route("/api/files", get(list_files))
        .route("/api/files/read", get(read_file))
        .route("/api/files/save", put(save_file))
        .route("/api/files/create", post(create_file))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("0.0.0.0:{port}");
    println!("app-workplace-http-prototype listening on http://{addr}");
    println!("workspace: {workspace_dir}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> impl IntoResponse {
    serve_asset("index.html", "text/html")
}

async fn serve_memo() -> impl IntoResponse {
    serve_asset("memo.html", "text/html")
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
    match resolve_path(&state.workspace_dir, &params.path) {
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
    match resolve_path(&state.workspace_dir, &body.path) {
        Ok(abs) => {
            if let Some(parent) = abs.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            match std::fs::write(&abs, &body.content) {
                Ok(_) => StatusCode::OK.into_response(),
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
    match resolve_path(&state.workspace_dir, &rel_path) {
        Ok(abs) => {
            let skeleton = format!(
                "<h1>{}</h1>\n<p></p>\n",
                html_escape(&body.name)
            );
            match std::fs::write(&abs, skeleton) {
                Ok(_) => (StatusCode::CREATED, Json(CreateResponse { path: rel_path }))
                    .into_response(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

fn resolve_path(workspace: &Path, rel: &str) -> Result<PathBuf, ()> {
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
