// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Phase 4.5 (`KNOWLEDGE-PLATFORM-PLAN.md`): WCAG 4.5:1 contrast verification.
//!
//! A regression gate, not a runtime feature — the pairs below are the
//! text/background combinations `tokens/design-tokens.json` +
//! `tokens/theme-overrides.css` actually ship (light + dark, both tenant
//! accent families). If either file's hex values change, update the pairs
//! here to match and re-run `cargo test` to confirm AA (4.5:1, normal text)
//! still holds — this is intentionally a duplicated, hand-verified snapshot
//! of the tokens, not a live read of the CSS, so a change to the shipped
//! palette can't silently drop below AA without a test failure forcing a
//! deliberate look here.

/// WCAG 2.x relative luminance of one sRGB channel (0-255).
fn srgb_channel_to_linear(c: u8) -> f64 {
    let c = f64::from(c) / 255.0;
    if c <= 0.03928 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// WCAG 2.x relative luminance of a `#rrggbb` color.
fn relative_luminance(hex: &str) -> f64 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).expect("valid hex");
    let g = u8::from_str_radix(&hex[2..4], 16).expect("valid hex");
    let b = u8::from_str_radix(&hex[4..6], 16).expect("valid hex");
    0.2126 * srgb_channel_to_linear(r)
        + 0.7152 * srgb_channel_to_linear(g)
        + 0.0722 * srgb_channel_to_linear(b)
}

/// WCAG 2.x contrast ratio between two `#rrggbb` colors (1.0 to 21.0).
pub fn contrast_ratio(a: &str, b: &str) -> f64 {
    let (l1, l2) = (relative_luminance(a), relative_luminance(b));
    let (lighter, darker) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    const AA_NORMAL_TEXT: f64 = 4.5;

    /// (label, foreground, background) — every text/surface pair actually
    /// shipped, across both themes and both tenant accent families.
    const PAIRS: &[(&str, &str, &str)] = &[
        ("light ink / surface", "#202122", "#ffffff"),
        ("light ink-secondary / surface", "#54595d", "#ffffff"),
        ("light ink-muted / surface-sunken", "#646a70", "#f8f9fa"),
        ("light link / surface", "#3366cc", "#ffffff"),
        (
            "light accent-ink / accent (documentation)",
            "#ffffff",
            "#1a4480",
        ),
        ("light accent-ink / accent (woodfine)", "#ffffff", "#164679"),
        ("dark ink / surface", "#e7e9ea", "#101418"),
        ("dark ink-secondary / surface", "#a6abb1", "#101418"),
        ("dark ink-muted / surface-sunken", "#868c93", "#171c22"),
        ("dark link / surface", "#7aa6f0", "#101418"),
        (
            "dark accent-ink / accent (default/documentation)",
            "#101418",
            "#6f9ee0",
        ),
        ("dark accent-ink / accent (woodfine)", "#101418", "#6ea0dc"),
    ];

    #[test]
    fn known_contrast_ratio_matches_reference_value() {
        // Black on white is the textbook 21:1 case — sanity-checks the math
        // itself before trusting it against the real token pairs below.
        let ratio = contrast_ratio("#000000", "#ffffff");
        assert!((ratio - 21.0).abs() < 0.01, "got {ratio}");
    }

    #[test]
    fn every_shipped_text_surface_pair_meets_wcag_aa_normal_text() {
        let mut failures = Vec::new();
        for (label, fg, bg) in PAIRS {
            let ratio = contrast_ratio(fg, bg);
            if ratio < AA_NORMAL_TEXT {
                failures.push(format!(
                    "{label}: {ratio:.2}:1 (needs >= {AA_NORMAL_TEXT}:1)"
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "WCAG AA failures:\n{}",
            failures.join("\n")
        );
    }
}
