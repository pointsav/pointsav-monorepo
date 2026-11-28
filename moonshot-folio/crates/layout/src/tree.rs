// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The `PageTree`: pages of positioned draw items, link annotations and named destinations.
//!
//! Coordinates in a finished tree use the PDF convention (origin at the bottom-left corner of
//! the page, y upwards, [`Emu`]), so a backend can pass them to a content stream unchanged. A
//! text item's `(x, y)` is its baseline origin; a rectangle's is its lower-left corner.

use moonshot_folio_core::{Emu, Ink, Result};

/// Which part of a section a node belongs to (the second component of a [`NodeId`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Area {
    /// The section's `blocks`.
    Body = 0,
    /// The section's `header`.
    Header = 1,
    /// The section's `footer`.
    Footer = 2,
}

/// A stable index path into the IR: `[section, area, block, ...]`. The next components depend
/// on the block: an inline index for a paragraph or heading, an item then an inline index for a
/// list, a row then a cell index for a table. Every prefix names an ancestor (the table is
/// `[s, a, b]` and its row `[s, a, b, r]`), so a tagged-PDF writer can rebuild the tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId {
    path: [u32; 5],
    len: u8,
}

fn index(i: usize) -> u32 {
    u32::try_from(i).unwrap_or(u32::MAX)
}

impl NodeId {
    /// The block at `block` in `area` of section `section`.
    #[must_use]
    pub fn block(section: usize, area: Area, block: usize) -> Self {
        Self {
            path: [index(section), area as u32, index(block), 0, 0],
            len: 3,
        }
    }

    /// The child at `i`. A node already five components deep is returned unchanged.
    #[must_use]
    pub fn child(self, i: usize) -> Self {
        let mut n = self;
        if let Some(slot) = n.path.get_mut(usize::from(n.len)) {
            *slot = index(i);
            n.len = n.len.saturating_add(1);
        }
        n
    }

    /// The path components.
    #[must_use]
    pub fn path(&self) -> &[u32] {
        self.path.get(..usize::from(self.len)).unwrap_or(&[])
    }

    /// The parent node, or `None` for a block.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        (self.len > 3).then(|| Self {
            path: self.path,
            len: self.len.saturating_sub(1),
        })
    }
}

/// The structure role of an item or node, for tagged PDF.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StructRole {
    /// Body text of a paragraph.
    Paragraph,
    /// A heading of the given level, 1 to 6.
    Heading(u8),
    /// A list item (its marker and its text).
    ListItem,
    /// A table: the role of the `[s, a, b]` node.
    Table,
    /// A table row: the role of the `[s, a, b, r]` node.
    TableRow,
    /// A table cell (its text).
    TableCell,
    /// Content of a section header.
    Header,
    /// Content of a section footer.
    Footer,
    /// A hyperlink (its text).
    Link,
    /// Decoration outside the logical structure (cell borders, highlight rectangles).
    Artifact,
}

/// Which of the two font faces an item uses; the backend maps it to its own font handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FontSlot {
    /// The regular face.
    Regular,
    /// The bold face.
    Bold,
}

/// A run of text on one baseline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextItem {
    /// The face.
    pub font: FontSlot,
    /// Font size.
    pub size: Emu,
    /// Fill colour.
    pub ink: Ink,
    /// Advance width of `text` as laid out.
    pub width: Emu,
    /// Height of the em box above the baseline.
    pub ascent: Emu,
    /// Depth of the em box below the baseline.
    pub descent: Emu,
    /// The characters to show; trailing spaces and line terminators are not included.
    pub text: String,
    /// True when the line this item ends was broken at a space or line terminator that is not
    /// drawn. Text of consecutive lines is joined with one space only then; a line broken inside
    /// a word, after a hyphen or in a long address is joined with none.
    pub space_after: bool,
}

/// A filled and/or stroked rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RectItem {
    /// Width.
    pub w: Emu,
    /// Height.
    pub h: Emu,
    /// Fill colour, if filled.
    pub fill: Option<Ink>,
    /// Stroke colour and width, if stroked.
    pub stroke: Option<(Ink, Emu)>,
}

/// What an item draws.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemKind {
    /// Text.
    Text(TextItem),
    /// A rectangle.
    Rect(RectItem),
}

/// One positioned draw item. Items are in paint order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    /// The IR node it came from.
    pub node: NodeId,
    /// Its structure role.
    pub role: StructRole,
    /// Horizontal position (see the module notes).
    pub x: Emu,
    /// Vertical position (see the module notes).
    pub y: Emu,
    /// What to draw.
    pub kind: ItemKind,
    /// True for an item of a table header row repeated on a continuation page, so tagged PDF
    /// can leave it out of the structure tree (it duplicates content shown earlier).
    pub repeated: bool,
}

/// A rectangle in page coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoxRect {
    /// Left edge.
    pub x: Emu,
    /// Bottom edge.
    pub y: Emu,
    /// Width.
    pub w: Emu,
    /// Height.
    pub h: Emu,
}

/// Where a link goes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkTarget {
    /// An external address, exactly as written in the IR (the backend checks the scheme).
    Uri(String),
    /// A named destination of this tree.
    Named(String),
}

/// A link annotation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkAnnot {
    /// The IR node of the link text.
    pub node: NodeId,
    /// The clickable area.
    pub rect: BoxRect,
    /// The target.
    pub target: LinkTarget,
}

/// A named destination: a position on a page, at the top of the line holding the anchor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedDest {
    /// The anchor name.
    pub name: String,
    /// Zero-based page index.
    pub page: usize,
    /// Horizontal position.
    pub x: Emu,
    /// Vertical position, from the bottom page edge.
    pub y: Emu,
}

/// One laid-out page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    /// Page width.
    pub width: Emu,
    /// Page height.
    pub height: Emu,
    /// Margin on all four sides; every item lies inside the margin box.
    pub margin: Emu,
    /// Zero-based index of the IR section.
    pub section: usize,
    /// The page number shown by a page field (restarts when the section asks for it).
    pub label: u32,
    /// Draw items in paint order.
    pub items: Vec<Item>,
    /// Link annotations.
    pub links: Vec<LinkAnnot>,
}

impl Page {
    /// The margin box (content area) in page coordinates.
    ///
    /// # Errors
    /// `Limit` on length overflow.
    pub fn content_box(&self) -> Result<BoxRect> {
        let twice = self.margin.checked_mul(2)?;
        Ok(BoxRect {
            x: self.margin,
            y: self.margin,
            w: self.width.checked_sub(twice)?,
            h: self.height.checked_sub(twice)?,
        })
    }
}

/// The result of layout.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageTree {
    /// Pages in order.
    pub pages: Vec<Page>,
    /// Named destinations in document order, unique by name.
    pub dests: Vec<NamedDest>,
    /// Characters laid out with a substitute width because the font lacks them (always 0 under
    /// `MissingGlyph::Reject`, which fails instead).
    pub substituted: usize,
    /// The body blocks that were laid out, in document order, with their kind. Gives structure
    /// consumers (tagged PDF) what the item nodes alone cannot say: whether `[s, 0, b, x, y]` is a
    /// list item or a table cell, and which blocks are headings (also those made only of links).
    pub blocks: Vec<BlockInfo>,
}

/// What kind of IR block a body block was.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BlockKind {
    /// A paragraph, or a field shown as a paragraph.
    Paragraph,
    /// A heading of the given level, 1 to 6.
    Heading(u8),
    /// A list. Items are `[s, 0, b, item]` (the marker text) and `[s, 0, b, item, inline]`.
    List {
        /// Numbered (true) or bulleted (false).
        ordered: bool,
    },
    /// A table. Row `r`, cell `c` is `[s, 0, b, r, c]`.
    Table(TableInfo),
    /// A table of contents. Entry `e` is `[s, 0, b, e]`.
    Toc,
}

/// The grid of a table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableInfo {
    /// Leading rows that are header rows.
    pub header_rows: u32,
    /// The column span of every cell, per row.
    pub spans: Vec<Vec<u32>>,
}

/// A laid-out body block: its node (`[section, 0, block]`) and kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockInfo {
    /// The block's node.
    pub node: NodeId,
    /// What it was.
    pub kind: BlockKind,
}

impl Item {
    /// Move an item from fragment coordinates (origin at the top-left of the content box, y
    /// downwards, rectangles anchored at their top edge) to page coordinates.
    pub(crate) fn placed(&self, dx: Emu, dy: Emu, page_h: Emu) -> Result<Item> {
        let top = dy.checked_add(self.y)?;
        let y = match &self.kind {
            ItemKind::Text(_) => page_h.checked_sub(top)?,
            ItemKind::Rect(r) => page_h.checked_sub(top.checked_add(r.h)?)?,
        };
        Ok(Item {
            x: dx.checked_add(self.x)?,
            y,
            ..self.clone()
        })
    }
}

impl LinkAnnot {
    /// As [`Item::placed`], for a link rectangle anchored at its top edge.
    pub(crate) fn placed(&self, dx: Emu, dy: Emu, page_h: Emu) -> Result<LinkAnnot> {
        let top = dy.checked_add(self.rect.y)?;
        Ok(LinkAnnot {
            node: self.node,
            rect: BoxRect {
                x: dx.checked_add(self.rect.x)?,
                y: page_h.checked_sub(top.checked_add(self.rect.h)?)?,
                w: self.rect.w,
                h: self.rect.h,
            },
            target: self.target.clone(),
        })
    }
}
