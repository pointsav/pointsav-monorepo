// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! XML text escaping. Characters XML 1.0 cannot carry are an error, never silently dropped.

use moonshot_folio_core::{FolioError, Result};

/// Is `c` allowed in an XML 1.0 document?
fn legal(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r')
        || ('\u{20}'..='\u{D7FF}').contains(&c)
        || ('\u{E000}'..='\u{FFFD}').contains(&c)
        || ('\u{10000}'..='\u{10FFFF}').contains(&c)
}

/// Escape `s` for element text or a double-quoted attribute value, appending to `out`.
///
/// # Errors
/// `Invalid` when `s` contains a character XML 1.0 forbids.
pub(crate) fn escape_into(out: &mut String, s: &str) -> Result<()> {
    for c in s.chars() {
        if !legal(c) {
            return Err(FolioError::Invalid(
                "text contains a character XML cannot carry",
            ));
        }
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    Ok(())
}
