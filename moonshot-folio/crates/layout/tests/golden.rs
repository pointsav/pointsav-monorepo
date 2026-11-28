// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Golden corpus group `layout/`: synthetic documents laid out and compared with the stored
//! `PageTree` dumps in `assurance/golden/layout/`. Regenerate with
//! `FOLIO_BLESS=1 cargo test -p moonshot-folio-layout --test golden` and review the diff. All
//! content is synthetic.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::too_many_lines,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation
)]

mod common;

use common::{
    cell, doc, heading, lay, list, num, para, para_in, para_with, paras, row, section, setup,
    small, span, table, text, words_of,
};
use moonshot_folio_ir::{Block, BreakKind, Document, FieldKind, Inline, Pagination};
use moonshot_folio_layout::dump;
use std::path::PathBuf;

type Scenario = (&'static str, Document);

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assurance/golden/layout")
}

fn flow(n: usize) -> Vec<Block> {
    let mut b = Vec::new();
    for i in 0..n {
        b.push(heading(1 + (i % 3) as u8, &words_of(i, 3)));
        b.push(para(&words_of(i, 60)));
    }
    b
}

fn rows(n: usize) -> Vec<moonshot_folio_ir::Row> {
    (0..n)
        .map(|i| {
            row(vec![
                cell(&words_of(i, 2)),
                num(&format!("{}.50", i * 7)),
                cell("ok"),
            ])
        })
        .collect()
}

fn scenarios() -> Vec<Scenario> {
    let mut out: Vec<Scenario> = Vec::new();
    out.push(("01-paragraphs", small(paras(6, 40))));
    out.push(("02-headings-keep-with-next", small(flow(6))));
    out.push((
        "03-lists",
        small(vec![
            para("Steps"),
            list(false, &["first item", &words_of(1, 30), "third"]),
            list(
                true,
                &[
                    "one",
                    "two",
                    &words_of(2, 25),
                    "four",
                    "five",
                    "six",
                    "seven",
                    "eight",
                    "nine",
                    "ten",
                    "eleven",
                ],
            ),
        ]),
    ));
    let mut t = vec![row(vec![cell("Item"), cell("Amount"), cell("State")])];
    t.extend(rows(26));
    out.push(("04-table-split-repeat-header", small(vec![table(3, 1, t)])));
    out.push((
        "05-table-colspan",
        small(vec![table(
            3,
            1,
            vec![
                row(vec![span("Statement", 2), cell("2026")]),
                row(vec![cell("Cash"), num("100.00"), num("-12.5")]),
                row(vec![span(&words_of(3, 12), 3)]),
            ],
        )]),
    ));
    let tall = words_of(4, 150);
    out.push((
        "06-oversize-row",
        small(vec![table(
            2,
            1,
            vec![
                row(vec![cell("Key"), cell("Notes")]),
                row(vec![cell("a"), cell(&tall)]),
                row(vec![cell("b"), cell("short")]),
            ],
        )]),
    ));
    let mut s = section(setup(200, 300, 20), flow(5));
    s.header = vec![para_in(vec![Inline::Text("Report".into())])];
    s.footer = vec![para_in(vec![
        Inline::Text("Page ".into()),
        Inline::FieldRef(FieldKind::Page),
        Inline::Text(" of ".into()),
        Inline::FieldRef(FieldKind::NumPages),
    ])];
    s.different_first_page = true;
    out.push(("07-header-footer-fields", doc(vec![s])));
    let mut a = section(setup(200, 300, 20), paras(3, 50));
    a.footer = vec![para_in(vec![Inline::FieldRef(FieldKind::Page)])];
    let mut b = section(setup(300, 200, 30), paras(3, 50));
    b.footer.clone_from(&a.footer);
    b.restart_page_numbers = true;
    out.push(("08-sections-geometry-restart", doc(vec![a, b])));
    out.push((
        "09-links-anchors",
        small(vec![
            para_in(vec![
                Inline::Anchor("start".into()),
                Inline::Text("Go to ".into()),
                Inline::Link {
                    target: "#end".into(),
                    text: "the end".into(),
                },
                Inline::Text(" or visit ".into()),
                Inline::Link {
                    target: "https://example.org/a".into(),
                    text: "example".into(),
                },
                Inline::Highlight(" marked".into()),
            ]),
            Block::Break(BreakKind::Page),
            para_in(vec![
                Inline::Anchor("end".into()),
                Inline::Text("End".into()),
            ]),
        ]),
    ));
    let wo = Pagination {
        widow_orphan: true,
        ..Pagination::default()
    };
    let mut blocks = paras(2, 70);
    blocks.push(para_with(&words_of(5, 70), wo));
    blocks.push(para_with(&words_of(6, 70), wo));
    out.push(("10-widow-orphan", small(blocks)));
    out.push((
        "11-breaks-and-fields",
        small(vec![
            para("before"),
            Block::Break(BreakKind::Line),
            Block::Field(FieldKind::Page),
            Block::Break(BreakKind::Page),
            Block::Break(BreakKind::Page),
            heading(2, "After the break"),
            para(&format!("tab\there\r\nnext line {}", text("x").len())),
        ]),
    ));
    let mut fields = Vec::new();
    for _ in 0..11 {
        fields.push(para_in(vec![
            Inline::Text("on page ".into()),
            Inline::FieldRef(FieldKind::Page),
            Inline::Text(" end".into()),
        ]));
        fields.push(Block::Break(BreakKind::Page));
    }
    out.push(("12-body-page-field-two-digits", small(fields)));
    let empty = || section(setup(200, 300, 20), vec![]);
    let with_break = section(
        setup(200, 300, 20),
        vec![para("first"), Block::Break(BreakKind::Page)],
    );
    let last = section(setup(200, 300, 20), vec![para("last")]);
    out.push((
        "13-trailing-break-and-empty-sections",
        doc(vec![empty(), with_break, empty(), last, empty()]),
    ));
    out
}

// REQ: FOLIO-REQ-060
#[test]
fn layout_dumps_match_the_golden_corpus() {
    let bless = std::env::var_os("FOLIO_BLESS").is_some();
    std::fs::create_dir_all(dir()).unwrap();
    for (name, d) in scenarios() {
        let got = dump(&lay(&d));
        let path = dir().join(format!("{name}.layout.txt"));
        if bless {
            std::fs::write(&path, &got).unwrap();
            continue;
        }
        let want = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing golden {name}; run with FOLIO_BLESS=1"));
        assert_eq!(got, want, "golden mismatch: {name}");
    }
}

// REQ: FOLIO-REQ-060
#[test]
fn the_corpus_has_no_stray_files() {
    let names: Vec<String> = scenarios()
        .iter()
        .map(|(n, _)| format!("{n}.layout.txt"))
        .collect();
    if let Ok(rd) = std::fs::read_dir(dir()) {
        for e in rd {
            let f = e.unwrap().file_name().to_string_lossy().to_string();
            assert!(names.contains(&f), "stray golden file {f}");
        }
    }
}
