---
artifact: brief
status: archived
---

# Final UI/UX Blueprint: A-Grade Knowledge Platform

## 1. Executive Summary
This blueprint defines the architectural and UI/UX unification for `corporate.woodfinegroup.com`, `projects.woodfinegroup.com`, and `documentation.pointsav.com`. By aligning these sites with our existing high-fidelity `pointsav-design-system` tokens, we aim to achieve an "A-" grade user experience optimized for professional audiences (Goldman Sachs, Google Developers, Bloomberg Terminal users).

## 2. Research Foundations (Wikipedia & Codex-Inspired)
- **Predictability & Density:** Information must follow a standardized hierarchy.
- **DTCG-Compliant Tokens:** Utilize our existing 3-tier Design Token system (`primitive`, `semantic`, `component`).
- **Functionality:** Sticky ToC, Infobox-metadata, and sortable data tables are mandatory for professional usability.

## 3. 2026 Enterprise UX Trends (Augmented Research)
To stay competitive through 2026, we are integrating:
- **Agentic UX:** Moving from passive search to autonomous task orchestration.
- **Outcome-Driven Design:** Prioritizing "time-to-value" over simple engagement.
- **Explainable AI (XAI):** UI must provide transparency in how AI-synthesized content is reached.
- **Liquid UX:** Design for cross-device context continuity (mobile, desktop, and spatial).

## 4. Unification Strategy: Home-Page Cohesion
To provide a seamless experience across sites:
- **Unified Navigation:** Implement a shared, cross-domain header that links the three knowledge portals, ensuring navigation predictability.
- **Design Token Application:** Apply `theme-pointsav` and `theme-woodfine` respectively across properties, utilizing the semantic `wiki.*` namespace.
- **Home Grid Pattern:** Adopt the `component.home-grid` (9-card layout) across all property home pages, ensuring a familiar browse pattern.

## 5. Audience Benchmarking (Goldman, Google, Bloomberg)
- **High-Trust Signals:** Use consistent typography (`Linux Libertine` headers) and clear, audit-ready citation links.
- **Financial/Technical Efficiency:** Data-tables must support advanced filtering/sorting, bridging the gap between "read-only" and "data-active" modes.
- **Accessibility:** Ensure all content meets WCAG 4.5:1 standards, prioritizing keyboard navigation and screen-reader friendliness.

## 6. Implementation Blueprint: Design Token Architecture
- **Tokens as Authority:** All new components must pull from `semantic` namespaces.
- **Dynamic Theming:** Leverage our `theme-*.yaml` system to toggle between PointSav and Woodfine visual identities while maintaining core structural layout tokens.

## 7. Actionable Blueprint (For AI Execution)
- [ ] Standardize header navigation across all three portals using `pointsav-design-system` tokens.
- [ ] Migrate home page layout to the unified `home-grid` structure.
- [ ] Audit and fix all low-contrast typography using the updated `wiki.freshness-ribbon` color tokens.
- [ ] Implement standardized "Infoboxes" for all major entities across domains.
- [ ] Consolidate all site-wide CSS into the shared `pointsav-design-system/tokens/css/` directory.
- [ ] Integrate XAI components to show content-synthesis provenance.

## 8. Version History
- 2026-05-20: Initial consolidation and final blueprint synthesis.
- 2026-05-20 (Post-Audit): Augmented with 2026 Agentic UX trends.
# Research Sprint Report: UX Standards for Technical & Financial Platforms

## 1. Typography & Legibility
*   **Font-stacks:** Serifs for headers (`Linux Libertine`, `Georgia`) to evoke authority. Sans-serifs (`Helvetica`, `Inter`, `Montserrat`) for body text for clarity at all scales.
*   **Scale:** Base body size of 16px (1rem). Type scale ratio 1.25x or 1.414x for clear hierarchy.
*   **Long-form:** Line height of 1.5–1.6, line length 50–75 characters for optimal cognitive flow.

## 2. Information Density & Layout
*   **Grid Systems:** Max-width constraints on content areas to avoid "wall of text" syndrome.
*   **Hierarchy:** Balanced use of headers, whitespace, and visual cues.
*   **Progressive Disclosure:** Collapsible sections and "More" menus to hide secondary info without losing utility.

## 3. Interactive Data Tables
*   **Sticky Elements:** Sticky headers for context; sticky first columns for wide data.
*   **Alignment:** Left-align identifiers; right-align numeric/currency values for magnitude comparison.
*   **Interactivity:** Persistent sort icons, shift+click multi-sort, global search + per-column filters.
*   **Scannability:** Subtle row striping (zebra striping) and row-hover states for visual tracking.
*   **Empty States:** Use `—` or `N/A` for missing data.

## 4. Navigation & Sticky UI
*   **TOC:** Persistent, scroll-synced Table of Contents.
*   **Breadcrumbs:** Persistent breadcrumb trail to maintain spatial context within deep technical corpora.
*   **Persistent Navigation:** Sticky site header for search and user tools.

## 5. Infobox/Metadata Design
*   **Patterns:** Left-right (Right-align keys, left-align values) for high-density tables; top-bottom for narrow views.
*   **Visual cues:** Bolding keys or light gray labeling to separate from values. Card-footer placement for object metadata.

## 6. Accessibility (WCAG)
*   **Contrast:** Minimum 4.5:1 ratio for body text; 3:1 for large text/UI elements.
*   **Screen Readers:** Use `scope="col"` and `scope="row"` for tables; explicit ARIA labels on interactive elements.

## 7. Cognitive Load Management
*   **Whitespace:** Generous padding around main blocks to separate content groups.
*   **Scanning:** Short paragraphs, clear headers, and list formats.

## 8. Search & Findability
*   **Features:** Integrated global search with autocomplete.
*   **Context:** Scoped filtering (by category/tag/type) for faster drill-down in technical corpora.

## 9. Citation & Auditability
*   **Patterns:** Inline footnotes that link to references; citation ribbons for "first-class" audit trails on data-heavy articles.
*   **Freshness:** JSON-LD schema for `dateModified` to display clear last-updated timestamps per section.

## 10. Branding & Design Tokens
*   **Hierarchy:** Global (Primitive), Alias (Semantic), Component-level tokens.
*   **Governance:** JSON/YAML source of truth with automated transformation pipelines (e.g., Style Dictionary).
