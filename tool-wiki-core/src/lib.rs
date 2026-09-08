//! tool-wiki-core — the reusable crawler/walker for wiki content trees (BRIEF-tool-wiki
//! plan Part 1). Parses a wiki content directory (frontmatter, headings, wikilinks, tables,
//! code blocks, AUTO-GENERATED MEMBERSHIP blocks) into a structured in-memory representation.
//!
//! Deliberately free of editorial-rule-checking logic — the token engine that reads this
//! crate's output is a separate module, not yet started (gated on the tool-wiki plan's Part 2
//! token-format decision, which this crawler does not depend on). Other `project-*` archives
//! that need to walk wiki content structurally can depend on this crate alone.

pub mod document;
pub mod frontmatter;
pub mod markdown;
pub mod crawler;
pub mod regenerate;
pub mod wiki_config;

pub use crawler::{crawl, CrawlError};
pub use document::Document;
pub use frontmatter::Frontmatter;
pub use regenerate::{check_membership, MembershipDrift};
pub use wiki_config::{
    find_category_title_drift, find_dead_redirects, find_orphan_categories,
    find_redirect_chains, find_unknown_categories, is_reserved_category, parse_categories,
    parse_redirects, CategoryEntry, CategoryTitleDrift, DeadRedirect, OrphanCategory,
    RedirectChain, RedirectRule, UnknownCategory, RESERVED_CATEGORY_IDS,
};
