// SPDX-License-Identifier: LicenseRef-PointSav-Proprietary

//! Business logic for `app-orchestration-command` — the CommandCentre hub.
//!
//! Modules:
//! - `error`   — `CommandError` type shared across modules
//! - `license` — Ed25519 license token verification (startup gate)
//! - `fleet`   — Reads pairings.yaml + manifests; exposes `Vec<ArchiveEntry>`
//! - `personnel` — Reads pairings.yaml contributor tiers
//! - `invite`  — Issues and verifies Ed25519-signed one-time invite tokens
//! - `pairing` — Records completed pairings; WORM ledger append
//! - `routing` — Confused-deputy-safe cross-archive message routing
//! - `child`   — Spawns and monitors `app-orchestration-slm` as a child process

pub mod error;
pub mod fleet;
pub mod invite;
pub mod license;
pub mod pairing;
pub mod personnel;
pub mod routing;
pub mod child;

pub use error::CommandError;
pub use license::{LicenseStatus, resolve_from_env};
