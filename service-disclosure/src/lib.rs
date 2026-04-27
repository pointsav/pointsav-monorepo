// SPDX-License-Identifier: Apache-2.0
//
// service-disclosure — schema + CFG substrate for Foundry editorial work.
//
// Phase 1A: 4-family adapter taxonomy + genre-template enum + ProtocolRequest
// + Frontmatter + Register + frontmatter validator. Operational anchor:
// `~/Foundry/conventions/language-protocol-substrate.md`.
//
// Consumed by `service-proofreader` (project-proofreader cluster) as a Cargo
// dependency. Schema-stable signal lands when Phase 1A unit tests pass and
// the public surface in this file is ratified by Master.

pub mod frontmatter;
pub mod genre;
pub mod request;
pub mod validate;

pub use frontmatter::Frontmatter;
pub use genre::{Family, GenreTemplate};
pub use request::{ProtocolRequest, Register};
pub use validate::{validate_frontmatter, ValidationError};

/// Crate semantic version exposed for downstream version assertions.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Banned vocabulary — the cross-genre prohibited list per
/// `language-protocol-substrate.md` §2.2. Emitted as a CFG fragment by the
/// future Phase 1B work; exposed here for early validator wiring.
pub const BANNED_VOCABULARY: &[&str] = &[
    "leverage",
    "empower",
    "next-generation",
    "industry-leading",
    "seamless",
    "robust",
    "cutting-edge",
    "world-class",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_string_is_non_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn banned_vocabulary_carries_eight_terms() {
        // Locks the v0.1.0 baseline. Phase 1B may grow the list; that is a
        // semver-MINOR change and updates this assertion in the same commit.
        assert_eq!(BANNED_VOCABULARY.len(), 8);
    }
}
