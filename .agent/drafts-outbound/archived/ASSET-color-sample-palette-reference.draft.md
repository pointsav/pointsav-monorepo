---
schema: foundry-draft-v1
state: destination-committed
originating_cluster: project-orgcharts
target_repo: pointsav-design-system
target_path: assets/reference/
target_filename: woodfine-org-chart-color-sample.html
audience: vendor-internal
bcsc_class: no-disclosure-implication
language_protocol: DESIGN-RESEARCH
authored: 2026-05-28T00:00:00Z
authored_by: totebox-project-orgcharts / claude-sonnet-4-6
authored_with: sonnet-4-6
research_done_count: 2
research_suggested_count: 1
open_questions_count: 0
research_provenance: sub-agent
research_inline: true
---

# ASSET — Woodfine Org Chart Color Sample (Palette Reference)

Visual reference file showing the complete Woodfine org-chart token
palette across three sections. Produced during `project-orgcharts`
session 2026-05-28 as a design decision aid.

Source file: `clones/project-orgcharts/current-org-chart-html/color-sample.html`

---

## Purpose

A standalone HTML file that renders every org-chart token class as a
labelled box swatch, organized into three sections:

1. **Core token classes** — 7 named tokens used across 3+ org charts,
   including both dashed and dotted ellipse variants of `token-blue`.
2. **Additional existing colors** — 6 tokens found in some charts but
   not universally applied: green, purple, purple-ellipse-dotted,
   olive, gray-dark, gray-light.
3. **Proposed new colors** — 6 net-new IBM Carbon tokens not yet used
   in any chart: crimson, magenta, cyan, slate, deep-violet, navy.

Each swatch shows: token class name, border hex, background hex, and
border style (solid / dashed / dotted ellipse).

---

## Companion token change draft

`DESIGN-TOKEN-CHANGE-ibm-carbon-org-chart-tokens.draft.md` in the
same drafts-outbound directory contains the four tokens actually
landed in org charts (magenta, teal, red, warm-gray). This ASSET
file covers the broader palette exploration including candidates not
yet adopted.

---

## Research trail

### Done

1. **Full palette audit via sub-agent** — All 19 org chart HTML files
   scanned for `.token-*` CSS class definitions. Every unique class
   name, border color, and background color extracted and catalogued.
   19 files audited; 15 distinct token classes found.
2. **Color system comparison** — IBM Carbon (12 hues) and Tailwind CSS
   (20 hues) evaluated as replacement palette candidates. IBM Carbon
   selected for the four new tokens due to better print performance
   and more muted professional character.

### Suggested

1. **Palette snapshot as DTCG reference artifact** — project-design
   may want to export this HTML color sample as a formal palette
   snapshot in the design-system `assets/` directory, linked from
   the Woodfine brand-kit entry.
