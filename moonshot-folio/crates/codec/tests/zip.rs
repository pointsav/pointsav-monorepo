// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! ZIP writer and reader tests (task F1.2).

// Test code only: the deny set protects shipped code paths, not assertions.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use moonshot_folio_codec::{ZipArchive, ZipWriter};
use moonshot_folio_core::{FolioError, Limits};

/// Layout of the one-entry archive `a.txt` = `hello`: local 40 bytes, central at 40, EOCD at 91.
fn single() -> Vec<u8> {
    let mut w = ZipWriter::new(Limits::default());
    w.add("a.txt", b"hello").unwrap();
    w.finish().unwrap()
}

fn two() -> Vec<u8> {
    let mut w = ZipWriter::new(Limits::default());
    w.add("one", b"1234").unwrap();
    w.add("two", b"5678").unwrap();
    w.finish().unwrap()
}

// REQ: FOLIO-REQ-016
#[test]
fn round_trip_preserves_names_order_and_bytes() {
    let mut w = ZipWriter::new(Limits::default());
    w.add("a.txt", b"hello").unwrap();
    w.add("dir/empty", b"").unwrap();
    w.add("caf\u{e9}.txt", &[0_u8, 255, 7]).unwrap();
    let bytes = w.finish().unwrap();
    let z = ZipArchive::parse(&bytes, &Limits::default()).unwrap();
    let names: Vec<&str> = z
        .entries()
        .iter()
        .map(moonshot_folio_codec::ZipEntry::name)
        .collect();
    assert_eq!(names, ["a.txt", "dir/empty", "caf\u{e9}.txt"]);
    assert_eq!(z.read(z.find("a.txt").unwrap()).unwrap(), b"hello");
    assert_eq!(z.read(z.find("dir/empty").unwrap()).unwrap(), b"");
    assert_eq!(
        z.read(z.find("caf\u{e9}.txt").unwrap()).unwrap(),
        [0_u8, 255, 7]
    );
}

// REQ: FOLIO-REQ-016
// REQ: FOLIO-REQ-001
#[test]
fn output_is_byte_identical_and_uses_the_fixed_timestamp() {
    assert_eq!(two(), two());
    let b = single();
    assert_eq!(&b[0..4], b"PK\x03\x04");
    assert_eq!(&b[10..14], [0_u8, 0, 0x21, 0]);
}

// REQ: FOLIO-REQ-016
#[test]
fn writer_rejects_unsafe_and_duplicate_names() {
    let mut w = ZipWriter::new(Limits::default());
    for bad in [
        "", "/abs", "../x", "a/../b", "a\\b", "a\0b", "a//b", "./a", "a/.",
    ] {
        assert!(
            matches!(w.add(bad, b"x"), Err(FolioError::Invalid(_))),
            "{bad:?}"
        );
    }
    w.add("ok", b"x").unwrap();
    assert_eq!(
        w.add("ok", b"y"),
        Err(FolioError::Invalid("duplicate entry name"))
    );
}

// REQ: FOLIO-REQ-006
// REQ: FOLIO-REQ-005
#[test]
fn writer_limits_hold_at_limit_and_fail_at_limit_plus_one() {
    let mut w = ZipWriter::new(Limits::default().with_max_zip_entries(2));
    w.add("a", b"").unwrap();
    w.add("b", b"").unwrap();
    assert!(matches!(w.add("c", b""), Err(FolioError::Limit(_))));

    let mut w = ZipWriter::new(Limits::default().with_max_entry_bytes(4));
    w.add("a", &[0; 4]).unwrap();
    assert!(matches!(w.add("b", &[0; 5]), Err(FolioError::Limit(_))));

    let mut w = ZipWriter::new(Limits::default().with_max_total_bytes(6));
    w.add("a", &[0; 3]).unwrap();
    w.add("b", &[0; 3]).unwrap();
    assert!(matches!(w.add("c", &[0; 1]), Err(FolioError::Limit(_))));

    let mut w = ZipWriter::new(Limits::default().with_max_name_bytes(8));
    w.add("12345678", b"").unwrap();
    assert!(matches!(w.add("123456789", b""), Err(FolioError::Limit(_))));
}

// REQ: FOLIO-REQ-006
// REQ: FOLIO-REQ-005
#[test]
fn reader_limits_hold_at_limit_and_fail_at_limit_plus_one() {
    let b = two();
    let len = b.len();
    assert!(ZipArchive::parse(&b, &Limits::default().with_max_file_bytes(len)).is_ok());
    let r = ZipArchive::parse(&b, &Limits::default().with_max_file_bytes(len - 1));
    assert!(matches!(r, Err(FolioError::Limit(_))));
    assert!(ZipArchive::parse(&b, &Limits::default().with_max_zip_entries(2)).is_ok());
    let r = ZipArchive::parse(&b, &Limits::default().with_max_zip_entries(1));
    assert!(matches!(r, Err(FolioError::Limit(_))));
    assert!(ZipArchive::parse(&b, &Limits::default().with_max_entry_bytes(4)).is_ok());
    let r = ZipArchive::parse(&b, &Limits::default().with_max_entry_bytes(3));
    assert!(matches!(r, Err(FolioError::Limit(_))));
    assert!(ZipArchive::parse(&b, &Limits::default().with_max_total_bytes(8)).is_ok());
    let r = ZipArchive::parse(&b, &Limits::default().with_max_total_bytes(7));
    assert!(matches!(r, Err(FolioError::Limit(_))));
}

// REQ: FOLIO-REQ-017
#[test]
fn reader_rejects_zip_slip_names() {
    let mut w = ZipWriter::new(Limits::default());
    w.add("aaa", b"zzz").unwrap();
    let mut b = w.finish().unwrap();
    // Same-length rewrite of the name in the local and central headers.
    let mut hits = 0_u8;
    for i in 0..b.len().saturating_sub(3) {
        if &b[i..i + 3] == b"aaa" {
            b[i..i + 3].copy_from_slice(b"../");
            hits += 1;
        }
    }
    assert_eq!(hits, 2);
    assert!(matches!(
        ZipArchive::parse(&b, &Limits::default()),
        Err(FolioError::Malformed(_))
    ));
}

// REQ: FOLIO-REQ-017
#[test]
fn reader_detects_corrupt_data_by_crc() {
    let mut b = single();
    b[35] ^= 0xFF; // first data byte
    let z = ZipArchive::parse(&b, &Limits::default()).unwrap();
    let e = z.find("a.txt").unwrap();
    assert_eq!(z.read(e), Err(FolioError::Malformed("crc mismatch")));
}

// REQ: FOLIO-REQ-017
#[test]
fn reader_reports_unsupported_features() {
    let mut b = single();
    b[50] = 99; // central method = unknown
    assert!(matches!(
        ZipArchive::parse(&b, &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
    let mut b = single();
    b[48] |= 1; // central flags: encrypted
    assert!(matches!(
        ZipArchive::parse(&b, &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
    let mut b = single();
    b[60..64].copy_from_slice(&[0xFF; 4]); // central size = zip64 sentinel
    assert!(matches!(
        ZipArchive::parse(&b, &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
    let mut b = single();
    b[91 + 10] = 0xFF; // EOCD total entries = 0xFFFF
    b[91 + 11] = 0xFF;
    assert!(matches!(
        ZipArchive::parse(&b, &Limits::default()),
        Err(FolioError::Unsupported(_))
    ));
}

// REQ: FOLIO-REQ-017
// REQ: FOLIO-REQ-004
#[test]
fn every_truncation_is_an_error_not_a_panic() {
    let b = two();
    for cut in 0..b.len() {
        assert!(
            ZipArchive::parse(&b[..cut], &Limits::default()).is_err(),
            "cut {cut}"
        );
    }
}

// REQ: FOLIO-REQ-017
// REQ: FOLIO-REQ-004
#[test]
fn every_single_byte_corruption_is_handled_without_panic() {
    let b = two();
    for i in 0..b.len() {
        for mask in [0xFF_u8, 0x01, 0x80] {
            let mut c = b.clone();
            c[i] ^= mask;
            if let Ok(z) = ZipArchive::parse(&c, &Limits::default()) {
                for e in z.entries() {
                    let _ = z.read(e);
                }
            }
        }
    }
}

// REQ: FOLIO-REQ-017
#[test]
fn trailing_garbage_and_empty_archive_behave() {
    let mut b = single();
    b.push(0);
    assert!(ZipArchive::parse(&b, &Limits::default()).is_err());
    let empty = ZipWriter::new(Limits::default()).finish().unwrap();
    assert_eq!(empty.len(), 22);
    assert_eq!(
        ZipArchive::parse(&empty, &Limits::default())
            .unwrap()
            .entries()
            .len(),
        0
    );
}

// REQ: FOLIO-REQ-016
#[test]
fn writer_rejects_names_the_format_cannot_hold_or_that_are_unsafe() {
    let lim = Limits::default().with_max_name_bytes(100_000);
    let mut w = ZipWriter::new(lim);
    let long = "a".repeat(65_536);
    assert!(matches!(w.add(&long, b""), Err(FolioError::Limit(_))));
    assert!(w.add(&"a".repeat(65_535), b"").is_ok());
    for bad in ["C:/x", "c:foo", "a\nb", "a\u{7f}b", "tab\there"] {
        assert!(
            matches!(w.add(bad, b""), Err(FolioError::Invalid(_))),
            "{bad:?}"
        );
    }
}

// REQ: FOLIO-REQ-016
// REQ: FOLIO-REQ-017
#[test]
fn writer_and_reader_agree_at_the_65534_entry_boundary() {
    let lim = Limits::default().with_max_zip_entries(100_000);
    let mut w = ZipWriter::new(lim);
    for i in 0..65_534_u32 {
        w.add(&format!("f{i}"), b"").unwrap();
    }
    assert!(matches!(
        w.add("one-too-many", b""),
        Err(FolioError::Limit(_))
    ));
    let bytes = w.finish().unwrap();
    let z = ZipArchive::parse(&bytes, &lim).unwrap();
    assert_eq!(z.entries().len(), 65_534);
}

// REQ: FOLIO-REQ-006
// REQ: FOLIO-REQ-017
#[test]
fn reader_name_limit_holds_at_limit_and_fails_at_limit_plus_one() {
    let b = single(); // name "a.txt" is 5 bytes
    assert!(ZipArchive::parse(&b, &Limits::default().with_max_name_bytes(5)).is_ok());
    let r = ZipArchive::parse(&b, &Limits::default().with_max_name_bytes(4));
    assert!(matches!(r, Err(FolioError::Limit(_))));
}

// REQ: FOLIO-REQ-017
#[test]
fn entry_data_may_not_run_into_the_central_directory_or_another_entry() {
    let mut b = single();
    b[60..64].copy_from_slice(&40_u32.to_le_bytes()); // compressed size
    b[64..68].copy_from_slice(&40_u32.to_le_bytes()); // size: data would end past the directory start
    assert_eq!(
        ZipArchive::parse(&b, &Limits::default()).err(),
        Some(FolioError::Malformed(
            "entry data overlaps the central directory"
        ))
    );
    // two(): entry one's data would run over entry two's local header.
    let mut b = two();
    b[94..98].copy_from_slice(&20_u32.to_le_bytes());
    b[98..102].copy_from_slice(&20_u32.to_le_bytes());
    assert_eq!(
        ZipArchive::parse(&b, &Limits::default()).err(),
        Some(FolioError::Malformed("overlapping entries"))
    );
}

// REQ: FOLIO-REQ-017
#[test]
fn an_archive_comment_holding_a_fake_end_record_is_read_as_that_record() {
    // Documented behaviour (same as common tools): the last end record whose comment length fits wins.
    let mut b = single();
    let eocd = b[b.len() - 22..].to_vec();
    let n = u16::try_from(eocd.len()).unwrap();
    let at = b.len() - 2;
    b[at..].copy_from_slice(&n.to_le_bytes());
    b.extend_from_slice(&[0; 0]);
    let mut fake = eocd.clone();
    fake[8..12].copy_from_slice(&[0; 4]); // zero entries
    fake[12..20].copy_from_slice(&[0; 8]);
    b.extend_from_slice(&fake);
    let z = ZipArchive::parse(&b, &Limits::default()).unwrap();
    assert_eq!(z.entries().len(), 0);
}
