// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! # slm-memory-adapters
//!
//! Ring 3b `LoRA` adapter registry and loader for service-slm.
//!
//! The adapter library is the compounding commercial asset: every project
//! leaves behind a versioned, Sigstore-signed `LoRA` adapter stored as an OCI
//! Artifact. This crate owns the registry schema, validation, OCI fetch, and
//! Sigstore verification. See `specs/YOYO-COMPUTE.md §4` for the full design.

mod registry;

pub use registry::{AdapterEntry, Registry, RegistryError};
