// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! moonshot-folio-csv (L4): one IR sheet to RFC 4180 CSV.
//!
//! Records end in CRLF. A field is quoted when it holds a comma, quote, CR or LF; quotes are
//! doubled. Text cells that a spreadsheet would read as a formula (leading `=`, `+`, `-`, `@`,
//! tab or CR) get a leading apostrophe unless [`CsvOptions::neutralise_formulas`] is cleared.
//! Number cells are never altered. The apostrophe is visible when a program imports the CSV as data
//! (it is only a text marker inside a spreadsheet cell), which is the accepted cost of the OWASP
//! mitigation; clear the option when the consumer is not a spreadsheet. Control characters other
//! than tab, CR and LF are rejected.

#![forbid(unsafe_code)]

use moonshot_folio_core::{FolioError, Limits, Result};
use moonshot_folio_ir::{check_target, validate, Body, CellValue, Document, Target};

/// Output choices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CsvOptions {
    /// Write a UTF-8 byte-order mark first (some spreadsheet programs need it to detect UTF-8).
    pub bom: bool,
    /// Prefix formula-looking text with `'` so it opens as text.
    pub neutralise_formulas: bool,
}

impl Default for CsvOptions {
    fn default() -> Self {
        Self {
            bom: false,
            neutralise_formulas: true,
        }
    }
}

/// Render sheet `sheet` of a workbook document as CSV bytes.
///
/// # Errors
/// Any validation failure, `Unsupported` for a flow body or merged cells, `Invalid` when `sheet`
/// is out of range, and `Limit` when the output would exceed `limits.max_file_bytes()`.
pub fn render_csv(
    doc: &Document,
    sheet: usize,
    opts: CsvOptions,
    limits: &Limits,
) -> Result<Vec<u8>> {
    validate(doc, limits)?;
    check_target(doc, Target::Csv)?;
    let Body::Workbook(sheets) = &doc.body else {
        return Err(FolioError::Unsupported("CSV needs a workbook body"));
    };
    let s = sheets
        .get(sheet)
        .ok_or(FolioError::Invalid("sheet index out of range"))?;
    let has_ctl = s.rows.iter().flat_map(|r| &r.cells).any(|c| {
        matches!(&c.value, CellValue::Text(t) if t.chars().any(|ch| ch.is_control() && !matches!(ch, '\t' | '\r' | '\n')))
    });
    if has_ctl {
        return Err(FolioError::Invalid("text contains a control character"));
    }
    let mut out = String::new();
    if opts.bom {
        out.push('\u{FEFF}');
    }
    for row in &s.rows {
        for (i, c) in row.cells.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            field(&mut out, &c.value, opts);
        }
        out.push_str("\r\n");
        if out.len() > limits.max_file_bytes() {
            return Err(FolioError::Limit("csv output size"));
        }
    }
    Ok(out.into_bytes())
}

fn field(out: &mut String, v: &CellValue, opts: CsvOptions) {
    match v {
        CellValue::Text(t) => {
            let risky = opts.neutralise_formulas && t.starts_with(['=', '+', '-', '@', '\t', '\r']);
            if risky {
                let mut s = String::from("'");
                s.push_str(t);
                quoted(out, &s);
            } else {
                quoted(out, t);
            }
        }
        CellValue::Number(n) => out.push_str(n),
        CellValue::Bool(true) => out.push_str("TRUE"),
        CellValue::Bool(false) => out.push_str("FALSE"),
        CellValue::Empty => {}
    }
}

/// Append `t`, quoted only when needed.
fn quoted(out: &mut String, t: &str) {
    if t.contains([',', '"', '\r', '\n']) {
        out.push('"');
        for c in t.chars() {
            if c == '"' {
                out.push('"');
            }
            out.push(c);
        }
        out.push('"');
    } else {
        out.push_str(t);
    }
}
