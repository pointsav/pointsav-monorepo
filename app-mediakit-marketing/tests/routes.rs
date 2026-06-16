//! In-process route tests — exercise the full axum stack (handler →
//! content::load_page_lang → shell::render_page) without binding a socket, via
//! `tower::ServiceExt::oneshot`. Covers the bilingual `/es` wiring added in P2.

use std::path::PathBuf;

use app_mediakit_marketing::pending::Queue;
use app_mediakit_marketing::server::{router, AppState};
use app_mediakit_shell::{tokens, Brand};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn app() -> axum::Router {
    let content_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");
    let tmp = tempfile::tempdir().unwrap();
    let pending = Queue::open(tmp.path()).unwrap();
    // Keep the temp state dir for the lifetime of the process; these tests do
    // not exercise the pending queue, so leaking it is harmless.
    std::mem::forget(tmp);
    let state = AppState {
        content_dir,
        brand: Brand::woodfine(),
        tokens_css: tokens::DEFAULT_TOKENS_CSS.to_string(),
        pending,
        mcp_enabled: false,
    };
    router(state)
}

async fn get(uri: &str) -> (StatusCode, String) {
    let resp = app()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

#[tokio::test]
async fn home_renders_new_sections_en() {
    let (status, body) = get("/").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("card-grid"));
    assert!(body.contains("section-feature"));
    assert!(body.contains("header-cta")); // persistent enquire CTA
    assert!(body.contains(r#"<html lang="en">"#));
    // The legacy fragile client-side pattern must be structurally absent.
    assert!(!body.contains("__bundler/template"));
}

#[tokio::test]
async fn es_route_serves_spanish() {
    let (status, body) = get("/es").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains(r#"<html lang="es">"#));
    assert!(body.contains("Qué hacemos"));
}

#[tokio::test]
async fn es_page_falls_back_when_no_variant() {
    // `contact` has no page.es.yaml -> falls back to page.yaml, still 200.
    let (status, _) = get("/es/page/contact").await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn healthz_ok() {
    let (status, body) = get("/healthz").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "ok");
}

#[tokio::test]
async fn unknown_page_404() {
    let (status, _) = get("/page/nope").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
