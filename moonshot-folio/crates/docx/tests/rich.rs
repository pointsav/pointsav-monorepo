// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! A document using every writer feature, rendered twice for determinism. Set
//! `FOLIO_DOCX_OUT=/path/rich.docx` to write it for a manual check in an external viewer.

#![allow(clippy::unwrap_used, clippy::too_many_lines)]

mod common;
use common::{part, well_formed};
use moonshot_folio_core::{Emu, Limits};
use moonshot_folio_docx::render_docx;
use moonshot_folio_ir::{
    Block, Body, BreakKind, Cell, CellValue, Document, FieldKind, Heading, Inline, List, Meta,
    PageSetup, Pagination, Para, Row, Section, Table,
};

fn t(s: &str) -> Inline {
    Inline::Text(s.into())
}
fn cell(v: CellValue, span: u32, role: &str) -> Cell {
    Cell {
        value: v,
        span,
        role: role.into(),
    }
}
fn para(role: &str, inlines: Vec<Inline>, pagination: Pagination) -> Block {
    Block::Para(Para {
        role: role.into(),
        inlines,
        pagination,
    })
}

fn rich() -> Document {
    let page = PageSetup {
        width: Emu::from_points(612).unwrap(),
        height: Emu::from_points(792).unwrap(),
        margin: Emu::from_points(72).unwrap(),
    };
    let head = |s: &str| cell(CellValue::Text(s.into()), 1, "header");
    let table = Table {
        columns: 3,
        repeat_header_rows: 1,
        rows: vec![
            Row {
                cells: vec![head("Item"), head("2026"), head("2025")],
                can_split: false,
            },
            Row {
                cells: vec![
                    cell(CellValue::Text("Cash".into()), 1, ""),
                    cell(CellValue::Number("100.00".into()), 1, ""),
                    cell(CellValue::Number("-90.50".into()), 1, ""),
                ],
                can_split: true,
            },
            Row {
                cells: vec![cell(
                    CellValue::Text("Banner spanning all columns".into()),
                    3,
                    "banner",
                )],
                can_split: true,
            },
        ],
    };
    let keep = Pagination {
        keep_with_next: true,
        ..Pagination::default()
    };
    let widow = Pagination {
        widow_orphan: true,
        ..Pagination::default()
    };
    let none = Pagination::default();
    let body_inlines = vec![
        t("Plain, "),
        Inline::Run {
            role: "bold".into(),
            text: "bold".into(),
        },
        t(", "),
        Inline::Highlight("highlighted".into()),
        t(" and a "),
        Inline::Link {
            target: "https://example.com/?a=1&b=2".into(),
            text: "link".into(),
        },
        Inline::Anchor("here".into()),
    ];
    let mut first = Section::new(
        page,
        vec![
            Block::Heading(Heading {
                level: 1,
                inlines: vec![t("Statement")],
                pagination: keep,
            }),
            para("body", body_inlines, widow),
            Block::Table(table),
            Block::List(List {
                numbering: 1,
                ordered: false,
                items: vec![vec![t("one")], vec![t("two")]],
            }),
            Block::List(List {
                numbering: 2,
                ordered: true,
                items: vec![vec![t("first")], vec![t("second")]],
            }),
            Block::Field(FieldKind::Toc),
            Block::Break(BreakKind::Page),
        ],
    );
    first.header = vec![para("running-header", vec![t("Example header")], none)];
    first.footer = vec![para(
        "page-number",
        vec![
            t("Page "),
            Inline::FieldRef(FieldKind::Page),
            t(" of "),
            Inline::FieldRef(FieldKind::NumPages),
        ],
        none,
    )];
    first.different_first_page = true;
    let mut second = Section::new(
        page,
        vec![para(
            "",
            vec![t("Second section, numbering restarts.")],
            none,
        )],
    );
    second.restart_page_numbers = true;
    Document {
        meta: Meta {
            title: "Rich example".into(),
            author: "Example".into(),
            date: "2026-11-22".into(),
            lang: "en-CA".into(),
        },
        body: Body::Flow(vec![first, second]),
    }
}

// REQ: FOLIO-REQ-032
#[test]
fn rich_document_is_well_formed_and_deterministic() {
    let a = render_docx(&rich(), &Limits::default()).unwrap();
    assert_eq!(a, render_docx(&rich(), &Limits::default()).unwrap());
    for p in [
        "[Content_Types].xml",
        "_rels/.rels",
        "word/document.xml",
        "word/_rels/document.xml.rels",
        "word/styles.xml",
        "word/settings.xml",
        "word/numbering.xml",
        "word/header1.xml",
        "word/footer1.xml",
        "docProps/core.xml",
    ] {
        assert!(well_formed(&part(&a, p)), "{p}");
    }
    if let Some(p) = std::env::var_os("FOLIO_DOCX_OUT") {
        std::fs::write(p, &a).unwrap();
    }
}
