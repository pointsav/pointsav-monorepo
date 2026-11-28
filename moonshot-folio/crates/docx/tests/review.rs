// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Regression tests for the independent review of the DOCX writer (findings D1 to D11).

#![allow(clippy::unwrap_used, clippy::too_many_lines)]

mod common;
use common::{part, well_formed};
use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_docx::render_docx;
use moonshot_folio_ir::{
    Block, Body, Cell, CellValue, Document, Inline, List, Meta, PageSetup, Pagination, Para, Row,
    Section, Table,
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
fn t(s: &str) -> Inline {
    Inline::Text(s.into())
}
fn para(role: &str, inl: Vec<Inline>) -> Block {
    Block::Para(Para {
        role: role.into(),
        inlines: inl,
        pagination: Pagination::default(),
    })
}
fn render(d: &Document) -> Vec<u8> {
    render_docx(d, &Limits::default()).unwrap()
}
fn list(n: u32, ordered: bool) -> Block {
    Block::List(List {
        numbering: n,
        ordered,
        items: vec![vec![t("x")]],
    })
}

// REQ: FOLIO-REQ-049
#[test]
fn every_list_restarts_at_one() {
    let b = render(&doc(vec![Section::new(
        page(),
        vec![list(7, true), list(8, true)],
    )]));
    let n = part(&b, "word/numbering.xml");
    assert_eq!(n.matches("<w:startOverride w:val=\"1\"/>").count(), 2);
    assert!(well_formed(&n));
}

// REQ: FOLIO-REQ-049
#[test]
fn a_section_without_chrome_does_not_inherit_the_previous_one() {
    let mut a = Section::new(page(), vec![para("", vec![t("a")])]);
    a.header = vec![para("", vec![t("HEAD")])];
    a.footer = vec![para("", vec![t("FOOT")])];
    let c = Section::new(page(), vec![para("", vec![t("c")])]);
    let b = render(&doc(vec![a, c]));
    let d = part(&b, "word/document.xml");
    assert_eq!(d.matches("headerReference").count(), 2);
    assert!(d.contains("rIdH2") && d.contains("rIdF2"));
    let h2 = part(&b, "word/header2.xml");
    assert!(h2.contains("<w:p/>") && !h2.contains("HEAD"));
    assert!(well_formed(&h2));
    // With no chrome anywhere there is still nothing to write.
    let plain = render(&doc(vec![
        Section::new(page(), vec![]),
        Section::new(page(), vec![]),
    ]));
    assert!(!part(&plain, "word/document.xml").contains("Reference"));
}

// REQ: FOLIO-REQ-049
#[test]
fn same_role_as_paragraph_and_run_gets_distinct_style_ids() {
    let b = render(&doc(vec![Section::new(
        page(),
        vec![para(
            "note",
            vec![Inline::Run {
                role: "note".into(),
                text: "r".into(),
            }],
        )],
    )]));
    let s = part(&b, "word/styles.xml");
    assert!(s.contains("w:styleId=\"TSP_note\"") && s.contains("w:styleId=\"TSC_note\""));
    assert!(!s.contains("w:styleId=\"TS_note\""));
}

// REQ: FOLIO-REQ-049
#[test]
fn style_names_are_unique_ignoring_case() {
    let b = render(&doc(vec![Section::new(
        page(),
        vec![para("Aa", vec![t("x")]), para("aA", vec![t("y")])],
    )]));
    let s = part(&b, "word/styles.xml").to_lowercase();
    assert_eq!(s.matches("<w:name w:val=\"ts aa\"/>").count(), 1);
    assert!(s.contains("ts aa (2)"));
}

// REQ: FOLIO-REQ-049
#[test]
fn parts_and_body_never_end_in_a_bare_table() {
    let tbl = Block::Table(Table {
        columns: 1,
        repeat_header_rows: 0,
        rows: vec![Row {
            cells: vec![Cell {
                value: CellValue::Empty,
                span: 1,
                role: String::new(),
            }],
            can_split: true,
        }],
    });
    let mut s = Section::new(page(), vec![tbl.clone()]);
    s.footer = vec![tbl];
    let b = render(&doc(vec![s]));
    assert!(part(&b, "word/footer1.xml").ends_with("</w:tbl><w:p/></w:ftr>"));
    assert!(part(&b, "word/document.xml").contains("</w:tbl><w:p/><w:sectPr>"));
}

// REQ: FOLIO-REQ-049
#[test]
fn widow_control_is_not_disabled_by_default() {
    let b = render(&doc(vec![Section::new(
        page(),
        vec![para("", vec![t("x")])],
    )]));
    assert!(!part(&b, "word/document.xml").contains("widowControl"));
}

// REQ: FOLIO-REQ-049
#[test]
fn header_and_footer_distance_is_derived_from_the_margin() {
    let b = render(&doc(vec![Section::new(page(), vec![])]));
    assert!(part(&b, "word/document.xml").contains("w:header=\"720\" w:footer=\"720\""));
    let mut p = page();
    p.margin = Emu::from_points(10).unwrap();
    let b = render(&doc(vec![Section::new(p, vec![])]));
    assert!(part(&b, "word/document.xml").contains("w:header=\"100\" w:footer=\"100\""));
}

// REQ: FOLIO-REQ-049
#[test]
fn carriage_returns_become_line_breaks() {
    let b = render(&doc(vec![Section::new(
        page(),
        vec![para("", vec![t("a\r\nb\rc")])],
    )]));
    let d = part(&b, "word/document.xml");
    assert!(!d.contains('\r'));
    assert_eq!(d.matches("<w:br/>").count(), 2);
}

// REQ: FOLIO-REQ-049
#[test]
fn anchors_must_start_with_a_letter_be_unique_and_exist() {
    let bad = |inl: Vec<Inline>| {
        matches!(
            render_docx(
                &doc(vec![Section::new(page(), vec![para("", inl)])]),
                &Limits::default()
            ),
            Err(FolioError::Invalid(_))
        )
    };
    assert!(bad(vec![Inline::Anchor("1a".into())]));
    assert!(bad(vec![Inline::Anchor("_hidden".into())]));
    assert!(bad(vec![
        Inline::Anchor("a".into()),
        Inline::Anchor("a".into())
    ]));
    assert!(bad(vec![Inline::Link {
        target: "#nowhere".into(),
        text: "x".into()
    }]));
    assert!(!bad(vec![
        Inline::Link {
            target: "#later".into(),
            text: "x".into()
        },
        Inline::Anchor("later".into())
    ]));
}

// REQ: FOLIO-REQ-049
#[test]
fn empty_run_role_is_plain_text() {
    let b = render(&doc(vec![Section::new(
        page(),
        vec![para(
            "",
            vec![Inline::Run {
                role: String::new(),
                text: "x".into(),
            }],
        )],
    )]));
    assert!(!part(&b, "word/document.xml").contains("rStyle"));
    assert!(!part(&b, "word/styles.xml").contains("TSC_\""));
}

// REQ: FOLIO-REQ-049
#[test]
fn images_are_reported_unsupported_by_the_capability_matrix() {
    use moonshot_folio_ir::{check_target, Image, ImageFormat, Target};
    let img = Block::Image(Image {
        alt_text: "a".into(),
        format: ImageFormat::Png,
        data: vec![1],
        width: Emu::from_points(1).unwrap(),
        height: Emu::from_points(1).unwrap(),
    });
    let d = doc(vec![Section::new(page(), vec![img])]);
    assert!(matches!(
        check_target(&d, Target::Docx),
        Err(FolioError::Unsupported(_))
    ));
    assert!(check_target(&d, Target::Html).is_ok());
}
