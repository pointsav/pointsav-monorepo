use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use pulldown_cmark::{html, Options, Parser};
use std::{collections::HashMap, env, fs, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::compression::CompressionLayer;

const SECTIONS: &[&str] = &["elements"];

#[derive(Clone)]
struct AppState {
    vault: PathBuf,
    nav: Arc<HashMap<String, Vec<String>>>,
}

fn discover_nav(vault: &std::path::Path) -> HashMap<String, Vec<String>> {
    let mut nav = HashMap::new();
    for section in SECTIONS {
        let dir = vault.join(section);
        if let Ok(entries) = fs::read_dir(&dir) {
            let mut slugs: Vec<String> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
                .filter_map(|e| e.file_name().into_string().ok())
                .collect();
            slugs.sort();
            if !slugs.is_empty() {
                nav.insert(section.to_string(), slugs);
            }
        }
    }
    nav
}

fn discover_tabs(vault: &std::path::Path, section: &str, slug: &str) -> Vec<String> {
    let dir = vault.join(section).join(slug);
    let Ok(entries) = fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut tabs: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter_map(|e| {
            let name = e.file_name().into_string().ok()?;
            if name.ends_with(".md") && !name.ends_with(".es.md") {
                Some(name[..name.len() - 3].to_string())
            } else {
                None
            }
        })
        .collect();
    tabs.sort();
    if let Some(pos) = tabs.iter().position(|t| t == "overview") {
        tabs.remove(pos);
        tabs.insert(0, "overview".to_string());
    }
    tabs
}

fn to_title(s: &str) -> String {
    s.split('-')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn render_markdown(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

fn render_nav(
    nav: &HashMap<String, Vec<String>>,
    active_section: &str,
    active_slug: &str,
) -> String {
    let mut out = String::new();
    for section in SECTIONS {
        let Some(slugs) = nav.get(*section) else {
            continue;
        };
        out.push_str("<div class=\"nav-section\">");
        out.push_str(&format!(
            "<span class=\"nav-section-title\">{}</span>",
            to_title(section)
        ));
        out.push_str("<ul>");
        for slug in slugs {
            let is_active = *section == active_section && slug == active_slug;
            let href = format!("/{}/{}/overview", section, slug);
            let class_attr = if is_active { " class=\"active\"" } else { "" };
            out.push_str(&format!(
                "<li><a href=\"{}\"{}>{}</a></li>",
                href,
                class_attr,
                to_title(slug)
            ));
        }
        out.push_str("</ul></div>");
    }
    out
}

fn render_tab_bar(section: &str, slug: &str, tabs: &[String], active_tab: &str) -> String {
    if tabs.len() <= 1 {
        return String::new();
    }
    let mut out = String::from("<nav class=\"tab-bar\">");
    for tab in tabs {
        let class_attr = if tab == active_tab {
            " class=\"active\""
        } else {
            ""
        };
        let href = format!("/{}/{}/{}", section, slug, tab);
        out.push_str(&format!(
            "<a href=\"{}\"{}>{}</a>",
            href,
            class_attr,
            to_title(tab)
        ));
    }
    out.push_str("</nav>");
    out
}

const CSS: &str = "
*,*::before,*::after{box-sizing:border-box;margin:0;padding:0}
body{font-family:'IBM Plex Sans',system-ui,sans-serif;font-size:16px;line-height:1.5;color:#161616;background:#fff;display:flex;flex-direction:column;min-height:100vh}
header{height:48px;background:#161616;color:#fff;display:flex;align-items:center;padding:0 1.5rem;flex-shrink:0;position:sticky;top:0;z-index:100}
header a{color:#fff;text-decoration:none;font-size:0.875rem;font-weight:600;letter-spacing:0.01em}
.layout{display:flex;flex:1;overflow:hidden}
.sidebar{width:256px;flex-shrink:0;background:#f4f4f4;border-right:1px solid #e0e0e0;overflow-y:auto;padding:1rem 0}
.nav-section{margin-bottom:0.5rem}
.nav-section-title{display:block;padding:0.5rem 1rem;font-size:0.6875rem;font-weight:600;letter-spacing:0.08em;text-transform:uppercase;color:#393939}
.sidebar ul{list-style:none}
.sidebar li a{display:block;padding:0.4rem 1rem 0.4rem 1.5rem;font-size:0.875rem;color:#161616;text-decoration:none;border-left:3px solid transparent}
.sidebar li a:hover{background:#e0e0e0;color:#0050e6}
.sidebar li a.active{border-left-color:#0e3a66;background:#e8f0f8;color:#0e3a66;font-weight:600}
.main{flex:1;overflow-y:auto;display:flex;flex-direction:column}
.tab-bar{display:flex;border-bottom:1px solid #e0e0e0;background:#fff;flex-shrink:0;padding:0 2rem}
.tab-bar a{display:inline-block;padding:0.75rem 1rem;font-size:0.875rem;color:#393939;text-decoration:none;border-bottom:3px solid transparent;margin-bottom:-1px}
.tab-bar a:hover{color:#0050e6}
.tab-bar a.active{color:#0e3a66;border-bottom-color:#0e3a66;font-weight:600}
.page-title{padding:2rem 2rem 1rem;font-size:1.75rem;font-weight:400;line-height:1.25;color:#161616;flex-shrink:0}
.content{padding:1rem 2rem 3rem;max-width:860px}
.content h1{font-size:1.75rem;font-weight:400;margin-bottom:1rem}
.content h2{font-size:1.25rem;font-weight:600;margin:2rem 0 0.75rem;padding-bottom:0.25rem;border-bottom:1px solid #e0e0e0}
.content h3{font-size:1rem;font-weight:600;margin:1.5rem 0 0.5rem}
.content p{margin-bottom:1rem}
.content ul,.content ol{padding-left:1.5rem;margin-bottom:1rem}
.content li{margin-bottom:0.25rem}
.content table{border-collapse:collapse;width:100%;margin-bottom:1.5rem;font-size:0.875rem}
.content th{background:#f4f4f4;text-align:left;padding:0.5rem 0.75rem;font-weight:600;border-bottom:2px solid #e0e0e0}
.content td{padding:0.5rem 0.75rem;border-bottom:1px solid #e0e0e0;vertical-align:top}
.content code{font-family:'IBM Plex Mono',monospace;font-size:0.875em;background:#f4f4f4;padding:0.1em 0.3em;border-radius:2px}
.content pre{background:#f4f4f4;padding:1rem;border-radius:4px;overflow-x:auto;margin-bottom:1rem;font-size:0.875rem}
.content pre code{background:none;padding:0}
.content a{color:#0e3a66}
.content figure{margin:0;display:inline-block}
.home-body{padding:4rem 2rem}
.home-body h1{font-size:2rem;font-weight:300;margin-bottom:1rem}
.home-body p{color:#393939}
";

fn shell(title: &str, nav_html: &str, tab_bar: &str, page_title: &str, content: &str) -> String {
    let mut out = String::new();
    out.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    out.push_str("<meta charset=\"utf-8\">\n");
    out.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    out.push_str("<title>");
    out.push_str(title);
    out.push_str("</title>\n<style>");
    out.push_str(CSS);
    out.push_str("</style>\n</head>\n<body>\n");
    out.push_str("<header><a href=\"/\">PointSav Design System</a></header>\n");
    out.push_str("<div class=\"layout\">\n<nav class=\"sidebar\">");
    out.push_str(nav_html);
    out.push_str("</nav>\n<div class=\"main\">\n");
    out.push_str(tab_bar);
    if !page_title.is_empty() {
        out.push_str("<h1 class=\"page-title\">");
        out.push_str(page_title);
        out.push_str("</h1>\n");
    }
    out.push_str("<div class=\"content\">");
    out.push_str(content);
    out.push_str("</div>\n</div>\n</div>\n</body>\n</html>");
    out
}

#[tokio::main]
async fn main() {
    let vault = PathBuf::from(
        env::var("DESIGN_VAULT_DIR")
            .or_else(|_| env::var("DESIGN_VAULT"))
            .unwrap_or_else(|_| "/srv/foundry/deployments/vault-privategit-design-1".to_string()),
    );
    let bind = env::var("DESIGN_BIND").unwrap_or_else(|_| "127.0.0.1:9094".to_string());

    let nav = Arc::new(discover_nav(&vault));
    eprintln!(
        "app-privategit-design: vault={:?} elements={:?}",
        vault,
        nav.get("elements").map(|v| v.len()).unwrap_or(0)
    );

    let state = AppState { vault, nav };
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/", get(index))
        .route("/elements/:slug", get(element_redirect))
        .route("/elements/:slug/:tab", get(element_tab))
        .with_state(state)
        .layer(CompressionLayer::new());

    let listener = TcpListener::bind(&bind).await.expect("bind failed");
    eprintln!("app-privategit-design listening on {}", bind);
    axum::serve(listener, app).await.expect("serve failed");
}

async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn index(State(state): State<AppState>) -> Html<String> {
    let nav_html = render_nav(&state.nav, "", "");
    let content = "<div class=\"home-body\"><h1>PointSav Design System</h1>\
                   <p>Select an element from the sidebar.</p></div>";
    Html(shell("PointSav Design System", &nav_html, "", "", content))
}

async fn element_redirect(Path(slug): Path<String>, State(state): State<AppState>) -> Response {
    if slug.contains("..") || slug.contains('/') {
        return (StatusCode::BAD_REQUEST, "invalid").into_response();
    }
    let tabs = discover_tabs(&state.vault, "elements", &slug);
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
    let tabs = discover_tabs(&state.vault, "elements", &slug);
    if tabs.is_empty() {
        return (StatusCode::NOT_FOUND, "element not found").into_response();
    }
    let md_path = state
        .vault
        .join("elements")
        .join(&slug)
        .join(format!("{}.md", tab));
    let md = match fs::read_to_string(&md_path) {
        Ok(s) => s,
        Err(_) => return (StatusCode::NOT_FOUND, "tab not found").into_response(),
    };
    let content = render_markdown(&md);
    let nav_html = render_nav(&state.nav, "elements", &slug);
    let tab_bar = render_tab_bar("elements", &slug, &tabs, &tab);
    let label = to_title(&slug);
    Html(shell(
        &format!("{} — PointSav Design System", label),
        &nav_html,
        &tab_bar,
        &label,
        &content,
    ))
    .into_response()
}
