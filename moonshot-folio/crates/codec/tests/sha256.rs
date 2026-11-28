// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! SHA-256 tests (task F1.4): FIPS 180-4 vectors and padding edges.

// Test code only: the deny set protects shipped code paths, not assertions.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use moonshot_folio_codec::{sha256, sha256_hex, Sha256};

/// SHA-256 of `n` bytes of `a`, from the Python `hashlib` reference (test data only).
const EDGES: [(usize, &str); 17] = [
    (
        0,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    ),
    (
        1,
        "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb",
    ),
    (
        54,
        "a3f01b6939256127582ac8ae9fb47a382a244680806a3f613a118851c1ca1d47",
    ),
    (
        55,
        "9f4390f8d30c2dd92ec9f095b65e2b9ae9b0a925a5258e241c9f1e910f734318",
    ),
    (
        56,
        "b35439a4ac6f0948b6d6f9e3c6af0f5f590ce20f1bde7090ef7970686ec6738a",
    ),
    (
        57,
        "f13b2d724659eb3bf47f2dd6af1accc87b81f09f59f2b75e5c0bed6589dfe8c6",
    ),
    (
        63,
        "7d3e74a05d7db15bce4ad9ec0658ea98e3f06eeecf16b4c6fff2da457ddc2f34",
    ),
    (
        64,
        "ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb",
    ),
    (
        65,
        "635361c48bb9eab14198e76ea8ab7f1a41685d6ad62aa9146d301d4f17eb0ae0",
    ),
    (
        119,
        "31eba51c313a5c08226adf18d4a359cfdfd8d2e816b13f4af952f7ea6584dcfb",
    ),
    (
        120,
        "2f3d335432c70b580af0e8e1b3674a7c020d683aa5f73aaaedfdc55af904c21c",
    ),
    (
        127,
        "c57e9278af78fa3cab38667bef4ce29d783787a2f731d4e12200270f0c32320a",
    ),
    (
        128,
        "6836cf13bac400e9105071cd6af47084dfacad4e5e302c94bfed24e013afb73e",
    ),
    (
        129,
        "c12cb024a2e5551cca0e08fce8f1c5e314555cc3fef6329ee994a3db752166ae",
    ),
    (
        191,
        "311834ddbafe20677dddbf9f4beb2b5b0b9e6ee97dddb70fce4e8f62c1b7518d",
    ),
    (
        192,
        "7cee24628d290c16183532716cc5a8a889bc951b4b0a1507c32b8e29cee01052",
    ),
    (
        193,
        "33f93a9879ef18f9779150b2dbace6f8cc17b29e1af6be4e1048fc647489f1c2",
    ),
];

// REQ: FOLIO-REQ-010
#[test]
fn nist_published_vectors() {
    assert_eq!(
        sha256_hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        sha256_hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
    assert_eq!(
        sha256_hex(b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"),
        "cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1"
    );
}

// REQ: FOLIO-REQ-010
#[test]
fn one_million_a() {
    let data = vec![b'a'; 1_000_000];
    assert_eq!(
        sha256_hex(&data),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}

// REQ: FOLIO-REQ-010
#[test]
fn padding_edges_match_the_reference() {
    for (n, expect) in EDGES {
        assert_eq!(sha256_hex(&vec![b'a'; n]), expect, "length {n}");
    }
}

// REQ: FOLIO-REQ-010
#[test]
fn streaming_matches_one_shot_for_every_split() {
    let data: Vec<u8> = (0..=255_u8).cycle().take(200).collect();
    let whole = sha256(&data);
    for cut in 0..=data.len() {
        let mut h = Sha256::new();
        h.update(&data[..cut]);
        h.update(&data[cut..]);
        assert_eq!(h.finish(), whole, "cut {cut}");
    }
    for (x, y) in [(1_usize, 63_usize), (63, 1), (64, 64), (55, 9), (7, 130)] {
        let mut h = Sha256::new();
        h.update(&data[..x]);
        h.update(&data[x..x + y]);
        h.update(&data[x + y..]);
        assert_eq!(h.finish(), whole);
    }
    let mut h = Sha256::new();
    for b in &data {
        h.update(std::slice::from_ref(b));
    }
    assert_eq!(h.finish(), whole);
}

// REQ: FOLIO-REQ-010
#[test]
fn finish_does_not_consume_the_hasher() {
    let mut h = Sha256::new();
    h.update(b"ab");
    let _ = h.finish();
    h.update(b"c");
    assert_eq!(h.finish(), sha256(b"abc"));
}
