---
schema: foundry-draft-v1
language_protocol: DESIGN-TOKEN-CHANGE
title: "MAIN PAGE Token 2 — Wikipedia-register home page with leapfrog 2030 extensions"
slug: design-main-page-token-2
state: master-cosigned
master_cosign: master@claude-code 2026-05-07T04:55Z
originating_cluster: project-editorial
target_repo: pointsav-design-system
target_path: tokens/main-page/
implementation_repo: pointsav-monorepo
implementation_path: app-mediakit-knowledge/src/server.rs
last_edited: 2026-05-05
editor: pointsav-engineering

research_sources:
  - wikipedia.org/wiki/Main_Page (live, 2026-05-05)
  - pointsav-monorepo/app-mediakit-knowledge/src/server.rs (home_chrome fn, read 2026-05-05)
  - content-wiki-documentation/applications/knowledge-wiki-home-page-design.md
  - content-wiki-documentation/index.md (live render, 2026-05-05)
  - content-wiki-corporate/index.md (live render, 2026-05-05)
  - content-wiki-projects/index.md (live render, 2026-05-05)
research_method: structural comparison — Wikipedia Main Page anatomy vs. home_chrome() output, per-tenant render inspection, server.rs code review
research_date: 2026-05-05
research_agent: claude-opus-4 sub-agent via project-editorial Task session
research_confidence: high (Wikipedia anatomy well-established; server.rs reviewed to line level; render gaps confirmed against live pages)
---

# MAIN PAGE Token 2 — design specification

## 1. Purpose

This draft specifies the second iteration of the generic main-page token for
`app-mediakit-knowledge`. Token 1 shipped as `home_chrome()` at
`app-mediakit-knowledge/src/server.rs` and implemented a working 9-slot home
page. Token 2 addresses structural gaps found in a comparative analysis against
Wikipedia's Main Page and adds five leapfrog-2030 extensions.

The token is generic — it parameterises over tenant (documentation, corporate,
projects) rather than hardcoding any single tenant's content.

---

## 2. Wikipedia anatomy baseline

Wikipedia's Main Page implements 10 structural slots:

| Slot | Wikipedia label | Notes |
|---|---|---|
| 1 | Site banner / logo / tagline | Language selector; "N articles in N languages" |
| 2 | Welcome lede | 1-2 sentences; encyclopedic register |
| 3 | Featured article | Framed panel; lead paragraph + "(Full article...)" closer |
| 4 | Did you know? | 5 rotating bullets; newly created articles |
| 5 | In the news | 4 news events + Portal:Current events link |
| 6 | On this day | Date-bound historical events; today's featured article |
| 7 | Featured picture | Captioned image with attribution |
| 8 | Article portals | Broad-topic gateway cards (Arts, Biography, Geography, …) |
| 9 | Other areas | Sister projects (Wiktionary, Wikimedia Commons, etc.) |
| 10 | Site footer | License notice; trademark line; language links |

Register observations (used to enforce Bloomberg-standard prose in Token 2):
- No H1 in lede — title is implied by the brand.
- All slot headings are sentence case, not title case.
- "From today's featured article" — not "Featured article of the day".
- "(Full article...)" closer — not "Read more →" or "Read the full analysis →".
- No marketing adjectives anywhere on the Main Page.
- Footer carries CC licence; it does NOT appear inline in the lede.

---

## 3. Token 1 current state (home_chrome())

### What is implemented

| Slot | Token 1 status | server.rs location |
|---|---|---|
| Site banner / h1 | Hardcoded: "PointSav Knowledge" for all tenants | ~line 807 |
| Stats banner | "N articles across 9 categories, last updated YYYY-MM-DD." — note trailing "." | ~line 821 |
| Lede panel | Reads index.md body; renders in `<div class="wiki-home-lede">` | ~line 825 |
| Category portal grid | Hardcoded 9 categories (RATIFIED_CATEGORIES, line 359) — all tenants | ~line 830 |
| Featured article | Reads featured-topic.yaml; renders framed panel | ~line 838 |
| Recent additions | Sorted by last_edited; top 5 | ~line 890 |
| Did you know? / knowledge portals | Rendered from index.md body when present | (body passthrough) |
| Other areas | Hardcoded GitHub links (pointsav, woodfine, design-system, factory-release-engineering) | ~line 944 |
| Contributing | Rendered from index.md body when present | (body passthrough) |
| Site footer | Hardcoded CC BY 4.0 + trademark div | ~line 1912 |

### Structural defects in Token 1

**P1 — Double-rendering (resolved by lede-only strip)**
The lede-only pattern (index.md body = lede paragraph only) was not established
when Token 1 shipped. Prior index.md files contained Platform areas, Featured
article, Recent additions sections — all also rendered by home_chrome() — producing
every slot twice. Resolved in project-editorial commit 2026-05-05 by stripping
all three index.md files to lede-only.

**P1 — "." after stats banner**
Stats banner ends with a period: "N articles, last updated YYYY-MM-DD." — the
period is inside the ENGINE-generated string, not from index.md. Minor; tracked.
Location: server.rs ~line 821.

**P2 — RATIFIED_CATEGORIES hardcoded for all tenants (server.rs line 359)**
The documentation wiki's 9-category list is used for ALL tenants.
Corporate and projects wikis are flat (no category subdirectories) but the engine
renders 9 empty category cards for them. Token 2 must replace the hardcoded
constant with a per-tenant configuration value.

**P2 — Breadcrumb root hardcoded as "Documentation" (server.rs ~line 1367)**
Non-documentation tenants display "Documentation" in the breadcrumb trail.
Token 2: read breadcrumb root label from per-tenant config.

**P2 — "Read more →" closer instead of "(Full article...)"**
Wikipedia uses "(Full article...)" as the featured article closer. Token 1 uses
"Read more →". Not wrong, but departs from Wikipedia muscle-memory. Tracked.
Location: server.rs ~line 833.

**P2 — "Other areas" hardcoded to documentation GitHub links (server.rs ~line 944-948)**
Corporate and projects wikis show PointSav GitHub links that have no relation to
their content scope. Token 2: per-tenant "other areas" link list.

**P3 — Bilingual notice when no .es.md files exist**
Corporate and projects wikis have no .es.md counterparts. If the engine emits a
bilingual toggle/notice, it should suppress when no .es.md exists. Verify at
render time.

**P3 — "." after "Home" in breadcrumb/page text**
A stray period was observed in a live page render after "Home" in the breadcrumb
or page header. Exact source not isolated — requires live DOM inspection. Token 2
audit should resolve.

---

## 4. Leapfrog 2030 extensions

Five extensions beyond Wikipedia's Main Page anatomy. These are the differentiating
claims of the leapfrog-2030 editorial strategy.

### Extension 1 — JSON-LD per slot (P3)

Emit `<script type="application/ld+json">` in the `<head>` for:
- Main page: `schema:WebSite` + `schema:ItemList` for the category grid
- Featured article slot: `schema:Article` with `name`, `url`, `dateModified`
- Recent additions slot: `schema:ItemList` of the 5 most-recent articles

This makes the main page machine-readable to crawlers, RAG systems, and the
planned `service-content` search index without bespoke adapters.

Target: `server.rs` `<head>` builder, new `home_json_ld()` function.

### Extension 2 — Editorial-cadence signal (P3)

Add a subtle but visible signal showing how recently the wiki was last updated.
Wikipedia equivalent: "This page was last edited on DD Month YYYY."

Implementation: surface `max(last_edited:)` across all articles in the
"Recent additions" heading or as a sub-line under the stats banner.
Register: plain factual sentence, not a badge or marketing claim.

### Extension 3 — Editor-as-onramp (P3)

Wikipedia's Main Page has no inline contribution flow, but the leapfrog-2030
vision positions the editor as a first-class entry point (contributor funnel).

Implementation: a minimal "Contribute" sentence in the Contributing section
(already in documentation wiki index.md body, inherited from home_chrome() for
other tenants) that includes a direct link to the browser editor at `/edit/`.
No marketing copy. Plain Wikipedia-style "To contribute, see [[contributing-as-engineer]]."

### Extension 4 — Research-trail teaser (planned)

In the Featured article panel, add a one-line "Research trail" footer under the
"(Full article...)" closer: "This article cites N sources — [view citation graph]."

Requires:
- Citation count derivable from `cites:` frontmatter field (existing)
- Citation graph page at `/citations/<slug>` (not yet implemented)

Gate: implement once `/citations/` route exists in app-mediakit-knowledge.

### Extension 5 — Citation-graph entry point (planned)

Add a "N citations across the encyclopedia" stat to the stats banner alongside
article count and last-updated date. Derivable at build time by summing `cites:`
entries across all articles.

Gate: `citations.yaml` resolution pass in the build pipeline (not yet wired).

---

## 5. Per-tenant configuration schema

Token 2 requires a per-tenant configuration struct. Proposed additions to
`AppConfig` in `app-mediakit-knowledge/src/server.rs`:

```rust
pub struct TenantConfig {
    pub tenant_id: &'static str,      // "documentation" | "corporate" | "projects"
    pub site_title: &'static str,     // "PointSav Knowledge" / "Woodfine Corporate" / …
    pub site_tagline: &'static str,   // one-sentence mission; appears in stats banner
    pub categories: &'static [&'static str],  // replaces RATIFIED_CATEGORIES
    pub breadcrumb_root: &'static str,         // replaces hardcoded "Documentation"
    pub other_areas: &'static [OtherAreaLink], // replaces hardcoded GitHub links
    pub contributing_slug: Option<&'static str>, // e.g. Some("contributing-as-engineer")
}

pub struct OtherAreaLink {
    pub label: &'static str,
    pub url: &'static str,
    pub description: &'static str,
}
```

Per-tenant instances:

| Field | documentation | corporate | projects |
|---|---|---|---|
| `site_title` | "PointSav Knowledge" | "Woodfine Corporate" | "Woodfine Capital Projects — Co-location Intelligence Platform" |
| `categories` | architecture, services, systems, applications, governance, infrastructure, company, reference, help | (flat — no grid) | (flat — no grid) |
| `breadcrumb_root` | "Documentation" | "Corporate" | "Projects" |
| `other_areas` | pointsav GitHub, woodfine GitHub, design-system, factory-release-engineering | glossary-corporate, csa-exempt-market-dealer, ni-31-103 | glossary-projects, gis.woodfinegroup.com, OpenStreetMap, Overture Maps |

For corporate and projects (flat wikis with no category directories), the category
grid should be suppressed entirely rather than rendering empty cards. Token 2:
render grid only when `categories` is non-empty.

---

## 6. Implementation priority

| Item | Priority | Owner |
|---|---|---|
| Strip index.md to lede-only pattern | P1 — DONE (2026-05-05) | project-editorial Task |
| Fix RATIFIED_CATEGORIES per-tenant | P2 | project-knowledge Task |
| Fix breadcrumb root per-tenant | P2 | project-knowledge Task |
| Per-tenant "other areas" links | P2 | project-knowledge Task |
| "(Full article...)" closer | P2 | project-knowledge Task |
| Remove trailing "." from stats banner | P2 | project-knowledge Task |
| JSON-LD per slot | P3 | project-knowledge Task (after P2 resolved) |
| Editorial-cadence signal | P3 | project-knowledge Task |
| Editor-as-onramp | P3 | project-knowledge Task |
| Research-trail teaser | planned | gate: /citations/ route |
| Citation-graph entry point | planned | gate: citations.yaml pipeline |

---

## 7. Design system token scope

The token for `pointsav-design-system` captures:

- **Layout spec**: slot order, grid dimensions, slot spacing, responsive breakpoints.
- **Slot anatomy**: heading text per slot (Wikipedia register), section dividers, panel borders.
- **Typography rules**: lede paragraph type scale, slot heading level (H2), portal card heading (H3).
- **Colour / chrome**: featured article frame colour, stats banner background, portal card hover state.
- **State variants**: empty category (0 articles card), no featured-topic.yaml (slot suppressed), single-article recent additions.

The implementation (Rust/HTML) lives in `app-mediakit-knowledge`; the design
token specifies the visual contract that implementation must satisfy. Root Claude
in `pointsav-design-system` commits the token as a COMPONENT or TOKEN artifact
per the design-system pipeline.

---

## 8. Handoff path

1. This draft stages in `project-editorial/.agent/drafts-outbound/` (Task scope done).
2. Master routes to design-system Root Claude for token extraction and commit.
3. design-system Root commits token to `pointsav-design-system/tokens/main-page/`.
4. project-knowledge Task implements P2 items in `app-mediakit-knowledge/src/server.rs`.
5. project-knowledge Task implements P3/leapfrog extensions in a subsequent pass.
