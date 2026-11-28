// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Embedded TrueType fonts: `Type0` / `Identity-H` / `CIDFontType2` with a `ToUnicode` `CMap`.

use crate::objects::{stream, Objects};
use crate::syntax::{fixed3, name};
use core::fmt::Write;
use moonshot_folio_codec::sha256;
use moonshot_folio_core::{Limits, Result};
use moonshot_folio_font::{subset, Font};
use std::collections::BTreeMap;

/// Scale a design-unit value to 1/1000 em, rounded to nearest.
fn scale(v: i64, upem: u16) -> i64 {
    let u = i64::from(upem.max(1));
    let n = v.saturating_mul(1000);
    let half = u / 2;
    if n >= 0 {
        n.saturating_add(half).checked_div(u).unwrap_or(0)
    } else {
        n.saturating_sub(half).checked_div(u).unwrap_or(0)
    }
}

/// Six deterministic capital letters derived from the glyph set, as the subset tag.
fn subset_tag(gids: &[u16], data_len: usize) -> String {
    let mut seed = data_len.to_be_bytes().to_vec();
    for g in gids {
        seed.extend_from_slice(&g.to_be_bytes());
    }
    let h = sha256(&seed);
    h.iter()
        .take(6)
        .map(|b| char::from(b'A'.saturating_add(b % 26)))
        .collect()
}

/// `/W` entries: runs of consecutive glyph ids as `first [w1 w2 ...]`.
fn widths(font: &Font<'_>, gids: &[u16]) -> Result<String> {
    let mut out = String::from("[");
    let mut prev: Option<u16> = None;
    for &g in gids {
        let w = scale(i64::from(font.advance(g)?), font.units_per_em());
        if prev.is_some_and(|p| p.checked_add(1) == Some(g)) {
            write!(out, " {w}").ok();
        } else {
            if prev.is_some() {
                out.push(']');
            }
            write!(out, " {g} [{w}").ok();
        }
        prev = Some(g);
    }
    if prev.is_some() {
        out.push(']');
    }
    out.push_str(" ]");
    Ok(out)
}

/// `CapHeight` from `OS/2` (version 2 and later) or 70 % of the ascent.
fn cap_height(font: &Font<'_>) -> i64 {
    let upem = font.units_per_em();
    let from_os2 = font.table_bytes(*b"OS/2").and_then(|t| {
        let version = u16::from_be_bytes(<[u8; 2]>::try_from(t.get(0..2)?).ok()?);
        let v = i16::from_be_bytes(<[u8; 2]>::try_from(t.get(88..90)?).ok()?);
        (version >= 2 && v > 0).then_some(i64::from(v))
    });
    let design = from_os2.unwrap_or_else(|| i64::from(font.ascent()).saturating_mul(7) / 10);
    scale(design, upem)
}

/// `StemV` estimated from `usWeightClass`.
fn stem_v(font: &Font<'_>) -> i64 {
    let w = font
        .table_bytes(*b"OS/2")
        .and_then(|t| Some(u16::from_be_bytes(<[u8; 2]>::try_from(t.get(4..6)?).ok()?)))
        .unwrap_or(400);
    if w >= 700 {
        140
    } else {
        80
    }
}

fn descriptor(font: &Font<'_>, ps: &str, file_id: usize) -> String {
    let upem = font.units_per_em();
    let b = font.bbox().map(|v| scale(i64::from(v), upem));
    let angle = font.italic_angle_fixed().unwrap_or(0);
    let mut flags = 32_u32;
    if font.is_fixed_pitch() {
        flags |= 1;
    }
    if angle != 0 {
        flags |= 64;
    }
    let angle_milli = i64::from(angle).saturating_mul(1000) / 65_536;
    format!(
        "<< /Type /FontDescriptor /FontName {} /Flags {flags} /FontBBox [{} {} {} {}] \
/ItalicAngle {} /Ascent {} /Descent {} /CapHeight {} /StemV {} /FontFile2 {file_id} 0 R >>",
        name(ps),
        b[0],
        b[1],
        b[2],
        b[3],
        fixed3(angle_milli),
        scale(i64::from(font.ascent()), upem),
        scale(i64::from(font.descent()), upem),
        cap_height(font),
        stem_v(font)
    )
}

fn utf16_hex(c: char) -> String {
    let mut buf = [0_u16; 2];
    let mut s = String::new();
    for u in c.encode_utf16(&mut buf).iter() {
        write!(s, "{u:04X}").ok();
    }
    s
}

/// A `ToUnicode` `CMap` using `bfrange` for runs and `bfchar` for single glyphs.
pub(crate) fn to_unicode(map: &BTreeMap<u16, char>) -> String {
    let mut singles: Vec<String> = Vec::new();
    let mut ranges: Vec<String> = Vec::new();
    let items: Vec<(u16, char)> = map.iter().map(|(g, c)| (*g, *c)).collect();
    let mut i = 0;
    while i < items.len() {
        let mut j = i;
        while let (Some(&(g0, c0)), Some(&(g1, c1))) =
            (items.get(j), items.get(j.saturating_add(1)))
        {
            let ok = g0.checked_add(1) == Some(g1)
                && u32::from(c0).checked_add(1) == Some(u32::from(c1))
                && g1 & 0xFF != 0
                && u32::from(c1) & 0xFF != 0
                && u32::from(c1) <= 0xFFFF;
            if !ok {
                break;
            }
            j = j.saturating_add(1);
        }
        match (items.get(i), items.get(j)) {
            (Some(&(ga, ca)), Some(&(gb, _))) if j > i => {
                ranges.push(format!("<{ga:04X}> <{gb:04X}> <{}>", utf16_hex(ca)));
            }
            (Some(&(ga, ca)), _) => singles.push(format!("<{ga:04X}> <{}>", utf16_hex(ca))),
            _ => {}
        }
        i = j.saturating_add(1);
    }
    let mut out = String::from(
        "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n\
/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n\
/CMapName /Adobe-Identity-UCS def\n/CMapType 2 def\n\
1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n",
    );
    for (kind, list) in [("bfchar", &singles), ("bfrange", &ranges)] {
        for chunk in list.chunks(100) {
            writeln!(out, "{} begin{kind}", chunk.len()).ok();
            for l in chunk {
                out.push_str(l);
                out.push('\n');
            }
            writeln!(out, "end{kind}").ok();
        }
    }
    out.push_str("endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n");
    out
}

/// Write the five objects of an embedded font and return the `Type0` font object number.
pub(crate) fn write_font(
    objs: &mut Objects,
    data: &[u8],
    used: &BTreeMap<u16, char>,
    limits: &Limits,
) -> Result<usize> {
    let font = Font::parse(data, limits)?;
    let gids: Vec<u16> = used.keys().copied().collect();
    let sub = subset(&font, &gids, limits)?;
    let base = font
        .postscript_name()
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| String::from("EmbeddedFont"));
    let ps = format!("{}+{base}", subset_tag(&gids, data.len()));
    let file = objs.add(stream(&format!("/Length1 {}", sub.len()), &sub))?;
    let desc = objs.add(descriptor(&font, &ps, file).into_bytes())?;
    let all: Vec<u16> = std::iter::once(0).chain(gids.iter().copied()).collect();
    let mut all = all;
    all.dedup();
    let cid = objs.add(
        format!(
            "<< /Type /Font /Subtype /CIDFontType2 /BaseFont {} \
/CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> \
/FontDescriptor {desc} 0 R /CIDToGIDMap /Identity /DW 1000 /W {} >>",
            name(&ps),
            widths(&font, &all)?
        )
        .into_bytes(),
    )?;
    let tu = objs.add(stream("", to_unicode(used).as_bytes()))?;
    objs.add(
        format!(
            "<< /Type /Font /Subtype /Type0 /BaseFont {} /Encoding /Identity-H \
/DescendantFonts [{cid} 0 R] /ToUnicode {tu} 0 R >>",
            name(&ps)
        )
        .into_bytes(),
    )
}
