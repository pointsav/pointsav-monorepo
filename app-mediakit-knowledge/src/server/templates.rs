use crate::renderer::RenderedArticle;
use crate::search::SearchResult;
use anyhow::Result;
use once_cell::sync::Lazy;
use tera::Tera;

static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut t = Tera::new("templates/**/*.html").unwrap_or_else(|e| {
        eprintln!("Template error: {e}");
        std::process::exit(1);
    });
    t.autoescape_on(vec!["html"]);
    t
});

pub fn render_article(
    article: &RenderedArticle,
    site_title: &str,
    editor_enabled: bool,
    current_url: &str,
) -> Result<String> {
    let mut ctx = tera::Context::new();
    ctx.insert("article",        article);
    ctx.insert("site_title",     site_title);
    ctx.insert("editor_enabled", &editor_enabled);
    ctx.insert("current_url",    current_url);
    Ok(TEMPLATES.render("article.html", &ctx)?)
}

pub fn render_search(query: &str, results: &[SearchResult], site_title: &str) -> Result<String> {
    let mut ctx = tera::Context::new();
    ctx.insert("query",        query);
    ctx.insert("results",      results);
    ctx.insert("result_count", &results.len());
    ctx.insert("site_title",   site_title);
    Ok(TEMPLATES.render("search.html", &ctx)?)
}

pub fn render_editor(
    slug: &str,
    section_heading: Option<&str>,
    section_markdown: &str,
    base_sha: &str,
    site_title: &str,
    conflict_message: Option<&str>,
) -> Result<String> {
    let mut ctx = tera::Context::new();
    ctx.insert("slug",             slug);
    ctx.insert("section_heading",  &section_heading);
    ctx.insert("section_markdown", section_markdown);
    ctx.insert("base_sha",         base_sha);
    ctx.insert("site_title",       site_title);
    ctx.insert("conflict_message", &conflict_message);
    Ok(TEMPLATES.render("editor.html", &ctx)?)
}

pub fn render_category(
    category_title: &str,
    body_html: &str,
    articles: &[serde_json::Value],
    toc: &[crate::renderer::toc::TocEntry],
    site_title: &str,
) -> Result<String> {
    let mut ctx = tera::Context::new();
    ctx.insert("category_title", category_title);
    ctx.insert("body_html",      body_html);
    ctx.insert("articles",       articles);
    ctx.insert("toc",            toc);
    ctx.insert("site_title",     site_title);
    Ok(TEMPLATES.render("category.html", &ctx)?)
}
