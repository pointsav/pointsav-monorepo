// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! DOCX header and footer tests (task F3.4).

#![allow(clippy::unwrap_used, clippy::many_single_char_names)]

mod common;
use common::{part, well_formed};
use moonshot_folio_codec::ZipArchive;
use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_docx::render_docx;
use moonshot_folio_ir::{
    Block, Body, Document, FieldKind, Inline, Meta, PageSetup, Pagination, Para, Section,
};

fn para(inl: Vec<Inline>) -> Block {
    Block::Para(Para {
        role: "chrome".into(),
        inlines: inl,
        pagination: Pagination::default(),
    })
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
fn page() -> PageSetup {
    PageSetup {
        width: Emu::from_points(612).unwrap(),
        height: Emu::from_points(792).unwrap(),
        margin: Emu::from_points(72).unwrap(),
    }
}
fn names(b: &[u8]) -> Vec<String> {
    let z = ZipArchive::parse(b, &Limits::default()).unwrap();
    z.entries().iter().map(|e| e.name().to_string()).collect()
}

// REQ: FOLIO-REQ-029
#[test]
fn header_and_footer_parts_references_and_content_types() {
    let mut s = Section::new(page(), vec![para(vec![Inline::Text("body".into())])]);
    s.header = vec![para(vec![Inline::Text("Running".into())])];
    s.footer = vec![para(vec![Inline::FieldRef(FieldKind::Page)])];
    s.different_first_page = true;
    let b = render_docx(&doc(vec![s]), &Limits::default()).unwrap();
    let n = names(&b);
    assert!(
        n.contains(&"word/header1.xml".to_string()) && n.contains(&"word/footer1.xml".to_string())
    );
    let d = part(&b, "word/document.xml");
    assert!(d.contains("<w:headerReference w:type=\"default\" r:id=\"rIdH1\"/>"));
    assert!(d.contains("<w:footerReference w:type=\"default\" r:id=\"rIdF1\"/>"));
    assert!(d.contains("<w:footerReference w:type=\"first\" r:id=\"rIdF1\"/>"));
    assert!(!d.contains("<w:headerReference w:type=\"first\""));
    assert!(d.contains("<w:titlePg/>"));
    // References precede page geometry in sectPr.
    assert!(d.find("headerReference").unwrap() < d.find("<w:pgSz").unwrap());
    let r = part(&b, "word/_rels/document.xml.rels");
    assert!(r.contains("Id=\"rIdH1\"") && r.contains("Target=\"header1.xml\""));
    assert!(r.contains("Id=\"rIdF1\"") && r.contains("/footer\" Target=\"footer1.xml\""));
    let ct = part(&b, "[Content_Types].xml");
    assert!(ct.contains("/word/header1.xml") && ct.contains("wordprocessingml.footer+xml"));
    for p in [
        "word/header1.xml",
        "word/footer1.xml",
        "word/document.xml",
        "[Content_Types].xml",
        "word/_rels/document.xml.rels",
    ] {
        assert!(well_formed(&part(&b, p)), "{p}");
    }
    assert!(part(&b, "word/footer1.xml").contains("w:instr=\" PAGE \""));
}

// REQ: FOLIO-REQ-029
#[test]
fn each_section_gets_its_own_parts() {
    let mut a = Section::new(page(), vec![]);
    a.footer = vec![para(vec![Inline::Text("A".into())])];
    let mut c = Section::new(page(), vec![]);
    c.footer = vec![para(vec![Inline::Text("C".into())])];
    let b = render_docx(&doc(vec![a, c]), &Limits::default()).unwrap();
    assert!(part(&b, "word/footer1.xml").contains(">A<"));
    assert!(part(&b, "word/footer2.xml").contains(">C<"));
    assert!(part(&b, "word/document.xml").contains("rIdF2"));
}

// REQ: FOLIO-REQ-029
#[test]
fn external_links_in_chrome_are_rejected() {
    let mut s = Section::new(page(), vec![]);
    s.header = vec![para(vec![Inline::Link {
        target: "https://example.com".into(),
        text: "x".into(),
    }])];
    assert!(matches!(
        render_docx(&doc(vec![s]), &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
}

// REQ: FOLIO-REQ-029
#[test]
fn chrome_is_validated_too() {
    use moonshot_folio_ir::Heading;
    let mut s = Section::new(page(), vec![]);
    s.footer = vec![Block::Heading(Heading {
        level: 9,
        inlines: vec![],
        pagination: Pagination::default(),
    })];
    assert!(matches!(
        render_docx(&doc(vec![s]), &Limits::default()),
        Err(FolioError::Invalid(_))
    ));
}
