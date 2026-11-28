// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Adapter tests (task F2.3).

#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::panic)]

use moonshot_folio_compat_typeset::convert;
use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_ir::{self as ir, validate, Block as IB, Body, Meta};
use tool_typeset::{
    Align, Block, Cell, ColWidth, Column, Doc, PageSetup, PageSize, ParaStyle, Row, Span, Table,
};

fn meta() -> Meta {
    Meta {
        title: "T".into(),
        author: "A".into(),
        date: "2026-11-22".into(),
        lang: "en".into(),
    }
}
fn col() -> Column {
    Column {
        width: ColWidth::Pct(50.0),
        align: Align::Left,
    }
}
fn doc(blocks: Vec<Block>) -> Doc {
    Doc {
        page: PageSetup::default(),
        blocks,
    }
}
fn blocks_of(c: &ir::Document) -> &Vec<IB> {
    match &c.body {
        Body::Flow(s) => &s[0].blocks,
        Body::Workbook(_) => panic!("flow expected"),
    }
}

// REQ: FOLIO-REQ-024
#[test]
fn page_geometry_is_exact_emu() {
    let c = convert(&doc(vec![]), meta()).unwrap();
    let Body::Flow(s) = &c.document.body else {
        panic!()
    };
    let p = &s[0].page;
    // Letter portrait, 612 x 792 pt.
    assert_eq!(p.width, Emu::from_points(612).unwrap());
    assert_eq!(p.height, Emu::from_points(792).unwrap());
    let mut d = doc(vec![]);
    d.page.landscape = true;
    let c = convert(&d, meta()).unwrap();
    let Body::Flow(s) = &c.document.body else {
        panic!()
    };
    assert_eq!(s[0].page.width, Emu::from_points(792).unwrap());
}

// REQ: FOLIO-REQ-024
#[test]
fn quantisation_rule_and_bad_lengths() {
    let mut d = doc(vec![]);
    d.page.margin_pt = 0.006; // rounds to one hundredth: 127 EMU
    let c = convert(&d, meta()).unwrap();
    let Body::Flow(s) = &c.document.body else {
        panic!()
    };
    assert_eq!(s[0].page.margin.raw(), 127);
    for bad in [f32::NAN, f32::INFINITY, 1.0e9] {
        let mut d = doc(vec![]);
        d.page.margin_pt = bad;
        assert!(matches!(convert(&d, meta()), Err(FolioError::Invalid(_))));
    }
}

// REQ: FOLIO-REQ-024
#[test]
fn text_blocks_convert_and_validate() {
    let d = doc(vec![
        Block::Masthead {
            entity_name: "E".into(),
            statement_title: "S".into(),
            period_label: "P".into(),
        },
        Block::Heading {
            level: 9,
            text: "H".into(),
        },
        Block::Para {
            spans: vec![Span::Text("a".into()), Span::Bold("b".into())],
            style: ParaStyle::default(),
        },
        Block::PageBreak,
        Block::Rule,
        Block::Placeholder {
            kind: "chart".into(),
            caption: "c".into(),
        },
    ]);
    let c = convert(&d, meta()).unwrap();
    validate(&c.document, &Limits::default()).unwrap();
    let b = blocks_of(&c.document);
    assert_eq!(b.len(), 8);
    assert!(matches!(&b[3], IB::Heading(h) if h.level == 6));
    assert!(matches!(&b[4], IB::Para(p) if p.inlines.len() == 2));
    assert!(c.notes.iter().any(|n| n.contains("indent")));
}

// REQ: FOLIO-REQ-024
#[test]
fn table_converts_pads_and_validates() {
    let mut t = Table::new(vec![col(), col(), col()]);
    t.header = Some(vec![Cell::text("a"), Cell::text("b"), Cell::text("c")]);
    t.repeat_header = true;
    t.rows.push(Row::data(vec![
        Cell::text("x").indent(2),
        Cell::figure("(1.00)"),
    ]));
    t.rows.push(Row::section_banner("S", 3));
    let c = convert(&doc(vec![Block::Table(t)]), meta()).unwrap();
    validate(&c.document, &Limits::default()).unwrap();
    let IB::Table(t) = &blocks_of(&c.document)[0] else {
        panic!()
    };
    assert_eq!(t.repeat_header_rows, 1);
    assert_eq!(t.rows.len(), 3);
    assert_eq!(t.rows[1].cells.len(), 3); // padded with one Empty
    assert_eq!(t.rows[1].cells[1].role, "figure");
    assert_eq!(t.rows[1].cells[0].role, "data indent2");
    assert_eq!(t.rows[2].cells[0].span, 3);
}

// REQ: FOLIO-REQ-024
#[test]
fn over_wide_row_is_malformed() {
    let mut t = Table::new(vec![col()]);
    t.rows
        .push(Row::data(vec![Cell::text("a"), Cell::text("b")]));
    assert!(matches!(
        convert(&doc(vec![Block::Table(t)]), meta()),
        Err(FolioError::Malformed(_))
    ));
}

// REQ: FOLIO-REQ-024
#[test]
fn page_sizes_all_convert() {
    for size in [PageSize::Letter, PageSize::Tabloid, PageSize::Legal] {
        let mut d = doc(vec![]);
        d.page.size = size;
        validate(&convert(&d, meta()).unwrap().document, &Limits::default()).unwrap();
    }
}
