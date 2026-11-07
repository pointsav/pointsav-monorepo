// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//
//! tool-precedent-schedule: the scheduling member of the `tool-precedent-*`
//! family. Pure critical-path-method (CPM) compute, no HTTP or storage.
//! Moved from `service-schedule/src/cpm.rs` (project-construction; only change:
//! `BTreeMap`/`BTreeSet` for `HashMap`/`HashSet`, see below)
//! so the Workbench Schedule pane, `service-schedule`'s daemon and the
//! planner all share one implementation. Units are caller-supplied day
//! offsets; links are finish-to-start only; no calendars or resources.
//!
//! `no_std` + `alloc` capable: the default `std` feature only controls whether the crate
//! links std (for the `serde` feature's std support); the engine itself uses `alloc` and
//! `BTreeMap`/`BTreeSet`, so bare-metal protection domains can build with
//! `--no-default-features`. Iteration order of internal maps never reaches the output
//! (ready tasks are sorted), so results are identical to the earlier `HashMap` build.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod cpm;

pub use cpm::{compute, ComputedTask, CpmError, Task};
