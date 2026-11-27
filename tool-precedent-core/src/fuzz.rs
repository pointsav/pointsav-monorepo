// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Fuzz-ready entry points (Foundry Rust Profile rule 10): one small pure function per decoder. Each
//! takes raw bytes, never touches the file system, and returns a `Result`; malformed input is an
//! `Err`, never a panic. A fuzzer (or the std-only corpus test `tests/hostile_inputs.rs`) calls
//! these directly. The size cap is the one the file readers use (`csv::MAX_FILE_BYTES`).
use crate::error::{CoreError, Result};
use crate::{csv, fence, lineage, registry};

/// The largest input an entry point accepts: the file cap, as a length.
pub const MAX_INPUT_BYTES: usize = csv::MAX_FILE_BYTES as usize;

/// The input as UTF-8 text, refused when larger than `MAX_INPUT_BYTES` or not UTF-8.
pub fn text(input: &[u8]) -> Result<&str> {
    text_capped(input, MAX_INPUT_BYTES)
}

/// `text` with an explicit cap (tests use a small one to check the edge).
pub fn text_capped(input: &[u8], cap: usize) -> Result<&str> {
    if input.len() > cap {
        return Err(CoreError::TooLarge(format!(
            "{} bytes, the limit is {cap}",
            input.len()
        )));
    }
    std::str::from_utf8(input)
        .map_err(|e| CoreError::Other(format!("not UTF-8 at byte {}", e.valid_up_to())))
}

/// CSV: the number of records of a strict parse (an unbalanced quote is an error).
pub fn csv(input: &[u8]) -> Result<usize> {
    Ok(csv::parse_records(text(input)?)?.len())
}

/// A registry table (`ts-*.csv`): the number of rows. The rows are then also run, as every table at
/// once, through the pure generators (CSS, DTCG, class list, numeric checks), so hostile cells reach
/// them too.
pub fn registry_table(input: &[u8]) -> Result<usize> {
    let rows = registry::parse_table(text(input)?)?;
    let n = rows.len();
    let reg = registry::Registry {
        classes: rows.clone(),
        geometry: rows.clone(),
        labels: rows.clone(),
        glyphs: rows.clone(),
        sections: rows.clone(),
        families: rows.clone(),
        engine: rows.clone(),
        changes: rows,
    };
    let _ = reg.css_files(None);
    let _ = reg.dtcg(None);
    let _ = reg.classes_list(None);
    let _ = reg.numeric_problems();
    Ok(n)
}

/// HTML: the number of body elements the lint scanner reads (refused past `MAX_HTML_DEPTH`).
pub fn html(input: &[u8]) -> Result<usize> {
    registry::scan_html_count(text(input)?)
}

/// The per-document overrides of a style block (unbalanced markers are an error).
pub fn fence_overrides(input: &[u8]) -> Result<String> {
    fence::overrides_checked(text(input)?).map_err(CoreError::Other)
}

/// One file name: its version label, `None` when it carries no version.
pub fn lineage_name(input: &[u8]) -> Result<Option<String>> {
    Ok(lineage::parse_name(text(input)?, lineage::DEFAULT_INITIALS).map(|p| p.label()))
}

/// Lineage from a list of file names, one per line; a line with two tabs is a draw row
/// (`base<TAB>version<TAB>drew_from`). Returns the number of instruments of the report.
pub fn lineage_build(input: &[u8]) -> Result<usize> {
    let mut files: Vec<String> = Vec::new();
    let mut draws: Vec<lineage::Draw> = Vec::new();
    for l in text(input)?.lines() {
        let mut it = l.split('\t');
        match (it.next(), it.next(), it.next()) {
            (Some(b), Some(v), Some(f)) => draws.push((b.into(), v.into(), f.into())),
            _ => files.push(l.to_string()),
        }
    }
    let r = lineage::report(&files, lineage::DEFAULT_INITIALS, &draws);
    Ok(r.instruments)
}
