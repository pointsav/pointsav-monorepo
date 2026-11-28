// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! JPEG header parser tests (task F1.7).

// Test code only: the deny set protects shipped code paths, not assertions.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use moonshot_folio_codec::parse_jpeg_header;
use moonshot_folio_core::{FolioError, Limits};

/// A marker segment: `FF marker`, big-endian length, payload.
fn seg(marker: u8, payload: &[u8]) -> Vec<u8> {
    let mut v = vec![0xFF, marker];
    v.extend_from_slice(&u16::try_from(payload.len() + 2).unwrap().to_be_bytes());
    v.extend_from_slice(payload);
    v
}

/// A frame header payload.
fn frame(precision: u8, h: u16, w: u16, comps: u8) -> Vec<u8> {
    let mut p = vec![precision];
    p.extend_from_slice(&h.to_be_bytes());
    p.extend_from_slice(&w.to_be_bytes());
    p.push(comps);
    for i in 0..comps {
        p.extend_from_slice(&[i + 1, 0x11, 0]);
    }
    p
}

/// SOI, JFIF, the given frame segment, then a scan header.
fn file(marker: u8, payload: &[u8]) -> Vec<u8> {
    let mut v = vec![0xFF, 0xD8];
    v.extend(seg(0xE0, b"JFIF\0\x01\x01\0\0\x01\0\x01\0\0"));
    v.extend(seg(marker, payload));
    v.extend(seg(0xDA, &[1, 1, 0, 0, 0x3F, 0]));
    v
}

// REQ: FOLIO-REQ-020
#[test]
fn reads_baseline_and_progressive_frames() {
    let info = parse_jpeg_header(&file(0xC0, &frame(8, 480, 640, 3)), &Limits::default()).unwrap();
    assert_eq!(
        (
            info.width(),
            info.height(),
            info.components(),
            info.precision()
        ),
        (640, 480, 3, 8)
    );
    assert!(!info.progressive());
    let info = parse_jpeg_header(&file(0xC2, &frame(8, 10, 20, 1)), &Limits::default()).unwrap();
    assert!(info.progressive());
    assert_eq!(
        (info.width(), info.height(), info.components()),
        (20, 10, 1)
    );
    assert_eq!(info.adobe_transform(), None);
    let info = parse_jpeg_header(&file(0xC1, &frame(8, 5, 5, 4)), &Limits::default()).unwrap();
    assert_eq!(info.components(), 4);
}

// REQ: FOLIO-REQ-020
#[test]
fn reads_the_adobe_transform_flag() {
    let mut v = vec![0xFF, 0xD8];
    v.extend(seg(0xEE, b"Adobe\0\x64\0\0\0\0\x02"));
    v.extend(seg(0xC0, &frame(8, 4, 4, 4)));
    v.extend(seg(0xDA, &[1, 1, 0, 0, 0x3F, 0]));
    let info = parse_jpeg_header(&v, &Limits::default()).unwrap();
    assert_eq!(info.adobe_transform(), Some(2));
}

// REQ: FOLIO-REQ-020
#[test]
fn tolerates_fill_bytes_before_markers() {
    let mut v = vec![0xFF, 0xD8, 0xFF, 0xFF];
    v.extend(seg(0xC0, &frame(8, 3, 3, 3))[1..].iter());
    v.extend(seg(0xDA, &[1, 1, 0, 0, 0x3F, 0]));
    assert!(parse_jpeg_header(&v, &Limits::default()).is_ok());
}

// REQ: FOLIO-REQ-020
#[test]
fn rejects_unsupported_and_invalid_frames() {
    for m in [0xC3_u8, 0xC5, 0xC9, 0xCF] {
        let r = parse_jpeg_header(&file(m, &frame(8, 4, 4, 3)), &Limits::default());
        assert!(
            matches!(r, Err(FolioError::Unsupported(_))),
            "marker {m:#x}"
        );
    }
    for bad in [frame(8, 0, 4, 3), frame(8, 4, 0, 3)] {
        assert!(matches!(
            parse_jpeg_header(&file(0xC0, &bad), &Limits::default()),
            Err(FolioError::Malformed(_))
        ));
    }
    assert!(matches!(
        parse_jpeg_header(&file(0xC0, &frame(8, 4, 4, 2)), &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
    assert!(matches!(
        parse_jpeg_header(&file(0xC0, &frame(16, 4, 4, 3)), &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
    assert!(
        parse_jpeg_header(&[0xFF, 0xD8, 0xFF, 0xD9], &Limits::default()).is_err(),
        "no frame"
    );
    assert!(
        parse_jpeg_header(&[0x89, 0x50], &Limits::default()).is_err(),
        "no SOI"
    );
    assert!(parse_jpeg_header(&[], &Limits::default()).is_err());
}

// REQ: FOLIO-REQ-006
// REQ: FOLIO-REQ-020
#[test]
fn pixel_and_file_limits_hold_at_limit_and_fail_at_limit_plus_one() {
    let v = file(0xC0, &frame(8, 100, 200, 3));
    assert!(parse_jpeg_header(&v, &Limits::default().with_max_image_pixels(20_000)).is_ok());
    let r = parse_jpeg_header(&v, &Limits::default().with_max_image_pixels(19_999));
    assert_eq!(r.unwrap_err(), FolioError::Limit("image pixels"));
    assert!(parse_jpeg_header(&v, &Limits::default().with_max_file_bytes(v.len())).is_ok());
    assert!(matches!(
        parse_jpeg_header(&v, &Limits::default().with_max_file_bytes(v.len() - 1)),
        Err(FolioError::Limit(_))
    ));
}

// REQ: FOLIO-REQ-020
// REQ: FOLIO-REQ-004
#[test]
fn every_truncation_and_corruption_is_handled_without_panic() {
    let v = file(0xC0, &frame(8, 480, 640, 3));
    for cut in 0..v.len() - 8 {
        assert!(
            parse_jpeg_header(&v[..cut], &Limits::default()).is_err(),
            "cut {cut}"
        );
    }
    assert!(
        parse_jpeg_header(&v[..v.len() - 8], &Limits::default()).is_ok(),
        "stops at the scan marker"
    );
    for i in 0..v.len() {
        for mask in [0xFF_u8, 0x01, 0x80] {
            let mut c = v.clone();
            c[i] ^= mask;
            let _ = parse_jpeg_header(&c, &Limits::default());
        }
    }
    let mut bad_len = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x01];
    bad_len.extend_from_slice(&[0; 8]);
    assert!(
        parse_jpeg_header(&bad_len, &Limits::default()).is_err(),
        "segment length below 2"
    );
}

// REQ: FOLIO-REQ-020
#[test]
fn rejects_duplicate_frames_bad_lengths_and_bad_sampling() {
    let mut dup = vec![0xFF, 0xD8];
    dup.extend(seg(0xC0, &frame(8, 10, 10, 3)));
    dup.extend(seg(0xC0, &frame(8, 200, 100, 3)));
    dup.extend(seg(0xDA, &[1, 1, 0, 0, 0x3F, 0]));
    assert_eq!(
        parse_jpeg_header(&dup, &Limits::default()).unwrap_err(),
        FolioError::Malformed("duplicate frame header")
    );
    let mut long = frame(8, 4, 4, 3);
    long.push(0);
    assert!(
        parse_jpeg_header(&file(0xC0, &long), &Limits::default()).is_err(),
        "extra payload"
    );
    let mut short = frame(8, 4, 4, 3);
    short.pop();
    assert!(
        parse_jpeg_header(&file(0xC0, &short), &Limits::default()).is_err(),
        "short payload"
    );
    for hv in [0x00_u8, 0x01, 0x10, 0x51, 0xFF] {
        let mut f = frame(8, 4, 4, 3);
        f[7] = hv;
        assert!(
            parse_jpeg_header(&file(0xC0, &f), &Limits::default()).is_err(),
            "sampling {hv:#x}"
        );
    }
    let mut q = frame(8, 4, 4, 3);
    q[8] = 4;
    assert!(
        parse_jpeg_header(&file(0xC0, &q), &Limits::default()).is_err(),
        "quantisation selector 4"
    );
}

// REQ: FOLIO-REQ-020
#[test]
fn accepts_twelve_bit_extended_and_progressive_frames() {
    for m in [0xC1_u8, 0xC2] {
        let info = parse_jpeg_header(&file(m, &frame(12, 8, 8, 1)), &Limits::default()).unwrap();
        assert_eq!(info.precision(), 12);
    }
    assert!(
        parse_jpeg_header(&file(0xC0, &frame(12, 8, 8, 1)), &Limits::default()).is_err(),
        "12-bit baseline"
    );
}
