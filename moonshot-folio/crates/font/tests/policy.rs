// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Embedding permission and missing-glyph policy tests (task F4.5).

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

mod common;
use common::{build, entry, CmapKind};
use moonshot_folio_core::{FolioError, Limits};
use moonshot_folio_font::{check_embedding, map_text, Font, MissingGlyph};

fn with_fs_type(v: u16) -> Vec<u8> {
    let mut d = build(CmapKind::Format4, false);
    let e = entry(&d, b"OS/2");
    let off = u32::from_be_bytes(d[e + 8..e + 12].try_into().unwrap()) as usize;
    d[off + 8..off + 10].copy_from_slice(&v.to_be_bytes());
    d
}

// REQ: FOLIO-REQ-040
#[test]
fn fs_type_rules() {
    let check = |v: u16, sub: bool| {
        let d = with_fs_type(v);
        check_embedding(&Font::parse(&d, &Limits::default()).unwrap(), sub)
    };
    for ok in [0_u16, 0x0004, 0x0008, 0x000C] {
        assert!(check(ok, true).is_ok(), "{ok:#x}");
    }
    assert!(matches!(
        check(0x0002, false),
        Err(FolioError::Unsupported(_))
    ));
    assert!(matches!(
        check(0x0200, false),
        Err(FolioError::Unsupported(_))
    ));
    assert!(check(0x0100, false).is_ok());
    assert!(matches!(
        check(0x0100, true),
        Err(FolioError::Unsupported(_))
    ));
    assert!(matches!(
        check(0x0104, true),
        Err(FolioError::Unsupported(_))
    ));
}

// REQ: FOLIO-REQ-040
#[test]
fn missing_glyph_policies() {
    let d = build(CmapKind::Format4, false);
    let f = Font::parse(&d, &Limits::default()).unwrap();
    assert_eq!(
        map_text(&f, "AB A", MissingGlyph::Reject).unwrap(),
        (vec![1, 2, 3, 1], 0)
    );
    assert!(matches!(
        map_text(&f, "AxB", MissingGlyph::Reject),
        Err(FolioError::Unsupported(_))
    ));
    assert_eq!(
        map_text(&f, "AxB\u{20AC}", MissingGlyph::Notdef).unwrap(),
        (vec![1, 0, 2, 0], 2)
    );
    assert_eq!(map_text(&f, "", MissingGlyph::Reject).unwrap(), (vec![], 0));
}
