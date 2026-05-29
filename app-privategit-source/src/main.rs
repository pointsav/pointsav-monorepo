use anyhow::Result;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{fs, path::PathBuf, sync::Arc};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

// ── State ─────────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    releases_dir: String,
    verify_key: Option<VerifyingKey>,
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

fn load_verify_key(path: &str) -> Option<VerifyingKey> {
    let hex = fs::read_to_string(path).ok()?;
    let bytes = hex::decode(hex.trim()).ok()?;
    let arr: [u8; 32] = bytes.try_into().ok()?;
    VerifyingKey::from_bytes(&arr).ok()
}

// ── Request / payload types ───────────────────────────────────────────────────

#[derive(Deserialize)]
struct VerifyKeyRequest {
    license_key_b64: String,
    product_id: String,
}

#[derive(Deserialize)]
struct LicensePayload {
    product: String,
    channel_expiry: String,
    entitlements: Vec<String>,
    version_floor: Option<String>,
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

// Token format: base64url( sig[64] || payload_json )
// sig is Ed25519 over payload_json bytes.
// 200: valid, authorized, not expired
// 401: bad signature or malformed token
// 403: valid sig but wrong product or channel expired
async fn verify_key_endpoint(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyKeyRequest>,
) -> (StatusCode, Json<Value>) {
    let Some(vk) = &state.verify_key else {
        tracing::warn!(
            result = "service-unavailable",
            "verify-key: VERIFY_KEY_PUB not set"
        );
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "verify key not configured — set VERIFY_KEY_PUB"})),
        );
    };
    let key_fp = hex::encode(&vk.as_bytes()[..4]);

    let token_bytes = match URL_SAFE_NO_PAD.decode(&req.license_key_b64) {
        Ok(b) => b,
        Err(_) => {
            tracing::info!(product_id = %req.product_id, key_fp = %key_fp, result = "unauthorized", reason = "malformed-token", "verify-key");
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"valid": false, "reason": "malformed token"})),
            );
        }
    };

    if token_bytes.len() <= 64 {
        tracing::info!(product_id = %req.product_id, key_fp = %key_fp, result = "unauthorized", reason = "token-too-short", "verify-key");
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"valid": false, "reason": "token too short"})),
        );
    }

    let (sig_bytes, payload_bytes) = token_bytes.split_at(64);
    let sig_arr: [u8; 64] = sig_bytes.try_into().expect("exactly 64 bytes");
    let sig = Signature::from_bytes(&sig_arr);

    if vk.verify_strict(payload_bytes, &sig).is_err() {
        tracing::info!(product_id = %req.product_id, key_fp = %key_fp, result = "unauthorized", reason = "invalid-signature", "verify-key");
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"valid": false, "reason": "invalid signature"})),
        );
    }

    let payload: LicensePayload = match serde_json::from_slice(payload_bytes) {
        Ok(p) => p,
        Err(_) => {
            tracing::info!(product_id = %req.product_id, key_fp = %key_fp, result = "unauthorized", reason = "invalid-payload", "verify-key");
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"valid": false, "reason": "invalid payload"})),
            );
        }
    };

    if payload.product != req.product_id {
        tracing::info!(product_id = %req.product_id, key_fp = %key_fp, result = "forbidden", reason = "wrong-product", "verify-key");
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"valid": false, "reason": "wrong product"})),
        );
    }

    // ISO 8601 YYYY-MM-DD is lexicographically ordered — string compare is correct.
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    if payload.channel_expiry < today {
        tracing::info!(product_id = %payload.product, key_fp = %key_fp, result = "forbidden", reason = "channel-expired", expired = %payload.channel_expiry, "verify-key");
        return (
            StatusCode::FORBIDDEN,
            Json(json!({
                "valid": false,
                "reason": "channel expired",
                "expired": payload.channel_expiry,
            })),
        );
    }

    tracing::info!(product_id = %payload.product, key_fp = %key_fp, result = "ok", "verify-key");
    (
        StatusCode::OK,
        Json(json!({
            "valid": true,
            "product": payload.product,
            "version_floor": payload.version_floor,
            "channel_expiry": payload.channel_expiry,
            "entitlements": payload.entitlements,
        })),
    )
}

async fn verify_key_pub(State(state): State<Arc<AppState>>) -> Response {
    match &state.verify_key {
        Some(vk) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            hex::encode(vk.to_bytes()),
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "verify key not configured"})),
        )
            .into_response(),
    }
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

    let verify_key = std::env::var("VERIFY_KEY_PUB")
        .ok()
        .and_then(|path| load_verify_key(&path));
    if verify_key.is_none() {
        tracing::warn!("VERIFY_KEY_PUB not set — /verify-key will return 503");
    }

    let state = Arc::new(AppState {
        releases_dir,
        verify_key,
    });

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/releases/", get(releases_index))
        .route("/releases/:product/", get(product_index))
        .route("/releases/:product/:version/MANIFEST", get(manifest))
        .route("/releases/:product/:version/:platform", get(binary))
        .route("/git/*path", get(git_stub).post(git_stub))
        .route("/verify-key", post(verify_key_endpoint))
        .route("/verify-key.pub", get(verify_key_pub))
        .with_state(state);

    tracing::info!("app-privategit-source listening on {bind_addr}");
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
