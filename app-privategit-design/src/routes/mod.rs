// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.


pub mod browse;
pub mod bundle;
pub mod edit;
pub mod search;
pub mod seo;
pub mod sse;

use crate::state::AppState;
use axum::{
    routing::{get, post, put},
    Router,
};
use tower_http::services::ServeDir;

pub fn build_router(state: AppState) -> Router {
    let static_dir = state.static_dir.clone();

    Router::new()
        .route("/healthz", get(healthz))
        .route("/robots.txt", get(seo::robots_txt))
        .route("/sitemap.xml", get(seo::sitemap_xml))
        .route("/llms.txt", get(seo::llms_txt))
        .route("/", get(browse::index))
        .route("/es", get(browse::index_es))
        .route("/tokens", get(browse::tokens_gallery_page))
        // "Foundations" is the v3 redesign's decided replacement vocabulary for
        // "Tokens" (BRIEF-design-pointsav-v3-ground-up-rethink.md, Phase 1 result) --
        // added 2026-08-02 as a real, working alias to the same handler so the
        // refreshed mockups' "Browse the tokens" CTA resolves to something real.
        // Deliberately NOT yet a full IA rename: /tokens stays live and is still
        // what every existing nav label, sitemap entry, and llms.txt line points at.
        // Full vocabulary migration (renaming the nav label, updating sitemap/
        // llms.txt generation, and the homepage copy rewrite to match) is scoped as
        // its own follow-up -- see NEXT.md.
        .route("/foundations", get(browse::tokens_gallery_page))
        .route("/tokens.json", get(bundle::tokens_json_redirect))
        .route("/adoption", get(browse::adoption_page))
        .route("/elements/:slug/download", get(browse::bundle_download))
        .route(
            "/components/:slug/recipe.json",
            get(browse::component_recipe),
        )
        .route("/:section/:slug", get(browse::item_redirect))
        .route("/:section/:slug/:tab", get(browse::item_tab))
        .route("/tokens/search", get(search::token_search))
        .route("/bundles/:name", get(bundle::list))
        .route("/bundles/:name/download", get(bundle::download))
        .route("/bundles/:name/:filename", get(bundle::file))
        .route("/sidebar/sse", get(sse::sidebar_sse))
        .route("/vault/:section/:slug/:tab/raw", get(edit::get_raw))
        .route("/vault/:section/:slug/:tab", put(edit::put_save))
        .route("/mcp", post(crate::mcp::mcp_handler))
        .nest_service("/static", ServeDir::new(static_dir))
        .with_state(state)
}

async fn healthz() -> &'static str {
    "ok"
}
