use std::collections::HashMap;
use pulldown_cmark::{html, Options, Parser};

// Carbon Design System tokens as CSS custom properties (D6).
// Consuming rules use var(--cds-*); future themes override these root values.
const CSS: &str = "
:root{
  --cds-background:#fff;
  --cds-layer:#f4f4f4;
  --cds-border-subtle:#e0e0e0;
  --cds-border-strong:#8d8d8d;
  --cds-text-primary:#161616;
  --cds-text-secondary:#393939;
  --cds-text-tertiary:#525252;
  --cds-link-primary:#0e3a66;
  --cds-interactive:#0050e6;
  --cds-focus:#0f62fe;
  --cds-background-brand:#161616;
  --cds-selected-background:#e8f0f8;
  --cds-selected-text:#0e3a66;
  --cds-sidebar-bg:#f4f4f4;
}
*,*::before,*::after{box-sizing:border-box;margin:0;padding:0}
body{font-family:'IBM Plex Sans',system-ui,sans-serif;font-size:16px;line-height:1.5;color:var(--cds-text-primary);background:var(--cds-background);display:flex;flex-direction:column;min-height:100vh}
header{height:48px;background:var(--cds-background-brand);color:#fff;display:flex;align-items:center;padding:0 1.5rem;flex-shrink:0;position:sticky;top:0;z-index:100}
header a{color:#fff;text-decoration:none;font-size:0.875rem;font-weight:600;letter-spacing:0.01em}
.layout{display:flex;flex:1;overflow:hidden}
.sidebar{width:256px;flex-shrink:0;background:var(--cds-sidebar-bg);border-right:1px solid var(--cds-border-subtle);overflow-y:auto;padding:1rem 0}
.nav-section{margin-bottom:0.5rem}
.nav-section-title{display:block;padding:0.5rem 1rem;font-size:0.6875rem;font-weight:600;letter-spacing:0.08em;text-transform:uppercase;color:var(--cds-text-secondary)}
.sidebar ul{list-style:none}
.sidebar li a{display:block;padding:0.4rem 1rem 0.4rem 1.5rem;font-size:0.875rem;color:var(--cds-text-primary);text-decoration:none;border-left:3px solid transparent}
.sidebar li a:hover{background:var(--cds-border-subtle);color:var(--cds-interactive)}
.sidebar li a.active{border-left-color:var(--cds-selected-text);background:var(--cds-selected-background);color:var(--cds-selected-text);font-weight:600}
.main{flex:1;overflow-y:auto;display:flex;flex-direction:column}
.tab-bar{display:flex;border-bottom:1px solid var(--cds-border-subtle);background:var(--cds-background);flex-shrink:0;padding:0 2rem}
.tab-bar a{display:inline-block;padding:0.75rem 1rem;font-size:0.875rem;color:var(--cds-text-secondary);text-decoration:none;border-bottom:3px solid transparent;margin-bottom:-1px}
.tab-bar a:hover{color:var(--cds-interactive)}
.tab-bar a.active{color:var(--cds-selected-text);border-bottom-color:var(--cds-selected-text);font-weight:600}
.page-title{padding:2rem 2rem 1rem;font-size:1.75rem;font-weight:400;line-height:1.25;color:var(--cds-text-primary);flex-shrink:0}
.content{padding:1rem 2rem 3rem;max-width:860px}
.content h1{font-size:1.75rem;font-weight:400;margin-bottom:1rem}
.content h2{font-size:1.25rem;font-weight:600;margin:2rem 0 0.75rem;padding-bottom:0.25rem;border-bottom:1px solid var(--cds-border-subtle)}
.content h3{font-size:1rem;font-weight:600;margin:1.5rem 0 0.5rem}
.content p{margin-bottom:1rem}
.content ul,.content ol{padding-left:1.5rem;margin-bottom:1rem}
.content li{margin-bottom:0.25rem}
.content table{border-collapse:collapse;width:100%;margin-bottom:1.5rem;font-size:0.875rem}
.content th{background:var(--cds-layer);text-align:left;padding:0.5rem 0.75rem;font-weight:600;border-bottom:2px solid var(--cds-border-subtle)}
.content td{padding:0.5rem 0.75rem;border-bottom:1px solid var(--cds-border-subtle);vertical-align:top}
.content code{font-family:'IBM Plex Mono',monospace;font-size:0.875em;background:var(--cds-layer);padding:0.1em 0.3em;border-radius:2px}
.content pre{background:var(--cds-layer);padding:1rem;border-radius:4px;overflow-x:auto;margin-bottom:1rem;font-size:0.875rem}
.content pre code{background:none;padding:0}
.content a{color:var(--cds-link-primary)}
.content figure{margin:0;display:inline-block}
.home-body{padding:4rem 2rem}
.home-body h1{font-size:2rem;font-weight:300;margin-bottom:1rem}
.home-body p{color:var(--cds-text-secondary)}
/* Schema badge + metadata panel (D2) */
.schema-badge{display:inline-block;padding:0.125rem 0.5rem;font-size:0.6875rem;font-weight:600;letter-spacing:0.04em;text-transform:uppercase;border-radius:2px;margin-bottom:1.25rem}
.schema-badge--component{background:var(--cds-selected-background);color:var(--cds-selected-text)}
.schema-badge--token{background:#defbe6;color:#0e6027}
.schema-badge--research{background:#fff5e8;color:#a56400}
.schema-badge--bundle{background:#f2e8ff;color:#6929c4}
.schema-meta{display:grid;grid-template-columns:8rem 1fr;gap:0.375rem 1rem;margin-bottom:1.5rem;font-size:0.875rem;border:1px solid var(--cds-border-subtle);border-radius:4px;padding:0.75rem 1rem;background:var(--cds-layer)}
.schema-meta dt{color:var(--cds-text-secondary);font-weight:600}
.schema-meta dd{color:var(--cds-text-primary)}
.schema-reserved{color:var(--cds-text-tertiary);font-style:italic}
/* Mobile responsive layout (D7) */
@media(max-width:672px){
  .layout{flex-direction:column}
  .sidebar{width:100%;height:auto;max-height:200px;border-right:none;border-bottom:1px solid var(--cds-border-subtle)}
  .tab-bar{padding:0 1rem}
  .content{padding:1rem 1rem 3rem}
  .page-title{padding:1.5rem 1rem 0.75rem}
}
";

pub fn render_markdown(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

pub fn render_nav(
    nav: &HashMap<String, Vec<String>>,
    sections: &[&str],
    active_section: &str,
    active_slug: &str,
) -> String {
    let mut out = String::new();
    for section in sections {
        let Some(slugs) = nav.get(*section) else {
            continue;
        };
        out.push_str("<div class=\"nav-section\">");
        out.push_str(&format!(
            "<span class=\"nav-section-title\">{}</span>",
            crate::vault::to_title(section)
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
                crate::vault::to_title(slug)
            ));
        }
        out.push_str("</ul></div>");
    }
    out
}

pub fn render_tab_bar(section: &str, slug: &str, tabs: &[String], active_tab: &str) -> String {
    if tabs.len() <= 1 {
        return String::new();
    }
    let mut out = String::from("<nav class=\"tab-bar\">");
    for tab in tabs {
        let class_attr = if tab == active_tab { " class=\"active\"" } else { "" };
        let href = format!("/{}/{}/{}", section, slug, tab);
        out.push_str(&format!(
            "<a href=\"{}\"{}>{}</a>",
            href,
            class_attr,
            crate::vault::to_title(tab)
        ));
    }
    out.push_str("</nav>");
    out
}

pub fn shell(title: &str, nav_html: &str, tab_bar: &str, page_title: &str, content: &str) -> String {
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
