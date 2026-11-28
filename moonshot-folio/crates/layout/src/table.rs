// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Tables: one fragment per row. A row keeps its cells, with prefix heights of their lines, and
//! turns them into items only when it is placed, so cutting a row taller than a page costs time
//! in proportion to the slice that is cut, not to the whole row.

use crate::ctx::{Blk, Ctx, Frag};
use crate::inline::cell_segs;
use crate::text::{layout_lines, Align, Geom};
use crate::theme::Style;
use crate::tree::{FontSlot, Item, ItemKind, NodeId, RectItem, StructRole};
use moonshot_folio_core::{Emu, FolioError, Ink, Result};
use moonshot_folio_ir::{Cell, CellValue, Table};
use std::rc::Rc;

/// One laid-out cell: its lines (items relative to each line's top-left; x is from the
/// content-box left) and the heights of the first `i` lines for every `i`.
#[derive(Clone, Debug)]
pub(crate) struct CellBox {
    x: Emu,
    w: Emu,
    node: NodeId,
    fill: Option<Ink>,
    lines: Vec<Frag>,
    cum: Vec<Emu>,
}

impl CellBox {
    fn new(x: Emu, w: Emu, node: NodeId, fill: Option<Ink>, lines: Vec<Frag>) -> Result<Self> {
        let mut cum = Vec::with_capacity(lines.len().saturating_add(1));
        let mut h = Emu::ZERO;
        cum.push(h);
        for l in &lines {
            h = h.checked_add(l.height)?;
            cum.push(h);
        }
        Ok(Self {
            x,
            w,
            node,
            fill,
            lines,
            cum,
        })
    }

    fn at(&self, i: usize) -> Emu {
        self.cum.get(i).copied().unwrap_or(Emu::ZERO)
    }

    /// Height of the lines from `from` on.
    fn remaining(&self, from: usize) -> Result<Emu> {
        self.at(self.lines.len()).checked_sub(self.at(from))
    }

    /// Height of `n` lines starting at `from`.
    fn span(&self, from: usize, n: usize) -> Result<Emu> {
        self.at(from.saturating_add(n)).checked_sub(self.at(from))
    }

    /// How many lines from `from` fit in `avail` (binary search over the prefix heights).
    fn fit(&self, from: usize, avail: Emu) -> Result<usize> {
        let limit = self.at(from).checked_add(avail)?;
        let tail = self.cum.get(from..).unwrap_or(&[]);
        Ok(tail.partition_point(|c| *c <= limit).saturating_sub(1))
    }
}

/// A row that has not been turned into items: its cells (shared by every slice of the row) and
/// how many lines of each are already placed.
#[derive(Clone, Debug)]
pub(crate) struct RowSrc {
    cells: Rc<Vec<CellBox>>,
    from: Vec<usize>,
    pad: Emu,
    rule: (Ink, Emu),
}

impl RowSrc {
    /// Height of the part of the row not placed yet.
    fn height(&self) -> Result<Emu> {
        let mut inner = Emu::ZERO;
        for (c, f) in self.cells.iter().zip(&self.from) {
            inner = inner.max(c.remaining(*f)?);
        }
        inner.checked_add(self.pad.checked_mul(2)?)
    }

    fn done(&self) -> bool {
        self.cells
            .iter()
            .zip(&self.from)
            .all(|(c, f)| *f >= c.lines.len())
    }

    fn frag(self) -> Result<Frag> {
        Ok(Frag {
            height: self.height()?,
            row: Some(self),
            ..Frag::default()
        })
    }
}

/// Build the pagination block for a table. `base` is the text style of ordinary cells.
pub(crate) fn table_blk(
    ctx: &Ctx<'_>,
    t: &Table,
    node: NodeId,
    width: Emu,
    (role, base): (StructRole, Style),
) -> Result<Blk> {
    let pad = ctx.theme.cell_padding;
    let cols = i64::from(t.columns);
    let col_w = Emu::from_raw(width.raw().checked_div(cols).unwrap_or(0));
    let min_w = pad.checked_mul(2)?.checked_add(base.size)?;
    if col_w < min_w {
        return Err(FolioError::Unsupported("table columns too narrow"));
    }
    let hdr = usize::try_from(t.repeat_header_rows).unwrap_or(usize::MAX);
    let mut frags = Vec::new();
    for (ri, row) in t.rows.iter().enumerate() {
        let mut cells = Vec::new();
        let mut col = 0_i64;
        for (ci, cell) in row.cells.iter().enumerate() {
            let span = i64::from(cell.span);
            let x = col_w.checked_mul(col)?;
            let end = col.saturating_add(span);
            let w = if end >= cols {
                width.checked_sub(x)?
            } else {
                col_w.checked_mul(span)?
            };
            let n = node.child(ri).child(ci);
            let st = Style {
                slot: if ri < hdr {
                    FontSlot::Bold
                } else {
                    FontSlot::Regular
                },
                ..base
            };
            cells.push(cell_box(ctx, cell, (x, w), n, (ri < hdr, role, st))?);
            col = end;
        }
        frags.push(
            RowSrc {
                from: vec![0; cells.len()],
                cells: Rc::new(cells),
                pad,
                rule: (ctx.theme.rule_ink, ctx.theme.rule_width),
            }
            .frag()?,
        );
    }
    let repeat = if frags.len() > hdr { hdr } else { 0 };
    Ok(Blk {
        after: ctx.theme.para_after,
        min_head: hdr.saturating_add(1).min(frags.len()),
        repeat,
        frags,
        ..Blk::default()
    })
}

fn cell_box(
    ctx: &Ctx<'_>,
    cell: &Cell,
    (x, w): (Emu, Emu),
    node: NodeId,
    (header, role, base): (bool, StructRole, Style),
) -> Result<CellBox> {
    let pad = ctx.theme.cell_padding;
    let base = ctx.theme.apply(&cell.role, base);
    let align = if matches!(cell.value, CellValue::Number(_)) {
        Align::Right
    } else {
        Align::Left
    };
    let g = Geom {
        x0: x.checked_add(pad)?,
        width: w.checked_sub(pad.checked_mul(2)?)?,
        align,
    };
    let segs = cell_segs(&cell.value, base, node, role);
    let lines = layout_lines(ctx, &segs, g, base)?;
    CellBox::new(x, w, node, header.then_some(ctx.theme.header_fill), lines)
}

/// Turn the part of `src` not placed yet into items, at most `budget` high when given. Returns
/// the fragment (`row` is `None`) and the line counts after it.
///
/// # Errors
/// `Unsupported` when not even one line fits in `budget`.
fn assemble(src: &RowSrc, budget: Option<Emu>) -> Result<(Frag, Vec<usize>)> {
    let avail = match budget {
        Some(b) => Some(b.checked_sub(src.pad.checked_mul(2)?)?),
        None => None,
    };
    let mut takes = Vec::new();
    let mut inner = Emu::ZERO;
    for (c, from) in src.cells.iter().zip(&src.from) {
        let n = match avail {
            Some(a) => c.fit(*from, a)?,
            None => c.lines.len().saturating_sub(*from),
        };
        inner = inner.max(c.span(*from, n)?);
        takes.push(n);
    }
    if budget.is_some() && !src.done() && takes.iter().all(|n| *n == 0) {
        return Err(FolioError::Unsupported("row line taller than page body"));
    }
    let mut f = Frag {
        height: inner.checked_add(src.pad.checked_mul(2)?)?,
        ..Frag::default()
    };
    let mut next = Vec::new();
    for ((c, from), n) in src.cells.iter().zip(&src.from).zip(&takes) {
        cell_items(&mut f, c, (*from, *n), (src.pad, src.rule))?;
        next.push(from.saturating_add(*n));
    }
    Ok((f, next))
}

/// Add the cell rectangle and `n` lines starting at `from` to `f`. The rectangle is inset by
/// half the rule width so the stroke stays inside the cell.
fn cell_items(
    f: &mut Frag,
    c: &CellBox,
    (from, n): (usize, usize),
    (pad, rule): (Emu, (Ink, Emu)),
) -> Result<()> {
    let mut half = Emu::from_raw(rule.1.raw().checked_div(2).unwrap_or(0));
    let twice = half.checked_mul(2)?;
    if f.height <= twice || c.w <= twice {
        half = Emu::ZERO;
    }
    let twice = half.checked_mul(2)?;
    f.items.push(Item {
        node: c.node,
        role: StructRole::Artifact,
        x: c.x.checked_add(half)?,
        y: half,
        kind: ItemKind::Rect(RectItem {
            w: c.w.checked_sub(twice)?,
            h: f.height.checked_sub(twice)?,
            fill: c.fill,
            stroke: Some(rule),
        }),
        repeated: false,
    });
    let mut dy = pad;
    for line in c.lines.iter().skip(from).take(n) {
        let base = f.items.len();
        for it in &line.items {
            let mut it = it.clone();
            it.y = it.y.checked_add(dy)?;
            f.items.push(it);
        }
        for l in &line.links {
            let mut l = l.clone();
            l.rect.y = l.rect.y.checked_add(dy)?;
            f.links.push(l);
        }
        for d in &line.dests {
            let mut d = d.clone();
            d.y = d.y.checked_add(dy)?;
            f.dests.push(d);
        }
        f.fields
            .extend(line.fields.iter().map(|i| i.saturating_add(base)));
        f.subst = f.subst.saturating_add(line.subst);
        dy = dy.checked_add(line.height)?;
    }
    Ok(())
}

/// The items of a lazy row fragment (all of its unplaced lines).
pub(crate) fn materialise(src: &RowSrc) -> Result<Frag> {
    Ok(assemble(src, None)?.0)
}

/// Cut the first slice, at most `budget` high, off a row. Returns the slice and the remainder
/// (`None` when the slice used everything). Only the slice is turned into items.
pub(crate) fn split_row(src: &RowSrc, budget: Emu) -> Result<(Frag, Option<Frag>)> {
    let (slice, next) = assemble(src, Some(budget))?;
    let rest = RowSrc {
        from: next,
        ..src.clone()
    };
    if rest.done() {
        return Ok((slice, None));
    }
    Ok((slice, Some(rest.frag()?)))
}
