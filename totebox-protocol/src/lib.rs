// SPDX-License-Identifier: Apache-2.0

//! `totebox-protocol` — the unified os-totebox wire protocol.
//!
//! One protocol for both client<->server and fleet server<->server
//! (`BRIEF-os-totebox-communication-complex-2026-09.md` decision 9) --
//! the difference between a local device and a fleet peer is which
//! [`capability::Capability`] their connection carries, not a second
//! protocol. See `frames.rs` for the wire types, `capability.rs` for
//! the deny-by-default enforcement, and `submit_handler.rs` for the
//! one genuinely shared ingestion surface (decision 15).
//!
//! `transport.rs` (QUIC via `quinn`+`rustls`) is the actual network
//! layer -- see that module's own doc for what's implemented in
//! Phase 0 vs. deferred.

pub mod capability;
pub mod frames;
pub mod submit_handler;
pub mod transport;

pub use capability::Capability;
pub use frames::{decode, encode, EnvelopeAppended, EnvelopeBytes, RefusalReason, Request, Response};
pub use submit_handler::{handle_submit, refuse, SubmitContext, SubmitError};
