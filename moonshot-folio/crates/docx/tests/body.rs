// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! DOCX body tests (task F3.2).

#![allow(clippy::unwrap_used)]

mod common;
use common::{part, well_formed};
use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_docx::render_docx;
use moonshot_folio_ir::{
    Block, Body, BreakKind, Document, FieldKind, Heading, Inline, Meta, PageSetup, Pagination,
    Para, Section,
};

fn meta() -> Meta {
    Meta {
        title: "T".into(),
        author: "A".into(),
        date: "2026-11-22".into(),
        lang: "en".into(),
    }
}
fn page() -> PageSetup {
    PageSetup {
        width: Emu::from_points(612).unwrap(),
        height: Emu::from_points(792).unwrap(),
        margin: Emu::from_points(72).unwrap(),
    }
}
fn doc(sections: Vec<Vec<Block>>) -> Document {
    Document {
        meta: meta(),
        body: Body::Flow(
            sections
                .into_iter()
                .map(|blocks| Section::new(page(), blocks))
                .collect(),
        ),
    }
}
fn para(role: &str, inlines: Vec<Inline>) -> Block {
    Block::Para(Para {
        role: role.into(),
        inlines,
        pagination: Pagination::default(),
    })
}
fn t(s: &str) -> Inline {
    Inline::Text(s.into())
}
fn render(d: &Document) -> Vec<u8> {
    render_docx(d, &Limits::default()).unwrap()
}

// REQ: FOLIO-REQ-027
#[test]
fn paragraph_text_runs_and_escaping() {
    let b = render(&doc(vec![vec![para(
        "",
        vec![
            t("a < b & \"c\""),
            Inline::Run {
                role: "bold".into(),
                text: "B".into(),
            },
            Inline::Highlight("H".into()),
        ],
    )]]));
    let d = part(&b, "word/document.xml");
    assert!(well_formed(&d));
    assert!(d.contains("<w:t xml:space=\"preserve\">a &lt; b &amp; &quot;c&quot;</w:t>"));
    assert!(d.contains("<w:rPr><w:b/></w:rPr>"));
    assert!(d.contains("<w:highlight w:val=\"yellow\"/>"));
}

// REQ: FOLIO-REQ-027
#[test]
fn tabs_and_newlines_become_elements() {
    let d = part(
        &render(&doc(vec![vec![para("", vec![t("a\tb\nc")])]])),
        "word/document.xml",
    );
    assert!(d.contains("a</w:t><w:tab/>"), "{d}");
    assert!(d.contains("b</w:t><w:br/>"));
    assert!(d.contains(">c</w:t>"));
    assert!(well_formed(&d));
}

// REQ: FOLIO-REQ-027
#[test]
fn roles_become_injective_styles() {
    let b = render(&doc(vec![vec![
        para("a b", vec![t("x")]),
        para("a-b", vec![t("y")]),
        para(
            "a b",
            vec![Inline::Run {
                role: "emph".into(),
                text: "z".into(),
            }],
        ),
    ]]));
    let d = part(&b, "word/document.xml");
    let s = part(&b, "word/styles.xml");
    assert!(d.contains("w:val=\"TSP_a_20b\"") && d.contains("w:val=\"TSP_a_2Db\""));
    assert!(s.contains("w:styleId=\"TSP_a_20b\"") && s.contains("w:styleId=\"TSP_a_2Db\""));
    assert!(s.contains("w:type=\"character\" w:customStyle=\"1\" w:styleId=\"TSC_emph\""));
    assert_eq!(s.matches("w:styleId=\"TSP_a_20b\"").count(), 1);
    assert!(well_formed(&s));
}

// REQ: FOLIO-REQ-027
#[test]
fn headings_and_pagination() {
    let b = render(&doc(vec![vec![
        Block::Heading(Heading {
            level: 3,
            inlines: vec![t("H")],
            pagination: Pagination {
                keep_with_next: true,
                widow_orphan: true,
                page_break_before: true,
            },
        }),
        para("", vec![t("")]),
    ]]));
    let d = part(&b, "word/document.xml");
    assert!(d.contains(
        "<w:pStyle w:val=\"Heading3\"/><w:keepNext/><w:pageBreakBefore/><w:widowControl/>"
    ));
    assert!(!d.contains("widowControl w:val=\"0\""));
    assert!(d.contains("<w:r><w:t/></w:r>"));
}

// REQ: FOLIO-REQ-027
#[test]
fn breaks_and_fields() {
    let b = render(&doc(vec![vec![
        Block::Break(BreakKind::Page),
        Block::Break(BreakKind::Line),
        Block::Field(FieldKind::Page),
        para(
            "",
            vec![
                Inline::FieldRef(FieldKind::NumPages),
                Inline::FieldRef(FieldKind::Toc),
            ],
        ),
    ]]));
    let d = part(&b, "word/document.xml");
    assert!(d.contains("<w:br w:type=\"page\"/>"));
    assert!(d.contains("w:instr=\" PAGE \"") && d.contains("w:instr=\" NUMPAGES \""));
    assert!(d.contains("TOC \\o &quot;1-3&quot;"));
    assert!(well_formed(&d));
}

// REQ: FOLIO-REQ-027
#[test]
fn links_anchors_and_relationships() {
    let b = render(&doc(vec![vec![para(
        "",
        vec![
            Inline::Anchor("top".into()),
            Inline::Link {
                target: "https://example.com/a?b=1&c=2".into(),
                text: "ext".into(),
            },
            Inline::Link {
                target: "https://example.com/a?b=1&c=2".into(),
                text: "again".into(),
            },
            Inline::Link {
                target: "#top".into(),
                text: "int".into(),
            },
        ],
    )]]));
    let d = part(&b, "word/document.xml");
    let r = part(&b, "word/_rels/document.xml.rels");
    assert!(d.contains("<w:bookmarkStart w:id=\"0\" w:name=\"top\"/>"));
    assert!(d.contains("<w:hyperlink w:anchor=\"top\">"));
    assert_eq!(d.matches("r:id=\"rId2\"").count(), 2);
    assert!(
        r.contains("Id=\"rId2\"")
            && r.contains("a?b=1&amp;c=2")
            && r.contains("TargetMode=\"External\"")
    );
    assert!(well_formed(&d) && well_formed(&r));
}

// REQ: FOLIO-REQ-027
#[test]
fn unsafe_links_and_bad_anchors_are_rejected() {
    for target in ["javascript:alert(1)", "file:///etc/passwd", "ftp://x", ""] {
        let d = doc(vec![vec![para(
            "",
            vec![Inline::Link {
                target: target.into(),
                text: "x".into(),
            }],
        )]]);
        assert!(
            matches!(
                render_docx(&d, &Limits::default()),
                Err(FolioError::Invalid(_))
            ),
            "{target}"
        );
    }
    for name in ["", "has space", "a\"b", &"x".repeat(41)] {
        let d = doc(vec![vec![para("", vec![Inline::Anchor(name.into())])]]);
        assert!(
            matches!(
                render_docx(&d, &Limits::default()),
                Err(FolioError::Invalid(_))
            ),
            "{name}"
        );
    }
}

// REQ: FOLIO-REQ-027
#[test]
fn multiple_sections_carry_their_own_page_setup() {
    let d = doc(vec![
        vec![para("", vec![t("one")])],
        vec![para("", vec![t("two")])],
    ]);
    let x = part(&render(&d), "word/document.xml");
    assert_eq!(x.matches("<w:sectPr>").count(), 2);
    let first = x.find("<w:sectPr>").unwrap();
    assert!(x[..first].contains(">one<") && !x[..first].contains(">two<"));
}
