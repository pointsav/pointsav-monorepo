// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! PNG decoder tests (task F1.7).

// Test code only: the deny set protects shipped code paths, not assertions.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

mod common;
#[allow(unreachable_pub, missing_docs, clippy::unreadable_literal)]
#[path = "common/png_vectors.rs"]
mod png_vectors;

use common::{hex, ZEROS};
use moonshot_folio_codec::{adler32, crc32, decode_png, PngColor};
use moonshot_folio_core::{FolioError, Limits};
use png_vectors::{PNG_ANCILLARY, PNG_CASES, PNG_CRITICAL, PNG_INTERLACED, PNG_PLAIN};

/// A PNG chunk: length, type, body, CRC.
fn chunk(kind: [u8; 4], body: &[u8]) -> Vec<u8> {
    let mut covered = kind.to_vec();
    covered.extend_from_slice(body);
    let mut out = u32::try_from(body.len()).unwrap().to_be_bytes().to_vec();
    out.extend_from_slice(&covered);
    out.extend_from_slice(&crc32(&covered).to_be_bytes());
    out
}

/// A PNG with the given IHDR fields and IDAT body.
fn build(w: u32, h: u32, depth: u8, ctype: u8, idat: &[u8]) -> Vec<u8> {
    let mut ihdr = w.to_be_bytes().to_vec();
    ihdr.extend_from_slice(&h.to_be_bytes());
    ihdr.extend_from_slice(&[depth, ctype, 0, 0, 0]);
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    out.extend(chunk(*b"IHDR", &ihdr));
    out.extend(chunk(*b"IDAT", idat));
    out.extend(chunk(*b"IEND", b""));
    out
}

// REQ: FOLIO-REQ-019
#[test]
fn every_vector_decodes_to_the_expected_rgba() {
    for (name, w, h, png, rgba) in PNG_CASES {
        let img =
            decode_png(&hex(png), &Limits::default()).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!((img.width(), img.height()), (*w, *h), "{name}");
        assert_eq!(
            img.to_rgba8(&Limits::default()).unwrap(),
            hex(rgba),
            "{name}"
        );
    }
}

// REQ: FOLIO-REQ-019
#[test]
fn colour_types_and_depths_are_reported() {
    let find = |n: &str| PNG_CASES.iter().find(|c| c.0 == n).unwrap();
    let img = decode_png(&hex(find("pal4").3), &Limits::default()).unwrap();
    assert_eq!(
        (img.color(), img.bit_depth(), img.palette().len()),
        (PngColor::Palette, 4, 16)
    );
    let img = decode_png(&hex(find("rgba16").3), &Limits::default()).unwrap();
    assert_eq!((img.color(), img.bit_depth()), (PngColor::Rgba, 16));
    assert_eq!(img.pixels().len(), 5 * 5 * 8);
}

// REQ: FOLIO-REQ-019
#[test]
fn unsupported_and_ignored_chunks() {
    let r = decode_png(&hex(PNG_INTERLACED), &Limits::default());
    assert_eq!(r.unwrap_err(), FolioError::Unsupported("interlaced png"));
    let r = decode_png(&hex(PNG_CRITICAL), &Limits::default());
    assert_eq!(
        r.unwrap_err(),
        FolioError::Unsupported("unknown critical png chunk")
    );
    let plain = decode_png(&hex(PNG_PLAIN), &Limits::default()).unwrap();
    let anc = decode_png(&hex(PNG_ANCILLARY), &Limits::default()).unwrap();
    assert_eq!(plain.pixels(), anc.pixels());
}

// REQ: FOLIO-REQ-019
// REQ: FOLIO-REQ-004
#[test]
fn every_truncation_is_an_error() {
    for (name, _, _, png, _) in PNG_CASES {
        let b = hex(png);
        for cut in 0..b.len() {
            assert!(
                decode_png(&b[..cut], &Limits::default()).is_err(),
                "{name} cut {cut}"
            );
        }
    }
}

// REQ: FOLIO-REQ-019
// REQ: FOLIO-REQ-004
#[test]
fn every_single_byte_corruption_is_handled_without_panic() {
    for (name, _, _, png, _) in PNG_CASES {
        let b = hex(png);
        for i in 0..b.len() {
            let mut c = b.clone();
            c[i] ^= 0xFF;
            if let Ok(img) = decode_png(&c, &Limits::default()) {
                let _ = img.to_rgba8(&Limits::default());
            }
            let _ = name;
        }
    }
}

// REQ: FOLIO-REQ-019
#[test]
fn rejects_bad_signature_crc_and_header_fields() {
    let good = hex(PNG_PLAIN);
    let mut b = good.clone();
    b[1] = b'X';
    assert_eq!(
        decode_png(&b, &Limits::default()).unwrap_err(),
        FolioError::Malformed("bad png signature")
    );
    let mut b = good.clone();
    b[20] ^= 1; // inside IHDR
    assert_eq!(
        decode_png(&b, &Limits::default()).unwrap_err(),
        FolioError::Malformed("png chunk crc mismatch")
    );
    let mut b = good.clone();
    b.push(0);
    assert_eq!(
        decode_png(&b, &Limits::default()).unwrap_err(),
        FolioError::Malformed("data after IEND")
    );
    assert!(decode_png(&build(0, 4, 8, 0, &[]), &Limits::default()).is_err());
    assert!(
        decode_png(&build(4, 4, 3, 0, &[]), &Limits::default()).is_err(),
        "depth 3 is illegal"
    );
    assert!(
        decode_png(&build(4, 4, 8, 5, &[]), &Limits::default()).is_err(),
        "colour type 5 is illegal"
    );
    assert!(
        decode_png(&build(4, 4, 16, 3, &[]), &Limits::default()).is_err(),
        "palette depth 16 is illegal"
    );
}

// REQ: FOLIO-REQ-006
// REQ: FOLIO-REQ-019
#[test]
fn pixel_and_size_limits_hold_at_limit_and_fail_at_limit_plus_one() {
    let (_, w, h, png, _) = PNG_CASES.iter().find(|c| c.0 == "rgba8").unwrap();
    let b = hex(png);
    let px = usize::try_from(w * h).unwrap();
    assert!(decode_png(&b, &Limits::default().with_max_image_pixels(px)).is_ok());
    let r = decode_png(&b, &Limits::default().with_max_image_pixels(px - 1));
    assert_eq!(r.unwrap_err(), FolioError::Limit("image pixels"));
    assert!(decode_png(&b, &Limits::default().with_max_file_bytes(b.len())).is_ok());
    assert!(matches!(
        decode_png(&b, &Limits::default().with_max_file_bytes(b.len() - 1)),
        Err(FolioError::Limit(_))
    ));
    let img = decode_png(&b, &Limits::default()).unwrap();
    assert!(img
        .to_rgba8(&Limits::default().with_max_entry_bytes(px * 4))
        .is_ok());
    assert!(matches!(
        img.to_rgba8(&Limits::default().with_max_entry_bytes(px * 4 - 1)),
        Err(FolioError::Limit(_))
    ));
}

// REQ: FOLIO-REQ-005
// REQ: FOLIO-REQ-019
#[test]
fn huge_declared_dimensions_fail_before_any_allocation() {
    let r = decode_png(&build(100_000, 100_000, 8, 0, &[]), &Limits::default());
    assert_eq!(r.unwrap_err(), FolioError::Limit("image pixels"));
    let r = decode_png(&build(u32::MAX, u32::MAX, 16, 6, &[]), &Limits::default());
    assert!(matches!(r, Err(FolioError::Limit(_))));
}

// REQ: FOLIO-REQ-005
// REQ: FOLIO-REQ-019
#[test]
fn a_decompression_bomb_in_idat_is_capped_by_the_header() {
    let mut z = vec![0x78, 0x9C];
    z.extend(hex(ZEROS));
    z.extend_from_slice(&adler32(&vec![0_u8; 1_000_000]).to_be_bytes());
    let r = decode_png(&build(1, 1, 8, 0, &z), &Limits::default());
    assert!(matches!(r, Err(FolioError::Limit(_))), "{r:?}");
}

/// Split a PNG into (type, body) chunks, checking nothing.
fn chunks(png: &[u8]) -> Vec<([u8; 4], Vec<u8>)> {
    let mut out = Vec::new();
    let mut pos = 8;
    while pos + 12 <= png.len() {
        let len =
            usize::try_from(u32::from_be_bytes(png[pos..pos + 4].try_into().unwrap())).unwrap();
        let kind: [u8; 4] = png[pos + 4..pos + 8].try_into().unwrap();
        out.push((kind, png[pos + 8..pos + 8 + len].to_vec()));
        pos += 12 + len;
    }
    out
}

/// Re-assemble chunks with fresh CRCs.
fn assemble(parts: &[([u8; 4], Vec<u8>)]) -> Vec<u8> {
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    for (k, b) in parts {
        out.extend(chunk(*k, b));
    }
    out
}

fn case(name: &str) -> Vec<([u8; 4], Vec<u8>)> {
    chunks(&hex(PNG_CASES.iter().find(|c| c.0 == name).unwrap().3))
}

/// Insert `extra` immediately before the first IDAT.
fn before_idat(mut parts: Vec<([u8; 4], Vec<u8>)>, extra: ([u8; 4], Vec<u8>)) -> Vec<u8> {
    let at = parts.iter().position(|c| &c.0 == b"IDAT").unwrap();
    parts.insert(at, extra);
    assemble(&parts)
}

fn decodes(png: &[u8]) -> bool {
    decode_png(png, &Limits::default()).is_ok()
}

// REQ: FOLIO-REQ-019
#[test]
fn trns_must_have_the_exact_length_for_its_colour_type() {
    assert!(decodes(&before_idat(case("gray8"), (*b"tRNS", vec![0, 7]))));
    assert!(!decodes(&before_idat(
        case("gray8"),
        (*b"tRNS", vec![0, 7, 0])
    )));
    assert!(!decodes(&before_idat(case("rgb8"), (*b"tRNS", vec![0; 5]))));
    assert!(decodes(&before_idat(case("rgb8"), (*b"tRNS", vec![0; 6]))));
    assert!(!decodes(&before_idat(
        case("gray_alpha8"),
        (*b"tRNS", vec![0, 1])
    )));
    assert!(!decodes(&before_idat(
        case("rgba8"),
        (*b"tRNS", vec![0; 6])
    )));
    // pal8 has 16 palette entries: 16 alpha values fit, 17 do not.
    let base = || {
        case("pal8_trns")
            .into_iter()
            .filter(|c| &c.0 != b"tRNS")
            .collect::<Vec<_>>()
    };
    assert!(decodes(&before_idat(base(), (*b"tRNS", vec![255; 16]))));
    assert!(!decodes(&before_idat(base(), (*b"tRNS", vec![255; 17]))));
}

// REQ: FOLIO-REQ-019
// REQ: FOLIO-REQ-005
#[test]
fn a_huge_trns_chunk_is_rejected_at_decode_time() {
    let start = std::time::Instant::now();
    let png = before_idat(case("gray8"), (*b"tRNS", vec![0xAA; 400_000]));
    assert!(!decodes(&png));
    assert!(start.elapsed().as_secs() < 5);
}

// REQ: FOLIO-REQ-019
#[test]
fn chunk_order_rules_are_enforced() {
    let plain = || case("gray8");
    assert!(
        !decodes(&before_idat(plain(), (*b"PLTE", vec![0, 0, 0]))),
        "PLTE in a gray image"
    );
    let mut dup = case("pal8_trns");
    let at = dup.iter().position(|c| &c.0 == b"PLTE").unwrap();
    let copy = dup[at].clone();
    dup.insert(at, copy);
    assert!(!decodes(&assemble(&dup)), "duplicate PLTE");
    let mut dup = case("pal8_trns");
    let at = dup.iter().position(|c| &c.0 == b"tRNS").unwrap();
    let copy = dup[at].clone();
    dup.insert(at, copy);
    assert!(!decodes(&assemble(&dup)), "duplicate tRNS");
    let mut late = case("pal8_trns");
    let at = late.iter().position(|c| &c.0 == b"tRNS").unwrap();
    let t = late.remove(at);
    let idat = late.iter().position(|c| &c.0 == b"IDAT").unwrap();
    late.insert(idat + 1, t);
    assert!(!decodes(&assemble(&late)), "tRNS after IDAT");
    let mut tr_first = case("pal8_trns");
    let at = tr_first.iter().position(|c| &c.0 == b"tRNS").unwrap();
    let t = tr_first.remove(at);
    let plte = tr_first.iter().position(|c| &c.0 == b"PLTE").unwrap();
    tr_first.insert(plte, t);
    assert!(!decodes(&assemble(&tr_first)), "tRNS before PLTE");
    let mut many = case("pal1");
    let at = many.iter().position(|c| &c.0 == b"PLTE").unwrap();
    many[at].1 = vec![0; 9];
    assert!(!decodes(&assemble(&many)), "3 palette entries at depth 1");
}

// REQ: FOLIO-REQ-019
#[test]
fn idat_must_be_consecutive_and_iend_empty() {
    let mut parts = case("gray8");
    let at = parts.iter().position(|c| &c.0 == b"IDAT").unwrap();
    let body = parts[at].1.clone();
    let (a, b) = body.split_at(body.len() / 2);
    parts[at].1 = a.to_vec();
    parts.insert(at + 1, (*b"IDAT", b.to_vec()));
    assert!(
        decodes(&assemble(&parts)),
        "two consecutive IDAT chunks are legal"
    );
    parts.insert(at + 1, (*b"tEXt", b"k\0v".to_vec()));
    assert!(!decodes(&assemble(&parts)), "a chunk between IDATs");
    let mut iend = case("gray8");
    let last = iend.len() - 1;
    iend[last].1 = vec![0];
    assert!(!decodes(&assemble(&iend)), "IEND with a body");
    let mut none: Vec<_> = case("gray8")
        .into_iter()
        .filter(|c| &c.0 != b"IDAT")
        .collect();
    assert!(!decodes(&assemble(&none)), "no IDAT");
    none.clear();
}

// REQ: FOLIO-REQ-019
#[test]
fn chunk_type_bytes_must_be_letters_with_the_reserved_bit_clear() {
    assert!(!decodes(&before_idat(case("gray8"), (*b"ab1d", vec![]))));
    assert!(
        !decodes(&before_idat(case("gray8"), (*b"tExt", vec![]))),
        "reserved bit set"
    );
    assert!(decodes(&before_idat(
        case("gray8"),
        (*b"tEXt", b"k\0v".to_vec())
    )));
}

// REQ: FOLIO-REQ-019
// REQ: FOLIO-REQ-004
#[test]
fn crc_repaired_corruption_of_every_chunk_body_never_panics() {
    for (name, _, _, png, _) in PNG_CASES {
        let parts = chunks(&hex(png));
        for (i, (_, body)) in parts.iter().enumerate() {
            for j in 0..body.len() {
                let mut p = parts.clone();
                p[i].1[j] ^= 0xFF;
                if let Ok(img) = decode_png(&assemble(&p), &Limits::default()) {
                    let _ = img.to_rgba8(&Limits::default());
                }
            }
        }
        let _ = name;
    }
}
