// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # slm-core
//!
//! Shared types, errors, and moduleId discipline for service-slm.
//!
//! ## Role in service-slm
//!
//! Owns shared types, errors, and the moduleId discipline threaded through every call. This is the foundational crate; every other workspace crate depends on it.
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
//! Alpha. The [`ModuleId`] newtype is implemented; remaining shared types
//! (`Error`, RF2 envelope, `Timestamp` / `EventId` / `InputHash`) are future work.
//! See [`../../STATUS.md`] for the machine-readable status of every crate
//! and [`../../TASKS.md`] for the ordered work queue.

mod module_id;

pub use module_id::{ModuleId, ModuleIdError, MAX_LEN as MODULE_ID_MAX_LEN};
