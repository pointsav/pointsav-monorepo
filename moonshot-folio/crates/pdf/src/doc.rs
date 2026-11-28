// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The document model: registered fonts, pages, named destinations, bookmarks.

use crate::content::Content;
use crate::nav::{validate_uri, Bookmark, Dest, Info, Link, LinkTarget};
use crate::syntax::winansi;
use crate::text::{FontId, TextPolicy, TextRun};
use moonshot_folio_core::{Emu, FolioError, Limits, Result, EMU_PER_INCH};
use moonshot_folio_font::{check_embedding, Font, Std14};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) enum FontKind {
    Std(Std14),
    Ttf {
        data: Vec<u8>,
        used: BTreeMap<u16, char>,
        /// Glyph lookups done so far, so the font is parsed only when a new character shows up.
        glyphs: BTreeMap<char, Option<u16>>,
    },
}

pub(crate) struct PageData {
    pub(crate) width: Emu,
    pub(crate) height: Emu,
    pub(crate) content: Vec<u8>,
    pub(crate) fonts: BTreeSet<usize>,
    pub(crate) links: Vec<Link>,
    /// Marked-content ids used (0..mcids).
    pub(crate) mcids: u32,
}

/// A PDF document under construction.
///
/// Fonts are registered first ([`PdfDoc::add_std14`], [`PdfDoc::add_truetype`]). Text is turned
/// into a [`TextRun`] with [`PdfDoc::encode_text`]; for an embedded font this is also where each
/// glyph is recorded as used, so the subset and the `ToUnicode` map built by [`PdfDoc::build`]
/// cover exactly the characters that were encoded. Pages are added with their finished
/// [`Content`]. `build` is a pure function of the document: no clock, no environment.
pub struct PdfDoc {
    pub(crate) limits: Limits,
    pub(crate) info: Info,
    pub(crate) fonts: Vec<FontKind>,
    pub(crate) pages: Vec<PageData>,
    pub(crate) dests: BTreeMap<String, Dest>,
    pub(crate) outline: Vec<Bookmark>,
    /// Page label runs: `(first page index, number shown on it)`.
    pub(crate) labels: Vec<(usize, u32)>,
    parses: usize,
    pub(crate) structure: Option<crate::tags::StructElem>,
}

impl PdfDoc {
    /// An empty document. Counts of pages and links are capped at [`Limits::max_zip_entries`],
    /// destinations and bookmarks at [`Limits::max_cells`]; one content stream at [`Limits::max_entry_bytes`]; the
    /// whole file at [`Limits::max_file_bytes`]; outline depth at [`Limits::max_depth`].
    #[must_use]
    pub fn new(limits: Limits) -> Self {
        Self {
            limits,
            info: Info::default(),
            fonts: Vec::new(),
            pages: Vec::new(),
            dests: BTreeMap::new(),
            outline: Vec::new(),
            labels: Vec::new(),
            parses: 0,
            structure: None,
        }
    }

    /// Set the information dictionary values.
    ///
    /// # Errors
    /// `Invalid` for a creation date that is not `YYYY-MM-DD`; `Limit` for oversize strings.
    pub fn set_info(&mut self, info: Info) -> Result<()> {
        if let Some(d) = &info.creation_date {
            crate::nav::date_digits(d)?;
        }
        let max = self.limits.max_name_bytes().saturating_mul(8);
        let fields = [
            &info.title,
            &info.author,
            &info.subject,
            &info.keywords,
            &info.creator,
            &info.lang,
        ];
        if info.lang.as_ref().is_some_and(|l| {
            l.is_empty() || !l.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        }) {
            return Err(FolioError::Invalid("lang must be a BCP 47 tag"));
        }
        if fields
            .iter()
            .any(|f| f.as_ref().is_some_and(|s| s.len() > max))
        {
            return Err(FolioError::Limit("info string length"));
        }
        self.info = info;
        Ok(())
    }

    /// How many times an embedded font was parsed while encoding text. A font is parsed only
    /// when a character not seen before arrives, so this stays small however much text is
    /// encoded.
    #[must_use]
    pub fn font_parses(&self) -> usize {
        self.parses
    }

    /// Register Helvetica or Helvetica-Bold as a simple `Type1` font with `WinAnsiEncoding`.
    pub fn add_std14(&mut self, font: Std14) -> FontId {
        self.fonts.push(FontKind::Std(font));
        FontId(self.fonts.len().saturating_sub(1))
    }

    /// Register a TrueType font to be embedded as a glyph-id-preserving subset.
    ///
    /// # Errors
    /// Whatever `Font::parse` reports, and `Unsupported` when `check_embedding` finds the font
    /// licence forbids embedding or subsetting.
    pub fn add_truetype(&mut self, data: Vec<u8>) -> Result<FontId> {
        let font = Font::parse(&data, &self.limits)?;
        check_embedding(&font, true)?;
        self.fonts.push(FontKind::Ttf {
            data,
            used: BTreeMap::new(),
            glyphs: BTreeMap::new(),
        });
        Ok(FontId(self.fonts.len().saturating_sub(1)))
    }

    /// Encode `text` for `font`. Standard fonts use `WinAnsi` (Latin-1 plus the Windows 1252
    /// punctuation block, euro sign included); embedded fonts use `Font::glyph_for`.
    ///
    /// # Errors
    /// `Invalid` for an unknown font; `Unsupported` for a character the font cannot show under
    /// [`TextPolicy::Reject`].
    pub fn encode_text(&mut self, font: FontId, text: &str, policy: TextPolicy) -> Result<TextRun> {
        let limits = self.limits;
        let kind = self
            .fonts
            .get_mut(font.index())
            .ok_or(FolioError::Invalid("unknown font"))?;
        let mut bytes = Vec::new();
        let mut substituted = 0_usize;
        let hex = match kind {
            FontKind::Std(_) => {
                for c in text.chars() {
                    if let Some(b) = winansi(c) {
                        bytes.push(b);
                    } else if policy == TextPolicy::Substitute {
                        bytes.push(b'?');
                        substituted = substituted.saturating_add(1);
                    } else {
                        return Err(FolioError::Unsupported("character has no WinAnsi code"));
                    }
                }
                false
            }
            FontKind::Ttf { data, used, glyphs } => {
                if text.chars().any(|c| !glyphs.contains_key(&c)) {
                    self.parses = self.parses.saturating_add(1);
                    let f = Font::parse(data, &limits)?;
                    for c in text.chars() {
                        glyphs.entry(c).or_insert_with(|| f.glyph_for(c));
                    }
                }
                let mut pending = Vec::new();
                for c in text.chars() {
                    match glyphs.get(&c).copied().flatten() {
                        Some(g) => pending.push((g, Some(c))),
                        None if policy == TextPolicy::Substitute => {
                            pending.push((0, None));
                            substituted = substituted.saturating_add(1);
                        }
                        None => return Err(FolioError::Unsupported("character not in font")),
                    }
                }
                for (g, c) in pending {
                    bytes.extend_from_slice(&g.to_be_bytes());
                    if let Some(c) = c {
                        used.entry(g).or_insert(c);
                    }
                }
                true
            }
        };
        if bytes.len() > limits.max_entry_bytes() {
            return Err(FolioError::Limit("text run size"));
        }
        Ok(TextRun {
            font,
            bytes,
            hex,
            substituted,
        })
    }

    /// Bound a count of destinations or bookmarks, which are far more numerous than pages.
    fn check_many(&self, n: usize, what: &'static str) -> Result<()> {
        if n >= self.limits.max_cells() {
            return Err(FolioError::Limit(what));
        }
        Ok(())
    }

    /// Set the page labels: each `(page index, number)` starts a run of decimal labels counting
    /// up from `number` at that page. The first run must start at page 0.
    ///
    /// # Errors
    /// `Invalid` unless the indices start at 0 and strictly increase and every number is at
    /// least 1; `Limit` for more runs than `max_zip_entries`. Indices past the last page are
    /// rejected by [`PdfDoc::build`].
    pub fn set_page_labels(&mut self, runs: Vec<(usize, u32)>) -> Result<()> {
        self.check_count(runs.len())?;
        if runs.len().saturating_mul(2) > crate::trees::MAX_ARRAY {
            return Err(FolioError::Limit("page label runs"));
        }
        let starts_at_zero = runs.first().is_none_or(|(i, _)| *i == 0);
        let increasing = runs.iter().zip(runs.iter().skip(1)).all(|(a, b)| a.0 < b.0);
        if !starts_at_zero || !increasing || runs.iter().any(|(_, n)| *n == 0) {
            return Err(FolioError::Invalid("page label runs are not well formed"));
        }
        self.labels = runs;
        Ok(())
    }

    fn check_count(&self, n: usize) -> Result<()> {
        if n >= self.limits.max_zip_entries() {
            return Err(FolioError::Limit("object count"));
        }
        Ok(())
    }

    /// Append a page and return its zero-based index. URI links are validated here; page and
    /// named-destination references are checked by [`PdfDoc::build`].
    ///
    /// # Errors
    /// `Invalid` for a non-positive or over-200-inch page size, an unfinished content stream or
    /// a bad URI; `Limit` for oversize content or too many pages or links.
    pub fn add_page(
        &mut self,
        width: Emu,
        height: Emu,
        content: &Content,
        links: Vec<Link>,
    ) -> Result<usize> {
        let max = EMU_PER_INCH.saturating_mul(200);
        if [width, height]
            .iter()
            .any(|e| e.raw() <= 0 || e.raw() > max)
        {
            return Err(FolioError::Invalid("page size out of range"));
        }
        self.check_count(self.pages.len())?;
        self.check_count(links.len())?;
        if links.len() > crate::trees::MAX_ARRAY {
            return Err(FolioError::Limit("links per page"));
        }
        for l in &links {
            if let LinkTarget::Uri(u) = &l.target {
                validate_uri(u)?;
            }
        }
        let (bytes, fonts) = content.finish()?;
        if bytes.len() > self.limits.max_entry_bytes() {
            return Err(FolioError::Limit("content stream size"));
        }
        if fonts.iter().any(|f| *f >= self.fonts.len()) {
            return Err(FolioError::Invalid("content uses an unregistered font"));
        }
        self.pages.push(PageData {
            width,
            height,
            content: bytes.to_vec(),
            fonts: fonts.clone(),
            links,
            mcids: content.mcid_count(),
        });
        Ok(self.pages.len().saturating_sub(1))
    }

    /// Add a named destination, usable from [`LinkTarget::Named`] and written to `/Names`.
    ///
    /// # Errors
    /// `Invalid` for an empty or duplicate name; `Limit` for a long name or too many names.
    pub fn add_named_dest(&mut self, name: &str, dest: Dest) -> Result<()> {
        if name.is_empty() {
            return Err(FolioError::Invalid("empty destination name"));
        }
        if name.len() > self.limits.max_name_bytes() {
            return Err(FolioError::Limit("destination name length"));
        }
        self.check_many(self.dests.len(), "destination count")?;
        if self.dests.insert(name.to_owned(), dest).is_some() {
            return Err(FolioError::Invalid("duplicate destination name"));
        }
        Ok(())
    }

    /// Replace the bookmark tree.
    ///
    /// # Errors
    /// `Limit` when the tree is deeper than `max_depth` or has too many entries; `Invalid` for an
    /// empty title.
    pub fn set_outline(&mut self, items: Vec<Bookmark>) -> Result<()> {
        let mut count = 0_usize;
        self.check_outline(&items, 1, &mut count)?;
        self.outline = items;
        Ok(())
    }

    fn check_outline(&self, items: &[Bookmark], depth: usize, count: &mut usize) -> Result<()> {
        if depth > self.limits.max_depth() {
            return Err(FolioError::Limit("outline depth"));
        }
        for b in items {
            *count = count.saturating_add(1);
            self.check_many(*count, "bookmark count")?;
            if b.title.is_empty() {
                return Err(FolioError::Invalid("empty bookmark title"));
            }
            if b.title.len() > self.limits.max_name_bytes().saturating_mul(8) {
                return Err(FolioError::Limit("bookmark title length"));
            }
            self.check_outline(&b.children, depth.saturating_add(1), count)?;
        }
        Ok(())
    }
}
