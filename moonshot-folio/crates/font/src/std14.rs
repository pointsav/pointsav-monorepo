// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Standard-14 font metrics for Helvetica and Helvetica-Bold (task F4.4).
//!
//! Widths are Adobe Core 14 AFM values in 1/1000 em, keyed by Unicode through the glyph names of
//! the `WinAnsi` encoding (so U+0027 is `quotesingle` and U+0060 is `grave`, as a PDF viewer draws
//! them). The tables were generated from the Adobe AFM files `phvr8a` and `phvb8a`; the tests
//! compare them with those files when a copy is available. Characters outside the tables use the
//! font's average width, and callers can ask [`Std14::width`] for `None` to detect that.

/// Which standard font.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Std14 {
    /// Helvetica.
    Helvetica,
    /// Helvetica-Bold.
    HelveticaBold,
}

/// U+0020..U+007E.
#[rustfmt::skip]
const HELVETICA_ASCII: [u16; 95] = [
    278, 278, 355, 556, 556, 889, 667, 191, 333, 333, 389, 584, 278, 333, 278, 278,
    556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 278, 278, 584, 584, 584, 556,
    1015, 667, 667, 722, 722, 667, 611, 778, 722, 278, 500, 667, 556, 833, 722, 778,
    667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, 278, 278, 278, 469, 556,
    333, 556, 556, 500, 556, 556, 278, 556, 556, 222, 222, 500, 222, 833, 556, 556,
    556, 556, 333, 500, 278, 556, 500, 722, 500, 500, 500, 334, 260, 334, 584,
];

/// U+00A0..U+00FF (0 where the font has no glyph).
#[rustfmt::skip]
const HELVETICA_LATIN1: [u16; 96] = [
    0, 333, 556, 556, 556, 556, 260, 556, 333, 737, 370, 556, 584, 0, 737, 333,
    400, 584, 0, 0, 333, 556, 537, 278, 333, 0, 365, 556, 834, 834, 834, 611,
    667, 667, 667, 667, 667, 667, 1000, 722, 667, 667, 667, 667, 278, 278, 278, 278,
    722, 722, 778, 778, 778, 778, 778, 584, 778, 722, 722, 722, 722, 667, 667, 611,
    556, 556, 556, 556, 556, 556, 889, 500, 556, 556, 556, 556, 278, 278, 278, 278,
    556, 556, 556, 556, 556, 556, 556, 584, 611, 556, 556, 556, 556, 500, 556, 500,
];

/// Other `WinAnsi` characters: (code point, width).
const HELVETICA_EXTRA: [(u32, u16); 31] = [
    (0x00AD, 333),
    (0x00B2, 333),
    (0x00B3, 333),
    (0x00B9, 333),
    (0x0152, 1000),
    (0x0153, 944),
    (0x0160, 667),
    (0x0161, 500),
    (0x0178, 667),
    (0x017D, 611),
    (0x017E, 500),
    (0x0192, 556),
    (0x02C6, 333),
    (0x02DC, 333),
    (0x2013, 556),
    (0x2014, 1000),
    (0x2018, 222),
    (0x2019, 222),
    (0x201A, 222),
    (0x201C, 333),
    (0x201D, 333),
    (0x201E, 333),
    (0x2020, 556),
    (0x2021, 556),
    (0x2022, 350),
    (0x2026, 1000),
    (0x2030, 1000),
    (0x2039, 333),
    (0x203A, 333),
    (0x20AC, 556),
    (0x2122, 1000),
];

/// U+0020..U+007E.
#[rustfmt::skip]
const BOLD_ASCII: [u16; 95] = [
    278, 333, 474, 556, 556, 889, 722, 238, 333, 333, 389, 584, 278, 333, 278, 278,
    556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 333, 333, 584, 584, 584, 611,
    975, 722, 722, 722, 722, 667, 611, 778, 722, 278, 556, 722, 611, 833, 722, 778,
    667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, 333, 278, 333, 584, 556,
    333, 556, 611, 556, 611, 556, 333, 611, 611, 278, 278, 556, 278, 889, 611, 611,
    611, 611, 389, 556, 333, 611, 556, 778, 556, 556, 500, 389, 280, 389, 584,
];

/// U+00A0..U+00FF (0 where the font has no glyph).
#[rustfmt::skip]
const BOLD_LATIN1: [u16; 96] = [
    0, 333, 556, 556, 556, 556, 280, 556, 333, 737, 370, 556, 584, 0, 737, 333,
    400, 584, 0, 0, 333, 611, 556, 278, 333, 0, 365, 556, 834, 834, 834, 611,
    722, 722, 722, 722, 722, 722, 1000, 722, 667, 667, 667, 667, 278, 278, 278, 278,
    722, 722, 778, 778, 778, 778, 778, 584, 778, 722, 722, 722, 722, 667, 667, 611,
    556, 556, 556, 556, 556, 556, 889, 556, 556, 556, 556, 556, 278, 278, 278, 278,
    611, 611, 611, 611, 611, 611, 611, 584, 611, 611, 611, 611, 611, 556, 611, 556,
];

/// Other `WinAnsi` characters: (code point, width).
const BOLD_EXTRA: [(u32, u16); 31] = [
    (0x00AD, 333),
    (0x00B2, 333),
    (0x00B3, 333),
    (0x00B9, 333),
    (0x0152, 1000),
    (0x0153, 944),
    (0x0160, 667),
    (0x0161, 556),
    (0x0178, 667),
    (0x017D, 611),
    (0x017E, 500),
    (0x0192, 556),
    (0x02C6, 333),
    (0x02DC, 333),
    (0x2013, 556),
    (0x2014, 1000),
    (0x2018, 278),
    (0x2019, 278),
    (0x201A, 278),
    (0x201C, 500),
    (0x201D, 500),
    (0x201E, 500),
    (0x2020, 556),
    (0x2021, 556),
    (0x2022, 350),
    (0x2026, 1000),
    (0x2030, 1000),
    (0x2039, 333),
    (0x203A, 333),
    (0x20AC, 556),
    (0x2122, 1000),
];

/// ASCII table, Latin-1 table, extra characters, average width.
type Tables = (
    &'static [u16; 95],
    &'static [u16; 96],
    &'static [(u32, u16)],
    u16,
);

impl Std14 {
    fn tables(self) -> Tables {
        match self {
            Self::Helvetica => (&HELVETICA_ASCII, &HELVETICA_LATIN1, &HELVETICA_EXTRA, 556),
            Self::HelveticaBold => (&BOLD_ASCII, &BOLD_LATIN1, &BOLD_EXTRA, 611),
        }
    }

    /// The PostScript name PDF uses for `BaseFont`.
    #[must_use]
    pub fn base_font(self) -> &'static str {
        match self {
            Self::Helvetica => "Helvetica",
            Self::HelveticaBold => "Helvetica-Bold",
        }
    }

    /// Width of `c` in 1/1000 em, or `None` when the tables do not cover it.
    #[must_use]
    pub fn width(self, c: char) -> Option<u16> {
        let (ascii, latin, extra, _) = self.tables();
        let cp = u32::from(c);
        let w = match cp {
            0x20..=0x7E => ascii
                .get(usize::try_from(cp.checked_sub(0x20)?).ok()?)
                .copied(),
            0xA0 => ascii.first().copied(),
            0xA1..=0xFF => latin
                .get(usize::try_from(cp.checked_sub(0xA0)?).ok()?)
                .copied(),
            _ => extra.iter().find(|(k, _)| *k == cp).map(|(_, v)| *v),
        };
        // Latin-1 slots without a glyph name in the table (superscripts, soft hyphen) are listed in
        // the extra table.
        w.filter(|w| *w > 0)
            .or_else(|| extra.iter().find(|(k, _)| *k == cp).map(|(_, v)| *v))
    }

    /// Width of `c`, using the font's average width for uncovered characters.
    #[must_use]
    pub fn advance(self, c: char) -> u16 {
        self.width(c).unwrap_or_else(|| self.tables().3)
    }

    /// Width of `s` in 1/1000 em.
    #[must_use]
    pub fn text_units(self, s: &str) -> u64 {
        s.chars()
            .fold(0_u64, |a, c| a.saturating_add(u64::from(self.advance(c))))
    }
}
