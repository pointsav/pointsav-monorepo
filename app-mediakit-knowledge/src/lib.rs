//! Wiki engine library — render, server, assets, error.
//!
//! See ARCHITECTURE.md for design and the build-phase plan. Phase 1
//! ships render + server + embedded static assets. Later phases add
//! search (tantivy), git-sync (git2 + gix), auth (tower-sessions +
//! axum-login), wikilink resolution, federation seam (blake3), and
//! disclosure mode (iXBRL + OpenTimestamps + RFC 3161).

pub mod assets;
pub mod edit;
pub mod error;
pub mod jsonld;
pub mod render;
pub mod server;
pub mod squiggle;
