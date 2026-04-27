//! moonshot-toolkit — Rust-only build orchestrator for Foundry's
//! seL4 unikernel images.
//!
//! Per MEMO §7 ("Microkit (Python/CMake) → moonshot-toolkit
//! (Rust-Only Toolchain)") and convention
//! `system-substrate-doctrine.md` §6 (Reproducible-Verification-On-
//! Customer-Metal).
//!
//! # Module layout
//!
//! - [`spec`] — `SystemSpec` Rust-native equivalent of Microkit
//!   2.2.0's system-description XML; TOML parser + validation
//! - [`plan`] — `BuildPlan` deterministic content-addressed
//!   manifest (lands in cluster task #36)
//!
//! The CLI (`main.rs`) consumes both modules to provide
//! `validate` / `plan` / `build` subcommands.

pub mod spec;
