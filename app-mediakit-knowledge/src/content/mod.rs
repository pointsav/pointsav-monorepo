// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Content pipeline — mounts, frontmatter, walk/index, and rendering.
//!
//! Markdown files in a Git tree are the source of truth; the `ContentIndex`
//! is a derived, regenerable slug → file lookup built at startup.

pub mod claims;
pub mod frontmatter;
pub mod index_topic;
pub mod mount;
pub mod render;
pub mod walk;

pub use claims::{extract_claims, Claim, Confidence};
pub use frontmatter::{parse, parse_raw, Frontmatter, ParsedDoc};
pub use index_topic::{parse_index_topic, Group, IndexTopic, Member, StartHere};
pub use mount::{Mount, MountSet};
pub use render::{
    render, render_doc, render_journal_doc, resolve_citations, syntax_css, Heading, Rendered,
};
pub use walk::{load, load_raw, ContentIndex, DocRef, Lang};
