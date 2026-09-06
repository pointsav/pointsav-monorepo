// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! app-mediakit-knowledge — knowledge wiki engine.
//!
//! Single-binary HTTP wiki over a flat-file Markdown tree. Structurally
//! modelled on Wikipedia Vector 2022 (sitenotice → white header → article
//! tabs above the title → two-column sidebar + content → institutional
//! footer), with a brand-as-accent visual system per instance.
//!
//! Module map (grows phase by phase — see plan virtual-twirling-parasol):
//!   config   — knowledge.toml schema (P0)
//!   assets   — embedded static/ (P0)
//!   error    — WikiError + response mapping (P0)
//!   app      — AppState + axum Router (P0, grows each phase)
//!   content  — mount / frontmatter / walk / render (P1)
//!   ui       — layout shell + page composition (P2/P3)

pub mod app;
pub mod assets;
pub mod citations;
pub mod claims_store;
pub mod config;
pub mod content;
pub mod discovery;
pub mod editorial;
pub mod error;
pub mod history;
pub mod jsonld;
pub mod legal;
pub mod mcp;
pub mod notice_text;
pub mod search;
pub mod sitedata;
pub mod ui;
pub mod verification;
pub mod wcag_contrast;
