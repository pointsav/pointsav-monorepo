//! The terms shard (`precedent/terms.csv`): which defined term each document
//! version used, what it replaced, and whether it is live or retired. Keyed
//! like `changes.csv` (doc_id + version); no master, no new file extension.

use crate::csv::parse as parse_csv;
use std::path::Path;

pub const TERMS_HEADER: &str = "doc_id,version,term,replaces,status,date";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TermRow {
    pub doc_id: String,
    pub version: String,
    pub term: String,
    pub replaces: String,
    pub status: String,
    pub date: String,
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
            if r.len() != 6 {
                return Err(format!(
                    "{}: a row has {} fields, expected 6",
                    path.display(),
                    r.len()
                ));
            }
            Ok(TermRow {
                doc_id: r[0].clone(),
                version: r[1].clone(),
                term: r[2].clone(),
                replaces: r[3].clone(),
                status: r[4].clone(),
                date: r[5].clone(),
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
    if row.term.is_empty() || row.doc_id.is_empty() || row.version.is_empty() {
        return Err("doc, version and term are required".into());
    }
    if path.exists() {
        let existing = read_terms(path)?;
        if existing
            .iter()
            .any(|r| r.doc_id == row.doc_id && r.version == row.version && r.term == row.term)
        {
            return Err(format!(
                "{} {} already records '{}'; a recorded version is never changed",
                row.doc_id, row.version, row.term
            ));
        }
    }
    use std::io::Write;
    let new_file = !path.exists();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    if new_file {
        writeln!(f, "{TERMS_HEADER}").map_err(|e| e.to_string())?;
    }
    let fields = [
        &row.doc_id,
        &row.version,
        &row.term,
        &row.replaces,
        &row.status,
        &row.date,
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
            doc.map(|d| r.doc_id == d).unwrap_or(true) && term.map(|t| r.term == t).unwrap_or(true)
        })
        .collect();
    out.sort_by_key(|r| (r.doc_id.clone(), n(&r.version)));
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
            doc_id: "D".into(),
            version: "JW2".into(),
            term: "LP Unit".into(),
            replaces: "Units, LP Units".into(),
            status: "live".into(),
            date: "2026-10-23".into(),
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
}
