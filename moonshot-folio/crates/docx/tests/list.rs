// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! DOCX list tests (task F3.5).

#![allow(clippy::unwrap_used)]

mod common;
use common::{part, well_formed};
use moonshot_folio_codec::ZipArchive;
use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_docx::render_docx;
use moonshot_folio_ir::{Block, Body, Document, Inline, List, Meta, PageSetup, Section};

fn doc(blocks: Vec<Block>) -> Document {
    let page = PageSetup {
        width: Emu::from_points(612).unwrap(),
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
        body: Body::Flow(vec![Section::new(page, blocks)]),
    }
}
fn list(numbering: u32, ordered: bool, n: usize) -> Block {
    Block::List(List {
        numbering,
        ordered,
        items: (0..n)
            .map(|i| vec![Inline::Text(format!("item {i}"))])
            .collect(),
    })
}

// REQ: FOLIO-REQ-030
#[test]
fn bulleted_and_numbered_lists_get_numbering_part() {
    let b = render_docx(
        &doc(vec![list(7, false, 2), list(9, true, 1), list(7, false, 1)]),
        &Limits::default(),
    )
    .unwrap();
    let d = part(&b, "word/document.xml");
    assert_eq!(d.matches("<w:numId w:val=\"1\"/>").count(), 3);
    assert_eq!(d.matches("<w:numId w:val=\"2\"/>").count(), 1);
    assert!(d.contains("<w:pStyle w:val=\"ListParagraph\"/>"));
    let n = part(&b, "word/numbering.xml");
    assert!(well_formed(&n));
    assert!(n.contains("<w:num w:numId=\"1\"><w:abstractNumId w:val=\"0\"/>"));
    assert!(n.contains("<w:num w:numId=\"2\"><w:abstractNumId w:val=\"1\"/>"));
    assert!(n.find("</w:abstractNum><w:num").is_some());
    assert!(part(&b, "[Content_Types].xml").contains("/word/numbering.xml"));
    assert!(part(&b, "word/_rels/document.xml.rels").contains("Target=\"numbering.xml\""));
    assert!(part(&b, "word/styles.xml").contains("w:styleId=\"ListParagraph\""));
    assert!(well_formed(&d));
}

// REQ: FOLIO-REQ-030
#[test]
fn no_numbering_part_without_lists() {
    let b = render_docx(&doc(vec![]), &Limits::default()).unwrap();
    let z = ZipArchive::parse(&b, &Limits::default()).unwrap();
    assert!(z.find("word/numbering.xml").is_none());
    assert!(!part(&b, "[Content_Types].xml").contains("numbering"));
}

// REQ: FOLIO-REQ-030
#[test]
fn conflicting_kind_for_one_numbering_id_is_rejected() {
    let r = render_docx(
        &doc(vec![list(1, false, 1), list(1, true, 1)]),
        &Limits::default(),
    );
    assert!(matches!(r, Err(FolioError::Invalid(_))));
}

// REQ: FOLIO-REQ-030
#[test]
fn empty_list_is_rejected_by_validation() {
    assert!(render_docx(&doc(vec![list(1, false, 0)]), &Limits::default()).is_err());
}
