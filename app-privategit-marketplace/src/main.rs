use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    bind_addr: String,
    wallet_address: String,
    fs_endpoint: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    let bind_addr = std::env::var("MARKETPLACE_BIND").unwrap_or_else(|_| "127.0.0.1:9200".into());
    let wallet_address = std::env::var("POLYGON_WALLET_ADDRESS").unwrap_or_default();
    let fs_endpoint = std::env::var("FS_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:8020".into());

    let state = Arc::new(AppState { bind_addr: bind_addr.clone(), wallet_address, fs_endpoint });

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(index))
        .route("/products", get(products))
        .with_state(state);

    tracing::info!("app-privategit-marketplace listening on {}", bind_addr);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn healthz() -> Json<Value> {
    Json(json!({"status": "ok", "service": "app-privategit-marketplace"}))
}

async fn index(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({
        "name": "PointSav Software",
        "domain": "software.pointsav.com",
        "payment_chain": "Polygon PoS",
        "payment_token": "USDC",
        "payment_address": state.wallet_address,
        "status": "scaffold — Task Claude to implement product catalog + license issuance"
    })))
}

async fn products(_state: State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
    // Task Claude: load product catalog from flat-file YAML in deployment directory.
    // Each product: {id, name, description, version, price_usdc, binary_url_template}
    // Binary URL issued after license verification via tool-wallet receipt lookup in service-fs.
    (StatusCode::OK, Json(json!({"products": [], "note": "catalog not yet populated"})))
}
