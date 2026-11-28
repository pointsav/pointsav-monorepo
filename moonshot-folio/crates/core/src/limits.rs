// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Resource limits for untrusted input and for output size.

/// Caps applied by parsers and writers. Fields are private; use the getters and `with_*` setters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    file_bytes: usize,
    entry_bytes: usize,
    total_bytes: usize,
    zip_entries: usize,
    name_bytes: usize,
    depth: usize,
    glyphs: usize,
    cells: usize,
    image_pixels: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            file_bytes: 268_435_456,
            entry_bytes: 268_435_456,
            total_bytes: 1_073_741_824,
            zip_entries: 10_000,
            name_bytes: 512,
            depth: 64,
            glyphs: 65_535,
            cells: 1_000_000,
            image_pixels: 64_000_000,
        }
    }
}

impl Limits {
    /// Largest accepted input file.
    #[must_use]
    pub fn max_file_bytes(&self) -> usize {
        self.file_bytes
    }
    /// Largest accepted single archive entry, after decompression.
    #[must_use]
    pub fn max_entry_bytes(&self) -> usize {
        self.entry_bytes
    }
    /// Largest accepted sum of all entries in one archive.
    #[must_use]
    pub fn max_total_bytes(&self) -> usize {
        self.total_bytes
    }
    /// Most entries in one archive.
    #[must_use]
    pub fn max_zip_entries(&self) -> usize {
        self.zip_entries
    }
    /// Longest entry name in bytes.
    #[must_use]
    pub fn max_name_bytes(&self) -> usize {
        self.name_bytes
    }
    /// Deepest nesting a tree walk may reach.
    #[must_use]
    pub fn max_depth(&self) -> usize {
        self.depth
    }
    /// Most glyphs in one font.
    #[must_use]
    pub fn max_glyphs(&self) -> usize {
        self.glyphs
    }
    /// Most cells in one workbook.
    #[must_use]
    pub fn max_cells(&self) -> usize {
        self.cells
    }

    /// Most pixels (width times height) in one decoded image.
    #[must_use]
    pub fn max_image_pixels(&self) -> usize {
        self.image_pixels
    }

    /// Set [`Self::max_file_bytes`].
    #[must_use]
    pub fn with_max_file_bytes(mut self, v: usize) -> Self {
        self.file_bytes = v;
        self
    }
    /// Set [`Self::max_entry_bytes`].
    #[must_use]
    pub fn with_max_entry_bytes(mut self, v: usize) -> Self {
        self.entry_bytes = v;
        self
    }
    /// Set [`Self::max_total_bytes`].
    #[must_use]
    pub fn with_max_total_bytes(mut self, v: usize) -> Self {
        self.total_bytes = v;
        self
    }
    /// Set [`Self::max_zip_entries`].
    #[must_use]
    pub fn with_max_zip_entries(mut self, v: usize) -> Self {
        self.zip_entries = v;
        self
    }
    /// Set [`Self::max_depth`].
    #[must_use]
    pub fn with_max_depth(mut self, v: usize) -> Self {
        self.depth = v;
        self
    }
    /// Set [`Self::max_glyphs`].
    #[must_use]
    pub fn with_max_glyphs(mut self, v: usize) -> Self {
        self.glyphs = v;
        self
    }
    /// Set [`Self::max_cells`].
    #[must_use]
    pub fn with_max_cells(mut self, v: usize) -> Self {
        self.cells = v;
        self
    }
    /// Set [`Self::max_image_pixels`].
    #[must_use]
    pub fn with_max_image_pixels(mut self, v: usize) -> Self {
        self.image_pixels = v;
        self
    }
    /// Set [`Self::max_name_bytes`].
    #[must_use]
    pub fn with_max_name_bytes(mut self, v: usize) -> Self {
        self.name_bytes = v;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Limits;

    // REQ: FOLIO-REQ-006
    #[test]
    fn defaults_match_the_architecture_brief() {
        let l = Limits::default();
        assert_eq!(l.max_file_bytes(), 256 * 1024 * 1024);
        assert_eq!(l.max_glyphs(), 65_535);
        assert_eq!(l.max_cells(), 1_000_000);
        assert_eq!(l.max_depth(), 64);
    }

    // REQ: FOLIO-REQ-006
    #[test]
    fn every_field_has_a_working_setter() {
        let l = Limits::default()
            .with_max_depth(1)
            .with_max_glyphs(2)
            .with_max_cells(3)
            .with_max_image_pixels(4);
        assert_eq!(
            (
                l.max_depth(),
                l.max_glyphs(),
                l.max_cells(),
                l.max_image_pixels()
            ),
            (1, 2, 3, 4)
        );
    }

    // REQ: FOLIO-REQ-006
    #[test]
    fn setters_change_only_their_field() {
        let l = Limits::default().with_max_zip_entries(3);
        assert_eq!(l.max_zip_entries(), 3);
        assert_eq!(l.max_name_bytes(), Limits::default().max_name_bytes());
    }
}
