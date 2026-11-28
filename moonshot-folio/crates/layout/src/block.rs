// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! IR blocks to pagination blocks.

use crate::ctx::{Blk, Ctx, Frag, RelDest};
use crate::headings::toc_dest_name;
use crate::inline::{field_seg, segs_of, InlineCx};
use crate::table::table_blk;
use crate::text::{layout_lines, Align, Geom};
use crate::theme::Style;
use crate::toc::toc_blks;
use crate::tree::{
    Area, BlockInfo, BlockKind, FontSlot, Item, ItemKind, NodeId, StructRole, TableInfo, TextItem,
};
use moonshot_folio_core::{Emu, FolioError, Result};
use moonshot_folio_ir::{Block, BreakKind, FieldKind, Heading, List, Pagination, Para};

/// What block conversion needs besides the block.
pub(crate) struct BlockCx<'a> {
    pub(crate) ctx: &'a Ctx<'a>,
    pub(crate) area: Area,
    /// The page number when known (headers and footers).
    pub(crate) label: Option<u32>,
    /// Width available to blocks.
    pub(crate) width: Emu,
}

impl BlockCx<'_> {
    fn role(&self, natural: StructRole) -> StructRole {
        match self.area {
            Area::Body => natural,
            Area::Header => StructRole::Header,
            Area::Footer => StructRole::Footer,
        }
    }

    pub(crate) fn base(&self) -> Style {
        let t = self.ctx.theme;
        Style {
            slot: FontSlot::Regular,
            size: if self.area == Area::Body {
                t.body_size
            } else {
                t.chrome_size
            },
            ink: t.text_ink,
        }
    }

    fn inlines(&self, role: StructRole) -> InlineCx<'_> {
        InlineCx {
            ctx: self.ctx,
            role: self.role(role),
            label: self.label,
        }
    }
}

pub(crate) fn left(x0: Emu, width: Emu) -> Geom {
    Geom {
        x0,
        width,
        align: Align::Left,
    }
}

/// Convert the blocks of one area of section `section`.
///
/// # Errors
/// `Unsupported` for images, a table of contents and page breaks in a header or footer; the
/// errors of text layout.
pub(crate) fn blocks_to_blks(
    bc: &BlockCx<'_>,
    section: usize,
    blocks: &[Block],
) -> Result<Vec<Blk>> {
    let mut out = Vec::new();
    for (bi, b) in blocks.iter().enumerate() {
        let node = NodeId::block(section, bc.area, bi);
        match b {
            Block::Para(p) => out.push(para_blk(bc, p, node)?),
            Block::Heading(h) => out.push(heading_blk(bc, h, node)?),
            Block::List(l) => list_blks(bc, l, node, &mut out)?,
            Block::Table(t) => {
                let blk = table_blk(
                    bc.ctx,
                    t,
                    node,
                    bc.width,
                    (bc.role(StructRole::TableCell), bc.base()),
                )?;
                if !blk.frags.is_empty() {
                    let kind = BlockKind::Table(TableInfo {
                        header_rows: t.repeat_header_rows,
                        spans: t
                            .rows
                            .iter()
                            .map(|r| r.cells.iter().map(|c| c.span).collect())
                            .collect(),
                    });
                    out.push(Blk {
                        block: info(bc, node, kind),
                        ..blk
                    });
                }
            }
            Block::Field(FieldKind::Toc) if bc.area == Area::Body => {
                let mut blks = toc_blks(bc, node)?;
                if let Some(first) = blks.first_mut() {
                    first.block = info(bc, node, BlockKind::Toc);
                }
                out.extend(blks);
            }
            Block::Field(FieldKind::Toc) => {
                return Err(FolioError::Unsupported(
                    "table of contents in a header or footer",
                ));
            }
            Block::Field(k) => out.push(field_blk(bc, *k, node)?),
            Block::Break(k) => out.push(break_blk(bc, *k)?),
            Block::Image(_) => return Err(FolioError::Unsupported("images are not laid out")),
        }
    }
    Ok(out)
}

/// The block record for a body block (headers and footers are not recorded).
pub(crate) fn info(bc: &BlockCx<'_>, node: NodeId, kind: BlockKind) -> Option<BlockInfo> {
    (bc.area == Area::Body).then_some(BlockInfo { node, kind })
}

fn flags(blk: &mut Blk, p: Pagination, area: Area) {
    blk.keep_next = p.keep_with_next;
    blk.page_break_before = p.page_break_before && area == Area::Body;
    if p.widow_orphan {
        blk.min_head = 2;
        blk.min_tail = 2;
    }
}

fn para_blk(bc: &BlockCx<'_>, p: &Para, node: NodeId) -> Result<Blk> {
    let base = bc.ctx.theme.apply(&p.role, bc.base());
    let segs = segs_of(&bc.inlines(StructRole::Paragraph), &p.inlines, base, node)?;
    let frags = layout_lines(bc.ctx, &segs, left(Emu::ZERO, bc.width), base)?;
    let mut blk = Blk {
        after: bc.ctx.theme.para_after,
        frags,
        block: info(bc, node, BlockKind::Paragraph),
        ..Blk::default()
    };
    flags(&mut blk, p.pagination, bc.area);
    Ok(blk)
}

fn heading_blk(bc: &BlockCx<'_>, h: &Heading, node: NodeId) -> Result<Blk> {
    let t = bc.ctx.theme;
    let level = usize::from(h.level.clamp(1, 6));
    let size = t
        .heading_sizes
        .get(level.saturating_sub(1))
        .copied()
        .unwrap_or(t.body_size);
    let base = Style {
        slot: FontSlot::Bold,
        size,
        ink: t.text_ink,
    };
    let ic = bc.inlines(StructRole::Heading(h.level.clamp(1, 6)));
    let segs = segs_of(&ic, &h.inlines, base, node)?;
    let frags = layout_lines(bc.ctx, &segs, left(Emu::ZERO, bc.width), base)?;
    let mut frags = frags;
    // A heading with no visible text (empty, or only anchors) is neither listed in a table of
    // contents nor bookmarked, and gets no destination.
    let has_text = frags
        .iter()
        .flat_map(|f| &f.items)
        .any(|i| matches!(&i.kind, ItemKind::Text(t) if !t.text.trim().is_empty()));
    if bc.ctx.toc_on && bc.area == Area::Body && has_text {
        if let Some(first) = frags.first_mut() {
            first.dests.push(RelDest {
                name: toc_dest_name(&node),
                x: Emu::ZERO,
                y: Emu::ZERO,
            });
        }
    }
    let mut blk = Blk {
        before: t.heading_before,
        after: t.heading_after,
        frags,
        keep_together: true,
        block: info(bc, node, BlockKind::Heading(h.level.clamp(1, 6))),
        ..Blk::default()
    };
    flags(&mut blk, h.pagination, bc.area);
    blk.keep_next = true;
    Ok(blk)
}

fn field_blk(bc: &BlockCx<'_>, kind: FieldKind, node: NodeId) -> Result<Blk> {
    let base = bc.base();
    let seg = field_seg(&bc.inlines(StructRole::Paragraph), kind, base, node)?;
    let frags = layout_lines(bc.ctx, &[seg], left(Emu::ZERO, bc.width), base)?;
    Ok(Blk {
        after: bc.ctx.theme.para_after,
        frags,
        block: info(bc, node, BlockKind::Paragraph),
        ..Blk::default()
    })
}

fn break_blk(bc: &BlockCx<'_>, kind: BreakKind) -> Result<Blk> {
    match kind {
        BreakKind::Page if bc.area == Area::Body => Ok(Blk {
            page_break_before: true,
            ..Blk::default()
        }),
        BreakKind::Page => Err(FolioError::Unsupported("page break in header or footer")),
        BreakKind::Line => Ok(Blk {
            frags: layout_lines(bc.ctx, &[], left(Emu::ZERO, bc.width), bc.base())?,
            ..Blk::default()
        }),
    }
}

fn marker_text(bc: &BlockCx<'_>, ordered: bool, n: usize) -> String {
    if ordered {
        format!("{n}.")
    } else if bc.ctx.fonts.has(FontSlot::Regular, '\u{2022}') {
        "\u{2022}".to_string()
    } else {
        "-".to_string()
    }
}

/// Width of `text` in the regular face, and how many characters use a substitute width.
fn text_width(bc: &BlockCx<'_>, text: &str, size: Emu) -> Result<(Emu, usize)> {
    let (mut w, mut subst) = (Emu::ZERO, 0_usize);
    for c in text.chars() {
        let (cw, sub) =
            bc.ctx
                .fonts
                .width(FontSlot::Regular, c, size, bc.ctx.opts.missing_glyph)?;
        subst = subst.saturating_add(usize::from(sub));
        w = w.checked_add(cw)?;
    }
    Ok((w, subst))
}

fn list_blks(bc: &BlockCx<'_>, l: &List, node: NodeId, out: &mut Vec<Blk>) -> Result<()> {
    let t = bc.ctx.theme;
    let base = bc.base();
    let markers: Vec<String> = (1..=l.items.len())
        .map(|n| marker_text(bc, l.ordered, n))
        .collect();
    let mut widest = Emu::ZERO;
    for m in &markers {
        widest = widest.max(text_width(bc, m, base.size)?.0);
    }
    let indent = t.list_indent.max(widest.checked_add(t.marker_gap)?);
    let width = bc.width.checked_sub(indent)?;
    if width <= Emu::ZERO {
        return Err(FolioError::Invalid("list indent leaves no room for text"));
    }
    let last = l.items.len().saturating_sub(1);
    for (i, (inl, marker)) in l.items.iter().zip(&markers).enumerate() {
        let n = node.child(i);
        let ic = bc.inlines(StructRole::ListItem);
        let segs = segs_of(&ic, inl, base, n)?;
        let mut frags = layout_lines(bc.ctx, &segs, left(indent, width), base)?;
        let (mw, subst) = text_width(bc, marker, base.size)?;
        let x = indent.checked_sub(t.marker_gap)?.checked_sub(mw)?;
        add_marker(bc, frags.first_mut(), marker, (x, mw, subst), n, base)?;
        out.push(Blk {
            after: if i == last {
                t.para_after
            } else {
                t.list_item_after
            },
            frags,
            min_head: 2,
            min_tail: 2,
            block: (i == 0)
                .then(|| info(bc, node, BlockKind::List { ordered: l.ordered }))
                .flatten(),
            ..Blk::default()
        });
    }
    Ok(())
}

/// Put the marker text first in the first line of a list item.
fn add_marker(
    bc: &BlockCx<'_>,
    first: Option<&mut Frag>,
    text: &str,
    (x, w, subst): (Emu, Emu, usize),
    node: NodeId,
    base: Style,
) -> Result<()> {
    let Some(f) = first else { return Ok(()) };
    let (asc, desc) = bc.ctx.fonts.extents(base.slot, base.size)?;
    let item = Item {
        node,
        role: bc.role(StructRole::ListItem),
        x,
        y: f.baseline,
        kind: ItemKind::Text(TextItem {
            font: base.slot,
            size: base.size,
            ink: base.ink,
            width: w,
            ascent: asc,
            descent: desc,
            text: text.to_string(),
            space_after: false,
        }),
        repeated: false,
    };
    f.items.insert(0, item);
    f.subst = f.subst.saturating_add(subst);
    for i in &mut f.fields {
        *i = i.saturating_add(1);
    }
    Ok(())
}
