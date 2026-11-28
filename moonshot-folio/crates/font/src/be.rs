// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Bounds-checked big-endian reads (sfnt data is big-endian).

use moonshot_folio_core::{FolioError, Result};

/// `len` bytes of `d` starting at `off`, or an error.
pub(crate) fn slice(d: &[u8], off: usize, len: usize) -> Result<&[u8]> {
    let end = off
        .checked_add(len)
        .ok_or(FolioError::Malformed("offset overflow"))?;
    d.get(off..end)
        .ok_or(FolioError::Malformed("read past end of font data"))
}

/// Big-endian `u16` at `off`.
pub(crate) fn u16_at(d: &[u8], off: usize) -> Result<u16> {
    let a =
        <[u8; 2]>::try_from(slice(d, off, 2)?).map_err(|_| FolioError::Malformed("short read"))?;
    Ok(u16::from_be_bytes(a))
}

/// Big-endian `i16` at `off`.
pub(crate) fn i16_at(d: &[u8], off: usize) -> Result<i16> {
    let a =
        <[u8; 2]>::try_from(slice(d, off, 2)?).map_err(|_| FolioError::Malformed("short read"))?;
    Ok(i16::from_be_bytes(a))
}

/// Big-endian `u32` at `off`.
pub(crate) fn u32_at(d: &[u8], off: usize) -> Result<u32> {
    let a =
        <[u8; 4]>::try_from(slice(d, off, 4)?).map_err(|_| FolioError::Malformed("short read"))?;
    Ok(u32::from_be_bytes(a))
}

/// Checked addition.
pub(crate) fn add(a: usize, b: usize) -> Result<usize> {
    a.checked_add(b)
        .ok_or(FolioError::Malformed("length overflow"))
}

/// Checked multiplication.
pub(crate) fn mul(a: usize, b: usize) -> Result<usize> {
    a.checked_mul(b)
        .ok_or(FolioError::Malformed("length overflow"))
}

/// Widen a `u32`.
pub(crate) fn to_usize(v: u32) -> Result<usize> {
    usize::try_from(v).map_err(|_| FolioError::Limit("value exceeds address space"))
}
