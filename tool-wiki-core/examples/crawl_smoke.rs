//! Smoke test (BRIEF-tool-wiki plan Phase 1 verification): crawl a real wiki content tree
//! and print a summary, so the output can be checked by hand against a few known articles.
//!
//! Usage: `cargo run --example crawl_smoke -- <path-to-wiki-content-root>`

use std::env;
use std::path::Path;

fn main() {
    let root = env::args().nth(1).expect("usage: crawl_smoke <path>");
    let result = tool_wiki_core::crawl(Path::new(&root)).expect("crawl failed");

    println!(
        "crawled {}: {} documents, {} skipped (non-article .md files)",
        root,
        result.documents.len(),
        result.skipped.len()
    );

    let mut total_headings = 0usize;
    let mut total_wikilinks = 0usize;
    let mut total_auto_blocks = 0usize;
    let mut total_tables = 0usize;
    let mut total_code_blocks = 0usize;
    for doc in &result.documents {
        total_headings += doc.headings.len();
        total_wikilinks += doc.wikilinks.len();
        total_auto_blocks += doc.auto_generated_blocks.len();
        total_tables += doc.tables.len();
        total_code_blocks += doc.code_blocks.len();
    }
    println!(
        "totals — headings: {total_headings}, wikilinks: {total_wikilinks}, \
         auto-generated blocks: {total_auto_blocks}, tables: {total_tables}, \
         code blocks: {total_code_blocks}"
    );

    println!("\nfirst 5 documents (path — title — headings/wikilinks):");
    for doc in result.documents.iter().take(5) {
        println!(
            "  {} — {:?} — {}h/{}w",
            doc.path.display(),
            doc.frontmatter.title().unwrap_or("<no title>"),
            doc.headings.len(),
            doc.wikilinks.len()
        );
    }

    println!("\nfirst 5 skipped (path — reason):");
    for (path, err) in result.skipped.iter().take(5) {
        println!("  {} — {err}", path.display());
    }
}
