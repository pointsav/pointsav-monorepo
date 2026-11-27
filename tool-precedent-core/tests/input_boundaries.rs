// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Input boundaries (Foundry Rust Profile rule 10): malformed CSV is reported, never swallowed or
//! padded; an append never lands on a file that does not parse; every file read through the library
//! has a size cap. Made-up data only.
use std::fs;
use std::path::PathBuf;
use tool_precedent_core::changes::{append_change, read_changes, ChangeRow, CHANGES_HEADER};
use tool_precedent_core::draws::{append_draw, read_draws, DrawRow, DRAWS_HEADER};
use tool_precedent_core::registry::Registry;
use tool_precedent_core::terms::{append_term, read_terms, TermRow, TERMS_HEADER};

struct Dir(PathBuf);
impl Dir {
    fn new(name: &str) -> Self {
        let p = std::env::temp_dir().join(format!("tpc-bound-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).expect("mkdir");
        Dir(p)
    }
    fn put(&self, rel: &str, text: &str) -> PathBuf {
        let p = self.0.join(rel);
        if let Some(d) = p.parent() {
            fs::create_dir_all(d).expect("mkdir");
        }
        fs::write(&p, text).expect("write");
        p
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
fn term() -> TermRow {
    TermRow {
        instrument: "DEMO".into(),
        version: "JW3".into(),
        term: "Made Up Term".into(),
        replaces: "".into(),
        status: "live".into(),
        date: "".into(),
        context: "".into(),
    }
}
fn draw() -> DrawRow {
    DrawRow {
        instrument: "NEWDOC".into(),
        version: "JW1".into(),
        drew_from: "OLDDOC".into(),
        drew_from_version: "JW2".into(),
        date: "".into(),
        note: "".into(),
    }
}

// ------------------------------------------------------------------------------------ (a) quotes

#[test]
fn an_unbalanced_quote_is_an_error_naming_the_line_it_opened_on() {
    let d = Dir::new("quote-changes");
    let p = d.put(
        "changes.csv",
        &format!("{CHANGES_HEADER}\nA,JW2,JW1,1,a,b,Added,,\nA,JW3,JW2,2,\"open,b,Added,,\nA,JW4,JW3,3,a,b,Added,,\n"),
    );
    let e = read_changes(&p).expect_err("an unbalanced quote must not read as one long field");
    assert!(e.contains("line 3"), "{e}");
    assert!(e.contains("quote"), "{e}");
}

#[test]
fn an_unbalanced_quote_in_terms_draws_and_a_registry_table_is_an_error() {
    let d = Dir::new("quote-others");
    let t = d.put(
        "terms.csv",
        &format!("{TERMS_HEADER}\nD,JW1,x,,live,,\nD,JW2,\"y,,live,,\n"),
    );
    assert!(read_terms(&t).expect_err("terms").contains("line 3"));
    let w = d.put(
        "draws.csv",
        &format!("{DRAWS_HEADER}\nN,JW1,O,JW1,,\"note\nN,JW2,O,JW1,,\n"),
    );
    assert!(read_draws(&w).expect_err("draws").contains("line 2"));
    let reg = registry_dir(&d, Some(("classes", "class,status\n\"p.x,current\n")));
    let e = Registry::load(&reg).err().expect("registry");
    assert!(e.contains("ts-classes.csv") && e.contains("line 2"), "{e}");
}

/// A registry folder with every table holding a one-column header only; `over` replaces one table.
fn registry_dir(d: &Dir, over: Option<(&str, &str)>) -> PathBuf {
    for n in [
        "classes", "geometry", "labels", "glyphs", "sections", "families", "engine", "changes",
    ] {
        let text = match over {
            Some((m, t)) if m == n => t.to_string(),
            _ => "id\n".to_string(),
        };
        d.put(&format!("registry/ts-{n}.csv"), &text);
    }
    d.0.join("registry")
}

// ------------------------------------------------------------------------------------ (a) ragged rows

#[test]
fn ragged_rows_are_reported_with_their_count_and_first_line() {
    let d = Dir::new("ragged");
    let t = d.put(
        "terms.csv",
        &format!("{TERMS_HEADER}\nD,JW1,x,,live,,\nD,JW2,y\nD,JW3,z,,live,,\nD,JW4\n"),
    );
    let e = read_terms(&t).expect_err("terms");
    assert!(e.contains("2 rows") && e.contains("line 3"), "{e}");
    let w = d.put(
        "draws.csv",
        &format!("{DRAWS_HEADER}\nN,JW1,O,JW1,,\nN,JW2,O,JW1,,,extra\n"),
    );
    let e = read_draws(&w).expect_err("draws");
    assert!(e.contains("1 row") && e.contains("line 3"), "{e}");
    // a quoted newline makes the physical line differ from the record number
    let c = d.put(
        "changes.csv",
        &format!("{CHANGES_HEADER}\nA,JW2,JW1,1,\"two\nlines\",b,Added,,\nA,JW3\n"),
    );
    let e = read_changes(&c).expect_err("changes");
    assert!(e.contains("1 row") && e.contains("line 4"), "{e}");
}

#[test]
fn a_registry_table_row_with_too_few_or_too_many_cells_is_an_error_not_padded() {
    let d = Dir::new("ragged-reg");
    let reg = registry_dir(
        &d,
        Some((
            "glyphs",
            "glyph_id,char,status\na,x,current\nb,y\nc,z,current,spare\n",
        )),
    );
    let e = Registry::load(&reg)
        .err()
        .expect("ragged table must not load");
    assert!(
        e.contains("ts-glyphs.csv") && e.contains("2 rows") && e.contains("line 3"),
        "{e}"
    );
    // the same table without the ragged rows still loads, blank lines included
    let d2 = Dir::new("clean-reg");
    let reg = registry_dir(
        &d2,
        Some((
            "glyphs",
            "glyph_id,char,status\na,x,current\n\nb,y,current\n",
        )),
    );
    assert_eq!(Registry::load(&reg).expect("clean").glyphs.len(), 2);
}

// ------------------------------------------------------------------------------------ (a) appends

#[test]
fn the_append_commands_refuse_a_file_that_does_not_parse_cleanly() {
    let d = Dir::new("append-refuse");
    let open = "\"never closed";
    let c = d.put(
        "changes.csv",
        &format!("{CHANGES_HEADER}\nA,JW2,JW1,1,{open},b,Added,,\n"),
    );
    let before = fs::read_to_string(&c).unwrap();
    assert!(append_change(&c, &change()).is_err(), "unbalanced quote");
    assert_eq!(fs::read_to_string(&c).unwrap(), before, "file untouched");
    let c2 = d.put("changes2.csv", &format!("{CHANGES_HEADER}\nA,JW2\n"));
    assert!(append_change(&c2, &change()).is_err(), "ragged changes");
    let t = d.put("terms.csv", &format!("{TERMS_HEADER}\nD,JW1,{open}\n"));
    assert!(append_term(&t, &term()).is_err(), "terms");
    let w = d.put("draws.csv", &format!("{DRAWS_HEADER}\nN,JW1,{open}\n"));
    assert!(append_draw(&w, &draw()).is_err(), "draws");
    // not UTF-8: refused, not appended to
    let b = d.0.join("bytes.csv");
    fs::write(
        &b,
        [CHANGES_HEADER.as_bytes(), b"\nA,\xff\xfe,x\n"].concat(),
    )
    .unwrap();
    assert!(append_change(&b, &change()).is_err(), "invalid UTF-8");
}

#[test]
fn appends_to_clean_files_still_work() {
    let d = Dir::new("append-ok");
    let c = d.0.join("changes.csv");
    append_change(&c, &change()).expect("new file");
    append_change(
        &c,
        &ChangeRow {
            new_text: "with \"quotes\", a comma\nand a newline".into(),
            ..change()
        },
    )
    .expect("second row");
    assert_eq!(read_changes(&c).expect("read").len(), 2);
    let t = d.0.join("terms.csv");
    append_term(&t, &term()).expect("term");
    let w = d.0.join("draws.csv");
    append_draw(&w, &draw()).expect("draw");
}
