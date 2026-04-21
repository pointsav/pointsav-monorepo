// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # slm-compute
//!
//! Ring 1 bootstrap layer for service-slm — Cloud Run `GPU` driver and
//! container management.
//!
//! ## Role in service-slm
//!
//! Owns Cloud Run spin-up and tear-down, container manifest parsing, weights
//! registry loading, warm-pool toggle, and Secret Manager reference
//! resolution. Every bootstrap operation emits `BOOT_REQUEST` and
//! `BOOT_COMPLETE` (or an error row) via `slm-ledger`.
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
//! Alpha. [`ComputeManifest`] parsing and validation are implemented.
//! Cloud Run driver, warm-pool toggle, and Secret Manager integration are
//! future work. See [`../../STATUS.md`] for the machine-readable status of
//! every crate and [`../../TASKS.md`] for the ordered work queue.

mod error;
mod manifest;

pub use error::ManifestError;
pub use manifest::{ComputeManifest, GpuTier};
