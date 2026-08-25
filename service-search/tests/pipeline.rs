//! Integration test: forge a tiny corpus, load a Strike, and assert the fusion invariants
//! — the anti-Spotlight substring guarantee and the trustworthy zero.

use std::fs;
use service_search::{forge, Config, RootSpec, Strike};

fn tmp_dir(tag: &str) -> std::path::PathBuf {
    // Unique-enough per run without pulling in a temp-dir crate.
    let base = std::env::temp_dir().join(format!("service-search-test-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();
    base
}

#[test]
fn forge_then_strike_honours_the_substring_guarantee() {
    let root = tmp_dir("corpus");
    // A corpus with a mid-word substring that a token index would split and miss.
    fs::write(root.join("letter-of-intent.md"), "the parties hereby agree to proceed").unwrap();
    fs::write(root.join("model.json"), "{\"cap_rate\": 0.065, \"rentRoll\": true}").unwrap();

    let index_path = tmp_dir("index");
    let config = Config {
        index_path: index_path.to_string_lossy().into_owned(),
        roots: vec![RootSpec {
            url_prefix: "test".into(),
            fs_path: root.to_string_lossy().into_owned(),
        }],
        exclude_dirs: vec![],
        bind: "127.0.0.1:0".into(),
        max_content_bytes: 5 * 1024 * 1024,
    };

    let stats = forge(&config).unwrap();
    assert_eq!(stats.files, 2);

    let strike = Strike::load(&config).unwrap();

    // 1) Filename substring → filenames band.
    let r = strike.search("intent").unwrap();
    assert!(r.filenames.iter().any(|h| h.name.contains("letter-of-intent")));

    // 2) Mid-word content substring "entRol" (inside "rentRoll") — the tokenizer killer.
    //    A BM25/token index misses this; the trigram floor must not.
    let r = strike.search("entRol").unwrap();
    assert!(
        r.contents.iter().any(|h| h.name.contains("model.json")),
        "mid-word substring must be found — the anti-Spotlight guarantee"
    );

    // 3) Content phrase present in the body.
    let r = strike.search("hereby agree").unwrap();
    assert!(r.contents.iter().any(|h| h.name.contains("letter-of-intent")));

    // 4) Truly absent → trustworthy zero (both bands empty), not an error.
    let r = strike.search("zzzznotanywhere").unwrap();
    assert!(r.filenames.is_empty() && r.contents.is_empty());
    assert_eq!(r.files_indexed, 2, "coverage line still reports what was searched");
}
