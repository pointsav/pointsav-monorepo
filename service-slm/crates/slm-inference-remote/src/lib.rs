// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # slm-inference-remote
//!
//! Remote GCP yo-yo inference driver for service-slm.
//!
//! ## Role in service-slm
//!
//! GCP yo-yo driver: spin up, run job, tear down, record ledger entries.
//! The remote counterpart to `slm-inference-local`. This crate owns the
//! HTTP client, the phase-transition ledger rows (boot / job / teardown /
//! preemption), and the retry / backoff policy.
//!
//! ## Boundaries
//!
//! This crate does not own behaviour that belongs to other crates in the
//! workspace. Refer to the per-crate `CLAUDE.md` and to
//! [`../../specs/SLM-STACK.md`] and [`../../specs/YOYO-COMPUTE.md`] for the
//! full responsibility map.
//!
//! ## Status
//!
//! Alpha. [`RemoteInferenceClient::boot`] is implemented and writes
//! `BOOT_REQUEST` / `BOOT_COMPLETE` rows via `slm-ledger`. Retry policy,
//! `JOB_*`, `TEARDOWN_*`, and `PREEMPTION` events are future work. See
//! [`../../STATUS.md`] for the machine-readable status of every crate in
//! the workspace and [`../../TASKS.md`] for the ordered work queue.

mod client;
mod config;
mod error;

pub use client::{NodeHandle, RemoteInferenceClient};
pub use config::{ConfigError, RemoteInferenceConfig};
pub use error::RemoteInferenceError;
