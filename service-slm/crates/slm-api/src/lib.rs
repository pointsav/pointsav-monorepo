// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # slm-api
//!
//! HTTP API surface for service-slm.
//!
//! ## Role in service-slm
//!
//! Owns the axum router, handler signatures, tower middleware stack, and
//! health/readiness/metrics endpoints. Handlers stay thin — they parse
//! input, call one library crate, and format the response. Real work lives
//! in the library crates.
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
//! Alpha. Router skeleton with `GET /health` and a [`tower_http`] tracing
//! layer. Additional routes are added as library crates reach alpha status.
//! See [`../../STATUS.md`] for the machine-readable status of every crate
//! and [`../../TASKS.md`] for the ordered work queue.

mod health;
mod router;

pub use router::router;
