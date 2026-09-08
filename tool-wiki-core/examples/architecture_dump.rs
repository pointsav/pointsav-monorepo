//! One-off dump for Track-B Phase A assessment of the projects wiki `architecture/` category.
//! Usage: `cargo run --example architecture_dump -- <path-to-wiki-content-root>`

use std::env;
use std::path::Path;

fn main() {
    let root = env::args().nth(1).expect("usage: architecture_dump <path>");
    let result = tool_wiki_core::crawl(Path::new(&root)).expect("crawl failed");

    let mut docs: Vec<_> = result
        .documents
        .iter()
        .filter(|d| d.path.to_string_lossy().starts_with("architecture/"))
        .collect();
    docs.sort_by_key(|d| d.path.clone());

    println!("architecture/ documents: {}", docs.len());
    println!();

    for doc in &docs {
        let fm = &doc.frontmatter;
        let h2_count = doc
            .headings
            .iter()
            .filter(|h| h.level == 2 && h.text != "See also")
            .count();
        println!(
            "{:60} | words={:>5} | H2(excl. See also)={:>2} | wikilinks={:>2} | category={:?} \
             content_type={:?} quality={:?} bcsc_class={:?} audience={:?} index_type={:?} index_group={:?} short_description={:?}",
            doc.path.file_name().unwrap().to_string_lossy(),
            doc.word_count_excluding_code(),
            h2_count,
            doc.wikilinks.len(),
            fm.get_str("category"),
            fm.get_str("content_type"),
            fm.get_str("quality"),
            fm.get_str("bcsc_class"),
            fm.get_str("audience"),
            fm.get_str("index_type"),
            fm.get_str("index_group"),
            fm.get_str("short_description"),
        );
    }

    println!("\nfield presence check (required TOPIC fields):");
    let required = [
        "title", "slug", "category", "type", "content_type", "quality", "status",
        "audience", "bcsc_class", "language_protocol", "last_edited", "editor",
        "short_description", "paired_with",
    ];
    for doc in &docs {
        let missing: Vec<&str> = required
            .iter()
            .filter(|f| doc.frontmatter.get_str(f).is_none())
            .copied()
            .collect();
        if !missing.is_empty() {
            println!(
                "  {} missing: {:?}",
                doc.path.file_name().unwrap().to_string_lossy(),
                missing
            );
        }
    }
}
