// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Error types for the ledger writer.

use thiserror::Error;

/// Errors produced by [`LedgerWriter`](crate::LedgerWriter).
#[derive(Debug, Error)]
pub enum LedgerError {
    /// A file-system operation failed.
    ///
    /// Covers file open, flush, and `fsync`. The underlying [`std::io::Error`]
    /// carries the OS error code.
    #[error("ledger I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// CSV serialisation of an [`Event`](crate::Event) failed.
    ///
    /// This indicates a bug — every well-formed `Event` should serialise
    /// without error. Log the event and the error for investigation.
    #[error("ledger CSV error: {0}")]
    Csv(#[from] csv::Error),
}
