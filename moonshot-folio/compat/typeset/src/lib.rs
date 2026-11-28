// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Adapter from `tool_typeset::Doc` to the folio IR (task F2.3).
//!
//! Lossy mappings are reported in [`Converted::notes`], never dropped silently. Lengths in
//! points are quantised to hundredths of a point, then to EMU with the family rounding rule.

#![forbid(unsafe_code)]

mod blocks;
mod units;

use moonshot_folio_core::Result;
use moonshot_folio_ir::{Body, Document, Meta, Section};
use tool_typeset::Doc;

/// The converted document and what the conversion could not carry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Converted {
    /// The IR document (not yet validated; call `moonshot_folio_ir::validate`).
    pub document: Document,
    /// Human-readable loss notes, in document order.
    pub notes: Vec<String>,
}

/// Convert a `tool-typeset` document. Metadata is supplied by the caller because `Doc` has none.
///
/// # Errors
/// `Invalid` for a non-finite or out-of-range length, `Malformed` for a table row wider than its
/// column list, `Limit` on length overflow.
pub fn convert(doc: &Doc, meta: Meta) -> Result<Converted> {
    let mut notes = Vec::new();
    let page = units::page_setup(&doc.page, &mut notes)?;
    let mut out = Vec::new();
    for b in &doc.blocks {
        blocks::convert_block(b, &mut out, &mut notes)?;
    }
    let mut section = Section::new(page, out);
    units::chrome(&doc.page, &mut section);
    let body = Body::Flow(vec![section]);
    Ok(Converted {
        document: Document { meta, body },
        notes,
    })
}
