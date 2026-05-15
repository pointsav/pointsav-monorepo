// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Brief Queue Substrate — file-backed durable queue for `ApprenticeshipBrief`
//! records (apprenticeship-substrate.md §7C, doctrine v0.0.14).
//!
//! # Why this exists
//!
//! Tier A (OLMo 7B on CPU, ~2.5 t/s) is too slow for the capture-edit.py
//! post-commit hook's 300s timeout. Tier B (Yo-Yo) runs with idle-shutdown
//! discipline and may be asleep when a commit fires. The queue decouples
//! brief acceptance from apprentice execution: the post-commit hook writes
//! a brief file and returns immediately; a background drain worker picks up
//! the brief when a tier is available.
//!
//! # Directory layout
//!
//! ```text
//! <base>/queue/                 incoming briefs (worker reads)
//! <base>/queue-in-flight/       leased briefs (worker holds during execution)
//! <base>/queue-done/            processed briefs (audit trail)
//! <base>/queue-poison/          malformed or repeatedly-failed briefs
//! ```
//!
//! The `<base>` directory defaults to `$SLM_APPRENTICESHIP_BASE_DIR` (or
//! `$FOUNDRY_ROOT/data/apprenticeship/` if the env var is absent). Tests
//! inject a `tempdir()` path via `QueueConfig::base_dir`.
//!
//! # File naming
//!
//! - Queue entry:         `<brief_id>.brief.jsonl`
//! - In-flight lease:     `<brief_id>.brief.jsonl.lease.<worker_id>.<ts_nanos>`
//! - Done / Poison entry: same base name, renamed into the target directory
//!
//! # Atomicity and safety
//!
//! - **Enqueue**: `OpenOptions::create(true).write(true)` — last writer wins
//!   for same `brief_id` (idempotent; deterministic filename).
//! - **Dequeue (take lease)**: `std::fs::rename` is atomic on POSIX for same
//!   filesystem. Worker acquires an exclusive `flock(2)` on the `.queue.lock`
//!   sentinel before scanning; the lock is released after the rename. This
//!   prevents two concurrent workers from leasing the same brief.
//! - **Release**: rename from `queue-in-flight/` to `queue-done/` or
//!   `queue-poison/`, OR back to `queue/` on `Retry`.
//! - **Reap**: scan `queue-in-flight/` for leases whose timestamp part is
//!   older than `max_age`; rename back to `queue/` so the next worker can
//!   retry.
//!
//! # No credentials in queue files
//!
//! Per `conventions/api-key-boundary-discipline.md` (cited by
//! `four-tier-slm-substrate.md` §3): API keys live ONLY at the Doorman
//! boundary. `ApprenticeshipBrief` (slm-core) carries `module_id` for
//! tenant routing and the brief body text; it MUST NOT carry credentials.
//! The queue inherits this discipline by serialising the brief as-is.

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fs2::FileExt;
use slm_core::ApprenticeshipBrief;
use tracing::{debug, info, warn};

use slm_doorman::DoormanError;

/// Result alias for queue operations.
pub type QueueResult<T> = std::result::Result<T, DoormanError>;

// ── Configuration ────────────────────────────────────────────────────────────

/// Configuration for the brief queue.
///
/// Construct via [`QueueConfig::from_env`] for production use or
/// [`QueueConfig::with_base_dir`] for tests.
#[derive(Clone, Debug)]
pub struct QueueConfig {
    /// Root directory containing `queue/`, `queue-in-flight/`, `queue-done/`,
    /// and `queue-poison/` subdirectories.
    ///
    /// Defaults to `$SLM_APPRENTICESHIP_BASE_DIR`, falling back to
    /// `$FOUNDRY_ROOT/data/apprenticeship/` (or `/srv/foundry/data/apprenticeship/`).
    pub base_dir: PathBuf,
}

impl QueueConfig {
    /// Build config from environment. Reads `SLM_APPRENTICESHIP_BASE_DIR`
    /// first; falls back to `$FOUNDRY_ROOT/data/apprenticeship/`.
    pub fn from_env() -> Self {
        let base_dir = std::env::var_os("SLM_APPRENTICESHIP_BASE_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                let foundry_root: PathBuf = std::env::var_os("FOUNDRY_ROOT")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("/srv/foundry"));
                foundry_root.join("data").join("apprenticeship")
            });
        Self { base_dir }
    }

    /// Build config pointing at a specific directory — used by tests.
    pub fn with_base_dir(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
        }
    }

    fn queue_dir(&self) -> PathBuf {
        self.base_dir.join("queue")
    }

    fn in_flight_dir(&self) -> PathBuf {
        self.base_dir.join("queue-in-flight")
    }

    fn done_dir(&self) -> PathBuf {
        self.base_dir.join("queue-done")
    }

    fn poison_dir(&self) -> PathBuf {
        self.base_dir.join("queue-poison")
    }

    /// Sentinel file used for `flock(2)` single-writer on dequeue / reap.
    fn lock_sentinel(&self) -> PathBuf {
        self.base_dir.join(".queue.lock")
    }
}

// ── Public types ─────────────────────────────────────────────────────────────

/// Metadata returned by a successful [`enqueue`] call.
#[derive(Clone, Debug)]
pub struct QueueEntry {
    /// The `brief_id` from the enqueued brief.
    pub brief_id: String,
    /// Absolute path to the queue file (for diagnostics / tests).
    pub queue_path: PathBuf,
}

/// A leased brief returned by [`dequeue`].
///
/// The lease file in `queue-in-flight/` remains on disk until
/// [`release`] is called with this handle. If the Doorman crashes
/// before calling `release`, [`reap_expired_leases`] will eventually
/// return the brief to `queue/`.
#[derive(Clone, Debug)]
pub struct LeasedBrief {
    /// Parsed brief content.
    pub brief: ApprenticeshipBrief,
    /// Worker identifier embedded in the lease filename
    /// (`brief_id.brief.jsonl.lease.<worker_id>.<ts>`).
    pub worker_id: String,
    /// Nanosecond timestamp embedded in the lease filename (for reap).
    pub lease_ts_nanos: u128,
    /// Absolute path to the lease file in `queue-in-flight/`.
    pub lease_path: PathBuf,
    /// Base filename (`<brief_id>.brief.jsonl`) used to rename back.
    pub base_filename: String,
}

/// Shadow-specific queue entry that bundles a brief with the actual diff
/// from the senior's commit. Written by `enqueue_shadow()` and dequeued
/// by `dequeue_shadow()`.
///
/// Using a wider type than `ApprenticeshipBrief` lets the drain worker
/// pass the `actual_diff` to `dispatch_shadow()` without needing to carry
/// it out-of-band or embed it in the brief body text.
///
/// File naming follows the same `<brief_id>.brief.jsonl` convention as
/// plain `enqueue()` so the reaper, poison bucket, and release paths are
/// identical.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ShadowQueueEntry {
    /// The apprenticeship brief (identifies task-type, scope, body, etc.).
    pub brief: ApprenticeshipBrief,
    /// The unified diff that the senior actually committed (the post-hoc
    /// reference). Empty string signals "unknown diff" (e.g. in tests or
    /// for briefs promoted to the queue from capture-edit.py direct writes
    /// before this field was introduced).
    pub actual_diff: String,
}

/// A leased shadow entry returned by [`dequeue_shadow`].
#[derive(Clone, Debug)]
pub struct LeasedShadowEntry {
    /// Parsed shadow entry (brief + actual_diff).
    pub entry: ShadowQueueEntry,
    /// Worker identifier embedded in the lease filename.
    pub worker_id: String,
    /// Nanosecond timestamp embedded in the lease filename (for reap).
    pub lease_ts_nanos: u128,
    /// Absolute path to the lease file in `queue-in-flight/`.
    pub lease_path: PathBuf,
    /// Base filename (`<brief_id>.brief.jsonl`) used to rename back.
    pub base_filename: String,
}

/// Outcome passed to [`release`] after the worker finishes with a leased brief.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReleaseOutcome {
    /// Worker completed successfully. Move to `queue-done/`.
    Done,
    /// Transient failure (e.g. Tier A timeout). Return to `queue/` for retry.
    Retry,
    /// Permanent failure (e.g. malformed brief, repeated errors). Move to
    /// `queue-poison/`.
    Poison,
}

// ── Directory bootstrap ───────────────────────────────────────────────────────

/// Ensure all four queue subdirectories exist. Called at queue startup
/// and by tests before first use.
pub fn ensure_dirs(cfg: &QueueConfig) -> QueueResult<()> {
    for dir in [
        &cfg.queue_dir(),
        &cfg.in_flight_dir(),
        &cfg.done_dir(),
        &cfg.poison_dir(),
    ] {
        fs::create_dir_all(dir).map_err(|e| DoormanError::QueueIo {
            path: dir.display().to_string(),
            reason: e.to_string(),
        })?;
    }
    Ok(())
}

/// Count of pending `.brief.jsonl` files in `queue/` at the moment of
/// the call. Used by `shadow_handler` to populate the `queue_position`
/// field in the 202 ACCEPTED response.
///
/// Best-effort: concurrent enqueues between the `enqueue()` write and
/// this call can produce a count that is off by ±N. The value is a
/// useful hint for callers; it is not a reservation.
///
/// Returns 0 on any I/O error (the caller still gets a valid 202; the
/// position field is "unknown but queue accepted").
pub fn pending_count(cfg: &QueueConfig) -> usize {
    match fs::read_dir(cfg.queue_dir()) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().ends_with(".brief.jsonl"))
            .count(),
        Err(_) => 0,
    }
}

// ── API ───────────────────────────────────────────────────────────────────────

/// Write a brief to `queue/<brief_id>.brief.jsonl`.
///
/// **Idempotent**: if a file with the same `brief_id` already exists in
/// `queue/`, it is overwritten (last-writer-wins for same-content retries;
/// same `brief_id` different content is the caller's bug). If the brief is
/// already in `queue-in-flight/` or `queue-done/` it is NOT overwritten —
/// the existing state wins (the brief is in progress or already processed).
///
/// Returns a [`QueueEntry`] with the path on success.
pub fn enqueue(cfg: &QueueConfig, brief: &ApprenticeshipBrief) -> QueueResult<QueueEntry> {
    ensure_dirs(cfg)?;

    let filename = brief_filename(&brief.brief_id);
    let queue_path = cfg.queue_dir().join(&filename);

    // Idempotency: if the brief is already in-flight or done, return a
    // QueueEntry pointing at the queue path without writing again.
    let in_flight_path = cfg.in_flight_dir().join(&filename);
    let done_path = cfg.done_dir().join(&filename);
    if in_flight_path.exists() || done_path.exists() {
        debug!(
            brief_id = %brief.brief_id,
            "enqueue: brief already in-flight or done; skipping overwrite"
        );
        return Ok(QueueEntry {
            brief_id: brief.brief_id.clone(),
            queue_path,
        });
    }

    // Serialise the brief as a single JSONL line.
    let line = serde_json::to_string(brief).map_err(|e| DoormanError::QueueIo {
        path: queue_path.display().to_string(),
        reason: format!("brief serialisation failed: {e}"),
    })?;

    // Write with create+truncate so same-brief_id re-enqueue overwrites.
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&queue_path)
        .map_err(|e| DoormanError::QueueIo {
            path: queue_path.display().to_string(),
            reason: e.to_string(),
        })?;
    f.write_all(line.as_bytes())
        .and_then(|_| f.write_all(b"\n"))
        .and_then(|_| f.flush())
        .map_err(|e| DoormanError::QueueIo {
            path: queue_path.display().to_string(),
            reason: e.to_string(),
        })?;

    info!(
        brief_id = %brief.brief_id,
        queue_path = %queue_path.display(),
        "brief enqueued"
    );
    Ok(QueueEntry {
        brief_id: brief.brief_id.clone(),
        queue_path,
    })
}

/// Write a `ShadowQueueEntry` (brief + actual_diff) to
/// `queue/<brief_id>.brief.jsonl`.
///
/// Same idempotency and atomicity semantics as [`enqueue`]; the only
/// difference is the serialised payload is a `ShadowQueueEntry` rather
/// than a bare `ApprenticeshipBrief`.
///
/// Returns a [`QueueEntry`] with the path on success.
pub fn enqueue_shadow(cfg: &QueueConfig, entry: &ShadowQueueEntry) -> QueueResult<QueueEntry> {
    ensure_dirs(cfg)?;

    let brief_id = &entry.brief.brief_id;
    let filename = brief_filename(brief_id);
    let queue_path = cfg.queue_dir().join(&filename);

    // Idempotency: already in-flight or done → skip overwrite.
    let in_flight_path = cfg.in_flight_dir().join(&filename);
    let done_path = cfg.done_dir().join(&filename);
    if in_flight_path.exists() || done_path.exists() {
        debug!(
            brief_id = %brief_id,
            "enqueue_shadow: brief already in-flight or done; skipping overwrite"
        );
        return Ok(QueueEntry {
            brief_id: brief_id.clone(),
            queue_path,
        });
    }

    let line = serde_json::to_string(entry).map_err(|e| DoormanError::QueueIo {
        path: queue_path.display().to_string(),
        reason: format!("shadow entry serialisation failed: {e}"),
    })?;

    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&queue_path)
        .map_err(|e| DoormanError::QueueIo {
            path: queue_path.display().to_string(),
            reason: e.to_string(),
        })?;
    f.write_all(line.as_bytes())
        .and_then(|_| f.write_all(b"\n"))
        .and_then(|_| f.flush())
        .map_err(|e| DoormanError::QueueIo {
            path: queue_path.display().to_string(),
            reason: e.to_string(),
        })?;

    info!(
        brief_id = %brief_id,
        queue_path = %queue_path.display(),
        "shadow brief enqueued"
    );
    Ok(QueueEntry {
        brief_id: brief_id.clone(),
        queue_path,
    })
}

/// Atomically take the next available `ShadowQueueEntry` from `queue/`
/// as a lease.
///
/// Same locking and rename semantics as [`dequeue`]; the parsed payload
/// is a `ShadowQueueEntry` (brief + actual_diff) rather than a bare
/// `ApprenticeshipBrief`. Files written by [`enqueue_shadow`] use this
/// path; files written by the legacy [`enqueue`] can also be read here
/// by wrapping them in a `ShadowQueueEntry` with `actual_diff: ""`
/// (backwards-compatible).
///
/// Returns:
/// - `Ok(Some(LeasedShadowEntry))` — a shadow entry was dequeued and leased.
/// - `Ok(None)` — the queue was empty at this moment.
/// - `Err(_)` — I/O or lock failure.
pub fn dequeue_shadow(
    cfg: &QueueConfig,
    worker_id: &str,
) -> QueueResult<Option<LeasedShadowEntry>> {
    ensure_dirs(cfg)?;

    let lock_file = acquire_lock(cfg)?;

    let entry = {
        let mut entries = fs::read_dir(cfg.queue_dir()).map_err(|e| DoormanError::QueueIo {
            path: cfg.queue_dir().display().to_string(),
            reason: e.to_string(),
        })?;

        let mut candidates: Vec<std::path::PathBuf> = Vec::new();
        for entry in entries.by_ref() {
            let e = entry.map_err(|e| DoormanError::QueueIo {
                path: cfg.queue_dir().display().to_string(),
                reason: e.to_string(),
            })?;
            let fname = e.file_name();
            let fname_str = fname.to_string_lossy();
            if fname_str.ends_with(".brief.jsonl") {
                candidates.push(e.path());
            }
        }
        candidates.sort();
        candidates.into_iter().next()
    };

    let Some(queue_path) = entry else {
        drop(lock_file);
        return Ok(None);
    };

    let base_filename = queue_path
        .file_name()
        .expect("queue path has filename")
        .to_string_lossy()
        .into_owned();
    let ts_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let lease_filename = format!(
        "{}.lease.{}.{}",
        base_filename,
        sanitise_worker_id(worker_id),
        ts_nanos
    );
    let lease_path = cfg.in_flight_dir().join(&lease_filename);

    fs::rename(&queue_path, &lease_path).map_err(|e| DoormanError::QueueIo {
        path: format!("{} → {}", queue_path.display(), lease_path.display()),
        reason: e.to_string(),
    })?;

    drop(lock_file);

    let raw = fs::read_to_string(&lease_path).map_err(|e| DoormanError::QueueIo {
        path: lease_path.display().to_string(),
        reason: e.to_string(),
    })?;

    // Try to parse as ShadowQueueEntry first; fall back to bare
    // ApprenticeshipBrief (for entries written by legacy `enqueue()`).
    let shadow_entry: ShadowQueueEntry = match serde_json::from_str::<ShadowQueueEntry>(raw.trim())
    {
        Ok(e) => e,
        Err(_) => {
            // Attempt legacy parse as bare ApprenticeshipBrief.
            match serde_json::from_str::<ApprenticeshipBrief>(raw.trim()) {
                Ok(brief) => ShadowQueueEntry {
                    brief,
                    actual_diff: String::new(),
                },
                Err(e) => {
                    let poison_path = cfg.poison_dir().join(&base_filename);
                    let _ = fs::rename(&lease_path, &poison_path);
                    return Err(DoormanError::QueueMalformedBrief {
                        path: lease_path.display().to_string(),
                        reason: e.to_string(),
                    });
                }
            }
        }
    };

    debug!(
        brief_id = %shadow_entry.brief.brief_id,
        lease_path = %lease_path.display(),
        worker_id,
        "shadow entry dequeued and leased"
    );

    Ok(Some(LeasedShadowEntry {
        entry: shadow_entry,
        worker_id: worker_id.to_string(),
        lease_ts_nanos: ts_nanos,
        lease_path,
        base_filename,
    }))
}

/// Release a `LeasedShadowEntry` lease. Identical semantics to
/// [`release`] but takes the shadow-specific lease handle.
pub fn release_shadow(
    cfg: &QueueConfig,
    lease: &LeasedShadowEntry,
    outcome: ReleaseOutcome,
) -> QueueResult<()> {
    if !lease.lease_path.exists() {
        debug!(
            brief_id = %lease.entry.brief.brief_id,
            "release_shadow: lease file no longer present; skipping"
        );
        return Ok(());
    }

    let dest = match outcome {
        ReleaseOutcome::Done => cfg.done_dir().join(&lease.base_filename),
        ReleaseOutcome::Retry => cfg.queue_dir().join(&lease.base_filename),
        ReleaseOutcome::Poison => cfg.poison_dir().join(&lease.base_filename),
    };

    fs::rename(&lease.lease_path, &dest).map_err(|e| DoormanError::QueueIo {
        path: format!("{} → {}", lease.lease_path.display(), dest.display()),
        reason: e.to_string(),
    })?;

    info!(
        brief_id = %lease.entry.brief.brief_id,
        outcome = ?outcome,
        dest = %dest.display(),
        "shadow entry released"
    );
    Ok(())
}

/// Atomically take the next available brief from `queue/` as a lease.
///
/// Acquires an exclusive `flock(2)` on the queue sentinel, scans `queue/`
/// for the lexicographically first `.brief.jsonl` file, renames it to
/// `queue-in-flight/<brief_id>.brief.jsonl.lease.<worker_id>.<ts_nanos>`,
/// and releases the lock.
///
/// Returns:
/// - `Ok(Some(LeasedBrief))` — a brief was dequeued and leased.
/// - `Ok(None)` — the queue was empty at this moment.
/// - `Err(_)` — I/O or lock failure.
pub fn dequeue(cfg: &QueueConfig, worker_id: &str) -> QueueResult<Option<LeasedBrief>> {
    ensure_dirs(cfg)?;

    // Acquire the queue lock (exclusive, blocking — short critical section).
    let lock_file = acquire_lock(cfg)?;

    // Scan queue/ for the first .brief.jsonl entry (FIFO by filename).
    let entry = {
        let mut entries = fs::read_dir(cfg.queue_dir()).map_err(|e| DoormanError::QueueIo {
            path: cfg.queue_dir().display().to_string(),
            reason: e.to_string(),
        })?;

        let mut candidates: Vec<PathBuf> = Vec::new();
        for entry in entries.by_ref() {
            let e = entry.map_err(|e| DoormanError::QueueIo {
                path: cfg.queue_dir().display().to_string(),
                reason: e.to_string(),
            })?;
            let fname = e.file_name();
            let fname_str = fname.to_string_lossy();
            if fname_str.ends_with(".brief.jsonl") {
                candidates.push(e.path());
            }
        }
        candidates.sort(); // lexicographic = FIFO when brief_id is ULID/UUIDv7
        candidates.into_iter().next()
    };

    let Some(queue_path) = entry else {
        // Lock released on drop.
        drop(lock_file);
        return Ok(None);
    };

    // Construct the lease filename.
    let base_filename = queue_path
        .file_name()
        .expect("queue path has filename")
        .to_string_lossy()
        .into_owned();
    let ts_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let lease_filename = format!(
        "{}.lease.{}.{}",
        base_filename,
        sanitise_worker_id(worker_id),
        ts_nanos
    );
    let lease_path = cfg.in_flight_dir().join(&lease_filename);

    // Rename: queue/ → queue-in-flight/ (atomic on POSIX same-fs).
    fs::rename(&queue_path, &lease_path).map_err(|e| DoormanError::QueueIo {
        path: format!("{} → {}", queue_path.display(), lease_path.display()),
        reason: e.to_string(),
    })?;

    // Release the flock before we do the expensive parse.
    drop(lock_file);

    // Parse the brief from the lease file.
    let raw = fs::read_to_string(&lease_path).map_err(|e| DoormanError::QueueIo {
        path: lease_path.display().to_string(),
        reason: e.to_string(),
    })?;

    let brief: ApprenticeshipBrief = match serde_json::from_str(raw.trim()) {
        Ok(b) => b,
        Err(e) => {
            // Malformed — move to poison immediately.
            let poison_path = cfg.poison_dir().join(&base_filename);
            let _ = fs::rename(&lease_path, &poison_path);
            return Err(DoormanError::QueueMalformedBrief {
                path: lease_path.display().to_string(),
                reason: e.to_string(),
            });
        }
    };

    debug!(
        brief_id = %brief.brief_id,
        lease_path = %lease_path.display(),
        worker_id,
        "brief dequeued and leased"
    );

    Ok(Some(LeasedBrief {
        brief,
        worker_id: worker_id.to_string(),
        lease_ts_nanos: ts_nanos,
        lease_path,
        base_filename,
    }))
}

/// Release a lease, moving the brief file to its final destination based on
/// the [`ReleaseOutcome`].
///
/// - [`ReleaseOutcome::Done`]   → `queue-done/<base_filename>`
/// - [`ReleaseOutcome::Retry`]  → `queue/<base_filename>` (re-enqueue for retry)
/// - [`ReleaseOutcome::Poison`] → `queue-poison/<base_filename>`
pub fn release(cfg: &QueueConfig, lease: &LeasedBrief, outcome: ReleaseOutcome) -> QueueResult<()> {
    if !lease.lease_path.exists() {
        // Already moved (e.g., by reaper). Treat as Done.
        debug!(
            brief_id = %lease.brief.brief_id,
            "release: lease file no longer present; skipping"
        );
        return Ok(());
    }

    let dest = match outcome {
        ReleaseOutcome::Done => cfg.done_dir().join(&lease.base_filename),
        ReleaseOutcome::Retry => cfg.queue_dir().join(&lease.base_filename),
        ReleaseOutcome::Poison => cfg.poison_dir().join(&lease.base_filename),
    };

    fs::rename(&lease.lease_path, &dest).map_err(|e| DoormanError::QueueIo {
        path: format!("{} → {}", lease.lease_path.display(), dest.display()),
        reason: e.to_string(),
    })?;

    info!(
        brief_id = %lease.brief.brief_id,
        outcome = ?outcome,
        dest = %dest.display(),
        "brief released"
    );
    Ok(())
}

/// Reap lease files older than `max_age` in `queue-in-flight/`, returning
/// them to `queue/` for retry by the next available worker.
///
/// This is the crash-safety mechanism: if a Doorman process exits while
/// holding a lease (SIGKILL, panic, VM preemption), the lease file remains
/// in `queue-in-flight/`. The next `reap_expired_leases` call will return
/// the brief to the queue.
///
/// Returns the number of leases that were reclaimed.
pub fn reap_expired_leases(cfg: &QueueConfig, max_age: Duration) -> QueueResult<usize> {
    ensure_dirs(cfg)?;

    // Acquire the queue lock so the reaper and a concurrent dequeue do not
    // race on the same lease file.
    let lock_file = acquire_lock(cfg)?;

    let now_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let max_age_nanos = max_age.as_nanos();

    let entries = fs::read_dir(cfg.in_flight_dir()).map_err(|e| DoormanError::QueueIo {
        path: cfg.in_flight_dir().display().to_string(),
        reason: e.to_string(),
    })?;

    let mut reclaimed = 0usize;
    for entry in entries {
        let e = entry.map_err(|e| DoormanError::QueueIo {
            path: cfg.in_flight_dir().display().to_string(),
            reason: e.to_string(),
        })?;
        let fname = e.file_name();
        let fname_str = fname.to_string_lossy().into_owned();

        // Lease filenames: `<base>.lease.<worker_id>.<ts_nanos>`
        if let Some(ts_nanos) = parse_lease_ts(&fname_str) {
            let age_nanos = now_nanos.saturating_sub(ts_nanos);
            if age_nanos >= max_age_nanos {
                // Extract the base filename by stripping `.lease.<worker_id>.<ts>`.
                if let Some(base) = extract_base_filename(&fname_str) {
                    let lease_path = cfg.in_flight_dir().join(&fname_str);
                    let queue_path = cfg.queue_dir().join(&base);
                    match fs::rename(&lease_path, &queue_path) {
                        Ok(()) => {
                            reclaimed += 1;
                            info!(
                                lease_file = %fname_str,
                                age_secs = age_nanos / 1_000_000_000,
                                "expired lease reclaimed to queue"
                            );
                        }
                        Err(e) => {
                            warn!(
                                lease_file = %fname_str,
                                error = %e,
                                "failed to reclaim expired lease; will retry next reap cycle"
                            );
                        }
                    }
                }
            }
        }
    }

    drop(lock_file);
    Ok(reclaimed)
}

// ── Private helpers ───────────────────────────────────────────────────────────

fn brief_filename(brief_id: &str) -> String {
    format!("{brief_id}.brief.jsonl")
}

/// Acquire an exclusive flock(2) on the queue sentinel.  Non-blocking
/// variant: returns `QueueLockFailed` immediately rather than blocking
/// indefinitely. The drain worker retries on the next poll interval.
fn acquire_lock(cfg: &QueueConfig) -> QueueResult<File> {
    ensure_dirs(cfg)?;
    let path = cfg.lock_sentinel();
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&path)
        .map_err(|e| DoormanError::QueueIo {
            path: path.display().to_string(),
            reason: format!("could not open lock sentinel: {e}"),
        })?;

    // try_lock_exclusive: returns EWOULDBLOCK immediately if another process
    // holds the lock; we surface this as QueueLockFailed.
    file.try_lock_exclusive()
        .map_err(|e| DoormanError::QueueLockFailed {
            path: path.display().to_string(),
            reason: e.to_string(),
        })?;

    Ok(file)
}

/// Sanitise a worker ID so it is safe to embed in a filename.
/// Replace path-separator characters and other unsafe chars with `_`.
fn sanitise_worker_id(worker_id: &str) -> String {
    worker_id
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Extract the nanosecond timestamp from a lease filename.
///
/// Format: `<base>.lease.<worker_id>.<ts_nanos>`
fn parse_lease_ts(filename: &str) -> Option<u128> {
    // Find `.lease.` marker.
    let lease_marker = ".lease.";
    let lease_pos = filename.find(lease_marker)?;
    let after_lease = &filename[lease_pos + lease_marker.len()..];
    // The last `.`-delimited component is the timestamp.
    let ts_str = after_lease.rsplit('.').next()?;
    ts_str.parse::<u128>().ok()
}

/// Extract the base filename (`<brief_id>.brief.jsonl`) from a lease filename.
///
/// Format: `<base>.lease.<worker_id>.<ts_nanos>`
fn extract_base_filename(lease_filename: &str) -> Option<String> {
    let lease_marker = ".lease.";
    let lease_pos = lease_filename.find(lease_marker)?;
    Some(lease_filename[..lease_pos].to_string())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use slm_core::{ApprenticeshipBrief, BriefScope, SeniorRole};

    fn tmp_queue(label: &str) -> QueueConfig {
        let base = std::env::temp_dir().join(format!(
            "slm-queue-test-{label}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        QueueConfig::with_base_dir(base)
    }

    fn make_brief(brief_id: &str) -> ApprenticeshipBrief {
        ApprenticeshipBrief {
            brief_id: brief_id.to_string(),
            created: Utc::now(),
            senior_role: SeniorRole::Master,
            senior_identity: "ps-administrator".to_string(),
            task_type: "version-bump-manifest".to_string(),
            scope: BriefScope {
                cluster: Some("project-slm".to_string()),
                files: vec![],
            },
            acceptance_test: "Tests pass.".to_string(),
            doctrine_citations: vec![],
            shadow: true,
            body: "Bump MANIFEST.md version.".to_string(),
        }
    }

    /// Enqueue a brief, dequeue it, verify file moved to queue-in-flight/ with
    /// the correct lease filename pattern, verify the returned LeasedBrief
    /// matches the original brief.
    #[test]
    fn enqueue_dequeue_round_trip() {
        let cfg = tmp_queue("round-trip");
        let brief = make_brief("01J9QUEUETEST0000000000001");

        // Enqueue.
        let entry = enqueue(&cfg, &brief).expect("enqueue ok");
        assert!(
            entry.queue_path.exists(),
            "brief file must exist in queue/ after enqueue"
        );

        // Dequeue.
        let leased = dequeue(&cfg, "worker-0")
            .expect("dequeue ok")
            .expect("queue must return the enqueued brief");

        // Brief is no longer in queue/.
        assert!(
            !entry.queue_path.exists(),
            "queue/ file must be gone after dequeue"
        );

        // Lease file exists in queue-in-flight/.
        assert!(
            leased.lease_path.exists(),
            "lease file must exist in queue-in-flight/"
        );
        assert!(
            leased.lease_path.starts_with(cfg.in_flight_dir()),
            "lease file must be inside queue-in-flight/"
        );

        // Lease filename embeds worker_id and a numeric timestamp.
        let lease_name = leased
            .lease_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        assert!(
            lease_name.contains(".lease."),
            "lease filename must contain .lease. marker"
        );
        assert!(
            lease_name.contains("worker-0"),
            "lease filename must contain the worker_id"
        );

        // Contents match.
        assert_eq!(leased.brief.brief_id, brief.brief_id);
        assert_eq!(leased.brief.task_type, brief.task_type);
        assert_eq!(leased.brief.body, brief.body);
    }

    /// Write a brief, dequeue (creates lease), then rename the lease file
    /// to embed an ancient timestamp (ts_nanos = 1 = epoch + 1 ns), call
    /// reap_expired_leases with a 1ms max_age, verify brief returns to queue/.
    ///
    /// Renaming the lease to an old-timestamp variant simulates "operator
    /// restarted the Doorman and the lease was left behind from a prior run
    /// that started immediately after the UNIX epoch" — a worst-case
    /// correctness scenario, and the same pattern the
    /// `queue_drain_resumes_after_doorman_restart` test uses for the
    /// manually-created lease.
    #[test]
    fn lease_expiration_returns_brief_to_queue() {
        let cfg = tmp_queue("lease-expire");
        let brief = make_brief("01J9QUEUETEST0000000000002");

        enqueue(&cfg, &brief).expect("enqueue ok");
        let leased = dequeue(&cfg, "worker-expiry")
            .expect("dequeue ok")
            .expect("queue not empty");

        // The lease file was just created with a current timestamp.
        // Rename it so its embedded timestamp is 1 (epoch + 1 ns = ancient).
        let ancient_lease_filename = format!("{}.lease.worker-expiry.1", leased.base_filename);
        let ancient_lease_path = cfg.in_flight_dir().join(&ancient_lease_filename);
        fs::rename(&leased.lease_path, &ancient_lease_path)
            .expect("rename lease to ancient timestamp");

        // Reap with 1ms max_age — the epoch+1ns lease is definitively expired.
        let reclaimed = reap_expired_leases(&cfg, Duration::from_millis(1)).expect("reap ok");
        assert_eq!(reclaimed, 1, "exactly one expired lease must be reclaimed");

        // Brief is back in queue/.
        let queue_file = cfg.queue_dir().join(&leased.base_filename);
        assert!(
            queue_file.exists(),
            "brief must be back in queue/ after lease expiration reap"
        );

        // Ancient lease file is gone from queue-in-flight/.
        assert!(
            !ancient_lease_path.exists(),
            "ancient lease file must be removed from queue-in-flight/ after reap"
        );
    }

    /// Write a brief, spawn two concurrent dequeue() calls, verify exactly ONE
    /// returns Ok(Some(_)) and the other returns Ok(None). Lease file count == 1.
    #[test]
    fn concurrent_workers_dont_double_lease() {
        let cfg = tmp_queue("concurrent");
        let brief = make_brief("01J9QUEUETEST0000000000003");

        enqueue(&cfg, &brief).expect("enqueue ok");

        // Run two concurrent dequeue calls on std::thread (queue is sync).
        let cfg_a = cfg.clone();
        let cfg_b = cfg.clone();

        let handle_a = std::thread::spawn(move || dequeue(&cfg_a, "worker-A"));
        let handle_b = std::thread::spawn(move || dequeue(&cfg_b, "worker-B"));

        let result_a = handle_a.join().expect("thread A joined");
        let result_b = handle_b.join().expect("thread B joined");

        // QueueLockFailed is expected when both workers race — treat as "missed".
        let got_a = match result_a {
            Ok(brief) => brief,
            Err(DoormanError::QueueLockFailed { .. }) => None,
            Err(e) => panic!("worker A unexpected error: {}", e),
        };
        let got_b = match result_b {
            Ok(brief) => brief,
            Err(DoormanError::QueueLockFailed { .. }) => None,
            Err(e) => panic!("worker B unexpected error: {}", e),
        };

        // Exactly one should have received the brief.
        let success_count = [got_a.is_some(), got_b.is_some()]
            .iter()
            .filter(|&&b| b)
            .count();
        assert_eq!(
            success_count,
            1,
            "exactly one worker must dequeue the brief; got_a={}, got_b={}",
            got_a.is_some(),
            got_b.is_some()
        );

        // Verify the lease file count in queue-in-flight/.
        let lease_count = fs::read_dir(cfg.in_flight_dir())
            .expect("read in-flight dir")
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().contains(".lease."))
            .count();
        assert_eq!(
            lease_count, 1,
            "exactly one lease file must exist post-test"
        );
    }

    /// Write a malformed brief file (invalid JSON), dequeue + parse fails,
    /// verify the file moved to queue-poison/. Subsequent dequeue returns None.
    #[test]
    fn poison_bucket_catches_malformed_brief() {
        let cfg = tmp_queue("poison");
        ensure_dirs(&cfg).expect("ensure dirs");

        // Write a malformed file directly into queue/.
        let malformed_id = "01J9QUEUETEST0000000000004";
        let malformed_path = cfg.queue_dir().join(brief_filename(malformed_id));
        fs::write(&malformed_path, b"{ this is NOT valid json \n").expect("write malformed");

        // dequeue() should return QueueMalformedBrief.
        let result = dequeue(&cfg, "worker-poison");
        match result {
            Err(DoormanError::QueueMalformedBrief { .. }) => {} // expected
            Err(e) => panic!("expected QueueMalformedBrief, got {e:?}"),
            Ok(Some(_)) => panic!("expected error for malformed brief, got Some"),
            Ok(None) => panic!("expected error for malformed brief, got None"),
        }

        // Malformed file must be in queue-poison/.
        let poison_path = cfg.poison_dir().join(brief_filename(malformed_id));
        assert!(
            poison_path.exists(),
            "malformed brief must be in queue-poison/"
        );

        // queue/ and queue-in-flight/ must be empty (except possible lock file).
        let queue_briefs: Vec<_> = fs::read_dir(cfg.queue_dir())
            .expect("read queue dir")
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().ends_with(".brief.jsonl"))
            .collect();
        assert!(
            queue_briefs.is_empty(),
            "queue/ must be empty after poison (no brief to retry)"
        );

        // Subsequent dequeue returns None.
        let second = dequeue(&cfg, "worker-poison-2").expect("second dequeue ok");
        assert!(
            second.is_none(),
            "second dequeue must return None (queue empty; poison'd file not re-attempted)"
        );
    }

    /// Write 3 briefs to queue/. Simulate "worker started" by manually creating
    /// a lease for one in queue-in-flight/ with an old timestamp. Simulate restart
    /// by calling reap_expired_leases. Verify all 3 are in queue/ ready for fresh
    /// workers.
    #[test]
    fn queue_drain_resumes_after_doorman_restart() {
        let cfg = tmp_queue("restart");
        ensure_dirs(&cfg).expect("ensure dirs");

        let ids = [
            "01J9QUEUETEST0000000000005",
            "01J9QUEUETEST0000000000006",
            "01J9QUEUETEST0000000000007",
        ];

        // Enqueue all 3.
        for id in &ids {
            enqueue(&cfg, &make_brief(id)).expect("enqueue ok");
        }
        assert_eq!(
            fs::read_dir(cfg.queue_dir())
                .unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().ends_with(".brief.jsonl"))
                .count(),
            3,
            "3 briefs in queue/ before simulated worker"
        );

        // Simulate "worker started" by manually creating an old-timestamp lease
        // for the first brief in queue-in-flight/.
        let first_id = ids[0];
        let base = brief_filename(first_id);
        let src = cfg.queue_dir().join(&base);
        // ts_nanos = 1 (epoch + 1 nanosecond = extremely old)
        let old_lease_name = format!("{base}.lease.crashed-worker.1");
        let lease_path = cfg.in_flight_dir().join(&old_lease_name);
        fs::rename(&src, &lease_path).expect("simulate lease creation");

        // Now 2 briefs in queue/, 1 in queue-in-flight/.
        assert_eq!(
            fs::read_dir(cfg.queue_dir())
                .unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().ends_with(".brief.jsonl"))
                .count(),
            2
        );
        assert_eq!(
            fs::read_dir(cfg.in_flight_dir())
                .unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().contains(".lease."))
                .count(),
            1
        );

        // Reap with max_age = 1ms — the epoch+1ns lease is ancient.
        let reclaimed = reap_expired_leases(&cfg, Duration::from_millis(1)).expect("reap ok");
        assert_eq!(reclaimed, 1, "the crashed worker's lease must be reclaimed");

        // All 3 briefs back in queue/.
        let queue_count = fs::read_dir(cfg.queue_dir())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().ends_with(".brief.jsonl"))
            .count();
        assert_eq!(queue_count, 3, "all 3 briefs must be in queue/ after reap");

        // queue-in-flight/ must be empty of leases.
        let inflight_count = fs::read_dir(cfg.in_flight_dir())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().contains(".lease."))
            .count();
        assert_eq!(
            inflight_count, 0,
            "queue-in-flight/ must be empty after reap"
        );
    }
}
