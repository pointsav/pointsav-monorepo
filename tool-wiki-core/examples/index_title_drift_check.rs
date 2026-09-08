//! Category-title drift check (`BRIEF-tool-wiki-core.md` roadmap item): compares
//! `categories.yaml`'s `name:` field against each category's `_index.md` `title:`
//! frontmatter, and separately flags an `_index.es.md` whose `title:` is literally
//! identical to its English pair's -- a likely missed translation, not proof of one (this
//! is a bare string-identity check, not a semantic-equivalence judgment).
//!
//! This is the exact defect class found by hand twice in one session per
//! `BRIEF-tool-wiki-core.md`: a category's display name changed in one place (the taxonomy
//! file) but not the other (the rendered `_index.md` page), or vice versa.
//!
//! Usage: `cargo run --example index_title_drift_check -- <path-to-wiki-content-root>`

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let root = env::args().nth(1).expect("usage: index_title_drift_check <path>");
    let root = Path::new(&root);

    let crawl = tool_wiki_core::crawl(root).expect("crawl failed");

    let categories = match fs::read_to_string(root.join("categories.yaml")) {
        Ok(yaml) => tool_wiki_core::parse_categories(&yaml).expect("parse categories.yaml"),
        Err(_) => {
            println!("no categories.yaml at this root -- nothing to check against");
            return;
        }
    };

    println!(
        "crawled {} documents; {} categories\n",
        crawl.documents.len(),
        categories.len()
    );

    // --- 1. categories.yaml `name:` vs. English _index.md `title:` ---
    let drift = tool_wiki_core::find_category_title_drift(&categories, &crawl.documents);
    println!("=== categories.yaml `name:` vs. `_index.md` `title:` drift ({}) ===", drift.len());
    for d in &drift {
        match (&d.index_title, &d.index_path) {
            (Some(title), Some(path)) => println!(
                "  `{}` -- categories.yaml says \"{}\"; {} says \"{}\"",
                d.category_id, d.categories_yaml_name, path, title
            ),
            _ => println!(
                "  `{}` -- categories.yaml says \"{}\"; NO English _index.md found for this category",
                d.category_id, d.categories_yaml_name
            ),
        }
    }

    // --- 2. English vs. Spanish _index title identity (a likely missed translation) ---
    println!("\n=== `_index.md` vs. `_index.es.md` title identity ({} categories checked) ===", categories.len());
    let mut identical_count = 0usize;
    for c in &categories {
        let en = crawl.documents.iter().find(|d| {
            d.is_index_page()
                && d.frontmatter.category() == Some(c.id.as_str())
                && !d.path.to_string_lossy().ends_with(".es.md")
        });
        let es = crawl.documents.iter().find(|d| {
            d.is_index_page()
                && d.frontmatter.category() == Some(c.id.as_str())
                && d.path.to_string_lossy().ends_with(".es.md")
        });
        match (en, es) {
            (Some(en_doc), Some(es_doc)) => {
                let en_title = en_doc.frontmatter.title();
                let es_title = es_doc.frontmatter.title();
                if en_title.is_some() && en_title == es_title {
                    identical_count += 1;
                    println!(
                        "  `{}` -- {} and {} share the IDENTICAL title \"{}\" -- likely missed translation",
                        c.id,
                        en_doc.path.display(),
                        es_doc.path.display(),
                        en_title.unwrap()
                    );
                }
            }
            (Some(_), None) => {
                println!("  `{}` -- has an English _index.md but no _index.es.md pair", c.id);
            }
            (None, Some(_)) => {
                println!("  `{}` -- has an _index.es.md but no English _index.md pair", c.id);
            }
            (None, None) => {
                // Already reported as "no English _index.md" above; not repeated here.
            }
        }
    }
    println!("\ntotal identical EN/ES titles: {identical_count}");
}
