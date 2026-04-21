// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Integration tests for `RemoteInferenceClient::boot`.
//!
//! Drives the client against a local wiremock-served `/healthz` endpoint
//! and reads the resulting CSV ledger back to assert row count, ordering,
//! completion status, and error code. Covers one happy path and one
//! transport-failure path; per-retry policy and preemption coverage land
//! in later tasks.

#![allow(clippy::disallowed_methods)]
#![allow(missing_docs)]

use std::path::Path;

use slm_core::ModuleId;
use slm_inference_remote::{RemoteInferenceClient, RemoteInferenceConfig, RemoteInferenceError};
use slm_ledger::{Event, EventType, LedgerWriter};
use tempfile::tempdir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn module_id() -> ModuleId {
    ModuleId::new("woodfine-v1").expect("valid module id")
}

fn read_ledger(ledger_path: &Path) -> Vec<Event> {
    let mut rdr = csv::Reader::from_path(ledger_path).expect("open ledger");
    rdr.deserialize::<Event>()
        .collect::<Result<Vec<_>, _>>()
        .expect("deserialize ledger rows")
}

#[tokio::test]
async fn boot_happy_path_writes_request_then_complete_success() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/healthz"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "node_id": "node-test-1"
        })))
        .mount(&server)
        .await;

    let dir = tempdir().expect("tempdir");
    let ledger_path = dir.path().join("ledger.csv");
    let mut writer = LedgerWriter::open(&ledger_path).expect("open ledger");

    let cfg = RemoteInferenceConfig::new(&server.uri(), module_id()).expect("valid config");
    let client = RemoteInferenceClient::new(cfg).expect("build client");

    let handle = client.boot(&mut writer).await.expect("boot succeeds");
    assert_eq!(handle.node_id(), "node-test-1");

    drop(writer);

    let rows = read_ledger(&ledger_path);
    assert_eq!(rows.len(), 2, "expected BOOT_REQUEST and BOOT_COMPLETE");
    assert_eq!(rows[0].event_type, EventType::BootRequest);
    assert_eq!(rows[1].event_type, EventType::BootComplete);
    assert_eq!(rows[1].completion_status.as_deref(), Some("SUCCESS"));
    assert_eq!(rows[1].node_id.as_deref(), Some("node-test-1"));
    assert!(rows[1].error_code.is_none());
    assert_eq!(rows[0].module_id, module_id());
    assert_eq!(rows[1].module_id, module_id());
}

#[tokio::test]
async fn boot_failure_records_failed_row_and_returns_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/healthz"))
        .respond_with(ResponseTemplate::new(503).set_body_string("service unavailable"))
        .mount(&server)
        .await;

    let dir = tempdir().expect("tempdir");
    let ledger_path = dir.path().join("ledger.csv");
    let mut writer = LedgerWriter::open(&ledger_path).expect("open ledger");

    let cfg = RemoteInferenceConfig::new(&server.uri(), module_id()).expect("valid config");
    let client = RemoteInferenceClient::new(cfg).expect("build client");

    let err = client
        .boot(&mut writer)
        .await
        .expect_err("boot should fail on 503");
    match err {
        RemoteInferenceError::RemoteStatus { status, body } => {
            assert_eq!(status, 503);
            assert_eq!(body, "service unavailable");
        }
        other => panic!("expected RemoteStatus(503), got {other:?}"),
    }

    drop(writer);

    let rows = read_ledger(&ledger_path);
    assert_eq!(
        rows.len(),
        2,
        "expected BOOT_REQUEST and BOOT_COMPLETE (FAILED)"
    );
    assert_eq!(rows[0].event_type, EventType::BootRequest);
    assert_eq!(rows[1].event_type, EventType::BootComplete);
    assert_eq!(rows[1].completion_status.as_deref(), Some("FAILED"));
    assert_eq!(rows[1].error_code.as_deref(), Some("HTTP_503"));
    assert!(rows[1].node_id.is_none());
}
