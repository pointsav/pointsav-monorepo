// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! ZIP constants shared by the writer and the reader (ZIP32, store-only, no extras).

use moonshot_folio_core::{FolioError, Result};

pub(crate) const SIG_LOCAL: u32 = 0x0403_4B50;
pub(crate) const SIG_CENTRAL: u32 = 0x0201_4B50;
pub(crate) const SIG_EOCD: u32 = 0x0605_4B50;
pub(crate) const LOCAL_LEN: usize = 30;
pub(crate) const CENTRAL_LEN: usize = 46;
pub(crate) const EOCD_LEN: usize = 22;
pub(crate) const VERSION: u16 = 20;
/// 1980-01-01, the DOS epoch; every entry gets this so output is deterministic.
pub(crate) const DOS_DATE: u16 = 0x0021;
pub(crate) const DOS_TIME: u16 = 0;
pub(crate) const FLAG_ENCRYPTED: u16 = 0x0001;
pub(crate) const FLAG_UTF8: u16 = 0x0800;
pub(crate) const METHOD_STORE: u16 = 0;
pub(crate) const METHOD_DEFLATE: u16 = 8;

/// Accept only relative, forward-slash names with no empty, `.` or `..` segments.
pub(crate) fn validate_name(name: &str, max_bytes: usize) -> Result<()> {
    if name.is_empty() {
        return Err(FolioError::Invalid("empty entry name"));
    }
    if name.len() > max_bytes || name.len() > usize::from(u16::MAX) {
        return Err(FolioError::Limit("entry name length"));
    }
    let bytes = name.as_bytes();
    let drive = bytes.get(1) == Some(&b':') && bytes.first().is_some_and(u8::is_ascii_alphabetic);
    if drive || name.chars().any(char::is_control) {
        return Err(FolioError::Invalid("unsafe entry name"));
    }
    if name.contains(['\\', '\0']) || name.starts_with('/') {
        return Err(FolioError::Invalid("unsafe entry name"));
    }
    let trimmed = name.strip_suffix('/').unwrap_or(name);
    if trimmed
        .split('/')
        .any(|seg| seg.is_empty() || seg == "." || seg == "..")
    {
        return Err(FolioError::Invalid("unsafe entry name"));
    }
    Ok(())
}
