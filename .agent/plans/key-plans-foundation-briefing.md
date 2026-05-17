# Key Plans Foundation — Operator Briefing

**Date:** 2026-05-17
**Author:** totebox@project-bim (claude-opus-4-7 deep read + claude-sonnet-4-6 synthesis)
**Full study:** `.agent/plans/key-plans-foundation-study.md` (711 lines, complete inventory + gap analysis)

This briefing is the executive digest of the deep-read study. It surfaces what the
deep read found and what operator decisions are needed before further work on Key Plans.

---

## TL;DR

A focused Opus agent read the three operator-chosen V12 PDFs top-to-bottom:

1. `--- March 03, 2025 -- Collaborators #32 --- /AEC_Floor Plates_Key Plans_Methodology_V12.pdf` (1 page, ~47 KB)
2. `--- April 01, 2025 -- Collaborators #11 ---/AEC_Floor Plates_Key Plans and Tiles_V12.pdf` (7 pages, ~72 KB)
3. `--- May 06, 2025 -- Collaborators #27 ---/AEC_Floor Plates_Key Plans_Index.pdf` (1 page, 72 rows, ~49 KB)

The deep read shows the cluster's current state diverges from these three documents
in significant, structural ways. The corridor edit we made today (Private Office +
Academic = 3.0 m) is HTML-only and not propagated to the DTCG JSON, which is the
fifth instance of the same drift pattern. We should pause further HTML/token edits
until the operator has decided which source of truth is canonical.

---

## The five findings

### 1. The three PDFs do NOT constitute a single coherent specification

Each PDF takes a different view of the same system, with three different naming
conventions for the same key plans:

| PDF | What it contains | Naming |
|---|---|---|
| **Methodology** (March) | Visual definition via a single drawn example (PO-1 Small Private Office); only Z1 Habitat = 6 m is numeric; Z2 and Z3 are annotated as "? metres" | Codes: PO-1 |
| **Tiles** (April) | Size codes tied to specific SF cells; building totals; tile composition recipes | Codes: PO-1/2/3, M-1/2/3, B-1/2/3 |
| **Index** (May) | Master inventory of 72 key plans with use-type assignments | Names: "Private Office Small/Medium/Large", "Chiropractor/Dentist/GP", "B-1/B-2/B-3" |

The Index PDF uses a different naming convention from the Tiles PDF for the same
underlying entities. The Methodology PDF anchors only one of them with a numeric
zone depth.

### 2. The token store's zone depths come from documents OUTSIDE the three V12 PDFs

The Medical 7.2 m, Business 7.3 m, Civic 3.6 m, Laboratory 6.78 m values currently
in the cluster's token store and HTML are sourced from documents NOT in the
operator's chosen three-document read list. Specifically, the DTCG `$description`
fields cite:

- `CONSTRUCTION_2026_01_06_Key Plan_Professional Office_FFE_FIN.xlsx`
- `building-width-calculator.docx`

A reader opening only the three V12 PDFs has no way to reach the metre values
currently in use. This means **the three PDFs are not actually the authoritative
substrate for zone depths** — they are the methodology / inventory / tile-composition
layer; the dimension data lives elsewhere.

### 3. HTML `BIM_TOKENS` is NOT mirroring the DTCG JSON

The HTML `building-width-calculator.html` at lines 884–905 declares itself as
"inline mirror of the DTCG token files." It isn't. For 4 of 7 use types the HTML
values diverge from the JSON:

| Use type | HTML vs DTCG mismatch |
|---|---|
| Private Office | Z3 |
| Professional Office | Z2 and Z3 |
| Academic | Z3 |
| Business | Z1, Z2, Z3 |

The corridor edit we made today (PO + Academic = 3.0 m) is HTML-only — not
propagated to `professional-office-subtypes.dtcg.json` or
`building-width-calculator.dtcg.json`. It is the fifth instance of the same
drift pattern.

### 4. ~30 documented Key Plans have no token representation

The Tiles PDF and the Index PDF enumerate key plans that do not exist in the
cluster's token store at all. Entire categories are missing:

- **Professional Centre common areas (12 entries):** Tenant Lounge N, Building
  Manager O, Mail Room P, R/S/T corridors, U/V/W utilities, X/Y/Z/AA/BB main
  floor service, CC/DD coffee / restrooms, EE Lobby Atrium
- **Retail Select (6 tiles):** A/B/C/D/E/M (4,500–7,700 SF building totals)
- **Tech Industrial (5 tiles):** A/B/C/D/M (7,200–8,400 SF building totals)
- Plus specific discrepancies: **Chiropractor missing** from Medical
  specialisations; **Corporate Office 1/3 Floor** missing from tile fractions

### 5. The Tiles PDF has internal inconsistencies before it can be source-of-truth

Even within the Tiles PDF, the data is not self-consistent:

- **Tile letter "A" is reused** across Corporate Office (2,500 SF), Retail
  Select (1,400 SF), and Tech Industrial (2,200 SF) — same code, three meanings
- **Corridor Expander "T"** is 100 SF in the legend but 300 SF in the diagram
- **Sample tile arithmetic doesn't close** — rows sum to 2,150 SF and 5,200 SF
  against headlines of 2,500 SF and 5,000 SF
- The p.3 footnote referencing Key Plans J/K/L/M uses vocabulary that appears
  nowhere else in any of the three documents

---

## What this means

The current HTML decks and token files are not derived from any single coherent
specification. They are a hybrid:

```
              Three V12 PDFs (this read list)
                       │
                       │  (methodology + tile composition + 72-row inventory)
                       │
              ─────────┴─────────
              │                   │
              ▼                   ▼
   Token store (DTCG)         HTML BIM_TOKENS
              │                   │
              │  zone depths from │
              │  xlsx + docx      │
              │  (cited in        │
              │   $description)   │
              ▼                   ▼
                   DIVERGED (4 of 7 use types)
```

The HTML and DTCG have drifted apart. The DTCG's authoritative source for zone
depths is not in the operator's chosen three-document read list. Neither the HTML
nor the DTCG fully implements the 72-row inventory.

Continuing to edit HTML and token files in isolation will compound the drift.

---

## Operator decisions needed

Before any further Key Plans work (HTML edits, token files, new TOPIC drafts,
Rust scaffold), the operator should answer these four questions:

### Decision 1 — Canonical naming convention

Three options exist across the documents:

- **A.** Codes (PO-1/PO-2/PO-3, M-1/M-2/M-3, B-1/B-2/B-3) — Tiles PDF style
- **B.** Sizes (Small / Medium / Large) — Index PDF style for Private Office
- **C.** Specialisations (Chiropractor / Dentist / GP / Pediatrician) — Index PDF style for Medical

Recommendation in the full study: **adopt one canonical convention and translate
the others**. Mixed conventions in one system create endless ambiguity.

### Decision 2 — HTML `BIM_TOKENS` block: mirror or delete

Two options:

- **A.** Delete the inline `BIM_TOKENS` block from the HTML; have the page fetch
  the DTCG JSON files at render time (build-pdf.mjs would inline the fetched
  values into the printed PDF)
- **B.** Keep the inline mirror; treat HTML drift from DTCG as a bug that must
  be caught by CI / pre-commit lint (the HTML must be hand-synced to JSON on
  every JSON change)

The current "claim to mirror but actually drift" state is not viable. Pick one.

### Decision 3 — Scope of v0.0.x

The Tiles PDF and Index PDF describe a complete commercial property fleet:
- Professional Centre (offices + common areas + amenities)
- Suburban Office (variant)
- Retail Select
- Tech Industrial
- Parking
- Landscaping

The cluster's current state covers the Professional Centre tier only. The
question is whether Retail Select + Tech Industrial + the 12 common-area key
plans are in scope for the current iteration, or are post-MVP.

### Decision 4 — Tiles PDF internal inconsistencies

The Tiles PDF cannot be source-of-truth until its internal contradictions are
resolved. The operator must decide:

- Should the "Tile A" code be disambiguated (e.g., Tile A-CO, Tile A-RS, Tile A-TI)?
- Is Corridor Expander T = 100 SF (legend) or 300 SF (diagram)?
- Are the sample-tile arithmetic gaps (2,150 vs 2,500; 5,200 vs 5,000) typos in
  the PDF or do they represent rounding / unaccounted SF the system should expose?
- What is the J/K/L/M footnote referring to?

These are not problems an agent can resolve without operator input — they require
either a corrected V13 PDF or operator interpretation of intent.

---

## What I recommend NOT doing until the four decisions are made

- **Do not edit HTML `BIM_TOKENS`** further — it deepens the HTML-vs-DTCG drift
- **Do not edit DTCG token files** for new key plans — naming convention must be
  settled first
- **Do not scaffold the Rust crates** (`tool-buildingwidth`, `tool-floorplates`)
  yet — they would encode the current ambiguities into compiled code
- **Do not regenerate the TOPIC drafts** from this morning's Opus army — they're
  fine as living-document scaffolds; they will accept clarification as new
  sections rather than rewrites

## What is safe to do while the decisions are pending

- Continue producing TOPIC drafts and DESIGN-RESEARCH artifacts (they accept
  iteration)
- Continue HTML print/PDF and visual polish work (cosmetic, doesn't touch data)
- Continue source-document research (more V12 docs, DISCOVERY notes)
- Continue operator-facing documentation (project.woodfinegroup.com content)

---

## Reference

- **Full deep-read study:** `.agent/plans/key-plans-foundation-study.md` (711 lines)
- **Agent reports (3 decks):** `.agent/plans/agent-{1,2,3}-*-report.md`
- **Token store:** `woodfine-bim-library/tokens/bim/*.dtcg.json`
- **HTML decks:** `preview/{building-width-calculator,floor-plate-methodology,key-plans-index}.html`
- **Source PDFs (read in this study):**
  - `~/sandbox/inputs/project-bim/--- March 03, 2025 -- Collaborators #32 --- /AEC_Floor Plates_Key Plans_Methodology_V12.pdf`
  - `~/sandbox/inputs/project-bim/--- April 01, 2025 -- Collaborators #11 ---/AEC_Floor Plates_Key Plans and Tiles_V12.pdf`
  - `~/sandbox/inputs/project-bim/--- May 06, 2025 -- Collaborators #27 ---/AEC_Floor Plates_Key Plans_Index.pdf`
