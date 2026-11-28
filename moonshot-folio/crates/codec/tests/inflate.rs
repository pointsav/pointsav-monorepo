// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Inflate, zlib and ZIP method 8 tests (task F1.3).

// Test code only: the deny set protects shipped code paths, not assertions.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

mod common;

use common::{
    hex, DYNAMIC, FIXED, HELLO, SINGLE_DIST_LEN1, SINGLE_DIST_LEN2, STORED, STORED_PLAIN,
    TEXT_ADLER, TEXT_CRC, TEXT_LEN, ZEROS, ZIP, ZLIB,
};
use moonshot_folio_codec::{adler32, crc32, inflate, zlib_decompress, ZipArchive};
use moonshot_folio_core::{FolioError, Limits};

const CAP: usize = 1 << 20;

// REQ: FOLIO-REQ-018
#[test]
fn decodes_fixed_dynamic_and_stored_blocks() {
    assert_eq!(inflate(&hex(FIXED), CAP).unwrap(), HELLO);
    assert_eq!(inflate(&hex(STORED), CAP).unwrap(), STORED_PLAIN);
    let text = inflate(&hex(DYNAMIC), CAP).unwrap();
    assert_eq!(text.len(), TEXT_LEN);
    assert_eq!(crc32(&text), TEXT_CRC);
    assert_eq!(adler32(&text), TEXT_ADLER);
}

// REQ: FOLIO-REQ-018
#[test]
fn zlib_wrapper_checks_header_and_adler() {
    let text = zlib_decompress(&hex(ZLIB), CAP).unwrap();
    assert_eq!(crc32(&text), TEXT_CRC);
    let mut bad = hex(ZLIB);
    let last = bad.len() - 1;
    bad[last] ^= 0xFF;
    assert_eq!(
        zlib_decompress(&bad, CAP),
        Err(FolioError::Malformed("adler-32 mismatch"))
    );
    let mut bad = hex(ZLIB);
    bad[0] = 0x79; // method 9
    assert!(matches!(
        zlib_decompress(&bad, CAP),
        Err(FolioError::Malformed(_))
    ));
    let mut dict = hex(ZLIB);
    dict[1] = (dict[1] | 0x20) & 0xE0; // FDICT set
    dict[1] |=
        u8::try_from((31 - ((u16::from(dict[0]) * 256 + u16::from(dict[1])) % 31)) % 31).unwrap();
    assert!(matches!(
        zlib_decompress(&dict, CAP),
        Err(FolioError::Unsupported(_) | FolioError::Malformed(_))
    ));
}

// REQ: FOLIO-REQ-005
// REQ: FOLIO-REQ-006
#[test]
fn output_cap_holds_at_limit_and_fails_at_limit_plus_one() {
    let z = hex(ZEROS);
    assert!(z.len() < 1000, "a 1 MB bomb in under 1 KB");
    assert_eq!(inflate(&z, 1_000_000).unwrap().len(), 1_000_000);
    assert_eq!(
        inflate(&z, 999_999),
        Err(FolioError::Limit("inflate output"))
    );
    assert_eq!(inflate(&hex(FIXED), 23).unwrap().len(), 23);
    assert!(matches!(
        inflate(&hex(FIXED), 22),
        Err(FolioError::Limit(_))
    ));
    assert_eq!(inflate(&hex(STORED), 17).unwrap().len(), 17);
    assert!(matches!(
        inflate(&hex(STORED), 16),
        Err(FolioError::Limit(_))
    ));
}

// REQ: FOLIO-REQ-018
#[test]
fn rejects_reserved_blocks_and_bad_stored_lengths() {
    assert!(matches!(
        inflate(&[0x07], CAP),
        Err(FolioError::Malformed(_))
    ));
    // BFINAL=1, stored; LEN=5, NLEN=0 (should be 0xFFFA).
    assert!(matches!(
        inflate(&[0x01, 5, 0, 0, 0, 1, 2, 3, 4, 5], CAP),
        Err(FolioError::Malformed(_))
    ));
    assert!(inflate(&[], CAP).is_err());
}

// REQ: FOLIO-REQ-018
// REQ: FOLIO-REQ-004
#[test]
fn every_truncation_of_a_valid_stream_is_an_error() {
    for v in [DYNAMIC, FIXED, STORED] {
        let b = hex(v);
        for cut in 0..b.len() {
            assert!(inflate(&b[..cut], CAP).is_err(), "cut {cut} of {}", b.len());
        }
    }
}

// REQ: FOLIO-REQ-004
#[test]
fn every_single_byte_corruption_is_handled_without_panic() {
    for v in [DYNAMIC, FIXED, STORED, ZLIB] {
        let b = hex(v);
        for i in 0..b.len() {
            for mask in [0xFF_u8, 0x01, 0x10] {
                let mut c = b.clone();
                c[i] ^= mask;
                let _ = inflate(&c, CAP);
                let _ = zlib_decompress(&c, CAP);
            }
        }
    }
}

// REQ: FOLIO-REQ-018
#[test]
fn zip_reads_deflated_entries() {
    let z = hex(ZIP);
    let a = ZipArchive::parse(&z, &Limits::default()).unwrap();
    let text = a.read(a.find("a.txt").unwrap()).unwrap();
    assert_eq!(crc32(&text), TEXT_CRC);
    let zeros = a.read(a.find("zeros.bin").unwrap()).unwrap();
    assert_eq!(zeros.len(), 20_000);
    assert!(zeros.iter().all(|&b| b == 0));
}

// REQ: FOLIO-REQ-005
// REQ: FOLIO-REQ-006
#[test]
fn zip_entry_limit_stops_a_declared_bomb_before_inflating() {
    let z = hex(ZIP);
    assert!(ZipArchive::parse(&z, &Limits::default().with_max_entry_bytes(20_000)).is_ok());
    let r = ZipArchive::parse(&z, &Limits::default().with_max_entry_bytes(19_999));
    assert!(matches!(r, Err(FolioError::Limit(_))));
    let r = ZipArchive::parse(
        &z,
        &Limits::default().with_max_total_bytes(20_000 + TEXT_LEN - 1),
    );
    assert!(
        matches!(r, Err(FolioError::Limit(_))),
        "total includes both entries"
    );
    let ok = Limits::default().with_max_total_bytes(20_000 + TEXT_LEN);
    assert!(ZipArchive::parse(&z, &ok).is_ok());
}

// REQ: FOLIO-REQ-017
// REQ: FOLIO-REQ-004
#[test]
fn zip_with_deflate_survives_every_single_byte_corruption() {
    let z = hex(ZIP);
    for i in 0..z.len() {
        let mut c = z.clone();
        c[i] ^= 0xFF;
        if let Ok(a) = ZipArchive::parse(&c, &Limits::default()) {
            for e in a.entries() {
                let _ = a.read(e);
            }
        }
    }
}

// REQ: FOLIO-REQ-018
#[test]
fn incomplete_codes_follow_zlib_a_single_one_bit_code_only() {
    assert_eq!(inflate(&hex(SINGLE_DIST_LEN1), CAP).unwrap(), b"a");
    assert!(matches!(
        inflate(&hex(SINGLE_DIST_LEN2), CAP),
        Err(FolioError::Malformed(_))
    ));
}

// REQ: FOLIO-REQ-018
// REQ: FOLIO-REQ-005
#[test]
fn many_empty_fixed_blocks_decode_in_linear_time() {
    // Each empty fixed block is 10 bits: BFINAL=0, BTYPE=01, then the 7-bit end-of-block code.
    let blocks = 50_000_usize;
    let mut bits: Vec<bool> = Vec::new();
    for i in 0..=blocks {
        bits.extend([i == blocks, true, false]);
        bits.extend([false; 7]);
    }
    let bytes: Vec<u8> = bits
        .chunks(8)
        .map(|c| {
            c.iter()
                .enumerate()
                .fold(0_u8, |acc, (i, &b)| acc | (u8::from(b) << i))
        })
        .collect();
    let start = std::time::Instant::now();
    assert_eq!(inflate(&bytes, CAP).unwrap(), Vec::<u8>::new());
    assert!(
        start.elapsed().as_secs() < 10,
        "fixed tables must be built once per stream"
    );
}
