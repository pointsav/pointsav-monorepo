//! Inverse-orphan check (`BRIEF-tool-wiki-core.md` roadmap item): find every document whose
//! frontmatter `category:` value does NOT exist as an `id:` in `categories.yaml`.
//!
//! `wiki_config_check` already checks the other direction (a `categories.yaml` entry with
//! no matching document -- an empty shelf). This is the inverse: a document referencing a
//! category id that was renamed or retired in the taxonomy file without updating the
//! article -- a link that resolves nowhere on the live site.
//!
//! `category: root` (and an empty `category:`) are reserved engine sentinels, not taxonomy
//! ids, and are excluded -- see `tool_wiki_core::RESERVED_CATEGORY_IDS` for the two live
//! engine call sites this mirrors. Before that exclusion this check reported every
//! site-root page on every wiki as a defect (24 false positives across the three).
//!
//! Usage: `cargo run --example orphan_category_check -- <path-to-wiki-content-root>`

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let root = env::args().nth(1).expect("usage: orphan_category_check <path>");
    let root = Path::new(&root);

    let crawl = tool_wiki_core::crawl(root).expect("crawl failed");

    let categories = match fs::read_to_string(root.join("categories.yaml")) {
        Ok(yaml) => tool_wiki_core::parse_categories(&yaml).expect("parse categories.yaml"),
        Err(_) => {
            println!("no categories.yaml at this root -- nothing to check against");
            return;
        }
    };
    let known_category_ids: HashSet<String> = categories.iter().map(|c| c.id.clone()).collect();

    println!(
        "crawled {} documents; {} known category ids\n",
        crawl.documents.len(),
        known_category_ids.len()
    );

    let reserved_docs = crawl
        .documents
        .iter()
        .filter(|d| {
            d.frontmatter
                .category()
                .map(tool_wiki_core::is_reserved_category)
                .unwrap_or(false)
        })
        .count();

    let unknown = tool_wiki_core::find_unknown_categories(&crawl.documents, &known_category_ids);
    println!("=== documents with an unknown `category:` value ({}) ===", unknown.len());
    for u in &unknown {
        println!("  {} -- category `{}` has no matching id in categories.yaml", u.path, u.category);
    }
    println!(
        "  ({reserved_docs} documents carry a reserved sentinel category {:?} and are \
         excluded by design)",
        tool_wiki_core::RESERVED_CATEGORY_IDS
    );
}
