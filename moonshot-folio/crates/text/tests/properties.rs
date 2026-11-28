// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Structural properties of `line_breaks` and robustness against arbitrary input.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::expect_used
)]

use moonshot_folio_text::{line_breaks, wrap, BreakKind};

/// Deterministic pseudo-random generator (64-bit LCG, high bits).
struct Rng(u64);

impl Rng {
    fn next(&mut self, n: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        usize::try_from((self.0 >> 33) % u64::try_from(n).unwrap()).unwrap()
    }
}

const POOL: &[char] = &[
    'a',
    'b',
    'Z',
    '0',
    '5',
    ' ',
    ' ',
    '\n',
    '\r',
    '-',
    '(',
    ')',
    '[',
    ']',
    '{',
    '}',
    '!',
    '?',
    ',',
    '.',
    ':',
    '/',
    '"',
    '\'',
    '$',
    '%',
    '\u{a0}',
    '\u{200b}',
    '\u{200d}',
    '\u{300}',
    '\u{2010}',
    '\u{2014}',
    '\u{3042}',
    '\u{4e00}',
    '\u{3001}',
    '\u{ac00}',
    '\u{1100}',
    '\u{1161}',
    '\u{11a8}',
    '\u{5d0}',
    '\u{1f1e6}',
    '\u{1f1e7}',
    '\u{1f466}',
    '\u{1f3fb}',
    '\u{1f600}',
    '\u{201c}',
    '\u{201d}',
    '\u{e01}',
    '\u{85}',
    '\u{b}',
    '\u{fffc}',
    '\u{2060}',
    '\u{378}',
];

fn random_text(rng: &mut Rng, max_len: usize) -> String {
    let n = rng.next(max_len);
    (0..n).map(|_| POOL[rng.next(POOL.len())]).collect()
}

#[test]
// REQ: FOLIO-REQ-050
fn offsets_are_ordered_in_range_on_boundaries_and_end_at_text_end() {
    let mut rng = Rng(7);
    for _ in 0..20_000 {
        let s = random_text(&mut rng, 24);
        let br = line_breaks(&s);
        let mut prev = 0;
        for b in &br {
            assert!(b.offset > prev, "{s:?}: not strictly increasing");
            assert!(b.offset <= s.len() && s.is_char_boundary(b.offset), "{s:?}");
            prev = b.offset;
        }
        if s.is_empty() {
            assert!(br.is_empty());
        } else {
            let last = br.last().unwrap();
            assert_eq!(last.offset, s.len(), "{s:?}: final break at end of text");
            let hard = matches!(
                s.chars().last(),
                Some('\n' | '\r' | '\u{b}' | '\u{c}' | '\u{85}' | '\u{2028}' | '\u{2029}')
            );
            assert_eq!(last.kind == BreakKind::Mandatory, hard, "{s:?}");
        }
    }
}

#[test]
// REQ: FOLIO-REQ-050
fn hard_breaks_are_mandatory_and_crlf_is_never_split() {
    let mut rng = Rng(11);
    for _ in 0..20_000 {
        let s = random_text(&mut rng, 24);
        let br = line_breaks(&s);
        let chars: Vec<(usize, char)> = s.char_indices().collect();
        for w in chars.windows(2) {
            let (po, pc) = w[0];
            let (co, cc) = w[1];
            let found = br.iter().find(|b| b.offset == co);
            if pc == '\r' && cc == '\n' {
                assert!(found.is_none(), "{s:?}: break inside CRLF at {po}");
            } else if matches!(
                pc,
                '\n' | '\r' | '\u{b}' | '\u{c}' | '\u{85}' | '\u{2028}' | '\u{2029}'
            ) {
                assert_eq!(
                    found.map(|b| b.kind),
                    Some(BreakKind::Mandatory),
                    "{s:?} after {pc:?}"
                );
            } else {
                assert_ne!(found.map(|b| b.kind), Some(BreakKind::Mandatory), "{s:?}");
            }
        }
    }
}

#[test]
// REQ: FOLIO-REQ-050
fn never_breaks_before_closing_punctuation_or_between_ascii_letters() {
    let mut rng = Rng(23);
    for _ in 0..30_000 {
        let s = random_text(&mut rng, 24);
        let br = line_breaks(&s);
        let chars: Vec<(usize, char)> = s.char_indices().collect();
        for (i, w) in chars.windows(2).enumerate() {
            let (_, pc) = w[0];
            let (co, cc) = w[1];
            let breaks_here = br.iter().any(|b| b.offset == co);
            // LB6 and LB8: a hard break, or ZW followed by spaces, forces a break first.
            let skipped = chars[..=i].iter().rev().find(|e| e.1 != ' ').map(|e| e.1);
            let forced =
                matches!(pc, '\n' | '\r' | '\u{b}' | '\u{85}') || skipped == Some('\u{200b}');
            if matches!(cc, ')' | ']' | '}' | '!' | '?') && !forced {
                assert!(!breaks_here, "{s:?}: break before {cc:?}");
            }
            if pc.is_ascii_alphabetic() && cc.is_ascii_alphabetic() {
                assert!(!breaks_here, "{s:?}: break between letters");
            }
        }
    }
}

#[test]
// REQ: FOLIO-REQ-050
fn spaces_break_after_not_before() {
    let br = line_breaks("ab cd  ef");
    let offs: Vec<usize> = br.iter().map(|b| b.offset).collect();
    assert_eq!(offs, vec![3, 7, 9]);
    assert!(br.iter().all(|b| b.kind == BreakKind::Allowed));
}

#[test]
// REQ: FOLIO-REQ-051
fn every_scalar_value_and_arbitrary_bytes_never_panic() {
    let mut chunk = String::new();
    let mut in_chunk = 0_u32;
    for cp in 0..=0x10_FFFF_u32 {
        let Some(c) = char::from_u32(cp) else {
            continue;
        };
        let single = c.to_string();
        let br = line_breaks(&single);
        assert_eq!(br.len(), 1, "U+{cp:04X}");
        chunk.push(c);
        in_chunk += 1;
        if in_chunk == 97 {
            in_chunk = 0;
            let _ = line_breaks(&chunk);
            chunk.clear();
        }
    }
    let _ = line_breaks(&chunk);
    let mut rng = Rng(99);
    for _ in 0..5_000 {
        let n = rng.next(64);
        let bytes: Vec<u8> = (0..n)
            .map(|_| u8::try_from(rng.next(256)).unwrap())
            .collect();
        let s = String::from_utf8_lossy(&bytes).into_owned();
        let br = line_breaks(&s);
        let w: Vec<i64> = (0..n)
            .map(|_| i64::try_from(rng.next(9)).unwrap() - 2)
            .collect();
        let _ = wrap(&s, &w, i64::try_from(rng.next(20)).unwrap() - 3, &br);
    }
    // Mutated real text: delete or duplicate characters and re-run.
    let base = "He said, \u{201c}Pay $(1,234.50) now\u{201d}\r\n\u{65e5}\u{672c}\u{8a9e} \u{1f468}\u{200d}\u{1f469} \u{1f1fa}\u{1f1f8}\u{1f1e9}\u{1f1ea}";
    let cs: Vec<char> = base.chars().collect();
    for i in 0..cs.len() {
        let mut m: Vec<char> = cs.clone();
        m.remove(i);
        let _ = line_breaks(&m.iter().collect::<String>());
        m.insert(i, cs[(i * 7) % cs.len()]);
        let _ = line_breaks(&m.iter().collect::<String>());
    }
}

#[test]
// REQ: FOLIO-REQ-051
fn long_runs_do_not_degrade() {
    let s: String = "\u{300}".repeat(200_000);
    assert_eq!(line_breaks(&s).len(), 1);
    let s: String = "\u{1f1e6}".repeat(100_001);
    assert_eq!(line_breaks(&s).len(), 50_001);
}
