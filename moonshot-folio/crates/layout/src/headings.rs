// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Headings found in a finished `PageTree`: the source of table-of-contents entries and of PDF
//! bookmarks.

use crate::tree::{Area, BlockKind, ItemKind, NodeId, PageTree};
use moonshot_folio_core::Emu;
use std::collections::BTreeMap;

/// One heading of the body: where it starts and what it says.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadingRef {
    /// Level, 1 to 6.
    pub level: u8,
    /// The text of the heading (lines joined by one space).
    pub title: String,
    /// Zero-based index of the page the heading starts on.
    pub page: usize,
    /// The page number shown for that page.
    pub label: u32,
    /// Left edge of the first text of the heading.
    pub x: Emu,
    /// Top of the first line of the heading, from the bottom page edge.
    pub y: Emu,
    /// The IR node of the heading block (`[section, area, block]`).
    pub node: NodeId,
}

/// Anchor names starting with this are reserved for heading destinations.
pub(crate) const RESERVED_PREFIX: &str = "folio-toc-";

/// The name of the named destination that a heading gets when the document has a table of
/// contents (reserved: `folio-toc-<section>-<block>`).
#[must_use]
pub(crate) fn toc_dest_name(node: &NodeId) -> String {
    let p = node.path();
    format!(
        "{RESERVED_PREFIX}{}-{}",
        p.first().copied().unwrap_or(0),
        p.get(2).copied().unwrap_or(0)
    )
}

/// The `(section, block)` key of a body item, if it is one.
fn body_key(node: &NodeId) -> Option<(u32, u32)> {
    let p = node.path();
    match (p.first(), p.get(1), p.get(2)) {
        (Some(s), Some(a), Some(b)) if *a == Area::Body as u32 => Some((*s, *b)),
        _ => None,
    }
}

struct Acc {
    level: u8,
    title: String,
    /// Page index and baseline of the last item, and whether its line ended at a dropped space.
    last: Option<(usize, Emu, bool)>,
    first: Option<(usize, u32, Emu, Emu, NodeId)>,
}

/// The headings of the body in document order, keyed by the IR block so a heading made only of
/// links is included. Headings in headers and footers are not included, nor are headings without
/// visible text. A title is the text of the heading with lines joined by one space only where a
/// line was broken at a space.
#[must_use]
pub fn headings(tree: &PageTree) -> Vec<HeadingRef> {
    let mut acc: BTreeMap<(u32, u32), Acc> = BTreeMap::new();
    for info in &tree.blocks {
        let (BlockKind::Heading(level), Some(k)) = (&info.kind, body_key(&info.node)) else {
            continue;
        };
        {
            acc.entry(k).or_insert(Acc {
                level: *level,
                title: String::new(),
                last: None,
                first: None,
            });
        }
    }
    for (pi, page) in tree.pages.iter().enumerate() {
        for it in &page.items {
            let (ItemKind::Text(t), Some(key)) = (&it.kind, body_key(&it.node)) else {
                continue;
            };
            let Some(a) = acc.get_mut(&key) else {
                continue;
            };
            if a.last.is_some_and(|(p, y, sp)| (p, y) != (pi, it.y) && sp) {
                a.title.push(' ');
            }
            a.last = Some((pi, it.y, t.space_after));
            a.title.push_str(&t.text);
            if a.first.is_none() {
                let top = it.y.checked_add(t.ascent).unwrap_or(it.y);
                a.first = Some((pi, page.label, it.x, top, it.node));
            }
        }
    }
    let mut out = Vec::new();
    for (_, a) in acc {
        let Some((page, label, x, y, node)) = a.first else {
            continue;
        };
        let title = a.title.trim().to_string();
        if title.is_empty() {
            continue;
        }
        let block = NodeId::block(
            node.path().first().copied().map_or(0, |v| v as usize),
            Area::Body,
            node.path().get(2).copied().map_or(0, |v| v as usize),
        );
        out.push(HeadingRef {
            level: a.level,
            title,
            page,
            label,
            x,
            y,
            node: block,
        });
    }
    out
}

/// A table-of-contents entry: a heading as the previous pass found it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TocEntry {
    pub(crate) level: u8,
    pub(crate) title: String,
    pub(crate) label: u32,
    pub(crate) dest: String,
}

/// The entries for the next pass.
pub(crate) fn toc_entries(tree: &PageTree) -> Vec<TocEntry> {
    headings(tree)
        .into_iter()
        .map(|h| TocEntry {
            level: h.level,
            dest: toc_dest_name(&h.node),
            title: h.title,
            label: h.label,
        })
        .collect()
}
