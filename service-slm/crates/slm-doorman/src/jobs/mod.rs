// SPDX-License-Identifier: Apache-2.0

//! Async job system behind `/v1/jobs`.
//!
//! A caller uploads or references an input, submits a job naming exactly one
//! registered engine, polls for the outcome, collects the result and acks it
//! (which deletes the staged buffer). There is never a fallback from one
//! engine to another: an unavailable engine yields `deferred`.
//!
//! Credentials bind `module_id`, the engine allow-list and the priority class
//! server-side; nothing in a request can widen them.

pub mod credentials;
pub mod engines;
pub mod prompts;
pub mod registry;
pub mod service;
pub mod store;
pub mod types;
pub mod witness;

pub use credentials::{Credential, CredentialSet};
pub use engines::{
    AdmissionGate, EngineError, EngineOutput, JobEngine, LlmLocalEngine, OcrOpenaiVisionEngine,
    SpanGlinerEngine,
};
pub use registry::{EngineKind, EngineSpec, Registry};
pub use service::{JobsConfig, JobsService, SubmitError};
pub use store::JobStore;
pub use types::{JobId, JobKind, JobRecord, JobStatus, StagedRef};

/// Lowercase hex of arbitrary bytes.
pub(crate) fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

/// SHA-256 of `data` as lowercase hex.
pub fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    hex(&Sha256::digest(data))
}
