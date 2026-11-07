//! The change log (`precedent/changes.csv`): one row per change per document
//! version, append-only. Generic part only; the Annex generator is document
//! specific and stays in tool-precedent-documents.

use std::path::Path;

pub const CHANGES_HEADER: &str =
    "doc_id,version,baseline,section,baseline_text,new_text,tag,decision_ref,date";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChangeRow {
    pub doc_id: String,
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

use crate::csv::parse as parse_csv;

pub fn read_changes(path: &Path) -> Result<Vec<ChangeRow>, String> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    let mut out = Vec::new();
    for (i, r) in parse_csv(&text).into_iter().enumerate() {
        if i == 0 {
            continue; // header
        }
        if r.len() != 9 {
            return Err(format!(
                "{}: row {} has {} fields, expected 9",
                path.display(),
                i + 1,
                r.len()
            ));
        }
        out.push(ChangeRow {
            doc_id: r[0].clone(),
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
        writeln!(f, "{CHANGES_HEADER}").map_err(|e| e.to_string())?;
    }
    let fields = [
        &row.doc_id,
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
