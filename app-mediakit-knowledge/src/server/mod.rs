pub mod handlers;
pub mod routes;
pub mod templates;

use crate::AppState;
use axum::Router;
use tower_http::services::ServeDir;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::wiki_routes())
        .merge(routes::api_routes())
        .merge(routes::editor_routes())
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}
