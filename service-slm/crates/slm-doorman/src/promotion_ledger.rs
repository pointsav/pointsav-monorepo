// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Apprenticeship promotion ledger writer (AS-3).
//!
//! Per `~/Foundry/conventions/apprenticeship-substrate.md` §6 the
//! promotion ledger is a single plain-text file at
//! `data/apprenticeship/ledger.md`, append-only, with each event
//! carrying an embedded SSH signature block. The Doorman appends only
//! after `ssh-keygen -Y verify` has accepted the senior's verdict
//! signature.
//!
//! Concurrency: `flock(2)` exclusive on
//! `data/apprenticeship/.ledger.lock` during append (design-pass Q3).
//! `fs2::FileExt::lock_exclusive` is used for the lock; the lock file
//! is created if missing. Single-writer serialisation across
//! processes; OK at the expected ≤tens-per-day verdict rate. SQLite
//! WAL is the v0.5+ upgrade once verdict rate exceeds the SQLite
//! crossover.
//!
//! The ledger.md file is the human-readable / signed form. Rolling
//! statistics for promotion-threshold computation come from a sidecar
//! JSONL at `data/apprenticeship/.stats.jsonl` written under the same
//! flock — keeps stat reads cheap and avoids re-parsing the markdown
//! on every verdict. Stages are persisted in
//! `data/apprenticeship/stages.json` and updated atomically when a
//! threshold crossing fires a promotion event.

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use slm_core::VerdictOutcome;
use tracing::{info, warn};

use crate::error::{DoormanError, Result};

/// Promotion stage per task-type (convention §2).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Stage {
    #[default]
    Review,
    SpotCheck,
    Autonomous,
}

impl Stage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Stage::Review => "review",
            Stage::SpotCheck => "spot-check",
            Stage::Autonomous => "autonomous",
        }
    }
}

/// Outcome of a single verdict append. Surfaced to the caller so the
/// HTTP layer can include it in the verdict response body.
#[derive(Clone, Debug, Serialize)]
pub struct PromotionOutcome {
    pub task_type: String,
    pub stage_before: Stage,
    pub stage_after: Stage,
    pub n_verdicts: usize,
    pub accept_rate: f32,
    pub promoted: bool,
}

/// One row of the sidecar stats log. Mirrors the data the convention
/// §6 promotion rules reference (verdict outcome + recency).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatRow {
    pub ts: DateTime<Utc>,
    pub task_type: String,
    pub verdict: VerdictOutcome,
    pub brief_id: String,
    pub attempt_id: String,
    pub self_confidence: f32,
    pub senior_identity: String,
}

#[derive(Clone, Debug)]
pub struct PromotionLedger {
    /// Directory holding ledger.md, .stats.jsonl, .ledger.lock, stages.json.
    dir: PathBuf,
}

impl PromotionLedger {
    pub fn new(dir: impl Into<PathBuf>) -> Result<Self> {
        let dir: PathBuf = dir.into();
        std::fs::create_dir_all(&dir)
            .map_err(|e| DoormanError::LedgerLock(format!("mkdir {}: {e}", dir.display())))?;
        Ok(Self { dir })
    }

    pub fn ledger_md_path(&self) -> PathBuf {
        self.dir.join("ledger.md")
    }
    pub fn stats_path(&self) -> PathBuf {
        self.dir.join(".stats.jsonl")
    }
    pub fn lock_path(&self) -> PathBuf {
        self.dir.join(".ledger.lock")
    }
    pub fn stages_path(&self) -> PathBuf {
        self.dir.join("stages.json")
    }

    /// Read current stage for a task-type (defaults to Review).
    pub fn current_stage(&self, task_type: &str) -> Stage {
        read_stages(&self.stages_path())
            .get(task_type)
            .copied()
            .unwrap_or_default()
    }

    /// Append one verdict. Writes the markdown event row to ledger.md,
    /// the stats row to .stats.jsonl, and (on threshold cross) a
    /// `promotion` event row. All four-or-fewer writes happen under a
    /// single flock(2). Returns the resulting `PromotionOutcome`.
    pub fn append_verdict(
        &self,
        row: StatRow,
        signed_event_block: &str,
    ) -> Result<PromotionOutcome> {
        let lock_path = self.lock_path();
        let lock_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&lock_path)
            .map_err(|e| DoormanError::LedgerLock(format!("open lock: {e}")))?;
        lock_file
            .lock_exclusive()
            .map_err(|e| DoormanError::LedgerLock(format!("flock: {e}")))?;
        let res = self.append_under_lock(&row, signed_event_block);
        let _ = FileExt::unlock(&lock_file);
        res
    }

    fn append_under_lock(
        &self,
        row: &StatRow,
        signed_event_block: &str,
    ) -> Result<PromotionOutcome> {
        // 1. Append the markdown event row.
        append_text(&self.ledger_md_path(), signed_event_block)?;

        // 2. Append the stats row.
        let mut stats_line = serde_json::to_string(row).map_err(|e| DoormanError::CorpusWrite {
            path: self.stats_path().display().to_string(),
            reason: e.to_string(),
        })?;
        stats_line.push('\n');
        append_text(&self.stats_path(), &stats_line)?;

        // 3. Compute rolling stats for this task-type.
        let stage_before = self.current_stage(&row.task_type);
        let (accept_rate, n) = rolling_stats(&self.stats_path(), &row.task_type, 100);
        let stage_after = next_stage(stage_before, accept_rate, n);
        let promoted = stage_after != stage_before;

        if promoted {
            // 4. Append a promotion event row.
            let promotion_block = format_promotion_event(
                &row.task_type,
                stage_before,
                stage_after,
                accept_rate,
                n,
                row.ts,
            );
            append_text(&self.ledger_md_path(), &promotion_block)?;

            // 5. Persist stage transition.
            let mut map = read_stages(&self.stages_path());
            map.insert(row.task_type.clone(), stage_after);
            write_stages(&self.stages_path(), &map)?;
            info!(
                target: "slm_doorman::ledger",
                task_type = %row.task_type,
                old = stage_before.as_str(),
                new = stage_after.as_str(),
                accept_rate,
                n,
                "task-type promoted"
            );
        }

        Ok(PromotionOutcome {
            task_type: row.task_type.clone(),
            stage_before,
            stage_after,
            n_verdicts: n,
            accept_rate,
            promoted,
        })
    }
}

fn append_text(path: &Path, body: &str) -> Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| DoormanError::CorpusWrite {
            path: path.display().to_string(),
            reason: e.to_string(),
        })?;
    f.write_all(body.as_bytes())
        .map_err(|e| DoormanError::CorpusWrite {
            path: path.display().to_string(),
            reason: e.to_string(),
        })?;
    f.flush().map_err(|e| DoormanError::CorpusWrite {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    Ok(())
}

fn read_stages(path: &Path) -> HashMap<String, Stage> {
    let body = match std::fs::read_to_string(path) {
        Ok(s) if !s.trim().is_empty() => s,
        _ => return HashMap::new(),
    };
    serde_json::from_str(&body).unwrap_or_else(|e| {
        warn!(target: "slm_doorman::ledger", error = %e, path = %path.display(),
              "stages.json malformed; treating as empty");
        HashMap::new()
    })
}

fn write_stages(path: &Path, map: &HashMap<String, Stage>) -> Result<()> {
    let body = serde_json::to_string_pretty(map).map_err(|e| DoormanError::CorpusWrite {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, body).map_err(|e| DoormanError::CorpusWrite {
        path: tmp.display().to_string(),
        reason: e.to_string(),
    })?;
    std::fs::rename(&tmp, path).map_err(|e| DoormanError::CorpusWrite {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;
    Ok(())
}

/// Rolling accept-rate over the most recent verdicts for `task_type`.
/// Returns `(accept_rate, n_verdicts)`. `cap` bounds how far back we
/// look (convention §2 thresholds use rolling 50 / rolling 100).
pub fn rolling_stats(stats_path: &Path, task_type: &str, cap: usize) -> (f32, usize) {
    let body = match std::fs::read_to_string(stats_path) {
        Ok(s) => s,
        Err(_) => return (0.0, 0),
    };
    let rows: Vec<StatRow> = body
        .lines()
        .rev()
        .filter_map(|line| serde_json::from_str::<StatRow>(line).ok())
        .filter(|r| r.task_type == task_type)
        .take(cap)
        .collect();
    if rows.is_empty() {
        return (0.0, 0);
    }
    let n = rows.len();
    let accepts = rows
        .iter()
        .filter(|r| r.verdict == VerdictOutcome::Accept)
        .count();
    (accepts as f32 / n as f32, n)
}

/// Apply the convention §2 promotion rules. Returns the post-rule
/// stage; `next_stage(old, ...) == old` means no transition.
pub fn next_stage(old: Stage, accept_rate: f32, n: usize) -> Stage {
    match old {
        Stage::Review if n >= 50 && accept_rate >= 0.85 => Stage::SpotCheck,
        Stage::SpotCheck if n >= 100 && accept_rate >= 0.95 => Stage::Autonomous,
        _ => old,
    }
}

fn format_promotion_event(
    task_type: &str,
    old: Stage,
    new: Stage,
    accept_rate: f32,
    n: usize,
    ts: DateTime<Utc>,
) -> String {
    format!(
        "\n{ts}  promotion  {task_type}  doorman-auto\n  \
         {old} -> {new} (auto-derived from rolling accept-rate \
         {accept_rate:.3} over n={n})\n  \
         (no senior signature — promotion is a derived event;\n  \
         the upstream verdict rows carry the senior's signature.)\n\n",
        ts = ts.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        old = old.as_str(),
        new = new.as_str(),
    )
}

/// Compose a markdown event block matching the convention §6 layout.
/// The verdict signature block is embedded inline (the senior's signed
/// SSH signature over the verdict body).
pub fn format_verdict_event(
    ts: DateTime<Utc>,
    task_type: &str,
    senior_identity: &str,
    body_summary: &str,
    signature_pem: &str,
) -> String {
    format!(
        "\n{ts}  verdict-batch  {task_type}  {senior_identity}\n  \
         {body_summary}\n\n  \
         -----BEGIN SSH SIGNATURE-----\n  {sig_indented}\n  \
         -----END SSH SIGNATURE-----\n\n",
        ts = ts.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        sig_indented = signature_pem.replace('\n', "\n  "),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use std::path::PathBuf;

    fn tmp_dir(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "slm-doorman-promotion-{label}-{}",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn row(verdict: VerdictOutcome, ts: DateTime<Utc>, n: usize) -> StatRow {
        StatRow {
            ts,
            task_type: "version-bump-manifest".into(),
            verdict,
            brief_id: format!("brief-{n}"),
            attempt_id: format!("attempt-{n}"),
            self_confidence: 0.9,
            senior_identity: "ps-administrator".into(),
        }
    }

    fn fake_event_block(n: usize, verdict: VerdictOutcome) -> String {
        format!(
            "\n2026-04-26T15:55:00Z  verdict-batch  version-bump-manifest  ps-administrator\n  \
             N=1 verdicts: brief-{n} attempt-{n} verdict={verdict_str}\n  \
             -----BEGIN SSH SIGNATURE-----\n  AAAATEST==\n  -----END SSH SIGNATURE-----\n\n",
            verdict_str = verdict.as_str(),
        )
    }

    #[test]
    fn next_stage_threshold_rules() {
        // n < 50 — never promotes.
        assert_eq!(next_stage(Stage::Review, 1.0, 49), Stage::Review);
        // n >= 50, rate < 0.85 — does not promote.
        assert_eq!(next_stage(Stage::Review, 0.84, 60), Stage::Review);
        // n >= 50, rate >= 0.85 — promote review -> spot-check.
        assert_eq!(next_stage(Stage::Review, 0.85, 50), Stage::SpotCheck);
        // spot-check -> autonomous needs n >= 100, rate >= 0.95.
        assert_eq!(next_stage(Stage::SpotCheck, 0.95, 99), Stage::SpotCheck);
        assert_eq!(next_stage(Stage::SpotCheck, 0.95, 100), Stage::Autonomous);
        // autonomous stays autonomous.
        assert_eq!(next_stage(Stage::Autonomous, 1.0, 200), Stage::Autonomous);
    }

    /// Synthesize 50 accept verdicts above the 0.85 rate; the 51st
    /// accept must trigger a `promotion` event in ledger.md and
    /// flip stages.json from review -> spot-check.
    #[test]
    fn promotes_review_to_spot_check_on_50_accepts() {
        let dir = tmp_dir("promote");
        let ledger = PromotionLedger::new(&dir).unwrap();
        let base = Utc::now();
        // Pre-populate 49 stat rows so the 50th verdict crosses.
        for i in 0..49 {
            let r = row(
                VerdictOutcome::Accept,
                base + Duration::seconds(i as i64),
                i,
            );
            let block = fake_event_block(i, VerdictOutcome::Accept);
            ledger.append_verdict(r, &block).unwrap();
        }
        // 49th append did NOT promote (n = 49 below threshold).
        assert_eq!(ledger.current_stage("version-bump-manifest"), Stage::Review);

        let r = row(VerdictOutcome::Accept, base + Duration::seconds(50), 50);
        let block = fake_event_block(50, VerdictOutcome::Accept);
        let outcome = ledger.append_verdict(r, &block).unwrap();
        assert!(outcome.promoted, "50 accepts at 1.0 rate must promote");
        assert_eq!(outcome.stage_before, Stage::Review);
        assert_eq!(outcome.stage_after, Stage::SpotCheck);
        assert_eq!(
            ledger.current_stage("version-bump-manifest"),
            Stage::SpotCheck
        );

        let md = std::fs::read_to_string(ledger.ledger_md_path()).unwrap();
        assert!(
            md.contains("promotion  version-bump-manifest  doorman-auto"),
            "ledger.md must carry a promotion event row\n--- got:\n{md}"
        );
        assert!(md.contains("review -> spot-check"));
    }

    /// Mixed verdicts below 0.85 rate must NOT promote.
    #[test]
    fn mixed_verdicts_below_rate_do_not_promote() {
        let dir = tmp_dir("no-promote");
        let ledger = PromotionLedger::new(&dir).unwrap();
        let base = Utc::now();
        for i in 0..50 {
            // 40 accept + 10 reject = 0.80 < 0.85
            let v = if i < 40 {
                VerdictOutcome::Accept
            } else {
                VerdictOutcome::Reject
            };
            let r = row(v, base + Duration::seconds(i as i64), i);
            let block = fake_event_block(i, v);
            let out = ledger.append_verdict(r, &block).unwrap();
            assert!(!out.promoted, "below-rate must not promote at i={i}");
        }
        assert_eq!(ledger.current_stage("version-bump-manifest"), Stage::Review);
    }

    /// rolling_stats counts only verdicts for the requested task-type.
    #[test]
    fn rolling_stats_filters_by_task_type() {
        let dir = tmp_dir("filter");
        let ledger = PromotionLedger::new(&dir).unwrap();
        let base = Utc::now();
        // 10 accepts on task-A, 10 rejects on task-B
        for i in 0..10 {
            let mut r = row(
                VerdictOutcome::Accept,
                base + Duration::seconds(i as i64),
                i,
            );
            r.task_type = "task-A".into();
            ledger
                .append_verdict(r, &fake_event_block(i, VerdictOutcome::Accept))
                .unwrap();
        }
        for i in 0..10 {
            let mut r = row(
                VerdictOutcome::Reject,
                base + Duration::seconds((100 + i) as i64),
                i,
            );
            r.task_type = "task-B".into();
            ledger
                .append_verdict(r, &fake_event_block(100 + i, VerdictOutcome::Reject))
                .unwrap();
        }
        let (rate_a, n_a) = rolling_stats(&ledger.stats_path(), "task-A", 100);
        let (rate_b, n_b) = rolling_stats(&ledger.stats_path(), "task-B", 100);
        assert_eq!(n_a, 10);
        assert_eq!(n_b, 10);
        assert!((rate_a - 1.0).abs() < 1e-6);
        assert!((rate_b - 0.0).abs() < 1e-6);
    }
}
