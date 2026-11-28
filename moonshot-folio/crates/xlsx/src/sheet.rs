// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! One worksheet part (`xl/worksheets/sheetN.xml`).

use crate::styles::{Registry, StyleMap};
use crate::xml::escape_into;
use core::fmt::Write;
use moonshot_folio_core::{FolioError, Result};
use moonshot_folio_ir::{CellValue, Sheet};

const MAX_COLS: u32 = 16_384;
const MAX_ROWS: usize = 1_048_576;

/// Column letters for a 0-based index (`0` is `A`, `26` is `AA`).
pub(crate) fn col_letters(mut i: u32) -> String {
    let mut v = Vec::new();
    loop {
        let r = u8::try_from(i % 26).unwrap_or(0);
        v.push(b'A'.saturating_add(r));
        if i < 26 {
            break;
        }
        i = (i / 26).saturating_sub(1);
    }
    v.reverse();
    String::from_utf8(v).unwrap_or_default()
}

/// `A1`-style reference for 0-based `(row, col)`.
pub(crate) fn cell_ref(row: usize, col: u32) -> String {
    format!("{}{}", col_letters(col), row.saturating_add(1))
}

/// Column width in hundredths of a character as a decimal string, trailing zeros trimmed.
fn width(w: u32) -> String {
    let (i, f) = (w / 100, w % 100);
    if f == 0 {
        i.to_string()
    } else {
        format!("{i}.{f:02}").trim_end_matches('0').to_string()
    }
}

fn head(s: &Sheet, first: bool, out: &mut String) {
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<worksheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">");
    if s.landscape_fit_width {
        out.push_str("<sheetPr><pageSetUpPr fitToPage=\"1\"/></sheetPr>");
    }
    let sel = if first { " tabSelected=\"1\"" } else { "" };
    write!(out, "<sheetViews><sheetView{sel} workbookViewId=\"0\">").ok();
    if s.freeze_rows > 0 || s.freeze_cols > 0 {
        let top = cell_ref(usize::try_from(s.freeze_rows).unwrap_or(0), s.freeze_cols);
        let pane = match (s.freeze_cols > 0, s.freeze_rows > 0) {
            (true, true) => "bottomRight",
            (true, false) => "topRight",
            _ => "bottomLeft",
        };
        out.push_str("<pane");
        if s.freeze_cols > 0 {
            write!(out, " xSplit=\"{}\"", s.freeze_cols).ok();
        }
        if s.freeze_rows > 0 {
            write!(out, " ySplit=\"{}\"", s.freeze_rows).ok();
        }
        write!(
            out,
            " topLeftCell=\"{top}\" activePane=\"{pane}\" state=\"frozen\"/>"
        )
        .ok();
    }
    out.push_str("</sheetView></sheetViews><sheetFormatPr defaultRowHeight=\"15\"/>");
    if s.col_widths.iter().any(|w| *w > 0) {
        out.push_str("<cols>");
        for (i, w) in s.col_widths.iter().enumerate().filter(|(_, w)| **w > 0) {
            let n = i.saturating_add(1);
            write!(
                out,
                "<col min=\"{n}\" max=\"{n}\" width=\"{}\" customWidth=\"1\"/>",
                width(*w)
            )
            .ok();
        }
        out.push_str("</cols>");
    }
}

/// Render one sheet.
pub(crate) fn sheet_xml(
    s: &Sheet,
    first: bool,
    map: &StyleMap,
    reg: &mut Registry,
) -> Result<String> {
    if s.columns > MAX_COLS || s.rows.len() > MAX_ROWS {
        return Err(FolioError::Limit("sheet dimensions"));
    }
    let mut out = String::new();
    head(s, first, &mut out);
    out.push_str("<sheetData>");
    let formulas: std::collections::BTreeMap<(usize, u32), &str> = s
        .formulas
        .iter()
        .filter_map(|f| Some(((usize::try_from(f.row).ok()?, f.col), f.text.as_str())))
        .collect();
    let mut merges: Vec<String> = Vec::new();
    for (r, row) in s.rows.iter().enumerate() {
        write!(out, "<row r=\"{}\">", r.saturating_add(1)).ok();
        let mut col = 0_u32;
        for c in &row.cells {
            let rf = cell_ref(r, col);
            let style = reg.index(map, &c.role);
            let formula = formulas.get(&(r, col));
            cell(&mut out, &rf, style, &c.value, formula.copied())?;
            if c.span > 1 {
                let end = col.saturating_add(c.span).saturating_sub(1);
                merges.push(format!("{rf}:{}", cell_ref(r, end)));
            }
            col = col.saturating_add(c.span);
        }
        out.push_str("</row>");
    }
    out.push_str("</sheetData>");
    if !merges.is_empty() {
        write!(out, "<mergeCells count=\"{}\">", merges.len()).ok();
        for m in &merges {
            write!(out, "<mergeCell ref=\"{m}\"/>").ok();
        }
        out.push_str("</mergeCells>");
    }
    crate::sheet_extra::conditional(&mut out, s, map, reg)?;
    crate::sheet_extra::validations(&mut out, s)?;
    out.push_str("<pageMargins left=\"0.7\" right=\"0.7\" top=\"0.75\" bottom=\"0.75\" header=\"0.3\" footer=\"0.3\"/>");
    if s.landscape_fit_width {
        out.push_str("<pageSetup orientation=\"landscape\" fitToWidth=\"1\" fitToHeight=\"0\"/>");
    }
    out.push_str("</worksheet>");
    Ok(out)
}

/// One `<c>`; a formula cell carries its cached value in `<v>`.
fn cell(
    out: &mut String,
    rf: &str,
    style: usize,
    v: &CellValue,
    formula: Option<&str>,
) -> Result<()> {
    let s = if style > 0 {
        format!(" s=\"{style}\"")
    } else {
        String::new()
    };
    let f = match formula {
        Some(t) => {
            let mut x = String::from("<f>");
            escape_into(&mut x, t)?;
            x.push_str("</f>");
            x
        }
        None => String::new(),
    };
    match v {
        CellValue::Empty => write!(out, "<c r=\"{rf}\"{s}/>").ok(),
        CellValue::Number(n) => write!(out, "<c r=\"{rf}\"{s}>{f}<v>{n}</v></c>").ok(),
        CellValue::Bool(b) => write!(
            out,
            "<c r=\"{rf}\"{s} t=\"b\">{f}<v>{}</v></c>",
            u8::from(*b)
        )
        .ok(),
        CellValue::Text(t) => {
            if formula.is_some() {
                write!(out, "<c r=\"{rf}\"{s} t=\"str\">{f}<v>").ok();
            } else {
                write!(
                    out,
                    "<c r=\"{rf}\"{s} t=\"inlineStr\"><is><t xml:space=\"preserve\">"
                )
                .ok();
            }
            escape_into(out, t)?;
            out.push_str(if formula.is_some() {
                "</v></c>"
            } else {
                "</t></is></c>"
            });
            Some(())
        }
    };
    Ok(())
}
