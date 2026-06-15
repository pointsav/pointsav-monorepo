mod config;
mod render;
mod schema;
mod state;
mod vault;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use std::{fs, sync::Arc};
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;

use config::Config;
use state::AppState;
use vault::SECTIONS;

#[tokio::main]
async fn main() {
    let cfg = Config::from_env();
    let nav = Arc::new(vault::discover_nav(&cfg.vault));
    eprintln!(
        "app-privategit-design v{}: vault={:?} elements={}",
        env!("CARGO_PKG_VERSION"),
        cfg.vault,
        nav.get("elements").map(|v| v.len()).unwrap_or(0)
    );

    let state = AppState {
        vault: cfg.vault,
        nav,
        tenant: cfg.tenant,
    };

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(index))
        .route("/elements/:slug", get(element_redirect))
        .route("/elements/:slug/:tab", get(element_tab))
        .with_state(state)
        .layer(CompressionLayer::new());

    let listener = TcpListener::bind(&cfg.bind).await.expect("bind failed");
    eprintln!("app-privategit-design listening on {}", cfg.bind);
    axum::serve(listener, app).await.expect("serve failed");
}

async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn index(State(state): State<AppState>) -> Html<String> {
    let nav_html = render::render_nav(&state.nav, SECTIONS, "", "");
    let content = "<div class=\"home-body\"><h1>PointSav Design System</h1>\
                   <p>Select an element from the sidebar.</p></div>";
    Html(render::shell("PointSav Design System", &nav_html, "", "", content))
}

async fn element_redirect(Path(slug): Path<String>, State(state): State<AppState>) -> Response {
    if slug.contains("..") || slug.contains('/') {
        return (StatusCode::BAD_REQUEST, "invalid").into_response();
    }
    let tabs = vault::discover_tabs(&state.vault, "elements", &slug);
    let first = tabs
        .into_iter()
        .next()
        .unwrap_or_else(|| "overview".to_string());
    Redirect::permanent(&format!("/elements/{}/{}", slug, first)).into_response()
}

async fn element_tab(
    Path((slug, tab)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Response {
    if slug.contains("..") || slug.contains('/') || tab.contains("..") || tab.contains('/') {
        return (StatusCode::BAD_REQUEST, "invalid").into_response();
    }
    let tabs = vault::discover_tabs(&state.vault, "elements", &slug);
    if tabs.is_empty() {
        return (StatusCode::NOT_FOUND, "element not found").into_response();
    }
    let md_path = state
        .vault
        .join("elements")
        .join(&slug)
        .join(format!("{}.md", tab));
    let raw = match fs::read_to_string(&md_path) {
        Ok(s) => s,
        Err(_) => return (StatusCode::NOT_FOUND, "tab not found").into_response(),
    };

    // Schema-aware rendering dispatch (D2)
    let (frontmatter, body) = vault::parse_frontmatter(&raw);
    let schema_type = schema::detect(&frontmatter);
    let content = schema::render(schema_type, &frontmatter, &body);

    let nav_html = render::render_nav(&state.nav, SECTIONS, "elements", &slug);
    let tab_bar = render::render_tab_bar("elements", &slug, &tabs, &tab);
    let label = vault::to_title(&slug);

    Html(render::shell(
        &format!("{} — PointSav Design System", label),
        &nav_html,
        &tab_bar,
        &label,
        &content,
    ))
    .into_response()
}
