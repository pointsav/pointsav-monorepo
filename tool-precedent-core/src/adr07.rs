// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! SYS-ADR-07 gate (development wording): a model may read, prepare, check and propose, but never
//! writes a record; a person approves each write. This gate answers "may this actor write this
//! path?": an AI actor is refused on a file whose extension marks it as structured data, a human
//! actor passes. `store::EditStore` lets a model *propose* anything and applies this gate to the
//! approver at commit.
//! The list errs toward blocking. `csv`/`tsv` are included because the precedent shards
//! themselves (changes, terms, draws, versions) are CSV.

use crate::error::{CoreError, Result};

/// Who is acting: proposing an edit, or approving (committing) one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Human,
    Ai,
}

/// Extensions treated as structured data (lowercase, no dot).
pub const STRUCTURED_EXTENSIONS: &[&str] = &[
    "json", "geojson", "jsonl", "ndjson", "schedule", "csv", "tsv", "toml", "yaml", "yml",
];

pub fn is_structured(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => {
            STRUCTURED_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str())
        }
        _ => false,
    }
}

/// `Err(StructuredBlocked)` if an AI actor targets structured data.
pub fn check(path: &str, origin: Origin) -> Result<()> {
    if origin == Origin::Ai && is_structured(path) {
        Err(CoreError::StructuredBlocked(path.to_string()))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_by_extension() {
        for p in [
            "a.json",
            "d/x.GeoJSON",
            "plan.schedule",
            "precedent/terms.csv",
            "c.yaml",
        ] {
            assert!(is_structured(p), "{p}");
        }
        for p in [
            "notes.md",
            "a.txt",
            "json",
            ".json",
            "dir.json/readme",
            "x.html",
        ] {
            assert!(!is_structured(p), "{p}");
        }
    }

    #[test]
    fn blocks_ai_only() {
        assert_eq!(
            check("a.json", Origin::Ai),
            Err(CoreError::StructuredBlocked("a.json".into()))
        );
        assert!(check("a.json", Origin::Human).is_ok());
        assert!(check("a.md", Origin::Ai).is_ok());
    }
}
