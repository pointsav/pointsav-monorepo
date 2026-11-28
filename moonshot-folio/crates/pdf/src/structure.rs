// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The logical structure of a laid-out document, built from the `PageTree` while its pages are
//! drawn: which items are content and which are artifacts, and which structure element owns each
//! piece of content.
//!
//! Mapping (reading order is document order; section, block and item numbers come from the item
//! node paths `[section, area, block, ...]` and the block kinds of the tree):
//!
//! - `Document` holds one `Sect` per IR section that has content.
//! - A paragraph is `P`; a heading `H1` to `H6`; a link is a `Link` element inside its parent.
//! - A list is `L` (with `/ListNumbering`), each item `LI` holding `Lbl` (the marker) and
//!   `LBody` (the item text).
//! - A table is `Table` > `TR` > `TH` (the leading header rows, `/Scope /Column`) or `TD`, with
//!   `/ColSpan` for spanning cells; every cell exists even when it is empty.
//! - A table of contents is `TOC` > `TOCI` > `Link`, the link holding the title, the number and
//!   the annotations of the entry.
//! - Everything else is an artifact: rectangles (cell rules, highlights), dot leaders, rows of a
//!   table header repeated on a continuation page, and the content of headers and footers (marked
//!   as pagination artifacts).

use crate::content::ArtifactKind;
use crate::tags::{Attr, ListNumbering, StructElem, StructKid, TableAttrs, TableScope};
use moonshot_folio_layout::{BlockKind, Item, ItemKind, NodeId, PageTree, StructRole};
use std::collections::BTreeMap;

/// How an item is marked in the content stream.
pub(crate) enum Mark {
    /// Part of the page that is not logical content.
    Artifact(ArtifactKind),
    /// Content of a structure element (index) whose type is the tag.
    Tagged(usize, String),
}

enum Kid {
    Node(usize),
    Mcr(usize, u32),
    Annot(usize, usize),
}

struct Node {
    tag: String,
    attrs: Vec<Attr>,
    kids: Vec<Kid>,
}

#[derive(Default)]
struct ItemNode {
    li: usize,
    lbl: Option<usize>,
    body: Option<usize>,
}

#[derive(Default)]
struct BlockCtx {
    elem: usize,
    items: BTreeMap<u32, ItemNode>,
    rows: BTreeMap<u32, (usize, BTreeMap<u32, usize>)>,
    entries: BTreeMap<u32, usize>,
}

/// Builds the tree. Items must be offered in page order and paint order.
pub(crate) struct Builder<'a> {
    kinds: BTreeMap<(u32, u32), &'a BlockKind>,
    nodes: Vec<Node>,
    sects: BTreeMap<u32, usize>,
    blocks: BTreeMap<(u32, u32), BlockCtx>,
    links: BTreeMap<NodeId, usize>,
}

/// `(section, block)` of a body node, or `None` for anything else.
fn key(node: &NodeId) -> Option<(u32, u32)> {
    match node.path() {
        [s, 0, b, ..] => Some((*s, *b)),
        _ => None,
    }
}

impl<'a> Builder<'a> {
    pub(crate) fn new(tree: &'a PageTree) -> Self {
        let kinds = tree
            .blocks
            .iter()
            .filter_map(|b| key(&b.node).map(|k| (k, &b.kind)))
            .collect();
        Self {
            kinds,
            nodes: vec![Node {
                tag: "Document".to_string(),
                attrs: Vec::new(),
                kids: Vec::new(),
            }],
            sects: BTreeMap::new(),
            blocks: BTreeMap::new(),
            links: BTreeMap::new(),
        }
    }

    fn node(&mut self, tag: &str, attrs: Vec<Attr>, parent: usize) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Node {
            tag: tag.to_string(),
            attrs,
            kids: Vec::new(),
        });
        if let Some(p) = self.nodes.get_mut(parent) {
            p.kids.push(Kid::Node(idx));
        }
        idx
    }

    fn sect(&mut self, section: u32) -> usize {
        if let Some(s) = self.sects.get(&section) {
            return *s;
        }
        let s = self.node("Sect", Vec::new(), 0);
        self.sects.insert(section, s);
        s
    }

    /// The element of a block, created on first use.
    fn block(&mut self, k: (u32, u32)) -> usize {
        if let Some(b) = self.blocks.get(&k) {
            return b.elem;
        }
        let (tag, attrs) = match self.kinds.get(&k).copied() {
            Some(BlockKind::Heading(l)) => (format!("H{l}"), Vec::new()),
            Some(BlockKind::List { ordered }) => {
                let n = if *ordered {
                    ListNumbering::Decimal
                } else {
                    ListNumbering::Disc
                };
                ("L".to_string(), vec![Attr::List(n)])
            }
            Some(BlockKind::Table(_)) => ("Table".to_string(), Vec::new()),
            Some(BlockKind::Toc) => ("TOC".to_string(), Vec::new()),
            Some(BlockKind::Paragraph) | None => ("P".to_string(), Vec::new()),
        };
        let parent = self.sect(k.0);
        let elem = self.node(&tag, attrs, parent);
        self.blocks.insert(
            k,
            BlockCtx {
                elem,
                ..BlockCtx::default()
            },
        );
        elem
    }

    /// The `Link` element for `node` inside `parent`.
    fn link_in(&mut self, parent: usize, node: NodeId) -> usize {
        if let Some(l) = self.links.get(&node) {
            return *l;
        }
        let l = self.node("Link", Vec::new(), parent);
        self.links.insert(node, l);
        l
    }

    fn list_item(&mut self, k: (u32, u32), item: u32) -> usize {
        let l = self.block(k);
        if let Some(it) = self.blocks.get(&k).and_then(|b| b.items.get(&item)) {
            return it.li;
        }
        let li = self.node("LI", Vec::new(), l);
        if let Some(b) = self.blocks.get_mut(&k) {
            b.items.insert(
                item,
                ItemNode {
                    li,
                    ..ItemNode::default()
                },
            );
        }
        li
    }

    fn list_part(&mut self, k: (u32, u32), item: u32, label: bool) -> usize {
        let li = self.list_item(k, item);
        let existing = self
            .blocks
            .get(&k)
            .and_then(|b| b.items.get(&item))
            .and_then(|i| if label { i.lbl } else { i.body });
        if let Some(e) = existing {
            return e;
        }
        let e = self.node(if label { "Lbl" } else { "LBody" }, Vec::new(), li);
        if let Some(i) = self.blocks.get_mut(&k).and_then(|b| b.items.get_mut(&item)) {
            if label {
                i.lbl = Some(e);
            } else {
                i.body = Some(e);
            }
        }
        e
    }

    /// The cell element at `(row, col)` of a table, with its row, created on first use.
    fn cell(&mut self, k: (u32, u32), row: u32, col: u32) -> usize {
        let table = self.block(k);
        let known = self
            .blocks
            .get(&k)
            .and_then(|b| b.rows.get(&row))
            .map(|(tr, cells)| (*tr, cells.get(&col).copied()));
        let (tr, found) = if let Some(x) = known {
            x
        } else {
            let tr = self.node("TR", Vec::new(), table);
            if let Some(b) = self.blocks.get_mut(&k) {
                b.rows.insert(row, (tr, BTreeMap::new()));
            }
            (tr, None)
        };
        if let Some(c) = found {
            return c;
        }
        let (header, span) = match self.kinds.get(&k).copied() {
            Some(BlockKind::Table(t)) => (
                row < t.header_rows,
                t.spans
                    .get(usize::try_from(row).unwrap_or(usize::MAX))
                    .and_then(|r| r.get(usize::try_from(col).unwrap_or(usize::MAX)))
                    .copied()
                    .unwrap_or(1),
            ),
            _ => (false, 1),
        };
        let attrs = vec![Attr::Table(TableAttrs {
            scope: header.then_some(TableScope::Column),
            col_span: span,
            row_span: 1,
        })];
        let c = self.node(if header { "TH" } else { "TD" }, attrs, tr);
        if let Some((_, cells)) = self.blocks.get_mut(&k).and_then(|b| b.rows.get_mut(&row)) {
            cells.insert(col, c);
        }
        c
    }

    /// Decide how `it` is marked, creating the structure elements that own it.
    pub(crate) fn classify(&mut self, it: &Item) -> Mark {
        let p = it.node.path();
        if let Some(area @ (1 | 2)) = p.get(1).copied() {
            return Mark::Artifact(if area == 1 {
                ArtifactKind::Header
            } else {
                ArtifactKind::Footer
            });
        }
        let Some(k) = key(&it.node) else {
            return Mark::Artifact(ArtifactKind::Plain);
        };
        let is_text = matches!(it.kind, ItemKind::Text(_));
        if it.repeated || !is_text || it.role == StructRole::Artifact {
            // A table cell's rectangle makes sure the cell exists, empty or not.
            if !it.repeated && !is_text && matches!(self.kinds.get(&k), Some(BlockKind::Table(_))) {
                if let [_, _, _, row, col] = *p {
                    self.cell(k, row, col);
                }
            }
            return Mark::Artifact(ArtifactKind::Plain);
        }
        let (owner, tag) = self.owner(k, p, it);
        Mark::Tagged(owner, tag)
    }

    fn owner(&mut self, k: (u32, u32), p: &[u32], it: &Item) -> (usize, String) {
        let link = it.role == StructRole::Link;
        match self.kinds.get(&k).copied() {
            Some(BlockKind::List { .. }) => {
                let item = p.get(3).copied().unwrap_or(0);
                if p.len() <= 4 {
                    return (self.list_part(k, item, true), "Lbl".to_string());
                }
                let body = self.list_part(k, item, false);
                self.maybe_link(body, "LBody", link, it.node)
            }
            Some(BlockKind::Table(_)) => {
                let (row, col) = (
                    p.get(3).copied().unwrap_or(0),
                    p.get(4).copied().unwrap_or(0),
                );
                let c = self.cell(k, row, col);
                let tag = self
                    .nodes
                    .get(c)
                    .map_or("TD", |n| n.tag.as_str())
                    .to_string();
                (c, tag)
            }
            Some(BlockKind::Toc) => {
                let entry = p.get(3).copied().unwrap_or(0);
                let toc = self.block(k);
                let known = self
                    .blocks
                    .get(&k)
                    .and_then(|b| b.entries.get(&entry))
                    .copied();
                let toci = if let Some(t) = known {
                    t
                } else {
                    let t = self.node("TOCI", Vec::new(), toc);
                    if let Some(b) = self.blocks.get_mut(&k) {
                        b.entries.insert(entry, t);
                    }
                    t
                };
                (self.link_in(toci, it.node), "Link".to_string())
            }
            other => {
                let tag = match other {
                    Some(BlockKind::Heading(l)) => format!("H{l}"),
                    _ => "P".to_string(),
                };
                let b = self.block(k);
                self.maybe_link(b, &tag, link, it.node)
            }
        }
    }

    fn maybe_link(
        &mut self,
        parent: usize,
        tag: &str,
        link: bool,
        node: NodeId,
    ) -> (usize, String) {
        if link {
            (self.link_in(parent, node), "Link".to_string())
        } else {
            (parent, tag.to_string())
        }
    }

    /// Record the marked-content id of content owned by `owner`.
    pub(crate) fn content(&mut self, owner: usize, page: usize, mcid: u32) {
        if let Some(n) = self.nodes.get_mut(owner) {
            n.kids.push(Kid::Mcr(page, mcid));
        }
    }

    /// Attach the link annotations and return the tree. A link in a header or footer has no
    /// content of its own in the structure (its text is an artifact), so each such annotation
    /// gets a bare `Link` element, collected in one `Div` at the end of the document.
    pub(crate) fn finish(mut self, tree: &PageTree) -> StructElem {
        let mut chrome_div: Option<usize> = None;
        for (pi, page) in tree.pages.iter().enumerate() {
            for (li, l) in page.links.iter().enumerate() {
                let in_body = l.node.path().get(1) == Some(&0);
                let known = if in_body {
                    self.links.get(&l.node).copied()
                } else {
                    None
                };
                let owner = if let Some(o) = known {
                    o
                } else {
                    let div = if let Some(d) = chrome_div {
                        d
                    } else {
                        let d = self.node("Div", Vec::new(), 0);
                        chrome_div = Some(d);
                        d
                    };
                    self.node("Link", Vec::new(), div)
                };
                if let Some(n) = self.nodes.get_mut(owner) {
                    n.kids.push(Kid::Annot(pi, li));
                }
            }
        }
        self.convert(0).into_iter().next().unwrap_or(StructElem {
            tag: "Document".to_string(),
            attrs: Vec::new(),
            kids: Vec::new(),
        })
    }

    /// The element for `idx`; a list with too many items becomes several `L` elements.
    fn convert(&self, idx: usize) -> Vec<StructElem> {
        let Some(n) = self.nodes.get(idx) else {
            return Vec::new();
        };
        let mut kids: Vec<StructKid> = Vec::new();
        for k in &n.kids {
            match k {
                Kid::Node(c) => kids.extend(self.convert(*c).into_iter().map(StructKid::Elem)),
                Kid::Mcr(page, mcid) => kids.push(StructKid::Content {
                    page: *page,
                    mcid: *mcid,
                }),
                Kid::Annot(page, index) => kids.push(StructKid::Annot {
                    page: *page,
                    index: *index,
                }),
            }
        }
        let make = |tag: &str, kids: Vec<StructKid>| StructElem {
            tag: tag.to_string(),
            attrs: n.attrs.clone(),
            kids,
        };
        if n.tag == "L" && kids.len() > GROUP {
            return kids.chunks(GROUP).map(|c| make("L", c.to_vec())).collect();
        }
        vec![make(&n.tag, group(&n.tag, kids))]
    }
}

/// Most children of one element: well under the 8191 elements a PDF array may hold.
const GROUP: usize = 4096;

/// The grouping element for the children of a `tag` element with too many of them: the least
/// odd standard type for the parent (`Part` for the document, `TBody` for a table, a nested
/// `TOC`, `Div` for sections and other blocks, `Span` inside text).
fn wrapper(tag: &str) -> &'static str {
    match tag {
        "Document" | "Part" => "Part",
        "Table" => "TBody",
        "TOC" => "TOC",
        "Sect" | "Div" | "TBody" => "Div",
        _ => "Span",
    }
}

/// Group `kids` into wrappers while there are more than [`GROUP`] of them.
fn group(tag: &str, mut kids: Vec<StructKid>) -> Vec<StructKid> {
    while kids.len() > GROUP {
        kids = kids
            .chunks(GROUP)
            .map(|c| {
                StructKid::Elem(StructElem {
                    tag: wrapper(tag).to_string(),
                    attrs: Vec::new(),
                    kids: c.to_vec(),
                })
            })
            .collect();
    }
    kids
}

/// The text of every link inline, for the annotations' `/Contents`: items of one node in order,
/// lines joined by a space where the line ended at a dropped space, and a space where the next
/// item does not start where the last one ended (a contents entry's number).
pub(crate) fn link_texts(tree: &PageTree) -> BTreeMap<NodeId, String> {
    let mut out: BTreeMap<NodeId, String> = BTreeMap::new();
    let mut last: BTreeMap<NodeId, (usize, i64, i64, bool)> = BTreeMap::new();
    for (pi, page) in tree.pages.iter().enumerate() {
        for it in &page.items {
            let ItemKind::Text(t) = &it.kind else {
                continue;
            };
            if it.role != StructRole::Link {
                continue;
            }
            let s = out.entry(it.node).or_default();
            if let Some((lp, ly, lend, space)) = last.get(&it.node) {
                let new_line = (*lp, *ly) != (pi, it.y.raw());
                if (new_line && *space) || (!new_line && it.x.raw() > *lend) {
                    s.push(' ');
                }
            }
            s.push_str(&t.text);
            last.insert(
                it.node,
                (
                    pi,
                    it.y.raw(),
                    it.x.raw().saturating_add(t.width.raw()),
                    t.space_after,
                ),
            );
        }
    }
    out
}
