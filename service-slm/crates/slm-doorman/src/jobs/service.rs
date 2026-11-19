// SPDX-License-Identifier: Apache-2.0

//! Job lifecycle: submit, queue, run, record, ack. One queue per engine and
//! `permits` worker tasks per engine; a job only ever runs on the engine it
//! names.

use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use chrono::Utc;
use tokio::sync::Notify;
use tracing::{error, warn};

use super::credentials::Credential;
use super::engines::{EngineError, EngineRequest, JobEngine};
use super::prompts;
use super::registry::{EngineKind, EngineSpec, Registry};
use super::sha256_hex;
use super::store::{InputMeta, JobStore};
use super::types::{
    Entity, InputRef, JobId, JobKind, JobRecord, JobResult, JobStatus, ResultRecord, StagedRef,
    SubmitRequest,
};
use super::witness;
use crate::ledger::JobAuditEntry;

pub type AuditSink = Arc<dyn Fn(JobAuditEntry) + Send + Sync>;

#[derive(Clone, Debug)]
pub struct JobsConfig {
    /// Cap for `POST /v1/jobs/inputs` bodies.
    pub max_upload_bytes: u64,
    /// Total bytes the input + result buffers may hold; uploads beyond it
    /// are refused (`BufferFull`).
    pub max_buffer_bytes: u64,
    /// Staged inputs and results older than this (finished jobs, or inputs
    /// never submitted) are deleted by the reaper.
    pub buffer_ttl_secs: u64,
}

impl Default for JobsConfig {
    fn default() -> Self {
        Self {
            max_upload_bytes: 64 * 1024 * 1024,
            max_buffer_bytes: 2 * 1024 * 1024 * 1024,
            buffer_ttl_secs: 7 * 24 * 3600,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubmitError {
    /// Unknown engine or not on the credential's allow-list (same answer).
    EngineNotAllowed,
    BadRequest(String),
    InputNotFound,
    TooLarge(String),
    /// The shared buffer is at its size cap.
    BufferFull,
    Storage(String),
}

struct EngineSlot {
    spec: EngineSpec,
    engine: Arc<dyn JobEngine>,
    queue: Mutex<VecDeque<JobId>>,
    wake: Notify,
}

pub struct JobsService {
    registry: Registry,
    store: JobStore,
    slots: HashMap<String, EngineSlot>,
    audit: AuditSink,
    cfg: JobsConfig,
}

fn valid_sha256(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// A single file name: no separators, no `.`/`..`, no NUL, no leading dot.
fn valid_local_name(n: &str) -> bool {
    !n.is_empty() && n.len() <= 255 && !n.starts_with('.') && !n.contains(['/', '\\', '\0'])
}

fn supports(kind: EngineKind, job: JobKind) -> bool {
    matches!(
        (kind, job),
        (EngineKind::LlmLocal, JobKind::ExtractLlm)
            | (EngineKind::OcrOpenaiVision, JobKind::OcrPage)
            | (EngineKind::SpanGliner, JobKind::ExtractSpans)
    )
}

/// Page images the OCR adapter will forward byte-for-byte.
const OCR_MEDIA_TYPES: &[&str] = &["image/png", "image/jpeg", "image/webp"];

impl JobsService {
    pub fn new(
        registry: Registry,
        store: JobStore,
        engines: HashMap<String, Arc<dyn JobEngine>>,
        audit: AuditSink,
        cfg: JobsConfig,
    ) -> Result<Arc<Self>, String> {
        let mut slots = HashMap::new();
        for spec in registry.all() {
            let engine = engines
                .get(&spec.engine_id)
                .ok_or_else(|| format!("no adapter for engine {}", spec.engine_id))?
                .clone();
            slots.insert(
                spec.engine_id.clone(),
                EngineSlot {
                    spec: spec.clone(),
                    engine,
                    queue: Mutex::new(VecDeque::new()),
                    wake: Notify::new(),
                },
            );
        }
        Ok(Arc::new(Self {
            registry,
            store,
            slots,
            audit,
            cfg,
        }))
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn config(&self) -> &JobsConfig {
        &self.cfg
    }

    /// Re-queue jobs left `queued`/`running` by a previous process, then
    /// start the workers.
    pub fn start(self: &Arc<Self>) {
        match self.store.list_jobs() {
            Ok(recs) => {
                for mut rec in recs {
                    if rec.status.is_terminal() {
                        continue;
                    }
                    match self.slots.get(&rec.engine_id) {
                        Some(slot) => {
                            rec.status = JobStatus::Queued;
                            let _ = self.store.save_job(&rec);
                            slot.queue
                                .lock()
                                .unwrap_or_else(|e| e.into_inner())
                                .push_back(rec.job_id);
                        }
                        None => {
                            rec.status = JobStatus::Failed;
                            rec.defer_reason = Some("engine-removed".into());
                            rec.finished_at = Some(Utc::now());
                            let _ = self.store.save_job(&rec);
                        }
                    }
                }
            }
            Err(e) => error!(error = %e, "job store recovery scan failed"),
        }
        let reaper = Arc::clone(self);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                let n = reaper.reap_expired(Utc::now());
                if n > 0 {
                    tracing::info!(reaped = n, "job buffer TTL reaper");
                }
            }
        });
        for (engine_id, slot) in &self.slots {
            for _ in 0..slot.spec.permits {
                let svc = Arc::clone(self);
                let engine_id = engine_id.clone();
                tokio::spawn(async move { svc.worker(engine_id).await });
            }
        }
    }

    async fn worker(self: Arc<Self>, engine_id: String) {
        let slot = &self.slots[&engine_id];
        loop {
            let next = slot
                .queue
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .pop_front();
            match next {
                Some(id) => self.run_job(slot, id).await,
                None => slot.wake.notified().await,
            }
        }
    }

    pub fn stage_input(
        &self,
        cred: &Credential,
        media_type: &str,
        bytes: &[u8],
    ) -> Result<(StagedRef, String), SubmitError> {
        if bytes.len() as u64 > self.cfg.max_upload_bytes {
            return Err(SubmitError::TooLarge(
                "upload exceeds max_upload_bytes".into(),
            ));
        }
        if self.store.buffer_bytes() + bytes.len() as u64 > self.cfg.max_buffer_bytes {
            return Err(SubmitError::BufferFull);
        }
        let sha = sha256_hex(bytes);
        let meta = InputMeta {
            module_id: cred.module_id.clone(),
            sha256: sha.clone(),
            size: bytes.len() as u64,
            media_type: media_type.chars().take(128).collect(),
            created_at: Utc::now(),
        };
        let r = self
            .store
            .put_input(&meta, bytes)
            .map_err(|e| SubmitError::Storage(e.to_string()))?;
        Ok((r, sha))
    }

    fn resolve_local(cred: &Credential, name: &str) -> Result<(String, u64), SubmitError> {
        if !valid_local_name(name) {
            return Err(SubmitError::BadRequest(
                "local_name must be a plain file name".into(),
            ));
        }
        let dir = cred
            .input_dir
            .as_ref()
            .ok_or_else(|| SubmitError::BadRequest("no input_dir for this credential".into()))?;
        let root = dir.canonicalize().map_err(|_| SubmitError::InputNotFound)?;
        let full = root
            .join(name)
            .canonicalize()
            .map_err(|_| SubmitError::InputNotFound)?;
        if !full.starts_with(&root) || !Path::new(&full).is_file() {
            return Err(SubmitError::InputNotFound);
        }
        let size = std::fs::metadata(&full)
            .map_err(|_| SubmitError::InputNotFound)?
            .len();
        Ok((full.to_string_lossy().into_owned(), size))
    }

    pub fn submit(&self, cred: &Credential, req: SubmitRequest) -> Result<JobId, SubmitError> {
        if !cred.allows_engine(&req.engine_id) {
            return Err(SubmitError::EngineNotAllowed);
        }
        let slot = self
            .slots
            .get(&req.engine_id)
            .ok_or(SubmitError::EngineNotAllowed)?;
        if !supports(slot.spec.kind, req.kind) {
            return Err(SubmitError::BadRequest(
                "engine does not support this job kind".into(),
            ));
        }
        if req.kind == JobKind::OcrPage && !OCR_MEDIA_TYPES.contains(&req.input.media_type.as_str())
        {
            return Err(SubmitError::BadRequest(
                "ocr-page input.media_type must be image/png, image/jpeg or image/webp".into(),
            ));
        }
        if !valid_sha256(&req.input.sha256) {
            return Err(SubmitError::BadRequest(
                "input.sha256 must be 64 lowercase hex chars".into(),
            ));
        }
        if req.kind == JobKind::ExtractSpans
            && req
                .params
                .prompt_id
                .as_deref()
                .is_some_and(|p| p != "spans-v1")
        {
            return Err(SubmitError::BadRequest(
                "extract-spans takes no prompt_id other than spans-v1".into(),
            ));
        }
        if let Some(p) = &req.params.prompt_id {
            if prompts::prompt(p).is_none() {
                return Err(SubmitError::BadRequest("unknown prompt_id".into()));
            }
        }
        if let Some(s) = &req.params.schema_id {
            if prompts::schema(s).is_none() {
                return Err(SubmitError::BadRequest("unknown schema_id".into()));
            }
        }
        let (local_path, size) = match (&req.input.staged_ref, &req.input.local_name) {
            (Some(r), None) => {
                let meta = self
                    .store
                    .input_meta_of(*r)
                    .map_err(|_| SubmitError::InputNotFound)?;
                if meta.module_id != cred.module_id {
                    return Err(SubmitError::InputNotFound);
                }
                if meta.sha256 != req.input.sha256 {
                    return Err(SubmitError::BadRequest(
                        "input.sha256 does not match the staged input".into(),
                    ));
                }
                (None, meta.size)
            }
            (None, Some(name)) => {
                let (p, size) = Self::resolve_local(cred, name)?;
                (Some(p), size)
            }
            _ => {
                return Err(SubmitError::BadRequest(
                    "exactly one of input.staged_ref or input.local_name is required".into(),
                ))
            }
        };
        if size > slot.spec.max_input_bytes {
            return Err(SubmitError::TooLarge(
                "input exceeds the engine's max_input_bytes".into(),
            ));
        }
        let rec = JobRecord {
            job_id: JobId::new(),
            module_id: cred.module_id.clone(),
            credential_id: cred.credential_id.clone(),
            engine_id: req.engine_id,
            kind: req.kind,
            params: req.params,
            input: req.input,
            input_size: size,
            local_path,
            status: JobStatus::Queued,
            defer_reason: None,
            created_at: Utc::now(),
            finished_at: None,
            record: None,
            buffer_deleted_at: None,
            replay_of: None,
            replay_match: None,
        };
        self.store
            .save_job(&rec)
            .map_err(|e| SubmitError::Storage(e.to_string()))?;
        slot.queue
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push_back(rec.job_id);
        slot.wake.notify_one();
        Ok(rec.job_id)
    }

    /// Visible only to the owning module; other modules see "not found".
    pub fn status(&self, cred: &Credential, id: JobId) -> Option<JobRecord> {
        self.store
            .load_job(id)
            .ok()
            .filter(|r| r.module_id == cred.module_id)
    }

    pub fn result(&self, cred: &Credential, id: JobId) -> Option<JobResult> {
        self.status(cred, id)?;
        self.store.load_result(id).ok()
    }

    /// Delete the staged input (upload mode) and the result. Idempotent.
    /// Returns `None` if the job does not exist for this module, `Some(false)`
    /// if it is not finished yet.
    pub fn ack(&self, cred: &Credential, id: JobId) -> Option<bool> {
        let mut rec = self.status(cred, id)?;
        if !rec.status.is_terminal() {
            return Some(false);
        }
        if rec.buffer_deleted_at.is_none() {
            if let Some(r) = rec.input.staged_ref {
                self.store.delete_input(r);
            }
            self.store.delete_result(id);
            rec.buffer_deleted_at = Some(Utc::now());
            if let Err(e) = self.store.save_job(&rec) {
                warn!(error = %e, "ack: could not persist buffer_deleted_at");
            }
            if let Some(r) = &rec.record {
                self.emit_audit(&rec, r);
            }
        }
        Some(true)
    }

    /// Compare two finished entity jobs of the same module (e.g. the span
    /// engine against OLMo). `None` if either job/result is missing.
    pub fn compare(&self, cred: &Credential, a: JobId, b: JobId) -> Option<witness::Comparison> {
        let ea = self.result(cred, a)?.entities?;
        let eb = self.result(cred, b)?.entities?;
        Some(witness::compare(&ea, &eb))
    }

    /// Delete buffers past the TTL: results and staged inputs of finished
    /// jobs, and staged inputs that no job ever used. Returns how many
    /// buffers were removed. Existing-file inputs are never touched.
    pub fn reap_expired(&self, now: chrono::DateTime<Utc>) -> usize {
        let ttl = chrono::Duration::seconds(self.cfg.buffer_ttl_secs as i64);
        let mut removed = 0;
        let jobs = self.store.list_jobs().unwrap_or_default();
        let mut referenced = std::collections::HashSet::new();
        for mut rec in jobs {
            if let Some(r) = rec.input.staged_ref {
                referenced.insert(r);
            }
            let Some(finished) = rec.finished_at else {
                continue;
            };
            if !rec.status.is_terminal() || rec.buffer_deleted_at.is_some() || now - finished < ttl
            {
                continue;
            }
            if let Some(r) = rec.input.staged_ref {
                self.store.delete_input(r);
            }
            self.store.delete_result(rec.job_id);
            rec.buffer_deleted_at = Some(now);
            if self.store.save_job(&rec).is_ok() {
                if let Some(r) = &rec.record {
                    self.emit_audit(&rec, r);
                }
                removed += 1;
            }
        }
        for (r, meta) in self.store.list_inputs().unwrap_or_default() {
            if !referenced.contains(&r) && now - meta.created_at >= ttl {
                self.store.delete_input(r);
                removed += 1;
            }
        }
        removed
    }

    fn emit_audit(&self, rec: &JobRecord, r: &ResultRecord) {
        (self.audit)(JobAuditEntry {
            entry_type: String::new(),
            timestamp_utc: Utc::now(),
            job_id: rec.job_id.to_string(),
            module_id: rec.module_id.clone(),
            engine_id: r.engine_id.clone(),
            model_id: r.model_id.clone(),
            weights_sha256: r.weights_sha256.clone(),
            prompt_sha256: r.prompt_sha256.clone(),
            schema_sha256: r.schema_sha256.clone(),
            input_sha256: r.input_sha256.clone(),
            output_sha256: r.output_sha256.clone(),
            outcome: rec.status.as_str().to_string(),
            latency_ms: r.latency_ms,
            defer_reason: rec.defer_reason.clone(),
            buffer_deleted_at: rec.buffer_deleted_at,
        });
    }

    fn read_input_bytes(&self, rec: &JobRecord) -> Result<Vec<u8>, String> {
        match (&rec.input.staged_ref, &rec.local_path) {
            (Some(r), _) => self
                .store
                .read_input(*r)
                .map_err(|e| format!("staged input unreadable: {e}")),
            (None, Some(p)) => std::fs::read(p).map_err(|e| format!("input file unreadable: {e}")),
            _ => Err("job has no input".into()),
        }
    }

    /// One engine call, or one per chunk for witness-validated prompts.
    async fn run_engine(
        &self,
        slot: &EngineSlot,
        rec: &JobRecord,
        prompt: Option<&prompts::Prompt>,
        schema: Option<&prompts::Schema>,
        bytes: &[u8],
    ) -> Result<Produced, EngineError> {
        let (Some(prompt), Some(schema)) = (prompt, schema) else {
            // OCR page: the image goes to the named engine unchanged.
            let out = slot
                .engine
                .run(EngineRequest {
                    input: bytes,
                    media_type: &rec.input.media_type,
                    params: &rec.params,
                    system_prompt: "",
                    schema: None,
                })
                .await?;
            return Ok(Produced::Text(out.text));
        };
        if !prompt.witness {
            let out = slot
                .engine
                .run(EngineRequest {
                    input: bytes,
                    media_type: &rec.input.media_type,
                    params: &rec.params,
                    system_prompt: prompt.text,
                    schema: Some((schema.json)()),
                })
                .await?;
            return Ok(Produced::Text(out.text));
        }
        let text = std::str::from_utf8(bytes)
            .map_err(|_| EngineError::Unreadable("input is not UTF-8 text".into()))?;
        let chunk_bytes = slot.spec.chunk_bytes.unwrap_or(8192) as usize;
        let overlap = slot.spec.chunk_overlap.unwrap_or(256) as usize;
        let mut entities: Vec<Entity> = Vec::new();
        let mut rejected = 0u32;
        for (offset, chunk) in witness::split_chunks(text, chunk_bytes, overlap) {
            let out = slot
                .engine
                .run(EngineRequest {
                    input: chunk.as_bytes(),
                    media_type: &rec.input.media_type,
                    params: &rec.params,
                    system_prompt: prompt.text,
                    schema: Some((schema.json)()),
                })
                .await?;
            let (accepted, rej) =
                witness::validate_chunk(chunk, offset, &out.text).map_err(EngineError::Failed)?;
            rejected += rej;
            for e in accepted {
                if !entities.contains(&e) {
                    entities.push(e);
                }
            }
        }
        entities.sort_by(|a, b| {
            (a.span.start, a.span.end, &a.class).cmp(&(b.span.start, b.span.end, &b.class))
        });
        Ok(Produced::Entities { entities, rejected })
    }

    /// Re-run a finished job with identical input and parameters as a new
    /// job; when it finishes, `replay_match` says whether `output_sha256`
    /// equals the original's. Needs the staged input (i.e. not yet acked).
    pub fn replay(&self, cred: &Credential, id: JobId) -> Result<JobId, SubmitError> {
        let orig = self.status(cred, id).ok_or(SubmitError::InputNotFound)?;
        if !orig.status.is_terminal() || orig.record.is_none() {
            return Err(SubmitError::BadRequest("job is not finished".into()));
        }
        if orig.buffer_deleted_at.is_some() && orig.input.staged_ref.is_some() {
            return Err(SubmitError::BadRequest(
                "input already deleted by ack".into(),
            ));
        }
        if !cred.allows_engine(&orig.engine_id) {
            return Err(SubmitError::EngineNotAllowed);
        }
        let slot = self
            .slots
            .get(&orig.engine_id)
            .ok_or(SubmitError::EngineNotAllowed)?;
        let mut rec = orig.clone();
        rec.job_id = JobId::new();
        rec.status = JobStatus::Queued;
        rec.defer_reason = None;
        rec.created_at = Utc::now();
        rec.finished_at = None;
        rec.record = None;
        rec.buffer_deleted_at = None;
        rec.replay_of = Some(id);
        rec.replay_match = None;
        self.store
            .save_job(&rec)
            .map_err(|e| SubmitError::Storage(e.to_string()))?;
        slot.queue
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .push_back(rec.job_id);
        slot.wake.notify_one();
        Ok(rec.job_id)
    }

    async fn run_job(&self, slot: &EngineSlot, id: JobId) {
        let mut rec = match self.store.load_job(id) {
            Ok(r) => r,
            Err(e) => {
                error!(job_id = %id, error = %e, "job record missing; dropping");
                return;
            }
        };
        rec.status = JobStatus::Running;
        let _ = self.store.save_job(&rec);

        let started = Instant::now();
        // Model prompts/schemas apply to LLM jobs only; OCR jobs carry none. A job
        // record reloaded from disk may name a prompt or schema this build no
        // longer has: that fails the job, it must not panic the runner.
        let lookup: Result<(Option<&prompts::Prompt>, Option<&prompts::Schema>), String> = if rec
            .kind
            == JobKind::OcrPage
        {
            Ok((None, None))
        } else {
            let default_prompt = if rec.kind == JobKind::ExtractSpans {
                "spans-v1"
            } else {
                prompts::DEFAULT_PROMPT_ID
            };
            match prompts::prompt(rec.params.prompt_id.as_deref().unwrap_or(default_prompt)) {
                None => Err("unknown-prompt".to_string()),
                Some(prompt) => match prompts::schema(rec.params.schema_id.as_deref().unwrap_or(
                    if prompts::schema(prompt.id).is_some() {
                        prompt.id
                    } else {
                        prompts::DEFAULT_SCHEMA_ID
                    },
                )) {
                    None => Err("unknown-schema".to_string()),
                    Some(schema) => Ok((Some(prompt), Some(schema))),
                },
            }
        };
        let (prompt, schema) = lookup
            .as_ref()
            .map(|(p, s)| (*p, *s))
            .unwrap_or((None, None));
        let input = match &lookup {
            Err(msg) => Err(msg.clone()),
            Ok(_) => self.read_input_bytes(&rec),
        };

        let (status, mut reason, produced) = match input {
            Err(msg) => (JobStatus::Failed, Some(msg), Produced::None),
            Ok(bytes) if sha256_hex(&bytes) != rec.input.sha256 => (
                JobStatus::Failed,
                Some("input-hash-mismatch".to_string()),
                Produced::None,
            ),
            Ok(bytes)
                if bytes.is_empty()
                    || (rec.kind != JobKind::OcrPage
                        && bytes.iter().all(|b| b.is_ascii_whitespace())) =>
            {
                (JobStatus::Empty, None, Produced::None)
            }
            Ok(bytes) => {
                let deadline = Duration::from_secs(slot.spec.max_job_secs);
                let work = self.run_engine(slot, &rec, prompt, schema, &bytes);
                match tokio::time::timeout(deadline, work).await {
                    Err(_) => (
                        JobStatus::Deferred,
                        Some("deadline".to_string()),
                        Produced::None,
                    ),
                    Ok(Ok(p)) => (JobStatus::Done, None, p),
                    Ok(Err(EngineError::Unavailable(m))) => {
                        (JobStatus::Deferred, Some(m), Produced::None)
                    }
                    Ok(Err(EngineError::Unreadable(m))) => {
                        (JobStatus::Unreadable, Some(m), Produced::None)
                    }
                    Ok(Err(EngineError::Failed(m))) => (JobStatus::Failed, Some(m), Produced::None),
                }
            }
        };

        // A witness job whose every proposal was rejected found nothing real.
        let status = match (&produced, status) {
            (Produced::Text(t), JobStatus::Done)
                if rec.kind == JobKind::OcrPage && t.trim().is_empty() =>
            {
                JobStatus::Empty
            }
            (Produced::Entities { entities, .. }, JobStatus::Done) if entities.is_empty() => {
                JobStatus::Empty
            }
            (_, s) => s,
        };
        let mut output_sha256 = produced.output_sha256();
        let mut status = status;
        if let Some(res) = produced.into_result(id) {
            if let Err(e) = self.store.save_result(&res) {
                // A job must not report `done`/`empty` when its result is not
                // on disk: the caller would fetch a result that never existed.
                error!(job_id = %id, error = %e, "could not persist job result");
                status = JobStatus::Failed;
                reason = Some("internal_error".to_string());
                output_sha256 = String::new();
            }
        }
        let record = ResultRecord {
            job_id: id,
            engine_id: slot.spec.engine_id.clone(),
            model_id: slot.spec.model.clone(),
            weights_sha256: slot.spec.weights_sha256.clone(),
            prompt_sha256: prompt.map(prompts::prompt_sha256).unwrap_or_default(),
            schema_sha256: schema.map(prompts::schema_sha256).unwrap_or_default(),
            params: rec.params.clone(),
            input_sha256: rec.input.sha256.clone(),
            output_sha256,
            outcome: status,
            latency_ms: started.elapsed().as_millis() as u64,
        };
        if let (Some(orig), JobStatus::Done | JobStatus::Empty) = (rec.replay_of, status) {
            rec.replay_match = self
                .store
                .load_job(orig)
                .ok()
                .and_then(|o| o.record)
                .map(|o| o.output_sha256 == record.output_sha256);
        }
        rec.status = status;
        rec.defer_reason = reason.map(|r| r.chars().take(200).collect());
        rec.finished_at = Some(Utc::now());
        rec.record = Some(record.clone());
        if let Err(e) = self.store.save_job(&rec) {
            error!(job_id = %id, error = %e, "could not persist job record");
        }
        self.emit_audit(&rec, &record);
    }
}

/// What a successful engine run produced.
enum Produced {
    None,
    Text(String),
    Entities {
        entities: Vec<Entity>,
        rejected: u32,
    },
}

impl Produced {
    /// Hash of the content callers receive: raw text, or the canonical JSON
    /// of the validated entities and rejected count.
    fn output_sha256(&self) -> String {
        match self {
            Produced::None => String::new(),
            Produced::Text(t) => sha256_hex(t.as_bytes()),
            Produced::Entities { entities, rejected } => sha256_hex(
                serde_json::to_string(
                    &serde_json::json!({"entities": entities, "rejected": rejected}),
                )
                .unwrap_or_default()
                .as_bytes(),
            ),
        }
    }

    fn into_result(self, job_id: JobId) -> Option<JobResult> {
        match self {
            Produced::None => None,
            Produced::Text(text) => Some(JobResult {
                job_id,
                text,
                entities: None,
                rejected: None,
            }),
            Produced::Entities { entities, rejected } => Some(JobResult {
                job_id,
                text: String::new(),
                entities: Some(entities),
                rejected: Some(rejected),
            }),
        }
    }
}

/// `InputRef` accessor used by the HTTP layer for the public status view.
pub fn input_summary(i: &InputRef) -> (String, String) {
    (i.sha256.clone(), i.media_type.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jobs::credentials::{tests_support::cred, Credential};
    use crate::jobs::engines::EngineOutput;
    use crate::jobs::registry::tests_support::spec;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Fake {
        delay: Duration,
        result: Result<String, EngineError>,
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl JobEngine for Fake {
        async fn run(&self, _req: EngineRequest<'_>) -> Result<EngineOutput, EngineError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(self.delay).await;
            self.result.clone().map(|text| EngineOutput { text })
        }
    }

    fn tmp(tag: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "slm-jobs-{tag}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    struct Rig {
        svc: Arc<JobsService>,
        audit: Arc<Mutex<Vec<JobAuditEntry>>>,
        calls: Vec<Arc<AtomicUsize>>,
        dir: std::path::PathBuf,
    }

    fn rig(engines: Vec<(&str, Duration, Result<String, EngineError>, u64)>) -> Rig {
        let dir = tmp("rig");
        let mut specs = Vec::new();
        let mut map: HashMap<String, Arc<dyn JobEngine>> = HashMap::new();
        let mut calls = Vec::new();
        for (id, delay, result, max_secs) in engines {
            let mut s = spec(id);
            s.max_job_secs = max_secs;
            s.max_input_bytes = 1024;
            specs.push(s);
            let c = Arc::new(AtomicUsize::new(0));
            calls.push(c.clone());
            map.insert(
                id.to_string(),
                Arc::new(Fake {
                    delay,
                    result,
                    calls: c,
                }),
            );
        }
        let audit = Arc::new(Mutex::new(Vec::new()));
        let a2 = audit.clone();
        let sink: AuditSink = Arc::new(move |e| a2.lock().unwrap().push(e));
        let svc = JobsService::new(
            Registry::from_specs(specs).unwrap(),
            JobStore::new(dir.join("store")).unwrap(),
            map,
            sink,
            JobsConfig::default(),
        )
        .unwrap();
        svc.start();
        Rig {
            svc,
            audit,
            calls,
            dir,
        }
    }

    fn submit_text(
        r: &Rig,
        c: &Credential,
        engine: &str,
        text: &str,
    ) -> Result<JobId, SubmitError> {
        let (sref, sha) = r.svc.stage_input(c, "text/plain", text.as_bytes()).unwrap();
        r.svc.submit(
            c,
            SubmitRequest {
                engine_id: engine.into(),
                kind: JobKind::ExtractLlm,
                input: InputRef {
                    sha256: sha,
                    media_type: "text/plain".into(),
                    staged_ref: Some(sref),
                    local_name: None,
                },
                params: Default::default(),
            },
        )
    }

    async fn wait_terminal(r: &Rig, c: &Credential, id: JobId) -> JobRecord {
        for _ in 0..10_000 {
            let rec = r.svc.status(c, id).unwrap();
            if rec.status.is_terminal() {
                return rec;
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        panic!("job never finished");
    }

    #[tokio::test(start_paused = true)]
    async fn job_outlives_120s_and_record_carries_hashes_without_content() {
        let r = rig(vec![(
            "e1",
            Duration::from_secs(130),
            Ok("SECRET-OUTPUT-TEXT".into()),
            900,
        )]);
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let id = submit_text(&r, &c, "e1", "SECRET-INPUT-TEXT").unwrap();
        let rec = wait_terminal(&r, &c, id).await;
        assert_eq!(rec.status, JobStatus::Done);
        let record = rec.record.unwrap();
        assert_eq!(record.output_sha256, sha256_hex(b"SECRET-OUTPUT-TEXT"));
        assert_eq!(record.input_sha256, sha256_hex(b"SECRET-INPUT-TEXT"));
        assert!(!record.prompt_sha256.is_empty() && !record.schema_sha256.is_empty());
        let a = r.audit.lock().unwrap();
        assert_eq!(a.len(), 1);
        let line = serde_json::to_string(&a[0]).unwrap();
        assert!(
            !line.contains("SECRET"),
            "audit must not hold content: {line}"
        );
        assert_eq!(r.svc.result(&c, id).unwrap().text, "SECRET-OUTPUT-TEXT");
    }

    #[tokio::test(start_paused = true)]
    async fn result_save_failure_gives_failed_not_done() {
        let r = rig(vec![("e1", Duration::from_secs(1), Ok("OUT".into()), 900)]);
        let c = cred("c", "tok", "mod-a", &["e1"]);
        // Replace the results directory with a plain file so the save fails.
        let results = r.dir.join("store").join("results");
        std::fs::remove_dir_all(&results).unwrap();
        std::fs::write(&results, b"blocked").unwrap();
        let id = submit_text(&r, &c, "e1", "hello").unwrap();
        let rec = wait_terminal(&r, &c, id).await;
        assert_eq!(rec.status, JobStatus::Failed);
        assert_eq!(rec.defer_reason.as_deref(), Some("internal_error"));
        assert!(r.svc.result(&c, id).is_none());
    }

    #[tokio::test(start_paused = true)]
    async fn deadline_gives_deferred() {
        let r = rig(vec![("e1", Duration::from_secs(2000), Ok("x".into()), 5)]);
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let id = submit_text(&r, &c, "e1", "hello").unwrap();
        let rec = wait_terminal(&r, &c, id).await;
        assert_eq!(rec.status, JobStatus::Deferred);
        assert_eq!(rec.defer_reason.as_deref(), Some("deadline"));
    }

    #[tokio::test(start_paused = true)]
    async fn unavailable_engine_defers_and_never_uses_another_engine() {
        let r = rig(vec![
            (
                "e1",
                Duration::ZERO,
                Err(EngineError::Unavailable("down".into())),
                60,
            ),
            ("e2", Duration::ZERO, Ok("other".into()), 60),
        ]);
        let c = cred("c", "tok", "mod-a", &["e1", "e2"]);
        let id = submit_text(&r, &c, "e1", "hello").unwrap();
        let rec = wait_terminal(&r, &c, id).await;
        assert_eq!(rec.status, JobStatus::Deferred);
        assert_eq!(r.calls[1].load(Ordering::SeqCst), 0, "no fallback to e2");
    }

    #[tokio::test(start_paused = true)]
    async fn empty_input_is_empty_without_engine_call() {
        let r = rig(vec![("e1", Duration::ZERO, Ok("x".into()), 60)]);
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let id = submit_text(&r, &c, "e1", "  \n ").unwrap();
        assert_eq!(wait_terminal(&r, &c, id).await.status, JobStatus::Empty);
        assert_eq!(r.calls[0].load(Ordering::SeqCst), 0);
    }

    #[tokio::test(start_paused = true)]
    async fn access_control_engine_module_and_hash() {
        let r = rig(vec![
            ("e1", Duration::ZERO, Ok("x".into()), 60),
            ("e2", Duration::ZERO, Ok("y".into()), 60),
        ]);
        let a = cred("a", "ta", "mod-a", &["e1"]);
        let b = cred("b", "tb", "mod-b", &["e1", "e2"]);
        assert_eq!(
            submit_text(&r, &a, "e2", "t").unwrap_err(),
            SubmitError::EngineNotAllowed
        );
        assert_eq!(
            submit_text(&r, &a, "nope", "t").unwrap_err(),
            SubmitError::EngineNotAllowed
        );
        let id = submit_text(&r, &a, "e1", "t").unwrap();
        // Other module cannot see, fetch or ack it.
        assert!(r.svc.status(&b, id).is_none());
        assert!(r.svc.result(&b, id).is_none());
        assert!(r.svc.ack(&b, id).is_none());
        // Staged input of module A cannot be used by module B.
        let (sref, sha) = r.svc.stage_input(&a, "text/plain", b"abc").unwrap();
        let req = |sha: String, sref| SubmitRequest {
            engine_id: "e1".into(),
            kind: JobKind::ExtractLlm,
            input: InputRef {
                sha256: sha,
                media_type: "text/plain".into(),
                staged_ref: Some(sref),
                local_name: None,
            },
            params: Default::default(),
        };
        assert_eq!(
            r.svc.submit(&b, req(sha.clone(), sref)).unwrap_err(),
            SubmitError::InputNotFound
        );
        assert!(matches!(
            r.svc.submit(&a, req(sha256_hex(b"other"), sref)),
            Err(SubmitError::BadRequest(_))
        ));
        assert!(matches!(
            r.svc.submit(&a, req("zz".into(), sref)),
            Err(SubmitError::BadRequest(_))
        ));
    }

    #[tokio::test(start_paused = true)]
    async fn local_name_refuses_traversal_and_symlink_escape() {
        let r = rig(vec![("e1", Duration::ZERO, Ok("x".into()), 60)]);
        let inputs = r.dir.join("inputs");
        std::fs::create_dir_all(&inputs).unwrap();
        std::fs::write(inputs.join("page1.txt"), "hello").unwrap();
        std::fs::write(r.dir.join("outside.txt"), "secret").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(r.dir.join("outside.txt"), inputs.join("link.txt")).unwrap();
        let mut c = cred("c", "tok", "mod-a", &["e1"]);
        c.input_dir = Some(inputs);
        let try_name = |name: &str, sha: &str| {
            r.svc.submit(
                &c,
                SubmitRequest {
                    engine_id: "e1".into(),
                    kind: JobKind::ExtractLlm,
                    input: InputRef {
                        sha256: sha.into(),
                        media_type: "text/plain".into(),
                        staged_ref: None,
                        local_name: Some(name.into()),
                    },
                    params: Default::default(),
                },
            )
        };
        let good = sha256_hex(b"hello");
        for bad in [
            "../outside.txt",
            "/etc/passwd",
            "a/b",
            "..",
            ".hidden",
            "",
            "link.txt",
            "missing.txt",
        ] {
            assert!(try_name(bad, &good).is_err(), "{bad:?} must be refused");
        }
        let id = try_name("page1.txt", &good).unwrap();
        assert_eq!(wait_terminal(&r, &c, id).await.status, JobStatus::Done);
        // Wrong declared hash for an existing file fails the job, not the engine.
        let id2 = try_name("page1.txt", &sha256_hex(b"different")).unwrap();
        let rec = wait_terminal(&r, &c, id2).await;
        assert_eq!(rec.status, JobStatus::Failed);
        assert_eq!(rec.defer_reason.as_deref(), Some("input-hash-mismatch"));
    }

    #[tokio::test(start_paused = true)]
    async fn ack_deletes_buffer_and_audits() {
        let r = rig(vec![("e1", Duration::ZERO, Ok("OUT".into()), 60)]);
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let id = submit_text(&r, &c, "e1", "IN").unwrap();
        let rec = wait_terminal(&r, &c, id).await;
        let sref = rec.input.staged_ref.unwrap();
        assert!(r.svc.store.read_input(sref).is_ok());
        assert_eq!(r.svc.ack(&c, id), Some(true));
        assert!(r.svc.store.read_input(sref).is_err());
        assert!(r.svc.result(&c, id).is_none());
        let after = r.svc.status(&c, id).unwrap();
        assert!(after.buffer_deleted_at.is_some());
        assert_eq!(r.svc.ack(&c, id), Some(true), "idempotent");
        let a = r.audit.lock().unwrap();
        assert_eq!(a.len(), 2);
        assert!(a[1].buffer_deleted_at.is_some());
    }

    #[tokio::test(start_paused = true)]
    async fn jobs_left_queued_by_a_previous_process_are_recovered() {
        let dir = tmp("recover");
        let store_dir = dir.join("store");
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let make = |start: bool| {
            let mut s = spec("e1");
            s.max_input_bytes = 1024;
            let mut map: HashMap<String, Arc<dyn JobEngine>> = HashMap::new();
            map.insert(
                "e1".into(),
                Arc::new(Fake {
                    delay: Duration::ZERO,
                    result: Ok("done".into()),
                    calls: Arc::new(AtomicUsize::new(0)),
                }),
            );
            let sink: AuditSink = Arc::new(|_| {});
            let svc = JobsService::new(
                Registry::from_specs(vec![s]).unwrap(),
                JobStore::new(store_dir.clone()).unwrap(),
                map,
                sink,
                JobsConfig::default(),
            )
            .unwrap();
            if start {
                svc.start();
            }
            svc
        };
        let first = make(false); // workers never started: job stays queued on disk
        let (sref, sha) = first.stage_input(&c, "text/plain", b"hello").unwrap();
        let id = first
            .submit(
                &c,
                SubmitRequest {
                    engine_id: "e1".into(),
                    kind: JobKind::ExtractLlm,
                    input: InputRef {
                        sha256: sha,
                        media_type: "text/plain".into(),
                        staged_ref: Some(sref),
                        local_name: None,
                    },
                    params: Default::default(),
                },
            )
            .unwrap();
        assert_eq!(first.status(&c, id).unwrap().status, JobStatus::Queued);
        let second = make(true);
        for _ in 0..100 {
            if second.status(&c, id).unwrap().status.is_terminal() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        assert_eq!(second.status(&c, id).unwrap().status, JobStatus::Done);
    }

    /// A job reloaded from disk may name a prompt this build no longer has. It must
    /// end `failed` with a reason, not panic the runner.
    #[tokio::test]
    async fn reloaded_job_with_unknown_prompt_fails_instead_of_panicking() {
        let dir = tmp("stale-prompt");
        let store_dir = dir.join("store");
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let make = |start: bool| {
            let mut s = spec("e1");
            s.max_input_bytes = 1024;
            let mut map: HashMap<String, Arc<dyn JobEngine>> = HashMap::new();
            map.insert(
                "e1".into(),
                Arc::new(Fake {
                    delay: Duration::ZERO,
                    result: Ok("done".into()),
                    calls: Arc::new(AtomicUsize::new(0)),
                }),
            );
            let sink: AuditSink = Arc::new(|_| {});
            let svc = JobsService::new(
                Registry::from_specs(vec![s]).unwrap(),
                JobStore::new(store_dir.clone()).unwrap(),
                map,
                sink,
                JobsConfig::default(),
            )
            .unwrap();
            if start {
                svc.start();
            }
            svc
        };
        let first = make(false);
        let (sref, sha) = first.stage_input(&c, "text/plain", b"hello").unwrap();
        let id = first
            .submit(
                &c,
                SubmitRequest {
                    engine_id: "e1".into(),
                    kind: JobKind::ExtractLlm,
                    input: InputRef {
                        sha256: sha,
                        media_type: "text/plain".into(),
                        staged_ref: Some(sref),
                        local_name: None,
                    },
                    params: Default::default(),
                },
            )
            .unwrap();
        let store = JobStore::new(store_dir.clone()).unwrap();
        let mut rec = store.load_job(id).unwrap();
        rec.params.prompt_id = Some("no-such-prompt".into());
        store.save_job(&rec).unwrap();
        let second = make(true);
        for _ in 0..100 {
            if second.status(&c, id).unwrap().status.is_terminal() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        let done = second.status(&c, id).unwrap();
        assert_eq!(done.status, JobStatus::Failed);
        assert_eq!(done.defer_reason.as_deref(), Some("unknown-prompt"));
    }

    struct Responder(Arc<dyn Fn(&str) -> String + Send + Sync>, Arc<AtomicUsize>);

    #[async_trait]
    impl JobEngine for Responder {
        async fn run(&self, req: EngineRequest<'_>) -> Result<EngineOutput, EngineError> {
            self.1.fetch_add(1, Ordering::SeqCst);
            let text = std::str::from_utf8(req.input).unwrap();
            Ok(EngineOutput {
                text: (self.0)(text),
            })
        }
    }

    fn witness_rig(
        f: impl Fn(&str) -> String + Send + Sync + 'static,
        chunk: u64,
    ) -> (Rig, Arc<AtomicUsize>) {
        let dir = tmp("witness");
        let mut s = spec("e1");
        s.max_input_bytes = 1 << 20;
        s.chunk_bytes = Some(chunk);
        s.chunk_overlap = Some(32);
        let calls = Arc::new(AtomicUsize::new(0));
        let mut map: HashMap<String, Arc<dyn JobEngine>> = HashMap::new();
        map.insert("e1".into(), Arc::new(Responder(Arc::new(f), calls.clone())));
        let audit = Arc::new(Mutex::new(Vec::new()));
        let a2 = audit.clone();
        let sink: AuditSink = Arc::new(move |e| a2.lock().unwrap().push(e));
        let svc = JobsService::new(
            Registry::from_specs(vec![s]).unwrap(),
            JobStore::new(dir.join("store")).unwrap(),
            map,
            sink,
            JobsConfig::default(),
        )
        .unwrap();
        svc.start();
        (
            Rig {
                svc,
                audit,
                calls: vec![calls.clone()],
                dir,
            },
            calls,
        )
    }

    fn submit_v2(r: &Rig, c: &Credential, text: &str) -> JobId {
        let (sref, sha) = r.svc.stage_input(c, "text/plain", text.as_bytes()).unwrap();
        r.svc
            .submit(
                c,
                SubmitRequest {
                    engine_id: "e1".into(),
                    kind: JobKind::ExtractLlm,
                    input: InputRef {
                        sha256: sha,
                        media_type: "text/plain".into(),
                        staged_ref: Some(sref),
                        local_name: None,
                    },
                    params: crate::jobs::types::JobParams {
                        prompt_id: Some("entities-v2".into()),
                        ..Default::default()
                    },
                },
            )
            .unwrap()
    }

    /// Fake model: proposes every capitalised word it sees as a Person, plus
    /// one invented entity that is never in the input.
    fn proposer(text: &str) -> String {
        let mut ents: Vec<serde_json::Value> = text
            .split_whitespace()
            .filter(|w| w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
            .map(|w| serde_json::json!({"class":"Person","value":w,"witness":w}))
            .collect();
        ents.push(serde_json::json!({"class":"Person","value":"Zed","witness":"Zed"}));
        serde_json::json!({"entities": ents}).to_string()
    }

    #[tokio::test(start_paused = true)]
    async fn witness_job_rejects_invented_entities_and_derives_spans() {
        let (r, _) = witness_rig(proposer, 8192);
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let text = "Alice read Dune";
        let id = submit_v2(&r, &c, text);
        let rec = wait_terminal(&r, &c, id).await;
        assert_eq!(rec.status, JobStatus::Done);
        let res = r.svc.result(&c, id).unwrap();
        let ents = res.entities.unwrap();
        assert_eq!(res.rejected, Some(1), "Zed is invented");
        assert_eq!(ents.len(), 2);
        for e in &ents {
            assert_eq!(&text[e.span.start..e.span.end], e.witness);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn long_input_is_chunked_and_offsets_are_rebased() {
        let (r, calls) = witness_rig(proposer, 200);
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let text = format!(
            "{} Quentin ends here",
            "filler words go on and on. ".repeat(100)
        );
        let id = submit_v2(&r, &c, &text);
        let rec = wait_terminal(&r, &c, id).await;
        assert_eq!(rec.status, JobStatus::Done);
        assert!(
            calls.load(Ordering::SeqCst) > 5,
            "one engine call per chunk"
        );
        let ents = r.svc.result(&c, id).unwrap().entities.unwrap();
        let q = ents
            .iter()
            .find(|e| e.value == "Quentin")
            .expect("found across chunks");
        assert_eq!(&text[q.span.start..q.span.end], "Quentin");
        assert_eq!(
            ents.iter().filter(|e| e.value == "Quentin").count(),
            1,
            "overlap duplicates collapse"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn all_rejected_is_empty() {
        let (r, _) = witness_rig(
            |_| r#"{"entities":[{"class":"Person","value":"Zed","witness":"Zed"}]}"#.into(),
            8192,
        );
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let id = submit_v2(&r, &c, "nothing to see");
        assert_eq!(wait_terminal(&r, &c, id).await.status, JobStatus::Empty);
        assert_eq!(r.svc.result(&c, id).unwrap().rejected, Some(1));
    }

    #[tokio::test(start_paused = true)]
    async fn replay_matches_for_deterministic_engine_and_flags_drift() {
        let (r, _) = witness_rig(proposer, 8192);
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let id = submit_v2(&r, &c, "Alice read Dune");
        wait_terminal(&r, &c, id).await;
        let rid = r.svc.replay(&c, id).unwrap();
        let rr = wait_terminal(&r, &c, rid).await;
        assert_eq!(rr.replay_of, Some(id));
        assert_eq!(rr.replay_match, Some(true));

        // A drifting engine: output differs on every call.
        let n = Arc::new(AtomicUsize::new(0));
        let n2 = n.clone();
        let (r, _) = witness_rig(
            move |_| {
                let k = n2.fetch_add(1, Ordering::SeqCst);
                format!(
                    r#"{{"entities":[{{"class":"Person","value":"Alice","witness":"Alice"}}{}]}}"#,
                    if k == 0 {
                        ""
                    } else {
                        r#",{"class":"Person","value":"Dune","witness":"Dune"}"#
                    }
                )
            },
            8192,
        );
        let id = submit_v2(&r, &c, "Alice read Dune");
        wait_terminal(&r, &c, id).await;
        let rid = r.svc.replay(&c, id).unwrap();
        assert_eq!(wait_terminal(&r, &c, rid).await.replay_match, Some(false));

        // After ack the input is gone: replay refused. Other modules cannot replay.
        let b = cred("b", "tb", "mod-b", &["e1"]);
        assert!(r.svc.replay(&b, id).is_err());
        r.svc.ack(&c, id);
        assert!(matches!(
            r.svc.replay(&c, id),
            Err(SubmitError::BadRequest(_))
        ));
    }

    fn rig_with_cfg(cfg: JobsConfig) -> Rig {
        let dir = tmp("cfg");
        let mut s = spec("e1");
        s.max_input_bytes = 1024;
        let mut map: HashMap<String, Arc<dyn JobEngine>> = HashMap::new();
        map.insert(
            "e1".into(),
            Arc::new(Fake {
                delay: Duration::ZERO,
                result: Ok("OUT".into()),
                calls: Arc::new(AtomicUsize::new(0)),
            }),
        );
        let audit = Arc::new(Mutex::new(Vec::new()));
        let a2 = audit.clone();
        let sink: AuditSink = Arc::new(move |e| a2.lock().unwrap().push(e));
        let svc = JobsService::new(
            Registry::from_specs(vec![s]).unwrap(),
            JobStore::new(dir.join("store")).unwrap(),
            map,
            sink,
            cfg,
        )
        .unwrap();
        svc.start();
        Rig {
            svc,
            audit,
            calls: vec![],
            dir,
        }
    }

    #[tokio::test(start_paused = true)]
    async fn buffer_cap_refuses_uploads_until_acked() {
        let r = rig_with_cfg(JobsConfig {
            max_buffer_bytes: 100,
            ..JobsConfig::default()
        });
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let first = vec![b'a'; 60];
        let (sref, sha) = r.svc.stage_input(&c, "text/plain", &first).unwrap();
        // 60 bytes + the metadata file already count; a second 60 does not fit.
        assert_eq!(
            r.svc.stage_input(&c, "text/plain", &first).unwrap_err(),
            SubmitError::BufferFull
        );
        let id = r
            .svc
            .submit(
                &c,
                SubmitRequest {
                    engine_id: "e1".into(),
                    kind: JobKind::ExtractLlm,
                    input: InputRef {
                        sha256: sha,
                        media_type: "text/plain".into(),
                        staged_ref: Some(sref),
                        local_name: None,
                    },
                    params: Default::default(),
                },
            )
            .unwrap();
        wait_terminal(&r, &c, id).await;
        assert_eq!(r.svc.ack(&c, id), Some(true));
        assert!(
            r.svc.stage_input(&c, "text/plain", &first).is_ok(),
            "space freed by ack"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn reaper_deletes_expired_buffers_and_orphans_but_not_fresh_ones() {
        let r = rig_with_cfg(JobsConfig {
            buffer_ttl_secs: 3600,
            ..JobsConfig::default()
        });
        let c = cred("c", "tok", "mod-a", &["e1"]);
        let id = submit_text(&r, &c, "e1", "hello").unwrap();
        let rec = wait_terminal(&r, &c, id).await;
        let sref = rec.input.staged_ref.unwrap();
        let (orphan, _) = r
            .svc
            .stage_input(&c, "text/plain", b"never submitted")
            .unwrap();

        // Nothing is old enough yet.
        assert_eq!(r.svc.reap_expired(Utc::now()), 0);
        assert!(r.svc.store.read_input(sref).is_ok());

        // Two hours later both the finished job's buffers and the orphan go.
        let later = Utc::now() + chrono::Duration::hours(2);
        assert_eq!(r.svc.reap_expired(later), 2);
        assert!(r.svc.store.read_input(sref).is_err());
        assert!(r.svc.store.read_input(orphan).is_err());
        assert!(r.svc.result(&c, id).is_none());
        let after = r.svc.status(&c, id).unwrap();
        assert!(after.buffer_deleted_at.is_some());
        assert_eq!(
            r.audit.lock().unwrap().last().unwrap().buffer_deleted_at,
            after.buffer_deleted_at
        );
        // Idempotent: a second pass finds nothing.
        assert_eq!(r.svc.reap_expired(later), 0);
    }
}
