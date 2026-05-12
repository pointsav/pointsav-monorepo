//! Wiki engine library — render, server, assets, error.
//!
//! See ARCHITECTURE.md for design and the build-phase plan. Phase 1
//! ships render + server + embedded static assets. Later phases add
//! search (tantivy), git-sync (git2 + gix), auth (tower-sessions +
//! axum-login), wikilink resolution, federation seam (blake3), and
//! disclosure mode (iXBRL + OpenTimestamps + RFC 3161).

pub mod assets;
pub mod auth;
pub mod citations;
pub mod collab;
pub mod config;
pub mod edit;
pub mod error;
pub mod feeds;
pub mod git;
pub mod glossary;
pub mod history;
pub mod git_protocol;
pub mod jsonld;
pub mod mcp;
pub mod links;
pub mod pending;
pub mod render;
pub mod search;
pub mod server;
pub mod squiggle;
pub mod users;
