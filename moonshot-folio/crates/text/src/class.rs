// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The effective Line_Break classes used by the breaker and the property lookup.

use crate::tables::LB_TABLE;

/// A UAX#14 Line_Break class, plus five derived refinements produced by the table generator.
///
/// The refinements carry extra facts the rules need so that no second table is required:
/// `QU_PI`/`QU_PF` (quotation marks by General_Category), `OP_EA`/`CP_EA` (brackets with East
/// Asian Width F/W/H, LB30) and `ID_PICT` (unassigned Extended_Pictographic, LB30b).
#[allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Lb {
    AI,
    AL,
    B2,
    BA,
    BB,
    BK,
    CB,
    CJ,
    CL,
    CM,
    CP,
    CP_EA,
    CR,
    EB,
    EM,
    EX,
    GL,
    H2,
    H3,
    HL,
    HY,
    ID,
    ID_PICT,
    IN,
    IS,
    JL,
    JT,
    JV,
    LF,
    NL,
    NS,
    NU,
    OP,
    OP_EA,
    PO,
    PR,
    QU,
    QU_PF,
    QU_PI,
    RI,
    SA,
    SG,
    SP,
    SY,
    WJ,
    XX,
    ZW,
    ZWJ,
}

impl Lb {
    /// LB1: resolve ambiguous and unknown classes (SA is already resolved by the generator).
    pub(crate) fn resolve(self) -> Self {
        match self {
            Self::AI | Self::SG | Self::XX | Self::SA => Self::AL,
            Self::CJ => Self::NS,
            other => other,
        }
    }

    /// Fold the generator refinements back to their parent UAX#14 class.
    pub(crate) fn base(self) -> Self {
        match self {
            Self::QU_PI | Self::QU_PF => Self::QU,
            Self::OP_EA => Self::OP,
            Self::CP_EA => Self::CP,
            Self::ID_PICT => Self::ID,
            other => other,
        }
    }

    /// True for classes that force a break after them (LB4, LB5).
    pub(crate) fn is_hard(self) -> bool {
        matches!(self, Self::BK | Self::CR | Self::LF | Self::NL)
    }
}

/// Raw (unresolved) Line_Break class of `c`.
pub(crate) fn class_of(c: char) -> Lb {
    let cp = u32::from(c);
    let idx = LB_TABLE.partition_point(|e| e.0 <= cp);
    idx.checked_sub(1)
        .and_then(|i| LB_TABLE.get(i))
        .map_or(Lb::XX, |e| e.1)
}
