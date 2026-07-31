// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Per-VM discovery/allocation (§14 #20) — an append-only, disk-persisted
//! ledger of module_ids ever handed out by `POST /v1/discovery/allocate`.
//!
//! Closes a real gap in the self-claimed `SLM_MODULE_ID`/`SLM_ARCHIVE_ID`
//! scheme: `FleetRegistry::register()` is a plain `HashMap::insert()` keyed
//! by a self-claimed module_id with zero collision detection — two VMs (or
//! an operator typo) claiming the same identity silently overwrite each
//! other's fleet entry, and an old Doorman's still-valid signed membership
//! token keeps authenticating under an identity the registry now associates
//! with a different VM's endpoint.
//!
//! Deliberately narrower than making the whole chassis stateful:
//! `FleetRegistry` (live membership — who's online right now, their
//! `doorman_endpoint`, subscription status) stays exactly as documented —
//! rebuilt from registration calls on every chassis restart. Only the much
//! smaller "which module_ids have ever been allocated" fact needs to survive
//! a restart, so a restarted chassis never reallocates one that's still in
//! active use by a Doorman that hasn't yet re-registered. This is an
//! append-only set, not a live-state cache — no update/delete path, no
//! staleness to reason about.

use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::error::{ChassisError, Result};

#[derive(Debug, Serialize, Deserialize)]
struct LedgerEntry {
    module_id: String,
    archive_id: String,
    allocated_at: chrono::DateTime<chrono::Utc>,
}

/// Thread-safe, disk-persisted set of allocated module_ids.
pub struct AllocationLedger {
    path: PathBuf,
    // Guards both the in-memory set and the append-only file write, so a
    // check-then-append never races with itself under concurrent requests.
    allocated: Mutex<HashSet<String>>,
}

impl AllocationLedger {
    /// Loads existing allocations from `path` (creating the parent directory
    /// if the ledger file doesn't exist yet — the file itself is created
    /// lazily on first append). Malformed lines are skipped, not fatal — an
    /// unreadable historical entry should never block chassis startup; the
    /// worst case is one already-allocated id becomes reallocatable, the
    /// same narrow risk this ledger exists to shrink, not eliminate
    /// entirely.
    pub fn load(path: PathBuf) -> Result<Self> {
        let mut allocated = HashSet::new();
        if path.exists() {
            let file = File::open(&path).map_err(|e| {
                ChassisError::AllocationLedgerIo(format!("open {}: {e}", path.display()))
            })?;
            for line in BufReader::new(file).lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => continue,
                };
                if line.trim().is_empty() {
                    continue;
                }
                if let Ok(entry) = serde_json::from_str::<LedgerEntry>(&line) {
                    allocated.insert(entry.module_id);
                }
                // malformed lines silently skipped — see doc comment above
            }
        } else if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                ChassisError::AllocationLedgerIo(format!(
                    "create_dir_all {}: {e}",
                    parent.display()
                ))
            })?;
        }
        Ok(Self { path, allocated: Mutex::new(allocated) })
    }

    /// In-memory only, for tests — never touches disk.
    #[cfg(test)]
    fn in_memory() -> Self {
        Self { path: PathBuf::new(), allocated: Mutex::new(HashSet::new()) }
    }

    /// Allocate a fresh, guaranteed-unique `(module_id, archive_id)` pair.
    /// `hint` is a purely cosmetic suggestion for the archive_id — never
    /// trusted for uniqueness; on collision a numeric suffix is appended
    /// until a free id is found. Appends the accepted entry to the ledger
    /// file *before* returning it, so a crash between allocation and the
    /// caller's own `/v1/discovery/register` call never silently loses the
    /// reservation (the id stays reserved even if the caller never shows up).
    pub fn allocate(&self, hint: Option<&str>) -> Result<(String, String)> {
        let base = hint
            .map(sanitize_hint)
            .filter(|h| !h.is_empty())
            .unwrap_or_else(|| "archive".to_string());

        let mut guard = self.allocated.lock().unwrap();
        let mut archive_id = base.clone();
        let mut module_id = format!("op::{archive_id}::slm");
        let mut suffix = 0u32;
        while guard.contains(&module_id) {
            suffix += 1;
            archive_id = format!("{base}-{suffix}");
            module_id = format!("op::{archive_id}::slm");
        }

        self.append(&module_id, &archive_id)?;
        guard.insert(module_id.clone());
        Ok((module_id, archive_id))
    }

    fn append(&self, module_id: &str, archive_id: &str) -> Result<()> {
        if self.path.as_os_str().is_empty() {
            return Ok(()); // in_memory() test mode — no disk backing
        }
        let entry = LedgerEntry {
            module_id: module_id.to_string(),
            archive_id: archive_id.to_string(),
            allocated_at: chrono::Utc::now(),
        };
        let line = serde_json::to_string(&entry)?;
        let mut file = OpenOptions::new().create(true).append(true).open(&self.path).map_err(
            |e| ChassisError::AllocationLedgerIo(format!("append {}: {e}", self.path.display())),
        )?;
        writeln!(file, "{line}").map_err(|e| {
            ChassisError::AllocationLedgerIo(format!("write {}: {e}", self.path.display()))
        })?;
        file.sync_data().map_err(|e| {
            ChassisError::AllocationLedgerIo(format!("fsync {}: {e}", self.path.display()))
        })?;
        Ok(())
    }
}

/// Lowercase, ASCII-alphanumeric-and-hyphen only, matching the archive's own
/// naming convention (`conventions/nomenclature-taxonomy.md`: lowercase ASCII
/// and hyphens). A hint like `"Cluster Totebox Data 1!"` becomes
/// `"cluster-totebox-data-1"`.
fn sanitize_hint(hint: &str) -> String {
    hint.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_with_no_hint_uses_default_base() {
        let ledger = AllocationLedger::in_memory();
        let (module_id, archive_id) = ledger.allocate(None).expect("first allocation");
        assert_eq!(module_id, "op::archive::slm");
        assert_eq!(archive_id, "archive");
    }

    #[test]
    fn allocate_with_hint_sanitizes_and_uses_it() {
        let ledger = AllocationLedger::in_memory();
        let (module_id, archive_id) =
            ledger.allocate(Some("Cluster Totebox Data 1!")).expect("first allocation");
        assert_eq!(archive_id, "cluster-totebox-data-1");
        assert_eq!(module_id, "op::cluster-totebox-data-1::slm");
    }

    /// The core guarantee: two allocations requesting the same hint must
    /// never collide — the second gets a disambiguating suffix.
    #[test]
    fn colliding_hints_get_distinct_ids() {
        let ledger = AllocationLedger::in_memory();
        let (m1, a1) = ledger.allocate(Some("data")).expect("first");
        let (m2, a2) = ledger.allocate(Some("data")).expect("second");
        assert_ne!(m1, m2, "module_ids must never collide");
        assert_ne!(a1, a2);
        assert_eq!(a2, "data-1");
    }

    #[test]
    fn three_way_collision_increments_suffix_each_time() {
        let ledger = AllocationLedger::in_memory();
        let (_, a1) = ledger.allocate(Some("x")).unwrap();
        let (_, a2) = ledger.allocate(Some("x")).unwrap();
        let (_, a3) = ledger.allocate(Some("x")).unwrap();
        assert_eq!([a1, a2, a3], ["x".to_string(), "x-1".to_string(), "x-2".to_string()]);
    }

    /// Real disk round-trip: allocate, drop the ledger, load a fresh one
    /// from the same path — the previously-allocated id must still be
    /// known, so a "restarted chassis" (a fresh `AllocationLedger::load`)
    /// never reallocates it.
    #[test]
    fn persists_across_reload_from_disk() {
        let dir = std::env::temp_dir().join(format!(
            "orchestration-slm-alloc-test-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        let path = dir.join("allocated-ids.jsonl");

        {
            let ledger = AllocationLedger::load(path.clone()).expect("first load");
            let (module_id, _) = ledger.allocate(Some("survives-restart")).unwrap();
            assert_eq!(module_id, "op::survives-restart::slm");
        } // ledger dropped — simulates chassis restart

        let reloaded = AllocationLedger::load(path).expect("reload from disk");
        // Allocating the same hint again must NOT reproduce the same id —
        // proves the reload actually saw the on-disk entry.
        let (module_id_2, archive_id_2) = reloaded.allocate(Some("survives-restart")).unwrap();
        assert_ne!(module_id_2, "op::survives-restart::slm");
        assert_eq!(archive_id_2, "survives-restart-1");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn sanitize_hint_strips_non_alphanumerics_and_lowercases() {
        assert_eq!(sanitize_hint("Cluster Totebox Data 1!"), "cluster-totebox-data-1");
        assert_eq!(sanitize_hint("already-lower"), "already-lower");
        assert_eq!(sanitize_hint("---"), "");
    }
}
