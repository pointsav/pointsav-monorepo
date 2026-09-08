//! Real-world consistency check (BRIEF-tool-wiki plan Part 4 items 1-2): validate a wiki's
//! `categories.yaml` and `redirects.yaml` against its real crawled content — orphan
//! categories, dead redirects, and multi-hop redirect chains.
//!
//! Usage: `cargo run --example wiki_config_check -- <path-to-wiki-content-root>`

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let root = env::args().nth(1).expect("usage: wiki_config_check <path>");
    let root = Path::new(&root);

    let crawl = tool_wiki_core::crawl(root).expect("crawl failed");
    let known_slugs: HashSet<String> = crawl
        .documents
        .iter()
        .filter_map(|d| d.frontmatter.slug().map(str::to_string))
        .collect();
    let document_categories: HashSet<String> = crawl
        .documents
        .iter()
        .filter_map(|d| d.frontmatter.category().map(str::to_string))
        .collect();

    let categories = match fs::read_to_string(root.join("categories.yaml")) {
        Ok(yaml) => tool_wiki_core::parse_categories(&yaml).expect("parse categories.yaml"),
        Err(_) => {
            println!("no categories.yaml at this root -- skipping category checks");
            Vec::new()
        }
    };
    let known_category_ids: HashSet<String> = categories.iter().map(|c| c.id.clone()).collect();

    let redirects = match fs::read_to_string(root.join("redirects.yaml")) {
        Ok(yaml) => tool_wiki_core::parse_redirects(&yaml).expect("parse redirects.yaml"),
        Err(_) => {
            println!("no redirects.yaml at this root -- skipping redirect checks");
            Vec::new()
        }
    };

    println!(
        "crawled {} documents; {} categories; {} redirects\n",
        crawl.documents.len(),
        categories.len(),
        redirects.len()
    );

    let orphans = tool_wiki_core::find_orphan_categories(&categories, &document_categories);
    println!("=== orphan categories ({}) ===", orphans.len());
    for o in &orphans {
        println!("  {} -- {}", o.id, o.reason);
    }

    let dead = tool_wiki_core::find_dead_redirects(&redirects, &known_slugs, &known_category_ids);
    println!("\n=== dead redirects ({}) ===", dead.len());
    for d in &dead {
        println!("  {} -> {} -- {}", d.from, d.to, d.reason);
    }

    let chains = tool_wiki_core::find_redirect_chains(&redirects);
    println!("\n=== multi-hop redirect chains ({}) ===", chains.len());
    for c in &chains {
        println!("  {} -> {:?} -> {}", c.from, c.via, c.final_to);
    }
}
