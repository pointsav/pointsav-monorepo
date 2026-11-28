// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! LSB-first bit reader for DEFLATE.

use crate::le::slice;
use moonshot_folio_core::{FolioError, Result};

/// Reads bits least-significant first; every read is bounds-checked.
pub(crate) struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    buf: u32,
    cnt: u32,
}

impl<'a> BitReader<'a> {
    /// A reader at the start of `data`.
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            buf: 0,
            cnt: 0,
        }
    }

    /// The next `n` bits (`n <= 16`).
    pub(crate) fn bits(&mut self, n: u32) -> Result<u32> {
        while self.cnt < n {
            let b = *self
                .data
                .get(self.pos)
                .ok_or(FolioError::Malformed("unexpected end of deflate stream"))?;
            self.buf |= u32::from(b).wrapping_shl(self.cnt);
            self.pos = self.pos.saturating_add(1);
            self.cnt = self.cnt.saturating_add(8);
        }
        let v = self.buf & 1_u32.wrapping_shl(n).wrapping_sub(1);
        self.buf = self.buf.wrapping_shr(n);
        self.cnt = self.cnt.saturating_sub(n);
        Ok(v)
    }

    /// One bit.
    pub(crate) fn bit(&mut self) -> Result<u32> {
        self.bits(1)
    }

    /// Drop buffered bits and return whole unread bytes to the stream.
    fn realign(&mut self) {
        let whole = usize::try_from(self.cnt / 8).unwrap_or(0);
        self.pos = self.pos.saturating_sub(whole);
        self.buf = 0;
        self.cnt = 0;
    }

    /// Skip to the next byte boundary and take `n` raw bytes.
    pub(crate) fn take_bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        self.realign();
        let out = slice(self.data, self.pos, n)?;
        self.pos = self.pos.saturating_add(n);
        Ok(out)
    }

    /// Bytes consumed so far, counting a partly read byte as consumed.
    pub(crate) fn consumed(&self) -> usize {
        self.pos
            .saturating_sub(usize::try_from(self.cnt / 8).unwrap_or(0))
    }
}
