// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Regression and stress tests found while building the layout engine (task F5.1).

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines
)]

mod common;

use common::{
    cell, check_invariants, heading, lay, para, paras, row, small, table, texts, words_of,
};
use moonshot_folio_core::Limits;
use moonshot_folio_layout::{layout, Fonts, Options, Theme};

// REQ: FOLIO-REQ-057
#[test]
fn header_rows_too_tall_to_repeat_are_not_repeated_and_do_not_fail() {
    // Two header rows of about 150 points each on a 260 point body: they would take most of
    // every page, so continuation pages carry only body rows.
    let big = words_of(1, 90);
    let mut rows = vec![row(vec![cell(&big)]), row(vec![cell(&big)])];
    for i in 0..30 {
        rows.push(row(vec![cell(&words_of(i, 6))]));
    }
    let t = lay(&small(vec![table(1, 2, rows)]));
    assert!(t.pages.len() > 2);
    check_invariants(&t);
    // The two header rows appear once each (joined lines restore the text), never again.
    let all = texts(&t).join(" ");
    assert_eq!(all.matches(&big).count(), 2);
}

// REQ: FOLIO-REQ-061
#[test]
fn a_row_cut_into_many_slices_is_charged_once_per_placed_item() {
    let d = small(vec![table(
        1,
        0,
        vec![row(vec![cell(&words_of(3, 3000))]), row(vec![cell("end")])],
    )]);
    let t = lay(&d);
    let placed: usize = t.pages.iter().map(|p| p.items.len()).sum();
    let opts = Options {
        limits: Limits::default().with_max_cells(placed),
        ..Options::default()
    };
    assert!(layout(&d, &Fonts::std14(), &Theme::standard(), &opts).is_ok());
}

// REQ: FOLIO-REQ-056
#[test]
fn a_long_run_of_keep_with_next_headings_terminates_quickly() {
    let mut blocks: Vec<_> = (0..3000).map(|i| heading(3, &words_of(i, 2))).collect();
    blocks.push(para("end"));
    let t = lay(&small(blocks));
    assert!(t.pages.len() > 10);
    assert_eq!(texts(&t).last().unwrap(), "end");
    check_invariants(&t);
}

// REQ: FOLIO-REQ-056
#[test]
fn a_large_document_lays_out_and_page_labels_count_up() {
    let t = lay(&small(paras(1500, 45)));
    assert!(t.pages.len() > 500);
    assert!(t
        .pages
        .iter()
        .enumerate()
        .all(|(i, p)| p.label as usize == i + 1));
}
