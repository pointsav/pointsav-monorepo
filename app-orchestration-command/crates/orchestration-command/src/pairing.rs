// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Pairing store — records completed pairings and appends to the WORM audit ledger.
//!
//! v0.0.2 writes pairings to `user-pairings.yaml` (same directory as pairings.yaml)
//! in addition to the in-process store. The WORM ledger (append-only JSONL) is written
//! to `COMMAND_AUDIT_LEDGER_PATH` (default: `./data/command-audit.jsonl`).
//!
//! v0.1.0 adds `PairingStore::load()`, called once at server startup, which parses
//! `user-pairings.yaml` back into the in-process store so a restart doesn't forget
//! prior pairings (previously `records` always started empty even though the file
//! held history).

use std::path::PathBuf;
use std::sync::Mutex;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

/// WORM ledger entry for a `pairing_revoked` event (v0.1.0, `schema_version: "2"`).
///
/// The ledger is append-only — revocation is recorded as a new event, never by
/// mutating the original `pairing_created` line. A key is "revoked" iff a
/// `pairing_revoked` entry with a matching `key_fingerprint` exists later in the
/// ledger; readers scanning `schema_version: "1"` (`pairing_created`-only) history
/// naturally see no revocation events and correctly treat every key as not revoked —
/// no special-casing needed for old entries.
#[derive(Debug, Serialize)]
struct RevocationLedgerEntry<'a> {
    schema_version: &'static str,
    event: &'static str,
    ts: DateTime<Utc>,
    key_fingerprint: String,
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

/// Owned, deserializable mirror of `UserPairingEntry` — used only by `load()` to
/// parse `user-pairings.yaml` back at startup.
#[derive(Debug, Deserialize)]
struct StoredUserPairingEntry {
    public_key: String,
    role: PairingRole,
    #[serde(default)]
    archive_scope: Vec<String>,
    #[serde(default)]
    node_label: String,
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

    /// Restore in-process state from `user-pairings.yaml` at startup (v0.1.0).
    ///
    /// A missing file is a fresh install, not an error — returns `Ok(0)`. A file
    /// that fails to parse is also not fatal to startup: the caller logs it and the
    /// server continues in the same state as a fresh install (empty in-process
    /// store), rather than refusing to boot over a corrupt history file. Returns the
    /// number of pairings restored.
    pub fn load(&self) -> Result<usize, CommandError> {
        let contents = match std::fs::read_to_string(&self.user_pairings_path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
            Err(e) => return Err(CommandError::Io(e)),
        };
        if contents.trim().is_empty() {
            return Ok(0);
        }

        let entries: Vec<StoredUserPairingEntry> = serde_yaml::from_str(&contents)
            .map_err(|e| CommandError::Pairing(format!("user-pairings yaml parse: {e}")))?;

        let mut records = self.records.lock().unwrap();
        let mut restored: u64 = 0;
        for entry in entries {
            if records.iter().any(|r| r.public_key == entry.public_key) {
                continue; // already restored — user-pairings.yaml has no unique constraint
            }
            records.push(PairingRecord {
                public_key: entry.public_key,
                role: entry.role,
                archive_scope: entry.archive_scope,
                paired_on: entry.paired_on,
                node_label: entry.node_label,
            });
            restored += 1;
        }
        drop(records);
        *self.audit_count.lock().unwrap() += restored;

        Ok(restored as usize)
    }

    /// Revoke a pairing by public key: append a `pairing_revoked` WORM ledger event
    /// and drop it from the in-process store (v0.1.0). Does NOT rewrite
    /// `user-pairings.yaml` — that file is an append-only application-layer history,
    /// matching the WORM ledger's own append-only contract; revocation status is
    /// derived from the ledger, not from removing the original entry.
    ///
    /// Returns `Ok(false)` if no matching in-process record exists (nothing to revoke).
    pub fn revoke(&self, public_key: &str) -> Result<bool, CommandError> {
        let removed = {
            let mut records = self.records.lock().unwrap();
            let before = records.len();
            records.retain(|r| r.public_key != public_key);
            before != records.len()
        };
        if !removed {
            return Ok(false);
        }

        let now = Utc::now();
        let entry = RevocationLedgerEntry {
            schema_version: "2",
            event: "pairing_revoked",
            ts: now,
            key_fingerprint: key_fingerprint(public_key.as_bytes()),
            instance: &self.instance_id,
        };
        let mut line = serde_json::to_string(&entry)
            .map_err(|e| CommandError::Pairing(format!("ledger json: {e}")))?;
        line.push('\n');
        append_to_file(&self.ledger_path, &line)?;

        Ok(true)
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

/// SHA-256 fingerprint for audit log key identification (v0.1.0 — replaces the
/// FNV-1a placeholder; `sha2` was pulled in specifically for this).
fn key_fingerprint(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(data);
    format!("sha256:{digest:x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// Construct a `PairingStore` rooted at `dir` — private-field literal, valid
    /// because `tests` is a descendant module of `pairing`. Avoids the process-global
    /// env vars `PairingStore::new()` reads, which would race across parallel tests.
    fn store_at(dir: &Path) -> PairingStore {
        PairingStore {
            records: Mutex::new(Vec::new()),
            ledger_path: dir.join("audit.jsonl"),
            user_pairings_path: dir.join("user-pairings.yaml"),
            instance_id: "test-instance".to_string(),
            audit_count: Mutex::new(0),
        }
    }

    /// Unique per-call temp dir — avoids collisions between parallel tests without
    /// pulling in a `tempfile` dependency for four call sites.
    fn temp_dir(label: &str) -> std::path::PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("orchestration-command-test-{label}-{unique}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn fingerprint_is_sha256() {
        let fp = key_fingerprint(b"test");
        assert_eq!(
            fp,
            "sha256:9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
        assert!(!fp.starts_with("fnv64:"));
    }

    #[test]
    fn record_then_load_restores_pairing() {
        let dir = temp_dir("restore");

        let store = store_at(&dir);
        store
            .record(
                "pubkey-abc".into(),
                PairingRole::User,
                vec!["bim".into()],
                "jennifer-macbook".into(),
            )
            .unwrap();
        assert_eq!(store.pairings_created(), 1);
        drop(store);

        // Fresh instance, same paths — simulates a server restart.
        let restarted = store_at(&dir);
        assert_eq!(restarted.pairings_created(), 0);
        let restored = restarted.load().unwrap();
        assert_eq!(restored, 1);
        assert_eq!(restarted.pairings_created(), 1);

        // Restoring the same key twice must not double up.
        let restored_again = restarted.load().unwrap();
        assert_eq!(restored_again, 0);
        assert_eq!(restarted.pairings_created(), 1);
    }

    #[test]
    fn load_missing_file_returns_zero() {
        let dir = temp_dir("missing");
        let store = store_at(&dir);
        assert_eq!(store.load().unwrap(), 0);
    }

    #[test]
    fn revoke_removes_record_and_appends_ledger_event() {
        let dir = temp_dir("revoke");
        let store = store_at(&dir);
        store
            .record("pubkey-xyz".into(), PairingRole::Admin, vec![], "node-1".into())
            .unwrap();

        let revoked = store.revoke("pubkey-xyz").unwrap();
        assert!(revoked);

        // Unknown / already-revoked key — nothing left to remove.
        let revoked_again = store.revoke("pubkey-xyz").unwrap();
        assert!(!revoked_again);

        let ledger = std::fs::read_to_string(dir.join("audit.jsonl")).unwrap();
        let lines: Vec<&str> = ledger.lines().collect();
        assert_eq!(lines.len(), 2, "expected pairing_created + pairing_revoked");

        let revoked_entry: serde_json::Value = serde_json::from_str(lines[1]).unwrap();
        assert_eq!(revoked_entry["event"], "pairing_revoked");
        assert_eq!(revoked_entry["schema_version"], "2");
        assert!(revoked_entry["key_fingerprint"]
            .as_str()
            .unwrap()
            .starts_with("sha256:"));

        // First entry is untouched — WORM ledger, revocation never mutates history.
        let created_entry: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(created_entry["event"], "pairing_created");
        assert_eq!(created_entry["schema_version"], "1");
    }
}
