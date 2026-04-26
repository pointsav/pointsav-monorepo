//! HTTP server and route handlers.

use axum::{
    extract::{Path, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;

use crate::assets::StaticAsset;
use crate::error::WikiError;
use crate::render::{parse_page, render_html};

#[derive(Clone)]
pub struct AppState {
    pub content_dir: PathBuf,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/wiki/{slug}", get(wiki_page))
        .route("/static/{*path}", get(static_asset))
        .route("/healthz", get(healthz))
        .with_state(Arc::new(state))
}

async fn healthz() -> &'static str {
    "ok"
}

async fn index(State(state): State<Arc<AppState>>) -> Result<Markup, WikiError> {
    let mut entries = fs::read_dir(&state.content_dir).await?;
    let mut pages: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(slug) = name.strip_suffix(".md") {
            // Skip the bilingual sibling files (`*.es.md`) for the index;
            // they're addressable directly by full slug if needed.
            if !slug.ends_with(".es") {
                pages.push(slug.to_string());
            }
        }
    }
    pages.sort();

    Ok(chrome(
        "Index",
        html! {
            h1 { "PointSav Knowledge" }
            p.lede {
                "Flat-file Markdown source-of-truth, single-binary engine, AI-optional. "
                "Phase 1 — render."
            }
            h2 { "Pages" }
            @if pages.is_empty() {
                p.empty { "No pages in content directory yet." }
            } @else {
                ul.page-list {
                    @for slug in &pages {
                        li {
                            a href=(format!("/wiki/{slug}")) { (slug) }
                        }
                    }
                }
            }
        },
    ))
}

async fn wiki_page(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Result<Markup, WikiError> {
    // Phase 1 slug safety: reject path traversal and nested paths.
    // Phase 6 introduces full slug normalisation rules.
    if slug.contains("..") || slug.contains('/') || slug.is_empty() {
        return Err(WikiError::NotFound(slug));
    }
    let path = state.content_dir.join(format!("{slug}.md"));
    let text = match fs::read_to_string(&path).await {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(WikiError::NotFound(slug));
        }
        Err(e) => return Err(e.into()),
    };
    let parsed = parse_page(&text)?;
    let body_html = render_html(&parsed.body_md);
    let title = parsed
        .frontmatter
        .title
        .clone()
        .unwrap_or_else(|| slug.clone());
    Ok(chrome(
        &title,
        html! {
            article {
                h1.page-title { (title) }
                @if parsed.frontmatter.forward_looking {
                    aside.fli-notice {
                        strong { "Forward-looking information." }
                        " Statements herein are subject to material assumptions and risks. "
                        "Per NI 51-102 / OSC SN 51-721 disclosure posture."
                    }
                }
                div.page-body {
                    (PreEscaped(body_html))
                }
            }
        },
    ))
}

async fn static_asset(Path(path): Path<String>) -> Response {
    match StaticAsset::get(&path) {
        Some(asset) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            let mut resp = asset.data.into_owned().into_response();
            if let Ok(value) = HeaderValue::from_str(mime.as_ref()) {
                resp.headers_mut().insert(header::CONTENT_TYPE, value);
            }
            resp
        }
        None => (StatusCode::NOT_FOUND, "not found").into_response(),
    }
}

fn chrome(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) " — PointSav Knowledge" }
                link rel="stylesheet" href="/static/style.css";
            }
            body {
                header.site-header {
                    a.site-title href="/" { "PointSav Knowledge" }
                    nav.site-nav {
                        a href="/" { "Index" }
                    }
                }
                main.site-main {
                    (body)
                }
                footer.site-footer {
                    p { "Engine: app-mediakit-knowledge — Phase 1 — see ARCHITECTURE.md" }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn fixture_state() -> (AppState, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        tokio::fs::write(
            dir.path().join("topic-test.md"),
            "---\ntitle: Test Topic\n---\n# Heading\n\nbody with [[Other]] link.\n",
        )
        .await
        .unwrap();
        (
            AppState {
                content_dir: dir.path().to_path_buf(),
            },
            dir,
        )
    }

    #[tokio::test]
    async fn healthz_responds_ok() {
        let (state, _dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(Request::builder().uri("/healthz").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn renders_known_page() {
        let (state, _dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/topic-test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("Test Topic"), "title should appear: {html}");
        assert!(html.contains("Heading"), "body heading should appear: {html}");
    }

    #[tokio::test]
    async fn returns_404_for_unknown_page() {
        let (state, _dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn rejects_path_traversal() {
        let (state, _dir) = fixture_state().await;
        let app = router(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/wiki/..%2Fetc%2Fpasswd")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
