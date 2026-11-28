// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Table of contents tests (task F5.5): entries, page numbers, links, fixed point.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines,
    clippy::cast_possible_truncation
)]

mod common;

use common::{
    check_invariants, doc, heading, lay, para, paras, pts, section, setup, small, try_lay, words_of,
};
use moonshot_folio_core::FolioError;
use moonshot_folio_ir::{Block, FieldKind};
use moonshot_folio_layout::{dump, headings, ItemKind, LinkTarget, PageTree, StructRole, Theme};

fn toc() -> Block {
    Block::Field(FieldKind::Toc)
}

/// A document: a TOC, then `n` chapters of a level 1 heading, a level 2 heading and text.
fn book(n: usize) -> Vec<Block> {
    let mut b = vec![toc()];
    for i in 0..n {
        b.push(heading(1, &format!("Chapter {i} {}", words_of(i, 2))));
        b.push(para(&words_of(i, 80)));
        b.push(heading(2, &format!("Section {i}.1")));
        b.push(para(&words_of(i + 3, 60)));
    }
    b
}

/// Text of the table-of-contents items: per entry (the `entry`th child of block 0), the title
/// text joined, the dots, and the number.
fn entry_texts(t: &PageTree, entry: u32) -> (String, String, String) {
    let (mut title, mut dots, mut num) = (String::new(), String::new(), String::new());
    for it in t.pages.iter().flat_map(|p| &p.items) {
        let p = it.node.path();
        if p.len() != 4 || p[0] != 0 || p[1] != 0 || p[2] != 0 || p[3] != entry {
            continue;
        }
        let ItemKind::Text(x) = &it.kind else {
            continue;
        };
        match it.role {
            StructRole::Artifact => dots.push_str(&x.text),
            StructRole::Link if x.text.chars().all(|c| c.is_ascii_digit()) => num.push_str(&x.text),
            _ => {
                if !title.is_empty() {
                    title.push(' ');
                }
                title.push_str(&x.text);
            }
        }
    }
    (title, dots, num)
}

// REQ: FOLIO-REQ-062
#[test]
fn entries_show_the_final_page_numbers_of_their_headings() {
    let t = lay(&small(book(14)));
    assert!(t.pages.len() > 10, "need two-digit page numbers");
    let hs = headings(&t);
    assert_eq!(hs.len(), 28);
    let mut two_digit = 0;
    for (i, h) in hs.iter().enumerate() {
        let (title, dots, num) = entry_texts(&t, i as u32);
        assert_eq!(title, h.title, "entry {i}");
        assert_eq!(
            num,
            h.label.to_string(),
            "entry {i}: number is the heading's page"
        );
        assert!(dots.len() >= 2, "entry {i} has a leader");
        two_digit += usize::from(num.len() == 2);
    }
    assert!(two_digit > 0);
    check_invariants(&t);
}

// REQ: FOLIO-REQ-062
#[test]
fn a_contents_longer_than_a_page_still_matches_the_headings() {
    // 120 headings: the contents alone fills several pages and moves every heading.
    let mut b = vec![toc()];
    for i in 0..120 {
        b.push(heading(2, &format!("Topic {i}")));
        b.push(para(&words_of(i, 25)));
    }
    let t = lay(&small(b));
    let hs = headings(&t);
    assert_eq!(hs.len(), 120);
    assert!(
        hs[0].page >= 3,
        "contents occupies pages first: heading on {}",
        hs[0].page
    );
    for (i, h) in hs.iter().enumerate() {
        assert_eq!(
            entry_texts(&t, i as u32).2,
            h.label.to_string(),
            "entry {i}"
        );
    }
    check_invariants(&t);
}

// REQ: FOLIO-REQ-062
#[test]
fn entries_are_links_to_destinations_on_the_heading_pages() {
    let t = lay(&small(book(6)));
    let hs = headings(&t);
    let mut at_right_edge = std::collections::BTreeSet::new();
    let mut per_dest = std::collections::BTreeMap::new();
    for p in &t.pages {
        for l in &p.links {
            let LinkTarget::Named(n) = &l.target else {
                continue;
            };
            let d = t.dests.iter().find(|d| &d.name == n).unwrap();
            // The k-th heading has destination folio-toc-<section>-<block>.
            let h = hs
                .iter()
                .find(|h| format!("folio-toc-{}-{}", h.node.path()[0], h.node.path()[2]) == *n)
                .unwrap();
            assert_eq!(d.page, h.page);
            *per_dest.entry(n.clone()).or_insert(0) += 1;
            if l.rect.x.raw() + l.rect.w.raw() == pts(180).raw() {
                at_right_edge.insert(n.clone());
            }
        }
    }
    // Per entry: the title is a link, and one more link covers the leader and the number.
    assert_eq!(per_dest.len(), hs.len());
    assert!(per_dest.values().all(|n| *n >= 2));
    assert_eq!(at_right_edge.len(), hs.len());
    assert_eq!(t.dests.len(), hs.len());
}

// REQ: FOLIO-REQ-062
#[test]
fn entries_are_indented_by_level_and_the_number_is_right_aligned() {
    let t = lay(&small(book(3)));
    let first_x = |entry: u32| {
        t.pages
            .iter()
            .flat_map(|p| &p.items)
            .find(|i| i.node.path() == [0, 0, 0, entry] && i.role == StructRole::Link)
            .unwrap()
            .x
            .raw()
    };
    let th = Theme::standard();
    assert_eq!(first_x(0), pts(20).raw());
    assert_eq!(first_x(1), pts(20).raw() + th.toc_indent.raw());
    for it in t.pages[0]
        .items
        .iter()
        .filter(|i| i.node.path().len() == 4 && i.node.path()[2] == 0)
    {
        if let ItemKind::Text(x) = &it.kind {
            if x.text.chars().all(|c| c.is_ascii_digit()) {
                assert_eq!(it.x.raw() + x.width.raw(), pts(180).raw());
            }
        }
    }
}

// REQ: FOLIO-REQ-062
#[test]
fn a_long_title_wraps_and_only_its_last_line_has_the_leader_and_number() {
    let mut checked = 0;
    for words in 20..40 {
        let long = words_of(5, words);
        let t = lay(&small(vec![toc(), heading(1, &long), para("text")]));
        let (title, dots, num) = entry_texts(&t, 0);
        assert_eq!(title, long);
        assert_eq!(num, headings(&t)[0].label.to_string());
        let ys = |pred: &dyn Fn(&moonshot_folio_layout::Item) -> bool| -> Vec<i64> {
            t.pages[0]
                .items
                .iter()
                .filter(|i| i.node.path() == [0, 0, 0, 0] && pred(i))
                .map(|i| i.y.raw())
                .collect()
        };
        let all: std::collections::BTreeSet<i64> = ys(&|_| true).into_iter().collect();
        assert!(all.len() > 1, "the title wraps");
        let number_y = ys(&|i| matches!(&i.kind, ItemKind::Text(x) if x.text == num));
        assert_eq!(
            number_y,
            [*all.iter().min().unwrap()],
            "number on the last line only"
        );
        if !dots.is_empty() {
            let dot_y = ys(&|i| i.role == StructRole::Artifact);
            assert_eq!(dot_y, number_y);
            checked += 1;
        }
        check_invariants(&t);
    }
    assert!(checked > 0);
}

// REQ: FOLIO-REQ-062
#[test]
fn a_document_without_headings_gets_no_output_from_the_field() {
    let t = lay(&small(vec![toc(), para("only text")]));
    assert_eq!(t.pages[0].items.len(), 1);
    assert!(t.dests.is_empty());
    let lonely = lay(&small(vec![toc()]));
    assert_eq!(lonely.pages.len(), 1);
    assert!(lonely.pages[0].items.is_empty());
}

// REQ: FOLIO-REQ-062
#[test]
fn headings_in_headers_and_footers_are_not_listed_and_the_field_is_body_only() {
    let mut s = section(
        setup(200, 300, 20),
        vec![toc(), heading(1, "Real"), para("x")],
    );
    s.header = vec![heading(1, "Running head")];
    let t = lay(&doc(vec![s]));
    assert_eq!(headings(&t).len(), 1);
    assert_eq!(entry_texts(&t, 0).0, "Real");
    let mut bad = section(setup(200, 300, 20), vec![para("x")]);
    bad.footer = vec![toc()];
    assert_eq!(
        try_lay(&doc(vec![bad])).unwrap_err(),
        FolioError::Unsupported("table of contents in a header or footer")
    );
}

// REQ: FOLIO-REQ-062
#[test]
fn contents_across_sections_with_restarted_numbering_use_the_labels() {
    let mut second = section(
        setup(200, 300, 20),
        vec![heading(1, "Appendix"), para(&words_of(2, 40))],
    );
    second.restart_page_numbers = true;
    let first = section(setup(200, 300, 20), {
        let mut b = vec![toc()];
        b.extend(paras(5, 50));
        b.push(heading(1, "Body"));
        b
    });
    let t = lay(&doc(vec![first, second]));
    let hs = headings(&t);
    assert_eq!(hs.len(), 2);
    assert_eq!(hs[1].label, 1);
    assert_eq!(entry_texts(&t, 1).2, "1");
}

// REQ: FOLIO-REQ-062
#[test]
fn layout_with_a_contents_is_deterministic_and_inside_the_box() {
    let d = small(book(9));
    let (a, b) = (lay(&d), lay(&d));
    assert_eq!(dump(&a), dump(&b));
    assert_eq!(a, b);
    check_invariants(&a);
}

/// The text of items in order, joining lines with one space only where the line ended at a
/// dropped space.
fn joined<'a>(items: impl Iterator<Item = &'a moonshot_folio_layout::Item>) -> String {
    let mut out = String::new();
    let mut prev: Option<(i64, bool)> = None;
    for it in items {
        let ItemKind::Text(t) = &it.kind else {
            continue;
        };
        if prev.is_some_and(|(y, sp)| y != it.y.raw() && sp) {
            out.push(' ');
        }
        prev = Some((it.y.raw(), t.space_after));
        out.push_str(&t.text);
    }
    out
}

// REQ: FOLIO-REQ-062
#[test]
fn wrapped_titles_keep_their_exact_text_in_headings_and_entries() {
    let titles = [
        (
            1_u8,
            "A well-known hyphenated-compound state-of-the-art-example".to_string(),
        ),
        (
            3,
            "https://example.org/alpha/bravo/charlie/delta/echo/foxtrot/golf/hotel".to_string(),
        ),
        (2, "x".repeat(300)),
        (
            2,
            "plain words that wrap at ordinary spaces only".to_string(),
        ),
    ];
    let mut b = vec![toc()];
    for (l, s) in &titles {
        b.push(heading(*l, s));
        b.push(para("body"));
    }
    let t = lay(&small(b));
    let hs = headings(&t);
    assert_eq!(hs.len(), titles.len());
    for (i, ((_, want), h)) in titles.iter().zip(&hs).enumerate() {
        assert_eq!(&h.title, want, "headings() title {i}");
        // Entry i: every Link/Artifact item of node [0,0,0,i] but the last (the number).
        let mut items: Vec<_> = t
            .pages
            .iter()
            .flat_map(|p| &p.items)
            .filter(|it| it.node.path() == [0, 0, 0, i as u32] && it.role == StructRole::Link)
            .collect();
        items.pop();
        let lines: std::collections::BTreeSet<i64> = items.iter().map(|it| it.y.raw()).collect();
        assert!(lines.len() > 1, "entry {i} wraps");
        assert_eq!(&joined(items.into_iter()), want, "entry {i} text");
    }
    // The heading blocks themselves, read straight from the items, say the same.
    for (k, (_, want)) in titles.iter().enumerate() {
        let block = 1 + 2 * k as u32;
        let items = t
            .pages
            .iter()
            .flat_map(|p| &p.items)
            .filter(|it| it.node.path().len() >= 3 && it.node.path()[..3] == [0, 0, block]);
        assert_eq!(&joined(items), want, "heading block {block}");
    }
}

// REQ: FOLIO-REQ-062
#[test]
fn headings_made_of_links_are_listed_and_empty_ones_are_skipped_with_their_destination() {
    use moonshot_folio_ir::{Heading, Inline, Pagination};
    let h = |inlines: Vec<Inline>| {
        Block::Heading(Heading {
            level: 2,
            inlines,
            pagination: Pagination::default(),
        })
    };
    let link = |t: &str| Inline::Link {
        target: "https://example.org/x".into(),
        text: t.into(),
    };
    let t = lay(&small(vec![
        toc(),
        h(vec![link("Only a link")]),
        para("a"),
        h(vec![
            Inline::Text("Pre ".into()),
            link("mid"),
            Inline::Text(" post".into()),
        ]),
        para("b"),
        h(vec![Inline::Anchor("only-anchor".into())]),
        h(vec![]),
        h(vec![Inline::Text("   ".into())]),
        h(vec![Inline::Text("Last".into())]),
    ]));
    let hs = headings(&t);
    let titles: Vec<&str> = hs.iter().map(|x| x.title.as_str()).collect();
    assert_eq!(titles, ["Only a link", "Pre mid post", "Last"]);
    // Three entries and three destinations (plus the user's anchor), none for skipped headings.
    let reserved: Vec<&str> = t
        .dests
        .iter()
        .map(|d| d.name.as_str())
        .filter(|n| n.starts_with("folio-toc-"))
        .collect();
    assert_eq!(reserved.len(), 3);
    assert!(t.dests.iter().any(|d| d.name == "only-anchor"));
    for i in 0..3 {
        assert!(!entry_texts(&t, i).0.is_empty());
    }
    assert!(entry_texts(&t, 3).0.is_empty());
}

// REQ: FOLIO-REQ-062
#[test]
fn user_anchors_may_not_use_the_reserved_heading_prefix() {
    let anchor = para_anchor("folio-toc-0-3");
    for with_toc in [false, true] {
        let mut b = if with_toc { vec![toc()] } else { vec![] };
        b.push(heading(1, "H"));
        b.push(anchor.clone());
        assert_eq!(
            try_lay(&small(b)).unwrap_err(),
            FolioError::Invalid("anchor name uses the reserved prefix folio-toc-"),
            "toc {with_toc}"
        );
    }
    let ok = para_anchor("folio-other");
    assert!(try_lay(&small(vec![ok])).is_ok());
}

fn para_anchor(n: &str) -> Block {
    common::para_in(vec![moonshot_folio_ir::Inline::Anchor(n.into())])
}

// REQ: FOLIO-REQ-062
#[test]
fn the_gap_before_the_leader_is_part_of_the_entry_link() {
    let t = lay(&small(vec![toc(), heading(1, "Short"), para("x")]));
    let mut rects: Vec<(i64, i64)> = t.pages[0]
        .links
        .iter()
        .filter(|l| l.node.path() == [0, 0, 0, 0])
        .map(|l| (l.rect.x.raw(), l.rect.x.raw() + l.rect.w.raw()))
        .collect();
    rects.sort_unstable();
    assert!(rects.len() >= 2);
    assert_eq!(rects[0].0, pts(20).raw(), "starts at the title");
    for w in rects.windows(2) {
        assert_eq!(w[0].1, w[1].0, "adjacent link rectangles leave no gap");
    }
    assert_eq!(
        rects.last().unwrap().1,
        pts(180).raw(),
        "reaches the right edge"
    );
}

// REQ: FOLIO-REQ-062
#[test]
fn deep_levels_on_narrow_pages_still_lay_out() {
    for width in [60_i64, 80, 100, 140] {
        let mut b = vec![toc()];
        for l in 1..=6 {
            b.push(heading(l, &format!("Level {l} title")));
            b.push(para("body"));
        }
        let d = doc(vec![section(setup(width, 400, 10), b)]);
        match try_lay(&d) {
            Ok(t) => check_invariants(&t),
            Err(e) => assert!(
                matches!(e, FolioError::Unsupported(_) | FolioError::Invalid(_)),
                "width {width}: {e:?}"
            ),
        }
    }
    // 100 pt wide is wide enough for every entry once the indent is clamped.
    let mut b = vec![toc()];
    b.push(heading(6, "Deep"));
    b.push(para("x"));
    let t = lay(&doc(vec![section(setup(100, 400, 10), b)]));
    check_invariants(&t);
    assert_eq!(headings(&t).len(), 1);
}
