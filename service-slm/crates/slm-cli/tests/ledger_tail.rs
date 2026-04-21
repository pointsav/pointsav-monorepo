// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![allow(missing_docs)]
#![allow(clippy::disallowed_methods)]

use assert_cmd::Command;
use chrono::Utc;
use slm_core::ModuleId;
use slm_ledger::{Event, EventType, LedgerWriter};
use tempfile::NamedTempFile;

fn write_events(count: usize) -> NamedTempFile {
    let f = NamedTempFile::new().expect("tempfile");
    let mut writer = LedgerWriter::open(f.path()).expect("open ledger");
    let module_id = ModuleId::new("test-module").expect("module id");
    for i in 0..count {
        let mut e = Event::new(module_id.clone(), EventType::BootRequest);
        e.timestamp_utc = Utc::now();
        e.node_id = Some(format!("node-{i}"));
        writer.append(&e).expect("append");
    }
    f
}

#[test]
fn tail_returns_last_n_events() {
    let f = write_events(5);
    Command::cargo_bin("slm-cli")
        .unwrap()
        .args([
            "ledger",
            "--path",
            f.path().to_str().unwrap(),
            "tail",
            "--n",
            "2",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("BOOT_REQUEST"))
        .stdout(predicates::str::contains("test-module"))
        .stdout(predicates::str::contains("node-4"))
        .stdout(predicates::str::contains("node-3"));
}

#[test]
fn tail_zero_prints_no_events() {
    let f = write_events(3);
    Command::cargo_bin("slm-cli")
        .unwrap()
        .args([
            "ledger",
            "--path",
            f.path().to_str().unwrap(),
            "tail",
            "--n",
            "0",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("(no events)"));
}

#[test]
fn tail_more_than_available_returns_all() {
    let f = write_events(2);
    Command::cargo_bin("slm-cli")
        .unwrap()
        .args([
            "ledger",
            "--path",
            f.path().to_str().unwrap(),
            "tail",
            "--n",
            "100",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("node-0"))
        .stdout(predicates::str::contains("node-1"));
}

#[test]
fn tail_missing_file_exits_nonzero() {
    Command::cargo_bin("slm-cli")
        .unwrap()
        .args(["ledger", "--path", "/nonexistent/ledger.csv", "tail"])
        .assert()
        .failure();
}
