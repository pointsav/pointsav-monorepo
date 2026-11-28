// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Unit tests for `Emu` (task F2.1).

#![allow(clippy::unwrap_used)]

use moonshot_folio_core::{Emu, FolioError, EMU_PER_INCH};

// REQ: FOLIO-REQ-021
#[test]
fn unit_constructors_are_exact() {
    assert_eq!(Emu::from_inches(1).unwrap().raw(), EMU_PER_INCH);
    assert_eq!(Emu::from_points(72).unwrap(), Emu::from_inches(1).unwrap());
    assert_eq!(Emu::from_twips(1440).unwrap(), Emu::from_inches(1).unwrap());
    assert_eq!(Emu::from_mm(254).unwrap(), Emu::from_inches(10).unwrap());
}

// REQ: FOLIO-REQ-021
#[test]
fn ratio_rounds_half_away_from_zero() {
    // 1/2 of 3 EMU units = 1.5 -> 2; negative -> -2; 1/3 of 1 -> 0.
    assert_eq!(Emu::from_ratio(1, 3, 2).unwrap().raw(), 2);
    assert_eq!(Emu::from_ratio(-1, 3, 2).unwrap().raw(), -2);
    assert_eq!(Emu::from_ratio(1, 3, -2).unwrap().raw(), -2);
    assert_eq!(Emu::from_ratio(1, 1, 3).unwrap().raw(), 0);
    assert_eq!(Emu::from_ratio(2, 1, 3).unwrap().raw(), 1);
}

// REQ: FOLIO-REQ-021
#[test]
fn overflow_and_zero_denominator_are_errors() {
    assert!(matches!(
        Emu::from_points(i64::MAX),
        Err(FolioError::Limit(_))
    ));
    assert!(matches!(
        Emu::from_ratio(1, 1, 0),
        Err(FolioError::Invalid(_))
    ));
    assert!(matches!(
        Emu::from_raw(i64::MAX).checked_add(Emu::from_raw(1)),
        Err(FolioError::Limit(_))
    ));
    assert!(matches!(
        Emu::from_raw(i64::MIN).checked_sub(Emu::from_raw(1)),
        Err(FolioError::Limit(_))
    ));
    assert!(matches!(
        Emu::from_raw(2).checked_mul(i64::MAX),
        Err(FolioError::Limit(_))
    ));
}
