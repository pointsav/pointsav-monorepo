use std::collections::HashMap;
use pulldown_cmark::{html, Options, Parser};

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
    out.push_str("</title>\n");
    out.push_str("<link rel=\"stylesheet\" href=\"/static/tokens.css\">\n");
    out.push_str("<link rel=\"stylesheet\" href=\"/static/portal.css\">\n");
    out.push_str("</head>\n<body>\n");
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
    out.push_str("</div>\n</div>\n</div>\n");
    // D4: SSE sidebar live-reload
    out.push_str("<script>
var _es=new EventSource('/sidebar/sse');
_es.onmessage=function(e){var n=document.querySelector('nav.sidebar');if(n)n.innerHTML=e.data;};
</script>\n");
    // D7: mobile drawer toggle (loaded after layout is ready)
    out.push_str("<script defer src=\"/static/drawer.js\"></script>\n");
    // D3: edit overlay (no-op on non-element pages; checks URL slug/tab)
    out.push_str("<script defer src=\"/static/edit.js\"></script>\n");
    out.push_str("</body>\n</html>");
    out
}
