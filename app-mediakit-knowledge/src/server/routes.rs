use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use super::handlers;

pub fn wiki_routes() -> Router<AppState> {
    Router::new()
        .route("/",              get(handlers::home))
        .route("/:category",     get(handlers::category_index))
        .route("/:category/:slug", get(handlers::article))
        .route("/search",        get(handlers::search_results))
}

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/api/search",  get(handlers::search_autocomplete))
        .route("/api/head",    get(handlers::current_head))
        .route("/api/preview", post(handlers::preview))
}

pub fn editor_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/edit/:category/:slug",
            get(handlers::editor_load).post(handlers::editor_submit),
        )
}
