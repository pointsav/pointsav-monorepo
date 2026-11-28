// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! moonshot-folio-font (L2): TrueType parsing (task F4.1). Subsetting and shaping follow in
//! F4.2 and F4.3.
//!
//! The parser treats the font as untrusted input: every offset and length is checked, table
//! sizes are bounded by the file, and the glyph count is bounded by [`Limits::max_glyphs`].

#![forbid(unsafe_code)]

mod be;
mod cmap;
mod name;
mod policy;
mod std14;
mod subset;
mod ttf;

pub use policy::{check_embedding, map_text, MissingGlyph};
pub use std14::Std14;
pub use subset::subset;
pub use ttf::Font;
