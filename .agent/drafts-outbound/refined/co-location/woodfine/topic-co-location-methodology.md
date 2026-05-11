---
schema: foundry-doc-v1
title: "Retail Co-location Methodology"
slug: co-location-methodology
category: governance
type: topic
quality: complete
status: active
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
last_edited: 2026-05-02
editor: pointsav-engineering
paired_with: co-location-methodology.es.md
cites:
  - ni-51-102
  - osc-sn-51-721
---

# Retail Co-location Methodology

The Woodfine co-location methodology is a deterministic spatial analysis framework used to identify and rank commercially defensible real estate nodes. Analogous to the stringent criteria of the Michelin Guide, the methodology evaluates sites based on the objective convergence of independent, capital-intensive retail operators rather than subjective market sentiment.

## The Named-Anchor Model

The foundation of the methodology is the **Named-Anchor Model**. Large-format retailers apply rigorous, data-driven site selection criteria before committing capital to a market. When multiple independent operators converge on the same geographic node, that convergence signals a validated commercial corridor: a location where multiple parties have independently confirmed the trade area's strength.

The methodology categorizes these operators into three distinct tiers based on their commercial function and foot-traffic contribution:

### 1. Primary Targets (The Anchor)
The foundational requirement for any evaluated node. The Primary Target represents the core traffic driver.
*   **North America:** Walmart Supercentre
*   **Europe:** IKEA (operational baseline)

### 2. Secondary Targets (Commercial Support)
Complementary large-format operators that validate the trade area's commercial depth. They are evaluated within a strict **1.0 km to 3.0 km** catchment radius of the Primary Target.
*   **Secondary-1 (Hardware):** Home Depot, Lowe's, Leroy Merlin.
*   **Secondary-2 (Warehouse Club):** Costco, Sam's Club, Makro.

### 3. Tertiary Targets (Institutional Support)
Civic and institutional infrastructure that provide a non-cyclical, stable demographic baseline. Evaluated within a **5.0 km** catchment radius.
*   **Tertiary-A (Healthcare):** Major hospitals and medical centers.
*   **Tertiary-B (Higher Education):** Universities and colleges.

## Quality Tiers and Site Validation

Sites are evaluated using a 12-rank matrix that maps to five quality tiers. This tiered approach allows for precise differentiation between "commodity" retail nodes and those rare locations where critical commercial elements converge.

| Tier | Description | Commercial Validation |
|------|-------------|-----------------------|
| ★★★★★ | Tier 5 — Full Co-location | Absolute commercial convergence: Anchor + Hardware + Warehouse + Both Tertiaries. |
| ★★★★ | Tier 4 — Strong Co-location | Complete commercial pairing with partial institutional support. |
| ★★★ | Tier 3 — Partial Co-location | The commercial baseline: pairing of the Anchor with both secondary categories. |
| ★★ | Tier 2 — Limited Co-location | Anchor presence with a single secondary validator. |
| ★ | Tier 1 — Anchor Only | Fundamental anchor presence; secondary co-location opportunity uncaptured. |

## Strategy and Application

The co-location index acts as a defensive filter for capital deployment. By focusing on Tier 4 and Tier 5 nodes, investors can prioritize sites with the highest levels of capital validation and the strongest multi-format demographic anchors.

The methodology is applied consistently across global markets by mapping regional operators to these canonical roles. Future expansion of the methodology includes the integration of logistics and transport data to add a fourth dimension to the matrix. [ni-51-102] [osc-sn-51-721]

---
## Provenance
- **Draft Source:** `TOPIC-co-location-methodology.md` (project-gis)
- **Refinement:** 2026-05-02 by project-language Task
- **Verification:** Methodology definitions confirmed against `app-orchestration-gis` documentation and current regional configurations.
- **BCSC Posture:** Forward-looking methodology expansions (transport data) labeled per ni-51-102.
