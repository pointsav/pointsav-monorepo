// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Logical structure (tagged PDF): the structure tree model, its checks and its objects
//! (`StructTreeRoot`, structure elements, the parent tree).

use crate::doc::PdfDoc;
use crate::objects::Objects;
use crate::syntax::name;
use crate::trees::{write_tree, Entry};
use core::fmt::Write;
use moonshot_folio_core::{FolioError, Result};
use std::collections::{BTreeMap, BTreeSet};

/// The scope of a table header cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableScope {
    /// Heads a column.
    Column,
    /// Heads a row.
    Row,
    /// Heads both.
    Both,
}

/// Table attributes of a cell (`/O /Table`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableAttrs {
    /// `/Scope`, for header cells.
    pub scope: Option<TableScope>,
    /// `/ColSpan`, written when above 1.
    pub col_span: u32,
    /// `/RowSpan`, written when above 1.
    pub row_span: u32,
}

/// List numbering (`/O /List`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListNumbering {
    /// Bulleted.
    Disc,
    /// Numbered with decimal numbers.
    Decimal,
}

/// An attribute object of a structure element.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attr {
    /// Table attributes.
    Table(TableAttrs),
    /// List attributes.
    List(ListNumbering),
}

/// A child of a structure element.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StructKid {
    /// A child element.
    Elem(StructElem),
    /// Marked content on a page, by its marked-content id.
    Content {
        /// Zero-based page index.
        page: usize,
        /// The id given by `Content::begin_tagged`.
        mcid: u32,
    },
    /// A link annotation (written as an object reference), by page and position in the page's
    /// link list.
    Annot {
        /// Zero-based page index.
        page: usize,
        /// Index into the links given to `add_page`.
        index: usize,
    },
}

/// A structure element: a standard structure type, its attributes and its children.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructElem {
    /// The structure type (`Document`, `P`, `H1`, `Table`, ...).
    pub tag: String,
    /// Attribute objects.
    pub attrs: Vec<Attr>,
    /// Children in reading order.
    pub kids: Vec<StructKid>,
}

/// The keys the page dictionaries and annotations carry (`/StructParents`, `/StructParent`).
#[derive(Debug, Default)]
pub(crate) struct TagKeys {
    pub(crate) enabled: bool,
    pub(crate) page: Vec<Option<usize>>,
    pub(crate) annot: BTreeMap<(usize, usize), usize>,
}

/// Structure elements allowed per unit of `max_cells`.
const STRUCT_ELEMENTS_PER_CELL: usize = 4;

/// Levels added to `max_depth` for the structure tree.
const STRUCT_DEPTH: usize = 8;

fn count(e: &StructElem, depth: usize, max_depth: usize, n: &mut usize) -> Result<()> {
    if depth > max_depth {
        return Err(FolioError::Limit("structure depth"));
    }
    *n = n.saturating_add(1);
    for k in &e.kids {
        if let StructKid::Elem(c) = k {
            count(c, depth.saturating_add(1), max_depth, n)?;
        }
    }
    Ok(())
}

fn leaves<'a>(e: &'a StructElem, out: &mut Vec<&'a StructKid>) {
    for k in &e.kids {
        match k {
            StructKid::Elem(c) => leaves(c, out),
            other => out.push(other),
        }
    }
}

impl PdfDoc {
    /// Set the logical structure: the root element (normally `Document`). Page, marked-content
    /// and annotation references are checked by `build`.
    ///
    /// # Errors
    /// `Limit` when the tree has more than four times `max_cells` elements or is deeper than
    /// `max_depth` plus 8 levels (the structure has bounds of its own, wider than the ones that
    /// apply to the content); `Invalid` for an empty tag.
    pub fn set_structure(&mut self, root: StructElem) -> Result<()> {
        // The structure has bounds of its own: an element for every few characters of content
        // and a fixed allowance of levels for the types themselves (Document, Sect, Table, TR,
        // TD, ...), so tagging fails only where the input is far beyond what untagged output
        // accepts. The hard bounds stay.
        let mut n = 0_usize;
        count(
            &root,
            1,
            self.limits.max_depth().saturating_add(STRUCT_DEPTH),
            &mut n,
        )?;
        if n > self
            .limits
            .max_cells()
            .saturating_mul(STRUCT_ELEMENTS_PER_CELL)
        {
            return Err(FolioError::Limit("structure elements"));
        }
        let mut stack = vec![&root];
        while let Some(e) = stack.pop() {
            if e.tag.is_empty() {
                return Err(FolioError::Invalid("empty structure tag"));
            }
            stack.extend(e.kids.iter().filter_map(|k| match k {
                StructKid::Elem(c) => Some(c),
                _ => None,
            }));
        }
        self.structure = Some(root);
        Ok(())
    }

    /// Check every reference of the structure tree and assign the parent-tree keys.
    pub(crate) fn tag_keys(&self) -> Result<TagKeys> {
        let Some(root) = &self.structure else {
            if self.pages.iter().any(|p| p.mcids > 0) {
                return Err(FolioError::Invalid(
                    "marked content without a structure tree",
                ));
            }
            return Ok(TagKeys::default());
        };
        let mut all = Vec::new();
        leaves(root, &mut all);
        let mut seen: BTreeSet<(usize, u32)> = BTreeSet::new();
        let mut annots: BTreeSet<(usize, usize)> = BTreeSet::new();
        let bad = FolioError::Invalid("structure tree refers to missing content");
        for k in all {
            match k {
                StructKid::Content { page, mcid } => {
                    let p = self.pages.get(*page).ok_or(bad)?;
                    if *mcid >= p.mcids || !seen.insert((*page, *mcid)) {
                        return Err(bad);
                    }
                }
                StructKid::Annot { page, index } => {
                    let p = self.pages.get(*page).ok_or(bad)?;
                    if *index >= p.links.len() || !annots.insert((*page, *index)) {
                        return Err(bad);
                    }
                }
                StructKid::Elem(_) => {}
            }
        }
        let total: usize = self.pages.iter().map(|p| p.mcids as usize).sum();
        if seen.len() != total {
            return Err(FolioError::Invalid(
                "marked content is not referenced by the structure tree",
            ));
        }
        let pages = self.pages.len();
        let page = self
            .pages
            .iter()
            .enumerate()
            .map(|(i, p)| (p.mcids > 0).then_some(i))
            .collect();
        let annot = annots
            .into_iter()
            .enumerate()
            .map(|(rank, k)| (k, pages.saturating_add(rank)))
            .collect();
        Ok(TagKeys {
            enabled: true,
            page,
            annot,
        })
    }

    /// Write the structure tree objects; returns the `StructTreeRoot` object number.
    pub(crate) fn write_structure(
        &self,
        o: &mut Objects,
        page_ids: &[usize],
        annot_ids: &BTreeMap<(usize, usize), usize>,
        keys: &TagKeys,
    ) -> Result<usize> {
        let root = self
            .structure
            .as_ref()
            .ok_or(FolioError::Invalid("no structure"))?;
        let tree = o.alloc()?;
        let mut cx = Cx {
            page_ids,
            annot_ids,
            parents: self
                .pages
                .iter()
                .map(|p| vec![None; p.mcids as usize])
                .collect(),
            annot_parent: BTreeMap::new(),
        };
        let doc = write_elem(o, root, tree, &mut cx)?;
        let mut entries: Vec<Entry> = Vec::new();
        for (i, p) in cx.parents.iter().enumerate() {
            if keys.page.get(i).copied().flatten().is_none() {
                continue;
            }
            let mut arr = String::new();
            for e in p {
                let id = e.ok_or(FolioError::Invalid("marked content without a parent"))?;
                write!(arr, "{id} 0 R ").ok();
            }
            entries.push((
                i.to_string().into_bytes(),
                format!("{i} [{arr}] ").into_bytes(),
            ));
        }
        let next = self.pages.len().saturating_add(keys.annot.len());
        for (k, key) in &keys.annot {
            let id = cx
                .annot_parent
                .get(k)
                .ok_or(FolioError::Invalid("annotation without a parent"))?;
            entries.push((
                key.to_string().into_bytes(),
                format!("{key} {id} 0 R ").into_bytes(),
            ));
        }
        let pt = write_tree(o, "Nums", &entries)?;
        o.set(
            tree,
            format!(
                "<< /Type /StructTreeRoot /K {doc} 0 R /ParentTree {pt} 0 R /ParentTreeNextKey {next} >>"
            )
            .into_bytes(),
        )?;
        Ok(tree)
    }
}

struct Cx<'a> {
    page_ids: &'a [usize],
    annot_ids: &'a BTreeMap<(usize, usize), usize>,
    parents: Vec<Vec<Option<usize>>>,
    annot_parent: BTreeMap<(usize, usize), usize>,
}

fn attrs_text(attrs: &[Attr]) -> String {
    let one = |a: &Attr| -> String {
        match a {
            Attr::Table(t) => {
                let mut s = String::from("<< /O /Table");
                match t.scope {
                    Some(TableScope::Column) => s.push_str(" /Scope /Column"),
                    Some(TableScope::Row) => s.push_str(" /Scope /Row"),
                    Some(TableScope::Both) => s.push_str(" /Scope /Both"),
                    None => {}
                }
                if t.col_span > 1 {
                    write!(s, " /ColSpan {}", t.col_span).ok();
                }
                if t.row_span > 1 {
                    write!(s, " /RowSpan {}", t.row_span).ok();
                }
                s.push_str(" >>");
                s
            }
            Attr::List(n) => format!(
                "<< /O /List /ListNumbering {} >>",
                match n {
                    ListNumbering::Disc => "/Disc",
                    ListNumbering::Decimal => "/Decimal",
                }
            ),
        }
    };
    match attrs {
        [] => String::new(),
        [a] => format!(" /A {}", one(a)),
        many => {
            let inner: Vec<String> = many.iter().map(one).collect();
            format!(" /A [{}]", inner.join(" "))
        }
    }
}

fn write_elem(o: &mut Objects, e: &StructElem, parent: usize, cx: &mut Cx<'_>) -> Result<usize> {
    let id = o.alloc()?;
    let mut kids = String::new();
    let missing = FolioError::Invalid("structure tree refers to missing content");
    for k in &e.kids {
        match k {
            StructKid::Elem(c) => {
                let cid = write_elem(o, c, id, cx)?;
                write!(kids, "{cid} 0 R ").ok();
            }
            StructKid::Content { page, mcid } => {
                let pg = cx.page_ids.get(*page).ok_or(missing)?;
                let slot = cx
                    .parents
                    .get_mut(*page)
                    .and_then(|p| p.get_mut(*mcid as usize))
                    .ok_or(missing)?;
                *slot = Some(id);
                write!(kids, "<< /Type /MCR /Pg {pg} 0 R /MCID {mcid} >> ").ok();
            }
            StructKid::Annot { page, index } => {
                let pg = cx.page_ids.get(*page).ok_or(missing)?;
                let obj = cx.annot_ids.get(&(*page, *index)).ok_or(missing)?;
                cx.annot_parent.insert((*page, *index), id);
                write!(kids, "<< /Type /OBJR /Pg {pg} 0 R /Obj {obj} 0 R >> ").ok();
            }
        }
    }
    let k = if e.kids.is_empty() {
        String::new()
    } else {
        format!(" /K [{kids}]")
    };
    let body = format!(
        "<< /Type /StructElem /S {} /P {parent} 0 R{k}{} >>",
        name(&e.tag),
        attrs_text(&e.attrs)
    );
    o.set(id, body.into_bytes())?;
    Ok(id)
}
