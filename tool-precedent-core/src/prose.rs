// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! The prose contract between the tool-precedent-<domain> crates and a proofreader. A domain crate
//! implements [`ProseDocument`]: it says which byte ranges of its source are prose
//! ([`ProseDocument::prose_spans`]) and applies accepted edits back
//! ([`ProseDocument::apply_edits`]). The proofreader sees only spans and returns [`Flag`]s; there is
//! no dependency in either direction beyond this module. Applying edits is pure: it returns or
//! mutates in memory. Writing to disk goes through `store::EditStore`, where a person approves
//! (SYS-ADR-07).

use crate::error::{CoreError, Result};
use crate::span::{apply_edit, Edit, Span};

/// A run of prose in a document's source, with the field it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProseSpan {
    /// Which field or section of the document this belongs to (the domain crate chooses the names).
    pub field: String,
    pub span: Span,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// What a proofreader returns for a span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flag {
    pub span: Span,
    pub rule: String,
    pub severity: Severity,
    pub suggestion: Option<String>,
    /// Who raised it (a rule set, a model name, a person).
    pub source: String,
}

/// Implemented by each domain crate's document type.
pub trait ProseDocument {
    /// The prose ranges of the document, in source order.
    fn prose_spans(&self) -> Vec<ProseSpan>;
    /// Apply accepted edits (byte ranges in the current source) back to the document.
    fn apply_edits(&mut self, edits: &[Edit]) -> Result<()>;
}

/// The edits that accepting every flag with a suggestion would make.
pub fn edits_from_flags(flags: &[Flag]) -> Vec<Edit> {
    flags
        .iter()
        .filter_map(|f| f.suggestion.as_ref().map(|s| Edit::new(f.span, s.clone())))
        .collect()
}

/// Apply a batch of edits, all expressed against the ORIGINAL `source`, in one pass. Order in the
/// slice does not matter; overlapping edits and invalid ranges are errors and nothing is applied.
/// Two insertions at the same offset keep their slice order.
pub fn apply_edits_to_source(source: &str, edits: &[Edit]) -> Result<String> {
    let mut order: Vec<usize> = (0..edits.len()).collect();
    order.sort_by_key(|&i| (edits[i].range.start, edits[i].range.end, i));
    for &i in &order {
        edits[i].range.validate(source)?;
    }
    for w in order.windows(2) {
        let (a, b) = (&edits[w[0]].range, &edits[w[1]].range);
        // touching ranges are fine; an overlap, or an insertion strictly inside a replaced range, is not
        if b.start < a.end {
            return Err(CoreError::OverlappingEdits);
        }
    }
    // apply from the back so earlier offsets stay valid
    let mut out = source.to_string();
    for &i in order.iter().rev() {
        out = apply_edit(&out, &edits[i])?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Plain(String);

    impl ProseDocument for Plain {
        fn prose_spans(&self) -> Vec<ProseSpan> {
            // every non-empty line is prose
            let mut out = Vec::new();
            let mut at = 0;
            for line in self.0.split_inclusive('\n') {
                let t = line.trim_end_matches('\n');
                if !t.is_empty() {
                    out.push(ProseSpan {
                        field: "body".into(),
                        span: Span::new(at, at + t.len()),
                        text: t.to_string(),
                    });
                }
                at += line.len();
            }
            out
        }
        fn apply_edits(&mut self, edits: &[Edit]) -> Result<()> {
            self.0 = apply_edits_to_source(&self.0, edits)?;
            Ok(())
        }
    }

    #[test]
    fn batch_is_order_independent_and_uses_original_offsets() {
        let s = "aaa bbb ccc";
        let e1 = Edit::new(Span::new(0, 3), "x");
        let e2 = Edit::new(Span::new(8, 11), "yyyyy");
        let a = apply_edits_to_source(s, &[e1.clone(), e2.clone()]).unwrap();
        let b = apply_edits_to_source(s, &[e2, e1]).unwrap();
        assert_eq!(a, "x bbb yyyyy");
        assert_eq!(a, b);
    }

    #[test]
    fn overlap_and_bad_ranges_apply_nothing() {
        let s = "abcdef";
        let r = apply_edits_to_source(
            s,
            &[
                Edit::new(Span::new(1, 4), "X"),
                Edit::new(Span::new(3, 5), "Y"),
            ],
        );
        assert_eq!(r, Err(CoreError::OverlappingEdits));
        // an insertion inside a replaced range overlaps
        let r = apply_edits_to_source(
            s,
            &[
                Edit::new(Span::new(1, 4), "X"),
                Edit::new(Span::new(2, 2), "Y"),
            ],
        );
        assert_eq!(r, Err(CoreError::OverlappingEdits));
        assert!(matches!(
            apply_edits_to_source(s, &[Edit::new(Span::new(4, 99), "X")]),
            Err(CoreError::OutOfRange { .. })
        ));
        // touching ranges and adjacent insertions are fine, insertions keep slice order
        let r = apply_edits_to_source(
            s,
            &[
                Edit::new(Span::new(2, 2), "1"),
                Edit::new(Span::new(2, 2), "2"),
                Edit::new(Span::new(0, 2), "Z"),
            ],
        )
        .unwrap();
        assert_eq!(r, "Z12cdef");
    }

    #[test]
    fn flags_become_edits_only_when_they_carry_a_suggestion() {
        let flags = vec![
            Flag {
                span: Span::new(0, 3),
                rule: "spell".into(),
                severity: Severity::Warning,
                suggestion: Some("the".into()),
                source: "rules".into(),
            },
            Flag {
                span: Span::new(4, 6),
                rule: "style".into(),
                severity: Severity::Info,
                suggestion: None,
                source: "rules".into(),
            },
        ];
        let edits = edits_from_flags(&flags);
        assert_eq!(edits, vec![Edit::new(Span::new(0, 3), "the")]);
        assert!(Severity::Error > Severity::Warning);
    }

    #[test]
    fn a_domain_document_round_trips_spans_and_edits() {
        let mut d = Plain("teh cat\n\nsat down\n".into());
        let spans = d.prose_spans();
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[1].text, "sat down");
        let flag = Flag {
            span: Span::new(0, 3),
            rule: "spell".into(),
            severity: Severity::Warning,
            suggestion: Some("the".into()),
            source: "t".into(),
        };
        d.apply_edits(&edits_from_flags(&[flag])).unwrap();
        assert_eq!(d.0, "the cat\n\nsat down\n");
        // spans re-derived from the new source point at the right text
        assert_eq!(d.prose_spans()[0].text, "the cat");
    }
}
