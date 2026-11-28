// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Workbook-level parts.

use crate::sheet::col_letters;
use crate::xml::escape_into;
use core::fmt::Write;
use moonshot_folio_core::Result;
use moonshot_folio_ir::{Meta, Sheet};

const DECL: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n";
const NS: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
const NS_R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const NS_REL: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

pub(crate) fn content_types(sheets: usize) -> String {
    let mut s = format!("{DECL}<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/><Default Extension=\"xml\" ContentType=\"application/xml\"/><Override PartName=\"/xl/workbook.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml\"/><Override PartName=\"/xl/styles.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml\"/><Override PartName=\"/docProps/core.xml\" ContentType=\"application/vnd.openxmlformats-package.core-properties+xml\"/>");
    for i in 1..=sheets {
        write!(s, "<Override PartName=\"/xl/worksheets/sheet{i}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml\"/>").ok();
    }
    s.push_str("</Types>");
    s
}

pub(crate) fn root_rels() -> String {
    format!("{DECL}<Relationships xmlns=\"{NS_REL}\"><Relationship Id=\"rId1\" Type=\"{NS_R}/officeDocument\" Target=\"xl/workbook.xml\"/><Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties\" Target=\"docProps/core.xml\"/></Relationships>")
}

pub(crate) fn workbook_rels(sheets: usize) -> String {
    let mut s = format!("{DECL}<Relationships xmlns=\"{NS_REL}\">");
    for i in 1..=sheets {
        write!(s, "<Relationship Id=\"rId{i}\" Type=\"{NS_R}/worksheet\" Target=\"worksheets/sheet{i}.xml\"/>").ok();
    }
    write!(
        s,
        "<Relationship Id=\"rId{}\" Type=\"{NS_R}/styles\" Target=\"styles.xml\"/></Relationships>",
        sheets.saturating_add(1)
    )
    .ok();
    s
}

/// `'Sheet name'!$A$1:$B$2`, with the sheet name always quoted.
fn range(sheet: &str, n: &moonshot_folio_ir::DefinedName) -> String {
    let quoted = sheet.replace('\'', "''");
    let abs = |r: usize, c: u32| format!("${}${}", col_letters(c), r.saturating_add(1));
    let r0 = usize::try_from(n.row).unwrap_or(0);
    let r1 = usize::try_from(n.row.saturating_add(n.rows).saturating_sub(1)).unwrap_or(0);
    let c1 = n.col.saturating_add(n.cols).saturating_sub(1);
    format!("'{quoted}'!{}:{}", abs(r0, n.col), abs(r1, c1))
}

pub(crate) fn workbook_xml(sheets: &[Sheet]) -> Result<String> {
    let mut s = format!("{DECL}<workbook xmlns=\"{NS}\" xmlns:r=\"{NS_R}\"><bookViews><workbookView/></bookViews><sheets>");
    for (i, sh) in sheets.iter().enumerate() {
        let n = i.saturating_add(1);
        s.push_str("<sheet name=\"");
        escape_into(&mut s, &sh.name)?;
        write!(s, "\" sheetId=\"{n}\" r:id=\"rId{n}\"/>").ok();
    }
    s.push_str("</sheets>");
    let mut names = String::new();
    for sh in sheets {
        for n in &sh.names {
            names.push_str("<definedName name=\"");
            escape_into(&mut names, &n.name)?;
            names.push_str("\">");
            escape_into(&mut names, &range(&sh.name, n))?;
            names.push_str("</definedName>");
        }
    }
    if !names.is_empty() {
        write!(s, "<definedNames>{names}</definedNames>").ok();
    }
    s.push_str("</workbook>");
    Ok(s)
}

pub(crate) fn core_props(m: &Meta) -> Result<String> {
    let mut s = format!("{DECL}<cp:coreProperties xmlns:cp=\"http://schemas.openxmlformats.org/package/2006/metadata/core-properties\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:dcterms=\"http://purl.org/dc/terms/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"><dc:title>");
    escape_into(&mut s, &m.title)?;
    s.push_str("</dc:title><dc:creator>");
    escape_into(&mut s, &m.author)?;
    s.push_str("</dc:creator><dc:language>");
    escape_into(&mut s, &m.lang)?;
    s.push_str("</dc:language><dcterms:created xsi:type=\"dcterms:W3CDTF\">");
    escape_into(&mut s, &m.date)?;
    s.push_str("T00:00:00Z</dcterms:created></cp:coreProperties>");
    Ok(s)
}
