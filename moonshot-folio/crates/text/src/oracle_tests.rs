// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Differential test against ICU 74.2 (`PyICU`), corpus from `tests/gen/make_corpus.py`.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::expect_used,
    clippy::many_single_char_names
)]

use crate::class::{class_of, Lb};
use crate::line_breaks;

const DATA: &str = include_str!("../tests/data/icu_line_breaks.txt");

/// Break positions of `s` as `char` indices (end of text included), like the oracle.
fn mine(s: &str) -> Vec<usize> {
    let idx: Vec<usize> = s.char_indices().map(|e| e.0).chain([s.len()]).collect();
    line_breaks(s)
        .iter()
        .map(|b| idx.iter().position(|&o| o == b.offset).unwrap())
        .collect()
}

const LB25_PAIRS: [(Lb, Lb); 14] = [
    (Lb::CL, Lb::PO),
    (Lb::CP, Lb::PO),
    (Lb::CL, Lb::PR),
    (Lb::CP, Lb::PR),
    (Lb::NU, Lb::PO),
    (Lb::NU, Lb::PR),
    (Lb::PO, Lb::OP),
    (Lb::PO, Lb::NU),
    (Lb::PR, Lb::OP),
    (Lb::PR, Lb::NU),
    (Lb::HY, Lb::NU),
    (Lb::IS, Lb::NU),
    (Lb::NU, Lb::NU),
    (Lb::SY, Lb::NU),
];

/// Index of the cluster base before position `k` (skips CM and ZWJ).
fn base_before(cls: &[Lb], k: usize) -> usize {
    let mut j = k - 1;
    while j > 0 && matches!(cls[j], Lb::CM | Lb::ZWJ) {
        j -= 1;
    }
    j
}

/// Why a difference at char index `k` is a documented, accepted one (None: a real bug).
fn accepted(chars: &[char], k: usize, icu_breaks_here: bool) -> Option<&'static str> {
    let cls: Vec<Lb> = chars
        .iter()
        .map(|&c| class_of(c).resolve().base())
        .collect();
    let j = base_before(&cls, k);
    let (p, c) = (cls[j], cls[k]);
    if icu_breaks_here && LB25_PAIRS.contains(&(p, c)) {
        // UAX#14 LB25 lists these pairs unconditionally; ICU applies the tailored number
        // regex instead, which keeps them together only inside a complete number.
        return Some("LB25: ICU uses the tailored number regex");
    }
    if !icu_breaks_here && matches!(p, Lb::IS | Lb::SY) && matches!(c, Lb::PO | Lb::PR) {
        let mut i = j;
        while i > 0 && matches!(cls[i], Lb::IS | Lb::SY) {
            i -= 1;
        }
        if cls[i] == Lb::NU {
            return Some("LB25: ICU's number regex NU (NU|SY|IS)* (PR|PO) keeps the suffix");
        }
    }
    let hyphen = p == Lb::HY || (p == Lb::BA && chars[j] == '\u{2010}');
    if icu_breaks_here && hyphen && c == Lb::AL && j > 0 && cls[base_before(&cls, j)] == Lb::GL {
        return Some("LB20a: ICU does not treat GL as a word-start context for a hyphen");
    }
    None
}

#[test]
// REQ: FOLIO-REQ-042
fn matches_icu_modulo_documented_differences() {
    let (mut total, mut skipped) = (0_usize, 0_usize);
    let mut bad = Vec::new();
    for line in DATA.lines() {
        let (left, right) = line.split_once('|').unwrap();
        let chars: Vec<char> = left
            .split(' ')
            .map(|h| char::from_u32(u32::from_str_radix(h, 16).unwrap()).unwrap())
            .collect();
        let want: Vec<usize> = right
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let text: String = chars.iter().collect();
        let got = mine(&text);
        total += 1;
        if got == want {
            continue;
        }
        let mut ok = true;
        for k in 1..chars.len() {
            let (icu_has, mine_has) = (want.contains(&k), got.contains(&k));
            if icu_has != mine_has && accepted(&chars, k, icu_has).is_none() {
                ok = false;
            }
        }
        if ok {
            skipped += 1;
        } else {
            bad.push(format!("{left}: icu {want:?} mine {got:?}"));
        }
    }
    assert!(total > 80_000, "corpus too small: {total}");
    assert!(
        bad.is_empty(),
        "{} unexplained differences, first: {:?}",
        bad.len(),
        bad.iter().take(5).collect::<Vec<_>>()
    );
    assert!(
        skipped < total / 50,
        "accepted differences unexpectedly frequent: {skipped}"
    );
}

#[test]
// REQ: FOLIO-REQ-043
fn class_table_spot_checks() {
    let checks = [
        ('a', Lb::AL),
        (' ', Lb::SP),
        ('\u{2d}', Lb::HY),
        ('\u{201c}', Lb::QU_PI),
        ('\u{201d}', Lb::QU_PF),
        ('\u{28}', Lb::OP),
        ('\u{3008}', Lb::OP_EA),
        ('\u{29}', Lb::CP),
        ('\u{1f466}', Lb::EB),
        ('\u{1f1e6}', Lb::RI),
        ('\u{ac00}', Lb::H2),
        ('\u{200d}', Lb::ZWJ),
        ('\u{e01}', Lb::AL),
        ('\u{e31}', Lb::CM),
        ('\u{10ffff}', Lb::XX),
    ];
    for (c, want) in checks {
        assert_eq!(class_of(c), want, "U+{:04X}", u32::from(c));
    }
}
