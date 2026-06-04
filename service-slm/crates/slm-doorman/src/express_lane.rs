// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Express lane — time-sensitive requests that bypass the priority queue.
//!
//! The file-backed priority queue is the right home for background work, but
//! an interactive request cannot wait behind a drain cycle. The express lane
//! dispatches such a request to a GPU node directly. It bypasses the *queue* —
//! it does **not** bypass the *kill switch*. Nothing bypasses the kill switch.
//!
//! ## The cold-start problem and the 202 pattern
//!
//! When an express request arrives and the target node is stopped, the node
//! must boot (2–3 minutes). Holding the caller's HTTP connection open for that
//! long fails the moment the client's socket timeout (often 30s) fires: the
//! connection drops, the request is lost, and the node has started for nothing.
//!
//! Instead the express lane returns **202 Accepted** with a `Location` pointing
//! at a status endpoint, registers a job, and triggers the start in the
//! background. The caller polls the status endpoint until the job completes.
//! This is the standard asynchronous-request-reply pattern; it survives client
//! timeouts because the work is tracked server-side by job id.
//!
//! ## Concurrency slots
//!
//! Each node label has a bounded number of in-flight express slots (an A100
//! can serve more concurrent requests than an L4). A request that cannot get a
//! slot receives 429 rather than queueing — express is for *now*, and a full
//! node means the caller should retry shortly.
//!
//! This module owns the *decision*, the *slots*, and the *job registry*. The
//! HTTP handler performs the actual tier dispatch and the background start; it
//! calls [`ExpressLane::decide`] to learn what to do and the job-registry
//! methods to track a 202 job to completion.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::Serialize;
use tokio::sync::{Semaphore, TryAcquireError};

use crate::flow_gate::FlowGate;
use crate::vm_lifecycle::{VmLifecycle, VmState};

/// Default express concurrency for an express (A100) node.
pub const DEFAULT_EXPRESS_SLOTS: usize = 4;
/// Default express concurrency for a batch (L4) node.
pub const DEFAULT_BATCH_SLOTS: usize = 2;

/// What the HTTP handler should do with an express request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressDecision {
    /// The node is available — dispatch the request inline and return the
    /// result directly.
    DispatchNow,
    /// The node was stopped or staging — a start has been triggered. The
    /// handler returns 202 with the given job id; the caller polls for the
    /// result.
    Accepted { job_id: String },
    /// The kill switch for this label (or the global gate) is closed. Return
    /// 503 with `Retry-After`; the blocking label is carried for the message.
    Rejected { blocking_label: String },
    /// The node is in a failed/zombie state. For chat the handler falls back to
    /// Tier A; for structured extraction (which a small local model cannot do)
    /// the handler returns 503 instead — see [`ExpressDecision::is_fallback`].
    FallbackTierA,
    /// All express slots for this label are in use. Return 429; the caller
    /// retries shortly.
    SlotsFull,
}

impl ExpressDecision {
    /// True for the Tier A fallback decision.
    pub fn is_fallback(&self) -> bool {
        matches!(self, ExpressDecision::FallbackTierA)
    }
}

/// The state of an express job tracked for 202 polling.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum JobState {
    /// Registered; the node is starting.
    Pending,
    /// The node is available and the request is being dispatched.
    Running,
    /// Completed successfully; holds the response payload.
    Done { result: serde_json::Value },
    /// Failed; holds an error message.
    Failed { error: String },
}

/// A per-label concurrency slot guard. Drop to release the slot.
pub struct ExpressSlot {
    _permit: tokio::sync::OwnedSemaphorePermit,
}

/// The express lane: per-label slots plus a job registry for 202 polling.
#[derive(Debug)]
pub struct ExpressLane {
    slots: HashMap<String, Arc<Semaphore>>,
    jobs: Mutex<HashMap<String, JobState>>,
}

impl ExpressLane {
    /// Build an express lane with the given per-label slot capacities.
    pub fn new(slot_caps: HashMap<String, usize>) -> Self {
        let slots = slot_caps
            .into_iter()
            .map(|(label, cap)| (label, Arc::new(Semaphore::new(cap.max(1)))))
            .collect();
        Self {
            slots,
            jobs: Mutex::new(HashMap::new()),
        }
    }

    /// Convenience constructor with the conventional defaults: an `express`
    /// node at [`DEFAULT_EXPRESS_SLOTS`] and a `batch` node at
    /// [`DEFAULT_BATCH_SLOTS`].
    pub fn with_default_labels() -> Self {
        let mut caps = HashMap::new();
        caps.insert("express".to_string(), DEFAULT_EXPRESS_SLOTS);
        caps.insert("batch".to_string(), DEFAULT_BATCH_SLOTS);
        Self::new(caps)
    }

    /// Set (replace) the slot capacity for a label at runtime. Existing
    /// in-flight permits are unaffected; the new cap applies to future
    /// acquisitions.
    pub fn set_capacity(&mut self, label: &str, cap: usize) {
        self.slots
            .insert(label.to_string(), Arc::new(Semaphore::new(cap.max(1))));
    }

    /// Try to acquire an express slot for `label` without waiting. Returns the
    /// guard on success, or `None` when all slots are in use (429) or the label
    /// is unknown.
    pub fn try_acquire_slot(&self, label: &str) -> Option<ExpressSlot> {
        let sem = self.slots.get(label)?.clone();
        match sem.try_acquire_owned() {
            Ok(permit) => Some(ExpressSlot { _permit: permit }),
            Err(TryAcquireError::NoPermits) => None,
            Err(TryAcquireError::Closed) => None,
        }
    }

    /// Number of currently-available slots for a label (for the status panel).
    pub fn available_slots(&self, label: &str) -> Option<usize> {
        self.slots.get(label).map(|s| s.available_permits())
    }

    /// Decide what to do with an express request for `label`, given the flow
    /// gate and the node lifecycle. `requires_tier_b` is true for structured
    /// extraction (which cannot fall back to the local model).
    ///
    /// The order of checks is deliberate:
    /// 1. **Kill switch first** — nothing bypasses it.
    /// 2. **Slots** — a full node returns 429 before we touch lifecycle.
    /// 3. **Lifecycle** — available → dispatch; stopped/staging → 202 + start;
    ///    failed/zombie → fall back (chat) or refuse (extract).
    ///
    /// On the 202 path this registers a [`JobState::Pending`] job and returns
    /// its id; it does **not** start the VM itself (the handler triggers the
    /// background start so it can also drive [`ExpressLane::complete_job`]).
    pub fn decide(
        &self,
        gate: &FlowGate,
        vm: &VmLifecycle,
        label: &str,
        requires_tier_b: bool,
    ) -> ExpressDecision {
        // 1. Kill switch — inviolable.
        if let Some(blocking) = gate.blocking_label(label) {
            return ExpressDecision::Rejected {
                blocking_label: blocking,
            };
        }

        // 2. Slots — fail fast if the node is saturated. (We check via
        //    available_permits to avoid consuming a permit during the decision;
        //    the handler acquires the real guard once it commits to dispatch.)
        if let Some(avail) = self.available_slots(label) {
            if avail == 0 {
                return ExpressDecision::SlotsFull;
            }
        }

        // 3. Lifecycle.
        match vm.current() {
            VmState::Available => ExpressDecision::DispatchNow,
            VmState::Stopped | VmState::Staging | VmState::Unknown | VmState::Running => {
                // Node not ready yet — register a job and let the handler kick
                // off the start. Caller polls.
                let job_id = self.create_job();
                ExpressDecision::Accepted { job_id }
            }
            VmState::FailedStart | VmState::Zombie => {
                if requires_tier_b {
                    // Structured extraction has no local fallback — surface as
                    // a rejection the handler maps to 503.
                    ExpressDecision::Rejected {
                        blocking_label: label.to_string(),
                    }
                } else {
                    ExpressDecision::FallbackTierA
                }
            }
        }
    }

    /// Register a new job in [`JobState::Pending`] and return its id.
    pub fn create_job(&self) -> String {
        let id = uuid::Uuid::now_v7().to_string();
        self.jobs
            .lock()
            .expect("express jobs lock poisoned")
            .insert(id.clone(), JobState::Pending);
        id
    }

    /// Mark a job as running (the node became available, dispatch started).
    pub fn set_running(&self, job_id: &str) {
        if let Some(s) = self
            .jobs
            .lock()
            .expect("express jobs lock poisoned")
            .get_mut(job_id)
        {
            *s = JobState::Running;
        }
    }

    /// Complete a job successfully with its result payload.
    pub fn complete_job(&self, job_id: &str, result: serde_json::Value) {
        self.jobs
            .lock()
            .expect("express jobs lock poisoned")
            .insert(job_id.to_string(), JobState::Done { result });
    }

    /// Fail a job with an error message.
    pub fn fail_job(&self, job_id: &str, error: impl Into<String>) {
        self.jobs
            .lock()
            .expect("express jobs lock poisoned")
            .insert(
                job_id.to_string(),
                JobState::Failed {
                    error: error.into(),
                },
            );
    }

    /// Look up a job's current state for the status endpoint. Returns `None`
    /// for an unknown id (404).
    pub fn job_status(&self, job_id: &str) -> Option<JobState> {
        self.jobs
            .lock()
            .expect("express jobs lock poisoned")
            .get(job_id)
            .cloned()
    }

    /// Remove terminal (Done/Failed) jobs older than the registry should keep.
    /// The handler can call this periodically; here it simply drops all
    /// terminal jobs, which is sufficient because the caller has already
    /// polled the result by the time cleanup runs. Returns the number removed.
    pub fn reap_terminal_jobs(&self) -> usize {
        let mut g = self.jobs.lock().expect("express jobs lock poisoned");
        let before = g.len();
        g.retain(|_, s| matches!(s, JobState::Pending | JobState::Running));
        before - g.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flow_gate::FlowGate;
    use crate::vm_lifecycle::VmLifecycle;
    use std::time::Duration;

    fn gate_dir(label: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "slm-express-{label}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn vm_available() -> VmLifecycle {
        let l = VmLifecycle::new("express", Duration::from_secs(120)).0;
        l.transition(VmState::Stopped);
        l.try_begin_start();
        l.finish_start(true); // Available
        l
    }

    fn vm_stopped() -> VmLifecycle {
        let l = VmLifecycle::new("express", Duration::from_secs(120)).0;
        l.transition(VmState::Stopped);
        l
    }

    #[test]
    fn available_node_dispatches_now() {
        let dir = gate_dir("dispatch");
        let gate = FlowGate::new(&dir, ["express"]).unwrap();
        let vm = vm_available();
        let lane = ExpressLane::with_default_labels();
        let d = lane.decide(&gate, &vm, "express", false);
        assert_eq!(d, ExpressDecision::DispatchNow);
    }

    #[test]
    fn stopped_node_returns_accepted_with_job() {
        let dir = gate_dir("accepted");
        let gate = FlowGate::new(&dir, ["express"]).unwrap();
        let vm = vm_stopped();
        let lane = ExpressLane::with_default_labels();
        match lane.decide(&gate, &vm, "express", false) {
            ExpressDecision::Accepted { job_id } => {
                assert!(matches!(lane.job_status(&job_id), Some(JobState::Pending)));
            }
            other => panic!("expected Accepted, got {other:?}"),
        }
    }

    #[test]
    fn closed_kill_switch_rejects_even_when_available() {
        let dir = gate_dir("killed");
        let gate = FlowGate::new(&dir, ["express"]).unwrap();
        gate.set("express", true).unwrap(); // close the gate
        let vm = vm_available();
        let lane = ExpressLane::with_default_labels();
        let d = lane.decide(&gate, &vm, "express", false);
        assert!(matches!(d, ExpressDecision::Rejected { .. }));
    }

    #[test]
    fn global_kill_switch_rejects() {
        let dir = gate_dir("global");
        let gate = FlowGate::new(&dir, ["express"]).unwrap();
        gate.set(crate::flow_gate::GLOBAL_LABEL, true).unwrap();
        let vm = vm_available();
        let lane = ExpressLane::with_default_labels();
        match lane.decide(&gate, &vm, "express", false) {
            ExpressDecision::Rejected { blocking_label } => assert_eq!(blocking_label, "global"),
            other => panic!("expected global Rejected, got {other:?}"),
        }
    }

    #[test]
    fn failed_node_falls_back_for_chat() {
        let dir = gate_dir("fallback");
        let gate = FlowGate::new(&dir, ["express"]).unwrap();
        let vm = VmLifecycle::new("express", Duration::from_secs(120)).0;
        vm.transition(VmState::Stopped);
        vm.try_begin_start();
        vm.finish_start(false); // FailedStart
        let lane = ExpressLane::with_default_labels();
        let d = lane.decide(&gate, &vm, "express", false);
        assert_eq!(d, ExpressDecision::FallbackTierA);
    }

    #[test]
    fn failed_node_refuses_extract_no_fallback() {
        let dir = gate_dir("no-fallback");
        let gate = FlowGate::new(&dir, ["express"]).unwrap();
        let vm = VmLifecycle::new("express", Duration::from_secs(120)).0;
        vm.transition(VmState::Stopped);
        vm.try_begin_start();
        vm.finish_start(false); // FailedStart
        let lane = ExpressLane::with_default_labels();
        // requires_tier_b = true (structured extraction).
        let d = lane.decide(&gate, &vm, "express", true);
        assert!(matches!(d, ExpressDecision::Rejected { .. }));
    }

    #[test]
    fn slots_exhausted_returns_full() {
        let dir = gate_dir("slots");
        let gate = FlowGate::new(&dir, ["express"]).unwrap();
        let vm = vm_available();
        let mut caps = HashMap::new();
        caps.insert("express".to_string(), 1usize);
        let lane = ExpressLane::new(caps);
        // Hold the only slot.
        let _held = lane.try_acquire_slot("express").unwrap();
        let d = lane.decide(&gate, &vm, "express", false);
        assert_eq!(d, ExpressDecision::SlotsFull);
    }

    #[test]
    fn slot_released_on_drop() {
        let mut caps = HashMap::new();
        caps.insert("express".to_string(), 1usize);
        let lane = ExpressLane::new(caps);
        {
            let _held = lane.try_acquire_slot("express").unwrap();
            assert_eq!(lane.available_slots("express"), Some(0));
        }
        assert_eq!(lane.available_slots("express"), Some(1));
    }

    #[test]
    fn job_lifecycle_pending_running_done() {
        let lane = ExpressLane::with_default_labels();
        let id = lane.create_job();
        assert!(matches!(lane.job_status(&id), Some(JobState::Pending)));
        lane.set_running(&id);
        assert!(matches!(lane.job_status(&id), Some(JobState::Running)));
        lane.complete_job(&id, serde_json::json!({"ok": true}));
        match lane.job_status(&id).unwrap() {
            JobState::Done { result } => assert_eq!(result["ok"], true),
            other => panic!("expected Done, got {other:?}"),
        }
    }

    #[test]
    fn job_failure_recorded() {
        let lane = ExpressLane::with_default_labels();
        let id = lane.create_job();
        lane.fail_job(&id, "boot timeout");
        match lane.job_status(&id).unwrap() {
            JobState::Failed { error } => assert_eq!(error, "boot timeout"),
            other => panic!("expected Failed, got {other:?}"),
        }
    }

    #[test]
    fn unknown_job_is_none() {
        let lane = ExpressLane::with_default_labels();
        assert!(lane.job_status("no-such-id").is_none());
    }

    #[test]
    fn reap_removes_terminal_jobs_only() {
        let lane = ExpressLane::with_default_labels();
        let pending = lane.create_job();
        let done = lane.create_job();
        lane.complete_job(&done, serde_json::json!({}));
        let removed = lane.reap_terminal_jobs();
        assert_eq!(removed, 1);
        assert!(lane.job_status(&pending).is_some());
        assert!(lane.job_status(&done).is_none());
    }

    #[test]
    fn set_capacity_changes_slots() {
        let mut lane = ExpressLane::with_default_labels();
        lane.set_capacity("express", 6);
        assert_eq!(lane.available_slots("express"), Some(6));
    }
}
