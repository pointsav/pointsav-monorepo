// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tenant identifier — the load-bearing multi-tenant primitive
//! (`ARCHITECTURE.md` §4).

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::CoreError;

/// Tenant moduleId. Lowercase ASCII letters, digits, and hyphens; 1..=64
/// characters. Used as a routing key at every layer (audit ledger, KV cache
/// namespace, LoRA adapter selection, graph partition).
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(s: impl Into<String>) -> Result<Self, CoreError> {
        let s = s.into();
        if s.is_empty() || s.len() > 64 {
            return Err(CoreError::InvalidModuleId {
                value: s,
                reason: "length must be 1..=64",
            });
        }
        if !s
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Err(CoreError::InvalidModuleId {
                value: s,
                reason: "only [a-z0-9-] permitted",
            });
        }
        Ok(Self(s))
    }

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
    type Err = CoreError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_canonical_examples() {
        assert!(ModuleId::new("foundry").is_ok());
        assert!(ModuleId::new("woodfine-mirror").is_ok());
    }

    #[test]
    fn rejects_uppercase_and_underscores() {
        assert!(ModuleId::new("Foundry").is_err());
        assert!(ModuleId::new("woodfine_mirror").is_err());
    }

    #[test]
    fn rejects_empty_and_oversize() {
        assert!(ModuleId::new("").is_err());
        assert!(ModuleId::new("a".repeat(65)).is_err());
    }
}
