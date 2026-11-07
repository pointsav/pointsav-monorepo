// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! The shared error type for tool-precedent-* crates. Zero dependencies.
//! The original six shard modules still return `String` errors; they convert with `From<String>`.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// A span or edit range falls outside the source, or is inverted.
    OutOfRange {
        start: usize,
        end: usize,
        len: usize,
    },
    /// A span or edit boundary falls inside a multi-byte UTF-8 character.
    NotCharBoundary(usize),
    /// A path resolves outside the configured root.
    OutsideRoot(String),
    /// The file changed on disk since it was read.
    Stale(String),
    /// The target is structured data that AI-originated edits may not touch (SYS-ADR-07).
    StructuredBlocked(String),
    /// No pending proposal with this id.
    NoProposal(u64),
    Io(String),
    Other(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::OutOfRange { start, end, len } => {
                write!(f, "range {start}..{end} outside source of {len} bytes")
            }
            CoreError::NotCharBoundary(i) => write!(f, "offset {i} is not on a character boundary"),
            CoreError::OutsideRoot(p) => write!(f, "path escapes root: {p}"),
            CoreError::Stale(p) => write!(f, "file changed since read: {p}"),
            CoreError::StructuredBlocked(p) => write!(
                f,
                "structured data may not be edited by AI (SYS-ADR-07): {p}"
            ),
            CoreError::NoProposal(id) => write!(f, "no pending proposal {id}"),
            CoreError::Io(m) | CoreError::Other(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for CoreError {}

impl From<String> for CoreError {
    fn from(m: String) -> Self {
        CoreError::Other(m)
    }
}

impl From<std::io::Error> for CoreError {
    fn from(e: std::io::Error) -> Self {
        CoreError::Io(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_and_converts() {
        let e: CoreError = String::from("boom").into();
        assert_eq!(e.to_string(), "boom");
        assert!(CoreError::OutOfRange {
            start: 3,
            end: 9,
            len: 5
        }
        .to_string()
        .contains("3..9"));
        assert!(CoreError::StructuredBlocked("a.json".into())
            .to_string()
            .contains("SYS-ADR-07"));
    }
}
