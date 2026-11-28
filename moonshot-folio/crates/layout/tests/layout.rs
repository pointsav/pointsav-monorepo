// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Behaviour tests for page layout (task F5.1).

#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines
)]

mod common;

use common::{
    cell, check_invariants, doc, heading, lay, list, num, page_texts, para, para_in, para_with,
    paras, pts, row, section, setup, small, span, table, texts, try_lay, words_of,
};
use moonshot_folio_core::{Emu, FolioError, Limits};
use moonshot_folio_font::{Font, MissingGlyph, Std14};
use moonshot_folio_ir::{
    Block, Body, BreakKind, FieldKind, Image, ImageFormat, Inline, Pagination,
};
use moonshot_folio_layout::{
    layout, Area, Face, Fonts, ItemKind, LinkTarget, NodeId, Options, StructRole, Theme,
};

fn lines_of(t: &moonshot_folio_layout::PageTree, page: usize) -> Vec<String> {
    page_texts(t, page).into_iter().map(|x| x.2).collect()
}

// REQ: FOLIO-REQ-055
#[test]
fn text_is_wrapped_to_the_content_width_with_standard_font_advances() {
    let t = lay(&small(paras(1, 40)));
    let content_w = pts(160).raw();
    let mut seen = 0;
    for it in &t.pages[0].items {
        let ItemKind::Text(x) = &it.kind else {
            continue;
        };
        assert!(x.width.raw() <= content_w);
        // The width is the sum of rounded per-character advances.
        let want: i64 = x
            .text
            .chars()
            .map(|c| {
                let u = i64::from(Std14::Helvetica.advance(c));
                Emu::from_ratio(u, x.size.raw(), 1000).unwrap().raw()
            })
            .sum();
        assert_eq!(x.width.raw(), want);
        seen += 1;
    }
    assert!(seen > 3);
    // Joining the lines restores the words in order.
    let joined = texts(&t).join(" ");
    assert_eq!(joined, words_of(0, 40));
}

// REQ: FOLIO-REQ-055
#[test]
fn baselines_step_by_the_leading_and_start_inside_the_box() {
    let t = lay(&small(paras(1, 30)));
    let ys: Vec<i64> = page_texts(&t, 0).iter().map(|x| x.1).collect();
    let step = ys[0] - ys[1];
    assert_eq!(
        step,
        Emu::from_ratio(pts(11).raw(), 120, 100).unwrap().raw()
    );
    assert!(ys.windows(2).all(|w| w[0] - w[1] == step));
    check_invariants(&t);
}

// REQ: FOLIO-REQ-055
#[test]
fn lists_use_hanging_markers_and_restart_numbering_per_list() {
    let t = lay(&small(vec![
        list(false, &["alpha", "bravo"]),
        list(true, &["one", "two", "three"]),
        list(true, &["again"]),
    ]));
    let all = texts(&t);
    assert_eq!(
        all,
        [
            "\u{2022}", "alpha", "\u{2022}", "bravo", "1.", "one", "2.", "two", "3.", "three",
            "1.", "again"
        ]
    );
    let items = &t.pages[0].items;
    let marker = &items[0];
    let body = &items[1];
    assert_eq!(marker.role, StructRole::ListItem);
    assert_eq!(body.role, StructRole::ListItem);
    // The text starts at the hanging indent; the marker ends before it; same baseline.
    assert_eq!(body.x.raw() - pts(20).raw(), pts(18).raw());
    let ItemKind::Text(m) = &marker.kind else {
        panic!()
    };
    assert!(marker.x.raw() + m.width.raw() <= body.x.raw());
    assert_eq!(marker.y, body.y);
    check_invariants(&t);
}

// REQ: FOLIO-REQ-055
#[test]
fn a_long_list_widens_the_indent_for_wide_markers() {
    let items: Vec<String> = (0..120).map(|i| format!("item {i}")).collect();
    let refs: Vec<&str> = items.iter().map(String::as_str).collect();
    let t = lay(&small(vec![list(true, &refs)]));
    let xs: Vec<i64> = t
        .pages
        .iter()
        .flat_map(|p| &p.items)
        .filter(|i| matches!(&i.kind, ItemKind::Text(x) if x.text.starts_with("item")))
        .map(|i| i.x.raw())
        .collect();
    assert_eq!(xs.len(), 120);
    assert!(
        xs.windows(2).all(|w| w[0] == w[1]),
        "indent must be uniform"
    );
    check_invariants(&t);
}

// REQ: FOLIO-REQ-055
#[test]
fn styled_runs_bold_links_and_highlights_split_a_line_into_items() {
    let t = lay(&small(vec![para_in(vec![
        Inline::Text("plain ".into()),
        Inline::Run {
            role: "bold".into(),
            text: "strong".into(),
        },
        Inline::Highlight(" marked".into()),
    ])]));
    let items = &t.pages[0].items;
    let kinds: Vec<(StructRole, bool)> = items
        .iter()
        .map(|i| (i.role, matches!(i.kind, ItemKind::Rect(_))))
        .collect();
    assert_eq!(
        kinds,
        [
            (StructRole::Paragraph, false),
            (StructRole::Paragraph, false),
            (StructRole::Artifact, true),
            (StructRole::Paragraph, false)
        ]
    );
    let ItemKind::Text(b) = &items[1].kind else {
        panic!()
    };
    assert_eq!(b.font, moonshot_folio_layout::FontSlot::Bold);
    assert_eq!(items[1].node.path(), &[0, 0, 0, 1]);
}

// REQ: FOLIO-REQ-056
#[test]
fn a_heading_stays_with_the_start_of_the_next_paragraph() {
    // Fill the page so that a heading would fit alone but not with its paragraph's first line.
    for fill in 8..16 {
        let mut blocks = paras(1, fill * 5);
        blocks.push(heading(2, "Heading"));
        blocks.push(para(&words_of(3, 30)));
        let t = lay(&small(blocks));
        let page_of = |s: &str| {
            (0..t.pages.len())
                .find(|p| lines_of(&t, *p).iter().any(|l| l == s))
                .unwrap()
        };
        let h = page_of("Heading");
        let first = texts(&t)
            .iter()
            .position(|x| x == "Heading")
            .map(|i| texts(&t)[i + 1].clone())
            .unwrap();
        assert_eq!(h, page_of(&first), "fill {fill}");
        check_invariants(&t);
    }
}

// REQ: FOLIO-REQ-056
#[test]
fn widow_and_orphan_control_keeps_two_lines_on_each_side() {
    let wo = Pagination {
        widow_orphan: true,
        ..Pagination::default()
    };
    let mut saw_split = false;
    for fill in 0..40 {
        let mut b = vec![para(&words_of(1, fill))];
        b.push(para_with(&words_of(2, 90), wo));
        let t = lay(&small(b));
        // Count lines of the second paragraph (node 0.0.1.0) per page.
        let per: Vec<usize> = t
            .pages
            .iter()
            .map(|p| {
                p.items
                    .iter()
                    .filter(|i| {
                        i.node.path().get(2) == Some(&1) && matches!(i.kind, ItemKind::Text(_))
                    })
                    .count()
            })
            .filter(|n| *n > 0)
            .collect();
        if per.len() > 1 {
            saw_split = true;
            assert!(per.iter().all(|n| *n >= 2), "fill {fill}: {per:?}");
        }
    }
    assert!(saw_split);
}

// REQ: FOLIO-REQ-056
#[test]
fn without_the_flag_a_single_line_may_be_left_alone() {
    let mut found = false;
    for fill in 0..40 {
        let b = vec![para(&words_of(1, fill)), para(&words_of(2, 90))];
        let t = lay(&small(b));
        for p in &t.pages {
            let n = p
                .items
                .iter()
                .filter(|i| i.node.path().get(2) == Some(&1) && matches!(i.kind, ItemKind::Text(_)))
                .count();
            if n == 1 {
                found = true;
            }
        }
    }
    assert!(found, "the control must be opt-in, as in the IR");
}

// REQ: FOLIO-REQ-056
#[test]
fn page_breaks_and_break_before_start_pages_and_never_leave_blank_ones() {
    let d = small(vec![
        para("one"),
        Block::Break(BreakKind::Page),
        Block::Break(BreakKind::Page),
        para_with(
            "two",
            Pagination {
                page_break_before: true,
                ..Pagination::default()
            },
        ),
        para("three"),
    ]);
    let t = lay(&d);
    assert_eq!(t.pages.len(), 2);
    assert_eq!(lines_of(&t, 0), ["one"]);
    // The second page break is ignored (page already empty), break-before on a fresh page too.
    assert_eq!(lines_of(&t, 1), ["two", "three"]);
}

// REQ: FOLIO-REQ-056
#[test]
fn each_section_starts_a_page_with_its_own_geometry_and_numbering() {
    let mut b = section(setup(300, 200, 30), paras(1, 10));
    b.restart_page_numbers = true;
    let d = doc(vec![
        section(setup(200, 300, 20), paras(1, 10)),
        section(setup(200, 300, 20), paras(1, 10)),
        b,
    ]);
    let t = lay(&d);
    let geo: Vec<(i64, i64, i64, usize, u32)> = t
        .pages
        .iter()
        .map(|p| {
            (
                p.width.raw(),
                p.height.raw(),
                p.margin.raw(),
                p.section,
                p.label,
            )
        })
        .collect();
    assert_eq!(
        geo,
        [
            (pts(200).raw(), pts(300).raw(), pts(20).raw(), 0, 1),
            (pts(200).raw(), pts(300).raw(), pts(20).raw(), 1, 2),
            (pts(300).raw(), pts(200).raw(), pts(30).raw(), 2, 1),
        ]
    );
    check_invariants(&t);
}

// REQ: FOLIO-REQ-057
#[test]
fn table_rows_stay_whole_and_header_rows_repeat_on_every_page() {
    let mut rows = vec![row(vec![cell("Item"), cell("Amount")])];
    for i in 0..40 {
        rows.push(row(vec![cell(&words_of(i, 4)), num("12.50")]));
    }
    let t = lay(&small(vec![table(2, 1, rows)]));
    assert!(t.pages.len() > 2);
    let mut body_rows = 0;
    for p in &t.pages {
        let rows_here: std::collections::BTreeSet<u32> = p
            .items
            .iter()
            .filter(|i| i.role == StructRole::Artifact)
            .map(|i| i.node.path()[3])
            .collect();
        assert!(rows_here.contains(&0), "header row missing on a page");
        body_rows += rows_here.iter().filter(|r| **r > 0).count();
        // Every row on the page is complete: both cells are present.
        for r in &rows_here {
            let cells = p
                .items
                .iter()
                .filter(|i| i.role == StructRole::Artifact && i.node.path()[3] == *r)
                .count();
            assert_eq!(cells, 2);
        }
    }
    assert_eq!(body_rows, 40, "each body row appears exactly once");
    check_invariants(&t);
}

// REQ: FOLIO-REQ-057
#[test]
fn a_table_without_header_rows_repeats_nothing() {
    let rows: Vec<_> = (0..40).map(|i| row(vec![cell(&words_of(i, 4))])).collect();
    let t = lay(&small(vec![table(1, 0, rows)]));
    let mut seen = std::collections::BTreeSet::new();
    for p in &t.pages {
        for i in p.items.iter().filter(|i| i.role == StructRole::Artifact) {
            assert!(seen.insert(i.node), "row repeated: {:?}", i.node);
        }
    }
    assert_eq!(seen.len(), 40);
}

// REQ: FOLIO-REQ-057
#[test]
fn colspan_cells_cover_their_columns_and_numbers_align_right() {
    let t = lay(&small(vec![table(
        3,
        0,
        vec![
            row(vec![span("wide", 2), cell("c")]),
            row(vec![cell("a"), cell("b"), num("1234.50")]),
        ],
    )]));
    let rects: Vec<(Vec<u32>, i64, i64)> = t.pages[0]
        .items
        .iter()
        .filter_map(|i| match &i.kind {
            ItemKind::Rect(r) => Some((i.node.path().to_vec(), i.x.raw(), r.w.raw())),
            ItemKind::Text(_) => None,
        })
        .collect();
    // 160 pt over 3 columns: 53.33.. pt each, remainder in the last column.
    // Each rectangle is inset by half the 0.5 pt rule so its stroke stays in the cell.
    let col = pts(160).raw() / 3;
    let half = 6350 / 2;
    assert_eq!(rects[0].2, 2 * col - 2 * half);
    assert_eq!(rects[1].1, pts(20).raw() + 2 * col + half);
    assert_eq!(rects[1].1 + rects[1].2, pts(180).raw() - half);
    assert_eq!(rects[0].2 + rects[1].2, pts(160).raw() - 4 * half);
    // The number ends at the cell's right edge less the padding.
    let number = t.pages[0]
        .items
        .iter()
        .find(|i| matches!(&i.kind, ItemKind::Text(x) if x.text == "1234.50"))
        .unwrap();
    let ItemKind::Text(x) = &number.kind else {
        panic!()
    };
    assert_eq!(
        number.x.raw() + x.width.raw(),
        pts(180).raw() - pts(3).raw()
    );
    assert_eq!(number.role, StructRole::TableCell);
    assert_eq!(number.node.path(), &[0, 0, 0, 1, 2]);
    assert_eq!(number.node.parent().unwrap().path(), &[0, 0, 0, 1]);
}

// REQ: FOLIO-REQ-057
#[test]
fn a_row_taller_than_a_page_is_cut_by_line_and_nothing_is_lost() {
    let tall = words_of(2, 400);
    let t = lay(&small(vec![table(
        2,
        1,
        vec![
            row(vec![cell("Key"), cell("Notes")]),
            row(vec![cell("k"), cell(&tall)]),
            row(vec![cell("after"), cell("x")]),
        ],
    )]));
    assert!(t.pages.len() >= 3);
    let notes: Vec<String> = texts(&t)
        .into_iter()
        .filter(|s| s != "Key" && s != "Notes" && s != "k" && s != "after" && s != "x")
        .collect();
    assert_eq!(notes.join(" "), tall);
    // The header repeats on each page and the last row follows the cut row.
    assert!(t
        .pages
        .iter()
        .all(|p| texts_on(p).contains(&"Key".to_string())));
    assert_eq!(texts(&t).last().unwrap(), "x");
    check_invariants(&t);
}

fn texts_on(p: &moonshot_folio_layout::Page) -> Vec<String> {
    p.items
        .iter()
        .filter_map(|i| match &i.kind {
            ItemKind::Text(x) => Some(x.text.clone()),
            ItemKind::Rect(_) => None,
        })
        .collect()
}

// REQ: FOLIO-REQ-057
#[test]
fn degenerate_tables_are_handled() {
    // No rows: nothing drawn. Too many columns for the width: an error, not an overflow.
    let t = lay(&small(vec![table(2, 0, vec![]), para("after")]));
    assert_eq!(texts(&t), ["after"]);
    let wide = vec![row((0..200).map(|_| cell("x")).collect())];
    let r = try_lay(&small(vec![table(200, 0, wide)]));
    assert_eq!(
        r.unwrap_err(),
        FolioError::Unsupported("table columns too narrow")
    );
}

// REQ: FOLIO-REQ-058
#[test]
fn page_and_page_count_fields_resolve_in_two_passes() {
    let mut s = section(setup(200, 300, 20), paras(25, 40));
    s.footer = vec![para_in(vec![
        Inline::FieldRef(FieldKind::Page),
        Inline::Text("/".into()),
        Inline::FieldRef(FieldKind::NumPages),
    ])];
    let t = lay(&doc(vec![s]));
    let n = t.pages.len();
    assert!(n >= 10, "need two-digit page counts, got {n}");
    for (i, p) in t.pages.iter().enumerate() {
        let foot: Vec<String> = p
            .items
            .iter()
            .filter(|x| x.role == StructRole::Footer)
            .filter_map(|x| match &x.kind {
                ItemKind::Text(t) => Some(t.text.clone()),
                ItemKind::Rect(_) => None,
            })
            .collect();
        assert_eq!(foot, [(i + 1).to_string(), "/".into(), n.to_string()]);
    }
    check_invariants(&t);
}

// REQ: FOLIO-REQ-058
#[test]
fn the_first_page_can_drop_the_header_but_keeps_the_footer() {
    let mut s = section(setup(200, 300, 20), paras(4, 30));
    s.header = vec![para("HEAD")];
    s.footer = vec![para("FOOT")];
    s.different_first_page = true;
    let t = lay(&doc(vec![s]));
    let has = |p: usize, w: &str| lines_of(&t, p).iter().any(|l| l == w);
    assert!(!has(0, "HEAD") && has(0, "FOOT"));
    assert!(has(1, "HEAD") && has(1, "FOOT"));
    // The header and footer sit inside the content box and the body clears them.
    check_invariants(&t);
}

// REQ: FOLIO-REQ-058
#[test]
fn body_page_fields_show_the_page_where_the_line_lands() {
    let mut blocks = paras(7, 40);
    blocks.push(Block::Field(FieldKind::Page));
    blocks.push(para_in(vec![
        Inline::Text("on page ".into()),
        Inline::FieldRef(FieldKind::Page),
        Inline::Text(" of ".into()),
        Inline::FieldRef(FieldKind::NumPages),
    ]));
    let t = lay(&small(blocks));
    let n = t.pages.len();
    let all = texts(&t);
    let at = all.iter().position(|s| s == "on page ").unwrap();
    assert_eq!(all[at + 1], n.to_string());
    assert_eq!(all[at + 3], n.to_string());
    assert_eq!(all[at - 1], n.to_string());
}

// REQ: FOLIO-REQ-058
#[test]
fn headers_and_footers_that_leave_no_room_are_rejected() {
    let mut s = section(setup(200, 100, 20), paras(1, 5));
    s.header = vec![para(&words_of(1, 200))];
    assert!(matches!(
        try_lay(&doc(vec![s])),
        Err(FolioError::Invalid(_))
    ));
}

// REQ: FOLIO-REQ-059
#[test]
fn links_and_anchors_become_annotations_and_named_destinations() {
    let d = small(vec![
        para_in(vec![
            Inline::Anchor("start".into()),
            Inline::Text("see ".into()),
            Inline::Link {
                target: "#end".into(),
                text: "the end".into(),
            },
            Inline::Link {
                target: "https://example.org/a".into(),
                text: "site".into(),
            },
        ]),
        Block::Break(BreakKind::Page),
        para_in(vec![
            Inline::Anchor("end".into()),
            Inline::Text("End".into()),
        ]),
    ]);
    let t = lay(&d);
    assert_eq!(t.dests.len(), 2);
    assert_eq!((t.dests[0].name.as_str(), t.dests[0].page), ("start", 0));
    assert_eq!((t.dests[1].name.as_str(), t.dests[1].page), ("end", 1));
    // The destination is the top of the line: the content-box top here.
    assert_eq!(t.dests[0].y.raw(), pts(280).raw());
    let l = &t.pages[0].links;
    assert_eq!(l.len(), 2);
    assert_eq!(l[0].target, LinkTarget::Named("end".into()));
    assert_eq!(l[1].target, LinkTarget::Uri("https://example.org/a".into()));
    assert_eq!(l[0].node.path(), &[0, 0, 0, 2]);
    // The annotation rectangle matches the link text item.
    let it = t.pages[0]
        .items
        .iter()
        .find(|i| i.role == StructRole::Link)
        .unwrap();
    let ItemKind::Text(x) = &it.kind else {
        panic!()
    };
    assert_eq!(l[0].rect.x, it.x);
    assert_eq!(l[0].rect.w, x.width);
    assert_eq!(l[0].rect.y.raw(), it.y.raw() - x.descent.raw());
    assert_eq!(l[0].rect.h.raw(), x.ascent.raw() + x.descent.raw());
}

// REQ: FOLIO-REQ-059
#[test]
fn unknown_empty_and_duplicate_anchors_are_rejected() {
    let link = |t: &str| {
        para_in(vec![Inline::Link {
            target: t.into(),
            text: "x".into(),
        }])
    };
    let anchor = |n: &str| para_in(vec![Inline::Anchor(n.into())]);
    assert_eq!(
        try_lay(&small(vec![link("#nowhere")])).unwrap_err(),
        FolioError::Invalid("link to unknown anchor")
    );
    assert_eq!(
        try_lay(&small(vec![link("#")])).unwrap_err(),
        FolioError::Invalid("link to an empty anchor name")
    );
    assert_eq!(
        try_lay(&small(vec![anchor("a"), anchor("a")])).unwrap_err(),
        FolioError::Invalid("duplicate anchor name")
    );
    assert_eq!(
        try_lay(&small(vec![anchor("")])).unwrap_err(),
        FolioError::Invalid("anchor name is empty")
    );
}

// REQ: FOLIO-REQ-059
#[test]
fn every_item_names_its_ir_node_and_structure_role() {
    let mut s = section(
        setup(200, 300, 20),
        vec![
            heading(2, "Title"),
            para("body"),
            list(false, &["item"]),
            table(1, 0, vec![row(vec![cell("c")])]),
        ],
    );
    s.header = vec![para("h")];
    s.footer = vec![para("f")];
    let t = lay(&doc(vec![s]));
    let mut seen = Vec::new();
    for i in &t.pages[0].items {
        let kind = if matches!(i.kind, ItemKind::Rect(_)) {
            "rect"
        } else {
            "text"
        };
        seen.push((i.node.path().to_vec(), i.role, kind));
    }
    let has =
        |p: &[u32], r: StructRole, k: &str| seen.iter().any(|s| s.0 == p && s.1 == r && s.2 == k);
    assert!(has(&[0, 1, 0, 0], StructRole::Header, "text"));
    assert!(has(&[0, 2, 0, 0], StructRole::Footer, "text"));
    assert!(has(&[0, 0, 0, 0], StructRole::Heading(2), "text"));
    assert!(has(&[0, 0, 1, 0], StructRole::Paragraph, "text"));
    assert!(has(&[0, 0, 2, 0], StructRole::ListItem, "text"));
    assert!(has(&[0, 0, 2, 0, 0], StructRole::ListItem, "text"));
    assert!(has(&[0, 0, 3, 0, 0], StructRole::TableCell, "text"));
    assert!(has(&[0, 0, 3, 0, 0], StructRole::Artifact, "rect"));
    assert_eq!(NodeId::block(0, Area::Body, 3).path(), &[0, 0, 3]);
    assert_eq!(NodeId::block(0, Area::Body, 3).parent(), None);
}

// REQ: FOLIO-REQ-008
#[test]
fn a_missing_glyph_is_an_error_or_a_counted_substitution() {
    let d = small(vec![para("na\u{101}ve")]);
    assert_eq!(
        try_lay(&d).unwrap_err(),
        FolioError::Unsupported("character not in font")
    );
    let opts = Options {
        missing_glyph: MissingGlyph::Notdef,
        ..Options::default()
    };
    let t = layout(&d, &Fonts::std14(), &Theme::standard(), &opts).unwrap();
    assert_eq!(t.substituted, 1);
    assert_eq!(texts(&t), ["na\u{101}ve"]);
}

// REQ: FOLIO-REQ-061
#[test]
fn unsupported_input_and_bad_themes_are_rejected() {
    let img = Block::Image(Image {
        alt_text: "x".into(),
        format: ImageFormat::Png,
        data: vec![1],
        width: pts(10),
        height: pts(10),
    });
    assert!(matches!(
        try_lay(&small(vec![img])),
        Err(FolioError::Unsupported(_))
    ));
    assert_eq!(
        try_lay(&small(vec![para_in(vec![Inline::FieldRef(
            FieldKind::Toc
        )])]))
        .unwrap_err(),
        FolioError::Unsupported("table of contents inside a paragraph")
    );
    let mut wb = small(vec![]);
    wb.body = Body::Workbook(vec![]);
    assert!(try_lay(&wb).is_err());
    let mut th = Theme::standard();
    th.body_size = Emu::ZERO;
    let r = layout(
        &small(vec![para("x")]),
        &Fonts::std14(),
        &th,
        &Options::default(),
    );
    assert_eq!(
        r.unwrap_err(),
        FolioError::Invalid("theme value out of range")
    );
    let mut bad = small(vec![para("x")]);
    if let Body::Flow(s) = &mut bad.body {
        s[0].page.margin = pts(500);
    }
    assert!(matches!(try_lay(&bad), Err(FolioError::Invalid(_))));
    let page_break_in_header = {
        let mut s = section(setup(200, 300, 20), vec![para("x")]);
        s.header = vec![Block::Break(BreakKind::Page)];
        doc(vec![s])
    };
    assert!(matches!(
        try_lay(&page_break_in_header),
        Err(FolioError::Unsupported(_))
    ));
}

// REQ: FOLIO-REQ-061
#[test]
fn page_and_item_limits_are_enforced_at_and_over_the_limit() {
    let d = small(paras(30, 60));
    let pages = lay(&d).pages.len();
    let run = |max_pages: usize| {
        let opts = Options {
            limits: Limits::default().with_max_zip_entries(max_pages),
            ..Options::default()
        };
        layout(&d, &Fonts::std14(), &Theme::standard(), &opts)
    };
    assert_eq!(run(pages).unwrap().pages.len(), pages);
    assert_eq!(run(pages - 1).unwrap_err(), FolioError::Limit("pages"));
    assert_eq!(run(0).unwrap_err(), FolioError::Limit("pages"));
    let items: usize = lay(&d).pages.iter().map(|p| p.items.len()).sum();
    let run = |cells: usize| {
        let opts = Options {
            limits: Limits::default().with_max_cells(cells),
            ..Options::default()
        };
        layout(&d, &Fonts::std14(), &Theme::standard(), &opts)
    };
    assert!(run(items).is_ok());
    // One item short: either the line budget or the item budget runs out first.
    assert!(matches!(
        run(items - 1).unwrap_err(),
        FolioError::Limit("layout items" | "layout lines")
    ));
}

// REQ: FOLIO-REQ-061
#[test]
fn unbreakable_content_that_cannot_fit_is_an_error_not_an_overflow() {
    let mut th = Theme::standard();
    th.body_size = pts(200);
    let r = layout(
        &small(vec![para("W")]),
        &Fonts::std14(),
        &th,
        &Options::default(),
    );
    assert_eq!(
        r.unwrap_err(),
        FolioError::Unsupported("glyph wider than its line")
    );
    let mut th = Theme::standard();
    th.body_size = pts(250);
    let r = layout(
        &small(vec![para(".")]),
        &Fonts::std14(),
        &th,
        &Options::default(),
    );
    assert_eq!(
        r.unwrap_err(),
        FolioError::Unsupported("content taller than the page body")
    );
}

// REQ: FOLIO-REQ-055
#[test]
fn trailing_spaces_and_terminators_do_not_widen_a_line() {
    let t = lay(&small(vec![para("one \r\ntwo\u{2028}three\tfour   ")]));
    assert_eq!(texts(&t), ["one", "two", "three four"]);
}

// REQ: FOLIO-REQ-055
#[test]
fn empty_paragraphs_and_line_breaks_take_one_line_of_height() {
    let t = lay(&small(vec![
        para("a"),
        para(""),
        Block::Break(BreakKind::Line),
        para("b"),
    ]));
    let ys: Vec<i64> = page_texts(&t, 0).iter().map(|x| x.1).collect();
    let line = Emu::from_ratio(pts(11).raw(), 120, 100).unwrap().raw();
    let gap = pts(6).raw();
    // a, empty paragraph, line break, b: three lines and two paragraph gaps between the texts.
    assert_eq!(ys[0] - ys[1], 3 * line + 2 * gap);
}

fn liberation() -> Option<Vec<u8>> {
    let p = "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf";
    match std::fs::read(p) {
        Ok(d) => Some(d),
        Err(_) if std::env::var("FOLIO_ALLOW_MISSING_FONT").is_ok_and(|v| v == "1") => None,
        Err(e) => panic!("{p}: {e}; install the font or set FOLIO_ALLOW_MISSING_FONT=1"),
    }
}

// REQ: FOLIO-REQ-055
#[test]
fn truetype_faces_are_measured_with_their_own_advances() {
    let Some(data) = liberation() else { return };
    let limits = Limits::default();
    let f = Font::parse(&data, &limits).unwrap();
    let fonts = Fonts::new(Face::TrueType(f.clone()), Face::TrueType(f.clone()));
    let d = small(vec![para(&words_of(1, 30)), para("caf\u{e9} \u{20ac}")]);
    let t = layout(&d, &fonts, &Theme::standard(), &Options::default()).unwrap();
    let per_em = i64::from(f.units_per_em());
    for it in &t.pages[0].items {
        let ItemKind::Text(x) = &it.kind else {
            continue;
        };
        let want: i64 = x
            .text
            .chars()
            .map(|c| {
                let adv = i64::from(f.advance(f.glyph_for(c).unwrap()).unwrap());
                Emu::from_ratio(adv, x.size.raw(), per_em).unwrap().raw()
            })
            .sum();
        assert_eq!(x.width.raw(), want, "{:?}", x.text);
    }
    assert!(texts(&t).contains(&"caf\u{e9} \u{20ac}".to_string()));
    check_invariants(&t);
}

// REQ: FOLIO-REQ-008
#[test]
fn truetype_missing_glyphs_follow_the_policy() {
    let Some(data) = liberation() else { return };
    let f = Font::parse(&data, &Limits::default()).unwrap();
    let fonts = Fonts::new(Face::TrueType(f.clone()), Face::TrueType(f));
    let d = small(vec![para("a\u{4e00}b")]);
    let r = layout(&d, &fonts, &Theme::standard(), &Options::default());
    assert_eq!(
        r.unwrap_err(),
        FolioError::Unsupported("character not in font")
    );
    let opts = Options {
        missing_glyph: MissingGlyph::Notdef,
        ..Options::default()
    };
    assert_eq!(
        layout(&d, &fonts, &Theme::standard(), &opts)
            .unwrap()
            .substituted,
        1
    );
}

// REQ: FOLIO-REQ-067
#[test]
fn the_tree_records_the_kind_of_every_body_block_for_structure() {
    use moonshot_folio_ir::{Heading, Inline, Pagination};
    use moonshot_folio_layout::{dump, BlockKind};
    let link_heading = Block::Heading(Heading {
        level: 3,
        inlines: vec![Inline::Link {
            target: "https://example.org/x".into(),
            text: "Only a link".into(),
        }],
        pagination: Pagination::default(),
    });
    let mut s = section(
        setup(200, 300, 20),
        vec![
            heading(1, "H"),
            para("p"),
            list(true, &["a", "b"]),
            list(false, &["c"]),
            table(
                3,
                1,
                vec![
                    row(vec![cell("a"), cell("b"), cell("c")]),
                    row(vec![span("wide", 2), cell("d")]),
                ],
            ),
            link_heading,
            Block::Field(FieldKind::Toc),
            Block::Field(FieldKind::Page),
        ],
    );
    s.header = vec![heading(1, "Running"), para("x")];
    let t = lay(&doc(vec![s]));
    let kinds: Vec<(Vec<u32>, BlockKind)> = t
        .blocks
        .iter()
        .map(|b| (b.node.path().to_vec(), b.kind.clone()))
        .collect();
    let table = BlockKind::Table(moonshot_folio_layout::TableInfo {
        header_rows: 1,
        spans: vec![vec![1, 1, 1], vec![2, 1]],
    });
    assert_eq!(
        kinds,
        [
            (vec![0, 0, 0], BlockKind::Heading(1)),
            (vec![0, 0, 1], BlockKind::Paragraph),
            (vec![0, 0, 2], BlockKind::List { ordered: true }),
            (vec![0, 0, 3], BlockKind::List { ordered: false }),
            (vec![0, 0, 4], table),
            (vec![0, 0, 5], BlockKind::Heading(3)),
            (vec![0, 0, 6], BlockKind::Toc),
            (vec![0, 0, 7], BlockKind::Paragraph),
        ],
        "headers are not recorded; a link-only heading is a heading"
    );
    let d = dump(&t);
    assert!(d.contains("block 0.0.2 List ordered=true"));
    assert!(d.contains("block 0.0.4 Table header_rows=1 spans=[[1, 1, 1], [2, 1]]"));
}
