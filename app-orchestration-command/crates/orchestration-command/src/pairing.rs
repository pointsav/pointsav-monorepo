// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Pairing store — records completed pairings and appends to the WORM audit ledger.
//!
//! v0.0.2 writes pairings to `user-pairings.yaml` (same directory as pairings.yaml)
//! in addition to the in-process store. The WORM ledger (append-only JSONL) is written
//! to `COMMAND_AUDIT_LEDGER_PATH` (default: `./data/command-audit.jsonl`).

use std::path::PathBuf;
use std::sync::Mutex;

use chrono::{DateTime, Utc};
use serde::Serialize;

use orchestration_command_core::{PairingRole, PairStatus, PairResponse};
use crate::error::CommandError;

/// One persisted pairing entry (in-process store).
#[derive(Debug, Clone)]
pub struct PairingRecord {
    pub public_key: String,
    pub role: PairingRole,
    pub archive_scope: Vec<String>,
    pub paired_on: DateTime<Utc>,
    pub node_label: String,
}

/// WORM ledger entry (one line of JSONL per pairing event).
#[derive(Debug, Serialize)]
struct LedgerEntry<'a> {
    schema_version: &'static str,
    event: &'static str,
    ts: DateTime<Utc>,
    role: &'a PairingRole,
    archive_scope: &'a [String],
    key_fingerprint: String,
    node_label: &'a str,
    instance: &'a str,
}

/// YAML entry written to user-pairings.yaml (application-layer pairing store).
/// Infrastructure-layer pairings.yaml is NEVER written by this code.
#[derive(Debug, Serialize)]
struct UserPairingEntry<'a> {
    public_key: &'a str,
    role: &'a PairingRole,
    archive_scope: &'a [String],
    node_label: &'a str,
    paired_on: DateTime<Utc>,
}

pub struct PairingStore {
    records: Mutex<Vec<PairingRecord>>,
    ledger_path: PathBuf,
    user_pairings_path: PathBuf,
    instance_id: String,
    audit_count: Mutex<u64>,
}

impl PairingStore {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let ledger_path = std::env::var("COMMAND_AUDIT_LEDGER_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("data/command-audit.jsonl"));
        if let Some(parent) = ledger_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        // user-pairings.yaml lives alongside pairings.yaml (infrastructure ACK 2026-06-29).
        let user_pairings_path = std::env::var("COMMAND_PAIRINGS_PATH")
            .ok()
            .and_then(|p| PathBuf::from(&p).parent().map(|d| d.join("user-pairings.yaml")))
            .unwrap_or_else(|| PathBuf::from("user-pairings.yaml"));

        Self {
            records: Mutex::new(Vec::new()),
            ledger_path,
            user_pairings_path,
            instance_id: instance_id.into(),
            audit_count: Mutex::new(0),
        }
    }

    /// Record a new pairing, write to WORM ledger and user-pairings.yaml, return a `PairResponse`.
    pub fn record(
        &self,
        public_key: String,
        role: PairingRole,
        archive_scope: Vec<String>,
        node_label: String,
    ) -> Result<PairResponse, CommandError> {
        let now = Utc::now();
        let fingerprint = key_fingerprint(public_key.as_bytes());

        // Check for duplicate (same public key).
        {
            let records = self.records.lock().unwrap();
            if records.iter().any(|r| r.public_key == public_key) {
                return Ok(PairResponse {
                    status: PairStatus::AlreadyPaired,
                    paired_on: records
                        .iter()
                        .find(|r| r.public_key == public_key)
                        .map(|r| r.paired_on)
                        .unwrap_or(now),
                    role,
                    archive_scope,
                });
            }
        }

        // Append to WORM ledger.
        let entry = LedgerEntry {
            schema_version: "1",
            event: "pairing_created",
            ts: now,
            role: &role,
            archive_scope: &archive_scope,
            key_fingerprint: fingerprint,
            node_label: &node_label,
            instance: &self.instance_id,
        };
        let mut line = serde_json::to_string(&entry)
            .map_err(|e| CommandError::Pairing(format!("ledger json: {e}")))?;
        line.push('\n');
        append_to_file(&self.ledger_path, &line)?;

        // Write to user-pairings.yaml (application-layer persistent store).
        let user_entry = UserPairingEntry {
            public_key: &public_key,
            role: &role,
            archive_scope: &archive_scope,
            node_label: &node_label,
            paired_on: now,
        };
        write_user_pairing(&self.user_pairings_path, &user_entry)?;

        // Store in-process.
        let record = PairingRecord {
            public_key,
            role: role.clone(),
            archive_scope: archive_scope.clone(),
            paired_on: now,
            node_label,
        };
        self.records.lock().unwrap().push(record);
        *self.audit_count.lock().unwrap() += 1;

        Ok(PairResponse {
            status: PairStatus::Paired,
            paired_on: now,
            role,
            archive_scope,
        })
    }

    pub fn pairings_created(&self) -> u64 {
        *self.audit_count.lock().unwrap()
    }
}

fn append_to_file(path: &PathBuf, line: &str) -> Result<(), CommandError> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(line.as_bytes())?;
    Ok(())
}

/// Append one pairing entry to user-pairings.yaml as a YAML list item.
/// Appending `- key: val` blocks sequentially produces a valid YAML list.
fn write_user_pairing(path: &PathBuf, entry: &UserPairingEntry) -> Result<(), CommandError> {
    let yaml = serde_yaml::to_string(&[entry])
        .map_err(|e| CommandError::Pairing(format!("user-pairings yaml: {e}")))?;
    append_to_file(path, &yaml)?;
    Ok(())
}

/// FNV-1a fingerprint for audit log key identification.
/// Keeping deps minimal for v0.0.1; real SHA-256 would require the `sha2` crate.
fn key_fingerprint(data: &[u8]) -> String {
    let mut hash: u64 = 14695981039346656037u64;
    for &byte in data {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(1099511628211u64);
    }
    format!("fnv64:{hash:016x}")
}
