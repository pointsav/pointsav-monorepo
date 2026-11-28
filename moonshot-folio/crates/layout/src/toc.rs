// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The table of contents: one block per entry, laid out from the headings the previous pass
//! found. Each entry is an indented title (wrapped when long), a dot leader and the page number
//! at the right edge, linked as a whole to the heading's named destination.

use crate::block::{left, BlockCx};
use crate::ctx::{Blk, Ctx};
use crate::headings::TocEntry;
use crate::text::{layout_lines, Seg};
use crate::theme::Style;
use crate::tree::{
    BoxRect, FontSlot, Item, ItemKind, LinkAnnot, LinkTarget, NodeId, StructRole, TextItem,
};
use moonshot_folio_core::{Emu, FolioError, Result};

/// Most dots in one leader.
const MAX_DOTS: i64 = 512;

fn measure(ctx: &Ctx<'_>, text: &str, st: Style) -> Result<Emu> {
    let mut w = Emu::ZERO;
    for c in text.chars() {
        let (cw, _) = ctx
            .fonts
            .width(st.slot, c, st.size, ctx.opts.missing_glyph)?;
        w = w.checked_add(cw)?;
    }
    Ok(w)
}

/// The blocks of a table of contents: nothing when the document has no headings.
///
/// # Errors
/// `Invalid` when an entry leaves no room for its title; the errors of text layout.
pub(crate) fn toc_blks(bc: &BlockCx<'_>, node: NodeId) -> Result<Vec<Blk>> {
    bc.ctx
        .toc
        .iter()
        .enumerate()
        .map(|(i, e)| entry_blk(bc, e, node.child(i)))
        .collect()
}

fn entry_blk(bc: &BlockCx<'_>, e: &TocEntry, node: NodeId) -> Result<Blk> {
    let (ctx, base) = (bc.ctx, bc.base());
    let t = ctx.theme;
    let label = e.label.to_string();
    let lw = measure(ctx, &label, base)?;
    // Room for the title once the number and the gap are taken off; the indent is clamped so
    // a deep level on a narrow page still leaves the title four ems (or all there is).
    let avail = bc.width.checked_sub(lw)?.checked_sub(t.marker_gap)?;
    if avail <= Emu::ZERO {
        return Err(FolioError::Invalid(
            "table of contents entry leaves no room",
        ));
    }
    let min_title = avail.min(base.size.checked_mul(4)?);
    let indent = t
        .toc_indent
        .checked_mul(i64::from(e.level.saturating_sub(1)))?
        .min(avail.checked_sub(min_title)?);
    let title_w = avail.checked_sub(indent)?;
    let seg = Seg {
        link: Some(format!("#{}", e.dest)),
        ..Seg::plain(e.title.clone(), base, node, StructRole::Link)
    };
    let mut frags = layout_lines(ctx, &[seg], left(indent, title_w), base)?;
    if let Some(last) = frags.last_mut() {
        let end = last
            .items
            .iter()
            .filter_map(|i| match &i.kind {
                ItemKind::Text(x) => i.x.checked_add(x.width).ok(),
                ItemKind::Rect(_) => None,
            })
            .max()
            .unwrap_or(indent);
        let tail = Tail {
            node,
            dest: &e.dest,
            label: &label,
            label_w: lw,
            title_end: end,
            width: bc.width,
            base,
        };
        add_tail(ctx, last, &tail)?;
    }
    Ok(Blk {
        after: t.toc_entry_after,
        frags,
        ..Blk::default()
    })
}

/// What the right end of an entry's last line is made of.
struct Tail<'a> {
    node: NodeId,
    dest: &'a str,
    label: &'a str,
    label_w: Emu,
    title_end: Emu,
    width: Emu,
    base: Style,
}

fn text_item(node: NodeId, role: StructRole, x: Emu, y: Emu, t: TextItem) -> Item {
    Item {
        node,
        role,
        x,
        y,
        kind: ItemKind::Text(t),
        repeated: false,
    }
}

/// Regular-face text of the entry's style.
fn run(st: Style, (asc, desc): (Emu, Emu), w: Emu, text: String) -> TextItem {
    TextItem {
        font: FontSlot::Regular,
        size: st.size,
        ink: st.ink,
        width: w,
        ascent: asc,
        descent: desc,
        text,
        space_after: false,
    }
}

/// The dot leader between the title and the number: its left edge and item, if it fits.
fn leader(
    ctx: &Ctx<'_>,
    tl: &Tail<'_>,
    ext: (Emu, Emu),
    num_x: Emu,
) -> Result<Option<(Emu, TextItem)>> {
    let gap = ctx.theme.marker_gap;
    let (dot_w, _) =
        ctx.fonts
            .width(FontSlot::Regular, '.', tl.base.size, ctx.opts.missing_glyph)?;
    let room = num_x
        .checked_sub(gap)?
        .checked_sub(tl.title_end.checked_add(gap)?)?;
    let dots = room
        .raw()
        .checked_div(dot_w.raw())
        .unwrap_or(0)
        .min(MAX_DOTS);
    if dots < 2 {
        return Ok(None);
    }
    let w = dot_w.checked_mul(dots)?;
    let x = num_x.checked_sub(gap)?.checked_sub(w)?;
    let text = ".".repeat(usize::try_from(dots).unwrap_or(0));
    Ok(Some((x, run(tl.base, ext, w, text))))
}

/// Add the dot leader and the page number to the last line of an entry, and one link over both.
fn add_tail(ctx: &Ctx<'_>, f: &mut crate::ctx::Frag, tl: &Tail<'_>) -> Result<()> {
    let ext = ctx.fonts.extents(FontSlot::Regular, tl.base.size)?;
    let num_x = tl.width.checked_sub(tl.label_w)?;
    let mut link_x = num_x;
    // The link starts where the title ends, so the gap before the leader is clickable.
    link_x = link_x.min(tl.title_end);
    if let Some((x, item)) = leader(ctx, tl, ext, num_x)? {
        f.items.push(text_item(
            tl.node,
            StructRole::Artifact,
            x,
            f.baseline,
            item,
        ));
    }
    let number = run(tl.base, ext, tl.label_w, tl.label.to_string());
    f.items.push(text_item(
        tl.node,
        StructRole::Link,
        num_x,
        f.baseline,
        number,
    ));
    f.links.push(LinkAnnot {
        node: tl.node,
        rect: BoxRect {
            x: link_x,
            y: f.baseline.checked_sub(ext.0)?,
            w: tl.width.checked_sub(link_x)?,
            h: ext.0.checked_add(ext.1)?,
        },
        target: LinkTarget::Named(tl.dest.to_string()),
    });
    Ok(())
}
