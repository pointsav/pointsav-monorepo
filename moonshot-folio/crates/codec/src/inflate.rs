// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! DEFLATE (RFC 1951) and zlib (RFC 1950) decoding with an absolute output cap.

use crate::bits::BitReader;
use crate::huffman::Huffman;
use crate::{adler32, le::u32_at};
use moonshot_folio_core::{FolioError, Result};

const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];
const CL_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

/// Decompress a raw DEFLATE stream; the output may not exceed `max_out` bytes.
///
/// # Errors
/// Malformed or truncated input, or output beyond `max_out`.
pub fn inflate(data: &[u8], max_out: usize) -> Result<Vec<u8>> {
    inflate_stream(data, max_out).map(|(out, _)| out)
}

/// Decompress a zlib stream (header, DEFLATE, Adler-32 trailer).
///
/// # Errors
/// Bad header, preset dictionary, malformed DEFLATE, checksum mismatch, or output beyond `max_out`.
pub fn zlib_decompress(data: &[u8], max_out: usize) -> Result<Vec<u8>> {
    let cmf = *data
        .first()
        .ok_or(FolioError::Malformed("empty zlib stream"))?;
    let flg = *data
        .get(1)
        .ok_or(FolioError::Malformed("short zlib header"))?;
    let check = u16::from(cmf)
        .wrapping_mul(256)
        .wrapping_add(u16::from(flg));
    if cmf & 0x0F != 8 || cmf >> 4 > 7 || check % 31 != 0 {
        return Err(FolioError::Malformed("bad zlib header"));
    }
    if flg & 0x20 != 0 {
        return Err(FolioError::Unsupported("zlib preset dictionary"));
    }
    let body = data
        .get(2..)
        .ok_or(FolioError::Malformed("short zlib header"))?;
    let (out, used) = inflate_stream(body, max_out)?;
    let trailer = u32_at(body, used)?.swap_bytes();
    if trailer != adler32(&out) {
        return Err(FolioError::Malformed("adler-32 mismatch"));
    }
    Ok(out)
}

/// Inflate and report how many input bytes the DEFLATE stream used.
fn inflate_stream(data: &[u8], max_out: usize) -> Result<(Vec<u8>, usize)> {
    let mut br = BitReader::new(data);
    let mut out = Vec::new();
    let mut fixed: Option<(Huffman, Huffman)> = None;
    loop {
        let last = br.bit()?;
        match br.bits(2)? {
            0 => stored(&mut br, &mut out, max_out)?,
            1 => {
                if fixed.is_none() {
                    fixed = Some(fixed_codes()?);
                }
                if let Some((lit, dist)) = &fixed {
                    codes(&mut br, &mut out, max_out, lit, dist)?;
                }
            }
            2 => {
                let (lit, dist) = dynamic_codes(&mut br)?;
                codes(&mut br, &mut out, max_out, &lit, &dist)?;
            }
            _ => return Err(FolioError::Malformed("reserved deflate block type")),
        }
        if last == 1 {
            return Ok((out, br.consumed()));
        }
    }
}

/// A stored (uncompressed) block.
fn stored(br: &mut BitReader<'_>, out: &mut Vec<u8>, max_out: usize) -> Result<()> {
    let head = br.take_bytes(4)?;
    let len = u16::from_le_bytes([
        head.first().copied().unwrap_or(0),
        head.get(1).copied().unwrap_or(0),
    ]);
    let nlen = u16::from_le_bytes([
        head.get(2).copied().unwrap_or(0),
        head.get(3).copied().unwrap_or(0),
    ]);
    if len != !nlen {
        return Err(FolioError::Malformed("stored block length check failed"));
    }
    if out.len().saturating_add(usize::from(len)) > max_out {
        return Err(FolioError::Limit("inflate output"));
    }
    out.extend_from_slice(br.take_bytes(usize::from(len))?);
    Ok(())
}

/// The fixed literal/length and distance codes.
fn fixed_codes() -> Result<(Huffman, Huffman)> {
    let lit: Vec<u8> = (0_u16..288)
        .map(|i| match i {
            144..=255 => 9,
            256..=279 => 7,
            _ => 8,
        })
        .collect();
    Ok((Huffman::new(&lit)?.0, Huffman::new(&[5_u8; 30])?.0))
}

/// Read a dynamic block header and build its two codes.
fn dynamic_codes(br: &mut BitReader<'_>) -> Result<(Huffman, Huffman)> {
    let nlen = usize::try_from(br.bits(5)?)
        .unwrap_or(0)
        .saturating_add(257);
    let ndist = usize::try_from(br.bits(5)?).unwrap_or(0).saturating_add(1);
    let ncode = usize::try_from(br.bits(4)?).unwrap_or(0).saturating_add(4);
    if nlen > 286 || ndist > 30 {
        return Err(FolioError::Malformed("too many length or distance codes"));
    }
    let mut cl = [0_u8; 19];
    for &slot in CL_ORDER.iter().take(ncode) {
        let v = u8::try_from(br.bits(3)?).unwrap_or(0);
        *cl.get_mut(slot)
            .ok_or(FolioError::Malformed("code length slot"))? = v;
    }
    let (clcode, complete) = Huffman::new(&cl)?;
    if !complete {
        return Err(FolioError::Malformed("incomplete code-length code"));
    }
    let lengths = read_lengths(br, &clcode, nlen.saturating_add(ndist))?;
    if lengths.get(256).copied().unwrap_or(0) == 0 {
        return Err(FolioError::Malformed("missing end-of-block code"));
    }
    let (lit_l, dist_l) = lengths
        .split_at_checked(nlen)
        .ok_or(FolioError::Malformed("length table split"))?;
    Ok((checked_code(lit_l)?, checked_code(dist_l)?))
}

/// Build a dynamic code; like zlib, an incomplete code is legal only when its longest code is 1 bit.
fn checked_code(lengths: &[u8]) -> Result<Huffman> {
    let (h, complete) = Huffman::new(lengths)?;
    if !complete && h.max_len() > 1 {
        return Err(FolioError::Malformed("incomplete huffman code"));
    }
    Ok(h)
}

/// Decode `total` code lengths using the code-length code.
fn read_lengths(br: &mut BitReader<'_>, clcode: &Huffman, total: usize) -> Result<Vec<u8>> {
    let mut lengths: Vec<u8> = Vec::with_capacity(total);
    while lengths.len() < total {
        let sym = clcode.decode(br)?;
        let (value, repeat) = match sym {
            0..=15 => (u8::try_from(sym).unwrap_or(0), 1),
            16 => {
                let prev = *lengths
                    .last()
                    .ok_or(FolioError::Malformed("repeat with no previous length"))?;
                (prev, br.bits(2)?.saturating_add(3))
            }
            17 => (0, br.bits(3)?.saturating_add(3)),
            _ => (0, br.bits(7)?.saturating_add(11)),
        };
        let repeat = usize::try_from(repeat).unwrap_or(usize::MAX);
        if lengths.len().saturating_add(repeat) > total {
            return Err(FolioError::Malformed(
                "code length repeat overruns the table",
            ));
        }
        lengths.extend(core::iter::repeat_n(value, repeat));
    }
    Ok(lengths)
}

/// Decode literals and matches until the end-of-block symbol.
fn codes(
    br: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    max_out: usize,
    lit: &Huffman,
    dist: &Huffman,
) -> Result<()> {
    loop {
        let sym = lit.decode(br)?;
        match sym {
            0..=255 => {
                if out.len() >= max_out {
                    return Err(FolioError::Limit("inflate output"));
                }
                out.push(u8::try_from(sym).unwrap_or(0));
            }
            256 => return Ok(()),
            257..=285 => copy_match(br, out, max_out, sym, dist)?,
            _ => return Err(FolioError::Malformed("invalid length symbol")),
        }
    }
}

/// Decode one length/distance pair and copy the match.
fn copy_match(
    br: &mut BitReader<'_>,
    out: &mut Vec<u8>,
    max_out: usize,
    sym: u16,
    dist: &Huffman,
) -> Result<()> {
    let li = usize::from(sym.saturating_sub(257));
    let base = LEN_BASE
        .get(li)
        .copied()
        .ok_or(FolioError::Malformed("length index"))?;
    let extra = LEN_EXTRA
        .get(li)
        .copied()
        .ok_or(FolioError::Malformed("length index"))?;
    let len =
        usize::from(base).saturating_add(usize::try_from(br.bits(u32::from(extra))?).unwrap_or(0));
    let di = usize::from(dist.decode(br)?);
    let dbase = DIST_BASE
        .get(di)
        .copied()
        .ok_or(FolioError::Malformed("distance symbol"))?;
    let dextra = DIST_EXTRA
        .get(di)
        .copied()
        .ok_or(FolioError::Malformed("distance symbol"))?;
    let distance = usize::from(dbase)
        .saturating_add(usize::try_from(br.bits(u32::from(dextra))?).unwrap_or(0));
    if distance > out.len() {
        return Err(FolioError::Malformed(
            "match distance before start of output",
        ));
    }
    if out.len().saturating_add(len) > max_out {
        return Err(FolioError::Limit("inflate output"));
    }
    for _ in 0..len {
        let b = *out
            .get(out.len().saturating_sub(distance))
            .ok_or(FolioError::Malformed("match source"))?;
        out.push(b);
    }
    Ok(())
}
