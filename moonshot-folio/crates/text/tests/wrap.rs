// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Behaviour of the greedy line wrapper.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::expect_used
)]

use moonshot_folio_text::{line_breaks, wrap};

fn run(text: &str, max: i64) -> Vec<&str> {
    let widths = vec![1_i64; text.chars().count()];
    split(text, &wrap(text, &widths, max, &line_breaks(text)))
}

fn split<'a>(text: &'a str, ends: &[usize]) -> Vec<&'a str> {
    let mut start = 0;
    ends.iter()
        .map(|&e| {
            let l = &text[start..e];
            start = e;
            l
        })
        .collect()
}

#[test]
// REQ: FOLIO-REQ-052
fn wraps_at_word_boundaries() {
    assert_eq!(run("aaa bbb ccc", 7), vec!["aaa bbb ", "ccc"]);
    assert_eq!(run("aaa bbb ccc", 8), vec!["aaa bbb ", "ccc"]);
    assert_eq!(run("aaa bbb ccc", 11), vec!["aaa bbb ccc"]);
    assert_eq!(run("aaa bbb ccc", 3), vec!["aaa ", "bbb ", "ccc"]);
}

#[test]
// REQ: FOLIO-REQ-052
fn trailing_spaces_do_not_count() {
    assert_eq!(run("aaaa     bbbb", 4), vec!["aaaa     ", "bbbb"]);
}

#[test]
// REQ: FOLIO-REQ-052
fn overlong_word_is_cut_at_a_char_boundary() {
    assert_eq!(run("abcdefghij", 4), vec!["abcd", "efgh", "ij"]);
    assert_eq!(run("ab abcdefgh", 4), vec!["ab ", "abcd", "efgh"]);
    assert_eq!(
        run("h\u{e9}llo w\u{f6}rld", 3),
        vec!["h\u{e9}l", "lo ", "w\u{f6}r", "ld"]
    );
}

#[test]
// REQ: FOLIO-REQ-052
fn mandatory_breaks_always_end_a_line() {
    assert_eq!(run("ab\ncd", 99), vec!["ab\n", "cd"]);
    assert_eq!(run("ab\r\n\r\ncd\n", 99), vec!["ab\r\n", "\r\n", "cd\n"]);
}

#[test]
// REQ: FOLIO-REQ-052
fn degenerate_inputs_make_progress() {
    assert!(run("", 5).is_empty());
    assert_eq!(run("abc", 0), vec!["a", "b", "c"]);
    assert_eq!(run("abc", -5), vec!["a", "b", "c"]);
    let text = "wide";
    let ends = wrap(
        text,
        &[i64::MAX, i64::MAX, i64::MAX, i64::MAX],
        i64::MAX,
        &line_breaks(text),
    );
    assert_eq!(ends.last(), Some(&4));
    // Missing and negative widths count as zero.
    assert_eq!(wrap("abc def", &[], 1, &line_breaks("abc def")), vec![7]);
    assert_eq!(wrap("ab", &[-9, -9], 1, &line_breaks("ab")), vec![2]);
}

#[test]
// REQ: FOLIO-REQ-052
fn uneven_widths_and_line_invariants_hold() {
    let text = "The quick brown fox, jumping over 3,000.00 lazy dogs\nand \u{65e5}\u{672c}\u{8a9e}\u{306e}\u{30c6}\u{30ad}\u{30b9}\u{30c8}.";
    let widths: Vec<i64> = text
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c == ' ' {
                2
            } else {
                3 + i64::try_from(i % 4).unwrap()
            }
        })
        .collect();
    for max in [1_i64, 5, 12, 30, 60, 1000] {
        let ends = wrap(text, &widths, max, &line_breaks(text));
        assert_eq!(ends.last(), Some(&text.len()));
        let mut start = 0;
        for &e in &ends {
            assert!(e > start && text.is_char_boundary(e));
            let line = &text[start..e];
            let first = text[..start].chars().count();
            let body = line.trim_end_matches([' ', '\n', '\r']);
            let n = body.chars().count();
            let w: i64 = widths[first..first + n].iter().sum();
            assert!(w <= max || n == 1, "line {line:?} width {w} > {max}");
            start = e;
        }
    }
}
