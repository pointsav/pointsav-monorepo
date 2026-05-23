---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-gis
target_repo: woodfine/content-wiki-projects
target_path: ./
target_filename: topic-co-location-ranking-system.md
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-02T04:00:00Z
authored_by: task-project-gis
authored_with: claude-sonnet-4-6
references:
  - ~/Foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis/build-clusters.py
  - ~/Foundry/deployments/gateway-orchestration-gis-1/app-orchestration-gis/config.py
notes_for_editor: |
  This TOPIC describes the ranking algorithm. Michelin Star framing is intentional
  and should be retained — the operator explicitly requested this presentation
  style. Use ★ / ☆ Unicode stars consistently.

  The 12-rank matrix table is accurate as of 2026-05-02 (build-clusters.py).
  The "Tier 5 = anchor + warehouse + hardware" simplification was proposed by
  the operator as a future algorithm direction — it is NOT yet implemented.
  Keep the current-fact matrix accurate; do NOT present the simplified version
  as current.

  Calibration threshold (10% Rank-1) is current-fact. Current Rank-1 = 3.7%
  of total anchors — well below threshold, no tightening applied.

  Target length: 700–1000 words English.
  Generate .es.md overview per Doctrine §XII.
---

# Retail Co-location Ranking System

The Woodfine co-location ranking system evaluates retail sites using a
**named-anchor combination matrix** — a deterministic algorithm that scores
each hypermarket anchor location by the combination of secondary and tertiary
retail and civic categories present within defined catchment radii.

The output is a 12-rank index mapped to five quality tiers, visualised on the
platform using a warm-to-cool colour scale: deep amber (★★★★★ Tier 5, highest)
through pale blue (★ Tier 1, lowest).

---

## The Named-Anchor Model

Every co-location site in the index is anchored by a single hypermarket or
large-format general merchandise operator — Walmart, IKEA, Carrefour, and their
regional equivalents. The anchor is a necessary condition: no site without a
qualifying anchor appears in the index.

Secondary operators are classified into four categories:

| Category | Role | Examples |
|----------|------|---------|
| **Hardware** | Secondary-1 | Home Depot, Lowe's, Leroy Merlin, K-Citymarket |
| **Warehouse Club** | Secondary-2 | Costco, Sam's Club, Makro, Metro |
| **Healthcare** | Tertiary-A | Hospitals, medical centres |
| **Higher Education** | Tertiary-B | Universities, colleges |

A secondary operator qualifies as co-located when it falls within the selected
secondary radius (default: 3 km from the anchor). Tertiary operators are scored
within a 5 km radius.

---

## The 12-Rank Matrix

The combination of present secondary categories determines the rank. There are
twelve named combinations:

| Rank | Tier | Hardware | Warehouse | Healthcare | Higher Ed |
|------|------|:--------:|:---------:|:----------:|:---------:|
| 1  | ★★★★★ | ✓ | ✓ | ✓ | ✓ |
| 2  | ★★★★ | ✓ | ✓ |   | ✓ |
| 3  | ★★★★ | ✓ | ✓ | ✓ |   |
| 4  | ★★★  | ✓ | ✓ |   |   |
| 5  | ★★★  | ✓ |   |   | ✓ |
| 6  | ★★★  | ✓ |   | ✓ |   |
| 7  | ★★★  |   | ✓ | ✓ | ✓ |
| 8  | ★★   | ✓ |   |   |   |
| 9  | ★★   |   | ✓ |   | ✓ |
| 10 | ★★   |   | ✓ | ✓ |   |
| 11 | ★    |   |   | ✓ | ✓ |
| 12 | ★    |   | ✓ |   |   |

An anchor present with no qualifying secondary operators does not receive a rank
and is excluded from the co-location index.

---

## Quality Tiers

The twelve ranks are grouped into five tiers for map display and reporting:

### ★★★★★ Tier 5 — Full Co-location
*Rank 1 only. All four secondary categories present.*

The highest designation. The anchor operates within 3 km of both a hardware
superstore and a warehouse club, and within 5 km of both a healthcare facility
and a university or college. All four independent operators have converged on the
same trade area. As of 2 May 2026: **102 sites** in North America.

### ★★★★ Tier 4 — Strong Co-location
*Ranks 2–3. Three secondary categories present.*

The anchor is paired with both a hardware superstore and a warehouse club, plus
one of the two tertiary categories. The commercial co-location is complete;
one tertiary dimension is absent. As of 2 May 2026: **268 sites** (NA: 259, EU: 9).

### ★★★ Tier 3 — Partial Co-location
*Ranks 4–7. Two secondary categories, various combinations.*

The most common tier. Includes the commercially significant Rank 4 combination
(anchor + hardware + warehouse, no tertiaries) as well as single-secondary
combinations supported by tertiary presence. As of 2 May 2026: **1,571 sites**
(NA: 1,396, EU: 175).

### ★★ Tier 2 — Limited Co-location
*Ranks 8–10. One secondary category, various combinations.*

The anchor has one major co-located secondary. Site quality is area-dependent:
a Rank 8 (hardware only) site in a dense healthcare corridor may outperform
a Tier 3 site in a lower-density market. As of 2 May 2026: **356 sites**
(NA: 333, EU: 23).

### ★ Tier 1 — Anchor Only
*Ranks 11–12. Tertiary categories only, or warehouse alone.*

The hypermarket anchor is present but the primary co-location secondaries
(hardware, warehouse club) are absent. These sites represent the floor of the
index — anchor presence confirmed, co-location opportunity not yet captured.
As of 2 May 2026: **441 sites** (NA: 398, EU: 43).

---

## Secondary Radius and Calibration

The secondary radius (the catchment distance applied to hardware and warehouse
operators) is configurable: 1 km, 2 km, or 3 km. A wider radius increases the
site count and changes rank distributions. The platform default is 3 km.

A **calibration rule** governs automatic tightening: if Rank-1 sites exceed 10%
of total anchor locations, the secondary radius automatically tightens from 3 km
toward 1 km to maintain index scarcity. As of 2 May 2026, Rank-1 represents 3.7%
of anchors — well below the threshold. No tightening is applied.

---

## Region Configuration

Each of the eight markets has a dedicated region configuration specifying which
chain identifiers map to the anchor, hardware, and warehouse roles. This allows
the algorithm to apply consistent logic across markets where different operators
fill equivalent functions: Costco in North America, Makro in Spain and Poland,
Metro in Italy.

Airport data (29,020 records) is ingested and retained but not yet incorporated
into tertiary scoring. Integration is planned for a future index version.

---

*Algorithm current as of build-clusters.py run 2 May 2026. Site counts reflect
the 3 km secondary radius default.*
