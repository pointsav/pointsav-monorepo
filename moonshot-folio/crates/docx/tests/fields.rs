// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! DOCX field refresh and page-number restart tests (tasks F3.6, F3.7).

#![allow(clippy::unwrap_used)]

mod common;
use common::{part, well_formed};
use moonshot_folio_codec::ZipArchive;
use moonshot_folio_core::{Emu, Limits};
use moonshot_folio_docx::render_docx;
use moonshot_folio_ir::{
    Block, Body, Document, FieldKind, Inline, Meta, PageSetup, Pagination, Para, Section,
};

fn page() -> PageSetup {
    PageSetup {
        width: Emu::from_points(612).unwrap(),
        height: Emu::from_points(792).unwrap(),
        margin: Emu::from_points(72).unwrap(),
    }
}
fn doc(sections: Vec<Section>) -> Document {
    Document {
        meta: Meta {
            title: "T".into(),
            author: "A".into(),
            date: "2026-11-22".into(),
            lang: "en".into(),
        },
        body: Body::Flow(sections),
    }
}
fn text_para(s: &str) -> Block {
    Block::Para(Para {
        role: String::new(),
        inlines: vec![Inline::Text(s.into())],
        pagination: Pagination::default(),
    })
}

// REQ: FOLIO-REQ-031
#[test]
fn update_fields_is_set_only_when_fields_need_it() {
    for f in [FieldKind::Toc, FieldKind::Page, FieldKind::NumPages] {
        let b = render_docx(
            &doc(vec![Section::new(page(), vec![Block::Field(f)])]),
            &Limits::default(),
        )
        .unwrap();
        let s = part(&b, "word/settings.xml");
        assert!(s.contains("<w:updateFields w:val=\"true\"/>") && well_formed(&s));
        assert!(part(&b, "[Content_Types].xml").contains("/word/settings.xml"));
        assert!(part(&b, "word/_rels/document.xml.rels").contains("Target=\"settings.xml\""));
    }
    let b = render_docx(
        &doc(vec![Section::new(page(), vec![text_para("x")])]),
        &Limits::default(),
    )
    .unwrap();
    assert!(ZipArchive::parse(&b, &Limits::default())
        .unwrap()
        .find("word/settings.xml")
        .is_none());
}

// REQ: FOLIO-REQ-031
#[test]
fn fields_in_footers_also_trigger_update() {
    let mut s = Section::new(page(), vec![]);
    s.footer = vec![Block::Field(FieldKind::Page)];
    let b = render_docx(&doc(vec![s]), &Limits::default()).unwrap();
    assert!(part(&b, "word/settings.xml").contains("updateFields"));
}

// REQ: FOLIO-REQ-031
#[test]
fn page_numbers_restart_per_section() {
    let a = Section::new(page(), vec![text_para("a")]);
    let mut c = Section::new(page(), vec![text_para("c")]);
    c.restart_page_numbers = true;
    c.different_first_page = true;
    let x = part(
        &render_docx(&doc(vec![a, c]), &Limits::default()).unwrap(),
        "word/document.xml",
    );
    assert_eq!(x.matches("<w:pgNumType w:start=\"1\"/>").count(), 1);
    assert!(x.contains("<w:pgNumType w:start=\"1\"/><w:titlePg/></w:sectPr>"));
    assert!(well_formed(&x));
}
