// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The page content-stream builder.

use crate::syntax::{hex_string, ink_ops, literal_string, name, pt};
use crate::text::{FontId, TextRun};
use crate::trees::MAX_ARRAY;
use moonshot_folio_core::{Emu, FolioError, Ink, Result};
use std::collections::BTreeSet;

/// Builds one page's content stream, one operator per line, uncompressed.
///
/// Coordinates are [`Emu`] measured from the bottom-left of the page and are written as points
/// with at most three decimals, computed with integer arithmetic. Operators that would make the
/// stream invalid (unbalanced `q`/`Q`, text outside `BT`/`ET`) return an error.
#[derive(Clone, Debug, Default)]
pub struct Content {
    buf: Vec<u8>,
    fonts: BTreeSet<usize>,
    depth: usize,
    in_text: bool,
    current: Option<FontId>,
    /// Open marked-content sequence: the graphics-state depth it began at.
    marked: Option<usize>,
    next_mcid: u32,
}

/// What an artifact marked-content sequence is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactKind {
    /// Page content outside the logical structure (rules, fills, leaders).
    Plain,
    /// A running header (pagination artifact).
    Header,
    /// A running footer (pagination artifact).
    Footer,
}

impl Content {
    /// An empty content stream.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn raw(&mut self, s: &str) {
        self.buf.extend_from_slice(s.as_bytes());
    }

    fn need_text(&self, yes: bool) -> Result<()> {
        if self.in_text == yes {
            Ok(())
        } else if yes {
            Err(FolioError::Invalid("operator needs an open text object"))
        } else {
            Err(FolioError::Invalid(
                "operator not allowed inside text object",
            ))
        }
    }

    /// `q`: push the graphics state.
    ///
    /// # Errors
    /// `Invalid` inside a text object, `Limit` past 28 nested levels (the PDF minimum).
    pub fn save(&mut self) -> Result<()> {
        self.need_text(false)?;
        if self.depth >= 28 {
            return Err(FolioError::Limit("graphics state nesting"));
        }
        self.depth = self.depth.saturating_add(1);
        self.raw("q\n");
        Ok(())
    }

    /// `Q`: pop the graphics state.
    ///
    /// # Errors
    /// `Invalid` when there is no matching [`Content::save`], inside a text object, or when it
    /// would pop the graphics state an open marked-content sequence began in.
    pub fn restore(&mut self) -> Result<()> {
        self.need_text(false)?;
        if self.marked.is_some_and(|at| self.depth <= at) {
            return Err(FolioError::Invalid(
                "restore would pop the graphics state a marked-content sequence began in",
            ));
        }
        self.depth = self
            .depth
            .checked_sub(1)
            .ok_or(FolioError::Invalid("restore without save"))?;
        self.raw("Q\n");
        Ok(())
    }

    /// Set the fill colour (`g`, `rg` or `k`).
    ///
    /// # Errors
    /// None today; returns `Result` for symmetry with the other operators.
    pub fn set_fill(&mut self, ink: Ink) -> Result<()> {
        self.raw(&ink_ops(ink, true));
        Ok(())
    }

    /// Set the stroke colour (`G`, `RG` or `K`).
    ///
    /// # Errors
    /// None today; returns `Result` for symmetry with the other operators.
    pub fn set_stroke(&mut self, ink: Ink) -> Result<()> {
        self.raw(&ink_ops(ink, false));
        Ok(())
    }

    /// Set the line width (`w`).
    ///
    /// # Errors
    /// `Invalid` for a negative width, `Limit` on overflow.
    pub fn set_line_width(&mut self, w: Emu) -> Result<()> {
        if w.raw() < 0 {
            return Err(FolioError::Invalid("negative line width"));
        }
        let s = format!("{} w\n", pt(w)?);
        self.raw(&s);
        Ok(())
    }

    /// Append a rectangle path (`re`).
    ///
    /// # Errors
    /// `Limit` on length overflow.
    pub fn rect(&mut self, origin: (Emu, Emu), size: (Emu, Emu)) -> Result<()> {
        let s = format!(
            "{} {} {} {} re\n",
            pt(origin.0)?,
            pt(origin.1)?,
            pt(size.0)?,
            pt(size.1)?
        );
        self.raw(&s);
        Ok(())
    }

    /// Start a subpath (`m`).
    ///
    /// # Errors
    /// `Limit` on length overflow.
    pub fn move_to(&mut self, x: Emu, y: Emu) -> Result<()> {
        let s = format!("{} {} m\n", pt(x)?, pt(y)?);
        self.raw(&s);
        Ok(())
    }

    /// Append a straight segment (`l`).
    ///
    /// # Errors
    /// `Limit` on length overflow.
    pub fn line_to(&mut self, x: Emu, y: Emu) -> Result<()> {
        let s = format!("{} {} l\n", pt(x)?, pt(y)?);
        self.raw(&s);
        Ok(())
    }

    /// Paint the current path: fill (`f`).
    pub fn fill(&mut self) {
        self.raw("f\n");
    }

    /// Paint the current path: stroke (`S`).
    pub fn stroke(&mut self) {
        self.raw("S\n");
    }

    /// Paint the current path: fill then stroke (`B`).
    pub fn fill_stroke(&mut self) {
        self.raw("B\n");
    }

    /// Close the current subpath (`h`).
    pub fn close_path(&mut self) {
        self.raw("h\n");
    }

    /// `BT`: open a text object.
    ///
    /// # Errors
    /// `Invalid` when one is already open.
    pub fn begin_text(&mut self) -> Result<()> {
        self.need_text(false)?;
        self.in_text = true;
        self.raw("BT\n");
        Ok(())
    }

    /// `ET`: close the text object.
    ///
    /// # Errors
    /// `Invalid` when none is open.
    pub fn end_text(&mut self) -> Result<()> {
        self.need_text(true)?;
        self.in_text = false;
        self.raw("ET\n");
        Ok(())
    }

    /// `Tf`: select a registered font and size, and mark the font as used by this page.
    ///
    /// # Errors
    /// `Invalid` outside a text object or for a size that is not positive.
    pub fn set_font(&mut self, font: FontId, size: Emu) -> Result<()> {
        self.need_text(true)?;
        if size.raw() <= 0 {
            return Err(FolioError::Invalid("font size must be positive"));
        }
        let s = format!("/F{} {} Tf\n", font.index().saturating_add(1), pt(size)?);
        self.raw(&s);
        self.fonts.insert(font.index());
        self.current = Some(font);
        Ok(())
    }

    /// `Td`: move the text position (absolute after `BT`, relative to the line start after that).
    ///
    /// # Errors
    /// `Invalid` outside a text object.
    pub fn move_text(&mut self, x: Emu, y: Emu) -> Result<()> {
        self.need_text(true)?;
        let s = format!("{} {} Td\n", pt(x)?, pt(y)?);
        self.raw(&s);
        Ok(())
    }

    /// `Tj`: show text encoded by `PdfDoc::encode_text` for the font last set with `set_font`.
    ///
    /// # Errors
    /// `Invalid` outside a text object, or when the run was encoded for a different font.
    pub fn show(&mut self, run: &TextRun) -> Result<()> {
        self.need_text(true)?;
        if self.current != Some(run.font()) {
            return Err(FolioError::Invalid("text run belongs to another font"));
        }
        if run.is_hex() {
            let mut s = hex_string(run.bytes());
            s.push_str(" Tj\n");
            self.raw(&s);
        } else {
            self.buf.extend_from_slice(&literal_string(run.bytes()));
            self.raw(" Tj\n");
        }
        Ok(())
    }

    /// Convenience: `BT`, font, position, show, `ET` in one call.
    ///
    /// # Errors
    /// As for the individual operators.
    pub fn text(&mut self, size: Emu, x: Emu, y: Emu, run: &TextRun) -> Result<()> {
        self.begin_text()?;
        self.set_font(run.font(), size)?;
        self.move_text(x, y)?;
        self.show(run)?;
        self.end_text()
    }

    /// Open a marked-content sequence for a structure element: `/Tag << /MCID n >> BDC`.
    /// Returns the marked-content id, which counts up from 0 within the page in the order the
    /// sequences are opened, so a structure tree can refer to the content.
    ///
    /// # Errors
    /// `Invalid` inside a text object, inside another marked-content sequence, or for an empty
    /// tag.
    pub fn begin_tagged(&mut self, tag: &str) -> Result<u32> {
        if self.next_mcid as usize >= MAX_ARRAY {
            return Err(FolioError::Limit("marked content per page"));
        }
        self.open_marked(tag)?;
        let id = self.next_mcid;
        self.next_mcid = id
            .checked_add(1)
            .ok_or(FolioError::Limit("marked content ids"))?;
        let s = format!("{} << /MCID {id} >> BDC\n", name(tag));
        self.raw(&s);
        Ok(id)
    }

    /// Open an artifact sequence: `/Artifact BMC`, or for a running header or footer
    /// `/Artifact << /Type /Pagination /Subtype /Header >> BDC`.
    ///
    /// # Errors
    /// `Invalid` inside a text object or another marked-content sequence.
    pub fn begin_artifact(&mut self, kind: ArtifactKind) -> Result<()> {
        self.open_marked("Artifact")?;
        match kind {
            ArtifactKind::Plain => self.raw("/Artifact BMC\n"),
            ArtifactKind::Header => {
                self.raw("/Artifact << /Type /Pagination /Subtype /Header >> BDC\n");
            }
            ArtifactKind::Footer => {
                self.raw("/Artifact << /Type /Pagination /Subtype /Footer >> BDC\n");
            }
        }
        Ok(())
    }

    /// `EMC`: close the open marked-content sequence.
    ///
    /// # Errors
    /// `Invalid` when none is open, inside a text object, or when graphics states opened inside
    /// the sequence are still open.
    pub fn end_marked(&mut self) -> Result<()> {
        self.need_text(false)?;
        let at = self
            .marked
            .ok_or(FolioError::Invalid("no marked content to end"))?;
        if at != self.depth {
            return Err(FolioError::Invalid(
                "marked content straddles a graphics state",
            ));
        }
        self.marked = None;
        self.raw("EMC\n");
        Ok(())
    }

    /// How many tagged sequences have been opened (the next marked-content id).
    #[must_use]
    pub fn mcid_count(&self) -> u32 {
        self.next_mcid
    }

    fn open_marked(&mut self, tag: &str) -> Result<()> {
        self.need_text(false)?;
        if tag.is_empty() {
            return Err(FolioError::Invalid("empty marked-content tag"));
        }
        if self.marked.is_some() {
            return Err(FolioError::Invalid("marked content does not nest"));
        }
        self.marked = Some(self.depth);
        Ok(())
    }

    /// Check the stream is complete and return `(bytes, fonts used)`.
    pub(crate) fn finish(&self) -> Result<(&[u8], &BTreeSet<usize>)> {
        if self.depth != 0 {
            return Err(FolioError::Invalid("unbalanced save/restore"));
        }
        if self.marked.is_some() {
            return Err(FolioError::Invalid("unterminated marked content"));
        }
        if self.in_text {
            return Err(FolioError::Invalid("unterminated text object"));
        }
        Ok((&self.buf, &self.fonts))
    }
}
