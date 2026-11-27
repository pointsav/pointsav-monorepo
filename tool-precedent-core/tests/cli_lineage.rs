// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Draw-register loops and the version index (draw cycles, missing folder, overlapping markers).
//! Made-up data only.
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_tool-precedent-core");
const OPEN: &str =
    "/* === PER-DOCUMENT OVERRIDES (preserved by apply: edit here, not above) === */";
const CLOSE: &str = "/* === END PER-DOCUMENT OVERRIDES === */";

struct Work(PathBuf);
impl Work {
    fn new(name: &str) -> Self {
        let p = std::env::temp_dir().join(format!("tpc-lineage-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(p.join("wip")).expect("mkdir");
        fs::create_dir_all(p.join("precedent")).expect("mkdir");
        Work(p)
    }
    fn doc(&self, name: &str) {
        fs::write(
            self.0.join("wip").join(name),
            "<html><body>x</body></html>\n",
        )
        .expect("doc");
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(BIN)
            .args(args)
            .arg("--root")
            .arg(&self.0)
            .output()
            .expect("run")
    }
}
impl Drop for Work {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn text(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

#[test]
fn a_loop_in_the_draw_register_does_not_remove_the_instrument_from_the_report() {
    let w = Work::new("cycle");
    w.doc("Memo_A_JW1.html");
    w.doc("Memo_A_JW2.html");
    fs::write(
        w.0.join("precedent/draws.csv"),
        "instrument,version,drew_from,drew_from_version,date,note\n\
         Memo_A,JW1,Memo_A,JW2,2026-01-01,n\nMemo_A,JW2,Memo_A,JW1,2026-01-02,n\n",
    )
    .expect("draws");
    let out = text(&w.run(&["lineage-report", "--dir", "wip"]));
    assert!(out.contains("heads per instrument: 1 head: 1"), "{out}");
    assert!(!out.contains("0 head"), "{out}");
    assert!(
        out.contains("draw rows ignored"),
        "the data error is reported: {out}"
    );
    let lang = text(&w.run(&["language-report", "--dir", "wip"]));
    assert!(lang.contains("documents read (one per head): 1"), "{lang}");
}

#[test]
fn a_version_that_draws_from_itself_is_ignored_not_fatal() {
    let w = Work::new("self");
    w.doc("Memo_A_JW1.html");
    fs::write(
        w.0.join("precedent/draws.csv"),
        "instrument,version,drew_from,drew_from_version,date,note\nMemo_A,JW1,Memo_A,JW1,2026-01-01,n\n",
    )
    .expect("draws");
    let out = text(&w.run(&["lineage-report", "--dir", "wip"]));
    assert!(out.contains("heads per instrument: 1 head: 1"), "{out}");
}

#[test]
fn a_clean_register_prints_no_ignored_line() {
    let w = Work::new("clean");
    w.doc("Memo_A_JW1.html");
    w.doc("Memo_A_JW2.html");
    let out = text(&w.run(&["lineage-report", "--dir", "wip"]));
    assert!(!out.contains("ignored"), "{out}");
}

#[test]
fn versions_with_a_missing_folder_is_an_error_and_keeps_the_old_index() {
    let w = Work::new("missing");
    let old = "instrument,version,file,pdf_present,captured\nMemo_A,JW1,Memo_A_JW1.html,false,\n";
    fs::write(w.0.join("precedent/versions.csv"), old).expect("old index");
    let o = w.run(&["versions", "--dir", "no-such-folder"]);
    assert!(
        !o.status.success(),
        "{}",
        String::from_utf8_lossy(&o.stdout)
    );
    assert_eq!(
        fs::read_to_string(w.0.join("precedent/versions.csv")).expect("read"),
        old
    );
}

#[test]
fn an_empty_existing_folder_still_rewrites_the_index_to_a_header() {
    let w = Work::new("emptydir");
    fs::write(w.0.join("precedent/versions.csv"), "old\n").expect("old index");
    let o = w.run(&["versions", "--dir", "wip"]);
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert_eq!(
        fs::read_to_string(w.0.join("precedent/versions.csv")).expect("read"),
        "instrument,version,file,pdf_present,captured\n"
    );
}

#[test]
fn override_markers_that_overlap_are_refused_not_a_panic() {
    let w = Work::new("overlap");
    let fx =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry/templates");
    fn copy(from: &std::path::Path, to: &std::path::Path) {
        fs::create_dir_all(to).expect("mkdir");
        for e in fs::read_dir(from).expect("dir").flatten() {
            let d = to.join(e.file_name());
            if e.path().is_dir() {
                copy(&e.path(), &d);
            } else {
                fs::copy(e.path(), d).expect("copy");
            }
        }
    }
    copy(&fx, &w.0.join("templates"));
    fs::write(
        w.0.join("templates/letter.css"),
        ":root { --made-up: 1; }\n",
    )
    .expect("token");
    fs::write(
        w.0.join("precedent/manifest.csv"),
        "instrument,family,components\npage,letter,recital\n",
    )
    .expect("manifest");
    let overlap = format!("{}{}", &OPEN[..OPEN.len() - 1], CLOSE);
    let html = format!("<html><head><style>\n{overlap}\n</style></head><body>x</body></html>\n");
    fs::write(w.0.join("page.html"), &html).expect("page");
    let o = w.run(&["apply", "page.html"]);
    assert_eq!(
        o.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&o.stderr)
    );
    assert_eq!(
        fs::read_to_string(w.0.join("page.html")).expect("read"),
        html
    );
}

#[test]
fn a_malformed_draw_register_is_reported_not_treated_as_empty() {
    let w = Work::new("badregister");
    w.doc("Memo_A_JW1.html");
    fs::write(
        w.0.join("precedent/draws.csv"),
        "instrument,version,drew_from,drew_from_version,date,note\nMemo_A,JW1,only,three\n",
    )
    .expect("draws");
    for cmd in ["lineage-report", "language-report"] {
        let out = text(&w.run(&[cmd, "--dir", "wip"]));
        assert!(
            out.contains("draw register could not be read"),
            "{cmd}: {out}"
        );
        assert!(out.contains("expected 6"), "{cmd}: {out}");
    }
}
