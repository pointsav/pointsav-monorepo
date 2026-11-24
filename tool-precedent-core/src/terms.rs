// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! The terms shard (`precedent/terms.csv`): which defined term each document
//! version used, what it replaced, and whether it is live or retired. Keyed
//! like `changes.csv` (instrument + version); no master, no new file extension.

use crate::csv::parse as parse_csv;
use std::path::Path;

pub const TERMS_HEADER: &str = "instrument,version,term,replaces,status,date,context";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TermRow {
    pub instrument: String,
    pub version: String,
    pub term: String,
    pub replaces: String,
    pub status: String,
    pub date: String,
    pub context: String,
}

pub fn valid_status(s: &str) -> bool {
    matches!(s, "live" | "retired")
}

fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

pub fn read_terms(path: &Path) -> Result<Vec<TermRow>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut rows = parse_csv(&text);
    if rows.is_empty() {
        return Ok(vec![]);
    }
    rows.remove(0);
    rows.into_iter()
        .filter(|r| r.iter().any(|c| !c.is_empty()))
        .map(|r| {
            if r.len() != 6 && r.len() != 7 {
                return Err(format!(
                    "{}: a row has {} fields, expected 6 or 7",
                    path.display(),
                    r.len()
                ));
            }
            Ok(TermRow {
                instrument: r[0].clone(),
                version: r[1].clone(),
                term: r[2].clone(),
                replaces: r[3].clone(),
                status: r[4].clone(),
                date: r[5].clone(),
                context: r.get(6).cloned().unwrap_or_default(),
            })
        })
        .collect()
}

pub fn append_term(path: &Path, row: &TermRow) -> Result<(), String> {
    if !valid_status(&row.status) {
        return Err(format!(
            "status must be live or retired, got {:?}",
            row.status
        ));
    }
    if row.term.is_empty() || row.instrument.is_empty() || row.version.is_empty() {
        return Err("instrument, version and term are required".into());
    }
    if path.exists() {
        let existing = read_terms(path)?;
        if existing.iter().any(|r| {
            r.instrument == row.instrument && r.version == row.version && r.term == row.term
        }) {
            return Err(format!(
                "{} {} already records '{}'; a recorded version is never changed",
                row.instrument, row.version, row.term
            ));
        }
    }
    use std::io::Write;
    let mut f = crate::csv::open_for_append(path, TERMS_HEADER)?;
    let fields = [
        &row.instrument,
        &row.version,
        &row.term,
        &row.replaces,
        &row.status,
        &row.date,
        &row.context,
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

/// Rows for one document and/or one term, oldest version first.
pub fn select<'a>(rows: &'a [TermRow], doc: Option<&str>, term: Option<&str>) -> Vec<&'a TermRow> {
    let n = |v: &str| v.trim_start_matches("JW").parse::<u32>().unwrap_or(0);
    let mut out: Vec<&TermRow> = rows
        .iter()
        .filter(|r| {
            doc.map(|d| r.instrument == d).unwrap_or(true)
                && term.map(|t| r.term == t).unwrap_or(true)
        })
        .collect();
    out.sort_by_key(|r| (r.instrument.clone(), n(&r.version)));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn append_and_read_round_trip() {
        let p = std::env::temp_dir().join(format!("terms-test-{}.csv", std::process::id()));
        let _ = std::fs::remove_file(&p);
        let r = TermRow {
            instrument: "D".into(),
            version: "JW2".into(),
            term: "LP Unit".into(),
            replaces: "Units, LP Units".into(),
            status: "live".into(),
            date: "2026-10-23".into(),
            context: "".into(),
        };
        append_term(&p, &r).unwrap();
        assert_eq!(read_terms(&p).unwrap(), vec![r.clone()]);
        assert!(append_term(&p, &r).is_err());
        let bad = TermRow {
            status: "gone".into(),
            ..r
        };
        assert!(append_term(&p, &bad).is_err());
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn reads_old_six_column_rows_and_refuses_to_append_to_an_old_header() {
        let p = std::env::temp_dir().join(format!("terms-old-{}.csv", std::process::id()));
        std::fs::write(
            &p,
            "doc_id,version,term,replaces,status,date\nD,JW1,x,,live,2026-11-01\n",
        )
        .unwrap();
        let rows = read_terms(&p).unwrap();
        assert_eq!(rows[0].instrument, "D");
        assert_eq!(rows[0].context, "");
        let r = TermRow {
            instrument: "D".into(),
            version: "JW2".into(),
            term: "y".into(),
            replaces: "".into(),
            status: "live".into(),
            date: "".into(),
            context: "a sentence, with a comma".into(),
        };
        assert!(append_term(&p, &r).unwrap_err().contains("re-key"));
        let _ = std::fs::remove_file(&p);
        append_term(&p, &r).unwrap();
        assert_eq!(read_terms(&p).unwrap(), vec![r]);
        let _ = std::fs::remove_file(&p);
    }
}
