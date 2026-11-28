// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Font handles and encoded text runs.

/// Handle of a font registered with `PdfDoc`. It names the resource `/F<n>` on every page that
/// selects it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FontId(pub(crate) usize);

impl FontId {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

/// What to do with a character the chosen font cannot show.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextPolicy {
    /// Fail with `Unsupported`, so the document never shows something the author did not write.
    Reject,
    /// Show `?` (standard fonts) or glyph 0 (embedded fonts) and count the substitution in
    /// [`TextRun::substituted`].
    Substitute,
}

/// Text already encoded for one font: `WinAnsi` bytes for a standard font, two-byte big-endian
/// glyph ids for an embedded one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextRun {
    pub(crate) font: FontId,
    pub(crate) bytes: Vec<u8>,
    pub(crate) hex: bool,
    pub(crate) substituted: usize,
}

impl TextRun {
    /// The font this run was encoded for.
    #[must_use]
    pub fn font(&self) -> FontId {
        self.font
    }
    /// The encoded bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// True for an embedded (two-byte code) font, written as a hex string.
    #[must_use]
    pub fn is_hex(&self) -> bool {
        self.hex
    }
    /// How many characters were substituted under [`TextPolicy::Substitute`].
    #[must_use]
    pub fn substituted(&self) -> usize {
        self.substituted
    }
}
