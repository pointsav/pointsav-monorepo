// SPDX-License-Identifier: Apache-2.0

//! `envelope` — the Envelope record: one signed schema for "an
//! assertion about entities at a time, with source, observed-vs-
//! attested state, and lineage." See `envelope.rs`'s module doc for
//! the full rationale and `BRIEF-os-totebox-communication-complex-
//! 2026-09.md` decision 8.

pub mod envelope;
pub mod signing;

pub use envelope::{
    AttachmentRef, AttestedBy, Envelope, EnvelopeError, EnvelopeState, EntityRef, Lineage,
    Payload, Source,
};
pub use signing::{sign_envelope, verify_envelope_signature, SigningError, VerifyError, ENVELOPE_NAMESPACE};
