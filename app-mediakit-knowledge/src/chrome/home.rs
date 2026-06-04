//! Homepage chrome — per-instance differentiated home page layouts.
//!
//! Phase 1 stub. Phase 3 implements three distinct homepage layouts:
//!
//! - **documentation.pointsav.com**: category grid (9 tiles, article count,
//!   scope description from `content/category-config.yaml`) + featured article
//!   rotation strip below.
//!
//! - **projects.woodfinegroup.com**: thematic cluster cards ("Location
//!   Intelligence", "Regional Markets", "Co-location Archetypes") with 3–4
//!   article cards per cluster, each using the `summary:` frontmatter field.
//!   A "Start here" card pins the foundational article per cluster.
//!
//! - **corporate.woodfinegroup.com**: two-column layout — left column is the
//!   "Due Diligence Path" (ordered 5-article sequence with `status:` badges);
//!   right column is "Browse by subject" (category links with counts). Explicit
//!   "If this is your first visit" link at top.

use maud::Markup;
use crate::server::AppState;

/// Render the homepage for the given instance and locale.
///
/// Phase 1 stub. Phase 3 dispatches to the per-instance template based on
/// `state.brand_instance` ("documentation" | "projects" | "corporate").
#[allow(unused_variables)]
pub fn home_page(state: &AppState, locale: &str) -> Markup {
    todo!("Phase 3: implement home_page with per-instance differentiation (§17.2)")
}
