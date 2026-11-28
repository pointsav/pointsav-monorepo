// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Font faces used for measuring: a standard-14 face or a parsed TrueType font.

use crate::tree::FontSlot;
use moonshot_folio_core::{Emu, FolioError, Result};
use moonshot_folio_font::{Font, MissingGlyph, Std14};

/// Helvetica and Helvetica-Bold ascender and descender in 1/1000 em (Adobe AFM).
const STD_ASCENT: i64 = 718;
const STD_DESCENT: i64 = 207;

/// One face.
#[derive(Clone, Debug)]
pub enum Face<'a> {
    /// A standard-14 face (metrics built in, nothing embedded).
    Std14(Std14),
    /// A parsed TrueType font (embedded by the backend).
    TrueType(Font<'a>),
}

/// The regular and bold faces of a document.
#[derive(Clone, Debug)]
pub struct Fonts<'a> {
    regular: Face<'a>,
    bold: Face<'a>,
}

/// Width of one character: advance and units per em, and whether it is a substitute.
pub(crate) struct Advance {
    pub(crate) units: i64,
    pub(crate) per_em: i64,
    pub(crate) substituted: bool,
}

impl<'a> Fonts<'a> {
    /// Helvetica and Helvetica-Bold.
    #[must_use]
    pub fn std14() -> Self {
        Self {
            regular: Face::Std14(Std14::Helvetica),
            bold: Face::Std14(Std14::HelveticaBold),
        }
    }

    /// Any two faces.
    #[must_use]
    pub fn new(regular: Face<'a>, bold: Face<'a>) -> Self {
        Self { regular, bold }
    }

    /// The face used for `slot`.
    #[must_use]
    pub fn face(&self, slot: FontSlot) -> &Face<'a> {
        match slot {
            FontSlot::Regular => &self.regular,
            FontSlot::Bold => &self.bold,
        }
    }

    /// Width of `c` in `slot` at `size`, rounded to whole EMU.
    pub(crate) fn width(
        &self,
        slot: FontSlot,
        c: char,
        size: Emu,
        policy: MissingGlyph,
    ) -> Result<(Emu, bool)> {
        let a = self.face(slot).advance(c, policy)?;
        Ok((
            Emu::from_ratio(a.units, size.raw(), a.per_em)?,
            a.substituted,
        ))
    }

    /// Ascent and descent of `slot` at `size`.
    pub(crate) fn extents(&self, slot: FontSlot, size: Emu) -> Result<(Emu, Emu)> {
        let (asc, desc, per_em) = self.face(slot).extents()?;
        Ok((
            Emu::from_ratio(asc, size.raw(), per_em)?,
            Emu::from_ratio(desc, size.raw(), per_em)?,
        ))
    }

    /// True when `slot` can draw `c` without substitution.
    pub(crate) fn has(&self, slot: FontSlot, c: char) -> bool {
        self.face(slot).has(c)
    }
}

impl Face<'_> {
    fn advance(&self, c: char, policy: MissingGlyph) -> Result<Advance> {
        match self {
            Self::Std14(f) => match (f.width(c), policy) {
                (Some(w), _) => Ok(Advance::exact(i64::from(w), 1000)),
                (None, MissingGlyph::Notdef) => {
                    Ok(Advance::substitute(i64::from(f.advance(c)), 1000))
                }
                (None, MissingGlyph::Reject) => Err(missing()),
            },
            Self::TrueType(f) => {
                let per_em = i64::from(f.units_per_em());
                match (f.glyph_for(c), policy) {
                    (Some(g), _) => Ok(Advance::exact(i64::from(f.advance(g)?), per_em)),
                    (None, MissingGlyph::Notdef) => {
                        Ok(Advance::substitute(i64::from(f.advance(0)?), per_em))
                    }
                    (None, MissingGlyph::Reject) => Err(missing()),
                }
            }
        }
    }

    /// Ascent, descent (positive) and units per em.
    fn extents(&self) -> Result<(i64, i64, i64)> {
        match self {
            Self::Std14(_) => Ok((STD_ASCENT, STD_DESCENT, 1000)),
            Self::TrueType(f) => {
                let per_em = i64::from(f.units_per_em());
                if per_em == 0 {
                    return Err(FolioError::Malformed("font has zero units per em"));
                }
                let asc = i64::from(f.ascent()).max(0);
                let desc = i64::from(f.descent()).abs();
                Ok((asc, desc, per_em))
            }
        }
    }

    fn has(&self, c: char) -> bool {
        match self {
            Self::Std14(f) => f.width(c).is_some(),
            Self::TrueType(f) => f.glyph_for(c).is_some(),
        }
    }
}

impl Advance {
    fn exact(units: i64, per_em: i64) -> Self {
        Self {
            units,
            per_em,
            substituted: false,
        }
    }
    fn substitute(units: i64, per_em: i64) -> Self {
        Self {
            units,
            per_em,
            substituted: true,
        }
    }
}

fn missing() -> FolioError {
    FolioError::Unsupported("character not in font")
}
