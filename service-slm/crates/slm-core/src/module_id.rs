// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! The [`ModuleId`] newtype and its validation rules.
//!
//! `moduleId` namespaces a project across all five service-slm layers
//! (bootstrap, KV cache, graph, adapters, ledger). See
//! [YOYO-COMPUTE §6](../../../specs/YOYO-COMPUTE.md) for the discipline this
//! type enforces.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Maximum length of a `ModuleId`, in bytes.
///
/// Matches the DNS label bound (RFC 1035 §2.3.4) so a `ModuleId` is safe as a
/// filesystem path component, URL path segment, and cache namespace key.
pub const MAX_LEN: usize = 63;

/// A validated project-namespace identifier.
///
/// Threaded through every call into service-slm. Constructed only via
/// [`ModuleId::new`] or [`FromStr`]; deserialisation runs the same validation,
/// so an invalid `ModuleId` value cannot exist at runtime.
///
/// # Grammar
///
/// ```text
/// ModuleId := [a-z0-9] ([a-z0-9-]{0,61} [a-z0-9])?
/// ```
///
/// Length 1..=63. Lowercase ASCII letters, digits, and internal hyphens only.
/// No leading or trailing hyphen.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleId(String);

impl ModuleId {
    /// Validates `raw` and constructs a `ModuleId`.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleIdError`] if `raw` violates the grammar above.
    pub fn new(raw: &str) -> Result<Self, ModuleIdError> {
        if raw.is_empty() {
            return Err(ModuleIdError::Empty);
        }
        if raw.len() > MAX_LEN {
            return Err(ModuleIdError::TooLong {
                actual: raw.len(),
                max: MAX_LEN,
            });
        }
        let bytes = raw.as_bytes();
        if bytes[0] == b'-' || bytes[bytes.len() - 1] == b'-' {
            return Err(ModuleIdError::LeadingOrTrailingHyphen);
        }
        for (position, &byte) in bytes.iter().enumerate() {
            let ok = byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-';
            if !ok {
                return Err(ModuleIdError::InvalidChar {
                    character: byte as char,
                    position,
                });
            }
        }
        Ok(Self(raw.to_owned()))
    }

    /// Returns the underlying string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for ModuleId {
    type Err = ModuleIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl AsRef<str> for ModuleId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Serialize for ModuleId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ModuleId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::new(&raw).map_err(serde::de::Error::custom)
    }
}

/// Validation errors produced by [`ModuleId::new`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ModuleIdError {
    /// The input was the empty string.
    #[error("moduleId must not be empty")]
    Empty,

    /// The input exceeded [`MAX_LEN`] bytes.
    #[error("moduleId is {actual} bytes; maximum is {max}")]
    TooLong {
        /// Actual length in bytes.
        actual: usize,
        /// Maximum permitted length in bytes.
        max: usize,
    },

    /// The input started or ended with `-`.
    #[error("moduleId must not start or end with '-'")]
    LeadingOrTrailingHyphen,

    /// The input contained a byte outside `[a-z0-9-]`.
    #[error("moduleId contains invalid character {character:?} at position {position}")]
    InvalidChar {
        /// The offending byte, interpreted as a `char`.
        character: char,
        /// Zero-based byte position of the offending byte.
        position: usize,
    },
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn accepts_values_present_in_specs() {
        for value in [
            "woodfine-v1",
            "dka-coa",
            "dka-archetype",
            "woodfine-entity",
            "a",
            "0",
        ] {
            ModuleId::new(value)
                .unwrap_or_else(|e| panic!("expected {value:?} to be valid, got {e}"));
        }
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(ModuleId::new(""), Err(ModuleIdError::Empty));
    }

    #[test]
    fn rejects_uppercase() {
        match ModuleId::new("Woodfine-v1") {
            Err(ModuleIdError::InvalidChar {
                character: 'W',
                position: 0,
            }) => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn rejects_leading_hyphen() {
        assert_eq!(
            ModuleId::new("-woodfine"),
            Err(ModuleIdError::LeadingOrTrailingHyphen)
        );
    }

    #[test]
    fn rejects_trailing_hyphen() {
        assert_eq!(
            ModuleId::new("woodfine-"),
            Err(ModuleIdError::LeadingOrTrailingHyphen)
        );
    }

    #[test]
    fn rejects_whitespace() {
        match ModuleId::new("wood fine") {
            Err(ModuleIdError::InvalidChar {
                character: ' ',
                position: 4,
            }) => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn rejects_over_max_len() {
        let raw = "a".repeat(MAX_LEN + 1);
        assert_eq!(
            ModuleId::new(&raw),
            Err(ModuleIdError::TooLong {
                actual: MAX_LEN + 1,
                max: MAX_LEN,
            })
        );
    }

    #[test]
    fn accepts_exactly_max_len() {
        let raw = "a".repeat(MAX_LEN);
        assert!(ModuleId::new(&raw).is_ok());
    }

    /// Generates strings that satisfy the `ModuleId` grammar.
    fn valid_module_id() -> impl Strategy<Value = String> {
        proptest::collection::vec(
            prop_oneof![Just(b'-'), (b'a'..=b'z'), (b'0'..=b'9'),],
            1..=MAX_LEN,
        )
        .prop_map(|mut bytes| {
            if bytes[0] == b'-' {
                bytes[0] = b'a';
            }
            let last = bytes.len() - 1;
            if bytes[last] == b'-' {
                bytes[last] = b'a';
            }
            String::from_utf8(bytes).expect("ascii by construction")
        })
    }

    proptest! {
        #[test]
        fn new_accepts_every_valid_grammar_string(raw in valid_module_id()) {
            let id = ModuleId::new(&raw).expect("valid by construction");
            prop_assert_eq!(id.as_str(), raw);
        }

        #[test]
        fn display_round_trips_through_from_str(raw in valid_module_id()) {
            let id = ModuleId::new(&raw).unwrap();
            let displayed = id.to_string();
            let parsed: ModuleId = displayed.parse().unwrap();
            prop_assert_eq!(parsed, id);
        }

        #[test]
        fn serde_json_round_trip(raw in valid_module_id()) {
            let id = ModuleId::new(&raw).unwrap();
            let json = serde_json::to_string(&id).unwrap();
            let back: ModuleId = serde_json::from_str(&json).unwrap();
            prop_assert_eq!(back, id);
        }

        #[test]
        fn deserialise_rejects_invalid(raw in "\\PC*") {
            let is_valid = ModuleId::new(&raw).is_ok();
            let via_json = serde_json::from_value::<ModuleId>(
                serde_json::Value::String(raw.clone())
            );
            prop_assert_eq!(via_json.is_ok(), is_valid);
        }
    }
}
