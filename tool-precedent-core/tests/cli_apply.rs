// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! `apply` and malformed per-document override markers: a style block with
//! one marker missing, or the close before the open, must be refused and left unchanged, never
//! rewritten with the document's own CSS dropped. Made-up data only.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_tool-precedent-core");
const OPEN: &str =
    "/* === PER-DOCUMENT OVERRIDES (preserved by apply: edit here, not above) === */";
const CLOSE: &str = "/* === END PER-DOCUMENT OVERRIDES === */";

struct Work(PathBuf);
impl Work {
    fn new(name: &str) -> Self {
        let p = std::env::temp_dir().join(format!("tpc-apply-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(p.join("precedent")).expect("mkdir");
        let fx = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry/templates");
        copy_dir(&fx, &p.join("templates"));
        fs::write(p.join("templates/letter.css"), ":root { --made-up: 1; }\n").expect("token");
        fs::write(
            p.join("precedent/manifest.csv"),
            "instrument,family,components\npage,letter,recital\n",
        )
        .expect("manifest");
        Work(p)
    }
    fn page(&self, style: &str) {
        let html =
            format!("<html><head><style>\n{style}\n</style></head><body><p>x</p></body></html>\n");
        fs::write(self.0.join("page.html"), html).expect("page");
    }
    fn text(&self) -> String {
        fs::read_to_string(self.0.join("page.html")).expect("read")
    }
    fn apply(&self, check: bool) -> Output {
        let mut c = Command::new(BIN);
        c.arg("apply").arg("page.html");
        if check {
            c.arg("--check");
        }
        c.arg("--root").arg(&self.0).output().expect("run")
    }
}
impl Drop for Work {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("mkdir");
    for e in fs::read_dir(from).expect("read_dir").flatten() {
        let dest = to.join(e.file_name());
        if e.path().is_dir() {
            copy_dir(&e.path(), &dest);
        } else {
            fs::copy(e.path(), dest).expect("copy");
        }
    }
}

#[test]
fn a_missing_close_marker_is_refused_and_the_file_is_unchanged() {
    let w = Work::new("no-close");
    w.page(&format!("{OPEN}\n.keep {{ color: blue; }}"));
    let before = w.text();
    let o = w.apply(false);
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("override markers"));
    assert_eq!(w.text(), before);
}

#[test]
fn a_missing_open_marker_is_refused() {
    let w = Work::new("no-open");
    w.page(&format!(".keep {{ color: blue; }}\n{CLOSE}"));
    let before = w.text();
    assert!(!w.apply(false).status.success());
    assert_eq!(w.text(), before);
}

#[test]
fn a_close_before_the_open_is_refused() {
    let w = Work::new("reversed");
    w.page(&format!("{CLOSE}\n.keep {{ color: blue; }}\n{OPEN}"));
    let before = w.text();
    assert!(!w.apply(false).status.success());
    assert_eq!(w.text(), before);
}

#[test]
fn check_reports_malformed_markers_as_a_failure_not_as_a_plain_difference() {
    let w = Work::new("check");
    w.page(&format!("{OPEN}\n.keep {{ color: blue; }}"));
    let o = w.apply(true);
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("override markers"));
}

#[test]
fn well_formed_overrides_are_preserved() {
    let w = Work::new("good");
    w.page(&format!("{OPEN}\n.keep {{ color: blue; }}\n{CLOSE}"));
    assert!(w.apply(false).status.success());
    assert!(w.text().contains(".keep { color: blue; }"));
    assert!(w.apply(false).status.success());
    assert!(w.text().contains(".keep { color: blue; }"));
}

#[test]
fn a_style_block_with_no_markers_at_all_still_applies() {
    let w = Work::new("none");
    w.page(".x { color: red; }");
    assert!(w.apply(false).status.success());
    assert!(w.text().contains(OPEN));
}
