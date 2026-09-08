//! One-off membership-drift check for architecture/ (Track-B Phase A projects wave).
//! Filters siblings by language to avoid EN/ES slug collision cross-talk.
use std::env;
use std::path::Path;
use tool_wiki_core::regenerate::check_membership;

fn main() {
    let root = env::args().nth(1).expect("usage: architecture_drift <path>");
    let result = tool_wiki_core::crawl(Path::new(&root)).expect("crawl failed");
    let docs: Vec<_> = result
        .documents
        .into_iter()
        .filter(|d| d.path.to_string_lossy().starts_with("architecture/"))
        .collect();

    let index_docs: Vec<_> = docs.iter().filter(|d| d.is_index_page()).collect();

    let mut total_drift = 0;
    for idx in &index_docs {
        let is_es = idx.path.to_string_lossy().ends_with(".es.md");
        let siblings: Vec<_> = docs
            .iter()
            .filter(|d| !d.is_index_page())
            .filter(|d| d.path.to_string_lossy().ends_with(".es.md") == is_es)
            .cloned()
            .collect();
        let drift = check_membership(idx, &siblings);
        println!("{}: {} drift(s)", idx.path.display(), drift.len());
        for d in &drift {
            println!("  group={} -> {:?}", d.index_group, d.actual_line);
        }
        total_drift += drift.len();
    }
    println!("\nTOTAL DRIFT: {total_drift}");
}
