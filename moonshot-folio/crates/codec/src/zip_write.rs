// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Deterministic store-only ZIP32 writer.

use crate::crc32;
use crate::le::{put_u16, put_u32};
use crate::zip_fmt::{
    validate_name, DOS_DATE, DOS_TIME, FLAG_UTF8, METHOD_STORE, SIG_CENTRAL, SIG_EOCD, SIG_LOCAL,
    VERSION,
};
use moonshot_folio_core::{FolioError, Limits, Result};
use std::collections::BTreeSet;

/// One finished entry, kept for the central directory.
struct Record {
    name: String,
    crc: u32,
    size: u32,
    offset: u32,
    flags: u16,
}

/// Builds a ZIP archive in memory. Entries are stored (method 0), in insertion order, with a fixed
/// timestamp, so identical input always gives identical bytes.
pub struct ZipWriter {
    buf: Vec<u8>,
    records: Vec<Record>,
    names: BTreeSet<String>,
    limits: Limits,
    total: usize,
}

impl ZipWriter {
    /// An empty archive under `limits`.
    #[must_use]
    pub fn new(limits: Limits) -> Self {
        Self {
            buf: Vec::new(),
            records: Vec::new(),
            names: BTreeSet::new(),
            limits,
            total: 0,
        }
    }

    /// Append one entry.
    ///
    /// # Errors
    /// Invalid or duplicate name, or any [`Limits`] value exceeded (ZIP32 caps apply too).
    pub fn add(&mut self, name: &str, data: &[u8]) -> Result<()> {
        validate_name(name, self.limits.max_name_bytes())?;
        if self.names.contains(name) {
            return Err(FolioError::Invalid("duplicate entry name"));
        }
        self.check_room(data.len())?;
        let size = zip32(data.len(), "zip32 entry size")?;
        let offset = zip32(self.buf.len(), "zip32 offset")?;
        let flags = if name.is_ascii() { 0 } else { FLAG_UTF8 };
        let crc = crc32(data);
        write_local(&mut self.buf, name, flags, crc, size);
        self.buf.extend_from_slice(data);
        self.total = self.total.saturating_add(data.len());
        self.names.insert(name.to_string());
        self.records.push(Record {
            name: name.to_string(),
            crc,
            size,
            offset,
            flags,
        });
        Ok(())
    }

    /// Reject an entry that would break a count or size limit.
    fn check_room(&self, len: usize) -> Result<()> {
        if self.records.len() >= self.limits.max_zip_entries() || self.records.len() >= 65_534 {
            return Err(FolioError::Limit("zip entry count"));
        }
        if self.buf.len().saturating_add(len) > self.limits.max_file_bytes() {
            return Err(FolioError::Limit("zip file size"));
        }
        if len > self.limits.max_entry_bytes() {
            return Err(FolioError::Limit("zip entry size"));
        }
        if self.total.saturating_add(len) > self.limits.max_total_bytes() {
            return Err(FolioError::Limit("zip total size"));
        }
        Ok(())
    }

    /// Write the central directory and return the archive bytes.
    ///
    /// # Errors
    /// The finished archive exceeds ZIP32 or [`Limits`] size caps.
    pub fn finish(mut self) -> Result<Vec<u8>> {
        let cd_start = zip32(self.buf.len(), "zip32 offset")?;
        for r in &self.records {
            write_central(&mut self.buf, r);
        }
        let cd_len = self
            .buf
            .len()
            .saturating_sub(usize::try_from(cd_start).unwrap_or(0));
        let cd_size = zip32(cd_len, "zip32 directory size")?;
        let count =
            u16::try_from(self.records.len()).map_err(|_| FolioError::Limit("zip entry count"))?;
        write_eocd(&mut self.buf, count, cd_size, cd_start);
        if self.buf.len() > self.limits.max_file_bytes() {
            return Err(FolioError::Limit("zip file size"));
        }
        Ok(self.buf)
    }
}

/// A value that fits ZIP32 and is not the 0xFFFFFFFF sentinel the reader treats as zip64.
fn zip32(v: usize, what: &'static str) -> Result<u32> {
    u32::try_from(v)
        .ok()
        .filter(|x| *x != u32::MAX)
        .ok_or(FolioError::Limit(what))
}

/// Local file header plus name.
fn write_local(out: &mut Vec<u8>, name: &str, flags: u16, crc: u32, size: u32) {
    put_u32(out, SIG_LOCAL);
    put_u16(out, VERSION);
    put_u16(out, flags);
    put_u16(out, METHOD_STORE);
    put_u16(out, DOS_TIME);
    put_u16(out, DOS_DATE);
    put_u32(out, crc);
    put_u32(out, size);
    put_u32(out, size);
    put_u16(out, u16::try_from(name.len()).unwrap_or(u16::MAX));
    put_u16(out, 0);
    out.extend_from_slice(name.as_bytes());
}

/// Central directory header plus name.
fn write_central(out: &mut Vec<u8>, r: &Record) {
    put_u32(out, SIG_CENTRAL);
    put_u16(out, VERSION);
    put_u16(out, VERSION);
    put_u16(out, r.flags);
    put_u16(out, METHOD_STORE);
    put_u16(out, DOS_TIME);
    put_u16(out, DOS_DATE);
    put_u32(out, r.crc);
    put_u32(out, r.size);
    put_u32(out, r.size);
    put_u16(out, u16::try_from(r.name.len()).unwrap_or(u16::MAX));
    put_u16(out, 0);
    put_u16(out, 0);
    put_u16(out, 0);
    put_u16(out, 0);
    put_u32(out, 0);
    put_u32(out, r.offset);
    out.extend_from_slice(r.name.as_bytes());
}

/// End-of-central-directory record with no comment.
fn write_eocd(out: &mut Vec<u8>, count: u16, cd_size: u32, cd_start: u32) {
    put_u32(out, SIG_EOCD);
    put_u16(out, 0);
    put_u16(out, 0);
    put_u16(out, count);
    put_u16(out, count);
    put_u32(out, cd_size);
    put_u32(out, cd_start);
    put_u16(out, 0);
}
