//! The Draw Register (`precedent/draws.csv`): which earlier instrument and
//! version a new document was drawn from. One row per draw, append-only, keyed
//! like `changes.csv` and `terms.csv`; no master, no new file extension.

use crate::csv::parse as parse_csv;
use std::path::Path;

pub const DRAWS_HEADER: &str = "project,doc_id,version,drew_from,drew_from_version,date,note";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawRow {
    pub project: String,
    pub doc_id: String,
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
            if r.len() != 7 {
                return Err(format!(
                    "{}: a row has {} fields, expected 7",
                    path.display(),
                    r.len()
                ));
            }
            Ok(DrawRow {
                project: r[0].clone(),
                doc_id: r[1].clone(),
                version: r[2].clone(),
                drew_from: r[3].clone(),
                drew_from_version: r[4].clone(),
                date: r[5].clone(),
                note: r[6].clone(),
            })
        })
        .collect()
}

pub fn append_draw(path: &Path, row: &DrawRow) -> Result<(), String> {
    for (n, v) in [
        ("project", &row.project),
        ("doc", &row.doc_id),
        ("version", &row.version),
        ("from", &row.drew_from),
        ("from-version", &row.drew_from_version),
    ] {
        if v.is_empty() {
            return Err(format!("--{n} is required"));
        }
    }
    if row.doc_id == row.drew_from && row.version == row.drew_from_version {
        return Err("a document cannot be drawn from itself".into());
    }
    if path.exists() {
        let existing = read_draws(path)?;
        if existing.iter().any(|r| {
            r.project == row.project
                && r.doc_id == row.doc_id
                && r.version == row.version
                && r.drew_from == row.drew_from
                && r.drew_from_version == row.drew_from_version
        }) {
            return Err("this draw is already recorded; a recorded draw is never changed".into());
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
        writeln!(f, "{DRAWS_HEADER}").map_err(|e| e.to_string())?;
    }
    let fields = [
        &row.project,
        &row.doc_id,
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

/// Draws for one project and/or one document (as the new document or as the source).
pub fn select<'a>(
    rows: &'a [DrawRow],
    project: Option<&str>,
    doc: Option<&str>,
) -> Vec<&'a DrawRow> {
    rows.iter()
        .filter(|r| {
            project.map(|p| r.project == p).unwrap_or(true)
                && doc
                    .map(|d| r.doc_id == d || r.drew_from == d)
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
            project: "P".into(),
            doc_id: "NEW".into(),
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
                project: "".into(),
                ..r
            }
        )
        .is_err());
        let _ = std::fs::remove_file(&p);
    }
}
