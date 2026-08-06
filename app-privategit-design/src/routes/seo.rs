// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.


use crate::{state::AppState, vault};
use axum::{extract::State, http::header, response::IntoResponse};

pub async fn robots_txt(State(state): State<AppState>) -> impl IntoResponse {
    let body = format!(
        "User-agent: *\nAllow: /\nDisallow: /research/\nSitemap: {}/sitemap.xml\n",
        state.site_origin
    );
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], body)
}

/// GET /llms.txt — generated from the same route table sitemap.xml uses (`vault::
/// SECTIONS` + `state.nav`'s discovered slugs), not hand-authored, so it can't drift
/// stale the way `static/tokens.css` once did. Format per the llms.txt convention: H1
/// title, a one-paragraph summary blockquote, then markdown link sections.
pub async fn llms_txt(State(state): State<AppState>) -> impl IntoResponse {
    let origin = &state.site_origin;
    let mut body = String::new();
    body.push_str("# PointSav Design System\n\n");
    body.push_str(
        "> Self-hostable, DTCG-native design-token system. Tokens, component recipes, \
         and research decisions live in one versioned source — machine-readable \
         directly by AI agents, not just documented for humans.\n\n",
    );

    body.push_str("## Machine-readable endpoints\n\n");
    body.push_str(&format!(
        "- [Token bundle]({origin}/bundles/tokens/tokens.full.json): full DTCG token \
         graph (`application/design-tokens+json`)\n"
    ));
    body.push_str(&format!(
        "- [MCP endpoint]({origin}/mcp): JSON-RPC 2.0 — tools: get_token, \
         list_components, get_component_recipe, search_design_system, \
         list_token_families\n"
    ));
    body.push_str(&format!(
        "- [Token search]({origin}/tokens/search): full-text search across tokens and \
         components\n"
    ));
    body.push_str(&format!(
        "- [Component recipes]({origin}/components/:slug/recipe.json): per-component \
         HTML+CSS+ARIA recipe, machine-readable\n\n"
    ));

    body.push_str("## Documentation\n\n");
    body.push_str(&format!("- [Tokens]({origin}/tokens)\n"));
    body.push_str(&format!(
        "- [Get started self-hosting]({origin}/developing/install/overview)\n"
    ));
    body.push_str(&format!(
        "- [MCP integration]({origin}/developing/mcp/overview)\n\n"
    ));

    for (section, default_tab, _) in vault::SECTIONS {
        if !vault::is_publicly_reachable(section) {
            continue;
        }
        let Some(slugs) = state.nav.get(*section) else {
            continue;
        };
        if slugs.is_empty() {
            continue;
        }
        body.push_str(&format!("## {}\n\n", title_case(section)));
        for slug in slugs {
            let tabs = vault::discover_tabs(&state.vault, section, slug);
            let tab = if tabs.iter().any(|t| t == default_tab) {
                (*default_tab).to_string()
            } else if let Some(first) = tabs.first() {
                first.clone()
            } else {
                continue;
            };
            body.push_str(&format!("- [{}]({origin}/{section}/{slug}/{tab})\n", slug));
        }
        body.push('\n');
    }

    (
        [(header::CONTENT_TYPE, "text/markdown; charset=utf-8")],
        body,
    )
}

fn title_case(s: &str) -> String {
    s.split('-')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn push_url(body: &mut String, site_origin: &str, path: &str) {
    body.push_str("  <url><loc>");
    body.push_str(site_origin);
    body.push_str(path);
    body.push_str("</loc></url>\n");
}

/// GET /sitemap.xml — generated from the real route table (`vault::SECTIONS` +
/// `state.nav`'s discovered slugs + `vault::discover_tabs`' discovered tabs), not a
/// hand-maintained list — stays correct as vault content is added or removed.
pub async fn sitemap_xml(State(state): State<AppState>) -> impl IntoResponse {
    let mut body = String::new();
    body.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    body.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    push_url(&mut body, &state.site_origin, "/");
    push_url(&mut body, &state.site_origin, "/es");
    push_url(&mut body, &state.site_origin, "/tokens");
    push_url(&mut body, &state.site_origin, "/adoption");

    for (section, _, _) in vault::SECTIONS {
        if !vault::is_publicly_reachable(section) {
            continue;
        }
        let Some(slugs) = state.nav.get(*section) else {
            continue;
        };
        for slug in slugs {
            for tab in vault::discover_tabs(&state.vault, section, slug) {
                push_url(
                    &mut body,
                    &state.site_origin,
                    &format!("/{section}/{slug}/{tab}"),
                );
            }
        }
    }

    body.push_str("</urlset>\n");
    (
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        body,
    )
}
