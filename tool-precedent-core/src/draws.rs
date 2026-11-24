// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! The Draw Register (`precedent/draws.csv`): which earlier instrument and
//! version a new document was drawn from. One row per draw, append-only, keyed
//! like `changes.csv` and `terms.csv`; no master, no new file extension.

use crate::csv::parse as parse_csv;
use std::path::Path;

pub const DRAWS_HEADER: &str = "instrument,version,drew_from,drew_from_version,date,note";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawRow {
    pub instrument: String,
    pub version: String,
    pub drew_from: String,
    pub drew_from_version: String,
    pub date: String,
    pub note: String,
}

fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

pub fn read_draws(path: &Path) -> Result<Vec<DrawRow>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut rows = parse_csv(&text);
    if rows.is_empty() {
        return Ok(vec![]);
    }
    rows.remove(0);
    rows.into_iter()
        .filter(|r| r.iter().any(|c| !c.is_empty()))
        .map(|r| {
            if r.len() != 6 {
                return Err(format!(
                    "{}: a row has {} fields, expected 6",
                    path.display(),
                    r.len()
                ));
            }
            Ok(DrawRow {
                instrument: r[0].clone(),
                version: r[1].clone(),
                drew_from: r[2].clone(),
                drew_from_version: r[3].clone(),
                date: r[4].clone(),
                note: r[5].clone(),
            })
        })
        .collect()
}

pub fn append_draw(path: &Path, row: &DrawRow) -> Result<(), String> {
    for (n, v) in [
        ("instrument", &row.instrument),
        ("version", &row.version),
        ("from", &row.drew_from),
        ("from-version", &row.drew_from_version),
    ] {
        if v.is_empty() {
            return Err(format!("--{n} is required"));
        }
    }
    if row.instrument == row.drew_from && row.version == row.drew_from_version {
        return Err("a document cannot be drawn from itself".into());
    }
    if path.exists() {
        let existing = read_draws(path)?;
        if existing.iter().any(|r| {
            r.instrument == row.instrument
                && r.version == row.version
                && r.drew_from == row.drew_from
                && r.drew_from_version == row.drew_from_version
        }) {
            return Err("this draw is already recorded; a recorded draw is never changed".into());
        }
    }
    use std::io::Write;
    let mut f = crate::csv::open_for_append(path, DRAWS_HEADER)?;
    let fields = [
        &row.instrument,
        &row.version,
        &row.drew_from,
        &row.drew_from_version,
        &row.date,
        &row.note,
    ];
    writeln!(
        f,
        "{}",
        fields
            .iter()
            .map(|s| csv_field(s))
            .collect::<Vec<_>>()
            .join(",")
    )
    .map_err(|e| e.to_string())
}

/// Draws for one instrument (as the new document or as the source).
pub fn select<'a>(rows: &'a [DrawRow], instrument: Option<&str>) -> Vec<&'a DrawRow> {
    rows.iter()
        .filter(|r| {
            instrument
                .map(|d| r.instrument == d || r.drew_from == d)
                .unwrap_or(true)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn append_read_and_guards() {
        let p = std::env::temp_dir().join(format!("draws-test-{}.csv", std::process::id()));
        let _ = std::fs::remove_file(&p);
        let r = DrawRow {
            instrument: "NEW".into(),
            version: "JW1".into(),
            drew_from: "OLD".into(),
            drew_from_version: "JW4".into(),
            date: "2026-10-23".into(),
            note: "a, b".into(),
        };
        append_draw(&p, &r).unwrap();
        assert_eq!(read_draws(&p).unwrap(), vec![r.clone()]);
        assert!(append_draw(&p, &r).is_err());
        assert!(append_draw(
            &p,
            &DrawRow {
                drew_from: "NEW".into(),
                drew_from_version: "JW1".into(),
                ..r.clone()
            }
        )
        .is_err());
        assert!(append_draw(
            &p,
            &DrawRow {
                instrument: "".into(),
                ..r
            }
        )
        .is_err());
        let _ = std::fs::remove_file(&p);
    }
}
