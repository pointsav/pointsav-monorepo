// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! DOCX table tests (task F3.3).

#![allow(clippy::unwrap_used)]

mod common;
use common::{part, well_formed};
use moonshot_folio_core::{Emu, Limits};
use moonshot_folio_docx::render_docx;
use moonshot_folio_ir::{
    Block, Body, Cell, CellValue, Document, Meta, PageSetup, Row, Section, Table,
};

fn cell(v: CellValue, span: u32, role: &str) -> Cell {
    Cell {
        value: v,
        span,
        role: role.into(),
    }
}
fn doc(t: Table, width_pt: i64) -> Document {
    let page = PageSetup {
        width: Emu::from_points(width_pt).unwrap(),
        height: Emu::from_points(792).unwrap(),
        margin: Emu::from_points(72).unwrap(),
    };
    Document {
        meta: Meta {
            title: "T".into(),
            author: "A".into(),
            date: "2026-11-22".into(),
            lang: "en".into(),
        },
        body: Body::Flow(vec![Section::new(page, vec![Block::Table(t)])]),
    }
}
fn xml(d: &Document) -> String {
    part(
        &render_docx(d, &Limits::default()).unwrap(),
        "word/document.xml",
    )
}

// REQ: FOLIO-REQ-028
#[test]
fn grid_spans_header_and_cantsplit() {
    let t = Table {
        columns: 3,
        repeat_header_rows: 1,
        rows: vec![
            Row {
                cells: vec![
                    cell(CellValue::Text("h".into()), 2, "header"),
                    cell(CellValue::Empty, 1, ""),
                ],
                can_split: false,
            },
            Row {
                cells: vec![
                    cell(CellValue::Number("1.50".into()), 1, ""),
                    cell(CellValue::Bool(true), 1, ""),
                    cell(CellValue::Text("a & b".into()), 1, ""),
                ],
                can_split: true,
            },
        ],
    };
    // Content width: 612pt - 144pt = 468pt = 9360 twips -> 3120 each.
    let x = xml(&doc(t, 612));
    assert!(well_formed(&x), "{x}");
    assert_eq!(x.matches("<w:gridCol w:w=\"3120\"/>").count(), 3);
    assert!(x.contains("<w:tblW w:w=\"9360\" w:type=\"dxa\"/>"));
    assert!(x.contains("<w:trPr><w:cantSplit/><w:tblHeader/></w:trPr>"));
    assert!(x.contains("<w:tcW w:w=\"6240\" w:type=\"dxa\"/><w:gridSpan w:val=\"2\"/>"));
    assert!(x.contains("<w:jc w:val=\"right\"/></w:pPr><w:r><w:t xml:space=\"preserve\">1.50"));
    assert!(x.contains(">TRUE<") && x.contains("a &amp; b"));
}

// REQ: FOLIO-REQ-028
#[test]
fn every_cell_ends_with_a_paragraph_and_remainder_goes_last() {
    let t = Table {
        columns: 4,
        repeat_header_rows: 0,
        rows: vec![Row {
            cells: (0..4).map(|_| cell(CellValue::Empty, 1, "")).collect(),
            can_split: true,
        }],
    };
    // 468pt content = 9360 twips is divisible; use 601pt page: 457pt = 9140 -> 2285 each; use 600: 456pt=9120.
    let x = xml(&doc(t, 603)); // 459pt = 9180 twips: 2295 each
    assert_eq!(x.matches("<w:tc>").count(), 4);
    assert_eq!(x.matches("</w:p></w:tc>").count(), 4);
    assert!(x.contains("<w:gridCol w:w=\"2295\"/>"));
    let t = Table {
        columns: 7,
        repeat_header_rows: 0,
        rows: vec![Row {
            cells: (0..7).map(|_| cell(CellValue::Empty, 1, "")).collect(),
            can_split: true,
        }],
    };
    let x = xml(&doc(t, 612)); // 9360 / 7 = 1337 r 1 -> last 1338
    assert_eq!(x.matches("<w:gridCol w:w=\"1337\"/>").count(), 6);
    assert!(x.contains("<w:gridCol w:w=\"1338\"/>"));
}

// REQ: FOLIO-REQ-028
#[test]
fn cell_roles_register_styles() {
    let t = Table {
        columns: 1,
        repeat_header_rows: 0,
        rows: vec![Row {
            cells: vec![cell(CellValue::Text("x".into()), 1, "data bold")],
            can_split: true,
        }],
    };
    let b = render_docx(&doc(t, 612), &Limits::default()).unwrap();
    assert!(part(&b, "word/document.xml").contains("w:val=\"TSP_data_20bold\""));
    assert!(part(&b, "word/styles.xml").contains("w:styleId=\"TSP_data_20bold\""));
}
