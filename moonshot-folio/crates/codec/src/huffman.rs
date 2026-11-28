// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Canonical Huffman decoding in the style of zlib's `puff.c`.

use crate::bits::BitReader;
use moonshot_folio_core::{FolioError, Result};

/// A canonical Huffman code: how many codes of each length, and the symbols in code order.
pub(crate) struct Huffman {
    count: [u16; 16],
    symbol: Vec<u16>,
    max_len: usize,
}

impl Huffman {
    /// Build from per-symbol code lengths. The flag is true when the code is complete.
    ///
    /// # Errors
    /// A length above 15 or an over-subscribed set of lengths.
    pub(crate) fn new(lengths: &[u8]) -> Result<(Self, bool)> {
        let mut count = [0_u16; 16];
        for &l in lengths {
            let c = count
                .get_mut(usize::from(l))
                .ok_or(FolioError::Malformed("code length above 15"))?;
            *c = c.saturating_add(1);
        }
        let mut left = 1_i32;
        for len in 1..16 {
            let used = i32::from(count.get(len).copied().unwrap_or(0));
            left = left.saturating_mul(2).saturating_sub(used);
            if left < 0 {
                return Err(FolioError::Malformed("over-subscribed huffman code"));
            }
        }
        let symbol = place_symbols(lengths, &count)?;
        let max_len = (1..16)
            .rev()
            .find(|&l| count.get(l).copied().unwrap_or(0) > 0)
            .unwrap_or(0);
        Ok((
            Self {
                count,
                symbol,
                max_len,
            },
            left == 0,
        ))
    }

    /// The longest code length in use (0 when no symbol has a code).
    pub(crate) fn max_len(&self) -> usize {
        self.max_len
    }

    /// Decode one symbol.
    pub(crate) fn decode(&self, br: &mut BitReader<'_>) -> Result<u16> {
        let (mut code, mut first, mut index) = (0_u32, 0_u32, 0_u32);
        for len in 1..16 {
            code |= br.bit()?;
            let c = u32::from(self.count.get(len).copied().unwrap_or(0));
            if code < first.saturating_add(c) {
                let at = index.saturating_add(code.saturating_sub(first));
                let at = usize::try_from(at).unwrap_or(usize::MAX);
                return self
                    .symbol
                    .get(at)
                    .copied()
                    .ok_or(FolioError::Malformed("invalid huffman code"));
            }
            index = index.saturating_add(c);
            first = first.saturating_add(c).wrapping_shl(1);
            code = code.wrapping_shl(1);
        }
        Err(FolioError::Malformed("invalid huffman code"))
    }
}

/// Order symbols by (length, value) as canonical codes require.
fn place_symbols(lengths: &[u8], count: &[u16; 16]) -> Result<Vec<u16>> {
    let mut offs = [0_usize; 16];
    for len in 1..15 {
        let here = offs.get(len).copied().unwrap_or(0);
        let c = usize::from(count.get(len).copied().unwrap_or(0));
        if let Some(next) = offs.get_mut(len.saturating_add(1)) {
            *next = here.saturating_add(c);
        }
    }
    let total = lengths.iter().filter(|&&l| l != 0).count();
    let mut symbol = vec![0_u16; total];
    for (sym, &l) in lengths.iter().enumerate() {
        if l == 0 {
            continue;
        }
        let slot = offs
            .get_mut(usize::from(l))
            .ok_or(FolioError::Malformed("code length above 15"))?;
        let cell = symbol
            .get_mut(*slot)
            .ok_or(FolioError::Malformed("huffman table overflow"))?;
        *cell = u16::try_from(sym).map_err(|_| FolioError::Malformed("symbol out of range"))?;
        *slot = slot.saturating_add(1);
    }
    Ok(symbol)
}
