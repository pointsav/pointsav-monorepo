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

## Work log

- **2026-06-29** — Operator Q&A completed (11 questions). Decisions locked. BRIEF created.
  Phase 6 web research launched in parallel with BRIEF creation.

---

## Carry-forward

- Phase 6 simulation results go in `work/top600-proforma-coverage.json` — paste summary here
- Phase 1 ingest QIDs go in the Required ingests table above once confirmed
