// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Table writer (`w:tbl`). Every cell ends with a paragraph, as the schema requires.

use crate::ctx::{style_id, Ctx};
use crate::xml::escape_into;
use core::fmt::Write;
use moonshot_folio_core::{FolioError, Result};
use moonshot_folio_ir::{Cell, CellValue, Row, Table};

/// Equal column widths over `total` twips; the remainder goes to the last column.
fn grid(total: i64, columns: u32) -> Result<Vec<i64>> {
    let n = i64::from(columns);
    let base = total
        .checked_div(n)
        .ok_or(FolioError::Invalid("zero columns"))?;
    let mut v = vec![base; usize::try_from(columns).map_err(|_| FolioError::Limit("columns"))?];
    let rem = total.saturating_sub(base.saturating_mul(n));
    if let Some(last) = v.last_mut() {
        *last = last.saturating_add(rem);
    }
    Ok(v)
}

pub(crate) fn table(out: &mut String, t: &Table, cx: &mut Ctx) -> Result<()> {
    let widths = grid(cx.content_twips, t.columns)?;
    out.push_str("<w:tbl><w:tblPr><w:tblW w:w=\"");
    write!(out, "{}", cx.content_twips).ok();
    out.push_str("\" w:type=\"dxa\"/><w:tblLayout w:type=\"fixed\"/></w:tblPr><w:tblGrid>");
    for w in &widths {
        write!(out, "<w:gridCol w:w=\"{w}\"/>").ok();
    }
    out.push_str("</w:tblGrid>");
    let header = usize::try_from(t.repeat_header_rows).unwrap_or(usize::MAX);
    for (i, r) in t.rows.iter().enumerate() {
        row(out, r, i < header, &widths, cx)?;
    }
    out.push_str("</w:tbl>");
    Ok(())
}

fn row(out: &mut String, r: &Row, header: bool, widths: &[i64], cx: &mut Ctx) -> Result<()> {
    out.push_str("<w:tr>");
    if header || !r.can_split {
        out.push_str("<w:trPr>");
        if !r.can_split {
            out.push_str("<w:cantSplit/>");
        }
        if header {
            out.push_str("<w:tblHeader/>");
        }
        out.push_str("</w:trPr>");
    }
    let mut col = 0_usize;
    for c in &r.cells {
        let span = usize::try_from(c.span).map_err(|_| FolioError::Limit("span"))?;
        let end = col.saturating_add(span);
        let w: i64 = widths.get(col..end).unwrap_or(&[]).iter().sum();
        cell(out, c, w, cx)?;
        col = end;
    }
    out.push_str("</w:tr>");
    Ok(())
}

fn cell(out: &mut String, c: &Cell, width: i64, cx: &mut Ctx) -> Result<()> {
    write!(out, "<w:tc><w:tcPr><w:tcW w:w=\"{width}\" w:type=\"dxa\"/>").ok();
    if c.span > 1 {
        write!(out, "<w:gridSpan w:val=\"{}\"/>", c.span).ok();
    }
    out.push_str("</w:tcPr><w:p><w:pPr>");
    if !c.role.is_empty() {
        cx.para_roles.insert(c.role.clone());
        write!(out, "<w:pStyle w:val=\"{}\"/>", style_id('P', &c.role)).ok();
    }
    if matches!(c.value, CellValue::Number(_)) {
        out.push_str("<w:jc w:val=\"right\"/>");
    }
    out.push_str("</w:pPr>");
    let text = match &c.value {
        CellValue::Text(t) | CellValue::Number(t) => t.as_str(),
        CellValue::Bool(true) => "TRUE",
        CellValue::Bool(false) => "FALSE",
        CellValue::Empty => "",
    };
    if !text.is_empty() {
        out.push_str("<w:r><w:t xml:space=\"preserve\">");
        escape_into(out, text)?;
        out.push_str("</w:t></w:r>");
    }
    out.push_str("</w:p></w:tc>");
    Ok(())
}
