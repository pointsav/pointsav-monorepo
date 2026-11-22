// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Characterisation tests for the CLI write commands (audit patch P0). They pin what the
//! commands do today on well-formed input, so the later fixes (root confinement, newline
//! repair, atomic writes) can be shown not to change it. Made-up data only. They deliberately
//! do not pin the known defects (a file lacking a final newline, an existing empty file, a
//! path outside `--root`): those get their own failing tests in the patch that fixes them.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_tool-precedent-core");
const CHANGES_HEADER: &str =
    "instrument,version,baseline,section,baseline_text,new_text,tag,decision_ref,date";
const TERMS_HEADER: &str = "instrument,version,term,replaces,status,date,context";
const DRAWS_HEADER: &str = "instrument,version,drew_from,drew_from_version,date,note";

struct Work(PathBuf);
impl Work {
    fn new(name: &str) -> Self {
        let p = std::env::temp_dir().join(format!("tpc-cli-write-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).expect("create work folder");
        Work(p)
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(BIN)
            .args(args)
            .arg("--root")
            .arg(&self.0)
            .output()
            .expect("run binary")
    }
    fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.0.join(rel)).unwrap_or_default()
    }
    fn put(&self, rel: &str, text: &str) {
        let p = self.0.join(rel);
        fs::create_dir_all(p.parent().expect("parent")).expect("mkdir");
        fs::write(p, text).expect("write fixture");
    }
}
impl Drop for Work {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn ok(o: &Output) -> bool {
    o.status.success()
}
fn change<'a>(old: &'a str, new: &'a str, tag: &'a str) -> Vec<&'a str> {
    vec![
        "changes-add",
        "--instrument",
        "Memo_A",
        "--version",
        "JW2",
        "--baseline",
        "JW1",
        "--section",
        "s1",
        "--old",
        old,
        "--new",
        new,
        "--tag",
        tag,
        "--decision",
        "D1",
        "--date",
        "2026-01-02",
    ]
}

#[test]
fn changes_add_creates_file_with_header_then_appends_one_row() {
    let w = Work::new("changes-new");
    assert!(ok(&w.run(&change("a", "b", "Replaced"))));
    assert_eq!(
        w.read("precedent/changes.csv"),
        format!("{CHANGES_HEADER}\nMemo_A,JW2,JW1,s1,a,b,Replaced,D1,2026-01-02\n")
    );
    assert!(ok(&w.run(&change("c", "d", "Added"))));
    assert_eq!(
        w.read("precedent/changes.csv"),
        format!(
            "{CHANGES_HEADER}\nMemo_A,JW2,JW1,s1,a,b,Replaced,D1,2026-01-02\n\
             Memo_A,JW2,JW1,s1,c,d,Added,D1,2026-01-02\n"
        )
    );
}

#[test]
fn changes_add_quotes_commas_and_quotes_in_fields() {
    let w = Work::new("changes-quote");
    assert!(ok(&w.run(&change("old, text", "say \"hi\"", "Replaced"))));
    let text = w.read("precedent/changes.csv");
    assert!(text
        .ends_with("Memo_A,JW2,JW1,s1,\"old, text\",\"say \"\"hi\"\"\",Replaced,D1,2026-01-02\n"));
}

#[test]
fn changes_add_refuses_a_bad_tag_and_writes_nothing() {
    let w = Work::new("changes-tag");
    let o = w.run(&change("a", "b", "Nonsense"));
    assert!(!ok(&o));
    assert!(!w.0.join("precedent/changes.csv").exists());
}

#[test]
fn changes_add_refuses_a_foreign_header_and_leaves_the_file_unchanged() {
    let w = Work::new("changes-header");
    let old = "doc_id,version,x\nMemo_A,JW1,y\n";
    w.put("precedent/changes.csv", old);
    assert!(!ok(&w.run(&change("a", "b", "Replaced"))));
    assert_eq!(w.read("precedent/changes.csv"), old);
}

#[test]
fn changes_add_appends_to_a_well_formed_file_without_touching_earlier_rows() {
    let w = Work::new("changes-existing");
    let before = format!("{CHANGES_HEADER}\nMemo_A,JW1,,s0,x,y,Added,D0,2026-01-01\n");
    w.put("precedent/changes.csv", &before);
    assert!(ok(&w.run(&change("a", "b", "Replaced"))));
    let after = w.read("precedent/changes.csv");
    assert!(after.starts_with(&before));
    assert_eq!(after.lines().count(), 3);
}

#[test]
fn terms_add_creates_then_appends() {
    let w = Work::new("terms");
    let a = [
        "terms-add",
        "--instrument",
        "Memo_A",
        "--version",
        "JW2",
        "--term",
        "holder",
        "--date",
        "2026-01-02",
        "--context",
        "made-up",
    ];
    assert!(ok(&w.run(&a)));
    let first = w.read("precedent/terms.csv");
    assert!(first.starts_with(&format!("{TERMS_HEADER}\n")));
    assert_eq!(first.lines().count(), 2);
    assert!(!ok(&w.run(&a)), "an identical term row is refused");
    assert_eq!(w.read("precedent/terms.csv"), first);
    let b = [
        "terms-add",
        "--instrument",
        "Memo_A",
        "--version",
        "JW3",
        "--term",
        "member",
        "--replaces",
        "holder",
        "--date",
        "2026-01-03",
        "--context",
        "made-up",
    ];
    assert!(ok(&w.run(&b)));
    let second = w.read("precedent/terms.csv");
    assert!(second.starts_with(&first));
    assert_eq!(second.lines().count(), 3);
}

#[test]
fn draw_add_creates_then_appends() {
    let w = Work::new("draws");
    let a = [
        "draw-add",
        "--instrument",
        "Memo_B",
        "--version",
        "JW1",
        "--from",
        "Memo_A",
        "--from-version",
        "JW3",
        "--date",
        "2026-01-04",
        "--note",
        "made-up",
    ];
    assert!(ok(&w.run(&a)));
    let first = w.read("precedent/draws.csv");
    assert!(first.starts_with(&format!("{DRAWS_HEADER}\n")));
    assert_eq!(first.lines().count(), 2);
    assert!(!ok(&w.run(&a)), "an identical draw is refused");
    assert_eq!(w.read("precedent/draws.csv"), first);
    let b = [
        "draw-add",
        "--instrument",
        "Memo_B",
        "--version",
        "JW2",
        "--from",
        "Memo_A",
        "--from-version",
        "JW3",
        "--date",
        "2026-01-05",
        "--note",
        "made-up",
    ];
    assert!(ok(&w.run(&b)));
    assert_eq!(w.read("precedent/draws.csv").lines().count(), 3);
}

#[test]
fn add_commands_honour_an_in_root_file_flag() {
    let w = Work::new("file-flag");
    let mut a = change("a", "b", "Replaced");
    a.extend(["--file", "precedent/other.csv"]);
    let o = Command::new(BIN)
        .args(&a)
        .arg("--root")
        .arg(&w.0)
        .current_dir(&w.0)
        .output()
        .expect("run");
    assert!(ok(&o));
    assert!(w.0.join("precedent/other.csv").exists());
    assert!(!w.0.join("precedent/changes.csv").exists());
}

fn doc(n: u32) -> String {
    format!("<html><body><h1>Memo A</h1><p>Version {n}, made-up text long enough to count as a unit of text for the scan.</p></body></html>\n")
}

#[test]
fn versions_rewrites_the_file_and_is_deterministic() {
    let w = Work::new("versions");
    for n in 1..=3 {
        w.put(&format!("outputs/Memo_A_JW{n}.html"), &doc(n));
    }
    assert!(ok(&w.run(&["versions"])));
    let first = w.read("precedent/versions.csv");
    assert!(
        first.lines().count() >= 4,
        "header plus three rows: {first}"
    );
    assert!(ok(&w.run(&["versions"])));
    assert_eq!(w.read("precedent/versions.csv"), first);
}

#[test]
fn versions_out_flag_writes_there_and_not_to_the_default() {
    let w = Work::new("versions-out");
    w.put("outputs/Memo_A_JW1.html", &doc(1));
    let o = Command::new(BIN)
        .args(["versions", "--out", "precedent/custom.csv", "--root"])
        .arg(&w.0)
        .current_dir(&w.0)
        .output()
        .expect("run");
    assert!(ok(&o));
    assert!(w.0.join("precedent/custom.csv").exists());
    assert!(!w.0.join("precedent/versions.csv").exists());
}

fn style_html() -> &'static str {
    "<html><head><style>\n.x { color: red; }\n</style></head><body><p>made-up</p></body></html>\n"
}

fn apply(dir: &Path, file: &str, check: bool) -> Output {
    let mut c = Command::new(BIN);
    c.arg("apply").arg(file);
    if check {
        c.arg("--check");
    }
    c.arg("--root")
        .arg(dir)
        .current_dir(dir)
        .output()
        .expect("run")
}

#[test]
fn apply_check_changes_nothing_and_a_second_apply_is_idempotent() {
    let w = Work::new("apply");
    let mut src = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry");
    src.push("precedent");
    fs::create_dir_all(w.0.join("precedent")).expect("mkdir");
    copy_dir(&src, &w.0.join("precedent"));
    copy_dir(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/registry/templates"),
        &w.0.join("templates"),
    );
    w.put(
        "precedent/manifest.csv",
        "instrument,family,components\npage,letter,recital\n",
    );
    w.put("templates/letter.css", ":root { --made-up: 1; }\n");
    w.put("page.html", style_html());
    let before = w.read("page.html");
    let _ = apply(&w.0, "page.html", true);
    assert_eq!(w.read("page.html"), before, "--check must not write");
    let a1 = apply(&w.0, "page.html", false);
    assert!(
        ok(&a1),
        "apply failed: {}",
        String::from_utf8_lossy(&a1.stderr)
    );
    let once = w.read("page.html");
    assert_ne!(once, before, "apply rewrites the style block");
    assert!(ok(&apply(&w.0, "page.html", false)));
    assert_eq!(w.read("page.html"), once, "a second apply changes nothing");
}

fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("mkdir");
    for e in fs::read_dir(from).expect("read_dir") {
        let e = e.expect("entry");
        let dest = to.join(e.file_name());
        if e.path().is_dir() {
            copy_dir(&e.path(), &dest);
        } else {
            fs::copy(e.path(), dest).expect("copy");
        }
    }
}
