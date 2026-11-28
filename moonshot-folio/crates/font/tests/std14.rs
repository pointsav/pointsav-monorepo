// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Standard-14 metrics tests (task F4.4).

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use moonshot_folio_font::Std14;

// REQ: FOLIO-REQ-041
#[test]
fn known_widths_and_fallbacks() {
    let h = Std14::Helvetica;
    let b = Std14::HelveticaBold;
    assert_eq!(
        (h.width('A'), h.width(' '), h.width('W'), h.width('i')),
        (Some(667), Some(278), Some(944), Some(222))
    );
    assert_eq!(
        (b.width('A'), b.width('W'), b.width('i')),
        (Some(722), Some(944), Some(278))
    );
    assert_eq!((h.width('\''), h.width('`')), (Some(191), Some(333))); // quotesingle, grave
    assert_eq!(
        (h.width('\u{E9}'), h.width('\u{2014}'), h.width('\u{2019}')),
        (Some(556), Some(1000), Some(222))
    );
    assert_eq!(
        (b.width('\u{2019}'), b.width('\u{201C}')),
        (Some(278), Some(500))
    );
    assert_eq!(h.width('\u{4E2D}'), None);
    assert_eq!(h.width('\u{A0}'), Some(278)); // no-break space
    assert_eq!((h.advance('\u{4E2D}'), b.advance('\u{4E2D}')), (556, 611));
    assert_eq!(h.text_units("AW"), 667 + 944);
    assert_eq!(h.text_units(""), 0);
    assert_eq!(
        (h.base_font(), b.base_font()),
        ("Helvetica", "Helvetica-Bold")
    );
    // Digits are tabular in both weights.
    for c in '0'..='9' {
        assert_eq!(h.width(c), Some(556));
        assert_eq!(b.width(c), Some(556));
    }
}

/// Parse `C <code> ; WX <w> ; N <name>` lines.
fn afm(path: &str) -> Option<std::collections::HashMap<String, u16>> {
    let text = std::fs::read(path).ok()?;
    let text: String = text.iter().map(|b| char::from(*b)).collect();
    let mut m = std::collections::HashMap::new();
    for l in text.lines() {
        let parts: Vec<&str> = l.split(';').map(str::trim).collect();
        if parts.len() >= 3
            && parts[0].starts_with("C ")
            && parts[1].starts_with("WX ")
            && parts[2].starts_with("N ")
        {
            m.insert(parts[2][2..].to_string(), parts[1][3..].parse().unwrap());
        }
    }
    Some(m)
}

// REQ: FOLIO-REQ-041
#[test]
fn matches_the_adobe_afm_files_when_present() {
    // Oracle: the Adobe AFM files that ship with matplotlib. Skipped when absent.
    let dir = std::env::var("FOLIO_AFM_DIR").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{home}/.local/lib/python3.12/site-packages/matplotlib/mpl-data/fonts/afm")
    });
    // Glyph names for the characters we check: ASCII by AGL name, plus the extras.
    let names: [(char, &str); 12] = [
        ('A', "A"),
        ('z', "z"),
        ('0', "zero"),
        (' ', "space"),
        ('\'', "quotesingle"),
        ('`', "grave"),
        ('\u{E9}', "eacute"),
        ('\u{FC}', "udieresis"),
        ('\u{2014}', "emdash"),
        ('\u{2019}', "quoteright"),
        ('\u{201C}', "quotedblleft"),
        ('\u{2022}', "bullet"),
    ];
    for (font, file) in [
        (Std14::Helvetica, "phvr8a.afm"),
        (Std14::HelveticaBold, "phvb8a.afm"),
    ] {
        let Some(m) = afm(&format!("{dir}/{file}")) else {
            continue;
        };
        for (c, n) in names {
            assert_eq!(font.width(c), Some(m[n]), "{file} {n}");
        }
        // Every Latin-1 letter with an AFM entry of the same AGL name agrees; spot all of ASCII too.
        let ascii_names = [
            "space",
            "exclam",
            "quotedbl",
            "numbersign",
            "dollar",
            "percent",
            "ampersand",
            "quotesingle",
            "parenleft",
            "parenright",
            "asterisk",
            "plus",
            "comma",
            "hyphen",
            "period",
            "slash",
        ];
        for (i, n) in ascii_names.iter().enumerate() {
            let c = char::from_u32(0x20 + u32::try_from(i).unwrap()).unwrap();
            assert_eq!(font.width(c), Some(m[*n]), "{file} {n}");
        }
    }
}

// REQ: FOLIO-REQ-041
#[test]
fn winansi_gaps_found_by_review_are_covered() {
    let h = Std14::Helvetica;
    let b = Std14::HelveticaBold;
    for (c, hw, bw) in [
        ('\u{B2}', 333, 333),
        ('\u{B3}', 333, 333),
        ('\u{B9}', 333, 333),
        ('\u{AD}', 333, 333),
        ('\u{20AC}', 556, 556),
        ('\u{160}', 667, 667),
        ('\u{161}', 500, 556),
        ('\u{152}', 1000, 1000),
        ('\u{2122}', 1000, 1000),
        ('\u{2020}', 556, 556),
        ('\u{201A}', 222, 278),
        ('\u{201E}', 333, 500),
        ('\u{2039}', 333, 333),
        ('\u{178}', 667, 667),
        ('\u{192}', 556, 556),
    ] {
        assert_eq!((h.width(c), b.width(c)), (Some(hw), Some(bw)), "{c:?}");
    }
}
