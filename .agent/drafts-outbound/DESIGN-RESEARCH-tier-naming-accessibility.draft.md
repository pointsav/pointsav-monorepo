---
schema: foundry-draft-v1
state: draft-pending-design-review
originating_cluster: project-gis
target_repo: vendor/pointsav-design-system
target_path: research/
target_filename: DESIGN-RESEARCH-tier-naming-accessibility.md
audience: internal-design
bcsc_class: internal
language_protocol: DESIGN-RESEARCH
authored: 2026-05-08
updated: 2026-05-16
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 6
research_suggested_count: 1
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 9 Phase 4 (tier-label rebrand) and Sprint 17 Phase 4
  (second rename to ICSC hierarchy + BentoBox dominant-badge redesign).
  Sprint 9: compound-noun accessibility issue → Prime/Strong/Core/Emerging.
  Sprint 17 (G1, 2026-05-16): aligned to ICSC → Regional/District/Local/Fringe.
  Phase 4 chip measurements taken from implementation (Sprint 17 Phase 4.1):
  dominant badge 42px, mobile inline chip 12px; "Regional" (8 chars) fits both.
  Implementation: build-clusters.py (tier_descriptor) + index.html (bento CSS).
research_inline: false
notes_for_editor: |
  Route to project-design for design-system integration.
  No open questions. One suggestion for next editor: extend the naming pattern
  to Beacon visual states if the PRODUCT_VISION.md radar-chart redesign lands.
---

# DESIGN RESEARCH: Tier-Label Accessibility Rename (Sprint 9 + Sprint 17)

**Surface:** Co-location BentoBox tier badge.
**Component:** `tier-badge` span in cluster inspector; `.bento-tier-badge` dominant badge (Sprint 17); mobile `.bento-tier-badge` inline chip.
**Status:** Sprint 9 rename shipped (`7e92013`); Sprint 17 ICSC rename + dominant-badge redesign shipped (`fe5148fd`, 2026-05-16).

---

## The Problem

Original tier labels combined a numeric code and a compound descriptor:

| Old label | Composition |
|---|---|
| T3 Full Complement | hyper + hardware + warehouse |
| T2 Retail Anchor | hyper + hardware |
| T2 Bulk + Scale | hyper + warehouse |
| T2 Home + Bulk Hub | hardware + warehouse |
| T1 Hypermarket Anchor | hyper only |
| T1 Hardware Node | hardware only |
| T1 Wholesale Node | warehouse only |
| T0 Commercial Node | none |

Two failure modes for neurodivergent and screen-reader audiences:

1. **Compound nouns with "+"**: "Home + Bulk Hub", "Bulk + Scale" — the "+" reads ambiguously as additive ("a hub combining home and bulk") versus alternative ("a hub for either home or bulk"). Reading-list confusion accumulates across the eight tier slots.
2. **Numeric prefix without semantic anchor**: T3 / T2 / T1 / T0 are unmemorable in isolation. A user navigating the BentoBox cannot recall which tier is "best" without the descriptor; the descriptor in turn requires parsing the compound noun.

## The Decision

Replace technical compound labels with single Plain-English nouns drawn from a four-step quality scale, augmented by a parenthetical specialty marker where one is needed.

**Sprint 9 labels (intermediate state, May 2026):**

| Old label | Composition | Sprint 9 label |
|---|---|---|
| T3 Full Complement | hyper + hardware + warehouse | **Prime** |
| T2 Retail Anchor | hyper + hardware | **Strong (Retail)** |
| T2 Bulk + Scale | hyper + warehouse | **Strong (Bulk)** |
| T2 Home + Bulk Hub | hardware + warehouse | **Strong (Hub)** |
| T1 Hypermarket Anchor | hyper only | **Core (Hyper)** |
| T1 Hardware Node | hardware only | **Core (Hardware)** |
| T1 Wholesale Node | warehouse only | **Core (Wholesale)** |
| T0 Commercial Node | none | **Emerging** |

**Sprint 17 labels (current, May 2026):**

Sprint 17 replaced the Sprint 9 labels with the ICSC retail property hierarchy. The composition detail that the Sprint 9 parentheticals carried moved to a separate composition chip below the badge. The badge itself became a dominant single noun.

| Tier | Name | Visual treatment |
|---|---|---|
| 1 | **Regional** | Large badge (42 px desktop); mobile inline chip (12 px) |
| 2 | **District** | Large badge (42 px desktop); mobile inline chip (12 px) |
| 3 | **Local** | Large badge (42 px desktop); mobile inline chip (12 px) |
| 4 | **Fringe** | Large badge (42 px desktop); mobile inline chip (12 px); light background, dark text |

Composition chip beneath the badge: "Hypermarket + Hardware + Warehouse", "Lifestyle + Hypermarket", etc. This separates the tier signal (Regional/District/Local/Fringe) from the composition detail, reducing the per-word cognitive load.

The ICSC hierarchy was chosen over the Sprint 9 nouns because "Regional" carries immediate spatial meaning without requiring the user to learn platform vocabulary. "Prime" has no spatial meaning; "Regional" does.

## ARIA Pattern

The dominant badge is paired with a screen-reader label that restates the technical interpretation:

```html
<span class="tier-badge bento-tier-badge" role="status"
      aria-label="${tc.name} tier cluster; ${count_3km} stores within 3 km"
      style="background:${tc.fill};color:${tc.text};">
  ${tc.name}
</span>
```

`role="status"` is preferred over `role="alert"`: tier information is informational, not interruptive. The aria-label states the tier name (Regional/District/Local/Fringe) and the geometric site count. When `tier_predicates_fired` is present (Phase 2 engine), the badge also carries a tooltip via `title` attribute: "Why Regional? ≥ Hypermarket + Warehouse, top 10% by population, regional hospital within 5 km".

## Chip Size Measurements

Measurements taken from implementation (`fe5148fd`, 2026-05-16):

| Context | CSS class | font-size | Weight | Label fits? |
|---|---|---|---|---|
| Desktop dominant badge | `.bento-tier-badge` | 42 px | 900 | "Regional" (8 chars) ✓ |
| Desktop composition chip | `.bento-composition-chip` | 11 px | 700 | "Hypermarket + Hardware + Warehouse" ✓ |
| Mobile inline chip (≤ 480 px) | `.bento-tier-badge` (media query) | 12 px | 800 | "Regional" (8 chars) ✓ |

"Regional" and "District" are both 8 characters. Both fit at 12 px with weight 800. Pre-flight estimate confirmed (pre-flight item 3, 2026-05-16). No layout breakage observed in Chrome dev-tools 480 px viewport simulation.

## Reusable Pattern

**When a categorical scale is rendered as a coloured badge:**

1. Choose a four-to-five word hierarchy of single nouns, each unambiguous in isolation.
2. Use parentheticals (not "+" or "&") for specialty differentiation within a tier.
3. Pair the visual label with an ARIA `aria-label` that restates the technical taxonomy and the most relevant geometric or quantitative anchor.
4. Validate against neurodivergent reading patterns: each label must be parseable as one chunk, not a compound.

## Research Trail

### Done
1. Cognitive-load audit on the original eight labels (2026-05-08); identified "+" ambiguity and numeric-prefix unmemorability as the two failure modes.
2. Plain-English noun candidates evaluated: Prime / Strong / Core / Emerging chosen for monotonic hierarchy (Sprint 9). Shipped `7e92013`.
3. ARIA-label pattern reviewed against WCAG 2.1 SC 1.3.1 (Info and Relationships) and SC 4.1.2 (Name, Role, Value). `role="status"` justified.
4. Sprint 9 implementation confirmed by operator visual review.
5. Sprint 17 rename: ICSC hierarchy (Regional/District/Local/Fringe) chosen for spatial-meaning independence. G1 operator decision 2026-05-16.
6. Chip-size measurements taken from `fe5148fd` implementation: 42px desktop badge, 12px mobile inline chip; "Regional" (8 chars) fits both. Pre-flight item 3 confirmed.

### Suggested for next editor
1. If the PRODUCT_VISION.md Beacon (radar-chart) redesign lands, extend the naming pattern to its visual states. Same constraints: spatially-meaningful single nouns; ARIA pairing.

## Implementation Reference

| File | Line | Change |
|---|---|---|
| `pointsav-monorepo/app-orchestration-gis/build-clusters.py` | 349–409 | Tier descriptor composition labels (4-class) |
| `pointsav-monorepo/app-orchestration-gis/www/index.html` | 46–55 | `.bento-tier-badge` + `.bento-composition-chip` CSS |
| `pointsav-monorepo/app-orchestration-gis/www/index.html` | 107–125 | Mobile Layout B `@media(max-width:480px)` |
| `pointsav-monorepo/app-orchestration-gis/www/index.html` | 357–368 | `TIER_COLORS` 4-slot + `effectiveTierKey()` |
| `pointsav-monorepo/app-orchestration-gis/www/index.html` | 1370–1375 | `showClusterDetail()` dominant badge cell |

## See Also

- DESIGN-RESEARCH-zoom-prefetch-pattern.draft.md (Sprint 9 zoom transition)
- DESIGN-RESEARCH-bento-merged-zones-disclosure.draft.md (Sprint 9 dedup transparency)
- DESIGN-RESEARCH-ring-retailer-click-ux.draft.md (Sprint 9 click model)
- topic-co-location-tier-nomenclature.draft.md (customer-facing TOPIC companion)
