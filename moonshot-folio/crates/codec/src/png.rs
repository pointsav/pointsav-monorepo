// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! PNG decoding (non-interlaced, all colour types and depths) and conversion to 8-bit RGBA.

use crate::le::{add, be32_at, slice};
use crate::png_filter::unfilter;
use crate::{crc32, zlib_decompress};
use moonshot_folio_core::{FolioError, Limits, Result};

const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

/// PNG colour type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PngColor {
    /// One gray sample per pixel.
    Gray,
    /// Red, green, blue.
    Rgb,
    /// Index into a palette.
    Palette,
    /// Gray plus alpha.
    GrayAlpha,
    /// Red, green, blue plus alpha.
    Rgba,
}

impl PngColor {
    /// Samples stored per pixel.
    fn channels(self) -> usize {
        match self {
            Self::Gray | Self::Palette => 1,
            Self::GrayAlpha => 2,
            Self::Rgb => 3,
            Self::Rgba => 4,
        }
    }

    /// From the IHDR colour type byte, checking the bit depth is legal for it.
    fn from_ihdr(color_type: u8, depth: u8) -> Result<Self> {
        let (color, ok) = match color_type {
            0 => (Self::Gray, matches!(depth, 1 | 2 | 4 | 8 | 16)),
            2 => (Self::Rgb, matches!(depth, 8 | 16)),
            3 => (Self::Palette, matches!(depth, 1 | 2 | 4 | 8)),
            4 => (Self::GrayAlpha, matches!(depth, 8 | 16)),
            6 => (Self::Rgba, matches!(depth, 8 | 16)),
            _ => return Err(FolioError::Malformed("unknown png colour type")),
        };
        if ok {
            Ok(color)
        } else {
            Err(FolioError::Malformed(
                "illegal png bit depth for colour type",
            ))
        }
    }
}

/// A decoded PNG: unfiltered, bit-packed scanlines plus palette and transparency.
#[derive(Clone, Debug)]
pub struct PngImage {
    width: u32,
    height: u32,
    color: PngColor,
    depth: u8,
    pixels: Vec<u8>,
    palette: Vec<[u8; 3]>,
    trns_key: Option<[u16; 3]>,
    trns_alpha: Vec<u8>,
}

impl PngImage {
    /// Width in pixels.
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Height in pixels.
    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Colour type.
    #[must_use]
    pub fn color(&self) -> PngColor {
        self.color
    }
    /// Bits per sample.
    #[must_use]
    pub fn bit_depth(&self) -> u8 {
        self.depth
    }
    /// Unfiltered scanlines, rows tightly packed (sub-byte samples stay packed).
    #[must_use]
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
    /// The palette (empty unless the colour type is `Palette`).
    #[must_use]
    pub fn palette(&self) -> &[[u8; 3]] {
        &self.palette
    }

    /// Convert to 8-bit RGBA, four bytes per pixel, rows top to bottom.
    ///
    /// # Errors
    /// The result would exceed `limits.max_entry_bytes()`, or a palette index is out of range.
    pub fn to_rgba8(&self, limits: &Limits) -> Result<Vec<u8>> {
        let (w, h) = (to_usize(self.width)?, to_usize(self.height)?);
        let total = w.saturating_mul(h).saturating_mul(4);
        if total > limits.max_entry_bytes() {
            return Err(FolioError::Limit("rgba output size"));
        }
        let stride = row_stride(w, self.color, self.depth)?;
        let mut out = Vec::with_capacity(total);
        for row in self.pixels.chunks_exact(stride.max(1)) {
            for x in 0..w {
                out.extend_from_slice(&self.rgba_at(row, x)?);
            }
        }
        Ok(out)
    }

    /// One pixel as RGBA8.
    fn rgba_at(&self, row: &[u8], x: usize) -> Result<[u8; 4]> {
        let ch = self.color.channels();
        let s = |c: usize| sample(row, x.saturating_mul(ch).saturating_add(c), self.depth);
        let scale = |v: u16| scale8(v, self.depth);
        let keyed = |raw: [u16; 3]| if self.trns_key == Some(raw) { 0 } else { 255 };
        Ok(match self.color {
            PngColor::Gray => {
                let g = scale(s(0));
                [g, g, g, keyed([s(0), 0, 0])]
            }
            PngColor::GrayAlpha => {
                let g = scale(s(0));
                [g, g, g, scale(s(1))]
            }
            PngColor::Rgb => [
                scale(s(0)),
                scale(s(1)),
                scale(s(2)),
                keyed([s(0), s(1), s(2)]),
            ],
            PngColor::Rgba => [scale(s(0)), scale(s(1)), scale(s(2)), scale(s(3))],
            PngColor::Palette => {
                let i = usize::from(s(0));
                let rgb = self
                    .palette
                    .get(i)
                    .ok_or(FolioError::Malformed("palette index out of range"))?;
                let alpha = self.trns_alpha.get(i).copied().unwrap_or(255);
                [byte(rgb, 0), byte(rgb, 1), byte(rgb, 2), alpha]
            }
        })
    }
}

/// Byte `i` of `b`, or 0 when absent (callers check lengths first).
fn byte(b: &[u8], i: usize) -> u8 {
    b.get(i).copied().unwrap_or(0)
}

/// Widen a `u32` to `usize`.
fn to_usize(v: u32) -> Result<usize> {
    usize::try_from(v).map_err(|_| FolioError::Limit("value exceeds address space"))
}

/// Bytes per scanline (without the filter byte).
fn row_stride(width: usize, color: PngColor, depth: u8) -> Result<usize> {
    let bits = width
        .checked_mul(color.channels())
        .and_then(|v| v.checked_mul(usize::from(depth)))
        .ok_or(FolioError::Limit("scanline size"))?;
    Ok(bits.div_ceil(8))
}

/// Raw sample `idx` of a scanline, as an unsigned value of `depth` bits.
fn sample(row: &[u8], idx: usize, depth: u8) -> u16 {
    match depth {
        16 => {
            let at = idx.saturating_mul(2);
            u16::from_be_bytes([
                row.get(at).copied().unwrap_or(0),
                row.get(at.saturating_add(1)).copied().unwrap_or(0),
            ])
        }
        8 => u16::from(row.get(idx).copied().unwrap_or(0)),
        d => {
            let bit = idx.saturating_mul(usize::from(d));
            let byte = row.get(bit / 8).copied().unwrap_or(0);
            let shift = 8_usize
                .saturating_sub(usize::from(d))
                .saturating_sub(bit % 8);
            let mask = 1_u16.wrapping_shl(u32::from(d)).wrapping_sub(1);
            (u16::from(byte).wrapping_shr(u32::try_from(shift).unwrap_or(0))) & mask
        }
    }
}

/// Scale a `depth`-bit sample to 0..=255.
fn scale8(v: u16, depth: u8) -> u8 {
    match depth {
        16 => u8::try_from(v >> 8).unwrap_or(255),
        8 => u8::try_from(v).unwrap_or(255),
        d => {
            let max = 1_u32.wrapping_shl(u32::from(d)).wrapping_sub(1);
            u8::try_from(
                u32::from(v)
                    .saturating_mul(255)
                    .checked_div(max)
                    .unwrap_or(255),
            )
            .unwrap_or(255)
        }
    }
}

/// Header fields from IHDR.
#[derive(Clone, Copy)]
struct Header {
    width: u32,
    height: u32,
    color: PngColor,
    depth: u8,
}

/// Where the IDAT run stands.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum IdatState {
    #[default]
    NotSeen,
    Open,
    Closed,
}

/// Everything collected from the chunks between IHDR and IEND.
#[derive(Default)]
struct Parts {
    palette: Vec<[u8; 3]>,
    trns_key: Option<[u16; 3]>,
    trns_alpha: Vec<u8>,
    idat: Vec<u8>,
    seen_trns: bool,
    idat_state: IdatState,
}

/// Decode a PNG file.
///
/// Chunk order follows the PNG specification: IHDR, optional PLTE, optional tRNS, consecutive IDAT
/// chunks, then an empty IEND with nothing after it.
///
/// # Errors
/// Bad signature or chunk CRC, structural errors, interlaced images or unknown critical chunks
/// (`Unsupported`), and any [`Limits`] value exceeded (pixels, output size).
pub fn decode_png(data: &[u8], limits: &Limits) -> Result<PngImage> {
    if data.len() > limits.max_file_bytes() {
        return Err(FolioError::Limit("png file size"));
    }
    if slice(data, 0, 8)? != SIGNATURE {
        return Err(FolioError::Malformed("bad png signature"));
    }
    let (kind, body, mut pos) = read_chunk(data, 8)?;
    if &kind != b"IHDR" {
        return Err(FolioError::Malformed("IHDR must come first"));
    }
    let header = read_header(body, limits)?;
    let mut parts = Parts::default();
    loop {
        let (kind, body, next) = read_chunk(data, pos)?;
        pos = next;
        if apply_chunk(&mut parts, kind, body, header)? {
            break;
        }
    }
    if pos != data.len() {
        return Err(FolioError::Malformed("data after IEND"));
    }
    finish(header, parts, limits)
}

/// Apply one chunk after IHDR; returns true at IEND.
fn apply_chunk(parts: &mut Parts, kind: [u8; 4], body: &[u8], h: Header) -> Result<bool> {
    if &kind == b"IDAT" {
        if parts.idat_state == IdatState::Closed {
            return Err(FolioError::Malformed("non-consecutive IDAT chunks"));
        }
        parts.idat_state = IdatState::Open;
        parts.idat.extend_from_slice(body);
        return Ok(false);
    }
    if parts.idat_state == IdatState::Open {
        parts.idat_state = IdatState::Closed;
    }
    match &kind {
        b"PLTE" => read_plte(parts, body, h)?,
        b"tRNS" => read_trns(parts, body, h)?,
        b"IEND" => {
            if !body.is_empty() || parts.idat_state == IdatState::NotSeen {
                return Err(FolioError::Malformed("IEND must be empty and follow IDAT"));
            }
            return Ok(true);
        }
        b"IHDR" => return Err(FolioError::Malformed("duplicate IHDR")),
        k if k.first().is_some_and(u8::is_ascii_uppercase) => {
            return Err(FolioError::Unsupported("unknown critical png chunk"));
        }
        _ => {}
    }
    Ok(false)
}

/// PLTE: once, before tRNS and IDAT, and only for colour types that allow it.
fn read_plte(parts: &mut Parts, body: &[u8], h: Header) -> Result<()> {
    if !parts.palette.is_empty() || parts.seen_trns || parts.idat_state != IdatState::NotSeen {
        return Err(FolioError::Malformed("misplaced or repeated PLTE"));
    }
    if matches!(h.color, PngColor::Gray | PngColor::GrayAlpha) {
        return Err(FolioError::Malformed(
            "PLTE not allowed for this colour type",
        ));
    }
    parts.palette = read_palette(body)?;
    let room = 1_usize
        .checked_shl(u32::from(h.depth))
        .unwrap_or(usize::MAX);
    if h.color == PngColor::Palette && parts.palette.len() > room {
        return Err(FolioError::Malformed(
            "PLTE has more entries than the bit depth allows",
        ));
    }
    Ok(())
}

/// tRNS: once, before IDAT, with the exact length the colour type requires.
fn read_trns(parts: &mut Parts, body: &[u8], h: Header) -> Result<()> {
    if parts.seen_trns || parts.idat_state != IdatState::NotSeen {
        return Err(FolioError::Malformed("misplaced or repeated tRNS"));
    }
    let word = |i: usize| u16::from_be_bytes([byte(body, i), byte(body, i.saturating_add(1))]);
    match h.color {
        PngColor::Gray if body.len() == 2 => parts.trns_key = Some([word(0), 0, 0]),
        PngColor::Rgb if body.len() == 6 => parts.trns_key = Some([word(0), word(2), word(4)]),
        PngColor::Palette if !parts.palette.is_empty() && body.len() <= parts.palette.len() => {
            parts.trns_alpha = body.to_vec();
        }
        _ => {
            return Err(FolioError::Malformed(
                "tRNS length or colour type is invalid",
            ))
        }
    }
    parts.seen_trns = true;
    Ok(())
}

/// One chunk at `pos`: type, body (CRC verified) and the offset of the next chunk.
fn read_chunk(data: &[u8], pos: usize) -> Result<([u8; 4], &[u8], usize)> {
    let len = to_usize(be32_at(data, pos)?)?;
    let kind = <[u8; 4]>::try_from(slice(data, add(pos, 4)?, 4)?)
        .map_err(|_| FolioError::Malformed("chunk type"))?;
    if !kind.iter().all(u8::is_ascii_alphabetic) || !kind.get(2).is_some_and(u8::is_ascii_uppercase)
    {
        return Err(FolioError::Malformed("invalid chunk type"));
    }
    let body = slice(data, add(pos, 8)?, len)?;
    let crc_at = add(add(pos, 8)?, len)?;
    let covered = slice(data, add(pos, 4)?, add(len, 4)?)?;
    if crc32(covered) != be32_at(data, crc_at)? {
        return Err(FolioError::Malformed("png chunk crc mismatch"));
    }
    Ok((kind, body, add(crc_at, 4)?))
}

/// Decode and validate IHDR.
fn read_header(body: &[u8], limits: &Limits) -> Result<Header> {
    if body.len() != 13 {
        return Err(FolioError::Malformed("IHDR length"));
    }
    let width = be32_at(body, 0)?;
    let height = be32_at(body, 4)?;
    let (depth, ctype, compression, filter, interlace) = (
        byte(body, 8),
        byte(body, 9),
        byte(body, 10),
        byte(body, 11),
        byte(body, 12),
    );
    if width == 0 || height == 0 {
        return Err(FolioError::Malformed("png dimension is zero"));
    }
    if compression != 0 || filter != 0 {
        return Err(FolioError::Malformed("png compression or filter method"));
    }
    if interlace != 0 {
        return Err(FolioError::Unsupported("interlaced png"));
    }
    let pixels = to_usize(width)?.saturating_mul(to_usize(height)?);
    if pixels > limits.max_image_pixels() {
        return Err(FolioError::Limit("image pixels"));
    }
    Ok(Header {
        width,
        height,
        color: PngColor::from_ihdr(ctype, depth)?,
        depth,
    })
}

/// PLTE: groups of three bytes, at most 256 entries.
fn read_palette(body: &[u8]) -> Result<Vec<[u8; 3]>> {
    if body.is_empty() || !body.len().is_multiple_of(3) || body.len() > 768 {
        return Err(FolioError::Malformed("PLTE length"));
    }
    Ok(body
        .chunks_exact(3)
        .map(|c| [byte(c, 0), byte(c, 1), byte(c, 2)])
        .collect())
}

/// Inflate IDAT with an exact-size cap, then unfilter.
fn finish(h: Header, parts: Parts, limits: &Limits) -> Result<PngImage> {
    if h.color == PngColor::Palette && parts.palette.is_empty() {
        return Err(FolioError::Malformed("palette image without PLTE"));
    }
    let (w, ht) = (to_usize(h.width)?, to_usize(h.height)?);
    let stride = row_stride(w, h.color, h.depth)?;
    let expected = stride
        .checked_add(1)
        .and_then(|r| r.checked_mul(ht))
        .ok_or(FolioError::Limit("png data size"))?;
    if expected > limits.max_entry_bytes() {
        return Err(FolioError::Limit("png data size"));
    }
    let raw = zlib_decompress(&parts.idat, expected)?;
    if raw.len() != expected {
        return Err(FolioError::Malformed(
            "png data shorter than the header implies",
        ));
    }
    let bits_pp = h.color.channels().saturating_mul(usize::from(h.depth));
    let bpp = bits_pp.div_ceil(8).max(1);
    let pixels = unfilter(&raw, stride, bpp)?;
    Ok(PngImage {
        width: h.width,
        height: h.height,
        color: h.color,
        depth: h.depth,
        pixels,
        palette: parts.palette,
        trns_key: parts.trns_key,
        trns_alpha: parts.trns_alpha,
    })
}
