// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! The [`SanitisationPolicy`] trait and the [`NoOp`] pass-through implementation.
//!
//! A `SanitisationPolicy` defines the two-sided contract at the trust boundary:
//! what leaves PointSav-trusted space (sanitise) and what returns to it
//! (rehydrate). Every implementation must satisfy the round-trip property:
//!
//! ```text
//! rehydrate(fst(sanitise(payload)), snd(sanitise(payload))) == payload
//! ```
//!
//! The [`NoOp`] implementation strips nothing and is intended for testing and
//! for contexts where no fields require protection. Real policies will use
//! domain-specific payload types and explicit field lists.

use crate::error::SanitisationError;

/// Defines the sanitise / rehydrate contract for one trust boundary crossing.
///
/// ## Contract
///
/// For any correct implementation:
///
/// ```text
/// let (sanitised, ctx) = policy.sanitise(payload)?;
/// let recovered        = policy.rehydrate(sanitised, ctx)?;
/// assert_eq!(recovered, payload);
/// ```
///
/// This property is verified by property tests on every concrete implementation.
/// If sanitise and rehydrate ever drift out of sync, the test suite catches it
/// before a production deployment can expose the divergence.
///
/// ## No associated-type bounds at the trait level
///
/// `Payload` and `Context` carry no trait bounds here. Bounds are applied at
/// call sites via `where` clauses, which allows implementors to use payload
/// types that are not `Clone` or `PartialEq` — for example, a payload backed
/// by an `Arc<[u8]>` for zero-copy efficiency.
pub trait SanitisationPolicy {
    /// The payload type that crosses the trust boundary.
    type Payload;

    /// The fields stripped from `Payload` during sanitisation, held locally
    /// until rehydration restores them.
    type Context;

    /// Strips sensitive fields from `payload` before it is sent to external compute.
    ///
    /// Returns the outbound payload (safe to send) and a context value holding
    /// whatever was removed. The context is never transmitted; it remains inside
    /// the trust boundary until [`rehydrate`](Self::rehydrate) restores it.
    ///
    /// # Errors
    ///
    /// Returns [`SanitisationError::Refused`] if the payload cannot be proven
    /// safe to transmit. A refusal is always the correct response to ambiguity;
    /// never degrade silently.
    fn sanitise(
        &self,
        payload: Self::Payload,
    ) -> Result<(Self::Payload, Self::Context), SanitisationError>;

    /// Restores stripped fields to the inbound `response` using the saved `context`.
    ///
    /// Called after external compute returns. `context` is the value produced by
    /// the corresponding [`sanitise`](Self::sanitise) call. The recovered value
    /// must equal the original pre-sanitise payload for a correct implementation.
    ///
    /// # Errors
    ///
    /// Returns [`SanitisationError::Rehydration`] if the context cannot be
    /// reattached — for example, if the response shape is incompatible with the
    /// stripped fields.
    fn rehydrate(
        &self,
        response: Self::Payload,
        context: Self::Context,
    ) -> Result<Self::Payload, SanitisationError>;
}

/// A pass-through [`SanitisationPolicy`] that strips nothing.
///
/// `sanitise` returns the payload unchanged with an empty context `()`.
/// `rehydrate` returns the response unchanged. The round-trip property holds
/// trivially, which makes `NoOp` useful as a test double and as a starting
/// point when no fields require protection.
///
/// Real policies replace `NoOp` with an implementation that enumerates the
/// specific fields to strip and the rules for restoring them.
pub struct NoOp;

impl SanitisationPolicy for NoOp {
    /// A plain `String` payload is sufficient for the pass-through case and for
    /// property testing. Future implementations will use structured payload types.
    type Payload = String;
    /// Nothing is stripped, so no context is needed.
    type Context = ();

    fn sanitise(&self, payload: String) -> Result<(String, ()), SanitisationError> {
        Ok((payload, ()))
    }

    fn rehydrate(&self, response: String, _context: ()) -> Result<String, SanitisationError> {
        Ok(response)
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn noop_sanitise_always_succeeds() {
        let result = NoOp.sanitise("hello doorman".to_owned());
        assert!(result.is_ok());
        let (sanitised, _ctx) = result.unwrap();
        assert_eq!(sanitised, "hello doorman");
    }

    #[test]
    fn noop_rehydrate_always_succeeds() {
        let result = NoOp.rehydrate("hello doorman".to_owned(), ());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello doorman");
    }

    #[test]
    fn noop_empty_payload_round_trips() {
        let (sanitised, ctx) = NoOp.sanitise(String::new()).unwrap();
        let recovered = NoOp.rehydrate(sanitised, ctx).unwrap();
        assert_eq!(recovered, String::new());
    }

    proptest! {
        /// Verifies the core doorman contract for [`NoOp`]:
        /// stripping nothing and reattaching nothing is the identity transformation.
        #[test]
        fn noop_sanitise_rehydrate_is_identity(payload in any::<String>()) {
            let (sanitised, ctx) = NoOp
                .sanitise(payload.clone())
                .expect("NoOp sanitise never fails");
            let recovered = NoOp
                .rehydrate(sanitised, ctx)
                .expect("NoOp rehydrate never fails");
            prop_assert_eq!(recovered, payload);
        }
    }
}
