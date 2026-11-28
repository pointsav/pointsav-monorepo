// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Character map (`cmap`) subtables, formats 4 and 12.

use crate::be::{add, mul, slice, to_usize, u16_at, u32_at};
use moonshot_folio_core::{FolioError, Result};

/// A validated cmap subtable: the table bytes and where the chosen subtable starts.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Cmap<'a> {
    table: &'a [u8],
    sub: usize,
    format: u16,
}

/// Preference order of `(platform, encoding)` pairs: full Unicode first.
const PREFERRED: [(u16, u16); 5] = [(3, 10), (0, 4), (0, 6), (3, 1), (0, 3)];

impl<'a> Cmap<'a> {
    /// Choose and validate the best Unicode subtable.
    pub(crate) fn parse(table: &'a [u8]) -> Result<Self> {
        let n = usize::from(u16_at(table, 2)?);
        let mut best: Option<(usize, usize, u16)> = None; // (rank, offset, format)
        for i in 0..n {
            let rec = add(4, mul(i, 8)?)?;
            let key = (u16_at(table, rec)?, u16_at(table, add(rec, 2)?)?);
            let Some(rank) = PREFERRED.iter().position(|p| *p == key) else {
                continue;
            };
            let off = to_usize(u32_at(table, add(rec, 4)?)?)?;
            let format = u16_at(table, off)?;
            if !matches!(format, 4 | 12) {
                continue;
            }
            if best.is_none_or(|(r, _, _)| rank < r) {
                best = Some((rank, off, format));
            }
        }
        let (_, sub, format) = best.ok_or(FolioError::Unsupported(
            "no Unicode cmap subtable (format 4 or 12)",
        ))?;
        let c = Self { table, sub, format };
        c.validate()?;
        Ok(c)
    }

    fn validate(&self) -> Result<()> {
        let t = self.table;
        if self.format == 12 {
            let groups = to_usize(u32_at(t, add(self.sub, 12)?)?)?;
            slice(t, add(self.sub, 16)?, mul(groups, 12)?)?;
        } else {
            let len = usize::from(u16_at(t, add(self.sub, 2)?)?);
            let seg = usize::from(u16_at(t, add(self.sub, 6)?)?) / 2;
            if seg == 0 {
                return Err(FolioError::Malformed("cmap format 4 has no segments"));
            }
            // header 14 + endCode + pad + startCode + idDelta + idRangeOffset
            let need = add(16, mul(seg, 8)?)?;
            if len < need {
                return Err(FolioError::Malformed("cmap format 4 length is too small"));
            }
            slice(t, self.sub, len)?;
            let mut prev = 0_u16;
            for k in 0..seg {
                let end = u16_at(t, add(add(self.sub, 14)?, mul(k, 2)?)?)?;
                if end < prev {
                    return Err(FolioError::Malformed("cmap format 4 endCode is not sorted"));
                }
                prev = end;
            }
        }
        Ok(())
    }

    /// Glyph index for `c`, or `None` when unmapped. The caller checks it against `numGlyphs`.
    pub(crate) fn lookup(&self, c: char) -> Option<u16> {
        let cp = u32::from(c);
        let g = if self.format == 12 {
            self.lookup12(cp)
        } else {
            self.lookup4(cp)
        };
        g.filter(|g| *g != 0)
    }

    fn lookup12(&self, cp: u32) -> Option<u16> {
        let t = self.table;
        let groups = to_usize(u32_at(t, self.sub.checked_add(12)?).ok()?).ok()?;
        let (mut lo, mut hi) = (0_usize, groups);
        while lo < hi {
            let mid = lo.checked_add(hi.checked_sub(lo)? / 2)?;
            let rec = self
                .sub
                .checked_add(16)?
                .checked_add(mid.checked_mul(12)?)?;
            let (start, end) = (u32_at(t, rec).ok()?, u32_at(t, rec.checked_add(4)?).ok()?);
            if cp < start {
                hi = mid;
            } else if cp > end {
                lo = mid.checked_add(1)?;
            } else {
                let first = u32_at(t, rec.checked_add(8)?).ok()?;
                let gid = first.checked_add(cp.checked_sub(start)?)?;
                return u16::try_from(gid).ok();
            }
        }
        None
    }

    fn lookup4(&self, cp: u32) -> Option<u16> {
        let t = self.table;
        let c = u16::try_from(cp).ok()?;
        let seg = usize::from(u16_at(t, self.sub.checked_add(6)?).ok()?) / 2;
        let ends = self.sub.checked_add(14)?;
        let starts = ends.checked_add(seg.checked_mul(2)?)?.checked_add(2)?;
        let deltas = starts.checked_add(seg.checked_mul(2)?)?;
        let ranges = deltas.checked_add(seg.checked_mul(2)?)?;
        // endCode is sorted (checked at parse): binary search for the first segment ending at or
        // after `c`.
        let (mut lo, mut hi) = (0_usize, seg);
        while lo < hi {
            let mid = lo.checked_add(hi.checked_sub(lo)? / 2)?;
            let end = u16_at(t, ends.checked_add(mid.checked_mul(2)?)?).ok()?;
            if end < c {
                lo = mid.checked_add(1)?;
            } else {
                hi = mid;
            }
        }
        if lo >= seg {
            return None;
        }
        let step = lo.checked_mul(2)?;
        let start = u16_at(t, starts.checked_add(step)?).ok()?;
        if c < start {
            return None;
        }
        let delta = u16_at(t, deltas.checked_add(step)?).ok()?;
        let ro_pos = ranges.checked_add(step)?;
        let ro = usize::from(u16_at(t, ro_pos).ok()?);
        if ro == 0 {
            return Some(c.wrapping_add(delta));
        }
        let idx = ro_pos
            .checked_add(ro)?
            .checked_add(usize::from(c.checked_sub(start)?).checked_mul(2)?)?;
        let g = u16_at(t, idx).ok()?;
        if g == 0 {
            None
        } else {
            Some(g.wrapping_add(delta))
        }
    }
}
