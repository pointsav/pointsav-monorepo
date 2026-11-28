// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Regression tests for the independent review of the layout engine (task F5.1).

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::many_single_char_names,
    clippy::panic
)]

mod common;

use common::{
    cell, check_invariants, doc, heading, lay, list, para, para_in, para_with, paras, pts, row,
    section, setup, small, table, texts, try_lay, words_of,
};
use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_font::{MissingGlyph, Std14};
use moonshot_folio_ir::{Block, BreakKind, FieldKind, Inline, Pagination};
use moonshot_folio_layout::{
    dump, layout, Fonts, Item, ItemKind, Options, PageTree, StructRole, Theme,
};
use std::time::{Duration, Instant};

fn text_of(i: &Item) -> Option<&str> {
    match &i.kind {
        ItemKind::Text(t) => Some(t.text.as_str()),
        ItemKind::Rect(_) => None,
    }
}

/// Width of `s` in Helvetica at `size`, as the sum of per-character rounded advances.
fn width_of(s: &str, size: Emu) -> i64 {
    s.chars()
        .map(|c| {
            let u = i64::from(Std14::Helvetica.advance(c));
            Emu::from_ratio(u, size.raw(), 1000).unwrap().raw()
        })
        .sum()
}

/// The page holding the first text item equal to `s`.
fn page_of(t: &PageTree, s: &str) -> usize {
    t.pages
        .iter()
        .position(|p| p.items.iter().any(|i| text_of(i) == Some(s)))
        .unwrap_or_else(|| panic!("text {s:?} not found"))
}

// REQ: FOLIO-REQ-058
#[test]
fn a_body_page_field_has_the_measured_width_of_its_label_and_moves_what_follows() {
    let mut blocks = Vec::new();
    for _ in 0..12 {
        blocks.push(para_in(vec![
            Inline::Text("on page ".into()),
            Inline::FieldRef(FieldKind::Page),
            Inline::Text(" end".into()),
        ]));
        blocks.push(Block::Break(BreakKind::Page));
    }
    let t = lay(&small(blocks));
    assert_eq!(t.pages.len(), 12, "a trailing page break adds no page");
    for (i, p) in t.pages.iter().enumerate() {
        let label = (i + 1).to_string();
        let items: Vec<&Item> = p.items.iter().filter(|x| text_of(x).is_some()).collect();
        assert_eq!(items.len(), 3);
        assert_eq!(text_of(items[1]), Some(label.as_str()));
        let ItemKind::Text(f) = &items[1].kind else {
            panic!()
        };
        assert_eq!(f.width.raw(), width_of(&label, f.size), "page {label}");
        let ItemKind::Text(first) = &items[0].kind else {
            panic!()
        };
        assert_eq!(items[1].x.raw(), items[0].x.raw() + first.width.raw());
        assert_eq!(items[2].x.raw(), items[1].x.raw() + f.width.raw());
    }
    check_invariants(&t);
}

// REQ: FOLIO-REQ-058
#[test]
fn a_page_field_after_an_unbreakable_word_never_overflows_the_line() {
    // The word and the placeholder form one unbreakable run; the field must wrap whole.
    for n in 1..45 {
        let mut blocks = Vec::new();
        for _ in 0..12 {
            blocks.push(para_in(vec![
                Inline::Text("m".repeat(n)),
                Inline::FieldRef(FieldKind::Page),
            ]));
            blocks.push(Block::Break(BreakKind::Page));
        }
        let t = lay(&small(blocks));
        check_invariants(&t);
        for (i, p) in t.pages.iter().enumerate() {
            let label = (i + 1).to_string();
            let hits = p
                .items
                .iter()
                .filter(|x| text_of(x) == Some(label.as_str()))
                .count();
            assert_eq!(hits, 1, "n {n} page {label}: field shown once");
        }
    }
}

fn pages_of(blocks: Vec<Block>) -> usize {
    lay(&small(blocks)).pages.len()
}

// REQ: FOLIO-REQ-056
#[test]
fn page_breaks_at_the_end_never_leave_a_blank_page() {
    assert_eq!(pages_of(vec![para("a"), Block::Break(BreakKind::Page)]), 1);
    assert_eq!(
        pages_of(vec![
            para("a"),
            Block::Break(BreakKind::Page),
            Block::Break(BreakKind::Page)
        ]),
        1
    );
    assert_eq!(
        pages_of(vec![
            para("a"),
            Block::Break(BreakKind::Page),
            para("b"),
            Block::Break(BreakKind::Page)
        ]),
        2
    );
    assert_eq!(pages_of(vec![Block::Break(BreakKind::Page), para("a")]), 1);
    // A break still separates content that follows it.
    assert_eq!(
        pages_of(vec![para("a"), Block::Break(BreakKind::Page), para("b")]),
        2
    );
}

// REQ: FOLIO-REQ-056
#[test]
fn a_page_break_at_the_end_of_a_section_does_not_add_a_page() {
    let a = section(
        setup(200, 300, 20),
        vec![para("a"), Block::Break(BreakKind::Page)],
    );
    let b = section(setup(200, 300, 20), vec![para("b")]);
    let t = lay(&doc(vec![a, b]));
    assert_eq!(t.pages.len(), 2);
    assert_eq!(
        t.pages.iter().map(|p| p.section).collect::<Vec<_>>(),
        [0, 1]
    );
}

// REQ: FOLIO-REQ-056
#[test]
fn an_empty_section_produces_no_page_unless_the_document_would_have_none() {
    let empty = || section(setup(200, 300, 20), vec![]);
    let text = |s: &str| section(setup(200, 300, 20), vec![para(s)]);
    let labels = |d| {
        lay(&d)
            .pages
            .iter()
            .map(|p| (p.section, p.label))
            .collect::<Vec<_>>()
    };
    assert_eq!(labels(doc(vec![empty(), text("a")])), [(1, 1)]);
    assert_eq!(labels(doc(vec![text("a"), empty()])), [(0, 1)]);
    // Numbering continues across a dropped section.
    assert_eq!(
        labels(doc(vec![text("a"), empty(), text("b")])),
        [(0, 1), (2, 2)]
    );
    // Nothing at all: one blank page (that of the last section), so the tree is never empty.
    assert_eq!(labels(doc(vec![empty(), empty()])), [(1, 1)]);
    // A header on an otherwise empty section does not make it a page.
    let mut chrome = empty();
    chrome.header = vec![para("H")];
    assert_eq!(labels(doc(vec![text("a"), chrome])), [(0, 1)]);
}

// REQ: FOLIO-REQ-057
#[test]
fn cutting_a_huge_row_takes_time_in_proportion_to_its_size() {
    let words = words_of(7, 20000);
    let d = small(vec![table(
        2,
        1,
        vec![
            row(vec![cell("Key"), cell("Notes")]),
            row(vec![cell("k"), cell(&words)]),
            row(vec![cell("after"), cell("x")]),
        ],
    )]);
    let started = Instant::now();
    let t = lay(&d);
    let took = started.elapsed();
    assert!(took < Duration::from_secs(8), "took {took:?}");
    assert!(t.pages.len() > 100);
    // Nothing is lost or duplicated.
    let notes: Vec<&str> = t
        .pages
        .iter()
        .flat_map(|p| &p.items)
        .filter(|i| i.node.path() == [0, 0, 0, 1, 1] && i.role == StructRole::TableCell)
        .filter_map(text_of)
        .collect();
    assert_eq!(notes.join(" "), words);
    assert_eq!(texts(&t).last().unwrap(), "x");
}

// REQ: FOLIO-REQ-061
#[test]
fn a_huge_paragraph_under_a_small_limit_fails_fast() {
    let big = "word ".repeat(1_000_000);
    let opts = Options {
        limits: Limits::default().with_max_cells(1000),
        ..Options::default()
    };
    let d = small(vec![para(&big)]);
    let started = Instant::now();
    let r = layout(&d, &Fonts::std14(), &Theme::standard(), &opts);
    assert!(matches!(r, Err(FolioError::Limit(_))), "{r:?}");
    assert!(started.elapsed() < Duration::from_secs(2));
    // The same for text inside a table cell and for many lines under a tiny line budget.
    let cellular = small(vec![table(1, 0, vec![row(vec![cell(&big)])])]);
    let r = layout(&cellular, &Fonts::std14(), &Theme::standard(), &opts);
    assert!(matches!(r, Err(FolioError::Limit(_))));
    let lines = small(paras(300, 20));
    let few = Options {
        limits: Limits::default().with_max_cells(100),
        ..Options::default()
    };
    let r = layout(&lines, &Fonts::std14(), &Theme::standard(), &few);
    assert!(matches!(r, Err(FolioError::Limit(_))));
}

/// A paragraph of `n` lines (found by search so the test does not depend on font tables).
fn lines_para(n: usize, wo: bool) -> Block {
    let pag = Pagination {
        widow_orphan: wo,
        ..Pagination::default()
    };
    for words in 1..200 {
        let b = para_with(&words_of(3, words), pag);
        let t = lay(&small(vec![b.clone()]));
        if t.pages[0].items.len() == n {
            return b;
        }
    }
    panic!("no paragraph with {n} lines");
}

// REQ: FOLIO-REQ-056
#[test]
fn a_heading_is_not_stranded_before_a_paragraph_that_cannot_split() {
    // Three lines with widow and orphan control cannot split (2 + 2 > 3).
    let p = lines_para(3, true);
    for fill in 0..30 {
        let mut blocks = paras(1, 1);
        blocks.extend((0..fill).map(|_| para("x")));
        blocks.push(heading(2, "Head"));
        blocks.push(p.clone());
        let t = lay(&small(blocks));
        let h = page_of(&t, "Head");
        let first = texts(&t)
            .iter()
            .position(|s| s == "Head")
            .map(|i| texts(&t)[i + 1].clone())
            .unwrap();
        assert_eq!(h, page_of(&t, &first), "fill {fill}");
    }
}

// REQ: FOLIO-REQ-056
#[test]
fn a_heading_is_not_stranded_before_a_list_item_that_cannot_split() {
    let item: String = (0..lines_para_words(3)).map(|_| "word ").collect();
    for fill in 0..30 {
        let mut blocks: Vec<Block> = (0..fill).map(|_| para("x")).collect();
        blocks.push(heading(3, "Head"));
        blocks.push(list(false, &[item.trim_end(), "two"]));
        let t = lay(&small(blocks));
        let after = texts(&t)
            .iter()
            .position(|s| s == "Head")
            .map(|i| texts(&t)[i + 1].clone())
            .unwrap();
        assert_eq!(page_of(&t, "Head"), page_of(&t, &after), "fill {fill}");
    }
}

/// Words of "word" that wrap to `n` lines in a list item of the small page.
fn lines_para_words(n: usize) -> usize {
    (1..100)
        .find(|w| {
            let item = "word ".repeat(*w);
            let t = lay(&small(vec![list(false, &[item.trim_end()])]));
            t.pages[0]
                .items
                .iter()
                .filter(|i| matches!(&i.kind, ItemKind::Text(x) if x.text.starts_with("word")))
                .count()
                == n
        })
        .unwrap()
}

// REQ: FOLIO-REQ-056
#[test]
fn a_keep_with_next_block_shares_a_page_with_the_start_of_its_successor() {
    // Rule: heading i (keep with next) is on the same page as the first line of the block after
    // it whenever the rest of its chain, from heading i through that first line, fits one page
    // body. When even that is taller than a page the rule cannot be met (the last heading
    // always can, so it is always checked). Swept over chain lengths, filler and page heights.
    let (head, gap, tail) = (243_840_i64, 76_200 + 152_400, 76_200 + 167_640);
    for height in [200_i64, 233, 260, 300, 349] {
        let body = pts(height - 40).raw();
        for n in 1..10_i64 {
            for fill in 0..60 {
                let mut blocks: Vec<Block> = (0..fill).map(|_| para("x")).collect();
                blocks.extend((0..n).map(|i| heading(3, &format!("H{i}"))));
                blocks.push(para("Tail"));
                let d = doc(vec![section(setup(200, height, 20), blocks)]);
                let t = lay(&d);
                for i in 0..n {
                    let rest = (n - i) * head + (n - i - 1) * gap + tail;
                    let this = page_of(&t, &format!("H{i}"));
                    let next = if i + 1 < n {
                        format!("H{}", i + 1)
                    } else {
                        "Tail".to_string()
                    };
                    if rest <= body {
                        assert_eq!(
                            this,
                            page_of(&t, &next),
                            "height {height} n {n} fill {fill} heading {i}"
                        );
                    }
                }
                check_invariants(&t);
            }
        }
    }
}

// REQ: FOLIO-REQ-057
#[test]
fn a_row_that_fits_a_page_is_not_cut_to_make_room_for_the_repeated_header() {
    // Header 72 pt, rows 204 pt, page body 260 pt: 72 + 204 > 260 but 204 fits alone.
    let lines = |n: usize, s: &str| vec![s; n].join("\n");
    let mut rows = vec![row(vec![cell(&lines(5, "h"))])];
    for _ in 0..3 {
        rows.push(row(vec![cell(&lines(15, "r"))]));
    }
    let t = lay(&small(vec![table(1, 1, rows)]));
    let count = |p: usize, s: &str| {
        t.pages[p]
            .items
            .iter()
            .filter(|i| text_of(i) == Some(s))
            .count()
    };
    let per_page: Vec<usize> = (0..t.pages.len()).map(|p| count(p, "r")).collect();
    // Every page holds whole rows (15 lines each) and the three rows are not split.
    assert!(per_page.iter().all(|n| n % 15 == 0), "{per_page:?}");
    assert_eq!(per_page.iter().sum::<usize>(), 45);
    // A page whose first row needs the whole body shows no repeated header.
    assert_eq!(count(1, "h"), 0);
    check_invariants(&t);
}

fn count_char(t: &PageTree, c: char) -> usize {
    t.pages
        .iter()
        .flat_map(|p| &p.items)
        .filter_map(text_of)
        .map(|s| s.chars().filter(|x| *x == c).count())
        .sum()
}

// REQ: FOLIO-REQ-008
#[test]
fn substituted_counts_each_drawn_occurrence() {
    let opts = Options {
        missing_glyph: MissingGlyph::Notdef,
        ..Options::default()
    };
    let run = |d| layout(&d, &Fonts::std14(), &Theme::standard(), &opts).unwrap();
    // A footer is drawn on every page.
    let mut s = section(setup(200, 300, 20), paras(12, 40));
    s.footer = vec![para("f\u{101}")];
    s.blocks.push(para("b\u{101}"));
    let t = run(doc(vec![s]));
    assert!(t.pages.len() > 3);
    assert_eq!(t.substituted, count_char(&t, '\u{101}'));
    assert_eq!(t.substituted, t.pages.len() + 1);
    // A repeated table header is drawn on every continuation page.
    let mut rows = vec![row(vec![cell("H\u{101}")])];
    rows.extend((0..40).map(|i| row(vec![cell(&words_of(i, 3))])));
    let t = run(small(vec![table(1, 1, rows)]));
    assert!(t.pages.len() > 2);
    assert_eq!(t.substituted, count_char(&t, '\u{101}'));
    assert_eq!(t.substituted, t.pages.len());
}

// REQ: FOLIO-REQ-057
#[test]
fn table_strokes_stay_inside_the_content_box_and_repeated_rows_are_marked() {
    let mut rows = vec![row(vec![cell("Head"), cell("Two")])];
    rows.extend((0..40).map(|i| row(vec![cell(&words_of(i, 3)), cell("x")])));
    let t = lay(&small(vec![table(2, 1, rows)]));
    check_invariants(&t);
    for p in &t.pages {
        let b = p.content_box().unwrap();
        for i in &p.items {
            if let ItemKind::Rect(r) = &i.kind {
                let hw = r.stroke.unwrap().1.raw() / 2;
                assert!(i.x.raw() - hw >= b.x.raw());
                assert!(i.x.raw() + r.w.raw() + hw <= b.x.raw() + b.w.raw());
                assert!(i.y.raw() - hw >= b.y.raw());
                assert!(i.y.raw() + r.h.raw() + hw <= b.y.raw() + b.h.raw());
            }
        }
    }
    // Header items are marked only where they are repeated.
    for (pi, p) in t.pages.iter().enumerate() {
        let header: Vec<&Item> = p
            .items
            .iter()
            .filter(|i| i.node.path().get(3) == Some(&0))
            .collect();
        assert!(!header.is_empty());
        assert!(header.iter().all(|i| i.repeated == (pi > 0)), "page {pi}");
        assert!(p
            .items
            .iter()
            .filter(|i| i.node.path().get(3) != Some(&0))
            .all(|i| !i.repeated));
    }
    assert!(dump(&t).contains("TableCell repeated"));
    assert!(!dump(&t).lines().next().unwrap().contains("repeated"));
}

// REQ: FOLIO-REQ-059
#[test]
fn an_empty_link_target_is_rejected() {
    let d = small(vec![para_in(vec![Inline::Link {
        target: String::new(),
        text: "x".into(),
    }])]);
    assert_eq!(
        try_lay(&d).unwrap_err(),
        FolioError::Invalid("link target is empty")
    );
}

// REQ: FOLIO-REQ-057
#[test]
fn header_and_footer_tables_use_the_chrome_size_and_theme_bounds_are_checked() {
    let mut s = section(setup(300, 300, 20), paras(1, 5));
    s.footer = vec![table(2, 0, vec![row(vec![cell("a"), cell("b")])])];
    let t = lay(&doc(vec![s]));
    let th = Theme::standard();
    let sizes: Vec<i64> = t
        .pages
        .iter()
        .flat_map(|p| &p.items)
        .filter(|i| i.role == StructRole::Footer)
        .filter_map(|i| match &i.kind {
            ItemKind::Text(x) => Some(x.size.raw()),
            ItemKind::Rect(_) => None,
        })
        .collect();
    assert_eq!(sizes, [th.chrome_size.raw(); 2]);
    let d = small(vec![para("x")]);
    let run = |th: Theme| layout(&d, &Fonts::std14(), &th, &Options::default());
    let mut a = Theme::standard();
    a.rule_width = pts(73);
    assert_eq!(
        run(a).unwrap_err(),
        FolioError::Invalid("theme value out of range")
    );
    let mut b = Theme::standard();
    b.list_indent = pts(721);
    assert_eq!(
        run(b).unwrap_err(),
        FolioError::Invalid("theme value out of range")
    );
}

// REQ: FOLIO-REQ-055
#[test]
fn list_indent_is_uniform_for_markers_of_different_widths() {
    let items: Vec<String> = (0..1200).map(|i| format!("i{i}")).collect();
    let refs: Vec<&str> = items.iter().map(String::as_str).collect();
    let t = lay(&small(vec![list(true, &refs)]));
    let xs: std::collections::BTreeSet<i64> = t
        .pages
        .iter()
        .flat_map(|p| &p.items)
        .filter(|i| matches!(&i.kind, ItemKind::Text(x) if x.text.starts_with('i')))
        .map(|i| i.x.raw())
        .collect();
    assert_eq!(xs.len(), 1);
    check_invariants(&t);
}

/// Timing probe for the 80000-word cell: `cargo test --release -p moonshot-folio-layout --test
/// review -- --ignored --nocapture`.
// REQ: FOLIO-REQ-057
#[test]
#[ignore = "timing probe"]
#[allow(clippy::print_stdout)]
fn timing_of_an_eighty_thousand_word_cell() {
    let words = words_of(7, 80000);
    let d = small(vec![table(1, 0, vec![row(vec![cell(&words)])])]);
    let started = Instant::now();
    let t = lay(&d);
    println!(
        "80000 words: {} pages in {:?}",
        t.pages.len(),
        started.elapsed()
    );
    assert!(started.elapsed() < Duration::from_secs(2));
}
