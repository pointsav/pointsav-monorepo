//! Per-category health census (`BRIEF-tool-wiki-core.md` roadmap item): article counts per
//! category, `quality:` frontmatter distribution, EN/ES pairing gaps, orphan articles (zero
//! inbound wikilinks from any other article in the crawl), and `category:` frontmatter vs.
//! top-level directory mismatches.
//!
//! Usage: `cargo run --example category_census -- <path-to-wiki-content-root>`

use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;

fn main() {
    let root = env::args().nth(1).expect("usage: category_census <path>");
    let root = Path::new(&root);

    let crawl = tool_wiki_core::crawl(root).expect("crawl failed");
    let docs = &crawl.documents;
    println!("crawled {} documents ({} skipped non-article files)\n", docs.len(), crawl.skipped.len());

    // --- 1. Per-category article counts ---
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for d in docs {
        if let Some(cat) = d.frontmatter.category() {
            *counts.entry(cat).or_insert(0) += 1;
        }
    }
    let mut counts_sorted: Vec<(&str, usize)> = counts.into_iter().collect();
    counts_sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("=== per-category article counts ({} categories referenced) ===", counts_sorted.len());
    for (cat, n) in &counts_sorted {
        println!("  {cat:<32} {n}");
    }

    // --- 2. `quality:` frontmatter distribution ---
    let mut quality: HashMap<&str, usize> = HashMap::new();
    for d in docs {
        let q = d.frontmatter.get_str("quality").unwrap_or("<none>");
        *quality.entry(q).or_insert(0) += 1;
    }
    let mut quality_sorted: Vec<(&str, usize)> = quality.into_iter().collect();
    quality_sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("\n=== `quality:` frontmatter distribution ===");
    for (q, n) in &quality_sorted {
        println!("  {q:<16} {n}");
    }

    // --- 3. EN/ES pairing gaps (excluding .archive/) ---
    let path_set: HashSet<String> =
        docs.iter().map(|d| d.path.to_string_lossy().replace('\\', "/")).collect();
    let mut unpaired_en: Vec<String> = Vec::new();
    let mut unpaired_es: Vec<String> = Vec::new();
    for d in docs {
        let path_str = d.path.to_string_lossy().replace('\\', "/");
        if path_str.contains(".archive/") || path_str.starts_with(".archive/") {
            continue;
        }
        if let Some(stem) = path_str.strip_suffix(".es.md") {
            let en_path = format!("{stem}.md");
            if !path_set.contains(&en_path) {
                unpaired_es.push(path_str.clone());
            }
        } else if let Some(stem) = path_str.strip_suffix(".md") {
            let es_path = format!("{stem}.es.md");
            if !path_set.contains(&es_path) {
                unpaired_en.push(path_str.clone());
            }
        }
    }
    unpaired_en.sort();
    unpaired_es.sort();
    println!("\n=== EN articles with no `.es.md` pair ({}) ===", unpaired_en.len());
    for p in &unpaired_en {
        println!("  {p}");
    }
    println!("\n=== ES articles with no English `.md` pair ({}) ===", unpaired_es.len());
    for p in &unpaired_es {
        println!("  {p}");
    }

    // --- 4. Orphan articles: zero inbound wikilinks from any other article ---
    // Build slug -> set of paths that link to it, in one pass, to avoid an O(n^2) scan.
    let mut inbound: HashMap<String, HashSet<String>> = HashMap::new();
    for d in docs {
        let from = d.path.to_string_lossy().replace('\\', "/");
        for w in &d.wikilinks {
            inbound.entry(w.target.clone()).or_default().insert(from.clone());
        }
    }
    let mut orphans: Vec<String> = Vec::new();
    for d in docs {
        if d.is_index_page() {
            continue; // index/MOC pages are entry points, not expected to have inbound links
        }
        let Some(slug) = d.frontmatter.slug() else { continue };
        let self_path = d.path.to_string_lossy().replace('\\', "/");
        let has_real_inbound = inbound
            .get(slug)
            .map(|linkers| linkers.iter().any(|p| p != &self_path))
            .unwrap_or(false);
        if !has_real_inbound {
            orphans.push(self_path);
        }
    }
    orphans.sort();
    println!("\n=== orphan articles: zero inbound wikilinks from another article ({}) ===", orphans.len());
    for p in &orphans {
        println!("  {p}");
    }

    // --- 5. `category:` frontmatter vs. top-level directory mismatches ---
    let mut dir_mismatches: Vec<(String, String, String)> = Vec::new();
    for d in docs {
        let Some(cat) = d.frontmatter.category() else { continue };
        // A root-level file (no parent directory at all -- e.g. `about.md`) has no
        // directory to compare against; only check documents actually nested one level
        // deep, using that first path component as "the category's directory".
        if d.path.parent().map(|p| p.as_os_str().is_empty()).unwrap_or(true) {
            continue;
        }
        let Some(top_dir) = d.path.components().next().and_then(|c| c.as_os_str().to_str())
        else {
            continue;
        };
        if top_dir != cat && !top_dir.starts_with('.') {
            dir_mismatches.push((d.path.display().to_string(), cat.to_string(), top_dir.to_string()));
        }
    }
    dir_mismatches.sort();
    println!("\n=== `category:` frontmatter vs. top-level directory mismatches ({}) ===", dir_mismatches.len());
    for (path, cat, dir) in &dir_mismatches {
        println!("  {path} -- category: `{cat}`, lives under `{dir}/`");
    }
}
