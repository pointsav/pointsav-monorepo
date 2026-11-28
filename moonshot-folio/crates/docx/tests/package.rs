// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! DOCX package tests (task F3.1).

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

mod common;
use common::{part, well_formed};
use moonshot_folio_codec::ZipArchive;
use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_docx::render_docx;
use moonshot_folio_ir::{Body, Cell, CellValue, Document, Meta, PageSetup, Row, Section, Sheet};

fn meta() -> Meta {
    Meta {
        title: "A & B <c>".into(),
        author: "Au".into(),
        date: "2026-11-22".into(),
        lang: "en-CA".into(),
    }
}
fn flow() -> Document {
    let page = PageSetup {
        width: Emu::from_points(612).unwrap(),
        height: Emu::from_points(792).unwrap(),
        margin: Emu::from_points(72).unwrap(),
    };
    Document {
        meta: meta(),
        body: Body::Flow(vec![Section::new(page, vec![])]),
    }
}

// REQ: FOLIO-REQ-026
#[test]
fn package_has_required_parts_in_fixed_order() {
    let bytes = render_docx(&flow(), &Limits::default()).unwrap();
    let z = ZipArchive::parse(&bytes, &Limits::default()).unwrap();
    let names: Vec<&str> = z
        .entries()
        .iter()
        .map(moonshot_folio_codec::ZipEntry::name)
        .collect();
    assert_eq!(
        names,
        [
            "[Content_Types].xml",
            "_rels/.rels",
            "word/document.xml",
            "word/_rels/document.xml.rels",
            "word/styles.xml",
            "docProps/core.xml"
        ]
    );
}

// REQ: FOLIO-REQ-026
#[test]
fn every_part_is_well_formed_and_properties_are_escaped() {
    let bytes = render_docx(&flow(), &Limits::default()).unwrap();
    for n in [
        "[Content_Types].xml",
        "_rels/.rels",
        "word/document.xml",
        "word/_rels/document.xml.rels",
        "word/styles.xml",
        "docProps/core.xml",
    ] {
        assert!(well_formed(&part(&bytes, n)), "{n}");
    }
    let core = part(&bytes, "docProps/core.xml");
    assert!(core.contains("<dc:title>A &amp; B &lt;c&gt;</dc:title>"));
    assert!(core.contains("2026-11-22T00:00:00Z"));
}

// REQ: FOLIO-REQ-026
#[test]
fn page_geometry_is_written_in_twips() {
    let bytes = render_docx(&flow(), &Limits::default()).unwrap();
    let d = part(&bytes, "word/document.xml");
    assert!(d.contains("w:w=\"12240\" w:h=\"15840\""));
    assert!(d.contains("w:top=\"1440\""));
}

// REQ: FOLIO-REQ-026
#[test]
fn output_is_deterministic() {
    let a = render_docx(&flow(), &Limits::default()).unwrap();
    let b = render_docx(&flow(), &Limits::default()).unwrap();
    assert_eq!(a, b);
    // Manual oracle hook: FOLIO_DOCX_OUT=/path/x.docx writes the file for an external opener.
    if let Some(p) = std::env::var_os("FOLIO_DOCX_OUT") {
        std::fs::write(p, &a).unwrap();
    }
}

// REQ: FOLIO-REQ-026
#[test]
fn workbook_and_bad_text_are_rejected() {
    let c = Cell {
        value: CellValue::Empty,
        span: 1,
        role: String::new(),
    };
    let wb = Document {
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
    };
    assert!(matches!(
        render_docx(&wb, &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
    let mut d = flow();
    d.meta.title = "bad\u{1}".into();
    assert!(matches!(
        render_docx(&d, &Limits::default()),
        Err(FolioError::Invalid(_))
    ));
    d = flow();
    d.meta.lang = String::new();
    assert!(render_docx(&d, &Limits::default()).is_err());
}
