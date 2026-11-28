// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! moonshot-folio-xlsx (L4): IR workbook to `SpreadsheetML`, packaged as a deterministic
//! store-only ZIP. Strings are inline (no shared-string table); every formula carries its
//! caller-supplied cached value, which is the cell's own value.
//!
//! Design: BRIEF-moonshot-folio-architecture. Tasks F6.2, F6.3.

#![forbid(unsafe_code)]

mod sheet;
mod sheet_extra;
mod styles;
mod workbook;
mod xml;

pub use styles::{Align, CellStyle, Rule, StyleMap};

use moonshot_folio_codec::ZipWriter;
use moonshot_folio_core::{FolioError, Limits, Result};
use moonshot_folio_ir::{check_target, validate, Body, Document, Target};

/// Render a workbook document as a `.xlsx` package.
///
/// # Errors
/// Any [`validate`] failure, `Unsupported` for a flow body or CMYK colour, `Invalid` for a bad
/// number format or text XML cannot carry, and `Limit` when `limits` or sheet bounds are exceeded.
pub fn render_xlsx(doc: &Document, map: &StyleMap, limits: &Limits) -> Result<Vec<u8>> {
    validate(doc, limits)?;
    check_target(doc, Target::Xlsx)?;
    let Body::Workbook(sheets) = &doc.body else {
        return Err(FolioError::Unsupported("XLSX needs a workbook body"));
    };
    let mut reg = styles::Registry::new();
    let mut parts = Vec::new();
    for (i, s) in sheets.iter().enumerate() {
        parts.push(sheet::sheet_xml(s, i == 0, map, &mut reg)?);
    }
    let mut zip = ZipWriter::new(*limits);
    zip.add(
        "[Content_Types].xml",
        workbook::content_types(sheets.len()).as_bytes(),
    )?;
    zip.add("_rels/.rels", workbook::root_rels().as_bytes())?;
    zip.add(
        "xl/workbook.xml",
        workbook::workbook_xml(sheets)?.as_bytes(),
    )?;
    zip.add(
        "xl/_rels/workbook.xml.rels",
        workbook::workbook_rels(sheets.len()).as_bytes(),
    )?;
    zip.add("xl/styles.xml", styles::styles_xml(&reg)?.as_bytes())?;
    for (i, p) in parts.iter().enumerate() {
        zip.add(
            &format!("xl/worksheets/sheet{}.xml", i.saturating_add(1)),
            p.as_bytes(),
        )?;
    }
    zip.add(
        "docProps/core.xml",
        workbook::core_props(&doc.meta)?.as_bytes(),
    )?;
    zip.finish()
}
