// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! SYS-ADR-07 gate (development wording): a model may read, prepare, check and propose, but never
//! writes a record; a person approves each write. This gate answers "may this actor write this
//! path?": an AI actor is refused on a file whose extension marks it as structured data, a human
//! actor passes. `store::EditStore` lets a model *propose* anything and applies this gate to the
//! approver at commit.
//!
//! Scope of the gate: it guards the library path (`store::EditStore::commit`). The command line
//! does not call it. The CLI write commands (`changes-add`, `terms-add`, `draw-add`, `versions`,
//! `apply`, `registry --out`) are run at a person's instruction (`commands.rs`, `Access::ai`),
//! and the person approving the command in the session is the gate. A CLI cannot tell a person
//! from a model, so a flag such as `--approved-by` would be a string a model could type too: it
//! would look like a gate without being one. What the CLI does enforce is mechanical: every write
//! target stays inside `--root` (audit decision of 2026-11-21).
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
    // The last real name in the path, as the file system will resolve it: a trailing `/`, `/.`
    // or `./` does not change which file is written.
    let name = std::path::Path::new(path)
        .components()
        .rev()
        .find_map(|c| match c {
            std::path::Component::Normal(n) => n.to_str(),
            _ => None,
        })
        .unwrap_or("");
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
    fn a_trailing_slash_or_dot_does_not_hide_a_structured_file() {
        for p in [
            "a.json/",
            "a.json/.",
            "d/a.csv/./",
            "d//a.yaml//",
            "./a.toml/",
        ] {
            assert!(is_structured(p), "{p} must count as structured");
            assert!(
                check(p, Origin::Ai).is_err(),
                "{p} must be blocked for an AI"
            );
        }
    }

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
