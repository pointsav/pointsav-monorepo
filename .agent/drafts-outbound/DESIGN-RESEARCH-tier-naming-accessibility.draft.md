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
authored_by: project-gis Task Claude
authored_with: claude-opus-4-7
research_done_count: 4
research_suggested_count: 1
open_questions_count: 0
research_provenance: |
  Derived from GIS Sprint 9 Phase 4 (tier-label rebrand). Operator constraint:
  "T3/T2/T1 labels need to be more clear and direct, understood by neurodivergent
  and accessibility users." Sprint 9 audit (2026-05-08) identified compound nouns
  ("Home + Bulk Hub") and ambiguous "+" symbols as the primary cognitive-load issue.
  Implementation: build-clusters.py:336–353 + index.html:863 (ARIA).
research_inline: false
notes_for_editor: |
  Route to project-design for design-system integration.
  No open questions. One suggestion for next editor: extend the naming pattern
  to Beacon visual states if the PRODUCT_VISION.md radar-chart redesign lands.
---

# DESIGN RESEARCH: Tier-Label Accessibility Rename

**Surface:** Co-location BentoBox tier badge.
**Component:** `tier-badge` span in cluster inspector.
**Status:** Shipped GIS Sprint 9 (`7e92013`); rationale documented here for design-system record.

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

| Tier | Composition | New label |
|---|---|---|
| T3 | hyper + hardware + warehouse | **Prime** |
| T2 | hyper + hardware | **Strong (Retail)** |
| T2 | hyper + warehouse | **Strong (Bulk)** |
| T2 | hardware + warehouse | **Strong (Hub)** |
| T1 | hyper only | **Core (Hyper)** |
| T1 | hardware only | **Core (Hardware)** |
| T1 | warehouse only | **Core (Wholesale)** |
| T0 | none | **Emerging** |

The four base words — Prime, Strong, Core, Emerging — convey hierarchy without numeric memory load. The parenthetical specialty marker preserves diagnostic detail (which anchor type dominates) without introducing the "+" ambiguity. Parentheses read as "kind of" rather than "and"; readers parse the full phrase as one unit, not a compound.

## ARIA Pattern

The visual badge is paired with a screen-reader label that restates the technical interpretation:

```html
<span class="tier-badge" role="status"
      aria-label="Tier ${rv} cluster: ${tierDesc}; ${count_3km} stores within 3 kilometres">
  ${tierDesc}
</span>
```

`role="status"` is preferred over `role="alert"`: tier information is informational, not interruptive. The aria-label includes the original tier number (Tier 0–3), the Plain-English label, and the geometric site count, giving non-sighted users the full diagnostic context that sighted users derive from colour + position + grid.

## Reusable Pattern

**When a categorical scale is rendered as a coloured badge:**

1. Choose a four-to-five word hierarchy of single nouns, each unambiguous in isolation.
2. Use parentheticals (not "+" or "&") for specialty differentiation within a tier.
3. Pair the visual label with an ARIA `aria-label` that restates the technical taxonomy and the most relevant geometric or quantitative anchor.
4. Validate against neurodivergent reading patterns: each label must be parseable as one chunk, not a compound.

## Research Trail

### Done
1. Cognitive-load audit on the original eight labels (2026-05-08); identified "+" ambiguity and numeric-prefix unmemorability as the two failure modes.
2. Plain-English noun candidates evaluated: Prime / Strong / Core / Emerging chosen for monotonic hierarchy and absence of cultural connotations.
3. ARIA-label pattern reviewed against WCAG 2.1 SC 1.3.1 (Info and Relationships) and SC 4.1.2 (Name, Role, Value). `role="status"` justified.
4. Implementation shipped to gis.woodfinegroup.com (Sprint 9 commit `7e92013`); operator visual confirmation received.

### Suggested for next editor
1. If the PRODUCT_VISION.md Beacon (radar-chart) redesign lands, extend the naming pattern to its five visual states. Same constraints: four-to-five Plain-English nouns; ARIA pairing.

## Implementation Reference

| File | Line | Change |
|---|---|---|
| `pointsav-monorepo/app-orchestration-gis/build-clusters.py` | 336–353 | Tier descriptor mapping |
| `pointsav-monorepo/app-orchestration-gis/www/index.html` | 863 | tier-badge with aria-label |

## See Also

- DESIGN-RESEARCH-zoom-prefetch-pattern.draft.md (Sprint 9 zoom transition)
- DESIGN-RESEARCH-bento-merged-zones-disclosure.draft.md (Sprint 9 dedup transparency)
- DESIGN-RESEARCH-ring-retailer-click-ux.draft.md (Sprint 9 click model)
- topic-co-location-tier-nomenclature.draft.md (customer-facing TOPIC companion)
