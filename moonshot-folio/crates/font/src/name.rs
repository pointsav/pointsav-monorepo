// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `name` table lookup (ASCII-safe strings only).

use crate::be::{add, mul, slice, u16_at};

/// The string for `id`, from a Windows (UTF-16BE) or Macintosh (single-byte) record, kept only
/// when every character is printable ASCII without spaces or PostScript delimiters, at most 63 long (a safe PostScript
/// name that can be written into a PDF name object).
pub(crate) fn find(t: &[u8], id: u16) -> Option<String> {
    let count = usize::from(u16_at(t, 2).ok()?);
    let storage = usize::from(u16_at(t, 4).ok()?);
    for i in 0..count {
        let rec = add(6, mul(i, 12).ok()?).ok()?;
        if u16_at(t, add(rec, 6).ok()?).ok()? != id {
            continue;
        }
        let platform = u16_at(t, rec).ok()?;
        let len = usize::from(u16_at(t, add(rec, 8).ok()?).ok()?);
        let off = add(storage, usize::from(u16_at(t, add(rec, 10).ok()?).ok()?)).ok()?;
        let raw = slice(t, off, len).ok()?;
        let bytes: Vec<u8> = match platform {
            3 => raw
                .chunks_exact(2)
                .map(|p| {
                    if p.first() == Some(&0) {
                        p.get(1).copied().unwrap_or(0)
                    } else {
                        0
                    }
                })
                .collect(),
            1 => raw.to_vec(),
            _ => continue,
        };
        if !bytes.is_empty()
            && bytes.len() <= 63
            && bytes
                .iter()
                .all(|b| (0x21..=0x7E).contains(b) && !b"()<>[]{}/%#".contains(b))
        {
            return String::from_utf8(bytes).ok();
        }
    }
    None
}
