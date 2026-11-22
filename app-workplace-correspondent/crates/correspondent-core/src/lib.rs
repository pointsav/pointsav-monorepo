// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Read-only reader for the `service-people` and `service-business` registries.
//!
//! The registries are CSV folders (`index.csv` is the primary table). This crate
//! reads them in place by path. It never writes, copies or creates data.
//!
//! Rules taken from project-email's registry constitution:
//! - columns are append-only, so unknown columns are ignored;
//! - a missing required column or file fails loudly with a clear error;
//! - identifiers are opaque strings, never parsed or derived.
//!
//! - when a folder has a `schema.csv`, `index.csv` is checked against it (header
//!   order and `required` columns); a folder without one is not checked.
//!
//! - `service-people/claims.csv`, when present, supplies name variants (searched)
//!   and company claims (linked to a business by provisional name match). All of
//!   it follows PROPOSED rules; see the `claims` module.
//!
//! Not covered yet: `redirects.csv`, `relationships.csv`, and history from
//!   `participations`.

mod claims;
mod error;
mod registry;

pub use claims::{normalise_name, Claim};
pub use error::Error;
pub use registry::{Business, Person, Registry};
