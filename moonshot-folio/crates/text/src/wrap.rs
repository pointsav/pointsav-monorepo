// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Greedy line wrapping over UAX#14 break opportunities.

use crate::breaks::{Break, BreakKind};

/// Characters whose advance never counts toward line fit at the end of a line: the space and
/// the line terminators.
fn hangs(c: char) -> bool {
    matches!(
        c,
        ' ' | '\n' | '\r' | '\u{b}' | '\u{c}' | '\u{85}' | '\u{2028}' | '\u{2029}'
    )
}

/// Greedily wrap `text` into lines no wider than `max_width`.
///
/// `widths[i]` is the advance of the `i`-th `char` of `text` in caller-chosen integer units
/// (missing entries count as 0, negative ones as 0); `breaks` is the output of
/// [`line_breaks`](crate::line_breaks) for the same text. The result lists, in increasing
/// order, the byte offset at which each line ends (the next line starts there), so the last
/// entry is `text.len()` for non-empty text and the result is empty for empty text.
///
/// Rules: a mandatory break always ends a line; spaces and line terminators at the end of a
/// line do not count toward its width; a line ends at the last allowed break that keeps it
/// within `max_width`; when no allowed break fits (a word wider than the line) the line is
/// cut before the first `char` that would overflow, so every line holds at least one `char`
/// and the wrapper always makes progress. Breaks fall on `char` boundaries, not grapheme
/// clusters. All arithmetic saturates.
#[must_use]
pub fn wrap(text: &str, widths: &[i64], max_width: i64, breaks: &[Break]) -> Vec<usize> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let offset_of = |i: usize| chars.get(i).map_or(text.len(), |e| e.0);
    let mut ends = Vec::new();
    let (mut start, mut i) = (0_usize, 0_usize);
    let (mut content, mut pending, mut cand) = (0_i64, 0_i64, None::<usize>);
    while let Some(&(off, ch)) = chars.get(i) {
        if i > start {
            let hit = breaks.binary_search_by_key(&off, |b| b.offset).ok();
            match hit.and_then(|k| breaks.get(k)).map(|b| b.kind) {
                Some(BreakKind::Mandatory) => {
                    ends.push(off);
                    (start, content, pending, cand) = (i, 0, 0, None);
                    continue;
                }
                Some(BreakKind::Allowed) => cand = Some(i),
                None => {}
            }
        }
        let w = widths.get(i).copied().unwrap_or(0).max(0);
        if hangs(ch) {
            pending = pending.saturating_add(w);
        } else {
            let next = content.saturating_add(pending).saturating_add(w);
            if next > max_width && i > start {
                let at = cand.filter(|&c| c > start).unwrap_or(i);
                ends.push(offset_of(at));
                (start, i, content, pending, cand) = (at, at, 0, 0, None);
                continue;
            }
            (content, pending) = (next, 0);
        }
        i = i.saturating_add(1);
    }
    if start < chars.len() {
        ends.push(text.len());
    }
    ends
}
