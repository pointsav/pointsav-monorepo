// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Adler-32 (RFC 1950): two sums modulo 65521, initial value 1.

/// Largest prime below 2^16.
const MOD: u32 = 65_521;
/// Longest run of bytes after which the sums cannot overflow `u32`.
const NMAX: usize = 5552;

/// Streaming Adler-32 hasher.
#[derive(Clone, Copy, Debug)]
pub struct Adler32 {
    a: u32,
    b: u32,
}

impl Default for Adler32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Adler32 {
    /// A fresh hasher (checksum 1).
    #[must_use]
    pub fn new() -> Self {
        Self { a: 1, b: 0 }
    }

    /// Feed bytes.
    pub fn update(&mut self, data: &[u8]) {
        for chunk in data.chunks(NMAX) {
            for &x in chunk {
                self.a = self.a.wrapping_add(u32::from(x));
                self.b = self.b.wrapping_add(self.a);
            }
            self.a %= MOD;
            self.b %= MOD;
        }
    }

    /// The checksum of everything fed so far (the hasher stays usable).
    #[must_use]
    pub fn finish(&self) -> u32 {
        (self.b << 16) | self.a
    }
}

/// One-shot Adler-32.
#[must_use]
pub fn adler32(data: &[u8]) -> u32 {
    let mut h = Adler32::new();
    h.update(data);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::{adler32, Adler32};

    /// Slow per-byte reference.
    fn reference(data: &[u8]) -> u32 {
        let (mut a, mut b) = (1_u32, 0_u32);
        for &x in data {
            a = a.wrapping_add(u32::from(x)) % 65_521;
            b = b.wrapping_add(a) % 65_521;
        }
        (b << 16) | a
    }

    // REQ: FOLIO-REQ-015
    #[test]
    fn known_answers() {
        assert_eq!(adler32(b""), 1);
        assert_eq!(adler32(b"Wikipedia"), 0x11E6_0398);
        assert_eq!(adler32(b"123456789"), 0x091E_01DE);
    }

    // REQ: FOLIO-REQ-015
    #[test]
    fn long_inputs_cross_the_nmax_boundary() {
        for len in [5551_usize, 5552, 5553, 11_104, 100_000] {
            let data = vec![0xFF_u8; len];
            assert_eq!(adler32(&data), reference(&data));
        }
    }

    // REQ: FOLIO-REQ-015
    #[test]
    fn streaming_split_matches_one_shot() {
        let data: Vec<u8> = (0..=255_u8).cycle().take(12_000).collect();
        let whole = adler32(&data);
        for cut in [0_usize, 1, 5551, 5552, 5553, 11_999, 12_000] {
            let mut h = Adler32::new();
            h.update(&data[..cut]);
            h.update(&data[cut..]);
            assert_eq!(h.finish(), whole);
        }
    }
}
