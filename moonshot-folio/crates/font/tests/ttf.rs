// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! TrueType parser tests (task F4.1).

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines
)]

mod common;
use common::{be16, be32, build, entry, CmapKind};
use moonshot_folio_core::{FolioError, Limits};
use moonshot_folio_font::Font;

fn parse(d: &[u8]) -> moonshot_folio_core::Result<Font<'_>> {
    Font::parse(d, &Limits::default())
}

// REQ: FOLIO-REQ-038
#[test]
fn metrics_cmap_and_glyph_data_for_every_layout() {
    for kind in [
        CmapKind::Format4,
        CmapKind::Format12,
        CmapKind::Format4RangeOffset,
    ] {
        for long in [false, true] {
            let data = build(kind, long);
            let f = parse(&data).unwrap();
            assert_eq!((f.units_per_em(), f.num_glyphs()), (1000, 4));
            assert_eq!((f.ascent(), f.descent(), f.line_gap()), (800, -200, 90));
            assert_eq!(f.bbox(), [-50, -200, 1100, 900]);
            assert_eq!(
                (f.glyph_for(' '), f.glyph_for('A'), f.glyph_for('B')),
                (Some(3), Some(1), Some(2)),
                "{long}"
            );
            assert_eq!(
                (
                    f.glyph_for('C'),
                    f.glyph_for('\u{10000}'),
                    f.glyph_for('\0')
                ),
                (None, None, None)
            );
            assert_eq!(
                [
                    f.advance(0).unwrap(),
                    f.advance(1).unwrap(),
                    f.advance(2).unwrap(),
                    f.advance(3).unwrap()
                ],
                [500, 600, 700, 700]
            );
            assert!(f.advance(4).is_err());
            assert_eq!(f.glyph_data(0).unwrap().len(), 0);
            assert_eq!(f.glyph_data(1).unwrap()[10..12], [0xAA, 0xBB]);
            assert_eq!(f.glyph_data(2).unwrap()[10..12], [0xCC, 0xDD]);
            assert!(f.glyph_data(9).is_err());
            assert_eq!(f.fs_type(), Some(4));
            assert_eq!(f.italic_angle_fixed(), Some(-0x000A_0000));
            assert!(f.is_fixed_pitch());
            assert_eq!(f.postscript_name().as_deref(), Some("Tiny-Regular"));
        }
    }
}

// REQ: FOLIO-REQ-038
#[test]
fn structural_rejections() {
    let good = build(CmapKind::Format4, false);
    let mut d = good.clone();
    d[0..4].copy_from_slice(&be32(0x4F54_544F));
    assert!(matches!(parse(&d), Err(FolioError::Unsupported(_))));
    d[0..4].copy_from_slice(&be32(0x7474_6366));
    assert!(matches!(parse(&d), Err(FolioError::Unsupported(_))));
    d[0..4].copy_from_slice(&be32(0xDEAD_BEEF));
    assert!(matches!(parse(&d), Err(FolioError::Malformed(_))));
    assert!(parse(&[]).is_err());
    let head = u32::from_be_bytes(
        good[entry(&good, b"head") + 8..entry(&good, b"head") + 12]
            .try_into()
            .unwrap(),
    ) as usize;
    let patch = |off: usize, bytes: &[u8]| {
        let mut d = good.clone();
        d[off..off + bytes.len()].copy_from_slice(bytes);
        d
    };
    assert!(parse(&patch(head + 12, &be32(0))).is_err()); // magic
    assert!(parse(&patch(head + 18, &be16(0))).is_err()); // unitsPerEm
    assert!(parse(&patch(head + 18, &be16(20_000))).is_err());
    assert!(parse(&patch(head + 50, &be16(2))).is_err()); // loca format
    let maxp = u32::from_be_bytes(
        good[entry(&good, b"maxp") + 8..entry(&good, b"maxp") + 12]
            .try_into()
            .unwrap(),
    ) as usize;
    assert!(parse(&patch(maxp + 4, &be16(0))).is_err());
    assert!(parse(&patch(maxp + 4, &be16(5000))).is_err()); // loca/hmtx too short for 5000 glyphs
    assert!(matches!(
        Font::parse(&good, &Limits::default().with_max_glyphs(3)),
        Err(FolioError::Limit(_))
    ));
    assert!(matches!(
        Font::parse(&good, &Limits::default().with_max_file_bytes(10)),
        Err(FolioError::Limit(_))
    ));
    let hhea = u32::from_be_bytes(
        good[entry(&good, b"hhea") + 8..entry(&good, b"hhea") + 12]
            .try_into()
            .unwrap(),
    ) as usize;
    assert!(parse(&patch(hhea + 34, &be16(0))).is_err());
    assert!(parse(&patch(hhea + 34, &be16(9))).is_err());
}

// REQ: FOLIO-REQ-038
#[test]
fn directory_faults() {
    let good = build(CmapKind::Format4, false);
    let mut d = good.clone();
    d[4..6].copy_from_slice(&be16(0));
    assert!(parse(&d).is_err());
    d[4..6].copy_from_slice(&be16(300));
    assert!(parse(&d).is_err());
    // A table whose offset points beyond the file.
    let mut d = good.clone();
    let e = entry(&d, b"glyf");
    d[e + 8..e + 12].copy_from_slice(&be32(0xFFFF_FF00));
    assert!(parse(&d).is_err());
    // A missing required table: rename `glyf` so it cannot be found.
    let mut d = good.clone();
    let e = entry(&d, b"glyf");
    d[e..e + 4].copy_from_slice(b"zzzz");
    assert!(matches!(parse(&d), Err(FolioError::Malformed(_))));
}

// REQ: FOLIO-REQ-038
#[test]
fn loca_must_be_monotonic_and_inside_glyf() {
    let good = build(CmapKind::Format4, true);
    let loca = u32::from_be_bytes(
        good[entry(&good, b"loca") + 8..entry(&good, b"loca") + 12]
            .try_into()
            .unwrap(),
    ) as usize;
    let mut d = good.clone();
    d[loca + 8..loca + 12].copy_from_slice(&be32(0x00FF_0000)); // glyph 2 starts far outside
    assert!(matches!(parse(&d), Err(FolioError::Malformed(_))));
    let mut d = good.clone();
    d[loca + 4..loca + 8].copy_from_slice(&be32(8));
    d[loca + 8..loca + 12].copy_from_slice(&be32(0));
    assert!(matches!(parse(&d), Err(FolioError::Malformed(_))));
    // Aliasing (0, L, 0, L ...) is the memory-amplification shape: refused at parse time.
    let mut d = good.clone();
    d[loca + 4..loca + 8].copy_from_slice(&be32(24));
    d[loca + 8..loca + 12].copy_from_slice(&be32(0));
    assert!(parse(&d).is_err());
}

// REQ: FOLIO-REQ-038
#[test]
fn cmap_faults() {
    for kind in [CmapKind::Format4, CmapKind::Format12] {
        let good = build(kind, false);
        let e = entry(&good, b"cmap");
        let off = u32::from_be_bytes(good[e + 8..e + 12].try_into().unwrap()) as usize;
        let mut d = good.clone();
        d[off + 4..off + 6].copy_from_slice(&be16(7)); // platform that is not Unicode
        assert!(matches!(parse(&d), Err(FolioError::Unsupported(_))));
        let mut d = good.clone();
        d[off + 8..off + 12].copy_from_slice(&be32(0x00FF_FFFF)); // subtable offset
        assert!(parse(&d).is_err());
        let mut d = good.clone();
        let sub = off + 12;
        if kind == CmapKind::Format12 {
            d[sub + 12..sub + 16].copy_from_slice(&be32(0x00FF_FFFF)); // numGroups
        } else {
            d[sub + 6..sub + 8].copy_from_slice(&be16(0)); // zero segments
        }
        assert!(parse(&d).is_err());
    }
}

// REQ: FOLIO-REQ-038
#[test]
fn truncation_and_single_byte_corruption_never_panic() {
    for kind in [
        CmapKind::Format4,
        CmapKind::Format12,
        CmapKind::Format4RangeOffset,
    ] {
        let good = build(kind, true);
        for cut in 0..good.len() {
            let _ = parse(&good[..cut]).map(|f| (f.glyph_for('A'), f.advance(1), f.glyph_data(1)));
        }
        for i in 0..good.len() {
            for v in [0x00, 0x7F, 0xFF, good[i] ^ 0x55] {
                let mut d = good.clone();
                d[i] = v;
                if let Ok(f) = parse(&d) {
                    for c in ['\0', ' ', 'A', 'B', 'z', '\u{FFFF}', '\u{10FFFF}'] {
                        let _ = f.glyph_for(c);
                    }
                    for g in 0..6 {
                        let _ = (f.advance(g), f.glyph_data(g));
                    }
                    let _ = (
                        f.fs_type(),
                        f.italic_angle_fixed(),
                        f.is_fixed_pitch(),
                        f.postscript_name(),
                    );
                }
            }
        }
    }
}

// REQ: FOLIO-REQ-038
#[test]
fn matches_fonttools_on_system_fonts_when_present() {
    // Expected values were read with python fontTools 4.63 from the same files. Skipped when a
    // font is absent; the fonts themselves are never committed.
    struct Case {
        path: &'static str,
        glyphs: u16,
        asc: i16,
        desc: i16,
        gap: i16,
        ps: &'static str,
        // (char, glyph, advance)
        chars: &'static [(char, u16, u16)],
    }
    let cases = [
        Case {
            path: "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
            glyphs: 2620,
            asc: 1854,
            desc: -434,
            gap: 67,
            ps: "LiberationSans",
            chars: &[
                ('A', 36, 1366),
                ('i', 76, 455),
                ('W', 58, 1933),
                ('\u{20AC}', 2088, 1139),
                ('\u{E9}', 171, 1139),
                (' ', 3, 569),
            ],
        },
        Case {
            path: "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            glyphs: 6253,
            asc: 1901,
            desc: -483,
            gap: 0,
            ps: "DejaVuSans",
            chars: &[
                ('A', 36, 1401),
                ('W', 58, 2025),
                ('\u{20AC}', 2948, 1303),
                (' ', 3, 651),
            ],
        },
    ];
    for c in cases {
        let Ok(data) = std::fs::read(c.path) else {
            continue;
        };
        let f = parse(&data).unwrap();
        assert_eq!(
            (
                f.units_per_em(),
                f.num_glyphs(),
                f.ascent(),
                f.descent(),
                f.line_gap()
            ),
            (2048, c.glyphs, c.asc, c.desc, c.gap),
            "{}",
            c.path
        );
        assert_eq!(f.postscript_name().as_deref(), Some(c.ps));
        assert_eq!(f.fs_type(), Some(0));
        for (ch, g, adv) in c.chars {
            assert_eq!(f.glyph_for(*ch), Some(*g), "{ch:?}");
            assert_eq!(f.advance(*g).unwrap(), *adv, "{ch:?}");
        }
        assert_eq!(f.glyph_for('\u{4E2D}'), None);
        // Every glyph's data is reachable without error.
        for g in 0..f.num_glyphs() {
            f.glyph_data(g).unwrap();
        }
    }
}

// REQ: FOLIO-REQ-004
#[test]
fn write_fuzz_seeds_when_asked() {
    // FOLIO_FUZZ_SEED_DIR=assurance/fuzz-corpus/ttf cargo test -p moonshot-folio-font writes the seeds.
    let Some(dir) = std::env::var_os("FOLIO_FUZZ_SEED_DIR") else {
        return;
    };
    let dir = std::path::PathBuf::from(dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (name, kind, long) in [
        ("tiny-f4-short", CmapKind::Format4, false),
        ("tiny-f4-long", CmapKind::Format4, true),
        ("tiny-f4-rangeoffset", CmapKind::Format4RangeOffset, true),
        ("tiny-f12-long", CmapKind::Format12, true),
    ] {
        std::fs::write(dir.join(format!("{name}.bin")), build(kind, long)).unwrap();
    }
}

// REQ: FOLIO-REQ-054
#[test]
fn review_fixes_head_os2_name_and_cmap_order() {
    let good = build(CmapKind::Format4, false);
    let tbl = |d: &[u8], tag: &[u8; 4]| {
        let e = entry(d, tag);
        (
            u32::from_be_bytes(d[e + 8..e + 12].try_into().unwrap()) as usize,
            e,
        )
    };
    // head shorter than 54 bytes is refused (parse and subset must agree).
    let mut d = good.clone();
    let (_, e) = tbl(&d, b"head");
    d[e + 12..e + 16].copy_from_slice(&be32(52));
    assert!(parse(&d).is_err());
    // A truncated OS/2 must not read as "no restriction".
    let mut d = good.clone();
    let (_, e) = tbl(&d, b"OS/2");
    d[e + 12..e + 16].copy_from_slice(&be32(6));
    assert!(parse(&d).is_err());
    // Unsorted format-4 endCode is refused, so the binary search stays sound.
    let mut d = good.clone();
    let (cm, _) = tbl(&d, b"cmap");
    let sub = cm + 12;
    d[sub + 14..sub + 16].copy_from_slice(&be16(0x0100)); // first endCode above the second
    assert!(parse(&d).is_err());
    // PostScript names with PDF delimiters or over 63 characters are not returned.
    for evil in ["ABC/Evil", "A(b)", "A<<B", "A%B", "A#B"] {
        let mut d = good.clone();
        let (nm, _) = tbl(&d, b"name");
        for (k, c) in evil.bytes().enumerate() {
            d[nm + 18 + 2 * k + 1] = c;
        }
        let len = u16::try_from(evil.len() * 2).unwrap();
        d[nm + 6 + 8..nm + 6 + 10].copy_from_slice(&len.to_be_bytes());
        assert_eq!(parse(&d).unwrap().postscript_name(), None, "{evil}");
    }
}
