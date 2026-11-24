// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Append repairs: appending to a register file that lacks a final
//! newline must not fuse rows, and appending to an existing empty file must write the header
//! first. Covers the three append commands. Made-up data only.
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_tool-precedent-core");

struct Work(PathBuf);
impl Work {
    fn new(name: &str) -> Self {
        let p = std::env::temp_dir().join(format!("tpc-append-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(p.join("precedent")).expect("mkdir");
        Work(p)
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(BIN)
            .args(args)
            .arg("--root")
            .arg(&self.0)
            .output()
            .expect("run")
    }
    fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.0.join(rel)).unwrap_or_default()
    }
    fn put(&self, rel: &str, text: &str) {
        fs::write(self.0.join(rel), text).expect("write");
    }
}
impl Drop for Work {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct Kind {
    name: &'static str,
    file: &'static str,
    header: &'static str,
    first_row: &'static str,
    add: Vec<&'static str>,
}
fn kinds() -> Vec<Kind> {
    vec![
        Kind {
            name: "changes",
            file: "precedent/changes.csv",
            header:
                "instrument,version,baseline,section,baseline_text,new_text,tag,decision_ref,date",
            first_row: "Memo_A,JW1,,s1,x,y,Added,D0,2026-01-01",
            add: vec![
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
                "a",
                "--new",
                "b",
                "--tag",
                "Replaced",
                "--decision",
                "D1",
                "--date",
                "2026-01-02",
            ],
        },
        Kind {
            name: "terms",
            file: "precedent/terms.csv",
            header: "instrument,version,term,replaces,status,date,context",
            first_row: "Memo_A,JW1,holder,,live,2026-01-01,made-up",
            add: vec![
                "terms-add",
                "--instrument",
                "Memo_A",
                "--version",
                "JW2",
                "--term",
                "member",
                "--date",
                "2026-01-02",
                "--context",
                "made-up",
            ],
        },
        Kind {
            name: "draws",
            file: "precedent/draws.csv",
            header: "instrument,version,drew_from,drew_from_version,date,note",
            first_row: "Memo_B,JW1,Memo_A,JW3,2026-01-01,made-up",
            add: vec![
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
                "2026-01-02",
                "--note",
                "made-up",
            ],
        },
    ]
}

#[test]
fn appending_to_a_file_without_a_final_newline_keeps_rows_apart() {
    for k in kinds() {
        let w = Work::new(&format!("nonl-{}", k.name));
        w.put(k.file, &format!("{}\n{}", k.header, k.first_row));
        let o = w.run(&k.add);
        assert!(
            o.status.success(),
            "{}: {}",
            k.name,
            String::from_utf8_lossy(&o.stderr)
        );
        let text = w.read(k.file);
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 3, "{}: {text:?}", k.name);
        assert_eq!(lines[0], k.header);
        assert_eq!(
            lines[1], k.first_row,
            "{}: the old last row is intact",
            k.name
        );
        assert!(text.ends_with('\n'));
    }
}

#[test]
fn appending_to_an_existing_empty_file_writes_the_header_first() {
    for k in kinds() {
        let w = Work::new(&format!("empty-{}", k.name));
        w.put(k.file, "");
        let o = w.run(&k.add);
        assert!(
            o.status.success(),
            "{}: {}",
            k.name,
            String::from_utf8_lossy(&o.stderr)
        );
        let text = w.read(k.file);
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2, "{}: {text:?}", k.name);
        assert_eq!(lines[0], k.header, "{}: header first", k.name);
    }
}

#[test]
fn a_file_that_already_ends_in_a_newline_gets_no_blank_line() {
    for k in kinds() {
        let w = Work::new(&format!("ok-{}", k.name));
        w.put(k.file, &format!("{}\n{}\n", k.header, k.first_row));
        assert!(w.run(&k.add).status.success());
        let text = w.read(k.file);
        assert!(!text.contains("\n\n"), "{}: {text:?}", k.name);
        assert_eq!(text.lines().count(), 3);
    }
}
