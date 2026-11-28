// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! A tiny TrueType builder for tests: a few glyphs, selectable cmap format and loca format.

#![allow(
    dead_code,
    clippy::too_many_lines,
    clippy::trivially_copy_pass_by_ref,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation
)]

pub(crate) fn be16(v: u16) -> [u8; 2] {
    v.to_be_bytes()
}
pub(crate) fn be32(v: u32) -> [u8; 4] {
    v.to_be_bytes()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum CmapKind {
    Format4,
    Format12,
    Format4RangeOffset,
}

/// Glyph payloads: 0 empty (notdef), 1 `A`, 2 `B`, 3 empty (space).
const GLYPHS: [&[u8]; 4] = [
    &[],
    &[0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0xAA, 0xBB],
    &[0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0xCC, 0xDD],
    &[],
];

fn head(long_loca: bool) -> Vec<u8> {
    let mut t = vec![0; 54];
    t[0..4].copy_from_slice(&be32(0x0001_0000));
    t[12..16].copy_from_slice(&be32(0x5F0F_3CF5));
    t[18..20].copy_from_slice(&be16(1000));
    t[36..38].copy_from_slice(&(-50_i16).to_be_bytes());
    t[38..40].copy_from_slice(&(-200_i16).to_be_bytes());
    t[40..42].copy_from_slice(&be16(1100));
    t[42..44].copy_from_slice(&be16(900));
    t[50..52].copy_from_slice(&be16(u16::from(long_loca)));
    t
}

fn cmap(kind: CmapKind) -> Vec<u8> {
    // Maps U+0020 -> 3, U+0041 -> 1, U+0042 -> 2.
    let mut sub = Vec::new();
    match kind {
        CmapKind::Format12 => {
            sub.extend(be16(12));
            sub.extend(be16(0));
            sub.extend(be32(16 + 12 * 2));
            sub.extend(be32(0));
            sub.extend(be32(2));
            for (s, e, g) in [(0x20_u32, 0x20_u32, 3_u32), (0x41, 0x42, 1)] {
                sub.extend(be32(s));
                sub.extend(be32(e));
                sub.extend(be32(g));
            }
        }
        CmapKind::Format4 | CmapKind::Format4RangeOffset => {
            // Segments: [0x20,0x20], [0x41,0x42], [0xFFFF,0xFFFF]
            let ro = kind == CmapKind::Format4RangeOffset;
            let (ends, starts) = ([0x20_u16, 0x42, 0xFFFF], [0x20_u16, 0x41, 0xFFFF]);
            let deltas = [
                3_u16.wrapping_sub(0x20),
                if ro { 0 } else { 1_u16.wrapping_sub(0x41) },
                1,
            ];
            let seg = 3_u16;
            let len = 16 + 8 * seg + if ro { 4 } else { 0 };
            sub.extend(be16(4));
            sub.extend(be16(len));
            sub.extend(be16(0));
            sub.extend(be16(seg * 2));
            sub.extend([0; 6]);
            for e in ends {
                sub.extend(be16(e));
            }
            sub.extend(be16(0));
            for s in starts {
                sub.extend(be16(s));
            }
            for d in deltas {
                sub.extend(be16(d));
            }
            // idRangeOffset: segment 1 (when used) points at the glyphIdArray after the 3 entries.
            let offs = [0, if ro { 4 } else { 0 }, 0];
            for o in offs {
                sub.extend(be16(o));
            }
            if ro {
                sub.extend(be16(1));
                sub.extend(be16(2));
            }
        }
    }
    let (plat, enc) = if kind == CmapKind::Format12 {
        (3, 10)
    } else {
        (3, 1)
    };
    let mut t = Vec::new();
    t.extend(be16(0));
    t.extend(be16(1));
    t.extend(be16(plat));
    t.extend(be16(enc));
    t.extend(be32(12));
    t.extend(sub);
    t
}

/// A complete small font.
pub(crate) fn build(kind: CmapKind, long_loca: bool) -> Vec<u8> {
    build_with(kind, long_loca, false)
}

/// Glyph 3 (space) becomes a composite of glyphs 1 and 2 when `composite` is set.
pub(crate) fn build_with(kind: CmapKind, long_loca: bool, composite: bool) -> Vec<u8> {
    let mut glyf = Vec::new();
    let mut offs = vec![0_u32];
    let comp: Vec<u8> = vec![
        0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x23, 0, 1, 0, 0, 0, 0, 0, 0x02, 0, 2, 0, 0,
    ];
    for (i, g) in GLYPHS.iter().enumerate() {
        let g: &[u8] = if composite && i == 3 { &comp } else { g };
        glyf.extend_from_slice(g);
        while glyf.len() % 4 != 0 {
            glyf.push(0);
        }
        offs.push(glyf.len() as u32);
    }
    let loca: Vec<u8> = if long_loca {
        offs.iter().flat_map(|o| be32(*o)).collect()
    } else {
        offs.iter().flat_map(|o| be16((o / 2) as u16)).collect()
    };
    let mut maxp = vec![0; 32];
    maxp[0..4].copy_from_slice(&be32(0x0001_0000));
    maxp[4..6].copy_from_slice(&be16(4));
    let mut hhea = vec![0; 36];
    hhea[4..6].copy_from_slice(&be16(800));
    hhea[6..8].copy_from_slice(&(-200_i16).to_be_bytes());
    hhea[8..10].copy_from_slice(&be16(90));
    hhea[34..36].copy_from_slice(&be16(3));
    // 3 full metrics (advances 500, 600, 700) + 1 extra lsb: glyph 3 reuses advance 700.
    let mut hmtx = Vec::new();
    for a in [500_u16, 600, 700] {
        hmtx.extend(be16(a));
        hmtx.extend(be16(0));
    }
    hmtx.extend(be16(0));
    let mut os2 = vec![0; 78];
    os2[8..10].copy_from_slice(&be16(4));
    let mut post = vec![0; 32];
    post[4..8].copy_from_slice(&(-0x000A_0000_i32).to_be_bytes());
    post[12..16].copy_from_slice(&be32(1));
    let mut name = Vec::new();
    let ps = "Tiny-Regular";
    name.extend(be16(0));
    name.extend(be16(1));
    name.extend(be16(18));
    name.extend(be16(3));
    name.extend(be16(1));
    name.extend(be16(0x409));
    name.extend(be16(6));
    name.extend(be16((ps.len() * 2) as u16));
    name.extend(be16(0));
    for c in ps.bytes() {
        name.extend(be16(u16::from(c)));
    }
    let tables: Vec<(&[u8; 4], Vec<u8>)> = vec![
        (b"OS/2", os2),
        (b"cmap", cmap(kind)),
        (b"glyf", glyf),
        (b"head", head(long_loca)),
        (b"hhea", hhea),
        (b"hmtx", hmtx),
        (b"loca", loca),
        (b"maxp", maxp),
        (b"name", name),
        (b"post", post),
    ];
    let mut out = Vec::new();
    out.extend(be32(0x0001_0000));
    out.extend(be16(tables.len() as u16));
    out.extend([0; 6]);
    let mut pos = 12 + 16 * tables.len();
    let mut body = Vec::new();
    for (tag, data) in &tables {
        out.extend_from_slice(*tag);
        out.extend(be32(0));
        out.extend(be32(pos as u32));
        out.extend(be32(data.len() as u32));
        body.extend_from_slice(data);
        while body.len() % 4 != 0 {
            body.push(0);
        }
        pos = 12 + 16 * tables.len() + body.len();
    }
    out.extend(body);
    out
}

/// Offset of the table directory entry for `tag`.
pub(crate) fn entry(font: &[u8], tag: &[u8; 4]) -> usize {
    (0..usize::from(u16::from_be_bytes([font[4], font[5]])))
        .map(|i| 12 + 16 * i)
        .find(|o| &font[*o..*o + 4] == tag)
        .unwrap()
}
