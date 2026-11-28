// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Shared test helpers.

#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use moonshot_folio_codec::ZipArchive;
use moonshot_folio_core::Limits;

/// Minimal well-formedness check: tags balance and nest, attributes are quoted.
pub(crate) fn well_formed(xml: &str) -> bool {
    let mut stack: Vec<String> = Vec::new();
    let mut rest = xml;
    if let Some(i) = rest.find("?>") {
        rest = &rest[i + 2..];
    }
    while let Some(i) = rest.find('<') {
        let after = &rest[i + 1..];
        let Some(j) = after.find('>') else {
            return false;
        };
        let tag = &after[..j];
        if let Some(name) = tag.strip_prefix('/') {
            if stack.pop().as_deref() != Some(name.trim()) {
                return false;
            }
        } else if !tag.ends_with('/') {
            stack.push(tag.split_whitespace().next().unwrap().to_string());
        }
        if !tag.matches('"').count().is_multiple_of(2) {
            return false;
        }
        rest = &after[j + 1..];
    }
    stack.is_empty()
}

pub(crate) fn part(bytes: &[u8], name: &str) -> String {
    let z = ZipArchive::parse(bytes, &Limits::default()).unwrap();
    let e = z.find(name).unwrap();
    String::from_utf8(z.read(e).unwrap()).unwrap()
}
