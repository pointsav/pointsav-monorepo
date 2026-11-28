// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Glyph subsetting (task F4.2). Glyph ids are preserved: unused glyphs become empty, so a PDF
//! `CIDToGIDMap` can stay the identity and no remapping table is needed.

use crate::be::{add, i16_at, mul, slice, u16_at};
use crate::ttf::Font;
use moonshot_folio_core::{FolioError, Limits, Result};

/// Components allowed in one composite glyph.
const MAX_COMPONENTS: usize = 256;

/// The glyph ids a composite glyph refers to.
fn components(g: &[u8]) -> Result<Vec<u16>> {
    let mut out = Vec::new();
    if g.len() < 10 || i16_at(g, 0)? >= 0 {
        return Ok(out);
    }
    let mut pos = 10_usize;
    loop {
        let flags = u16_at(g, pos)?;
        out.push(u16_at(g, add(pos, 2)?)?);
        if out.len() > MAX_COMPONENTS {
            return Err(FolioError::Limit("composite glyph components"));
        }
        let args = if flags & 0x0001 != 0 { 4 } else { 2 };
        let scale = if flags & 0x0008 != 0 {
            2
        } else if flags & 0x0040 != 0 {
            4
        } else if flags & 0x0080 != 0 {
            8
        } else {
            0
        };
        pos = add(pos, add(4, add(args, scale)?)?)?;
        if flags & 0x0020 == 0 {
            if pos > g.len() {
                return Err(FolioError::Malformed("composite glyph is truncated"));
            }
            return Ok(out);
        }
    }
}

/// `gids` plus glyph 0 and every composite component, by closure, in ascending order.
fn closure(font: &Font<'_>, gids: &[u16], limits: &Limits) -> Result<Vec<u16>> {
    let n = usize::from(font.num_glyphs());
    let mut keep = vec![false; n];
    let mut work: Vec<(u16, usize)> = vec![(0, 0)];
    work.extend(gids.iter().map(|g| (*g, 0)));
    while let Some((g, depth)) = work.pop() {
        if g >= font.num_glyphs() {
            return Err(FolioError::Invalid("glyph index out of range"));
        }
        let slot = keep
            .get_mut(usize::from(g))
            .ok_or(FolioError::Invalid("glyph index out of range"))?;
        if *slot {
            continue;
        }
        *slot = true;
        let comps = components(font.glyph_data(g)?)?;
        if !comps.is_empty() && depth >= limits.max_depth() {
            return Err(FolioError::Limit("composite glyph nesting"));
        }
        for c in comps {
            // Only unvisited glyphs go on the stack, so it stays at most one entry per glyph edge
            // actually followed.
            if keep.get(usize::from(c)).is_some_and(|k| !*k) {
                work.push((c, add(depth, 1)?));
            } else if usize::from(c) >= keep.len() {
                return Err(FolioError::Invalid("composite refers to a missing glyph"));
            }
        }
    }
    Ok(keep
        .iter()
        .enumerate()
        .filter(|(_, k)| **k)
        .filter_map(|(i, _)| u16::try_from(i).ok())
        .collect())
}

fn checksum(d: &[u8]) -> u32 {
    let mut sum = 0_u32;
    for c in d.chunks(4) {
        let mut w = [0_u8; 4];
        for (i, b) in c.iter().enumerate() {
            if let Some(slot) = w.get_mut(i) {
                *slot = *b;
            }
        }
        sum = sum.wrapping_add(u32::from_be_bytes(w));
    }
    sum
}

fn pad4(v: &mut Vec<u8>) {
    while !v.len().is_multiple_of(4) {
        v.push(0);
    }
}

/// Build `glyf` and long `loca` holding only the kept glyphs.
fn build_glyf(font: &Font<'_>, keep: &[u16], limits: &Limits) -> Result<(Vec<u8>, Vec<u8>)> {
    let mut glyf = Vec::new();
    let mut loca = Vec::new();
    let mut next = keep.iter().copied().peekable();
    for g in 0..font.num_glyphs() {
        loca.extend_from_slice(
            &u32::try_from(glyf.len())
                .map_err(|_| FolioError::Limit("glyf size"))?
                .to_be_bytes(),
        );
        if next.peek() == Some(&g) {
            next.next();
            let data = font.glyph_data(g)?;
            if add(glyf.len(), data.len())? > limits.max_file_bytes() {
                return Err(FolioError::Limit("subset size"));
            }
            glyf.extend_from_slice(data);
            pad4(&mut glyf);
        }
    }
    loca.extend_from_slice(
        &u32::try_from(glyf.len())
            .map_err(|_| FolioError::Limit("glyf size"))?
            .to_be_bytes(),
    );
    Ok((glyf, loca))
}

/// Subset `font` to `gids` (glyph 0 and composite components are added).
///
/// # Errors
/// `Invalid` for a glyph index out of range, `Limit` for oversize composites or output, and
/// `Malformed` for broken glyph data.
pub fn subset(font: &Font<'_>, gids: &[u16], limits: &Limits) -> Result<Vec<u8>> {
    let keep = closure(font, gids, limits)?;
    let (glyf, loca) = build_glyf(font, &keep, limits)?;
    let mut head = font
        .table_bytes(*b"head")
        .ok_or(FolioError::Malformed("font lacks head"))?
        .to_vec();
    slice(&head, 0, 54)?;
    head.get_mut(50..52)
        .ok_or(FolioError::Malformed("short head"))?
        .copy_from_slice(&1_u16.to_be_bytes());
    head.get_mut(8..12)
        .ok_or(FolioError::Malformed("short head"))?
        .copy_from_slice(&[0; 4]);
    let mut tables: Vec<([u8; 4], Vec<u8>)> = Vec::new();
    for tag in [*b"cvt ", *b"fpgm"] {
        if let Some(t) = font.table_bytes(tag) {
            tables.push((tag, t.to_vec()));
        }
    }
    tables.push((*b"glyf", glyf));
    tables.push((*b"head", head));
    for tag in [*b"hhea", *b"hmtx"] {
        tables.push((
            tag,
            font.table_bytes(tag)
                .ok_or(FolioError::Malformed("font lacks a table"))?
                .to_vec(),
        ));
    }
    tables.push((*b"loca", loca));
    tables.push((
        *b"maxp",
        font.table_bytes(*b"maxp")
            .ok_or(FolioError::Malformed("font lacks maxp"))?
            .to_vec(),
    ));
    if let Some(t) = font.table_bytes(*b"prep") {
        tables.push((*b"prep", t.to_vec()));
    }
    assemble(&mut tables, limits)
}

/// Write the sfnt: directory, then tables in the given (tag-sorted) order, 4-byte padded, with
/// per-table checksums and the `head` checksum adjustment.
fn assemble(tables: &mut [([u8; 4], Vec<u8>)], limits: &Limits) -> Result<Vec<u8>> {
    tables.sort_by_key(|(t, _)| *t);
    let n = tables.len();
    let n16 = u16::try_from(n).map_err(|_| FolioError::Limit("table count"))?;
    let mut entry_selector = 0_u16;
    while 1_usize
        .checked_shl(u32::from(entry_selector).saturating_add(1))
        .is_some_and(|v| v <= n)
    {
        entry_selector = entry_selector.saturating_add(1);
    }
    let search_range = (1_u16 << entry_selector).saturating_mul(16);
    let mut out = Vec::new();
    out.extend_from_slice(&0x0001_0000_u32.to_be_bytes());
    out.extend_from_slice(&n16.to_be_bytes());
    out.extend_from_slice(&search_range.to_be_bytes());
    out.extend_from_slice(&entry_selector.to_be_bytes());
    out.extend_from_slice(
        &n16.saturating_mul(16)
            .saturating_sub(search_range)
            .to_be_bytes(),
    );
    let mut offset = add(12, mul(n, 16)?)?;
    let mut body = Vec::new();
    let mut head_pos = None;
    for (tag, data) in tables.iter() {
        out.extend_from_slice(tag);
        out.extend_from_slice(&checksum(data).to_be_bytes());
        out.extend_from_slice(
            &u32::try_from(offset)
                .map_err(|_| FolioError::Limit("font size"))?
                .to_be_bytes(),
        );
        out.extend_from_slice(
            &u32::try_from(data.len())
                .map_err(|_| FolioError::Limit("font size"))?
                .to_be_bytes(),
        );
        if tag == b"head" {
            head_pos = Some(body.len());
        }
        body.extend_from_slice(data);
        pad4(&mut body);
        offset = add(12, add(mul(n, 16)?, body.len())?)?;
        if offset > limits.max_file_bytes() {
            return Err(FolioError::Limit("subset size"));
        }
    }
    out.extend_from_slice(&body);
    if let Some(p) = head_pos {
        let adj = 0xB1B0_AFBA_u32.wrapping_sub(checksum(&out));
        let at = add(add(12, mul(n, 16)?)?, add(p, 8)?)?;
        out.get_mut(at..add(at, 4)?)
            .ok_or(FolioError::Malformed("head position"))?
            .copy_from_slice(&adj.to_be_bytes());
    }
    Ok(out)
}
