// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Byte spans, text edits and span remapping. Every tool-precedent-* domain engine addresses
//! its source by byte span, so one remap primitive serves all of them. Unlike a bare splice,
//! `apply_edit` validates bounds and UTF-8 boundaries and returns an error instead of panicking.

use crate::error::{CoreError, Result};

/// A byte range `[start, end)` into a source string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }

    /// Check the span lies inside `source` on character boundaries.
    pub fn validate(&self, source: &str) -> Result<()> {
        if self.start > self.end || self.end > source.len() {
            return Err(CoreError::OutOfRange {
                start: self.start,
                end: self.end,
                len: source.len(),
            });
        }
        for i in [self.start, self.end] {
            if !source.is_char_boundary(i) {
                return Err(CoreError::NotCharBoundary(i));
            }
        }
        Ok(())
    }

    /// The slice this span points at, or an error if it is invalid for `source`.
    pub fn text<'a>(&self, source: &'a str) -> Result<&'a str> {
        self.validate(source)?;
        Ok(&source[self.start..self.end])
    }
}

/// Replace the bytes in `range` with `replacement`. Insertion is an empty `range`;
/// deletion is an empty `replacement`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub range: Span,
    pub replacement: String,
}

impl Edit {
    pub fn new(range: Span, replacement: impl Into<String>) -> Self {
        Self {
            range,
            replacement: replacement.into(),
        }
    }

    /// Signed change in byte length.
    pub fn delta(&self) -> isize {
        self.replacement.len() as isize - self.range.len() as isize
    }
}

/// Map a span from old-source to new-source coordinates across `edit`. Entirely before the edit:
/// unchanged. Entirely after: shifted by the delta. Overlapping: `None` (re-derive it).
pub fn remap_span(span: Span, edit: &Edit) -> Option<Span> {
    if span.end <= edit.range.start {
        Some(span)
    } else if span.start >= edit.range.end {
        let d = edit.delta();
        Some(Span::new(
            (span.start as isize + d) as usize,
            (span.end as isize + d) as usize,
        ))
    } else {
        None
    }
}

/// The source after applying `edit`, or an error if the edit's range is invalid for `source`.
pub fn apply_edit(source: &str, edit: &Edit) -> Result<String> {
    edit.range.validate(source)?;
    let mut out = String::with_capacity((source.len() as isize + edit.delta()).max(0) as usize);
    out.push_str(&source[..edit.range.start]);
    out.push_str(&edit.replacement);
    out.push_str(&source[edit.range.end..]);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_replace_insert_delete() {
        let s = "hello world";
        assert_eq!(
            apply_edit(s, &Edit::new(Span::new(6, 11), "there")).unwrap(),
            "hello there"
        );
        assert_eq!(
            apply_edit(s, &Edit::new(Span::new(5, 5), ",")).unwrap(),
            "hello, world"
        );
        assert_eq!(
            apply_edit(s, &Edit::new(Span::new(5, 11), "")).unwrap(),
            "hello"
        );
    }

    #[test]
    fn invalid_ranges_error_not_panic() {
        assert!(matches!(
            apply_edit("abc", &Edit::new(Span::new(2, 9), "x")),
            Err(CoreError::OutOfRange { .. })
        ));
        assert!(matches!(
            apply_edit("abc", &Edit::new(Span::new(3, 1), "x")),
            Err(CoreError::OutOfRange { .. })
        ));
        assert_eq!(
            apply_edit("é", &Edit::new(Span::new(1, 2), "x")),
            Err(CoreError::NotCharBoundary(1))
        );
        assert_eq!(Span::new(0, 2).text("é").unwrap(), "é");
    }

    #[test]
    fn remap_before_after_overlap() {
        let e = Edit::new(Span::new(10, 15), "xy"); // delta -3
        assert_eq!(remap_span(Span::new(0, 10), &e), Some(Span::new(0, 10)));
        assert_eq!(remap_span(Span::new(15, 20), &e), Some(Span::new(12, 17)));
        assert_eq!(remap_span(Span::new(8, 12), &e), None);
        assert_eq!(remap_span(Span::new(12, 14), &e), None);
    }

    #[test]
    fn remapped_span_points_at_same_text() {
        let s = "aaa BBB ccc DDD";
        let tail = Span::new(12, 15);
        let e = Edit::new(Span::new(0, 3), "zzzzzz");
        let after = apply_edit(s, &e).unwrap();
        assert_eq!(remap_span(tail, &e).unwrap().text(&after).unwrap(), "DDD");
    }
}
