// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Embedding permission (`OS/2` `fsType`) and the missing-glyph policy (task F4.5).

use crate::ttf::Font;
use moonshot_folio_core::{FolioError, Result};

/// What to do with a character the font cannot draw.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MissingGlyph {
    /// Fail: the document would not show what the author wrote.
    Reject,
    /// Draw glyph 0 (`.notdef`) and count the substitution so the caller can report it.
    Notdef,
}

/// Check that the font licence allows embedding in a PDF, and subsetting when `subsetting`.
///
/// # Errors
/// `Unsupported` when `fsType` forbids embedding (restricted licence, bitmap-only) or forbids
/// subsetting while `subsetting` is true.
pub fn check_embedding(font: &Font<'_>, subsetting: bool) -> Result<()> {
    let Some(fs) = font.fs_type() else {
        return Ok(());
    };
    if fs & 0x0002 != 0 {
        return Err(FolioError::Unsupported(
            "font licence forbids embedding (fsType restricted)",
        ));
    }
    if fs & 0x0200 != 0 {
        return Err(FolioError::Unsupported(
            "font permits only bitmap embedding",
        ));
    }
    if subsetting && fs & 0x0100 != 0 {
        return Err(FolioError::Unsupported("font licence forbids subsetting"));
    }
    Ok(())
}

/// Glyphs for the characters of `text`, and how many were missing.
///
/// # Errors
/// `Unsupported` for the first missing character under [`MissingGlyph::Reject`].
pub fn map_text(font: &Font<'_>, text: &str, policy: MissingGlyph) -> Result<(Vec<u16>, usize)> {
    let mut out = Vec::new();
    let mut missing = 0_usize;
    for c in text.chars() {
        match font.glyph_for(c) {
            Some(g) => out.push(g),
            None => match policy {
                MissingGlyph::Reject => {
                    return Err(FolioError::Unsupported("character not in font"))
                }
                MissingGlyph::Notdef => {
                    out.push(0);
                    missing = missing.saturating_add(1);
                }
            },
        }
    }
    Ok((out, missing))
}
