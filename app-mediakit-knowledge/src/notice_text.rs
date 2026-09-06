// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Canonical JOURNAL notice-banner text, loaded at startup from
//! `factory-release-engineering/tokens/notice-text-journal.yaml` — the
//! counsel/editorial-governed source of truth (schema
//! `foundry-journal-notice-text-v1`), same discipline `legal.rs` already
//! follows for trademark/copyright text: this engine must never author
//! disclosure copy locally.
//!
//! Deliberately generic (no app-mediakit-knowledge-specific logic) — the same
//! file is intended for other conforming renderers (e.g. a future
//! app-orchestration-gis JOURNAL surface) per
//! `SPEC-journal-wiki-render-contract.md` §0.3/§4.

use std::path::Path;

use serde::Deserialize;

/// Default location of the canonical token file on the workspace VM — same
/// directory `legal.rs`'s `DEFAULT_LEGAL_TOKENS_DIR` reads from.
pub const DEFAULT_NOTICE_TEXT_DIR: &str = "/srv/foundry/vendor/factory-release-engineering/tokens";

const FILE_NAME: &str = "notice-text-journal.yaml";

#[derive(Debug, Clone, Deserialize)]
pub struct Template {
    pub template: String,
}

/// Parsed `notice-text-journal.yaml`.
#[derive(Debug, Clone, Deserialize)]
pub struct NoticeText {
    pub working_paper_notice: Template,
    pub forward_looking_statements: Template,
    pub citation_banner: Template,
    pub superseded_notice: Template,
}

/// Load `notice-text-journal.yaml` from `dir`. Malformed or absent → `None`
/// (caller renders nothing rather than fabricating disclosure text).
pub fn load(dir: &Path) -> Option<NoticeText> {
    let text = std::fs::read_to_string(dir.join(FILE_NAME)).ok()?;
    serde_yaml::from_str(&text).ok()
}

/// Load using the default canonical directory.
pub fn load_default() -> Option<NoticeText> {
    load(Path::new(DEFAULT_NOTICE_TEXT_DIR))
}

/// Fill `{key}` placeholders in `template` from `vars`. A placeholder with no
/// matching var is left as literal `{key}` text rather than silently blanked
/// — a visible gap is easier to notice and fix than a sentence that quietly
/// stops making sense.
pub fn fill_template(template: &str, vars: &[(&str, &str)]) -> String {
    let mut out = template.trim().to_string();
    for (key, value) in vars {
        out = out.replace(&format!("{{{key}}}"), value);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_SHAPE: &str = r#"
schema: foundry-journal-notice-text-v1
working_paper_notice:
  template: >
    This is a working paper (v{version}), posted {preprint_posted_date}, {license}.
    Correspondence: {corresponding_author}. Cite as: {cite_as}.
forward_looking_statements:
  template: >
    Static forward-looking text, no placeholders.
citation_banner:
  template: >
    Published version: {cite_as} DOI: {doi}
superseded_notice:
  template: >
    Superseded, see v{revision_history.latest.version}.
"#;

    #[test]
    fn loads_real_shape() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(FILE_NAME), REAL_SHAPE).unwrap();
        let loaded = load(dir.path()).unwrap();
        assert!(loaded
            .working_paper_notice
            .template
            .contains("{corresponding_author}"));
        assert!(loaded
            .forward_looking_statements
            .template
            .contains("Static"));
    }

    #[test]
    fn missing_file_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        assert!(load(dir.path()).is_none());
    }

    #[test]
    fn malformed_yaml_returns_none() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(FILE_NAME), ":::not valid:::").unwrap();
        assert!(load(dir.path()).is_none());
    }

    #[test]
    fn fill_template_substitutes_all_present_vars() {
        let filled = fill_template(
            "v{version} by {corresponding_author}",
            &[
                ("version", "0.4.0"),
                ("corresponding_author", "a@example.com"),
            ],
        );
        assert_eq!(filled, "v0.4.0 by a@example.com");
    }

    #[test]
    fn fill_template_leaves_unmatched_placeholder_literal() {
        let filled = fill_template("v{version}, {missing}", &[("version", "0.4.0")]);
        assert_eq!(filled, "v0.4.0, {missing}");
    }
}
