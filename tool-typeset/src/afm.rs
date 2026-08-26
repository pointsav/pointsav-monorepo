//! Standard-14 font metrics (Adobe Core 14, public domain AFM values, in
//! 1/1000 em units). Hardcoded because this crate embeds no fonts (v1 scope
//! boundary) -- every PDF-capable viewer or printer already has these built
//! in. Covers printable ASCII 32-126 plus the WinAnsi punctuation prose
//! commonly uses beyond that (em/en-dash, curly quotes, ellipsis -- see
//! `pdf_writer::winansi_byte`, which maps the same characters to their
//! WinAnsi byte values); anything else falls back to Helvetica's own
//! average glyph width rather than failing the render.

/// Helvetica (regular) widths, index = ascii_code - 32.
#[rustfmt::skip]
const HELVETICA_WIDTHS: [u16; 95] = [
    278, 278, 355, 556, 556, 889, 667, 191, 333, 333, 389, 584, 278, 333, 278, 278, // space..slash
    556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 278, 278, 584, 584, 584, 556, // 0..?
    1015, 667, 667, 722, 722, 667, 611, 778, 722, 278, 500, 667, 556, 833, 722, 778, // @..O
    667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, 278, 278, 278, 469, 556, // P..underscore
    333, 556, 556, 500, 556, 556, 278, 556, 556, 222, 222, 500, 222, 833, 556, 556, // `..o
    556, 556, 333, 500, 278, 556, 500, 722, 500, 500, 500, 334, 260, 334, 584, // p..~
];

/// Helvetica-Bold widths, same index scheme. A real second table -- digit
/// widths happen to match regular Helvetica (both 556), but most other
/// glyphs are meaningfully wider bold, and treating bold text as if it
/// measured like regular text is a real bug (a bold right-aligned header
/// like "Balance" measures short, missing the figure rail below it).
#[rustfmt::skip]
const HELVETICA_BOLD_WIDTHS: [u16; 95] = [
    278, 333, 474, 556, 556, 889, 722, 238, 333, 333, 389, 584, 278, 333, 278, 278, // space..slash
    556, 556, 556, 556, 556, 556, 556, 556, 556, 556, 333, 333, 584, 584, 584, 611, // 0..?
    975, 722, 722, 722, 722, 667, 611, 778, 722, 278, 556, 722, 611, 833, 722, 778, // @..O
    667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, 333, 278, 333, 584, 556, // P..underscore
    333, 556, 611, 556, 611, 556, 333, 611, 611, 278, 278, 556, 278, 889, 611, 611, // `..o
    611, 611, 389, 556, 333, 611, 556, 778, 556, 556, 500, 389, 280, 389, 584, // p..~
];

const HELVETICA_AVG_WIDTH: u16 = 556;
const HELVETICA_BOLD_AVG_WIDTH: u16 = 611;

/// WinAnsi punctuation beyond ASCII that real prose content commonly uses
/// (account names lean on em-dash constantly). Widths are identical between
/// Helvetica and Helvetica-Bold for these glyphs in the standard AFM
/// metrics, so one table covers both weights.
fn winansi_extra_width(c: char) -> Option<u16> {
    match c {
        '\u{2013}' => Some(556),              // en dash
        '\u{2014}' => Some(1000),             // em dash
        '\u{2018}' | '\u{2019}' => Some(222), // single quotes
        '\u{201C}' | '\u{201D}' => Some(333), // double quotes
        '\u{2022}' => Some(350),              // bullet
        '\u{2026}' => Some(1000),             // ellipsis
        _ => None,
    }
}

/// Width of one string in points, at the given point size, in Helvetica
/// (`bold = false`) or Helvetica-Bold (`bold = true`).
pub fn text_width_pt(s: &str, size_pt: f32, bold: bool) -> f32 {
    let (table, avg) = if bold {
        (&HELVETICA_BOLD_WIDTHS, HELVETICA_BOLD_AVG_WIDTH)
    } else {
        (&HELVETICA_WIDTHS, HELVETICA_AVG_WIDTH)
    };
    let units: u32 = s
        .chars()
        .map(|c| {
            let code = c as u32;
            if (32..127).contains(&code) {
                table[(code - 32) as usize] as u32
            } else if let Some(w) = winansi_extra_width(c) {
                w as u32
            } else {
                avg as u32
            }
        })
        .sum();
    (units as f32 / 1000.0) * size_pt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_strings_measure_reasonably() {
        // "1,910.56" is 8 chars; digits+comma+period are all narrower than
        // an em, so this should land well under 8 * size_pt.
        let w = text_width_pt("1,910.56", 10.0, false);
        assert!(w > 30.0 && w < 60.0, "unexpected width: {w}");
    }

    #[test]
    fn empty_string_is_zero_width() {
        assert_eq!(text_width_pt("", 10.0, false), 0.0);
    }

    #[test]
    fn bold_measures_wider_than_regular_for_non_digit_text() {
        let regular = text_width_pt("Balance", 10.0, false);
        let bold = text_width_pt("Balance", 10.0, true);
        assert!(
            bold > regular,
            "bold ({bold}) should exceed regular ({regular})"
        );
    }

    #[test]
    fn digit_widths_match_across_weight() {
        let regular = text_width_pt("1234567890", 10.0, false);
        let bold = text_width_pt("1234567890", 10.0, true);
        assert_eq!(
            regular, bold,
            "digit widths should be identical in Helvetica vs Helvetica-Bold"
        );
    }

    #[test]
    fn em_dash_is_a_full_em_wide_not_the_average_fallback() {
        let w = text_width_pt("\u{2014}", 10.0, false);
        assert_eq!(w, 10.0, "em dash should measure as exactly 1000/1000 em");
    }
}
