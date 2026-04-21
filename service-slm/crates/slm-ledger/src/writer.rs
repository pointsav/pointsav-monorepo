// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Append-only CSV writer with `fsync` durability semantics.
//!
//! [`LedgerWriter`] is the only path through which rows enter the ledger file.
//! Every successful [`append`](LedgerWriter::append) call guarantees the row is
//! on durable storage before returning: it serialises the row, flushes the
//! csv crate's internal buffer to the OS, then calls `fsync(2)` via
//! [`File::sync_all`](std::fs::File::sync_all). A process crash after
//! `append` returns cannot lose the row.

use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use crate::error::LedgerError;
use crate::event::Event;

/// Append-only writer for the yo-yo compute audit ledger.
///
/// Opens or creates the ledger file in append mode. If the file is new
/// (length zero), the CSV header is written automatically on the first
/// [`append`](Self::append) call. If the file already contains rows, the
/// header is suppressed so that reopening an existing ledger does not
/// corrupt it with a duplicate header.
///
/// ## Durability guarantee
///
/// After [`append`](Self::append) returns `Ok`, the row is on durable
/// storage. The sequence on every write is:
///
/// 1. `csv::Writer::serialize` — encode the row into the csv buffer
/// 2. `csv::Writer::flush` — write the buffer to the `File` (OS page cache)
/// 3. `File::sync_all` — `fsync(2)`, waits for disk acknowledgement
///
/// ## Append-only invariant
///
/// The file is opened with `O_APPEND`. There is no method to seek, truncate,
/// or overwrite. Corrections to ledger rows are new rows referencing the
/// corrected `event_id`, per the YOYO-COMPUTE §5 schema.
pub struct LedgerWriter {
    inner: csv::Writer<File>,
    path: PathBuf,
}

impl LedgerWriter {
    /// Opens or creates the ledger file at `path` for appending.
    ///
    /// If the file does not exist it is created. If it already exists its
    /// contents are preserved and new rows are appended after the last
    /// existing row.
    ///
    /// # Errors
    ///
    /// Returns [`LedgerError::Io`] if the file cannot be opened or its
    /// metadata cannot be read.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, LedgerError> {
        let path = path.as_ref();

        let file = OpenOptions::new().create(true).append(true).open(path)?;

        // Determine whether to write a header. A non-empty file already has
        // one; writing it again would corrupt every existing row's column
        // alignment for any reader that follows the spec header strictly.
        let is_new = file.metadata()?.len() == 0;

        let inner = csv::WriterBuilder::new()
            .has_headers(is_new)
            .from_writer(file);

        Ok(Self {
            inner,
            path: path.to_owned(),
        })
    }

    /// Appends one [`Event`] row to the ledger and fsyncs before returning.
    ///
    /// The row is durable on disk when this method returns `Ok`. If any step
    /// — serialisation, flush, or fsync — fails, the error is returned and
    /// the caller should treat the row as unwritten.
    ///
    /// # Errors
    ///
    /// Returns [`LedgerError::Csv`] if serialisation fails or
    /// [`LedgerError::Io`] if the flush or fsync fails.
    pub fn append(&mut self, event: &Event) -> Result<(), LedgerError> {
        self.inner.serialize(event)?;
        self.inner.flush()?;
        // sync_all() calls fsync(2). get_ref() returns &File directly because
        // csv::Writer<W> wraps W without an intermediate BufWriter.
        self.inner.get_ref().sync_all()?;
        Ok(())
    }

    /// Returns the path of the ledger file this writer is writing to.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}
