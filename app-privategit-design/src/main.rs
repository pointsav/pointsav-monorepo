use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::{env, fs, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;

#[derive(Clone)]
struct AppState {
    vault: PathBuf,
    elements: Arc<Vec<String>>,
}

#[tokio::main]
async fn main() {
    let vault =
        PathBuf::from(env::var("DESIGN_VAULT").unwrap_or_else(|_| {
            "/srv/foundry/vendor/pointsav-design-system/elements/".to_string()
        }));
    let bind = env::var("DESIGN_BIND").unwrap_or_else(|_| "127.0.0.1:9094".to_string());

    let elements = Arc::new(read_elements(&vault));
    eprintln!(
        "app-privategit-design: {} element(s) from {:?}",
        elements.len(),
        vault
    );

    let state = AppState { vault, elements };
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(index))
        .route("/elements/:slug/overview", get(element_overview))
        .with_state(state)
        .layer(CompressionLayer::new());

    let listener = TcpListener::bind(&bind).await.expect("bind failed");
    eprintln!("app-privategit-design listening on {bind}");
    axum::serve(listener, app).await.expect("serve failed");
}

fn read_elements(vault: &PathBuf) -> Vec<String> {
    let Ok(dir) = fs::read_dir(vault) else {
        return Vec::new();
    };
    let mut slugs: Vec<String> = dir
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    slugs.sort();
    slugs
}

async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn index(State(state): State<AppState>) -> Html<String> {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>PointSav Design System</title>
<style>
body { font-family: system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 0 20px; }
h1 { font-size: 1.5rem; margin-bottom: 0.25rem; }
.count { color: #666; font-size: 0.875rem; margin-bottom: 1.5rem; }
ul { list-style: none; padding: 0; }
li { margin: 0.5rem 0; }
a { color: #0E3A66; text-decoration: none; font-weight: 500; }
a:hover { text-decoration: underline; }
</style>
</head>
<body>
<h1>PointSav Design System</h1>
"#,
    );

    if state.elements.is_empty() {
        html.push_str("<p class=\"count\">No elements found in vault.</p>");
    } else {
        html.push_str(&format!(
            "<p class=\"count\">{} element{}</p><ul>\n",
            state.elements.len(),
            if state.elements.len() == 1 { "" } else { "s" }
        ));
        for slug in state.elements.as_ref() {
            html.push_str(&format!(
                "  <li><a href=\"/elements/{slug}/overview\">{slug}</a></li>\n"
            ));
        }
        html.push_str("</ul>");
    }

    html.push_str("\n</body></html>");
    Html(html)
}

async fn element_overview(Path(slug): Path<String>, State(state): State<AppState>) -> Response {
    if slug.contains("..") || slug.contains('/') {
        return (StatusCode::BAD_REQUEST, "invalid slug").into_response();
    }

    let base = state.vault.join(&slug);
    let html_path = base.join("overview.html");
    let md_path = base.join("overview.md");

    if html_path.exists() {
        match fs::read_to_string(&html_path) {
            Ok(content) => Html(content).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "read error").into_response(),
        }
    } else if md_path.exists() {
        match fs::read_to_string(&md_path) {
            Ok(content) => Html(wrap_markdown(&slug, &content)).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "read error").into_response(),
        }
    } else if !state.elements.contains(&slug) {
        (StatusCode::NOT_FOUND, "element not found").into_response()
    } else {
        Html(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>{slug}</title>
<style>body{{font-family:system-ui,sans-serif;max-width:800px;margin:40px auto;padding:0 20px}}
a{{color:#0E3A66}}</style>
</head>
<body>
<p><a href="/">← Elements</a></p>
<h1>{slug}</h1>
<p><em>No overview file found in this element's vault directory.</em></p>
</body></html>"#
        ))
        .into_response()
    }
}

fn wrap_markdown(slug: &str, content: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>{slug}</title>
<style>
body {{ font-family: system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 0 20px; }}
pre {{ white-space: pre-wrap; word-break: break-word; background: #f5f5f5; padding: 1rem; border-radius: 4px; }}
a {{ color: #0E3A66; }}
</style>
</head>
<body>
<p><a href="/">← Elements</a></p>
<pre>{}</pre>
</body></html>"#,
        html_escape(content)
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
