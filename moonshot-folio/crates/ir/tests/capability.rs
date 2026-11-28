// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Capability matrix tests (task F2.2).

#![allow(clippy::unwrap_used)]

use moonshot_folio_core::{Emu, FolioError};
use moonshot_folio_ir::{
    check_target, features, support, Block, Body, Cell, CellValue, Document, Feature, FieldKind,
    Inline, Meta, PageSetup, Pagination, Para, Row, Section, Sheet, Support, Table, Target,
};

const TARGETS: [Target; 5] = [
    Target::Pdf,
    Target::Docx,
    Target::Html,
    Target::Xlsx,
    Target::Csv,
];
const FEATURES: [Feature; 14] = [
    Feature::Text,
    Feature::List,
    Feature::Table,
    Feature::ColumnSpan,
    Feature::RepeatHeader,
    Feature::ImagePng,
    Feature::ImageJpeg,
    Feature::PageFields,
    Feature::Toc,
    Feature::Links,
    Feature::Breaks,
    Feature::Workbook,
    Feature::FlowBody,
    Feature::NumberCell,
];

fn meta() -> Meta {
    Meta {
        title: "T".into(),
        author: "A".into(),
        date: "2026-11-21".into(),
        lang: "en".into(),
    }
}
fn flow(blocks: Vec<Block>) -> Document {
    let page = PageSetup {
        width: Emu::from_inches(8).unwrap(),
        height: Emu::from_inches(11).unwrap(),
        margin: Emu::ZERO,
    };
    Document {
        meta: meta(),
        body: Body::Flow(vec![Section::new(page, blocks)]),
    }
}
fn book() -> Document {
    let c = Cell {
        value: CellValue::Number("1".into()),
        span: 1,
        role: String::new(),
    };
    Document {
        meta: meta(),
        body: Body::Workbook(vec![Sheet {
            name: "S".into(),
            columns: 1,
            freeze_rows: 0,
            rows: vec![Row {
                cells: vec![c],
                can_split: true,
            }],
            ..Sheet::default()
        }]),
    }
}

// REQ: FOLIO-REQ-023
#[test]
fn every_cell_has_an_answer_and_reasons_are_nonempty() {
    for f in FEATURES {
        for t in TARGETS {
            match support(f, t) {
                Support::Native => {}
                Support::Degraded(r) | Support::Rejected(r) => {
                    assert!(!r.is_empty(), "{f:?} {t:?}");
                }
            }
        }
    }
}

// REQ: FOLIO-REQ-023
#[test]
fn body_kind_matches_targets() {
    for t in [Target::Pdf, Target::Docx, Target::Html] {
        assert_eq!(support(Feature::FlowBody, t), Support::Native);
    }
    for t in [Target::Xlsx, Target::Csv] {
        assert!(matches!(
            support(Feature::FlowBody, t),
            Support::Rejected(_)
        ));
        assert_eq!(support(Feature::Workbook, t), Support::Native);
    }
    assert!(matches!(
        support(Feature::Workbook, Target::Docx),
        Support::Rejected(_)
    ));
}

// REQ: FOLIO-REQ-023
#[test]
fn flow_document_checks() {
    let d = flow(vec![
        Block::Para(Para {
            role: "body".into(),
            inlines: vec![Inline::Text("x".into()), Inline::FieldRef(FieldKind::Page)],
            pagination: Pagination::default(),
        }),
        Block::Field(FieldKind::Toc),
        Block::Table(Table {
            columns: 1,
            repeat_header_rows: 1,
            rows: vec![],
        }),
    ]);
    let f = features(&d);
    assert!(
        f.contains(&Feature::PageFields)
            && f.contains(&Feature::Toc)
            && f.contains(&Feature::RepeatHeader)
    );
    assert_eq!(f.iter().filter(|x| **x == Feature::Text).count(), 1);
    assert!(check_target(&d, Target::Pdf).unwrap().is_empty());
    assert_eq!(check_target(&d, Target::Docx).unwrap().len(), 1);
    assert_eq!(check_target(&d, Target::Html).unwrap().len(), 3);
    assert!(matches!(
        check_target(&d, Target::Xlsx),
        Err(FolioError::Unsupported(_))
    ));
}

// REQ: FOLIO-REQ-023
#[test]
fn workbook_document_checks() {
    let d = book();
    for t in [Target::Xlsx, Target::Csv, Target::Html] {
        assert!(check_target(&d, t).unwrap().is_empty());
    }
    for t in [Target::Pdf, Target::Docx] {
        assert!(matches!(
            check_target(&d, t),
            Err(FolioError::Unsupported(_))
        ));
    }
}

// REQ: FOLIO-REQ-023
#[test]
fn csv_rejects_merged_cells() {
    let mut d = book();
    if let Body::Workbook(s) = &mut d.body {
        s[0].columns = 2;
        s[0].rows[0].cells[0].span = 2;
    }
    assert!(matches!(
        check_target(&d, Target::Csv),
        Err(FolioError::Unsupported(_))
    ));
    assert!(check_target(&d, Target::Xlsx).unwrap().is_empty());
}
