// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Bounds-checked little-endian reads and writes.

use moonshot_folio_core::{FolioError, Result};

/// `len` bytes of `d` starting at `off`, or an error.
pub(crate) fn slice(d: &[u8], off: usize, len: usize) -> Result<&[u8]> {
    let end = off
        .checked_add(len)
        .ok_or(FolioError::Malformed("offset overflow"))?;
    d.get(off..end)
        .ok_or(FolioError::Malformed("read past end of data"))
}

/// Little-endian `u16` at `off`.
pub(crate) fn u16_at(d: &[u8], off: usize) -> Result<u16> {
    let a =
        <[u8; 2]>::try_from(slice(d, off, 2)?).map_err(|_| FolioError::Malformed("short read"))?;
    Ok(u16::from_le_bytes(a))
}

/// Little-endian `u32` at `off`.
pub(crate) fn u32_at(d: &[u8], off: usize) -> Result<u32> {
    let a =
        <[u8; 4]>::try_from(slice(d, off, 4)?).map_err(|_| FolioError::Malformed("short read"))?;
    Ok(u32::from_le_bytes(a))
}

/// Big-endian `u16` at `off` (JPEG).
pub(crate) fn be16_at(d: &[u8], off: usize) -> Result<u16> {
    let a =
        <[u8; 2]>::try_from(slice(d, off, 2)?).map_err(|_| FolioError::Malformed("short read"))?;
    Ok(u16::from_be_bytes(a))
}

/// Big-endian `u32` at `off` (PNG).
pub(crate) fn be32_at(d: &[u8], off: usize) -> Result<u32> {
    let a =
        <[u8; 4]>::try_from(slice(d, off, 4)?).map_err(|_| FolioError::Malformed("short read"))?;
    Ok(u32::from_be_bytes(a))
}

/// Checked addition of two offsets or lengths.
pub(crate) fn add(a: usize, b: usize) -> Result<usize> {
    a.checked_add(b)
        .ok_or(FolioError::Malformed("length overflow"))
}

/// Widen a `u32` to `usize`.
pub(crate) fn to_usize(v: u32) -> Result<usize> {
    usize::try_from(v).map_err(|_| FolioError::Limit("value exceeds address space"))
}

/// Append a little-endian `u16`.
pub(crate) fn put_u16(out: &mut Vec<u8>, v: u16) {
    out.extend_from_slice(&v.to_le_bytes());
}

/// Append a little-endian `u32`.
pub(crate) fn put_u32(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_le_bytes());
}
