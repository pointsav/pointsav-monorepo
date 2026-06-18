---
artifact: brief
name: BRIEF-proforma-engine
status: archived
archived: 2026-05-23
superseded_by: BRIEF-tool-proforma-leapfrog-2030
created: 2026-05-22
updated: 2026-05-23
owner: totebox@project-proforma
---

# BRIEF — tool-proforma-engine

## Mission

Rust CLI that reads WCP 42M / PCLP 1 / TitleCo 3 Excel workbooks and renders
10-year financial proformas as Markdown, HTML, and JSON. Three base reports
(D1/D2/D3) plus three derived SPV reports (BenCal, Ambassadors Direct 1 & 2).

## Current state (as of 2026-05-23)

Binary: `/srv/foundry/cargo-target/jennifer/debug/tool-proforma-engine`
Outputs: `/srv/foundry/clones/project-proforma/outputs/`
Monorepo branch: `main` — 2 commits ahead of `origin/main` (Stage 6 pending)

### Subcommands

| Command | Format | Source Excel |
|---|---|---|
| `direct-hold <xlsx>` | D2 — PCLP 1 IS/CF/BS + Financial Forecast | PCLP 1 |
| `wcp <xlsx>` | D3 — WCP IS/BS + Revenue Generator + Valuation | WCP 42M |
| `dev-classes <xlsx>` | D1 — parameterised 10-year proformas per dev class | TitleCo 3 |
| `spv-bencal --pclp --wcp --out-dir` | 9 files: BenCal + AD1 + AD2 × (md/html/json) | WCP 42M + PCLP 1 |

Flags: `--html`, `--json`, `--out <file>`, `--assumptions <file>` (legacy)

### SPV derivation logic

- **Ambassadors Direct 1 Inc.** — 3,000,000 WCP shares (30% of 10M); sf = 0.30
- **Ambassadors Direct 2 LP** — 250,000 PCLP 1 units ($25M); sf = 250K/2,777,777
- **BenCal Holdings Inc.** — 10% of AD1 (3% WCP) + 25,000 PCLP units via 10% of AD2
  - Commission income: $100K/year Y1–Y3 from WCP share-sale activity

### SPV expenses (all SPV entities)

Source: `PUBLISHED_MCorp_2024_02_07_SPV Info Guide V3_Tab 04b_Budget_LCB3_JW5.xlsx`

| Row label | Y1 | Y2–Y10 |
|---|---|---|
| Legal Services | $8,875 (incl. $7,730 setup) | $1,145/yr |
| Accounting Services | $3,774 (incl. $1,375 setup) | $2,399/yr |

G&A NYC/Berlin replaced in AD1 + BenCal (d3_wcp format) with above.
Renderer uses `gna_label_1`/`gna_label_2` fields on `WcpData` — WCP keeps its
original labels; SPV entities override to "Legal Services" / "Accounting Services".

### Formatter

`fmt_smart` on G&A rows: auto-scales per cell — `$n` / `n.nK` / `n.nnM` by magnitude.
All other rows use `fmt_m` (millions) or `fmt_dollar`.

### JSON audit trail

SPV JSON outputs include `_derivation` block at top: source model, method,
scale factors, authority citation. Non-SPV JSON is raw struct serialisation.

## Pending

- **Stage 6** — push `main` branch (2 unpromoted commits) to origin + staging-j + staging-p
  Commits: `017a8f2d`, `05b0cce6`
- **Number audit (BRIEF-proforma-V2)** — verify engine-computed values against Excel tab-by-tab
- **AD2 expense review** — AD2 uses d2_direct_hold format (PCLP1 structure); current
  admin_compliance is scaled PCLP overhead, not standalone SPV costs. May need separate pass.
- **D1 entity name** — title uses TitleCo source entity; may need override to target name
