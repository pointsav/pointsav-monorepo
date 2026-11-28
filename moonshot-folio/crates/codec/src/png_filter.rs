// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! PNG scanline unfiltering (the five filter types of PNG section 9).

use moonshot_folio_core::{FolioError, Result};

/// Undo the per-row filters. `raw` holds `height` rows of one filter byte plus `stride` bytes;
/// `bpp` is the filter distance in bytes (at least 1).
pub(crate) fn unfilter(raw: &[u8], stride: usize, bpp: usize) -> Result<Vec<u8>> {
    let row_len = stride
        .checked_add(1)
        .ok_or(FolioError::Malformed("stride overflow"))?;
    let mut out: Vec<u8> = Vec::with_capacity(raw.len());
    let mut prev = vec![0_u8; stride];
    for row in raw.chunks_exact(row_len) {
        let (&filter, data) = row
            .split_first()
            .ok_or(FolioError::Malformed("empty scanline"))?;
        let mut cur: Vec<u8> = Vec::with_capacity(stride);
        for (i, &x) in data.iter().enumerate() {
            let back = i.checked_sub(bpp);
            let a = back.and_then(|j| cur.get(j)).copied().unwrap_or(0);
            let b = prev.get(i).copied().unwrap_or(0);
            let c = back.and_then(|j| prev.get(j)).copied().unwrap_or(0);
            cur.push(x.wrapping_add(predictor(filter, a, b, c)?));
        }
        out.extend_from_slice(&cur);
        prev = cur;
    }
    Ok(out)
}

/// The prediction for one byte: `a` left, `b` above, `c` above-left.
fn predictor(filter: u8, a: u8, b: u8, c: u8) -> Result<u8> {
    Ok(match filter {
        0 => 0,
        1 => a,
        2 => b,
        3 => u8::try_from((u16::from(a).saturating_add(u16::from(b))) / 2).unwrap_or(0),
        4 => paeth(a, b, c),
        _ => return Err(FolioError::Malformed("unknown png filter type")),
    })
}

/// The Paeth predictor.
fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let (ia, ib, ic) = (i32::from(a), i32::from(b), i32::from(c));
    let p = ia.saturating_add(ib).saturating_sub(ic);
    let pa = p.abs_diff(ia);
    let pb = p.abs_diff(ib);
    let pc = p.abs_diff(ic);
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}
