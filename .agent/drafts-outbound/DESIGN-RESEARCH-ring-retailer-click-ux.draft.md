---
schema: foundry-draft-v1
state: draft-pending-design-review
originating_cluster: project-gis
target_repo: pointsav/pointsav-design-system
target_path: dtcg-vault/research/
target_filename: DESIGN-RESEARCH-ring-retailer-click-ux.md
audience: internal-design
bcsc_class: internal
language_protocol: DESIGN-RESEARCH
authored: 2026-05-08
authored_by: project-gis Task Claude
authored_with: claude-sonnet-4-6
research_done_count: 5
research_suggested_count: 0
open_questions_count: 1
research_provenance: |
  Derived from GIS Session 8 UX design decisions (2026-05-08).
  User requirement: "If I click on a Retailer in a ring it should bring up that retailer
  at the bottom of the BentoBox, on the first click, rather than going to the Anchor first."
  And: "it should go to the Anchor of the ring when clicking on any area in the ring that
  is not the retailer dots themselves."
  Implementation: proximity-fill, nodes, all-locations click handlers in index.html.
research_inline: false
notes_for_editor: |
  Route to project-design for review.
  Open question: should the Selected Location section be collapsible?
  If the BentoBox is showing anchor + 6 co-tenants and the user also clicks a retailer,
  the panel gets long. Consider a collapsed/expanded state for the #sel-el section.
---

# DESIGN RESEARCH: Ring and Retailer Click Interaction Model

**Surface:** Co-location map — Retail Level (zoom ≥ 11)
**Component:** BentoBox inspector panel (right-side drawer)
**Status:** Implemented in Session 8; rationale documented here for design-system record.

---

## Interaction Model

The map at Retail Level shows two overlapping interactive surfaces:

1. **Proximity rings** — coloured fill polygons centred on each cluster anchor store
2. **Retailer dots** — point markers for every ingested location (all-locations layer)

These surfaces overlap: retailer dots appear inside rings belonging to that cluster and also inside rings belonging to neighbouring clusters.

### Decision: Click target determines BentoBox behaviour

**Click on ring fill (no dot underneath):**
→ Show the anchor cluster's full BentoBox. Display name, tier, NA rank, site count, co-tenant grid, and anchor name in the Selected Location section at the bottom.

**Click on a retailer dot inside a ring:**
→ Show the enclosing cluster's BentoBox immediately (same as ring fill), with the clicked retailer's name, category, and city pre-populated in the Selected Location section. Do not require a second click to switch from an anchor view to a retailer view.

**Click on a retailer dot outside any ring:**
→ Show the standalone retailer detail card. No cluster BentoBox — the retailer is not associated with a visible cluster zone.

### Rationale

The user's goal when clicking a retailer inside a ring is to understand that retailer's relationship to the co-location node — not to navigate away from the cluster context entirely. Replacing the BentoBox with a bare retailer card on first click discards the node context that motivated the click.

Requiring two clicks (first click → anchor, second click → retailer) was rejected as unnecessary friction. The cluster context is established by the ring geometry itself; the BentoBox should reflect that context from the first tap.

The Selected Location section at the bottom of the BentoBox (`#sel-el`) provides a dedicated slot for the most recently clicked entity within the ring. It is updated in place without re-rendering the full BentoBox, preserving scroll position and co-tenant visibility.

### Hit-test priority

When both a retailer dot and a ring polygon exist at the same screen coordinates (which is always the case for dots inside rings), MapLibre's layer event order determines which handler fires first. The `all-locations` click handler fires before `proximity-fill` because `all-locations` is drawn above `proximity-fill` in the layer stack.

To detect ring context inside the `all-locations` handler, the code queries rendered features at the click point against the `proximity-fill` layer. If a ring hit is found, the enclosing cluster id is extracted from that feature's properties and used to retrieve full cluster data from the in-memory index. This avoids a race condition where a retailer click inside a ring would need to re-fetch cluster data already held in memory.

### Fallback: Haversine distance

If the proximity-fill layer query returns no hit (e.g., tile not yet rendered, or click at the boundary of a ring), the handler falls back to a Haversine distance check against all clusters in the in-memory index (`findEnclosingCluster`). The fallback uses the cluster's 1 km radius as the containment test. This ensures ring association works even when the fill polygon tile has not yet loaded for the current viewport.

---

## Open Question

Should the Selected Location section (`#sel-el`) be collapsible?

When the BentoBox is at full height — showing anchor, 6 co-tenants, and a selected retailer — the panel extends beyond the viewport on smaller screens. A collapsed/expanded toggle for `#sel-el` would allow users to return focus to the co-tenant grid without scrolling. Not addressed in Session 8; flag for BentoBox accessibility sprint (Block 5).

---

## Implementation Reference

**Files:** `pointsav-monorepo/app-orchestration-gis/www/index.html`

| Handler | Trigger | Result |
|---|---|---|
| `proximity-fill` click | Ring fill, no retailer dot underneath | Full cluster BentoBox |
| `nodes` click | Cluster bubble (Co-location Level) | Drill into cluster → Retail Level |
| `all-locations` click | Retailer dot; inside ring | Cluster BentoBox + retailer in Selected Location |
| `all-locations` click | Retailer dot; outside all rings | Standalone retailer card |
| `individual-points` click | Secondary dot inside BentoBox ring view | Selected Location update only |
