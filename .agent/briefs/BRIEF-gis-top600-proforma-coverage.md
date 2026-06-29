---
artifact: brief
schema: foundry-brief-v1
brief-id: gis-top600-proforma-coverage
title: TOP600 — Proforma Coverage Redesign
status: active
owner: project-gis
created: 2026-06-29
updated: 2026-06-29
---

## Context

The Woodfine Buildings Portfolio Proforma V2 (Direct-Hold Solution) defines per-country
development site targets. The GIS platform's TOP400 Regional Markets list must supply 3x
those targets as a candidate pool for site selection. The current TOP400 fails in two ways:

1. **Algorithm bias** — no per-country normalization. US dominates NA (100% US).
   France + Germany dominate EU (89%). All other target markets are effectively absent.

2. **Data gaps** — Greece has 20 total co-location clusters; Italy has 9 T1s. Neither
   can produce 66 candidates (3× proforma) without additional chain ingests.

**Proforma source:**
`project-proforma/inputs/COMPLIANCE_MCorp_2026_06_04_Proforma_BuildingPortfolio_V2.pdf`

---

## Scope

- Rename TOP400 → TOP600 everywhere (scripts, data files, UI, research pages, JOURNAL)
- Redesign scoring in `score-regional-markets.py` to apply per-country normalization
- Ingest missing anchor chains for Greece and Italy
- Produce `work/top600-proforma-coverage.json` as the formal 3x proof document
- Update `research-colocation.html`, `research-regional-markets.html`, and retail
  co-location JOURNAL (`JOURNAL-retail-colocation-v0.2.draft.md`)

Out of scope: changes to taxonomy tiers (T1/T2/T3), ingest of any country other than GR/IT.

---

## Decisions locked

### Per-country proforma targets

From Proforma V2 (CA/US/ES/MX confirmed); Spain template applied to remaining markets:

| Country / Group | Proforma sites | 3x candidates | Current clusters | Gap |
|---|---|---|---|---|
| CA | 22 | 66 | 376 | — |
| US | 44 | 132 | 3,104 | — |
| MX | 22 | 66 | 284 | — |
| ES | 22 | 66 | 212 | — |
| PL | 22 | 66 | 158 | — |
| NORDICs (DK+SE+NO+FI) | 22 (group) | 66 | 158 | — |
| UK (GB) | 22 | 66 | 457 | — |
| IT | 22 | 66 | 134 | borderline |
| GR | 22 | 66 | 20 | DATA GAP |

NA actual need: 264 candidates. EU actual need: 396. Cap: 600 per continent for forward capacity.

### Scoring redesign — country normalization

Algorithm insertion point: after existing `score = quality × isolation × civic × confidence`
loop, before top-N slice. The raw formula is unchanged.

```python
NORDIC_ISOS = {'DK', 'SE', 'NO', 'FI'}
PROFORMA_TARGETS = {
    'CA': 22, 'US': 44, 'MX': 22,
    'ES': 22, 'PL': 22, 'NORDICS': 22,
    'GB': 22, 'IT': 22, 'GR': 22,
}

def norm_group(iso):
    return 'NORDICS' if iso in NORDIC_ISOS else iso

# After raw scoring:
# 1. Group RMs by norm_group(iso)
# 2. min-max normalize: norm = (raw - g_min) / (g_max - g_min)
#    edge case: g_max == g_min → norm = 1.0
# 3. Sort continent by norm_score DESC
# 4. Slice top 600
```

Nordics treated as one group: best site across DK/SE/NO/FI scores 1.0; 66 candidate
slots drawn from the best across all four countries.

### Cap and naming

- TOP_N changes from 400 → 600 in both scripts
- All output files: `top400-*` → `top600-*`
- UI label: "Top 600 Regional Market · Rank #N of 600"
- JOURNAL version: v0.1 → v0.2 (un-archive and update)

---

## Decisions open

- [ ] **GR proforma target** — reduce from 22 to 11 until 2025/2026 store openings hit OSM?
      Market research shows only ~5–8 viable T1 clusters in Greece today. Operator decision required.
- [ ] **Catchment population dimension** — methodology research (CBRE/JLL) strongly recommends
      adding 10-min drive-time population as a fourth scoring dimension alongside isolation.
      OpenRouteService isochrone API + WorldPop 100m grid. Phase 3 candidate.
- [ ] Whether to add `pop_10min` to scoring formula before running the full TOP600 build,
      or run TOP600 first and add population in a later iteration.

---

## Required ingests

### Greece (CRITICAL — only 20 clusters today)

Market size note: Greece has approximately 4 IKEA + 6 LM + 3 Decathlon stores nationally.
This yields ~5–8 viable T1 co-location clusters today, concentrated in Attica. Even
post-ingest, the maximum candidate pool may only reach ~15–24 — below the 66-candidate
3× target. New stores (IKEA Patras 2025, IKEA Heraklion, LM new locations) will expand
the pool but are not yet in OSM. **Operator flag: consider reducing GR proforma target
from 22 to 11 sites until 2025/2026 store openings are reflected in OSM data.**

| Chain | Category | OSM brand tag | Wikidata QID |
|---|---|---|---|
| AB Vassilopoulos | hypermarket | `AB` / `ΑΒ Βασιλόπουλος` | Q4721807 |
| Kotsovolos | electronics | `Kotsovolos` / `Κωτσόβολος` | Q17050427 |
| Public Stores | electronics | `Public` / `Public Stores` | Q12871976 |
| Plaisio Computers | electronics | `Plaisio` | Q7200794 |
| Leroy Merlin GR | hardware | `Leroy Merlin` | Q889624 (global) |
| Lidl GR large-format | hypermarket | `Lidl` | — (supplement OSM) |

Note: Kotsovolos and Public Stores are separate brands; ingest both.

### Italy (borderline — 134 total, 9 T1s)

Market size note: ~50 Leroy Merlin + 147 Decathlon stores exist in Italy. The current
9 T1 count is a data gap (LM not fully ingested), not a market gap. Expected outcome
post-ingest: 30–50 T1 clusters — well above the 22-site proforma target.

| Chain | Category | OSM brand tag | Wikidata QID |
|---|---|---|---|
| Leroy Merlin IT | hardware | `Leroy Merlin` | Q889624 (global) |
| Decathlon IT | sport | `Decathlon` | Q509349 (global) |
| OBI Italy | hardware | `OBI` | Q300518 (global; franchise — use name_query fallback) |

Pattern: `ingest-osm.py` with Wikidata QID + `name_query` fallback.

---

## Script references

| Script | Role | Phase |
|---|---|---|
| `app-orchestration-gis/score-regional-markets.py` | Scoring + normalization + rm-top600.json | Phase 2 |
| `app-orchestration-gis/generate-top400.py` → `generate-top600.py` | Editorial TOP_N | Phase 3 |
| `app-orchestration-gis/www/index.html` | UI rename | Phase 4 |
| `app-orchestration-gis/www/research-colocation.html` | Research page | Phase 5 |
| `app-orchestration-gis/www/research-regional-markets.html` | Research page | Phase 5 |
| `JOURNAL/archive/JOURNAL-retail-colocation-v0.1.es.draft.md` | JOURNAL (un-archive) | Phase 5 |

---

## Simulation results (2026-06-29)

Run: `score-regional-markets.py` with country normalization on current cluster data.

### NA TOP600 distribution
| Country | In TOP600 | Needed (3×) | Status |
|---|---|---|---|
| US | 515 (85.8%) | 132 | ✓ |
| CA | 56 (9.3%) | 66 | ✗ gap=10 |
| MX | 29 (4.8%) | 66 | ✗ gap=37 |

**TOP 5 NA (normalized):**
1. London, Ontario (CA) — nr. Kitchener 76 km — index 100
2. Lake Charles, LA (US) — nr. Baton Rouge 109 km — index 100
3. Boca del Río (MX) — nr. Puebla 90 km — index 100
4. Mesa, AZ (US) — nr. Phoenix 18 km — index 95
5. Fort Wayne, IN (US) — nr. Indianapolis 107 km — index 93

### EU TOP600 distribution
| Country | In TOP600 | Proforma target | Needed (3×) | Status |
|---|---|---|---|---|
| FR | 177 (29.5%) | — | — | non-proforma |
| DE | 175 (29.2%) | — | — | non-proforma |
| GB | 86 (14.3%) | 22 | 66 | ✓ |
| ES | 47 (7.8%) | 22 | 66 | ✗ gap=19 |
| IT | 43 (7.2%) | 22 | 66 | ✗ gap=23 |
| PL | 37 (6.2%) | 22 | 66 | ✗ gap=29 |
| NL | 21 (3.5%) | — | — | non-proforma |
| PT | 5 (0.8%) | — | — | non-proforma |
| DK | 4 (0.7%) | (Nordic group) | — | — |
| FI | 2 (0.3%) | (Nordic group) | — | — |
| SE | 1 (0.2%) | (Nordic group) | — | — |
| NO | 0 | (Nordic group) | — | — |
| GR | 0 | 22 | 66 | ✗ gap=66 DATA GAP |
| **NORDICS total** | **7** | 22 | 66 | ✗ gap=59 |

**TOP 5 EU (normalized — one per country at index 100):**
1. Esbjerg (DK) — nr. Odense 118 km — index 100
2. TauntonDeane (GB) — nr. Cardiff 110 km — index 100
3. Rosenheim (DE) — nr. Munich 80 km — index 100
4. Albi (FR) — nr. Toulouse 77 km — index 100
5. Jerez de la Frontera (ES) — nr. Cádiz 76 km — index 100

### Key findings
- Country normalization ensures the #1 site per country reaches index 100 and competes
  at the top of the global list. This solves the "best of Canada is buried" problem.
- However, by volume, DE+FR still take 58.7% of EU TOP600 (350/600). Proforma markets
  (ES/IT/PL/NORDICs/GR) get 220/600 combined — short of 396 needed for full 3× coverage.
- NORDICS has only 7 eligible suburban-regional markets total (across DK+SE+NO+FI).
  This is the tightest gap — structural, not a data gap. Norway has 0 eligible markets.
- GR is a data gap. CA gap (10) is likely addressable with more Canadian chain ingests.
- Algorithm note: normalization alone doesn't guarantee quota. To guarantee 66 slots
  per country requires stratified sampling (Phase N+1 decision, see Decisions open).

---

## Work log

- **2026-06-29** — Operator Q&A completed (11 questions). Decisions locked. BRIEF created.
  Phase 6 web research launched in parallel with BRIEF creation.
  Score simulation run. Results logged above. Research page (v0.4) + index.html updated.
  All code committed (commit 4de80f9c + second commit pending research page changes).

---

## Carry-forward

- Phase 1 GR/IT chain ingests — QIDs confirmed in BRIEF. Run after normalization is deployed.
- CA gap (10) — check how many CA markets are in the suburban-regional pool; may need
  more Canadian chain ingests (Canadian Tire large-format, Home Depot CA suburban)
- NORDICS structural gap — only 7 eligible markets; flag to operator for proforma adjustment
- GR proforma target reduction decision still open (see Decisions open)
