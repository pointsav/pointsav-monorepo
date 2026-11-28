// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Pagination: place fragments on pages, with header and footer chrome per page.

use crate::block::{blocks_to_blks, BlockCx};
use crate::ctx::{Blk, Ctx, Frag};
use crate::table::{materialise, split_row};
use crate::tree::{Area, BlockInfo, ItemKind, LinkTarget, NamedDest, Page, PageTree};
use moonshot_folio_core::{Emu, FolioError, Result};
use moonshot_folio_ir::{Block, Section};
use std::collections::BTreeSet;

/// The section being laid out and its content box size.
#[derive(Clone, Copy)]
struct Sec<'a> {
    idx: usize,
    section: &'a Section,
    content_w: Emu,
    content_h: Emu,
}

/// Header or footer fragments with their offsets from the chrome's top.
struct Chrome {
    frags: Vec<(Emu, Frag)>,
    height: Emu,
}

/// The page being filled. `y` is measured down from the top of the body area.
struct Cur {
    page: Page,
    body_top: Emu,
    body_h: Emu,
    y: Emu,
    /// Nothing but repeated rows has been placed on this page yet.
    fresh: bool,
    prev_after: Emu,
}

/// The pagination state.
pub(crate) struct Flow<'a> {
    ctx: &'a Ctx<'a>,
    pages: Vec<Page>,
    dests: Vec<NamedDest>,
    sec: Sec<'a>,
    cur: Cur,
    /// A page break was seen; the page opens when content follows in the same section.
    pending_break: bool,
    blocks: Vec<BlockInfo>,
}

fn sec_info(idx: usize, section: &Section) -> Result<Sec<'_>> {
    let twice = section.page.margin.checked_mul(2)?;
    Ok(Sec {
        idx,
        section,
        content_w: section.page.width.checked_sub(twice)?,
        content_h: section.page.height.checked_sub(twice)?,
    })
}

/// Stack blocks top to bottom with their spacing; no pagination.
fn stack(blks: Vec<Blk>) -> Result<Chrome> {
    let (mut y, mut prev_after, mut first) = (Emu::ZERO, Emu::ZERO, true);
    let mut frags = Vec::new();
    for blk in blks {
        if !first {
            y = y.checked_add(prev_after)?.checked_add(blk.before)?;
        }
        first = false;
        prev_after = blk.after;
        for f in blk.frags {
            let h = f.height;
            frags.push((y, f));
            y = y.checked_add(h)?;
        }
    }
    Ok(Chrome { frags, height: y })
}

fn chrome(ctx: &Ctx<'_>, sec: Sec<'_>, area: Area, blocks: &[Block], label: u32) -> Result<Chrome> {
    let bc = BlockCx {
        ctx,
        area,
        label: Some(label),
        width: sec.content_w,
    };
    stack(blocks_to_blks(&bc, sec.idx, blocks)?)
}

/// Move the width and the following items of a page-number placeholder to the real label.
fn resolve_field(ctx: &Ctx<'_>, page: &mut Page, at: usize, links_from: usize) -> Result<()> {
    let label = page.label.to_string();
    let Some(it) = page.items.get_mut(at) else {
        return Ok(());
    };
    let ItemKind::Text(t) = &mut it.kind else {
        return Ok(());
    };
    let mut w = Emu::ZERO;
    for c in label.chars() {
        let (cw, _) = ctx.fonts.width(t.font, c, t.size, ctx.opts.missing_glyph)?;
        w = w.checked_add(cw)?;
    }
    let old_end = it.x.checked_add(t.width)?;
    let delta = t.width.checked_sub(w)?;
    t.text = label;
    t.width = w;
    for later in page.items.iter_mut().skip(at.saturating_add(1)) {
        if later.x >= old_end {
            later.x = later.x.checked_sub(delta)?;
        }
    }
    for l in page.links.iter_mut().skip(links_from) {
        if l.rect.x >= old_end {
            l.rect.x = l.rect.x.checked_sub(delta)?;
        }
    }
    Ok(())
}

/// Where an emitted fragment comes from, which decides its anchors and its `repeated` flag.
enum Emit<'a> {
    /// A header or footer: anchors are dropped.
    Chrome,
    /// A table header row repeated on a continuation page: items are marked `repeated`,
    /// anchors are dropped.
    Repeat,
    /// Body content: anchors are recorded in the list, for the page with the given index.
    Body(&'a mut Vec<NamedDest>, usize),
}

/// Add `frag` to `page` with its top at `dy` below the content-box top. A lazy table row is
/// turned into items here.
fn emit(ctx: &Ctx<'_>, page: &mut Page, frag: &Frag, dy: Emu, mode: Emit<'_>) -> Result<()> {
    let built;
    let frag = match &frag.row {
        Some(src) => {
            built = materialise(src)?;
            &built
        }
        None => frag,
    };
    ctx.charge(frag.items.len())?;
    ctx.count_substitutes(frag.subst);
    let repeated = matches!(mode, Emit::Repeat);
    let dx = page.margin;
    let dy = page.margin.checked_add(dy)?;
    let (base, links_from) = (page.items.len(), page.links.len());
    for it in &frag.items {
        let mut placed = it.placed(dx, dy, page.height)?;
        placed.repeated = repeated;
        page.items.push(placed);
    }
    for l in &frag.links {
        page.links.push(l.placed(dx, dy, page.height)?);
    }
    for idx in &frag.fields {
        resolve_field(ctx, page, base.saturating_add(*idx), links_from)?;
    }
    if let Emit::Body(out, index) = mode {
        for d in &frag.dests {
            out.push(NamedDest {
                name: d.name.clone(),
                page: index,
                x: dx.checked_add(d.x)?,
                y: page.height.checked_sub(dy.checked_add(d.y)?)?,
            });
        }
    }
    Ok(())
}

fn build_cur(ctx: &Ctx<'_>, sec: Sec<'_>, label: u32, first: bool) -> Result<Cur> {
    let s = sec.section;
    let mut page = Page {
        width: s.page.width,
        height: s.page.height,
        margin: s.page.margin,
        section: sec.idx,
        label,
        items: Vec::new(),
        links: Vec::new(),
    };
    let none = Chrome {
        frags: Vec::new(),
        height: Emu::ZERO,
    };
    let head = if first && s.different_first_page {
        none
    } else {
        chrome(ctx, sec, Area::Header, &s.header, label)?
    };
    let foot = chrome(ctx, sec, Area::Footer, &s.footer, label)?;
    for (y, f) in &head.frags {
        emit(ctx, &mut page, f, *y, Emit::Chrome)?;
    }
    let foot_top = sec.content_h.checked_sub(foot.height)?;
    for (y, f) in &foot.frags {
        emit(ctx, &mut page, f, foot_top.checked_add(*y)?, Emit::Chrome)?;
    }
    let gap = ctx.theme.chrome_gap;
    let above = if head.height > Emu::ZERO {
        head.height.checked_add(gap)?
    } else {
        Emu::ZERO
    };
    let below = if foot.height > Emu::ZERO {
        foot.height.checked_add(gap)?
    } else {
        Emu::ZERO
    };
    let body_h = sec.content_h.checked_sub(above)?.checked_sub(below)?;
    if body_h <= Emu::ZERO {
        return Err(FolioError::Invalid(
            "header and footer leave no room for content",
        ));
    }
    Ok(Cur {
        page,
        body_top: above,
        body_h,
        y: Emu::ZERO,
        fresh: true,
        prev_after: Emu::ZERO,
    })
}

impl<'a> Flow<'a> {
    /// Start laying out with the first page of section `idx`.
    pub(crate) fn new(ctx: &'a Ctx<'a>, idx: usize, section: &'a Section) -> Result<Self> {
        let sec = sec_info(idx, section)?;
        let cur = build_cur(ctx, sec, 1, true)?;
        Ok(Self {
            ctx,
            pages: Vec::new(),
            dests: Vec::new(),
            sec,
            cur,
            pending_break: false,
            blocks: Vec::new(),
        })
    }

    /// Width of the content box of the current section.
    pub(crate) fn content_width(&self) -> Emu {
        self.sec.content_w
    }

    /// Open a new current page. The old one is kept unless it is still empty: a page nothing
    /// was placed on (an empty section) is dropped, so it never shows up as a blank page.
    fn open(&mut self, sec: Sec<'a>, label: u32, first: bool) -> Result<()> {
        if self.pages.len().saturating_add(2) > self.ctx.max_pages() {
            return Err(FolioError::Limit("pages"));
        }
        let next = build_cur(self.ctx, sec, label, first)?;
        let old = core::mem::replace(&mut self.cur, next);
        if !old.fresh {
            self.pages.push(old.page);
        }
        self.sec = sec;
        Ok(())
    }

    /// Begin section `idx` on a new page. A pending page break is dropped: the section starts
    /// a page anyway.
    pub(crate) fn begin_section(&mut self, idx: usize, section: &'a Section) -> Result<()> {
        let sec = sec_info(idx, section)?;
        self.pending_break = false;
        let after_last = if self.cur.fresh {
            self.pages.last().map_or(0, |p| p.label)
        } else {
            self.cur.page.label
        };
        let label = if section.restart_page_numbers {
            1
        } else {
            after_last.saturating_add(1)
        };
        self.open(sec, label, true)
    }

    /// Continue on a new page. The leading rows of `repeat.0` repeat there when the next
    /// fragment to place (`repeat.1`) allows it.
    fn next_page(&mut self, repeat: Option<(&Blk, usize)>) -> Result<()> {
        let label = self.cur.page.label.saturating_add(1);
        self.open(self.sec, label, false)?;
        if let Some((b, next)) = repeat {
            self.repeat_header(b, next)?;
        }
        Ok(())
    }

    /// Repeat the header rows of `b` unless they would take over half the page, or the row to
    /// place next fits a page by itself but not beside them (it is not cut for the header).
    fn repeat_header(&mut self, b: &Blk, next: usize) -> Result<()> {
        let mut total = Emu::ZERO;
        for f in b.frags.iter().take(b.repeat) {
            total = total.checked_add(f.height)?;
        }
        let body = self.cur.body_h;
        let row = b.frags.get(next).map_or(Emu::ZERO, |f| f.height);
        let crowded = row <= body && total.checked_add(row)? > body;
        if total.raw().saturating_mul(2) > body.raw() || crowded {
            return Ok(());
        }
        for f in b.frags.iter().take(b.repeat) {
            self.put(f, Emu::ZERO, true)?;
        }
        Ok(())
    }

    /// Place `frag` after `gap`. Repeated rows do not make the page non-fresh.
    fn put(&mut self, frag: &Frag, gap: Emu, repeat: bool) -> Result<()> {
        let c = &mut self.cur;
        let top = c.y.checked_add(gap)?;
        let bottom = top.checked_add(frag.height)?;
        if bottom > c.body_h {
            return Err(FolioError::Unsupported("content taller than the page body"));
        }
        let at = c.body_top.checked_add(top)?;
        let index = self.pages.len();
        let mode = if repeat {
            Emit::Repeat
        } else {
            Emit::Body(&mut self.dests, index)
        };
        emit(self.ctx, &mut c.page, frag, at, mode)?;
        c.y = bottom;
        if !repeat {
            c.fresh = false;
        }
        Ok(())
    }

    /// Space before a block's first fragment.
    fn gap(&self, blk: &Blk, first: bool) -> Result<Emu> {
        if self.cur.fresh || !first {
            Ok(Emu::ZERO)
        } else {
            self.cur.prev_after.checked_add(blk.before)
        }
    }

    /// How many frags from `i` fit in the space left after `gap`.
    fn fit_count(&self, blk: &Blk, i: usize, gap: Emu) -> Result<usize> {
        let mut used = self.cur.y.checked_add(gap)?;
        let mut n = 0_usize;
        for f in blk.frags.iter().skip(i) {
            used = used.checked_add(f.height)?;
            if used > self.cur.body_h {
                break;
            }
            n = n.saturating_add(1);
        }
        Ok(n)
    }

    /// How many of the `fit` fragments to place here, after widow, orphan and keep rules.
    fn split_count(&self, blk: &Blk, i: usize, fit: usize, remaining: usize) -> usize {
        let mut k = fit;
        if blk.unsplittable() && i == 0 {
            k = 0;
        } else {
            if remaining.saturating_sub(k) < blk.min_tail {
                k = remaining.saturating_sub(blk.min_tail);
            }
            if i == 0 && k < blk.min_head {
                k = 0;
            }
        }
        if k == 0 && self.cur.fresh {
            k = fit;
        }
        k
    }

    /// Lay out all blocks of the current section.
    pub(crate) fn place_all(&mut self, blks: &mut [Blk]) -> Result<()> {
        let mut chain = Chain::default();
        for i in 0..blks.len() {
            let Some(b) = blks.get(i) else { break };
            if b.frags.is_empty() {
                // A page break: it takes effect only when content follows in this section.
                self.pending_break |= b.page_break_before && !self.cur.fresh;
                continue;
            }
            let (break_before, keep_next) = (b.page_break_before, b.keep_next);
            if (break_before || self.pending_break) && !self.cur.fresh {
                self.next_page(None)?;
            }
            self.pending_break = false;
            if let Some(info) = blks.get_mut(i).and_then(|b| b.block.take()) {
                self.blocks.push(info);
            }
            if keep_next {
                self.keep_checks(blks, i, &mut chain)?;
            }
            if let Some(b) = blks.get_mut(i) {
                self.place_block(b)?;
            }
        }
        Ok(())
    }

    /// Height needed to place block `k` and then the first unit of block `k + 1`, or `None`
    /// when nothing has to follow it on the page.
    fn pair_need(&self, blks: &[Blk], k: usize) -> Result<Option<Emu>> {
        let (Some(b), Some(next)) = (blks.get(k), blks.get(k.saturating_add(1))) else {
            return Ok(None);
        };
        if next.frags.is_empty() || next.page_break_before {
            return Ok(None);
        }
        let first = if self.cur.fresh {
            Emu::ZERO
        } else {
            self.cur.prev_after.checked_add(b.before)?
        };
        let need = first
            .checked_add(b.height()?)?
            .checked_add(b.after)?
            .checked_add(next.before)?
            .checked_add(next.first_unit()?)?;
        Ok(Some(need))
    }

    /// Keep-with-next for block `i`, which has the flag. For a chain of such blocks and the
    /// block after it, `chain.suffix[k]` is the height from block `k` to the end of the start
    /// of that block. When that fits a page, it all goes on one page: here if it fits the
    /// space left, otherwise on the next page. Re-checked at each block, so the rest of a
    /// chain that began on an earlier page still moves as a whole. When even the rest is
    /// taller than a page the keep rule cannot be met; each block is then at least kept with
    /// the start of its successor.
    fn keep_checks(&mut self, blks: &[Blk], i: usize, chain: &mut Chain) -> Result<()> {
        if i >= chain.end {
            Self::start_chain(blks, i, chain)?;
        }
        let Some(b) = blks.get(i) else {
            return Ok(());
        };
        let first = if self.cur.fresh {
            Emu::ZERO
        } else {
            self.cur.prev_after.checked_add(b.before)?
        };
        let rest = chain
            .suffix
            .get(i.saturating_sub(chain.start))
            .copied()
            .unwrap_or(Emu::ZERO);
        // On a fresh page the space before the block drops out, so it is `rest` alone that has
        // to fit a page.
        let need = if rest <= self.cur.body_h {
            Some(rest.checked_add(first)?)
        } else {
            self.pair_need(blks, i)?
        };
        let left = self.cur.body_h.checked_sub(self.cur.y)?;
        if need.is_some_and(|n| !self.cur.fresh && n > left) {
            self.next_page(None)?;
        }
        Ok(())
    }

    /// Compute `chain.suffix` for the chain of keep-with-next blocks that starts at `i`.
    fn start_chain(blks: &[Blk], i: usize, chain: &mut Chain) -> Result<()> {
        let mut j = i;
        while blks.get(j).is_some_and(|b| b.keep_next) {
            j = j.saturating_add(1);
        }
        let last_after = blks.get(j.saturating_sub(1)).map_or(Emu::ZERO, |b| b.after);
        let mut acc = match blks
            .get(j)
            .filter(|b| !b.frags.is_empty() && !b.page_break_before)
        {
            Some(next) => last_after
                .checked_add(next.before)?
                .checked_add(next.first_unit()?)?,
            None => Emu::ZERO,
        };
        let mut suffix = vec![Emu::ZERO; j.saturating_sub(i)];
        for k in (i..j).rev() {
            let Some(b) = blks.get(k) else { continue };
            if k.saturating_add(1) < j {
                let next_before = blks
                    .get(k.saturating_add(1))
                    .map_or(Emu::ZERO, |n| n.before);
                acc = acc.checked_add(b.after)?.checked_add(next_before)?;
            }
            acc = acc.checked_add(b.height()?)?;
            if let Some(slot) = suffix.get_mut(k.saturating_sub(i)) {
                *slot = acc;
            }
        }
        *chain = Chain {
            start: i,
            end: j,
            suffix,
        };
        Ok(())
    }

    fn place_block(&mut self, blk: &mut Blk) -> Result<()> {
        let n = blk.frags.len();
        let (mut i, mut first) = (0_usize, true);
        while i < n {
            let gap = self.gap(blk, first)?;
            let fit = self.fit_count(blk, i, gap)?;
            let remaining = n.saturating_sub(i);
            if fit == remaining {
                self.put_run(blk, i, fit, gap)?;
                break;
            }
            let k = self.split_count(blk, i, fit, remaining);
            if k > 0 {
                self.put_run(blk, i, k, gap)?;
                i = i.saturating_add(k);
                self.next_page(repeat_for(blk, i))?;
            } else if self.cur.fresh {
                i = self.place_oversize(blk, i)?;
            } else {
                self.next_page(repeat_for(blk, i))?;
                continue;
            }
            first = false;
        }
        self.cur.prev_after = blk.after;
        Ok(())
    }

    fn put_run(&mut self, blk: &Blk, from: usize, count: usize, gap: Emu) -> Result<()> {
        for (k, f) in blk.frags.iter().skip(from).take(count).enumerate() {
            self.put(f, if k == 0 { gap } else { Emu::ZERO }, false)?;
        }
        Ok(())
    }

    /// A fragment that does not fit even an empty page: cut a table row by line.
    fn place_oversize(&mut self, blk: &mut Blk, i: usize) -> Result<usize> {
        let avail = self.cur.body_h.checked_sub(self.cur.y)?;
        let Some(src) = blk.frags.get(i).and_then(|f| f.row.clone()) else {
            return Err(FolioError::Unsupported("content taller than the page body"));
        };
        let (slice, rest) = split_row(&src, avail)?;
        self.put(&slice, Emu::ZERO, false)?;
        let next = match rest {
            Some(r) => {
                if let Some(slot) = blk.frags.get_mut(i) {
                    *slot = r;
                }
                i
            }
            None => i.saturating_add(1),
        };
        if next < blk.frags.len() {
            self.next_page(repeat_for(blk, next))?;
        }
        Ok(next)
    }

    /// Finish: close the last page and check anchors and links. A last page nothing was
    /// placed on is dropped unless it is the only page.
    pub(crate) fn finish(mut self) -> Result<PageTree> {
        if !self.cur.fresh || self.pages.is_empty() {
            self.pages.push(self.cur.page);
        }
        let mut names = BTreeSet::new();
        for d in &self.dests {
            if !names.insert(d.name.as_str()) {
                return Err(FolioError::Invalid("duplicate anchor name"));
            }
        }
        for l in self.pages.iter().flat_map(|p| &p.links) {
            if let LinkTarget::Named(n) = &l.target {
                if !names.contains(n.as_str()) {
                    return Err(FolioError::Invalid("link to unknown anchor"));
                }
            }
        }
        Ok(PageTree {
            pages: self.pages,
            dests: self.dests,
            substituted: self.ctx.substituted(),
            blocks: self.blocks,
        })
    }
}

/// State of the keep-with-next chain being placed.
#[derive(Default)]
struct Chain {
    /// First block of the chain.
    start: usize,
    /// First block after the chain; blocks before it need no new chain.
    end: usize,
    /// For each chain block, the height from it to the end of the first unit of the block
    /// after the chain, spacing included (the space before the block itself excluded).
    suffix: Vec<Emu>,
}

/// The block whose leading rows repeat on a continuation page, once they have been placed, and
/// the next fragment to place.
fn repeat_for(blk: &Blk, next: usize) -> Option<(&Blk, usize)> {
    (blk.repeat > 0 && next >= blk.repeat && next < blk.frags.len()).then_some((blk, next))
}
