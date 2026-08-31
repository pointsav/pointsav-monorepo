// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

// Moved out of src/afm.rs: this test's example string is a comma-grouped
// decimal, which self-service-promote.sh's crate-purity gate treats as
// currency-shaped and flags outside tests/. The value itself is a synthetic
// width-measurement fixture, not a real figure.

use tool_typeset::afm::text_width_pt;

#[test]
fn digit_strings_measure_reasonably() {
    // "1,910.56" is 8 chars; digits+comma+period are all narrower than
    // an em, so this should land well under 8 * size_pt.
    let w = text_width_pt("1,910.56", 10.0, false);
    assert!(w > 30.0 && w < 60.0, "unexpected width: {w}");
}
