// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Subsetter tests (task F4.2).

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines
)]

mod common;
use common::{build, build_with, CmapKind};
use moonshot_folio_core::{FolioError, Limits};
use moonshot_folio_font::{subset, Font};

fn parse(d: &[u8]) -> Font<'_> {
    Font::parse(d, &Limits::default()).unwrap()
}

/// Sum of the file as big-endian words; a correctly adjusted sfnt sums to 0xB1B0AFBA.
fn file_sum(d: &[u8]) -> u32 {
    let mut s = 0_u32;
    for c in d.chunks(4) {
        let mut w = [0_u8; 4];
        w[..c.len()].copy_from_slice(c);
        s = s.wrapping_add(u32::from_be_bytes(w));
    }
    s
}

// REQ: FOLIO-REQ-039
#[test]
fn keeps_requested_glyphs_and_empties_the_rest() {
    let src = build(CmapKind::Format4, false);
    let f = parse(&src);
    let out = subset(&f, &[2], &Limits::default()).unwrap();
    let s = parse(&out);
    assert_eq!(
        (s.units_per_em(), s.num_glyphs(), s.ascent(), s.descent()),
        (1000, 4, 800, -200)
    );
    assert_eq!(s.glyph_data(2).unwrap(), f.glyph_data(2).unwrap());
    assert!(s.glyph_data(1).unwrap().is_empty());
    assert!(s.glyph_data(3).unwrap().is_empty());
    for g in 0..4 {
        assert_eq!(s.advance(g).unwrap(), f.advance(g).unwrap());
    }
    assert!(s.loca_is_long());
    assert_eq!(s.glyph_for('A'), None); // no cmap in a subset
    assert_eq!(file_sum(&out), 0xB1B0_AFBA);
    assert_eq!(out, subset(&f, &[2], &Limits::default()).unwrap());
}

// REQ: FOLIO-REQ-039
#[test]
fn composite_components_are_pulled_in() {
    let src = build_with(CmapKind::Format4, true, true);
    let f = parse(&src);
    let out = subset(&f, &[3], &Limits::default()).unwrap();
    let s = parse(&out);
    for g in [1_u16, 2, 3] {
        assert_eq!(
            s.glyph_data(g).unwrap(),
            f.glyph_data(g).unwrap(),
            "glyph {g}"
        );
    }
    assert_eq!(file_sum(&out), 0xB1B0_AFBA);
}

// REQ: FOLIO-REQ-039
#[test]
fn composite_cycles_and_deep_nesting_are_bounded() {
    // Glyph 3 refers to itself: the visited set stops the loop and the subset still builds.
    let mut d = build_with(CmapKind::Format4, true, true);
    let f0 = parse(&d);
    let g3 = f0.glyph_data(3).unwrap().as_ptr() as usize - d.as_ptr() as usize;
    d[g3 + 11 + 1..g3 + 11 + 3].copy_from_slice(&3_u16.to_be_bytes());
    let f = parse(&d);
    let out = subset(&f, &[3], &Limits::default()).unwrap();
    assert!(parse(&out).glyph_data(3).is_ok());
    // A depth limit of zero rejects any composite.
    let d = build_with(CmapKind::Format4, true, true);
    let f = parse(&d);
    assert!(matches!(
        subset(&f, &[3], &Limits::default().with_max_depth(0)),
        Err(FolioError::Limit(_))
    ));
}

// REQ: FOLIO-REQ-039
#[test]
fn bad_requests_and_bad_composites_are_errors() {
    let d = build(CmapKind::Format4, false);
    let f = parse(&d);
    assert!(matches!(
        subset(&f, &[99], &Limits::default()),
        Err(FolioError::Invalid(_))
    ));
    // A composite whose last component claims "more components" runs off the glyph data.
    let mut d = build_with(CmapKind::Format4, true, true);
    let f0 = parse(&d);
    let g3 = f0.glyph_data(3).unwrap().as_ptr() as usize - d.as_ptr() as usize;
    d[g3 + 19] |= 0x20;
    let f = parse(&d);
    assert!(matches!(
        subset(&f, &[3], &Limits::default()),
        Err(FolioError::Malformed(_))
    ));
}

// REQ: FOLIO-REQ-039
#[test]
fn subset_of_tiny_fonts_parses_for_every_layout() {
    for kind in [
        CmapKind::Format4,
        CmapKind::Format12,
        CmapKind::Format4RangeOffset,
    ] {
        for long in [false, true] {
            let src = build(kind, long);
            let f = parse(&src);
            let out = subset(&f, &[1, 2], &Limits::default()).unwrap();
            let s = parse(&out);
            assert_eq!(s.glyph_data(1).unwrap(), f.glyph_data(1).unwrap());
            assert_eq!(file_sum(&out), 0xB1B0_AFBA);
        }
    }
}

// REQ: FOLIO-REQ-039
#[test]
fn subset_of_a_real_font_loads_in_fonttools_oracle_dir() {
    // Writes subsets of the system fonts for the manual fontTools check; also self-parses them.
    let Some(dir) = std::env::var_os("FOLIO_SUBSET_OUT") else {
        return;
    };
    let dir = std::path::PathBuf::from(dir);
    for (name, path) in [
        (
            "liberation",
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        ),
        ("dejavu", "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"),
    ] {
        let Ok(data) = std::fs::read(path) else {
            continue;
        };
        let f = parse(&data);
        let gids: Vec<u16> = "Hello, World! \u{E9}\u{20AC}"
            .chars()
            .filter_map(|c| f.glyph_for(c))
            .collect();
        let out = subset(&f, &gids, &Limits::default()).unwrap();
        let s = parse(&out);
        for g in &gids {
            assert_eq!(s.glyph_data(*g).unwrap(), f.glyph_data(*g).unwrap());
        }
        std::fs::write(dir.join(format!("{name}-subset.ttf")), &out).unwrap();
        std::fs::write(
            dir.join(format!("{name}-gids.txt")),
            gids.iter()
                .map(u16::to_string)
                .collect::<Vec<_>>()
                .join(","),
        )
        .unwrap();
    }
}

// REQ: FOLIO-REQ-054
#[test]
fn depth_limit_applies_only_to_composites_and_output_is_bounded() {
    // A font with no composites subsets even with max_depth 0.
    let d = build(CmapKind::Format4, false);
    let f = parse(&d);
    assert!(subset(&f, &[1, 2], &Limits::default().with_max_depth(0)).is_ok());
    // A truncated composite is refused instead of being copied.
    let mut d = build_with(CmapKind::Format4, true, true);
    let f0 = parse(&d);
    let g3 = f0.glyph_data(3).unwrap().as_ptr() as usize - d.as_ptr() as usize;
    // Claim the second component has scale words it does not have (flag 0x0008).
    d[g3 + 19] |= 0x08;
    let f = parse(&d);
    assert!(matches!(
        subset(&f, &[3], &Limits::default()),
        Err(FolioError::Malformed(_))
    ));
    // Output is capped by max_file_bytes even for a valid font.
    let d = build(CmapKind::Format4, false);
    let f = parse(&d);
    assert!(matches!(
        subset(&f, &[1, 2], &Limits::default().with_max_file_bytes(60)),
        Err(FolioError::Limit(_) | FolioError::Malformed(_))
    ));
}
