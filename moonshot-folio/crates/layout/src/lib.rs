// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! moonshot-folio-layout (L3): IR to `PageTree` (task F5.1).
//!
//! [`layout`] turns a flow [`Document`](moonshot_folio_ir::Document) into pages of positioned
//! draw items that a PDF backend can feed straight into a content stream (see [`tree`
//! coordinates](PageTree)). Scope of v1: paragraphs, headings, bulleted and numbered lists,
//! tables (rows move whole between pages, leading header rows repeat, a row taller than a page
//! is cut by line), per-section page size and margins, headers and footers, page-number and
//! page-count fields, widow and orphan control, keep-with-next for headings, hyperlinks and
//! named destinations. Not supported: images, floats, footnotes, columns, a table of contents,
//! kerning and ligatures, justification, bidirectional text.
//!
//! Fields are resolved in passes: the first pass assumes one page, counts the pages, and a
//! further pass runs with that count until it matches (at most eight passes). A document that
//! has no page-count field needs one pass. A page-number field in the body reserves the width
//! of as many of the widest digit as the page count has digits, as one unbreakable unit, and is
//! then replaced by the real number with its real width, the rest of the line moving to follow.
//!
//! Table of contents: `Block::Field(FieldKind::Toc)` in a section body lists every heading of the
//! document body (the IR field has no level range) with its page number, found by the same
//! repeated passes (the entries of one pass are laid out in the next, until they no longer
//! change). Each entry is an indented title, a dot leader and the page number at the right edge,
//! linked to a named destination `folio-toc-<section>-<block>` that the heading receives when
//! the document has a table of contents. Entries carry the node `[section, 0, block, entry]`:
//! title and number have role `Link`, the leader `Artifact`. A document without headings gets
//! no output from the field. A heading with no visible text (empty, only spaces or only anchors)
//! is neither listed, bookmarked nor given a destination; a heading made only of links is listed.
//! Anchors named `folio-toc-...` are reserved and rejected. Headings in headers and footers are
//! not listed, and a table of
//! contents inside a paragraph, header or footer is `Unsupported`. The page number shown is the
//! page label.
//!
//! Page breaks and sections: a page break takes effect only when content follows it in the same
//! section, so a trailing break never leaves a blank page. A section with no placed content
//! produces no page (its header and footer alone do not make one); if no section has content the
//! document still gets one blank page, that of the last section. Page numbering continues across
//! a dropped section.
//!
//! Keep-with-next: a chain of flagged blocks together with the first unit of the block after it
//! (its first line, or all of it when it cannot split) moves to a new page as a whole whenever
//! the rest of the chain would not fit the space left but would fit a page; when even the rest
//! of the chain is taller than a page, each block is only kept with the start of its successor.
//! Table header rows repeat on continuation pages (the items are marked `repeated`), except
//! where they would take over half the page or where the next row fits a page by itself but not
//! beside them. Anchors in headers and footers are not recorded (a link to one is an error).
//!
//! Every item carries the [`NodeId`] of the IR node it came from and a [`StructRole`], so tagged
//! PDF can be added without redesign. Output is a pure function of the document, fonts, theme
//! and options: all lengths are integer EMU with checked arithmetic and no float is used.

#![forbid(unsafe_code)]

mod block;
mod ctx;
mod dump;
mod flow;
mod fonts;
mod headings;
mod inline;
mod table;
mod text;
mod theme;
mod toc;
mod tree;

pub use ctx::Options;
pub use dump::dump;
pub use fonts::{Face, Fonts};
pub use headings::{headings, HeadingRef};
pub use theme::{RoleStyle, Theme};
pub use tree::{
    Area, BlockInfo, BlockKind, BoxRect, FontSlot, Item, ItemKind, LinkAnnot, LinkTarget,
    NamedDest, NodeId, Page, PageTree, RectItem, StructRole, TableInfo, TextItem,
};

use block::{blocks_to_blks, BlockCx};
use ctx::Ctx;
use flow::Flow;
use headings::toc_entries;
use moonshot_folio_core::{FolioError, Result};
use moonshot_folio_ir::{validate, Block, Body, Document, FieldKind, Section};

/// Most passes of the fixed point over page counts and table-of-contents entries.
const MAX_PASSES: u32 = 8;

/// Lay out a flow document.
///
/// # Errors
/// Whatever `validate` reports for the document; `Unsupported` for a workbook body, images, a
/// table of contents field inside a paragraph or in a header or footer, a character the font lacks (under `MissingGlyph::Reject`) or a
/// glyph or row that cannot fit the page; `Invalid` for a bad theme, an unknown anchor target,
/// a duplicate anchor or a header and footer that leave no room; `Limit` for more pages than
/// `max_zip_entries`, more items than `max_cells`, or page-count fields that do not settle.
pub fn layout(
    doc: &Document,
    fonts: &Fonts<'_>,
    theme: &Theme,
    opts: &Options,
) -> Result<PageTree> {
    validate(doc, &opts.limits)?;
    theme.check()?;
    let Body::Flow(sections) = &doc.body else {
        return Err(FolioError::Unsupported("a workbook has no page layout"));
    };
    if opts.limits.max_zip_entries() == 0 {
        return Err(FolioError::Limit("pages"));
    }
    let toc_on = sections.iter().any(|s| {
        s.blocks
            .iter()
            .any(|b| matches!(b, Block::Field(FieldKind::Toc)))
    });
    fixed_point(MAX_PASSES, (1_u32, Vec::new()), |(assumed, toc)| {
        let ctx = Ctx::new(fonts, theme, opts, (*assumed, toc, toc_on));
        let tree = pass(&ctx, sections)?;
        let count = u32::try_from(tree.pages.len()).unwrap_or(u32::MAX);
        let digits = |n: u32| n.to_string().len();
        let count_ok = !ctx.used_num_pages() || count == *assumed;
        let width_ok = !ctx.used_page_field() || digits(count) <= digits(*assumed);
        let entries = if toc_on {
            toc_entries(&tree)
        } else {
            Vec::new()
        };
        let settled = count_ok && width_ok && entries == *toc;
        Ok((tree, (count, entries), settled))
    })
}

/// Run `step` from `state` until it reports a settled result, at most `max` times. `step`
/// returns its result, the state for the next round and whether the result is consistent with
/// the state it was given.
///
/// # Errors
/// The first error of `step`, or `Limit` when no round settles.
fn fixed_point<S, T>(
    max: u32,
    mut state: S,
    mut step: impl FnMut(&S) -> Result<(T, S, bool)>,
) -> Result<T> {
    for _ in 0..max {
        let (out, next, settled) = step(&state)?;
        if settled {
            return Ok(out);
        }
        state = next;
    }
    Err(FolioError::Limit(
        "page count and contents fields did not settle",
    ))
}

/// One full layout of every section.
fn pass(ctx: &Ctx<'_>, sections: &[Section]) -> Result<PageTree> {
    let Some(first) = sections.first() else {
        return Err(FolioError::Invalid("flow body has no sections"));
    };
    let mut flow = Flow::new(ctx, 0, first)?;
    for (i, section) in sections.iter().enumerate() {
        if i > 0 {
            flow.begin_section(i, section)?;
        }
        let bc = BlockCx {
            ctx,
            area: Area::Body,
            label: None,
            width: flow.content_width(),
        };
        let mut blks = blocks_to_blks(&bc, i, &section.blocks)?;
        flow.place_all(&mut blks)?;
    }
    flow.finish()
}

#[cfg(test)]
mod tests {
    use super::fixed_point;
    use moonshot_folio_core::FolioError;

    // REQ: FOLIO-REQ-058
    #[test]
    fn the_fixed_point_stops_when_settled_and_fails_when_it_never_does() {
        let mut rounds = 0_u32;
        let r = fixed_point(8, 0_u32, |s| {
            rounds += 1;
            Ok((*s, s + 1, *s == 3))
        });
        assert_eq!((r, rounds), (Ok(3), 4));
        let mut rounds = 0_u32;
        let r: Result<u32, FolioError> = fixed_point(8, 0_u32, |s| {
            rounds += 1;
            Ok((*s, s + 1, false))
        });
        assert_eq!(
            r,
            Err(FolioError::Limit(
                "page count and contents fields did not settle"
            ))
        );
        assert_eq!(rounds, 8, "never more than the cap");
        let r = fixed_point(8, 0_u32, |s| {
            if *s == 2 {
                Err(FolioError::Invalid("boom"))
            } else {
                Ok(((), s + 1, false))
            }
        });
        assert_eq!(r, Err(FolioError::Invalid("boom")));
    }
}
