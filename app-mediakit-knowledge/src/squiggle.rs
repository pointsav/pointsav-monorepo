//! Phase 2 Step 4 — SAA squiggle framework (deterministic rules).
//!
//! Per PHASE-2-PLAN.md §1 Step 4 + UX-DESIGN.md §5.3 (Grammarly-pattern
//! coloured squiggles, with cited authority on hover).
//!
//! Phase 2 ships a small illustrative rule set so the framework round-trips
//! end-to-end (server → JSON → CodeMirror lint → squiggle in editor → tooltip
//! cites the rule). The set grows over time via straight edits to this file
//! plus, eventually, the Phase 9 CCA constitutional-layer adapter that
//! emits jurisdiction-specific rule packs.
//!
//! Rule severities mirror the convention in UX-DESIGN.md §5.3:
//! - `error` (red)   — hard substrate violation; commit gate blocks
//! - `warning` (amber) — unsourced claim; commit allowed with friction
//! - `info` (blue)   — unlabelled FLI; informational
//! - `hint` (gray)   — style-guide drift; lowest noise

use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug, Serialize, Clone)]
pub struct SquiggleRule {
    /// Stable rule identifier (used in tooltip + future suppression syntax).
    pub id: String,
    /// CodeMirror lint Diagnostic severity.
    pub severity: Severity,
    /// JavaScript-compatible regex source (no leading/trailing slashes).
    pub pattern: String,
    /// Regex flags for the JS RegExp constructor (`i`, `m`, `g`, etc.).
    pub flags: String,
    /// Tooltip text shown on hover.
    pub message: String,
    /// Citation registry ID or doctrine reference establishing the rule.
    pub citation: String,
}

/// Compile-time deterministic rule set. Rules consume:
/// - `~/Foundry/CLAUDE.md` §6 (BCSC continuous-disclosure posture)
/// - `~/Foundry/conventions/bcsc-disclosure-posture.md` (FLI patterns)
/// - `~/Foundry/POINTSAV-Project-Instructions.md` §5 ("Do Not Use" terms)
/// - `DOCTRINE.md` (ADR-07 / ADR-19 hard rules)
///
/// Per-jurisdiction + per-tenant rule packs land via Phase 9 CCA — the
/// constitutional-layer adapter emits them as additional entries grafted
/// onto this baseline.
pub fn deterministic_rule_set() -> Vec<SquiggleRule> {
    vec![
        // === Red — substrate violation, commit blocked ===
        SquiggleRule {
            id: "bcsc-rule-2-foundation-tense".into(),
            severity: Severity::Error,
            pattern: r"Sovereign Data Foundation\s+(is|are)\s+(currently|now|the\s+active|the\s+current|presently)".into(),
            flags: "gi".into(),
            message: "Sovereign Data Foundation must be referred to in planned / intended terms only — never current-tense as equity holder or active auditor.".into(),
            citation: "claude-md-§6-bcsc-rule-2".into(),
        },
        SquiggleRule {
            id: "adr-19-automated-publishing".into(),
            severity: Severity::Error,
            pattern: r"automated\s+AI\s+(publishing|publication)\s+(to|of)\s+(verified|signed|sec|edgar|sedar)".into(),
            flags: "gi".into(),
            message: "SYS-ADR-19 prohibits automated AI publishing to verified ledgers. The publication path must include a human checkpoint (F12 per SYS-ADR-10).".into(),
            citation: "sys-adr-19".into(),
        },
        // === Amber — unsourced claim ===
        SquiggleRule {
            id: "amber-unsourced-confident-claim".into(),
            severity: Severity::Warning,
            // Confident-claim verbs at start-of-sentence followed by a clause
            // not containing a [citation-id]. Heuristic only — false positives
            // are tolerated; the goal is to surface unsourced assertions.
            pattern: r"(?:^|\.\s+)(?:Notably|Indeed|Clearly|Obviously|Crucially|Demonstrably)\b[^.]{1,140}\.".into(),
            flags: "gm".into(),
            message: "Confident-claim adverb without nearby citation — consider grounding with a `[citation-id]` reference per CLAUDE.md §16.".into(),
            citation: "claude-md-§16-citation-discipline".into(),
        },
        // === Blue — forward-looking-information labelling ===
        SquiggleRule {
            id: "blue-fli-language-pattern".into(),
            severity: Severity::Info,
            // Forward-looking verb patterns. If the frontmatter has
            // `forward_looking: true`, the rendered chrome already shows
            // the FLI banner; in the editor, this hint reminds the author
            // to verify that frontmatter flag matches the prose. Phase 9 CCA
            // will combine this with the parsed frontmatter to upgrade /
            // downgrade severity dynamically.
            pattern: r"\b(intend(?:s|ed)?\s+to|expect(?:s|ed)?\s+to|plan(?:s|ned)?\s+to|target(?:s|ed)?\s+to|will\s+(?:deliver|launch|ship|release|achieve))\b".into(),
            flags: "gi".into(),
            message: "Forward-looking statement detected. Confirm frontmatter `forward_looking: true` is set and the cautionary-language patterns from `conventions/bcsc-disclosure-posture.md` are present.".into(),
            citation: "osc-sn-51-721".into(),
        },
        // === Gray — style / Do-Not-Use vocabulary ===
        SquiggleRule {
            id: "gray-do-not-use-cognitive-forge".into(),
            severity: Severity::Hint,
            pattern: r"\bcognitive\s+forge\b".into(),
            flags: "gi".into(),
            message: "\"Cognitive Forge\" is on the Do-Not-Use list (POINTSAV-Project-Instructions.md §5). Use `service-slm/router/` (runtime) or `service-slm/router-trainer/` (distillation workflow).".into(),
            citation: "pointsav-project-instructions-§5".into(),
        },
        SquiggleRule {
            id: "gray-do-not-use-archiveos".into(),
            severity: Severity::Hint,
            pattern: r"\b(ArchiveOS|Archive\s+OS)\b".into(),
            flags: "g".into(),
            message: "Canonical name is ToteboxOS, not ArchiveOS. (Nomenclature Matrix §2.)".into(),
            citation: "nomenclature-matrix-§2".into(),
        },
        SquiggleRule {
            id: "gray-marketing-vocabulary".into(),
            severity: Severity::Hint,
            pattern: r"\b(revolutionary|game-changing|cutting-edge|world-class|seamlessly|leverage|disrupt(?:ive)?)\b".into(),
            flags: "gi".into(),
            message: "Marketing vocabulary detected. Bloomberg article standard (CLAUDE.md §6): clear, professional, no marketing copy.".into(),
            citation: "claude-md-§6-bloomberg-standard".into(),
        },
    ]
}

/// HTTP handler for `GET /api/squiggle-rules` — returns the deterministic
/// rule set as JSON. The CodeMirror lint extension on the client side
/// fetches this once at editor init and runs each rule against doc changes.
pub async fn get_squiggle_rules() -> Json<Vec<SquiggleRule>> {
    Json(deterministic_rule_set())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_set_is_non_empty() {
        let rules = deterministic_rule_set();
        assert!(!rules.is_empty());
    }

    #[test]
    fn rule_set_has_each_severity_at_least_once() {
        let rules = deterministic_rule_set();
        let severities: std::collections::HashSet<&str> = rules
            .iter()
            .map(|r| match r.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Info => "info",
                Severity::Hint => "hint",
            })
            .collect();
        assert!(severities.contains("error"), "missing error rules");
        assert!(severities.contains("warning"), "missing warning rules");
        assert!(severities.contains("info"), "missing info rules");
        assert!(severities.contains("hint"), "missing hint rules");
    }

    #[test]
    fn rule_ids_are_unique() {
        let rules = deterministic_rule_set();
        let mut ids: Vec<&str> = rules.iter().map(|r| r.id.as_str()).collect();
        ids.sort();
        let len = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), len, "duplicate rule IDs in rule set");
    }

    #[test]
    fn every_rule_carries_a_citation() {
        for r in deterministic_rule_set() {
            assert!(!r.citation.is_empty(), "rule {} missing citation", r.id);
            assert!(!r.message.is_empty(), "rule {} missing message", r.id);
            assert!(!r.pattern.is_empty(), "rule {} missing pattern", r.id);
        }
    }

    #[test]
    fn serialises_severity_lowercase() {
        let rule = SquiggleRule {
            id: "x".into(),
            severity: Severity::Warning,
            pattern: "foo".into(),
            flags: "g".into(),
            message: "m".into(),
            citation: "c".into(),
        };
        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains(r#""severity":"warning""#));
    }
}
