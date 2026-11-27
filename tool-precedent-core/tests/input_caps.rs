// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! The input size cap (Foundry Rust Profile rule 10): every file read through the library is
//! refused past `csv::MAX_FILE_BYTES`, with a typed error, before it is read into memory. Made-up data only.
use std::fs;
use std::path::{Path, PathBuf};
use tool_precedent_core::changes::{append_change, read_changes, ChangeRow, CHANGES_HEADER};
use tool_precedent_core::terms::{read_terms, TERMS_HEADER};

struct Dir(PathBuf);
impl Dir {
    fn new(name: &str) -> Self {
        let p = std::env::temp_dir().join(format!("tpc-cap-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).expect("mkdir");
        Dir(p)
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn change() -> ChangeRow {
    ChangeRow {
        instrument: "DEMO".into(),
        version: "JW2".into(),
        baseline: "JW1".into(),
        section: "1.1".into(),
        baseline_text: "old".into(),
        new_text: "new".into(),
        tag: "Replaced".into(),
        decision_ref: "".into(),
        date: "2026-01-02".into(),
    }
}

/// A sparse file one byte past the library cap (no real 64 MiB is written on most file systems).
fn sparse(p: &Path, len: u64, head: &str) {
    let f = fs::File::create(p).expect("create");
    use std::io::Write;
    (&f).write_all(head.as_bytes()).expect("head");
    f.set_len(len).expect("set_len");
}

#[test]
fn the_register_readers_refuse_a_file_past_the_cap() {
    let d = Dir::new("cap-readers");
    let cap = tool_precedent_core::csv::MAX_FILE_BYTES;
    assert_eq!(cap, 64 * 1024 * 1024);
    let p = d.0.join("changes.csv");
    sparse(&p, cap + 1, &format!("{CHANGES_HEADER}\n"));
    let e = read_changes(&p).expect_err("past the cap");
    assert!(e.contains("limit"), "{e}");
    assert!(append_change(&p, &change()).is_err());
    assert_eq!(fs::metadata(&p).unwrap().len(), cap + 1, "not appended to");
    let t = d.0.join("terms.csv");
    sparse(&t, cap + 1, &format!("{TERMS_HEADER}\n"));
    assert!(read_terms(&t).expect_err("terms").contains("limit"));
}

#[test]
fn read_text_accepts_exactly_the_cap_and_refuses_one_byte_more() {
    use tool_precedent_core::csv::read_text_capped;
    use tool_precedent_core::error::CoreError;
    let d = Dir::new("cap-edge");
    let cap = 4096u64;
    let at = d.0.join("at.txt");
    fs::write(&at, "a".repeat(cap as usize)).unwrap();
    assert_eq!(read_text_capped(&at, cap).expect("at the cap").len(), 4096);
    let past = d.0.join("past.txt");
    fs::write(&past, "a".repeat(cap as usize + 1)).unwrap();
    let e = read_text_capped(&past, cap).expect_err("one byte past");
    assert_eq!(e.kind(), std::io::ErrorKind::FileTooLarge);
    assert!(matches!(CoreError::from(e), CoreError::TooLarge(_)));
    // a missing file is still NotFound, so "no register yet" keeps working
    let none = read_text_capped(&d.0.join("none.csv"), cap).expect_err("missing");
    assert_eq!(none.kind(), std::io::ErrorKind::NotFound);
}
