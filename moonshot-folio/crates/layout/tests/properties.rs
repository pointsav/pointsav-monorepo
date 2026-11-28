// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Property tests: seeded random documents and mutated IR (task F5.1).

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names,
    clippy::too_many_lines
)]

mod common;

use common::{
    cell, check_invariants, doc, generate, lay, para, paras, pts, row, section, setup, table,
    try_lay, words_of, Rng,
};
use moonshot_folio_ir::{Block, Body, Document, Inline};
use moonshot_folio_layout::{dump, ItemKind, StructRole};
use std::collections::BTreeSet;

const SEEDS: u64 = 120;

// REQ: FOLIO-REQ-055
#[test]
fn no_item_leaves_the_content_box_and_no_text_overlaps() {
    for seed in 1..=SEEDS {
        let d = generate(seed, seed % 2 == 0);
        let t = try_lay(&d).unwrap_or_else(|e| panic!("seed {seed}: {e}"));
        check_invariants(&t);
        // Page labels are consecutive within a section.
        for w in t.pages.windows(2) {
            if w[0].section == w[1].section {
                assert_eq!(w[1].label, w[0].label + 1, "seed {seed}");
            }
        }
    }
}

// REQ: FOLIO-REQ-060
#[test]
fn layout_is_deterministic() {
    for seed in 1..=SEEDS {
        let d = generate(seed, true);
        let (a, b) = (dump(&lay(&d)), dump(&lay(&d.clone())));
        assert_eq!(a, b, "seed {seed}");
        assert_eq!(lay(&d), lay(&d), "seed {seed}");
    }
}

// REQ: FOLIO-REQ-056
#[test]
fn page_count_never_shrinks_when_content_is_appended() {
    for seed in 1..=SEEDS {
        let d = generate(seed, false);
        let before = lay(&d).pages.len();
        let mut more = d.clone();
        let mut r = Rng(seed | 1);
        if let Body::Flow(s) = &mut more.body {
            let last = s.last_mut().unwrap();
            for b in paras(1 + r.below(8), 20 + r.below(60)) {
                last.blocks.push(b);
            }
        }
        let after = lay(&more).pages.len();
        assert!(after >= before, "seed {seed}: {before} -> {after}");
    }
}

// REQ: FOLIO-REQ-057
#[test]
fn header_rows_are_present_on_every_continuation_page() {
    for seed in 1..=SEEDS {
        let mut r = Rng(seed.wrapping_mul(31) | 1);
        let cols = 1 + r.below(4) as u32;
        let hdr = 1 + r.below(2) as u32;
        let n = 5 + r.below(60);
        let rows: Vec<_> = (0..n)
            .map(|i| {
                row((0..cols)
                    .map(|c| cell(&words_of(i + c as usize, 1 + r.below(4))))
                    .collect())
            })
            .collect();
        let before = if r.chance(50) {
            paras(1, 10 + r.below(100))
        } else {
            vec![]
        };
        let mut blocks = before.clone();
        blocks.push(table(cols, hdr, rows));
        let d = doc(vec![section(setup(240, 320, 30), blocks)]);
        let t = lay(&d);
        check_invariants(&t);
        let tb = before.len() as u32;
        let mut seen = BTreeSet::new();
        for p in &t.pages {
            let rows_here: BTreeSet<u32> = p
                .items
                .iter()
                .filter(|i| i.role == StructRole::Artifact && i.node.path()[2] == tb)
                .map(|i| i.node.path()[3])
                .collect();
            if rows_here.iter().any(|x| *x >= hdr) {
                for h in 0..hdr {
                    assert!(
                        rows_here.contains(&h),
                        "seed {seed}: header row {h} missing"
                    );
                }
            }
            for x in rows_here.iter().filter(|x| **x >= hdr) {
                assert!(seen.insert(*x), "seed {seed}: row {x} placed twice");
            }
        }
        assert_eq!(seen.len(), n - hdr as usize, "seed {seed}: rows lost");
    }
}

fn mutate(d: &mut Document, r: &mut Rng) {
    let Body::Flow(sections) = &mut d.body else {
        return;
    };
    let si = r.below(sections.len());
    let s = &mut sections[si];
    match r.below(14) {
        0 => s.page.margin = pts(r.below(400) as i64 - 20),
        1 => s.page.width = pts(r.below(500) as i64),
        2 => s.page.height = pts(r.below(500) as i64),
        3 => s.blocks.clear(),
        4 => s
            .blocks
            .push(Block::Field(moonshot_folio_ir::FieldKind::Toc)),
        5 => s.header.push(para(&words_of(1, 300))),
        6 => s.footer.push(para(&words_of(1, 40))),
        7 => s
            .blocks
            .push(para("\u{0}\u{7}\r\n\t\u{301}\u{1F600}\u{202E}x")),
        8 => s.blocks.push(Block::Para(moonshot_folio_ir::Para {
            role: "bold".into(),
            inlines: vec![Inline::Anchor(String::new()), Inline::Text("t".into())],
            pagination: moonshot_folio_ir::Pagination::default(),
        })),
        9 => s.blocks.push(Block::Para(moonshot_folio_ir::Para {
            role: String::new(),
            inlines: vec![Inline::Link {
                target: "#missing".into(),
                text: "x".into(),
            }],
            pagination: moonshot_folio_ir::Pagination::default(),
        })),
        10 => {
            for b in &mut s.blocks {
                if let Block::Table(t) = b {
                    t.columns = [0, 1, 2, 200, u32::MAX][r.below(5)];
                    t.repeat_header_rows = r.below(40) as u32;
                }
            }
        }
        11 => {
            for b in &mut s.blocks {
                if let Block::Table(t) = b {
                    if let Some(c) = t.rows.first_mut().and_then(|x| x.cells.first_mut()) {
                        c.span = [0, 1, 7, u32::MAX][r.below(4)];
                    }
                }
            }
        }
        12 => s.different_first_page = !s.different_first_page,
        _ => {
            let t = table(
                1 + r.below(3) as u32,
                0,
                vec![row(vec![cell(&words_of(2, 400))])],
            );
            s.blocks.push(t);
        }
    }
}

// REQ: FOLIO-REQ-061
#[test]
fn mutated_ir_never_panics_and_valid_results_hold_the_invariants() {
    let mut ok = 0;
    for seed in 1..=SEEDS * 2 {
        let mut d = generate(seed, seed % 3 == 0);
        let mut r = Rng(seed.wrapping_mul(77) | 1);
        for _ in 0..=r.below(2) {
            mutate(&mut d, &mut r);
        }
        if let Ok(t) = try_lay(&d) {
            check_invariants(&t);
            ok += 1;
        }
    }
    assert!(ok > 50, "mutants should still often be laid out ({ok})");
}

// REQ: FOLIO-REQ-061
#[test]
fn text_made_of_awkward_characters_is_laid_out_or_rejected_cleanly() {
    let nasty = [
        "\u{301}\u{301}\u{301}",
        "\u{1F1E6}\u{1F1E7}\u{1F1E8}",
        "a\u{200B}b\u{200D}c",
        "\r\r\n\n\u{2029}",
        "      ",
        "e\u{301}e\u{301}e\u{301}",
        "\u{3042}\u{3044}\u{3046}",
    ];
    for s in nasty {
        let d = doc(vec![section(setup(200, 300, 20), vec![para(s)])]);
        if let Ok(t) = try_lay(&d) {
            check_invariants(&t);
            assert!(t
                .pages
                .iter()
                .flat_map(|p| &p.items)
                .all(|i| match &i.kind {
                    ItemKind::Text(x) => !x.text.ends_with(' ') && !x.text.contains('\n'),
                    ItemKind::Rect(_) => true,
                }));
        }
    }
}
