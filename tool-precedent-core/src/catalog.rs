// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Flat-file shard catalog: finds every `*.csv` shard under a directory of a root (default
//! `precedent/`) and reads any of them as a generic table, without knowing its columns in
//! advance. There is no master register; the files are the catalog. Reads only.

use crate::csv;
use crate::error::{CoreError, Result};
use crate::tree::{Root, Stamp};
use std::fs;

pub const DEFAULT_DIR: &str = "precedent";

/// One CSV file read as header + rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn parse(text: &str) -> Result<Table> {
        let mut all = csv::parse(text);
        if all.is_empty() {
            return Err(CoreError::Other("empty shard: no header row".into()));
        }
        let columns = all.remove(0);
        Ok(Table { columns, rows: all })
    }

    pub fn column(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c == name)
    }

    /// Rows whose `column` equals `value`; an unknown column is an error, not an empty result.
    pub fn select(&self, column: &str, value: &str) -> Result<Vec<&Vec<String>>> {
        let i = self
            .column(column)
            .ok_or_else(|| CoreError::Other(format!("no column {column}")))?;
        Ok(self
            .rows
            .iter()
            .filter(|r| r.get(i).map(String::as_str) == Some(value))
            .collect())
    }

    /// Rows whose width differs from the header.
    pub fn ragged_rows(&self) -> usize {
        self.rows
            .iter()
            .filter(|r| r.len() != self.columns.len())
            .count()
    }
}

/// A shard found by `scan`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shard {
    /// Path relative to the root, e.g. `precedent/terms.csv`.
    pub path: String,
    pub columns: Vec<String>,
    pub rows: usize,
    pub ragged: usize,
    pub stamp: Stamp,
}

/// Read one shard by root-relative path.
pub fn read_table(root: &Root, rel: &str) -> Result<(Table, Stamp)> {
    let (text, stamp) = root.read(rel)?;
    Ok((Table::parse(&text)?, stamp))
}

/// List the `*.csv` shards directly inside `dir` (root-relative), sorted by path. A missing
/// directory is an empty catalog; a shard that cannot be read or has no header is an error.
pub fn scan(root: &Root, dir: &str) -> Result<Vec<Shard>> {
    let d = root.resolve(dir)?;
    if !d.is_dir() {
        return Ok(vec![]);
    }
    let mut names: Vec<String> = fs::read_dir(&d)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.ends_with(".csv") && !n.starts_with('.'))
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|n| {
            let path = format!("{dir}/{n}");
            let (t, stamp) = read_table(root, &path)?;
            Ok(Shard {
                path,
                ragged: t.ragged_rows(),
                rows: t.rows.len(),
                columns: t.columns,
                stamp,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn scratch() -> std::path::PathBuf {
        static N: AtomicUsize = AtomicUsize::new(0);
        let d = std::env::temp_dir().join(format!(
            "tpc-cat-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir_all(d.join("precedent")).unwrap();
        d
    }

    #[test]
    fn scans_and_describes_shards() {
        let d = scratch();
        fs::write(
            d.join("precedent/terms.csv"),
            "doc_id,version,term\nA,1,\"x, y\"\nB,1,z\n",
        )
        .unwrap();
        fs::write(d.join("precedent/changes.csv"), "doc_id,section\nA,s1\nA\n").unwrap();
        fs::write(d.join("precedent/notes.txt"), "ignored").unwrap();
        let root = Root::new(&d).unwrap();
        let shards = scan(&root, DEFAULT_DIR).unwrap();
        let got: Vec<_> = shards
            .iter()
            .map(|s| (s.path.as_str(), s.rows, s.ragged))
            .collect();
        assert_eq!(
            got,
            vec![
                ("precedent/changes.csv", 2, 1),
                ("precedent/terms.csv", 2, 0)
            ]
        );
        assert_eq!(shards[1].columns, vec!["doc_id", "version", "term"]);
        let _ = fs::remove_dir_all(d);
    }

    #[test]
    fn select_reads_generic_table() {
        let t = Table::parse("doc_id,term\nA,x\nB,y\nA,z\n").unwrap();
        assert_eq!(t.select("doc_id", "A").unwrap().len(), 2);
        assert!(t.select("nope", "A").is_err());
        assert!(Table::parse("").is_err());
    }

    #[test]
    fn missing_dir_is_empty_and_escape_refused() {
        let d = scratch();
        let root = Root::new(&d).unwrap();
        assert!(scan(&root, "absent").unwrap().is_empty());
        assert!(matches!(
            scan(&root, "../x"),
            Err(CoreError::OutsideRoot(_))
        ));
        let _ = fs::remove_dir_all(d);
    }
}
