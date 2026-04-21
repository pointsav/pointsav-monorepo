// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # slm-ledger
//!
//! Append-only audit ledger for service-slm with SOC3 processing-integrity semantics.
//!
//! ## Role in service-slm
//!
//! Owns the [`Event`] type and its ten [`EventType`] variants, an append-only
//! CSV writer with fsync semantics, and a `SQLite` mirror for audit queries.
//! Every inference event, every adapter load, every teardown writes a row.
//! This is the SOC3 processing-integrity artefact described in YOYO-COMPUTE §5.
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
//! Alpha. [`Event`], [`EventType`], and [`LedgerWriter`] are implemented.
//! The `SQLite` mirror is future work. See [`../../STATUS.md`] for the
//! machine-readable status of every crate and [`../../TASKS.md`] for the
//! ordered work queue.

mod error;
mod event;
mod event_type;
mod reader;
mod writer;

pub use error::LedgerError;
pub use event::Event;
pub use event_type::{EventType, EventTypeParseError};
pub use reader::tail as tail_events;
pub use writer::LedgerWriter;
