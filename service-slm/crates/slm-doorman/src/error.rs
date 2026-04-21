// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2026 Woodfine Capital Projects Inc.

//! Error types for the doorman sanitisation layer.

use thiserror::Error;

/// Errors produced during sanitisation or rehydration.
///
/// Per the doorman invariant, sanitisation failure is never silent. Any payload
/// that cannot be proven safe to cross the trust boundary is refused with
/// [`SanitisationError::Refused`]. A refused payload never reaches external
/// compute.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SanitisationError {
    /// The policy determined the payload is not safe to send to external compute.
    ///
    /// This is the correct outcome when a field that should be stripped cannot
    /// be identified unambiguously. Prefer a refusal over a degraded response.
    #[error("sanitisation refused: {reason}")]
    Refused {
        /// Human-readable explanation of why the payload was refused.
        reason: String,
    },

    /// The stripped context cannot be reattached to the inbound response.
    ///
    /// Indicates that rehydration is out of sync with sanitisation — a protocol
    /// invariant violation. Treat as a hard error; do not attempt a fallback.
    #[error("rehydration failed: {reason}")]
    Rehydration {
        /// Human-readable explanation of why rehydration failed.
        reason: String,
    },
}
