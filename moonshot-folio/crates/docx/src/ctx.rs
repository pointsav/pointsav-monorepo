// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! State gathered while writing the body: style roles, external links, bookmark ids.

use core::fmt::Write;
use moonshot_folio_core::{FolioError, Result};
use std::collections::BTreeSet;

/// A header or footer part.
pub(crate) struct Part {
    /// Path inside the package, e.g. `word/header1.xml`.
    pub(crate) name: String,
    /// Relationship id used by `w:sectPr`.
    pub(crate) rel_id: String,
    /// True for a footer, false for a header.
    pub(crate) footer: bool,
    /// The XML.
    pub(crate) xml: String,
}

/// Things the body writer discovers that other parts must declare.
#[derive(Default)]
pub(crate) struct Ctx {
    /// Roles used by paragraphs, in sorted order.
    pub(crate) para_roles: BTreeSet<String>,
    /// Roles used by runs, in sorted order.
    pub(crate) char_roles: BTreeSet<String>,
    /// External link targets; link `i` has relationship id `rId{i + 2}` (`rId1` is styles).
    pub(crate) links: Vec<String>,
    /// Next bookmark id.
    pub(crate) next_bookmark: u32,
    /// Bookmark names defined so far.
    pub(crate) anchors: Vec<String>,
    /// Internal link targets used (without the `#`).
    pub(crate) anchor_links: Vec<String>,
    /// Width between the margins of the current section, in twips.
    pub(crate) content_twips: i64,
    /// List numbering ids in first-use order with their ordered flag; `numId` is index + 1.
    pub(crate) nums: Vec<(u32, bool)>,
    /// Header and footer parts, in section order.
    pub(crate) parts: Vec<Part>,
    /// True while writing a header or footer part (which has no relationships of its own).
    pub(crate) in_part: bool,
}

impl Ctx {
    /// Relationship id for an external target, registering it on first use.
    pub(crate) fn link_id(&mut self, target: &str) -> String {
        let idx = if let Some(i) = self.links.iter().position(|t| t == target) {
            i
        } else {
            self.links.push(target.to_string());
            self.links.len().saturating_sub(1)
        };
        format!("rId{}", idx.saturating_add(2))
    }
    /// Next bookmark id.
    pub(crate) fn bookmark_id(&mut self) -> u32 {
        let id = self.next_bookmark;
        self.next_bookmark = id.saturating_add(1);
        id
    }
}

/// Injective style id for a role: `TSP_` (paragraph) or `TSC_` (character) plus the role with
/// every byte outside `[A-Za-z0-9]` written as `_XX` (hex). Distinct roles, and the same role used
/// as paragraph and as run style, never share an id.
#[must_use]
pub(crate) fn style_id(kind: char, role: &str) -> String {
    let mut s = format!("TS{kind}_");
    for b in role.bytes() {
        if b.is_ascii_alphanumeric() {
            s.push(char::from(b));
        } else {
            write!(s, "_{b:02X}").ok();
        }
    }
    s
}

/// Only these URI schemes may leave the document through a hyperlink.
///
/// # Errors
/// `Invalid` for any other scheme.
pub(crate) fn check_target(t: &str) -> Result<()> {
    let ok = ["http://", "https://", "mailto:"]
        .iter()
        .any(|p| t.get(..p.len()).is_some_and(|h| h.eq_ignore_ascii_case(p)));
    if ok {
        Ok(())
    } else {
        Err(FolioError::Invalid(
            "link target must be http, https, mailto or #anchor",
        ))
    }
}

/// Bookmark names: a letter, then up to 39 of `[A-Za-z0-9_]`, as Word requires.
///
/// # Errors
/// `Invalid` otherwise.
pub(crate) fn check_bookmark(n: &str) -> Result<()> {
    let ok = n.bytes().next().is_some_and(|b| b.is_ascii_alphabetic())
        && n.len() <= 40
        && n.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
    if ok {
        Ok(())
    } else {
        Err(FolioError::Invalid(
            "anchor name must start with a letter and use only A-Z a-z 0-9 _ (40 at most)",
        ))
    }
}
