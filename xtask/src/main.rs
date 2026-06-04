//! xtask — build-time content gate (cargo xtask check-content).
//!
//! Usage:
//!   cargo xtask check-content <path1> [path2] ...
//!
//! Walks all Markdown files in the given mount paths, collects every slug,
//! then checks all [[wikilinks]] against the slug set. Reports dead links
//! and missing required frontmatter fields. Exits 1 if any dead links or
//! missing required fields are found.
//!
//! This is the Phase 5 hard-promote gate (L18 + L29): an unresolved [[slug]]
//! across the mount set BLOCKS promote. The gate must pass before
//! `wikilink-missing` render is removed from the live sites.
//!
//! Implementation note: this xtask duplicates the dead-link + frontmatter
//! logic from `app-mediakit-knowledge/src/check.rs` inline rather than
//! importing it, to avoid a circular or cross-crate dependency at build time.

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // First arg is the binary name; remaining args are mount paths.
    let subcommand = args.get(1).map(|s| s.as_str());
    let paths: Vec<PathBuf> = if subcommand == Some("check-content") {
        args[2..].iter().map(PathBuf::from).collect()
    } else if args.len() > 1 && subcommand != Some("check-content") {
        // Support bare invocation: cargo xtask <path1> [path2] ...
        args[1..].iter().map(PathBuf::from).collect()
    } else {
        eprintln!("Usage: cargo xtask check-content <path1> [path2] ...");
        eprintln!("       cargo xtask <path1> [path2] ...");
        std::process::exit(2);
    };

    if paths.is_empty() {
        eprintln!("error: at least one content path is required");
        std::process::exit(2);
    }

    // Validate that all paths exist.
    for p in &paths {
        if !p.is_dir() {
            eprintln!("error: not a directory: {}", p.display());
            std::process::exit(2);
        }
    }

    let (dead_links, missing_fields, total) = check_content(&paths);

    println!("checked {} articles across {} mount(s)", total, paths.len());
    for (source, target) in &dead_links {
        println!("DEAD LINK   {} -> [[{}]]", source, target);
    }
    for (slug, field) in &missing_fields {
        println!("MISSING     {}: {}", slug, field);
    }
    println!(
        "{} dead link(s), {} article(s) with missing required frontmatter fields",
        dead_links.len(),
        missing_fields.len()
    );

    if !dead_links.is_empty() || !missing_fields.is_empty() {
        std::process::exit(1);
    }
}

/// Run the content gate over a set of mount paths.
///
/// Returns `(dead_links, missing_fields, total_articles)`.
///
/// `dead_links` is a list of `(source_slug, target_slug)` pairs.
/// `missing_fields` is a list of `(slug, field_name)` pairs.
fn check_content(paths: &[PathBuf]) -> (Vec<(String, String)>, Vec<(String, String)>, usize) {
    // Collect all article files across all mounts.
    let mut all_files: Vec<(PathBuf, String)> = Vec::new(); // (path, slug)
    for base in paths {
        collect_md_files(base, base, &mut all_files);
    }

    // Build the slug set for dead-link resolution.
    let slug_set: HashSet<String> = all_files.iter().map(|(_, slug)| slug.clone()).collect();

    let mut dead_links: Vec<(String, String)> = Vec::new();
    let mut missing_fields: Vec<(String, String)> = Vec::new();
    let total = all_files.len();

    // Required frontmatter fields for the built-in `topic` and `guide` blueprints.
    let required_topic = ["title", "slug"];
    let required_guide = ["title", "slug"];

    for (path, slug) in &all_files {
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let (yaml, body) = split_frontmatter(&text);

        // Dead-link gate.
        for target in parse_wikilinks(&body) {
            if is_non_page_target(&target) {
                continue;
            }
            // Normalise: lowercase, spaces→hyphens (mirrors render.rs behaviour).
            let normalised = normalise_slug(&target);
            if !slug_set.contains(&normalised) {
                dead_links.push((slug.clone(), target));
            }
        }

        // Blueprint frontmatter validation.
        if let Some(map) = yaml.and_then(|y| {
            serde_yaml_simple_parse(y)
        }) {
            let content_type = map
                .get("type")
                .or_else(|| map.get("content_type"))
                .map(|s| s.as_str())
                .unwrap_or("topic");
            let required = match content_type {
                "guide" => &required_guide[..],
                _ => &required_topic[..],
            };
            for &field in required {
                if !map.contains_key(field) {
                    missing_fields.push((slug.clone(), field.to_string()));
                }
            }
        }
    }

    (dead_links, missing_fields, total)
}

/// Recursively collect all `.md` files under `dir`, computing their slugs
/// relative to `base`.
fn collect_md_files(dir: &Path, base: &Path, out: &mut Vec<(PathBuf, String)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_md_files(&path, base, out);
        } else if path.extension().map(|e| e == "md").unwrap_or(false) {
            let slug = path
                .strip_prefix(base)
                .ok()
                .and_then(|r| r.to_str())
                .map(|s| s.trim_end_matches(".md").replace('\\', "/"))
                .unwrap_or_default();
            out.push((path, slug));
        }
    }
}

/// Split raw Markdown text into `(Option<yaml_block>, body)`.
fn split_frontmatter(text: &str) -> (Option<&str>, String) {
    if let Some(rest) = text.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---\n") {
            return (Some(&rest[..end]), rest[end + "\n---\n".len()..].to_string());
        }
    }
    (None, text.to_string())
}

/// Minimal YAML parser: extract key: value pairs from a flat YAML block.
/// Does not handle nested structures — only the top-level scalar fields
/// needed for frontmatter validation.
fn serde_yaml_simple_parse(yaml: &str) -> Option<BTreeMap<String, String>> {
    let mut map = BTreeMap::new();
    for line in yaml.lines() {
        if let Some((key, val)) = line.split_once(':') {
            let key = key.trim().to_string();
            let val = val.trim().trim_matches('"').trim_matches('\'').to_string();
            if !key.is_empty() && !key.starts_with('-') && !key.starts_with('#') {
                map.insert(key, val);
            }
        }
    }
    if map.is_empty() { None } else { Some(map) }
}

/// Extract `[[wikilink]]` targets from Markdown body text.
fn parse_wikilinks(body: &str) -> Vec<String> {
    let mut targets = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find("[[") {
        rest = &rest[start + 2..];
        if let Some(end) = rest.find("]]") {
            let inner = rest[..end].trim();
            // Support `[[Display Text|slug]]` and `[[slug]]` forms.
            let target = if let Some((_, slug)) = inner.split_once('|') {
                slug.trim()
            } else {
                inner
            };
            targets.push(target.to_string());
            rest = &rest[end + 2..];
        } else {
            break;
        }
    }
    targets
}

/// Return true for wikilink targets that are not wiki article slugs.
fn is_non_page_target(target: &str) -> bool {
    target.starts_with("/category/")
        || target.contains("://")
        || target.starts_with("http")
        || target.starts_with('#')
}

/// Normalise a wikilink target to a slug: lowercase, spaces→hyphens.
/// Mirrors the normalisation applied in `render.rs`.
fn normalise_slug(target: &str) -> String {
    target
        .to_lowercase()
        .replace(' ', "-")
        .trim_matches('/')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_wikilinks() {
        let body = "See [[Genesis Protocol]] and [[ppn-architecture-overview]].";
        let links = parse_wikilinks(body);
        assert_eq!(links, vec!["Genesis Protocol", "ppn-architecture-overview"]);
    }

    #[test]
    fn parses_display_text_wikilinks() {
        let body = "See [[Sovereign Mesh|sovereign-mesh]].";
        let links = parse_wikilinks(body);
        assert_eq!(links, vec!["sovereign-mesh"]);
    }

    #[test]
    fn skips_non_page_targets() {
        assert!(is_non_page_target("/category/architecture"));
        assert!(is_non_page_target("https://example.com"));
        assert!(!is_non_page_target("genesis-protocol"));
    }

    #[test]
    fn normalises_slug() {
        assert_eq!(normalise_slug("Genesis Protocol"), "genesis-protocol");
        assert_eq!(normalise_slug("PPn Architecture"), "ppn-architecture");
    }

    #[test]
    fn simple_yaml_parse() {
        let yaml = "title: Test Article\nslug: test-article\ncategory: architecture\n";
        let map = serde_yaml_simple_parse(yaml).unwrap();
        assert_eq!(map.get("title").map(|s| s.as_str()), Some("Test Article"));
        assert_eq!(map.get("slug").map(|s| s.as_str()), Some("test-article"));
    }

    #[test]
    fn check_content_catches_dead_links() {
        let tmp = std::env::temp_dir()
            .join(format!("xtask-test-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(
            tmp.join("existing.md"),
            "---\ntitle: Existing\nslug: existing\n---\nBody.\n",
        )
        .unwrap();
        std::fs::write(
            tmp.join("linker.md"),
            "---\ntitle: Linker\nslug: linker\n---\nSee [[existing]] and [[missing-page]].\n",
        )
        .unwrap();
        let (dead, missing, total) = check_content(&[tmp.clone()]);
        assert_eq!(total, 2);
        assert_eq!(dead.len(), 1);
        assert_eq!(dead[0].1, "missing-page");
        assert!(missing.is_empty());
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn check_content_catches_missing_required_fields() {
        let tmp = std::env::temp_dir()
            .join(format!("xtask-miss-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        // Missing `slug` field.
        std::fs::write(
            tmp.join("bad.md"),
            "---\ntitle: Bad Article\n---\nBody.\n",
        )
        .unwrap();
        let (dead, missing, _) = check_content(&[tmp.clone()]);
        assert!(dead.is_empty());
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].1, "slug");
        std::fs::remove_dir_all(&tmp).ok();
    }
}
