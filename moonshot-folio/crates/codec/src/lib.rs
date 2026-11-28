// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! moonshot-folio-codec (L0): CRC32, Adler32, ZIP, inflate, SHA-256, PNG/JPEG.
//!
//! Design: BRIEF-moonshot-folio-architecture. Implemented so far: CRC-32 and Adler-32 (F1.1), ZIP store-only writer and reader (F1.2), inflate and zlib (F1.3), SHA-256 (F1.4), JPEG header and PNG decode (F1.7).

#![forbid(unsafe_code)]

mod adler32;
mod crc32;

pub use adler32::{adler32, Adler32};
pub use crc32::{crc32, Crc32};
mod bits;
mod huffman;
mod inflate;
mod jpeg;
mod le;
mod png;
mod png_filter;
mod sha256;
mod zip_fmt;
mod zip_read;
mod zip_write;

pub use inflate::{inflate, zlib_decompress};
pub use jpeg::{parse_jpeg_header, JpegInfo};
pub use png::{decode_png, PngColor, PngImage};
pub use sha256::{sha256, sha256_hex, Sha256};
pub use zip_read::{ZipArchive, ZipEntry};
pub use zip_write::ZipWriter;
