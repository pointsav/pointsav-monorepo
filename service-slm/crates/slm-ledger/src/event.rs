// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! The [`Event`] struct — one row in the append-only ledger.
//!
//! Field order matches the YOYO-COMPUTE §5 CSV schema verbatim. The `csv`
//! crate's serde integration derives the CSV header from field declaration
//! order, so no post-processing is needed to match the spec.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slm_core::ModuleId;
use uuid::Uuid;

use crate::event_type::EventType;

/// One row in the yo-yo compute audit ledger.
///
/// Fields are declared in the column order defined by YOYO-COMPUTE §5 so that
/// the CSV header produced by the `csv` crate's serde integration matches the
/// spec exactly. The `moduleId` serde rename preserves the camelCase wire name
/// from the spec.
///
/// Optional fields are `None` when not applicable to the specific event type.
/// For example, `BOOT_REQUEST` carries no `job_id`; `KV_POOL_SYNC` carries no
/// `input_hash`. The append-only writer enforces which fields must be non-null
/// at construction time (task 13).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Universally unique identifier for this ledger row (UUID v7, time-ordered).
    pub event_id: Uuid,
    /// Wall-clock time at which the event was recorded, UTC.
    pub timestamp_utc: DateTime<Utc>,
    /// Category of this event.
    pub event_type: EventType,
    /// Project-namespace identifier threaded through every service-slm layer.
    #[serde(rename = "moduleId")]
    pub module_id: ModuleId,
    /// GCP node identifier. Present for node-lifecycle events.
    pub node_id: Option<String>,
    /// Job identifier. Present for job-lifecycle events.
    pub job_id: Option<String>,
    /// SHA-256 hash of the sanitised input payload.
    pub input_hash: Option<String>,
    /// Semicolon-separated `adapter_id:version` pairs active for this event.
    pub adapter_versions: Option<String>,
    /// Fraction of tokens served from the KV cache (0.0–1.0).
    pub cache_hit_ratio: Option<f64>,
    /// Total tokens processed (prompt + completion).
    pub tokens_processed: Option<u64>,
    /// Billable GPU-seconds consumed.
    pub gpu_seconds: Option<f64>,
    /// Estimated cost in US dollars for this event.
    pub cost_usd: Option<f64>,
    /// Terminal state of the job or node lifecycle step.
    pub completion_status: Option<String>,
    /// Error code if the event ended in failure; absent otherwise.
    pub error_code: Option<String>,
    /// Identity of the human operator who triggered the event, if applicable.
    pub operator_id: Option<String>,
}

impl Event {
    /// Constructs a minimal event with a fresh `event_id` and `timestamp_utc`.
    ///
    /// All optional fields default to `None`. Callers populate whichever
    /// fields apply to the specific event type before handing the row to the
    /// ledger writer.
    #[must_use]
    pub fn new(module_id: ModuleId, event_type: EventType) -> Self {
        Self {
            event_id: Uuid::now_v7(),
            timestamp_utc: Utc::now(),
            event_type,
            module_id,
            node_id: None,
            job_id: None,
            input_hash: None,
            adapter_versions: None,
            cache_hit_ratio: None,
            tokens_processed: None,
            gpu_seconds: None,
            cost_usd: None,
            completion_status: None,
            error_code: None,
            operator_id: None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;

    fn fixture(event_type: EventType) -> Event {
        Event {
            event_id: Uuid::nil(),
            timestamp_utc: "2026-04-20T10:00:00Z".parse::<DateTime<Utc>>().unwrap(),
            event_type,
            module_id: ModuleId::new("woodfine-v1").unwrap(),
            node_id: None,
            job_id: None,
            input_hash: None,
            adapter_versions: None,
            cache_hit_ratio: None,
            tokens_processed: None,
            gpu_seconds: None,
            cost_usd: None,
            completion_status: None,
            error_code: None,
            operator_id: None,
        }
    }

    fn csv_round_trip(event: &Event) -> Event {
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(event).expect("serialize");
        wtr.flush().expect("flush");
        let data = wtr.into_inner().expect("into_inner");
        let mut rdr = csv::Reader::from_reader(data.as_slice());
        rdr.deserialize()
            .next()
            .expect("one row")
            .expect("deserialize")
    }

    #[test]
    fn csv_header_matches_spec_column_order() {
        let event = fixture(EventType::BootRequest);
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(&event).expect("serialize");
        wtr.flush().expect("flush");
        let raw = String::from_utf8(wtr.into_inner().expect("into_inner")).expect("utf8");
        let header = raw.lines().next().expect("header line");
        assert_eq!(
            header,
            concat!(
                "event_id,timestamp_utc,event_type,moduleId,",
                "node_id,job_id,input_hash,adapter_versions,",
                "cache_hit_ratio,tokens_processed,gpu_seconds,cost_usd,",
                "completion_status,error_code,operator_id",
            ),
        );
    }

    #[test]
    fn round_trip_boot_request() {
        let e = fixture(EventType::BootRequest);
        assert_eq!(csv_round_trip(&e), e);
    }

    #[test]
    fn round_trip_boot_complete() {
        let e = fixture(EventType::BootComplete);
        assert_eq!(csv_round_trip(&e), e);
    }

    #[test]
    fn round_trip_job_start() {
        let e = fixture(EventType::JobStart);
        assert_eq!(csv_round_trip(&e), e);
    }

    #[test]
    fn round_trip_job_complete() {
        let e = fixture(EventType::JobComplete);
        assert_eq!(csv_round_trip(&e), e);
    }

    #[test]
    fn round_trip_checkpoint() {
        let e = fixture(EventType::Checkpoint);
        assert_eq!(csv_round_trip(&e), e);
    }

    #[test]
    fn round_trip_teardown_request() {
        let e = fixture(EventType::TeardownRequest);
        assert_eq!(csv_round_trip(&e), e);
    }

    #[test]
    fn round_trip_teardown_complete() {
        let e = fixture(EventType::TeardownComplete);
        assert_eq!(csv_round_trip(&e), e);
    }

    #[test]
    fn round_trip_preemption() {
        let e = fixture(EventType::Preemption);
        assert_eq!(csv_round_trip(&e), e);
    }

    #[test]
    fn round_trip_adapter_load() {
        let e = fixture(EventType::AdapterLoad);
        assert_eq!(csv_round_trip(&e), e);
    }

    #[test]
    fn round_trip_kv_pool_sync() {
        let e = fixture(EventType::KvPoolSync);
        assert_eq!(csv_round_trip(&e), e);
    }

    #[test]
    fn round_trip_with_all_optional_fields_populated() {
        let e = Event {
            event_id: Uuid::nil(),
            timestamp_utc: "2026-04-20T12:34:56.789Z".parse::<DateTime<Utc>>().unwrap(),
            event_type: EventType::JobComplete,
            module_id: ModuleId::new("woodfine-v1").unwrap(),
            node_id: Some("node-gcp-us-central1-1".to_owned()),
            job_id: Some("job-abc123".to_owned()),
            input_hash: Some("sha256:deadbeef".to_owned()),
            adapter_versions: Some("dka-coa:v3.2;woodfine-entity:v1.4".to_owned()),
            cache_hit_ratio: Some(0.87),
            tokens_processed: Some(14_500),
            gpu_seconds: Some(12.4),
            cost_usd: Some(0.0031),
            completion_status: Some("SUCCESS".to_owned()),
            error_code: None,
            operator_id: Some("op-pwoodfine".to_owned()),
        };
        assert_eq!(csv_round_trip(&e), e);
    }
}
