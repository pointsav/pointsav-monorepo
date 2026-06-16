pub mod ai;
pub mod browse;
pub mod edit;
pub mod search;
pub mod sse;

use crate::state::AppState;
use axum::{
    routing::get,
    Router,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(browse::index))
        .route("/elements/:slug", get(browse::element_redirect))
        .route("/elements/:slug/:tab", get(browse::element_tab))
        .route("/tokens/search", get(search::token_search))
        .with_state(state)
}

async fn healthz() -> &'static str {
    "ok"
}
