// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Read-side helpers for the append-only ledger CSV.

use std::path::Path;

use crate::{Event, LedgerError};

/// Return the last `n` events from the ledger at `path`.
///
/// Reads the entire file into memory and returns the trailing slice. If the
/// file contains fewer than `n` rows, all rows are returned. If `n` is zero,
/// returns an empty `Vec`.
///
/// # Errors
///
/// Returns [`LedgerError::Io`] if the file cannot be opened, or
/// [`LedgerError::Csv`] if any row fails to deserialise.
pub fn tail(path: &Path, n: usize) -> Result<Vec<Event>, LedgerError> {
    if n == 0 {
        return Ok(Vec::new());
    }

    let mut rdr = csv::Reader::from_path(path)?;
    let all: Result<Vec<Event>, _> = rdr.deserialize().collect();
    let all = all?;

    let start = all.len().saturating_sub(n);
    Ok(all[start..].to_vec())
}
