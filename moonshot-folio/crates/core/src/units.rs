// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Length unit (`Emu`) and colour (`Ink`). Every length in the family is an integer EMU
//! (914,400 per inch), which is exact for points, twips, half-points and millimetres.

use crate::{FolioError, Result};

/// EMU per inch.
pub const EMU_PER_INCH: i64 = 914_400;
/// EMU per typographic point (1/72 inch).
pub const EMU_PER_POINT: i64 = 12_700;
/// EMU per twip (1/20 point).
pub const EMU_PER_TWIP: i64 = 635;
/// EMU per millimetre.
pub const EMU_PER_MM: i64 = 36_000;

/// A length in English Metric Units. Arithmetic is checked; there is no float conversion.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Emu(i64);

impl Emu {
    /// Zero length.
    pub const ZERO: Self = Self(0);

    /// Wrap a raw EMU count.
    #[must_use]
    pub fn from_raw(v: i64) -> Self {
        Self(v)
    }
    /// The raw EMU count.
    #[must_use]
    pub fn raw(self) -> i64 {
        self.0
    }
    /// Whole points.
    ///
    /// # Errors
    /// `Limit` on overflow.
    pub fn from_points(v: i64) -> Result<Self> {
        Self::scaled(v, EMU_PER_POINT)
    }
    /// Whole inches.
    ///
    /// # Errors
    /// `Limit` on overflow.
    pub fn from_inches(v: i64) -> Result<Self> {
        Self::scaled(v, EMU_PER_INCH)
    }
    /// Whole twips.
    ///
    /// # Errors
    /// `Limit` on overflow.
    pub fn from_twips(v: i64) -> Result<Self> {
        Self::scaled(v, EMU_PER_TWIP)
    }
    /// Whole millimetres.
    ///
    /// # Errors
    /// `Limit` on overflow.
    pub fn from_mm(v: i64) -> Result<Self> {
        Self::scaled(v, EMU_PER_MM)
    }
    /// `num / den` of a unit worth `unit` EMU, rounded half away from zero. This is the single
    /// rounding rule of the family; intermediates are `i128`.
    ///
    /// # Errors
    /// `Invalid` when `den` is zero; `Limit` when the result does not fit.
    pub fn from_ratio(num: i64, unit: i64, den: i64) -> Result<Self> {
        if den == 0 {
            return Err(FolioError::Invalid("ratio denominator is zero"));
        }
        let n = i128::from(num).saturating_mul(i128::from(unit));
        let d = i128::from(den);
        let half = d.saturating_abs() / 2;
        let adj = if n < 0 { half.wrapping_neg() } else { half };
        let q = n.saturating_add(adj).checked_div(d).unwrap_or(0);
        i64::try_from(q)
            .map(Self)
            .map_err(|_| FolioError::Limit("length overflow"))
    }
    /// Checked sum.
    ///
    /// # Errors
    /// `Limit` on overflow.
    pub fn checked_add(self, o: Self) -> Result<Self> {
        self.0
            .checked_add(o.0)
            .map(Self)
            .ok_or(FolioError::Limit("length overflow"))
    }
    /// Checked difference.
    ///
    /// # Errors
    /// `Limit` on overflow.
    pub fn checked_sub(self, o: Self) -> Result<Self> {
        self.0
            .checked_sub(o.0)
            .map(Self)
            .ok_or(FolioError::Limit("length overflow"))
    }
    /// Checked multiplication by a count.
    ///
    /// # Errors
    /// `Limit` on overflow.
    pub fn checked_mul(self, k: i64) -> Result<Self> {
        Self::scaled(self.0, k)
    }

    fn scaled(v: i64, unit: i64) -> Result<Self> {
        v.checked_mul(unit)
            .map(Self)
            .ok_or(FolioError::Limit("length overflow"))
    }
}

/// A colour with its model kept explicit, so a CMYK document never silently becomes RGB.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ink {
    /// Gray level, 0 black to 255 white.
    Gray(u8),
    /// Red, green, blue.
    Rgb(u8, u8, u8),
    /// Cyan, magenta, yellow, key (black); 255 is full ink.
    Cmyk(u8, u8, u8, u8),
}
