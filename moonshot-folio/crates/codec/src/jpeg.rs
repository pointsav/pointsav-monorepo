// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! JPEG header parsing. No pixel decoding: PDF and OOXML embed the DCT stream unchanged and need
//! only the frame facts read here.

use crate::le::{add, be16_at, slice};
use moonshot_folio_core::{FolioError, Limits, Result};

/// Facts from a JPEG start-of-frame segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JpegInfo {
    width: u32,
    height: u32,
    components: u8,
    precision: u8,
    progressive: bool,
    adobe_transform: Option<u8>,
}

impl JpegInfo {
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
    /// Colour components: 1 (gray), 3 (YCbCr or RGB) or 4 (CMYK or YCCK).
    #[must_use]
    pub fn components(&self) -> u8 {
        self.components
    }
    /// Bits per sample (8 for baseline).
    #[must_use]
    pub fn precision(&self) -> u8 {
        self.precision
    }
    /// True for a progressive (SOF2) frame.
    #[must_use]
    pub fn progressive(&self) -> bool {
        self.progressive
    }
    /// The Adobe APP14 colour transform flag, if present (CMYK files usually need an inverted decode array).
    #[must_use]
    pub fn adobe_transform(&self) -> Option<u8> {
        self.adobe_transform
    }
}

/// Frame fields found so far while scanning segments.
struct Frame {
    width: u32,
    height: u32,
    components: u8,
    precision: u8,
    progressive: bool,
}

/// Read the frame header of a JPEG file.
///
/// # Errors
/// Missing SOI, truncated or malformed segments, an unsupported frame type (lossless, arithmetic,
/// hierarchical), zero or oversized dimensions, or a component count other than 1, 3 or 4.
pub fn parse_jpeg_header(data: &[u8], limits: &Limits) -> Result<JpegInfo> {
    if data.len() > limits.max_file_bytes() {
        return Err(FolioError::Limit("jpeg file size"));
    }
    if slice(data, 0, 2)? != [0xFF, 0xD8] {
        return Err(FolioError::Malformed("missing JPEG start-of-image"));
    }
    let mut pos = 2_usize;
    let mut frame: Option<Frame> = None;
    let mut adobe = None;
    loop {
        let marker = next_marker(data, &mut pos)?;
        match marker {
            0xD9 | 0xDA => break,
            0xD0..=0xD8 | 0x01 => continue,
            _ => {}
        }
        let len = usize::from(be16_at(data, pos)?);
        if len < 2 {
            return Err(FolioError::Malformed("jpeg segment length below 2"));
        }
        let payload = slice(data, add(pos, 2)?, len.saturating_sub(2))?;
        match marker {
            0xC0..=0xC2 => {
                if frame.is_some() {
                    return Err(FolioError::Malformed("duplicate frame header"));
                }
                frame = Some(read_frame(marker, payload, limits)?);
            }
            0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF => {
                return Err(FolioError::Unsupported("jpeg frame type"));
            }
            0xEE => adobe = adobe_transform(payload).or(adobe),
            _ => {}
        }
        pos = add(pos, len)?;
    }
    let f = frame.ok_or(FolioError::Malformed("no JPEG frame header before scan"))?;
    Ok(JpegInfo {
        width: f.width,
        height: f.height,
        components: f.components,
        precision: f.precision,
        progressive: f.progressive,
        adobe_transform: adobe,
    })
}

/// Advance to the next marker, skipping `0xFF` fill bytes; returns the marker code.
fn next_marker(data: &[u8], pos: &mut usize) -> Result<u8> {
    if *slice(data, *pos, 1)?.first().unwrap_or(&0) != 0xFF {
        return Err(FolioError::Malformed("expected a JPEG marker"));
    }
    loop {
        *pos = add(*pos, 1)?;
        let b = *slice(data, *pos, 1)?.first().unwrap_or(&0);
        if b != 0xFF {
            if b == 0x00 {
                return Err(FolioError::Malformed(
                    "stuffed byte where a marker was expected",
                ));
            }
            *pos = add(*pos, 1)?;
            return Ok(b);
        }
    }
}

/// Decode a SOF0/1/2 payload.
fn read_frame(marker: u8, payload: &[u8], limits: &Limits) -> Result<Frame> {
    let precision = *payload
        .first()
        .ok_or(FolioError::Malformed("short frame header"))?;
    let height = u32::from(be16_at(payload, 1)?);
    let width = u32::from(be16_at(payload, 3)?);
    let components = *payload
        .get(5)
        .ok_or(FolioError::Malformed("short frame header"))?;
    if width == 0 || height == 0 {
        return Err(FolioError::Malformed("jpeg dimension is zero"));
    }
    if !matches!(components, 1 | 3 | 4) {
        return Err(FolioError::Unsupported("jpeg component count"));
    }
    if !matches!(precision, 8 | 12) || (precision == 12 && marker == 0xC0) {
        return Err(FolioError::Unsupported("jpeg sample precision"));
    }
    let pixels = usize::try_from(width)
        .unwrap_or(usize::MAX)
        .saturating_mul(usize::try_from(height).unwrap_or(usize::MAX));
    if pixels > limits.max_image_pixels() {
        return Err(FolioError::Limit("image pixels"));
    }
    let want = usize::from(components).saturating_mul(3);
    if payload.len() != want.saturating_add(6) {
        return Err(FolioError::Malformed(
            "frame header length does not match its component count",
        ));
    }
    for comp in slice(payload, 6, want)?.chunks_exact(3) {
        let hv = comp.get(1).copied().unwrap_or(0);
        let (h, v, tq) = (hv >> 4, hv & 0x0F, comp.get(2).copied().unwrap_or(0));
        if !(1..=4).contains(&h) || !(1..=4).contains(&v) || tq > 3 {
            return Err(FolioError::Malformed(
                "invalid sampling factor or quantisation selector",
            ));
        }
    }
    Ok(Frame {
        width,
        height,
        components,
        precision,
        progressive: marker == 0xC2,
    })
}

/// The transform byte of an Adobe APP14 segment, if this is one.
fn adobe_transform(payload: &[u8]) -> Option<u8> {
    if payload.get(..5) == Some(b"Adobe".as_slice()) {
        return payload.get(11).copied();
    }
    None
}
