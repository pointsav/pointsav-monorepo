pub mod ai;
pub mod browse;
pub mod edit;
pub mod search;
pub mod sse;

use crate::state::AppState;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub fn build_router(state: AppState) -> Router {
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static");

    Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(browse::index))
        .route("/elements/:slug", get(browse::element_redirect))
        .route("/elements/:slug/:tab", get(browse::element_tab))
        .route("/tokens/search", get(search::token_search))
        .route("/sidebar/sse", get(sse::sidebar_sse))
        .nest_service("/static", ServeDir::new(static_dir))
        .with_state(state)
}

async fn healthz() -> &'static str {
    "ok"
}
