// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Bounded ZIP reader (ZIP32, stored entries). Every read is bounds-checked; malformed input
//! returns an error and never panics.

use crate::le::{add, slice, to_usize, u16_at, u32_at};
use crate::zip_fmt::{
    validate_name, CENTRAL_LEN, EOCD_LEN, FLAG_ENCRYPTED, LOCAL_LEN, METHOD_DEFLATE, METHOD_STORE,
    SIG_CENTRAL, SIG_EOCD, SIG_LOCAL,
};
use crate::{crc32, inflate};
use moonshot_folio_core::{FolioError, Limits, Result};
use std::collections::BTreeSet;

/// Longest comment a ZIP end record can carry.
const MAX_COMMENT: usize = 65_535;

/// One archive member.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZipEntry {
    name: String,
    crc: u32,
    size: usize,
    stored_len: usize,
    method: u16,
    local_offset: usize,
    data_offset: usize,
}

impl ZipEntry {
    /// The entry name (validated: relative, forward slashes, no `..`).
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Uncompressed size in bytes.
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }
}

/// A parsed archive borrowing its bytes.
pub struct ZipArchive<'a> {
    data: &'a [u8],
    entries: Vec<ZipEntry>,
    limits: Limits,
}

/// Fields of the end-of-central-directory record that the walk needs.
struct Eocd {
    count: usize,
    cd_offset: usize,
    cd_size: usize,
}

impl<'a> ZipArchive<'a> {
    /// Parse the central directory of `data`. Entry bytes are checked on [`Self::read`].
    ///
    /// The end record is the last signature whose comment length fits the file exactly, so an
    /// archive comment that itself contains a well-formed end record is read as the archive (the
    /// same behaviour as common tools). Any byte after the end record makes the file unparseable.
    ///
    /// # Errors
    /// Malformed input, an unsupported feature, or a [`Limits`] value exceeded.
    pub fn parse(data: &'a [u8], limits: &Limits) -> Result<Self> {
        if data.len() > limits.max_file_bytes() {
            return Err(FolioError::Limit("zip file size"));
        }
        let eocd = find_eocd(data, limits)?;
        let mut entries = Vec::new();
        let mut names = BTreeSet::new();
        let mut cursor = eocd.cd_offset;
        let mut total = 0_usize;
        for _ in 0..eocd.count {
            let (entry, next) = parse_central(data, cursor, eocd.cd_offset, limits)?;
            total = add(total, entry.size)?;
            if total > limits.max_total_bytes() {
                return Err(FolioError::Limit("zip total size"));
            }
            if !names.insert(entry.name.clone()) {
                return Err(FolioError::Malformed("duplicate entry name"));
            }
            entries.push(entry);
            cursor = next;
        }
        if cursor != add(eocd.cd_offset, eocd.cd_size)? {
            return Err(FolioError::Malformed("central directory size mismatch"));
        }
        check_no_overlap(&entries)?;
        Ok(Self {
            data,
            entries,
            limits: *limits,
        })
    }

    /// All entries in directory order.
    #[must_use]
    pub fn entries(&self) -> &[ZipEntry] {
        &self.entries
    }

    /// The entry called `name`, if present.
    #[must_use]
    pub fn find(&self, name: &str) -> Option<&ZipEntry> {
        self.entries.iter().find(|e| e.name == name)
    }

    /// The bytes of `entry`, with the CRC-32 checked.
    ///
    /// # Errors
    /// The entry exceeds [`Limits`], lies outside the data, or fails its checksum.
    pub fn read(&self, entry: &ZipEntry) -> Result<Vec<u8>> {
        if entry.size > self.limits.max_entry_bytes() {
            return Err(FolioError::Limit("zip entry size"));
        }
        let raw = slice(self.data, entry.data_offset, entry.stored_len)?;
        let bytes = if entry.method == METHOD_DEFLATE {
            let out = inflate(raw, entry.size).map_err(|e| match e {
                FolioError::Limit(_) => {
                    FolioError::Malformed("deflate output exceeds declared size")
                }
                other => other,
            })?;
            if out.len() != entry.size {
                return Err(FolioError::Malformed(
                    "deflate output shorter than declared size",
                ));
            }
            out
        } else {
            raw.to_vec()
        };
        if crc32(&bytes) != entry.crc {
            return Err(FolioError::Malformed("crc mismatch"));
        }
        Ok(bytes)
    }
}

/// Entries (local header through data) may not overlap one another.
fn check_no_overlap(entries: &[ZipEntry]) -> Result<()> {
    let mut spans: Vec<(usize, usize)> = entries
        .iter()
        .map(|e| (e.local_offset, e.data_offset.saturating_add(e.stored_len)))
        .collect();
    spans.sort_unstable();
    for pair in spans.windows(2) {
        if let [a, b] = pair {
            if a.1 > b.0 {
                return Err(FolioError::Malformed("overlapping entries"));
            }
        }
    }
    Ok(())
}

/// Locate and decode the end-of-central-directory record.
fn find_eocd(data: &[u8], limits: &Limits) -> Result<Eocd> {
    let last = data
        .len()
        .checked_sub(EOCD_LEN)
        .ok_or(FolioError::Malformed("too short for a zip"))?;
    let floor = last.saturating_sub(MAX_COMMENT);
    let mut pos = last;
    loop {
        if u32_at(data, pos).is_ok_and(|s| s == SIG_EOCD) && comment_fits(data, pos) {
            return decode_eocd(data, pos, limits);
        }
        if pos == floor {
            return Err(FolioError::Malformed("end of central directory not found"));
        }
        pos = pos.saturating_sub(1);
    }
}

/// The comment length field must account for exactly the rest of the file.
fn comment_fits(data: &[u8], pos: usize) -> bool {
    let rest = data.len().saturating_sub(pos).saturating_sub(EOCD_LEN);
    u16_at(data, pos.saturating_add(20)).is_ok_and(|c| usize::from(c) == rest)
}

/// Validate the record's fields and the directory range.
fn decode_eocd(data: &[u8], pos: usize, limits: &Limits) -> Result<Eocd> {
    if u16_at(data, add(pos, 4)?)? != 0 || u16_at(data, add(pos, 6)?)? != 0 {
        return Err(FolioError::Unsupported("multi-disk archive"));
    }
    let on_disk = u16_at(data, add(pos, 8)?)?;
    let total = u16_at(data, add(pos, 10)?)?;
    let cd_size = u32_at(data, add(pos, 12)?)?;
    let cd_offset = u32_at(data, add(pos, 16)?)?;
    if total == u16::MAX || cd_size == u32::MAX || cd_offset == u32::MAX {
        return Err(FolioError::Unsupported("zip64"));
    }
    if on_disk != total {
        return Err(FolioError::Malformed("entry counts disagree"));
    }
    if usize::from(total) > limits.max_zip_entries() {
        return Err(FolioError::Limit("zip entry count"));
    }
    let (cd_size, cd_offset) = (to_usize(cd_size)?, to_usize(cd_offset)?);
    if add(cd_offset, cd_size)? > pos {
        return Err(FolioError::Malformed("central directory outside file"));
    }
    Ok(Eocd {
        count: usize::from(total),
        cd_offset,
        cd_size,
    })
}

/// Decode one central header at `at`; returns the entry and the next header's offset.
fn parse_central(
    data: &[u8],
    at: usize,
    data_end: usize,
    limits: &Limits,
) -> Result<(ZipEntry, usize)> {
    if u32_at(data, at)? != SIG_CENTRAL {
        return Err(FolioError::Malformed("bad central header signature"));
    }
    let flags = u16_at(data, add(at, 8)?)?;
    let method = u16_at(data, add(at, 10)?)?;
    let crc = u32_at(data, add(at, 16)?)?;
    let (csize, size) = (u32_at(data, add(at, 20)?)?, u32_at(data, add(at, 24)?)?);
    let name_len = usize::from(u16_at(data, add(at, 28)?)?);
    let skip = add(
        usize::from(u16_at(data, add(at, 30)?)?),
        usize::from(u16_at(data, add(at, 32)?)?),
    )?;
    let local = u32_at(data, add(at, 42)?)?;
    if flags & FLAG_ENCRYPTED != 0 {
        return Err(FolioError::Unsupported("encrypted entry"));
    }
    if method != METHOD_STORE && method != METHOD_DEFLATE {
        return Err(FolioError::Unsupported("compression method"));
    }
    if csize == u32::MAX || size == u32::MAX || local == u32::MAX {
        return Err(FolioError::Unsupported("zip64"));
    }
    if method == METHOD_STORE && csize != size {
        return Err(FolioError::Malformed("stored entry sizes differ"));
    }
    let name = decode_name(slice(data, add(at, CENTRAL_LEN)?, name_len)?, limits)?;
    let (size, stored_len) = (to_usize(size)?, to_usize(csize)?);
    if size > limits.max_entry_bytes() {
        return Err(FolioError::Limit("zip entry size"));
    }
    let local_offset = to_usize(local)?;
    let data_offset = local_data_offset(data, local_offset, &name, stored_len, data_end)?;
    let next = add(add(at, CENTRAL_LEN)?, add(name_len, skip)?)?;
    Ok((
        ZipEntry {
            name,
            crc,
            size,
            stored_len,
            method,
            local_offset,
            data_offset,
        },
        next,
    ))
}

/// UTF-8 decode and validate an entry name.
fn decode_name(bytes: &[u8], limits: &Limits) -> Result<String> {
    let name =
        std::str::from_utf8(bytes).map_err(|_| FolioError::Malformed("entry name is not UTF-8"))?;
    validate_name(name, limits.max_name_bytes()).map_err(|e| match e {
        FolioError::Limit(w) => FolioError::Limit(w),
        _ => FolioError::Malformed("unsafe entry name"),
    })?;
    Ok(name.to_string())
}

/// Check the local header agrees with the central one and return where the data starts.
fn local_data_offset(
    data: &[u8],
    local: usize,
    name: &str,
    stored: usize,
    data_end: usize,
) -> Result<usize> {
    if u32_at(data, local)? != SIG_LOCAL {
        return Err(FolioError::Malformed("bad local header signature"));
    }
    let name_len = usize::from(u16_at(data, add(local, 26)?)?);
    let extra_len = usize::from(u16_at(data, add(local, 28)?)?);
    if slice(data, add(local, LOCAL_LEN)?, name_len)? != name.as_bytes() {
        return Err(FolioError::Malformed("local and central names differ"));
    }
    let start = add(add(local, LOCAL_LEN)?, add(name_len, extra_len)?)?;
    if add(start, stored)? > data_end {
        return Err(FolioError::Malformed(
            "entry data overlaps the central directory",
        ));
    }
    Ok(start)
}
