// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! "Counts only" reports print numbers only by default. Words taken from file
//! names (author initials, variant words, renditions, document types) appear only behind
//! `--names`. Made-up data only.
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_tool-precedent-core");

fn corpus(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("tpc-counts-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&p);
    let d = p.join("wip");
    fs::create_dir_all(&d).expect("mkdir");
    let page = "<html><body><h1>T</h1><p>Some made up paragraph that is long enough to count as a paragraph of twelve words or more here.</p></body></html>\n";
    for n in [
        "PROJ_ACMECO_2026_01_05_Zebracharter_JW1.html",
        "PROJ_ACMECO_2026_01_06_Zebracharter_JW1_FINALX.html",
        "Memo_A_JW1.html",
    ] {
        fs::write(d.join(n), page).expect("file");
    }
    p
}
fn run(root: &PathBuf, cmd: &str, extra: &[&str]) -> String {
    let o = Command::new(BIN)
        .arg(cmd)
        .args(["--dir", "wip", "--root"])
        .arg(root)
        .args(extra)
        .output()
        .expect("run");
    String::from_utf8_lossy(&o.stdout).into_owned()
}

#[test]
fn lineage_report_prints_no_file_name_words_by_default() {
    let r = corpus("lineage");
    let out = run(&r, "lineage-report", &[]);
    let _ = fs::remove_dir_all(&r);
    for word in ["JW", "FINALX", "html"] {
        assert!(!out.contains(word), "{word} leaked: {out}");
    }
    assert!(out.contains("distinct"), "{out}");
}

#[test]
fn lineage_report_shows_the_words_behind_names() {
    let r = corpus("lineage-names");
    let out = run(&r, "lineage-report", &["--names"]);
    let _ = fs::remove_dir_all(&r);
    assert!(out.contains("\"JW\""), "{out}");
    assert!(out.contains("FINALX"), "{out}");
    assert!(out.contains("\"html\""), "{out}");
}

#[test]
fn language_report_prints_no_document_type_by_default() {
    let r = corpus("language");
    let out = run(&r, "language-report", &[]);
    assert!(!out.contains("Zebracharter"), "type leaked: {out}");
    let named = run(&r, "language-report", &["--names"]);
    let _ = fs::remove_dir_all(&r);
    assert!(named.contains("Zebracharter"), "{named}");
}
