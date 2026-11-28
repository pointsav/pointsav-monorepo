// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Error type shared by the whole family.

use core::fmt;

/// Why an operation failed. Untrusted input always ends in one of these, never a panic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FolioError {
    /// A configured [`Limits`](crate::Limits) value was exceeded.
    Limit(&'static str),
    /// The input is structurally invalid.
    Malformed(&'static str),
    /// The input is valid but uses a feature this build does not support.
    Unsupported(&'static str),
    /// The caller supplied an invalid argument.
    Invalid(&'static str),
}

impl fmt::Display for FolioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Limit(w) => write!(f, "limit exceeded: {w}"),
            Self::Malformed(w) => write!(f, "malformed input: {w}"),
            Self::Unsupported(w) => write!(f, "unsupported: {w}"),
            Self::Invalid(w) => write!(f, "invalid argument: {w}"),
        }
    }
}

impl std::error::Error for FolioError {}

/// Result alias used across the family.
pub type Result<T> = core::result::Result<T, FolioError>;
