// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # slm-doorman
//!
//! Doorman protocol for service-slm: sanitise / send / receive / rehydrate.
//!
//! ## Role in service-slm
//!
//! Every external call to service-slm flows through this crate. It owns the
//! five-step doorman protocol, the sanitisation rules (what fields are stripped
//! before data crosses the trust boundary), the rehydration logic (what fields
//! are re-attached on return), and the retry, timeout, and backoff policies.
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
//! Alpha. [`SanitisationPolicy`] trait and [`NoOp`] pass-through implementation
//! are present with property tests. The full five-step cycle, ledger integration,
//! and retry/backoff policies are future work. See [`../../STATUS.md`] for the
//! machine-readable status of every crate and [`../../TASKS.md`] for the ordered
//! work queue.

mod error;
mod policy;

pub use error::SanitisationError;
pub use policy::{NoOp, SanitisationPolicy};
