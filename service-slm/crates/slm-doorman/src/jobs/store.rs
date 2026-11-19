// SPDX-License-Identifier: Apache-2.0

//! Durable job buffer. Layout under the base dir (mode 0700):
//! `inputs/<ref>.bin` + `inputs/<ref>.meta.json`, `jobs/<id>.json`,
//! `results/<id>.json`. Every write is tmp-file + rename. File names are
//! built only from parsed `JobId`/`StagedRef` values (strict UUIDs).

use std::io;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use slm_core::ModuleId;

use super::types::{JobId, JobRecord, JobResult, StagedRef};

#[derive(Clone, Debug)]
pub struct JobStore {
    base: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputMeta {
    pub module_id: ModuleId,
    pub sha256: String,
    pub size: u64,
    pub media_type: String,
    pub created_at: DateTime<Utc>,
}

fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let tmp = path.with_extension(format!(
        "{}.tmp",
        path.extension().and_then(|e| e.to_str()).unwrap_or("")
    ));
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path)
}

impl JobStore {
    pub fn new(base: PathBuf) -> io::Result<Self> {
        for d in ["inputs", "jobs", "results"] {
            std::fs::create_dir_all(base.join(d))?;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for d in ["", "inputs", "jobs", "results"] {
                std::fs::set_permissions(base.join(d), std::fs::Permissions::from_mode(0o700))?;
            }
        }
        Ok(Self { base })
    }

    fn input_bin(&self, r: StagedRef) -> PathBuf {
        self.base.join("inputs").join(format!("{r}.bin"))
    }
    fn input_meta(&self, r: StagedRef) -> PathBuf {
        self.base.join("inputs").join(format!("{r}.meta.json"))
    }
    fn job_path(&self, id: JobId) -> PathBuf {
        self.base.join("jobs").join(format!("{id}.json"))
    }
    fn result_path(&self, id: JobId) -> PathBuf {
        self.base.join("results").join(format!("{id}.json"))
    }

    pub fn put_input(&self, meta: &InputMeta, bytes: &[u8]) -> io::Result<StagedRef> {
        let r = StagedRef::new();
        write_atomic(&self.input_bin(r), bytes)?;
        write_atomic(&self.input_meta(r), &serde_json::to_vec(meta)?)?;
        Ok(r)
    }

    pub fn input_meta_of(&self, r: StagedRef) -> io::Result<InputMeta> {
        Ok(serde_json::from_slice(&std::fs::read(self.input_meta(r))?)?)
    }

    pub fn read_input(&self, r: StagedRef) -> io::Result<Vec<u8>> {
        std::fs::read(self.input_bin(r))
    }

    pub fn delete_input(&self, r: StagedRef) {
        let _ = std::fs::remove_file(self.input_bin(r));
        let _ = std::fs::remove_file(self.input_meta(r));
    }

    pub fn save_job(&self, rec: &JobRecord) -> io::Result<()> {
        write_atomic(&self.job_path(rec.job_id), &serde_json::to_vec(rec)?)
    }

    pub fn load_job(&self, id: JobId) -> io::Result<JobRecord> {
        Ok(serde_json::from_slice(&std::fs::read(self.job_path(id))?)?)
    }

    pub fn save_result(&self, res: &JobResult) -> io::Result<()> {
        write_atomic(&self.result_path(res.job_id), &serde_json::to_vec(res)?)
    }

    pub fn load_result(&self, id: JobId) -> io::Result<JobResult> {
        Ok(serde_json::from_slice(&std::fs::read(
            self.result_path(id),
        )?)?)
    }

    pub fn delete_result(&self, id: JobId) {
        let _ = std::fs::remove_file(self.result_path(id));
    }

    /// Staged inputs with their metadata. Entries whose names are not strict
    /// UUIDs or whose metadata cannot be parsed are skipped.
    pub fn list_inputs(&self) -> io::Result<Vec<(StagedRef, InputMeta)>> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(self.base.join("inputs"))? {
            let path = entry?.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            let Some(stem) = name.strip_suffix(".meta.json") else {
                continue;
            };
            let Ok(r) = stem.parse::<StagedRef>() else {
                continue;
            };
            if let Ok(meta) = self.input_meta_of(r) {
                out.push((r, meta));
            }
        }
        Ok(out)
    }

    /// Total bytes currently held in the input and result buffers.
    pub fn buffer_bytes(&self) -> u64 {
        let mut total = 0u64;
        for d in ["inputs", "results"] {
            if let Ok(rd) = std::fs::read_dir(self.base.join(d)) {
                for e in rd.flatten() {
                    if let Ok(m) = e.metadata() {
                        total += m.len();
                    }
                }
            }
        }
        total
    }

    /// Every job status file, oldest first (UUIDv7 names sort by time).
    /// Unparseable files are skipped.
    pub fn list_jobs(&self) -> io::Result<Vec<JobRecord>> {
        let mut recs = Vec::new();
        for entry in std::fs::read_dir(self.base.join("jobs"))? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Ok(bytes) = std::fs::read(&path) {
                if let Ok(rec) = serde_json::from_slice::<JobRecord>(&bytes) {
                    recs.push(rec);
                }
            }
        }
        recs.sort_by_key(|r| r.job_id.to_string());
        Ok(recs)
    }
}
