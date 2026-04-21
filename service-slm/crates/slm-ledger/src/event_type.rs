// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! The [`EventType`] enum and its ten variants.
//!
//! Wire strings are the SCREAMING\_SNAKE\_CASE names from YOYO-COMPUTE §5,
//! reproduced verbatim in the CSV `event_type` column so the exported ledger
//! is readable by any audit tool that follows the spec without translation.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The ten categories of yo-yo compute events recorded in the audit ledger.
///
/// Serialises to and from SCREAMING\_SNAKE\_CASE wire strings as defined in
/// YOYO-COMPUTE §5. The serde rename applies to both JSON and CSV paths, so
/// the same type is used throughout the codebase without conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    /// `SkyPilot` has been asked to spin up the GCP node.
    BootRequest,
    /// The GCP node is up and serving requests.
    BootComplete,
    /// An ingest or query job has been submitted to the node.
    JobStart,
    /// The job finished and the result delta has been returned.
    JobComplete,
    /// A GCS checkpoint has been written.
    Checkpoint,
    /// An explicit tear-down has been issued.
    TeardownRequest,
    /// The node is gone and the final cost has been recorded.
    TeardownComplete,
    /// The spot instance was preempted by the hyperscaler.
    Preemption,
    /// A `LoRA` adapter was activated for a request.
    AdapterLoad,
    /// Mooncake Store reconciliation event.
    KvPoolSync,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::BootRequest => "BOOT_REQUEST",
            Self::BootComplete => "BOOT_COMPLETE",
            Self::JobStart => "JOB_START",
            Self::JobComplete => "JOB_COMPLETE",
            Self::Checkpoint => "CHECKPOINT",
            Self::TeardownRequest => "TEARDOWN_REQUEST",
            Self::TeardownComplete => "TEARDOWN_COMPLETE",
            Self::Preemption => "PREEMPTION",
            Self::AdapterLoad => "ADAPTER_LOAD",
            Self::KvPoolSync => "KV_POOL_SYNC",
        })
    }
}

/// Error returned by [`EventType::from_str`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("unknown event_type: {0:?}")]
pub struct EventTypeParseError(String);

impl FromStr for EventType {
    type Err = EventTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BOOT_REQUEST" => Ok(Self::BootRequest),
            "BOOT_COMPLETE" => Ok(Self::BootComplete),
            "JOB_START" => Ok(Self::JobStart),
            "JOB_COMPLETE" => Ok(Self::JobComplete),
            "CHECKPOINT" => Ok(Self::Checkpoint),
            "TEARDOWN_REQUEST" => Ok(Self::TeardownRequest),
            "TEARDOWN_COMPLETE" => Ok(Self::TeardownComplete),
            "PREEMPTION" => Ok(Self::Preemption),
            "ADAPTER_LOAD" => Ok(Self::AdapterLoad),
            "KV_POOL_SYNC" => Ok(Self::KvPoolSync),
            other => Err(EventTypeParseError(other.to_owned())),
        }
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;

    const ALL_VARIANTS: &[(EventType, &str)] = &[
        (EventType::BootRequest, "BOOT_REQUEST"),
        (EventType::BootComplete, "BOOT_COMPLETE"),
        (EventType::JobStart, "JOB_START"),
        (EventType::JobComplete, "JOB_COMPLETE"),
        (EventType::Checkpoint, "CHECKPOINT"),
        (EventType::TeardownRequest, "TEARDOWN_REQUEST"),
        (EventType::TeardownComplete, "TEARDOWN_COMPLETE"),
        (EventType::Preemption, "PREEMPTION"),
        (EventType::AdapterLoad, "ADAPTER_LOAD"),
        (EventType::KvPoolSync, "KV_POOL_SYNC"),
    ];

    #[test]
    fn all_ten_variants_covered() {
        assert_eq!(
            ALL_VARIANTS.len(),
            10,
            "spec defines exactly 10 event types"
        );
    }

    #[test]
    fn display_matches_spec_wire_strings() {
        for (variant, wire) in ALL_VARIANTS {
            assert_eq!(variant.to_string(), *wire, "variant: {variant:?}");
        }
    }

    #[test]
    fn from_str_round_trips_all_variants() {
        for (variant, wire) in ALL_VARIANTS {
            let parsed: EventType = wire
                .parse()
                .unwrap_or_else(|e| panic!("expected {wire:?} to parse, got {e}"));
            assert_eq!(parsed, *variant);
        }
    }

    #[test]
    fn from_str_rejects_unknown() {
        assert!(matches!(
            "UNKNOWN_EVENT".parse::<EventType>(),
            Err(EventTypeParseError(_))
        ));
    }

    #[test]
    fn serde_json_round_trips_all_variants() {
        for (variant, wire) in ALL_VARIANTS {
            let json = serde_json::to_string(variant).unwrap();
            assert_eq!(json, format!("\"{wire}\""), "variant: {variant:?}");
            let back: EventType = serde_json::from_str(&json).unwrap();
            assert_eq!(back, *variant);
        }
    }
}
