// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The `Font` type: sfnt directory and the required tables.

use crate::be::{add, i16_at, mul, slice, to_usize, u16_at, u32_at};
use crate::cmap::Cmap;
use moonshot_folio_core::{FolioError, Limits, Result};

/// A parsed TrueType font borrowing its file bytes.
#[derive(Clone, Debug)]
pub struct Font<'a> {
    data: &'a [u8],
    units_per_em: u16,
    num_glyphs: u16,
    loca_long: bool,
    num_h_metrics: u16,
    ascent: i16,
    descent: i16,
    line_gap: i16,
    bbox: [i16; 4],
    hmtx: &'a [u8],
    loca: &'a [u8],
    glyf: &'a [u8],
    cmap: Option<Cmap<'a>>,
    os2: Option<&'a [u8]>,
    post: Option<&'a [u8]>,
    name: Option<&'a [u8]>,
    tables: usize,
}

/// Find a table by tag in the directory: `(offset, length)` checked against the file.
fn table(data: &[u8], n: usize, tag: [u8; 4]) -> Result<Option<&[u8]>> {
    for i in 0..n {
        let rec = add(12, mul(i, 16)?)?;
        if slice(data, rec, 4)? == tag.as_slice() {
            let off = to_usize(u32_at(data, add(rec, 8)?)?)?;
            let len = to_usize(u32_at(data, add(rec, 12)?)?)?;
            return Ok(Some(slice(data, off, len)?));
        }
    }
    Ok(None)
}

fn required(data: &[u8], n: usize, tag: [u8; 4]) -> Result<&[u8]> {
    table(data, n, tag)?.ok_or(FolioError::Malformed("font lacks a required table"))
}

/// `loca` must be non-decreasing and end inside `glyf`, so glyph ranges never overlap and the
/// total glyph data cannot exceed the `glyf` table (no amplification by aliasing).
fn check_loca(loca: &[u8], entries: usize, long: bool, glyf_len: usize) -> Result<()> {
    let mut prev = 0_usize;
    for k in 0..entries {
        let v = if long {
            to_usize(u32_at(loca, mul(k, 4)?)?)?
        } else {
            mul(usize::from(u16_at(loca, mul(k, 2)?)?), 2)?
        };
        if v < prev {
            return Err(FolioError::Malformed("loca offsets decrease"));
        }
        prev = v;
    }
    if prev > glyf_len {
        return Err(FolioError::Malformed("loca points beyond the glyf table"));
    }
    Ok(())
}

/// Header facts shared by the required tables.
struct Header {
    units_per_em: u16,
    num_glyphs: u16,
    loca_long: bool,
    num_h_metrics: u16,
}

/// Check the sfnt version and table directory; returns the table count.
fn directory(data: &[u8], limits: &Limits) -> Result<usize> {
    if data.len() > limits.max_file_bytes() {
        return Err(FolioError::Limit("font file size"));
    }
    match u32_at(data, 0)? {
        0x0001_0000 | 0x7472_7565 => {}
        0x4F54_544F => return Err(FolioError::Unsupported("CFF outlines (OTTO)")),
        0x7474_6366 => return Err(FolioError::Unsupported("font collections")),
        _ => return Err(FolioError::Malformed("not a TrueType font")),
    }
    let n = usize::from(u16_at(data, 4)?);
    if n == 0 || n > 256 {
        return Err(FolioError::Malformed("table count out of range"));
    }
    slice(data, 12, mul(n, 16)?)?;
    Ok(n)
}

/// Read and range-check `head`, `maxp` and `hhea`.
fn read_header(head: &[u8], maxp: &[u8], hhea: &[u8], limits: &Limits) -> Result<Header> {
    if u32_at(head, 12)? != 0x5F0F_3CF5 {
        return Err(FolioError::Malformed("bad head magic number"));
    }
    let units_per_em = u16_at(head, 18)?;
    if !(16..=16_384).contains(&units_per_em) {
        return Err(FolioError::Malformed("unitsPerEm out of range"));
    }
    let loca_long = match i16_at(head, 50)? {
        0 => false,
        1 => true,
        _ => return Err(FolioError::Malformed("bad indexToLocFormat")),
    };
    let num_glyphs = u16_at(maxp, 4)?;
    if num_glyphs == 0 || usize::from(num_glyphs) > limits.max_glyphs() {
        return Err(FolioError::Limit("glyph count"));
    }
    let num_h_metrics = u16_at(hhea, 34)?;
    if num_h_metrics == 0 || num_h_metrics > num_glyphs {
        return Err(FolioError::Malformed("numberOfHMetrics out of range"));
    }
    Ok(Header {
        units_per_em,
        num_glyphs,
        loca_long,
        num_h_metrics,
    })
}

impl<'a> Font<'a> {
    /// Parse a TrueType font.
    ///
    /// # Errors
    /// `Malformed` for any structural fault, `Unsupported` for CFF (`OTTO`) fonts, collections
    /// and a `cmap` without a Unicode subtable, `Limit` for oversize input or glyph counts.
    pub fn parse(data: &'a [u8], limits: &Limits) -> Result<Self> {
        let n = directory(data, limits)?;
        let head = required(data, n, *b"head")?;
        let hhea = required(data, n, *b"hhea")?;
        let h = read_header(head, required(data, n, *b"maxp")?, hhea, limits)?;
        let hmtx = required(data, n, *b"hmtx")?;
        let tail = usize::from(h.num_glyphs.saturating_sub(h.num_h_metrics));
        slice(
            hmtx,
            0,
            add(mul(usize::from(h.num_h_metrics), 4)?, mul(tail, 2)?)?,
        )?;
        let loca = required(data, n, *b"loca")?;
        let entries = add(usize::from(h.num_glyphs), 1)?;
        slice(loca, 0, mul(entries, if h.loca_long { 4 } else { 2 })?)?;
        let glyf = required(data, n, *b"glyf")?;
        check_loca(loca, entries, h.loca_long, glyf.len())?;
        if head.len() < 54 {
            return Err(FolioError::Malformed("head table is shorter than 54 bytes"));
        }
        let os2 = table(data, n, *b"OS/2")?;
        if os2.is_some_and(|t| t.len() < 10) {
            // Fail closed: a damaged OS/2 must not read as "no embedding restriction".
            return Err(FolioError::Malformed(
                "OS/2 table is too short to hold fsType",
            ));
        }
        Ok(Self {
            data,
            units_per_em: h.units_per_em,
            num_glyphs: h.num_glyphs,
            loca_long: h.loca_long,
            num_h_metrics: h.num_h_metrics,
            ascent: i16_at(hhea, 4)?,
            descent: i16_at(hhea, 6)?,
            line_gap: i16_at(hhea, 8)?,
            bbox: [
                i16_at(head, 36)?,
                i16_at(head, 38)?,
                i16_at(head, 40)?,
                i16_at(head, 42)?,
            ],
            hmtx,
            loca,
            glyf,
            cmap: table(data, n, *b"cmap")?.map(Cmap::parse).transpose()?,
            os2,
            post: table(data, n, *b"post")?,
            name: table(data, n, *b"name")?,
            tables: n,
        })
    }

    /// Design units per em.
    #[must_use]
    pub fn units_per_em(&self) -> u16 {
        self.units_per_em
    }
    /// Number of glyphs.
    #[must_use]
    pub fn num_glyphs(&self) -> u16 {
        self.num_glyphs
    }
    /// Typographic ascent from `hhea`, in design units.
    #[must_use]
    pub fn ascent(&self) -> i16 {
        self.ascent
    }
    /// Typographic descent from `hhea` (negative below the baseline).
    #[must_use]
    pub fn descent(&self) -> i16 {
        self.descent
    }
    /// Line gap from `hhea`.
    #[must_use]
    pub fn line_gap(&self) -> i16 {
        self.line_gap
    }
    /// Font bounding box `[xMin, yMin, xMax, yMax]` from `head`.
    #[must_use]
    pub fn bbox(&self) -> [i16; 4] {
        self.bbox
    }
    /// The bytes of any table by tag, if the font has it.
    #[must_use]
    pub fn table_bytes(&self, tag: [u8; 4]) -> Option<&'a [u8]> {
        table(self.data, self.tables, tag).ok().flatten()
    }
    /// Whether `loca` uses 32-bit offsets.
    #[must_use]
    pub fn loca_is_long(&self) -> bool {
        self.loca_long
    }
    /// The whole file.
    #[must_use]
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Glyph for a character, or `None` when the font does not map it (or has no `cmap`, as a subset
    /// does not).
    #[must_use]
    pub fn glyph_for(&self, c: char) -> Option<u16> {
        self.cmap
            .as_ref()?
            .lookup(c)
            .filter(|g| *g < self.num_glyphs)
    }

    /// Advance width of a glyph in design units.
    ///
    /// # Errors
    /// `Invalid` for a glyph index at or beyond `numGlyphs`.
    pub fn advance(&self, gid: u16) -> Result<u16> {
        if gid >= self.num_glyphs {
            return Err(FolioError::Invalid("glyph index out of range"));
        }
        let i = gid.min(self.num_h_metrics.saturating_sub(1));
        u16_at(self.hmtx, mul(usize::from(i), 4)?)
    }

    /// The raw `glyf` bytes of a glyph (empty for a glyph with no outline).
    ///
    /// # Errors
    /// `Invalid` for a bad glyph index and `Malformed` when `loca` points outside `glyf`.
    pub fn glyph_data(&self, gid: u16) -> Result<&'a [u8]> {
        if gid >= self.num_glyphs {
            return Err(FolioError::Invalid("glyph index out of range"));
        }
        let i = usize::from(gid);
        let at = |k: usize| -> Result<usize> {
            if self.loca_long {
                to_usize(u32_at(self.loca, mul(k, 4)?)?)
            } else {
                mul(usize::from(u16_at(self.loca, mul(k, 2)?)?), 2)
            }
        };
        let (start, end) = (at(i)?, at(add(i, 1)?)?);
        if end < start {
            return Err(FolioError::Malformed("loca offsets decrease"));
        }
        slice(self.glyf, start, end.saturating_sub(start))
    }

    /// The `fsType` embedding flags from `OS/2`, if the table exists.
    #[must_use]
    pub fn fs_type(&self) -> Option<u16> {
        u16_at(self.os2?, 8).ok()
    }
    /// `post.italicAngle` as a 16.16 fixed-point value, if the table exists.
    #[must_use]
    pub fn italic_angle_fixed(&self) -> Option<i32> {
        let b = <[u8; 4]>::try_from(slice(self.post?, 4, 4).ok()?).ok()?;
        Some(i32::from_be_bytes(b))
    }
    /// True when `post.isFixedPitch` is non-zero.
    #[must_use]
    pub fn is_fixed_pitch(&self) -> bool {
        self.post
            .and_then(|p| u32_at(p, 12).ok())
            .is_some_and(|v| v != 0)
    }
    /// The PostScript name (name ID 6), printable ASCII only, if present.
    #[must_use]
    pub fn postscript_name(&self) -> Option<String> {
        crate::name::find(self.name?, 6)
    }
}
