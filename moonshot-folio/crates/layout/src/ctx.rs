// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Layout options, the per-pass context and the internal fragment types.

use crate::fonts::Fonts;
use crate::headings::TocEntry;
use crate::theme::Theme;
use crate::tree::{Item, LinkAnnot};
use moonshot_folio_core::{Emu, FolioError, Limits, Result};
use moonshot_folio_font::MissingGlyph;
use std::cell::Cell;

/// Caller options.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Resource limits. Pages are capped at `max_zip_entries` (the PDF writer's page cap), draw
    /// items and table cells at `max_cells`.
    pub limits: Limits,
    /// What to do with a character the font cannot draw.
    pub missing_glyph: MissingGlyph,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            limits: Limits::default(),
            missing_glyph: MissingGlyph::Reject,
        }
    }
}

/// Everything one layout pass reads, plus the counters it updates.
pub(crate) struct Ctx<'a> {
    pub(crate) fonts: &'a Fonts<'a>,
    pub(crate) theme: &'a Theme,
    pub(crate) opts: &'a Options,
    /// The page count assumed for `NumPages` fields in this pass.
    pub(crate) num_pages: u32,
    /// Table-of-contents entries as the previous pass found them.
    pub(crate) toc: &'a [TocEntry],
    /// The document has a table of contents, so headings get named destinations.
    pub(crate) toc_on: bool,
    used_num_pages: Cell<bool>,
    used_page_field: Cell<bool>,
    substituted: Cell<usize>,
    items: Cell<usize>,
    chars: Cell<usize>,
    lines: Cell<usize>,
}

/// Characters of text one unit of the `max_cells` limit stands for. Text and wrapped lines are
/// charged before they are built, so a huge paragraph fails fast under a small limit.
const CHARS_PER_NODE: usize = 256;

impl<'a> Ctx<'a> {
    pub(crate) fn new(
        fonts: &'a Fonts<'a>,
        theme: &'a Theme,
        opts: &'a Options,
        (num_pages, toc, toc_on): (u32, &'a [TocEntry], bool),
    ) -> Self {
        Self {
            fonts,
            theme,
            opts,
            num_pages,
            toc,
            toc_on,
            used_num_pages: Cell::new(false),
            used_page_field: Cell::new(false),
            substituted: Cell::new(0),
            items: Cell::new(0),
            chars: Cell::new(0),
            lines: Cell::new(0),
        }
    }

    pub(crate) fn mark_num_pages(&self) {
        self.used_num_pages.set(true);
    }

    pub(crate) fn used_num_pages(&self) -> bool {
        self.used_num_pages.get()
    }

    pub(crate) fn mark_page_field(&self) {
        self.used_page_field.set(true);
    }

    pub(crate) fn used_page_field(&self) -> bool {
        self.used_page_field.get()
    }

    /// Count `n` characters drawn with a substitute width (once per drawn occurrence).
    pub(crate) fn count_substitutes(&self, n: usize) {
        self.substituted
            .set(self.substituted.get().saturating_add(n));
    }

    pub(crate) fn substituted(&self) -> usize {
        self.substituted.get()
    }

    /// Account for `n` new draw items.
    pub(crate) fn charge(&self, n: usize) -> Result<()> {
        let total = self.items.get().saturating_add(n);
        if total > self.opts.limits.max_cells() {
            return Err(FolioError::Limit("layout items"));
        }
        self.items.set(total);
        Ok(())
    }

    /// Account for `n` characters about to be measured.
    pub(crate) fn charge_chars(&self, n: usize) -> Result<()> {
        let total = self.chars.get().saturating_add(n);
        let cap = self.opts.limits.max_cells().saturating_mul(CHARS_PER_NODE);
        if total > cap {
            return Err(FolioError::Limit("text size"));
        }
        self.chars.set(total);
        Ok(())
    }

    /// Account for one wrapped line.
    pub(crate) fn charge_line(&self) -> Result<()> {
        let total = self.lines.get().saturating_add(1);
        if total > self.opts.limits.max_cells() {
            return Err(FolioError::Limit("layout lines"));
        }
        self.lines.set(total);
        Ok(())
    }

    /// Most pages a tree may hold.
    pub(crate) fn max_pages(&self) -> usize {
        self.opts.limits.max_zip_entries()
    }
}

/// A named anchor inside a fragment: `x` from the content-box left, `y` from the fragment top.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RelDest {
    pub(crate) name: String,
    pub(crate) x: Emu,
    pub(crate) y: Emu,
}

/// The unit pagination moves: a line, or a table row (or a slice of one). Items use fragment
/// coordinates: x from the content-box left, y downwards from the fragment top, rectangles and
/// link rectangles anchored at their top edge, text at its baseline.
#[derive(Clone, Debug, Default)]
pub(crate) struct Frag {
    pub(crate) height: Emu,
    /// Baseline of the first line, from the fragment top (for list markers).
    pub(crate) baseline: Emu,
    pub(crate) items: Vec<Item>,
    pub(crate) links: Vec<LinkAnnot>,
    pub(crate) dests: Vec<RelDest>,
    /// Indices into `items` of text to replace by the page number when placed.
    pub(crate) fields: Vec<usize>,
    /// Characters in `items` laid out with a substitute width.
    pub(crate) subst: usize,
    /// A table row that has not been turned into items yet (or the rest of one that was cut):
    /// the items are built from it when the fragment is placed.
    pub(crate) row: Option<crate::table::RowSrc>,
}

/// A block's worth of fragments plus its pagination rules.
#[derive(Clone, Debug, Default)]
pub(crate) struct Blk {
    pub(crate) before: Emu,
    pub(crate) after: Emu,
    pub(crate) frags: Vec<Frag>,
    pub(crate) keep_next: bool,
    pub(crate) keep_together: bool,
    pub(crate) page_break_before: bool,
    /// Fewest fragments that may stay at the end of a page.
    pub(crate) min_head: usize,
    /// Fewest fragments that may start the next page.
    pub(crate) min_tail: usize,
    /// Leading fragments repeated on each continuation page.
    pub(crate) repeat: usize,
    /// Set on the first `Blk` of a body block: what the block is (recorded when it is placed).
    pub(crate) block: Option<crate::tree::BlockInfo>,
}

impl Blk {
    pub(crate) fn height(&self) -> Result<Emu> {
        sum_heights(self.frags.iter())
    }

    /// True when the block may not be split across pages: it is kept together, or it has too
    /// few fragments to leave `min_head` on one page and `min_tail` on the next.
    pub(crate) fn unsplittable(&self) -> bool {
        let need = self.min_head.saturating_add(self.min_tail);
        self.keep_together || (need > 0 && self.frags.len() < need)
    }

    /// Height of the smallest part of the block that may end a page: all of it when it cannot
    /// be split, otherwise its first `min_head` (at least one) fragments.
    pub(crate) fn first_unit(&self) -> Result<Emu> {
        let n = if self.unsplittable() {
            self.frags.len()
        } else {
            self.min_head.max(1)
        };
        sum_heights(self.frags.iter().take(n))
    }
}

fn sum_heights<'a>(it: impl Iterator<Item = &'a Frag>) -> Result<Emu> {
    let mut h = Emu::ZERO;
    for f in it {
        h = h.checked_add(f.height)?;
    }
    Ok(h)
}
