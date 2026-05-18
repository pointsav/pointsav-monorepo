---
schema: foundry-draft-v1
draft_type: DESIGN-COMPONENT
title: "Wiki home page — Phase D redesign (home_chrome v2)"
slug: design-home-chrome-v2
status: draft
created: 2026-05-17
author: project-editorial@claude-code
route_to: project-knowledge
destination_crate: pointsav-monorepo/app-mediakit-knowledge
requires_cosign: false
research_trail:
  method: "10-agent Opus synthesis (2026-05-17). Agents: main-page content strategy, award-winning docs analysis, leapfrog 2030 UX, DYK + FA mechanics, Wikipedia + library portals, multi-audience institutional design, IA for flat wikis, home_chrome() Rust audit, content inventory, cross-wiki family design."
  sources:
    - "Wikipedia Main Page + DYK mechanics (https://en.wikipedia.org/wiki/Wikipedia:Today%27s_featured_article)"
    - "Diátaxis documentation system (https://diataxis.fr/)"
    - "Stripe Docs, Vercel Docs, Linear Docs — main-page pattern comparison"
    - "NIST, SEC EDGAR, IETF RFC Index — institutional pattern comparison"
    - "NN/g card-sorting and navigation benchmarks"
    - "Baymard Institute navigation best practices 2025"
  prior_draft: ".agent/drafts-outbound/design-main-page-token-2.draft.md (2026-05-07, Master-cosigned) — reconcile: v2 supersedes the 10-slot model in that draft; structural invariants from it are carried forward"
  corpus_state: "Documentation wiki: 174 articles, 10 categories. Projects wiki: 24 flat topics. Corporate wiki: 5 flat topics."
---

# Wiki home page — Phase D redesign (home_chrome v2)

## Scope

Changes required in `app-mediakit-knowledge/src/server.rs` to the `home_chrome()` function, plus companion YAML data files and one new frontmatter field.

Constraint: this draft does not touch article rendering, category pages, or search. Phase D is home-page-only.

---

## 1. Critical fix — "From the doctrine" panel (D.4)

**Current state:** `home_chrome()` at `server.rs:1150–1184` renders a fully hardcoded panel with heading "From the doctrine" and four hardcoded bullets referencing ADR SYS-ADR-10, ADR SYS-ADR-07, Claim #39, and Active NOTAM.

**Problem:** The words "doctrine," "convention," and "NOTAM" are workspace-internal governance vocabulary explicitly banned from public wiki content per the 2026-05-09 cleanup-log decision. This panel is currently serving that banned vocabulary on all three public wikis.

**Fix:** Replace the hardcoded panel with a data-driven panel loaded from a per-wiki YAML file: `reference-invariants.yaml`. Each wiki's `reference-invariants.yaml` lives at that wiki's content root alongside `leapfrog-facts.yaml`.

**Schema** (same pattern as `LeapfrogFacts`):

```rust
#[derive(Deserialize)]
struct ReferenceInvariants {
    heading: String,                  // e.g. "From the engineering record"
    items: Vec<ReferenceInvariant>,
}

#[derive(Deserialize)]
struct ReferenceInvariant {
    label: Option<String>,            // optional bold prefix (e.g. "WORM ledger")
    text: String,                     // the invariant claim
    link_slug: Option<String>,        // optional [more] link
}
```

**Panel heading per wiki:**

| Wiki | Heading |
|---|---|
| documentation.pointsav.com | "From the engineering record" |
| projects.woodfinegroup.com | "Reference geometry" |
| corporate.woodfinegroup.com | "Holding structure" |

**Fallback:** if `reference-invariants.yaml` is absent, suppress the panel entirely (same behaviour as `leapfrog-facts.yaml` absence).

**YAML content per wiki** is specified in §5 below.

**Loader function** (add alongside `load_dyk` at server.rs:885):

```rust
async fn load_reference_invariants(content_dir: &Path) -> Option<ReferenceInvariants> {
    let path = content_dir.join("reference-invariants.yaml");
    let text = fs::read_to_string(path).await.ok()?;
    serde_yaml::from_str(&text).ok()
}
```

Pass the loaded value as a new parameter `ref_inv: Option<ReferenceInvariants>` to `home_chrome()`.

---

## 2. Sister surfaces — trim to 4 per wiki

**Current state:** hardcoded 10-tile grid at `server.rs:1271–1335` including links to Doctrine, NOTAM, OpenAPI, llms.txt — workspace-internal artefacts that should not be on the public home page.

**Fix:** Load from a per-wiki YAML file `sister-surfaces.yaml`. Or, simpler for v1: keep as hardcoded constants but trim the list and remove the governance links.

**Hardcoded constants approach** (lower implementation cost):

```rust
// Inside home_chrome(), branch on brand_theme:
let sister_links: &[(&str, &str)] = match brand_theme {
    "pointsav" => &[
        ("Projects Platform", "https://projects.woodfinegroup.com"),
        ("Corporate Reference", "https://corporate.woodfinegroup.com"),
        ("Design System", "https://github.com/pointsav/pointsav-design-system"),
        ("PointSav on GitHub", "https://github.com/pointsav"),
    ],
    "woodfine-projects" => &[
        ("Corporate Reference", "https://corporate.woodfinegroup.com"),
        ("Live Platform", "https://gis.woodfinegroup.com"),
        ("Engineering Documentation", "https://documentation.pointsav.com"),
        ("Newsroom", "/wiki/newsroom"),
    ],
    _ => &[ // woodfine (corporate)
        ("Projects Platform", "https://projects.woodfinegroup.com"),
        ("Engineering Documentation", "https://documentation.pointsav.com"),
        ("Newsroom", "/wiki/newsroom"),
    ],
};
```

**Drop entirely:** links to Doctrine, NOTAM, OpenAPI 3.1, llms.txt, Factory Release Engineering, Woodfine on GitHub (these are power-user links; they belong in `/about` or a footer nav link, not the home page grid).

---

## 3. Hero search — add to welcome banner

**Current state:** no search affordance on the home page. Tantivy search exists at `/search` but is invisible.

**Fix:** add a search `<input>` to the welcome banner section, posting to `/search`. No JS required for v1.

```rust
// In home_chrome(), inside the welcome banner div (#mp-topbanner):
div.wiki-home-search {
    form action="/search" method="get" {
        input type="search" name="q" 
              placeholder={ "Search " (stats.article_count) " articles" }
              aria-label="Search the wiki" {}
        button type="submit" { "Search" }
    }
}
```

**CSS:** `.wiki-home-search` — centered, max-width 520px, margin 1.5rem auto.

**Keyboard shortcut:** in `static/wiki.js`, add:
```js
document.addEventListener('keydown', e => {
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        document.querySelector('.wiki-home-search input[type="search"]')?.focus();
    }
});
```

**Skip if not on home page** (guard with `document.querySelector('.wiki-home-search')`).

---

## 4. Category grid — compact cards (remove article preview lists)

**Current state:** each category card shows up to 8 article previews (titles + descriptions). This creates a very long page and duplicates what category landing pages (`/category/<name>`) do better.

**Fix:** replace the 8-article preview list with a one-line category description sourced from a new optional frontmatter field `description` in each category's `_index.md`.

**New `_index.md` frontmatter field:**
```yaml
description: "Cross-cutting platform architecture, doctrine, and systems composition — the reasoning behind how PointSav components are assembled."
```

**Updated card rendering** (replace the inner `ul` loop):

```rust
// Per category card — compact version:
div.wiki-home-cat-section {
    h2 { a href={ "/category/" (cat) } { (humanize_category(cat)) } }
    p.wiki-home-cat-count { (count) " article" @if count != 1 { "s" } }
    @if let Some(desc) = category_description {
        p.wiki-home-cat-desc { (desc) }
    }
    // Remove the ul.wiki-home-cat-articles loop entirely
}
```

**Fallback:** if no `description` in `_index.md`, render only name + count. Compact with no preview is better than the current 8-article dump.

**The article previews move to:** `/category/<name>` landings, which already auto-list all articles below the `_index.md` MOC content.

---

## 5. Reference invariants YAML content per wiki

### documentation.pointsav.com — `reference-invariants.yaml`

```yaml
heading: "From the engineering record"
items:
  - label: "Compute boundary"
    text: "Every AI request enters and exits through a single audited gateway. No model call occurs outside this path — not by configuration, not by policy, but by structural firewall and bearer-only enforcement."
    link_slug: doorman-protocol
  - label: "Data ownership"
    text: "Operational interactions generate training signal that compounds across deployments. No tenant surrenders ownership of their data or model improvements."
    link_slug: compounding-substrate
  - label: "Audit ledger"
    text: "Every record appended to the WORM ledger is immutable. Retroactive rewrites are structurally impossible, not merely prohibited."
    link_slug: worm-ledger-design
```

### projects.woodfinegroup.com — `reference-invariants.yaml`

```yaml
heading: "Reference geometry"
items:
  - text: "102 Tier 5 sites in North America — 3.7% of anchor locations, below the 10% calibration threshold."
    link_slug: tier-index-north-america
  - text: "European Tier 5 count: 0. Data maturity gap, not market absence."
    link_slug: tier-index-europe
  - text: "7,594 co-location clusters validated from 229,054 retail locations across 13 countries."
    link_slug: co-location-intelligence-overview
```

### corporate.woodfinegroup.com — `reference-invariants.yaml`

```yaml
heading: "Holding structure"
items:
  - text: "A vacancy at one asset cannot propagate to the investor's equity in a different asset. Isolation is structural, not contractual."
    link_slug: direct-hold-framework
  - text: "No redemption queue exists. An investor seeking liquidity locates a willing counterparty independently of the corporate entity."
    link_slug: redemption-elimination
  - text: "At 1.2x interest coverage, operating income structurally exceeds debt service — the margin that prevents foreclosure cascade."
    link_slug: interest-coverage-ratio
```

---

## 6. Per-wiki zone differentiation (follow-on, not Phase D)

The IA synthesis recommends per-wiki zone sets (e.g. projects has no category grid; corporate has no hero search; projects gets a regional-groupings layout). These are higher-cost changes and should be Phase E.

The home_chrome() function signature already branches on `brand_theme`. Extending to per-wiki zone suppression is additive.

**Specifically deferred to Phase E:**
- projects: replace 10-category grid with 3-bucket regional groupings
- corporate: suppress hero search (corpus too small)
- corporate: suppress `/wanted` surface (institutional — no incompleteness signal)
- documentation: add `/wanted` zone (3-entry preview)

---

## 7. Bilingual home page (separate phase)

`index.es.md` exists in the documentation repo but `index()` never reads it. `<html lang="en">` is hardcoded. This is a significant scoped project:

- Route `/es/` → load `index.es.md` instead of `index.md`
- Route `/es/wiki/{slug}` → prefer `{slug}.es.md` if present
- Load `leapfrog-facts.es.yaml` for Spanish DYK
- Pass locale flag through `home_chrome()` to render `lang="es"`

Explicitly out of scope for Phase D. Documented here so the work is not lost.

---

## 8. Implementation order for Phase D

1. **`reference-invariants.yaml`** — create in all 3 wiki content repos. No code change. Immediate.
2. **Load + render `reference-invariants.yaml`** — add `load_reference_invariants()` + new parameter to `home_chrome()`. Low risk; replaces hardcoded block.
3. **Sister surfaces trim** — update hardcoded constants. Very low risk.
4. **Hero search** — add form to welcome banner + Cmd-K in wiki.js. Low risk.
5. **Compact category grid** — largest visual change; requires `description:` field in all 10 `_index.md` files + grid rendering update.

Steps 1–4 can ship in a single commit. Step 5 is a separate commit after the `_index.md` descriptions are written.

---

## 9. Files affected

| File | Change |
|---|---|
| `pointsav-monorepo/app-mediakit-knowledge/src/server.rs` | `home_chrome()`: new `ref_inv` parameter; load_reference_invariants(); sister surfaces trim; hero search form; category grid compact |
| `pointsav-monorepo/app-mediakit-knowledge/static/wiki.js` | Add Cmd-K handler |
| `content-wiki-documentation/reference-invariants.yaml` | New file — "From the engineering record" panel |
| `content-wiki-projects/reference-invariants.yaml` | New file — "Reference geometry" panel |
| `content-wiki-corporate/reference-invariants.yaml` | New file — "Holding structure" panel |
| `content-wiki-documentation/<category>/_index.md` (10 files) | Add `description:` frontmatter field to each |

---

## 10. What not to change in Phase D

- Article rendering — unchanged
- Category landing pages — unchanged (they remain the deep-browse surface)
- Search routing — unchanged (`/search` already works; Phase D only surfaces it)
- Feed routes — unchanged
- Footer — unchanged (Sprint C already shipped the institutional footer)
- Featured rotation and DYK — content updates are editorial work done separately via `featured-topic.yaml` and `leapfrog-facts.yaml`
