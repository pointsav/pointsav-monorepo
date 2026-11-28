// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! CRC-32 (ISO 3309 / ZIP / PNG): reflected polynomial `0xEDB8_8320`, init and final xor `0xFFFF_FFFF`.

/// Reflected CRC-32 polynomial.
const POLY: u32 = 0xEDB8_8320;

/// Table entry for one byte value.
fn entry(n: u32) -> u32 {
    let mut c = n;
    for _ in 0..8 {
        c = if c & 1 == 1 { POLY ^ (c >> 1) } else { c >> 1 };
    }
    c
}

/// Streaming CRC-32 hasher.
#[derive(Clone, Debug)]
pub struct Crc32 {
    table: [u32; 256],
    state: u32,
}

impl Default for Crc32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Crc32 {
    /// A fresh hasher.
    #[must_use]
    pub fn new() -> Self {
        let table = core::array::from_fn(|i| entry(u32::try_from(i).unwrap_or(0)));
        Self {
            table,
            state: 0xFFFF_FFFF,
        }
    }

    /// Feed bytes.
    pub fn update(&mut self, data: &[u8]) {
        let mut c = self.state;
        for &b in data {
            let idx = usize::from(u8::try_from((c ^ u32::from(b)) & 0xFF).unwrap_or(0));
            let t = self.table.get(idx).copied().unwrap_or(0);
            c = t ^ (c >> 8);
        }
        self.state = c;
    }

    /// The checksum of everything fed so far (the hasher stays usable).
    #[must_use]
    pub fn finish(&self) -> u32 {
        self.state ^ 0xFFFF_FFFF
    }
}

/// One-shot CRC-32.
#[must_use]
pub fn crc32(data: &[u8]) -> u32 {
    let mut h = Crc32::new();
    h.update(data);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::{crc32, Crc32};

    /// Slow bitwise reference.
    fn reference(data: &[u8]) -> u32 {
        let mut c = 0xFFFF_FFFF_u32;
        for &b in data {
            c ^= u32::from(b);
            for _ in 0..8 {
                c = if c & 1 == 1 {
                    0xEDB8_8320 ^ (c >> 1)
                } else {
                    c >> 1
                };
            }
        }
        c ^ 0xFFFF_FFFF
    }

    // REQ: FOLIO-REQ-015
    #[test]
    fn known_answers() {
        assert_eq!(crc32(b""), 0);
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(
            crc32(b"The quick brown fox jumps over the lazy dog"),
            0x414F_A339
        );
        assert_eq!(crc32(&[0_u8; 32]), 0x190A_55AD);
    }

    // REQ: FOLIO-REQ-015
    #[test]
    fn streaming_split_matches_one_shot_at_every_position() {
        let data: Vec<u8> = (0..=255_u8).cycle().take(1000).collect();
        let whole = crc32(&data);
        for cut in 0..=data.len() {
            let mut h = Crc32::new();
            h.update(&data[..cut]);
            h.update(&data[cut..]);
            assert_eq!(h.finish(), whole);
        }
    }

    // REQ: FOLIO-REQ-015
    #[test]
    fn matches_bitwise_reference() {
        let data: Vec<u8> = (0..4096_u32)
            .map(|i| u8::try_from(i.wrapping_mul(31) & 0xFF).unwrap_or(0))
            .collect();
        assert_eq!(crc32(&data), reference(&data));
    }
}
