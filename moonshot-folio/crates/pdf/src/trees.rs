// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Name trees and number trees. A small tree is one node with a flat array; a large one is a
//! root with `/Kids` and leaves with `/Limits`, so no array holds more than 8191 elements (the
//! PDF implementation limit).

use crate::objects::Objects;
use core::fmt::Write;
use moonshot_folio_core::{FolioError, Result};

/// Most elements the PDF implementation limits allow in one array.
pub(crate) const MAX_ARRAY: usize = 8191;

/// Entries per leaf: two array elements each, so a leaf stays well under [`MAX_ARRAY`].
const LEAF: usize = 4000;

/// One tree entry: how the key reads in `/Limits` (a number, or a literal string) and the bytes of
/// the entry in the array (`key value `, with a trailing space).
pub(crate) type Entry = (Vec<u8>, Vec<u8>);

fn node(key: &str, entries: &[Entry], limits: bool) -> Vec<u8> {
    let mut out = b"<< ".to_vec();
    if limits {
        if let (Some(first), Some(last)) = (entries.first(), entries.last()) {
            out.extend_from_slice(b"/Limits [");
            out.extend_from_slice(&first.0);
            out.push(b' ');
            out.extend_from_slice(&last.0);
            out.extend_from_slice(b"] ");
        }
    }
    out.extend_from_slice(format!("/{key} [").as_bytes());
    for e in entries {
        out.extend_from_slice(&e.1);
    }
    out.extend_from_slice(b"] >>");
    out
}

/// Write a tree whose array key is `key` (`Names` or `Nums`) and return the root object number.
/// `entries` must be sorted by key.
///
/// # Errors
/// `Limit` when even a two-level tree could not hold the entries.
pub(crate) fn write_tree(o: &mut Objects, key: &str, entries: &[Entry]) -> Result<usize> {
    if entries.len() <= LEAF {
        return o.add(node(key, entries, false));
    }
    let mut leaves = String::new();
    let mut count = 0_usize;
    for chunk in entries.chunks(LEAF) {
        let id = o.add(node(key, chunk, true))?;
        write!(leaves, "{id} 0 R ").ok();
        count = count.saturating_add(1);
    }
    if count > MAX_ARRAY {
        return Err(FolioError::Limit("tree size"));
    }
    o.add(format!("<< /Kids [{leaves}] >>").into_bytes())
}
