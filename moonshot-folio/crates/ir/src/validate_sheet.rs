// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Worksheet-specific invariants: widths, frozen panes, defined names and formulas.

use crate::model::{
    CellValue, CondFormat, DefinedName, Formula, Range, Sheet, Validation, ValidationKind,
};
use moonshot_folio_core::{FolioError, Result};

/// Functions a formula may call. Anything else is rejected, so no output can reach outside the file.
pub const FORMULA_FUNCTIONS: [&str; 10] = [
    "SUM", "AVERAGE", "MIN", "MAX", "COUNT", "ROUND", "ABS", "IF", "AND", "OR",
];

/// Check the parts of `s` beyond the grid rules.
pub(crate) fn check_extras(s: &Sheet, limits: &moonshot_folio_core::Limits) -> Result<()> {
    let rows = u32::try_from(s.rows.len()).map_err(|_| FolioError::Limit("rows"))?;
    let cap = limits.max_cells();
    if s.formulas.len() > cap || s.validations.len() > cap || s.cond_formats.len() > cap {
        return Err(FolioError::Limit(
            "formulas, validations or conditional formats",
        ));
    }
    if s.freeze_cols > s.columns {
        return Err(FolioError::Invalid("more frozen columns than columns"));
    }
    let widths_ok = s.col_widths.len() <= usize::try_from(s.columns).unwrap_or(usize::MAX)
        && s.col_widths.iter().all(|w| *w <= 25_500);
    if !widths_ok {
        return Err(FolioError::Invalid("column widths out of range"));
    }
    for n in &s.names {
        check_name(n, rows, s.columns)?;
    }
    for v in &s.validations {
        check_range(&v.range, rows, s.columns)?;
        check_validation(v)?;
    }
    for c in &s.cond_formats {
        check_range(&c.range, rows, s.columns)?;
        check_cond(c)?;
    }
    let mut seen: std::collections::BTreeSet<(u32, u32)> = std::collections::BTreeSet::new();
    for f in &s.formulas {
        check_formula(s, f)?;
        if !seen.insert((f.row, f.col)) {
            return Err(FolioError::Invalid("two formulas on one cell"));
        }
    }
    Ok(())
}

/// Defined-name syntax: starts with a letter or `_`, continues with letters, digits, `_` or `.`,
/// is at most 255 long, and does not read as a cell reference such as `A1` or `R2C3`.
pub(crate) fn name_ok(n: &str) -> bool {
    let b = n.as_bytes();
    let first = b
        .first()
        .is_some_and(|c| c.is_ascii_alphabetic() || *c == b'_');
    let rest = b
        .iter()
        .all(|c| c.is_ascii_alphanumeric() || *c == b'_' || *c == b'.');
    first
        && rest
        && b.len() <= 255
        && !looks_like_cell(n)
        && !looks_like_r1c1(n)
        && !n.to_ascii_lowercase().starts_with("_xlnm.")
}

fn looks_like_cell(n: &str) -> bool {
    let letters = n.bytes().take_while(u8::is_ascii_alphabetic).count();
    let digits = n.len().saturating_sub(letters);
    let tail_numeric = n
        .get(letters..)
        .is_some_and(|t| !t.is_empty() && t.bytes().all(|c| c.is_ascii_digit()));
    let r1c1 = n.len() <= 2 && matches!(n.to_ascii_uppercase().as_str(), "R" | "C");
    (letters > 0 && letters <= 3 && tail_numeric && digits > 0) || r1c1
}

/// `R`, `C`, `RC`, `R12`, `C3`, `R1C1`, `RC5` and similar forms Excel reads as R1C1 references.
fn looks_like_r1c1(n: &str) -> bool {
    let u = n.to_ascii_uppercase();
    let mut rest = u.as_str();
    let mut parts = 0_u8;
    for tag in ['R', 'C'] {
        if let Some(r) = rest.strip_prefix(tag) {
            rest = r.trim_start_matches(|c: char| c.is_ascii_digit());
            parts = parts.saturating_add(1);
        }
    }
    parts > 0 && rest.is_empty()
}

fn check_name(n: &DefinedName, rows: u32, columns: u32) -> Result<()> {
    if !name_ok(&n.name) {
        return Err(FolioError::Invalid("defined name is not valid"));
    }
    let fits = n.rows >= 1
        && n.cols >= 1
        && n.row.checked_add(n.rows).is_some_and(|e| e <= rows)
        && n.col.checked_add(n.cols).is_some_and(|e| e <= columns);
    if fits {
        Ok(())
    } else {
        Err(FolioError::Invalid(
            "defined name range is outside the sheet",
        ))
    }
}

/// The value of the cell whose first column is `col` in row `row`.
fn cell_at(s: &Sheet, row: u32, col: u32) -> Option<&CellValue> {
    let r = s.rows.get(usize::try_from(row).ok()?)?;
    let mut start = 0_u32;
    for c in &r.cells {
        if start == col {
            return Some(&c.value);
        }
        start = start.saturating_add(c.span);
    }
    None
}

fn check_formula(s: &Sheet, f: &Formula) -> Result<()> {
    match cell_at(s, f.row, f.col) {
        None => return Err(FolioError::Invalid("formula is not on a cell")),
        Some(CellValue::Empty) => {
            return Err(FolioError::Invalid("formula cell needs a cached value"));
        }
        Some(_) => {}
    }
    check_formula_text(&f.text)
}

/// Whitelist check on formula text.
///
/// # Errors
/// `Invalid` for an empty, over-long or `=`-prefixed formula, a forbidden character, or a call to
/// a function outside [`FORMULA_FUNCTIONS`].
pub fn check_formula_text(t: &str) -> Result<()> {
    if t.is_empty() || t.len() > 8192 || t.starts_with('=') {
        return Err(FolioError::Invalid(
            "formula is empty, too long or starts with =",
        ));
    }
    let mut in_str = false;
    let mut depth = 0_u32;
    let mut word = String::new();
    for c in t.chars() {
        if c == '"' {
            in_str = !in_str;
            word.clear();
            continue;
        }
        if in_str {
            if c.is_control() {
                return Err(FolioError::Invalid("control character in formula text"));
            }
            continue;
        }
        let allowed = c.is_ascii_alphanumeric() || " .,:$()+-*/^<>=&%!_".contains(c);
        if !allowed {
            return Err(FolioError::Invalid(
                "formula contains a forbidden character",
            ));
        }
        if c.is_ascii_alphanumeric() || c == '_' || c == '.' {
            word.push(c);
        } else if c == ' ' {
            // A space before `(` must not hide the function name.
        } else {
            if c == ')' {
                depth = depth
                    .checked_sub(1)
                    .ok_or(FolioError::Invalid("unbalanced parentheses in formula"))?;
            }
            if c == '(' {
                depth = depth.saturating_add(1);
                if depth > 64 {
                    return Err(FolioError::Invalid("formula nests deeper than 64 levels"));
                }
            }
            if c == '(' && !function_ok(&word) {
                return Err(FolioError::Invalid(
                    "formula calls a function outside the whitelist",
                ));
            }
            word.clear();
        }
    }
    if in_str {
        return Err(FolioError::Invalid("unterminated string in formula"));
    }
    check_edges(t)?;
    if depth != 0 {
        return Err(FolioError::Invalid("unbalanced parentheses in formula"));
    }
    Ok(())
}

fn function_ok(w: &str) -> bool {
    w.is_empty() || FORMULA_FUNCTIONS.contains(&w.to_ascii_uppercase().as_str())
}

fn check_range(r: &Range, rows: u32, columns: u32) -> Result<()> {
    let fits = r.rows >= 1
        && r.cols >= 1
        && r.row.checked_add(r.rows).is_some_and(|e| e <= rows)
        && r.col.checked_add(r.cols).is_some_and(|e| e <= columns);
    if fits {
        Ok(())
    } else {
        Err(FolioError::Invalid("range is outside the sheet"))
    }
}

fn check_validation(v: &Validation) -> Result<()> {
    match &v.kind {
        ValidationKind::List(items) => {
            let total: usize = items
                .iter()
                .map(String::len)
                .sum::<usize>()
                .saturating_add(items.len().saturating_sub(1));
            let ok = !items.is_empty()
                && total <= 255
                && items.iter().all(|i| {
                    !i.is_empty() && !i.contains([',', '"']) && !i.chars().any(char::is_control)
                });
            if ok {
                Ok(())
            } else {
                Err(FolioError::Invalid(
                    "validation list is empty, too long or has a bad item",
                ))
            }
        }
        ValidationKind::WholeNumber { min, max } => {
            if min <= max {
                Ok(())
            } else {
                Err(FolioError::Invalid("validation minimum exceeds maximum"))
            }
        }
    }
}

fn check_cond(c: &CondFormat) -> Result<()> {
    if c.role.is_empty() {
        return Err(FolioError::Invalid("conditional format needs a style role"));
    }
    if crate::validate::is_decimal(&c.value) {
        Ok(())
    } else {
        Err(FolioError::Invalid(
            "conditional format value is not a plain decimal",
        ))
    }
}

/// A formula may not start or end with a bare operator.
fn check_edges(t: &str) -> Result<()> {
    let t = t.trim();
    if t.ends_with(['+', '-', '*', '/', '^', '&', '=', '<', '>', ',', ':', '!'])
        || t.starts_with(['*', '/', '^', '&', ',', ')', ':', '!', '<', '>', '='])
    {
        return Err(FolioError::Invalid(
            "formula starts or ends with an operator",
        ));
    }
    Ok(())
}
