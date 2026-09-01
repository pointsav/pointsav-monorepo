// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Phase 8 (`KNOWLEDGE-PLATFORM-PLAN.md`): editorial-standards linter.
//!
//! **Provisional starter ruleset**, not project-editorial's eventual real
//! one — swap `RULESET` (or the whole check functions) out once that lands.
//! Encodes what this workspace already enforces informally per
//! `CLAUDE.md`/`AGENT.md`: the Bloomberg-article standard (precise,
//! professional, no AI-product marketing vocabulary), the BCSC
//! continuous-disclosure posture (forward-looking claims need hedge
//! language; the Sovereign Data Foundation specifically must stay in
//! planned/intended terms), and structural-positioning-only competitive
//! language (no named comparisons).
//!
//! Line-based, case-insensitive substring matching — no NLP, no external
//! calls (SYS-ADR-07-safe by construction: this never routes structured
//! data through an AI model, it's a static wordlist check).

use serde::Serialize;

/// One flagged line.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EditorialFinding {
    pub rule: &'static str,
    pub severity: &'static str,
    pub line: usize,
    pub excerpt: String,
    pub message: &'static str,
}

/// AI-product marketing vocabulary — the Bloomberg-standard "Do Not Use"
/// starter list. Case-insensitive whole-word-ish substring match.
const MARKETING_TERMS: &[&str] = &[
    "revolutionary",
    "game-changing",
    "game changing",
    "cutting-edge",
    "cutting edge",
    "seamless",
    "seamlessly",
    "unlock the power",
    "supercharge",
    "next-generation",
    "next generation",
    "disruptive",
    "state-of-the-art",
    "state of the art",
    "empower",
    "empowering",
    "unprecedented",
    "world-class",
    "best-in-class",
    "hyperscale your",
    "reimagine",
    "reimagining",
    "transformative",
    "paradigm shift",
    "unleash",
];

/// Forward-looking hedge words — at least one must appear near a
/// forward-looking claim about the Sovereign Data Foundation for it to pass
/// the BCSC check.
const HEDGE_WORDS: &[&str] = &[
    "planned",
    "intended",
    "may ",
    "target",
    "anticipated",
    "expected to",
    "is expected",
    "will be considered",
    "proposed",
];

/// Run the provisional starter ruleset over one document's body text.
/// Returns every flagged line, in document order.
pub fn validate_editorial_standards(body_md: &str) -> Vec<EditorialFinding> {
    let mut findings = Vec::new();
    for (idx, line) in body_md.lines().enumerate() {
        let line_no = idx + 1;
        let lower = line.to_lowercase();

        for term in MARKETING_TERMS {
            if lower.contains(term) {
                findings.push(EditorialFinding {
                    rule: "marketing-vocabulary",
                    severity: "medium",
                    line: line_no,
                    excerpt: line.trim().to_string(),
                    message: "AI-product marketing vocabulary — Bloomberg-standard \
                              prose avoids this register; state the fact plainly instead.",
                });
                break; // one flag per line for this rule is enough signal
            }
        }

        if lower.contains("sovereign data foundation") {
            let hedged = HEDGE_WORDS.iter().any(|h| lower.contains(h));
            if !hedged {
                findings.push(EditorialFinding {
                    rule: "bcsc-forward-looking",
                    severity: "high",
                    line: line_no,
                    excerpt: line.trim().to_string(),
                    message: "Sovereign Data Foundation must be referred to in \
                              planned/intended terms only (BCSC continuous-disclosure \
                              posture) — no hedge word found on this line.",
                });
            }
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_marketing_vocabulary() {
        let body = "This is a revolutionary, seamless new platform.";
        let findings = validate_editorial_standards(body);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "marketing-vocabulary");
        assert_eq!(findings[0].line, 1);
    }

    #[test]
    fn flags_unhedged_sovereign_data_foundation_claim() {
        let body = "The Sovereign Data Foundation audits every disclosure filed here.";
        let findings = validate_editorial_standards(body);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, "bcsc-forward-looking");
    }

    #[test]
    fn passes_hedged_sovereign_data_foundation_claim() {
        let body = "The Sovereign Data Foundation is planned to audit disclosures filed here.";
        assert!(validate_editorial_standards(body).is_empty());
    }

    #[test]
    fn clean_prose_has_no_findings() {
        let body = "PointSav Digital Systems maintains this record under SYS-ADR-07.\n\
                     Filings are reviewed quarterly.";
        assert!(validate_editorial_standards(body).is_empty());
    }

    #[test]
    fn reports_correct_line_numbers_across_multiple_lines() {
        let body = "Line one is fine.\nThis line is revolutionary.\nLine three is fine too.";
        let findings = validate_editorial_standards(body);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].line, 2);
    }
}
