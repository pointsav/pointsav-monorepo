// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # slm-memory-kv
//!
//! Ring 2 working-memory client for service-slm — `LMCache` + Mooncake Store.
//!
//! ## Role in service-slm
//!
//! Client for the `LMCache` + Mooncake Store wire protocol. Talks to the Mooncake master as a sidecar, never links it as a library.
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
//! Scaffold only. No business logic has been written yet. See
//! [`../../STATUS.md`] for the machine-readable status of every crate in
//! the workspace and [`../../TASKS.md`] for the ordered work queue.

/// Placeholder entry point to keep the crate compiling during scaffolding.
///
/// Remove this once real items are added. Tests importing it should be
/// deleted at the same time.
pub fn __scaffold_placeholder() {}
