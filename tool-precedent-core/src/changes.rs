// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! The change log (`precedent/changes.csv`): one row per change per document
//! version, append-only. Generic part only; the Annex generator is document
//! specific and stays in tool-precedent-documents.

use std::path::Path;

pub const CHANGES_HEADER: &str =
    "instrument,version,baseline,section,baseline_text,new_text,tag,decision_ref,date";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeRow {
    pub instrument: String,
    pub version: String,
    pub baseline: String,
    pub section: String,
    pub baseline_text: String,
    pub new_text: String,
    pub tag: String,
    pub decision_ref: String,
    pub date: String,
}

pub fn valid_tag(tag: &str) -> bool {
    matches!(tag, "Added" | "Deleted" | "Replaced")
}

fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

pub fn read_changes(path: &Path) -> Result<Vec<ChangeRow>, String> {
    let text = match crate::csv::read_text(path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    let at = |e: String| format!("{}: {e}", path.display());
    let rows = crate::csv::parse_records(&text).map_err(at)?;
    let body = rows.get(1..).unwrap_or_default(); // after the header
    crate::csv::check_widths(body, "9", |n| n == 9).map_err(at)?;
    let mut out = Vec::new();
    for (_, r) in body {
        out.push(ChangeRow {
            instrument: r[0].clone(),
            version: r[1].clone(),
            baseline: r[2].clone(),
            section: r[3].clone(),
            baseline_text: r[4].clone(),
            new_text: r[5].clone(),
            tag: r[6].clone(),
            decision_ref: r[7].clone(),
            date: r[8].clone(),
        });
    }
    Ok(out)
}

/// Appends one row (creating the file with its header if absent). Append-only.
pub fn append_change(path: &Path, row: &ChangeRow) -> Result<(), String> {
    if !valid_tag(&row.tag) {
        return Err(format!(
            "tag must be Added, Deleted or Replaced, got {:?}",
            row.tag
        ));
    }
    read_changes(path)?; // never append to a file that does not read cleanly
    use std::io::Write;
    let mut f = crate::csv::open_for_append(path, CHANGES_HEADER)?;
    let fields = [
        &row.instrument,
        &row.version,
        &row.baseline,
        &row.section,
        &row.baseline_text,
        &row.new_text,
        &row.tag,
        &row.decision_ref,
        &row.date,
    ];
    let line: Vec<String> = fields.iter().map(|s| csv_field(s)).collect();
    writeln!(f, "{}", line.join(",")).map_err(|e| e.to_string())
}
