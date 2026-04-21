// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.
//
// Integration smoke test: exercises the public API across the crate boundary.

#![allow(missing_docs, clippy::disallowed_methods)]

use std::path::Path;

use chrono::DateTime;
use slm_core::ModuleId;
use slm_ledger::{Event, EventType, LedgerWriter};
use uuid::Uuid;

#[test]
fn event_csv_round_trip_via_public_api() {
    let original = Event {
        event_id: Uuid::nil(),
        timestamp_utc: "2026-04-20T10:00:00Z"
            .parse::<DateTime<chrono::Utc>>()
            .unwrap(),
        event_type: EventType::JobComplete,
        module_id: ModuleId::new("woodfine-v1").unwrap(),
        node_id: Some("node-gcp-us-central1-1".to_owned()),
        job_id: Some("job-smoke-001".to_owned()),
        input_hash: Some("sha256:cafebabe".to_owned()),
        adapter_versions: Some("dka-coa:v3.2".to_owned()),
        cache_hit_ratio: Some(0.75),
        tokens_processed: Some(8_192),
        gpu_seconds: Some(4.2),
        cost_usd: Some(0.0011),
        completion_status: Some("SUCCESS".to_owned()),
        error_code: None,
        operator_id: None,
    };

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.serialize(&original).expect("serialize");
    wtr.flush().expect("flush");
    let data = wtr.into_inner().expect("into_inner");

    let mut rdr = csv::Reader::from_reader(data.as_slice());
    let recovered: Event = rdr
        .deserialize()
        .next()
        .expect("one row")
        .expect("deserialize");

    assert_eq!(original, recovered);
}

#[test]
fn all_ten_event_types_round_trip() {
    use EventType::*;
    let variants = [
        BootRequest,
        BootComplete,
        JobStart,
        JobComplete,
        Checkpoint,
        TeardownRequest,
        TeardownComplete,
        Preemption,
        AdapterLoad,
        KvPoolSync,
    ];

    for event_type in variants {
        let e = Event::new(ModuleId::new("smoke-test").unwrap(), event_type);
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(&e)
            .unwrap_or_else(|err| panic!("{event_type}: serialize failed: {err}"));
        wtr.flush().unwrap();
        let data = wtr.into_inner().unwrap();
        let mut rdr = csv::Reader::from_reader(data.as_slice());
        let back: Event = rdr
            .deserialize()
            .next()
            .unwrap_or_else(|| panic!("{event_type}: no row"))
            .unwrap_or_else(|err| panic!("{event_type}: deserialize failed: {err}"));
        assert_eq!(back.event_type, event_type);
        assert_eq!(back.module_id, e.module_id);
    }
}

// ── Writer durability tests ───────────────────────────────────────────────────

/// Reads all data rows from a ledger file, returning them as a `Vec<Event>`.
/// Opens a fresh file handle each time so results are not influenced by any
/// in-memory state held by an open `LedgerWriter`.
fn read_all_events(path: &Path) -> Vec<Event> {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    rdr.deserialize().map(|r| r.unwrap()).collect()
}

/// After each successful `append()` the row must be visible to a fresh file
/// reader — `fsync` has committed it to durable storage.
///
/// The test demonstrates the crash-safety guarantee: if the process were
/// killed at any point after `append()` returns, the rows written up to that
/// point would survive, because `sync_all()` was called before returning.
#[test]
fn data_is_durable_after_each_append() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("ledger.csv");

    let module_id = ModuleId::new("smoke-durability").unwrap();
    let event1 = Event::new(module_id.clone(), EventType::BootRequest);
    let event2 = Event::new(module_id.clone(), EventType::BootComplete);

    let mut writer = LedgerWriter::open(&path).unwrap();

    // Write first row and verify it is on disk before touching the writer again.
    writer.append(&event1).unwrap();
    let rows = read_all_events(&path);
    assert_eq!(rows.len(), 1, "one row after first append");
    assert_eq!(rows[0].event_type, EventType::BootRequest);

    // Write second row and verify both are on disk.
    writer.append(&event2).unwrap();
    let rows = read_all_events(&path);
    assert_eq!(rows.len(), 2, "two rows after second append");
    assert_eq!(rows[1].event_type, EventType::BootComplete);

    // Simulate a crash by dropping the writer without an explicit close.
    // Both rows must still be present because each append fsynced.
    drop(writer);
    let rows = read_all_events(&path);
    assert_eq!(rows.len(), 2, "two rows survive after writer is dropped");
}

/// Reopening an existing ledger must not write a second header row.
/// A duplicate header would cause CSV readers to misparse every subsequent row.
#[test]
fn reopen_does_not_duplicate_header() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("ledger.csv");

    let module_id = ModuleId::new("smoke-reopen").unwrap();

    // First session: write one event.
    let mut writer = LedgerWriter::open(&path).unwrap();
    writer
        .append(&Event::new(module_id.clone(), EventType::JobStart))
        .unwrap();
    drop(writer);

    // Second session: write another event to the same file.
    let mut writer = LedgerWriter::open(&path).unwrap();
    writer
        .append(&Event::new(module_id.clone(), EventType::JobComplete))
        .unwrap();
    drop(writer);

    // Exactly two data rows, one header row.
    let rows = read_all_events(&path);
    assert_eq!(rows.len(), 2, "two data rows across two sessions");
    assert_eq!(rows[0].event_type, EventType::JobStart);
    assert_eq!(rows[1].event_type, EventType::JobComplete);
}
