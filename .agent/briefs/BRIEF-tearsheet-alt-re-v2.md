---
artifact: brief
name: BRIEF-tearsheet-alt-re-v2
status: active
created: 2026-06-09
updated: 2026-06-09
owner: totebox@project-proforma
---

# BRIEF — Alternative Real Estate tear sheet (V2)

## Mission

Engine-built, two-sided **structural** tear sheet comparing the two ways the same $250M of investor
equity can be deployed: **D7 Legacy JV** (traditional single-shot bank-financed joint venture) vs **D2
Direct-Hold Solutions** (phased debenture-funded structure that retains developer profit as equity).
It is the **companion (internally "V2")** to `Woodfine Direct-Hold Solutions — Sensitivity Analysis_JW3.html`
— same layout/voice/palette, but a comparison rather than a sensitivity study. One chart = one message,
told across six sections, ending with an illustrative Years 11–20 continuation.

Not to be confused with the *sensitivity analysis* (a different deliverable). This is the *comparison*.

## Current state — COMMITTED (2026-06-19)

Complete and verified. Committed in monorepo commit `f5cc00cc feat(tool-proforma-engine): add JW1 allocation proforma + V8/DHS/AltRE modules`. Stage 6 pending Command Session.

- **Generator:** `pointsav-monorepo/tool-proforma-engine/src/report/tearsheet_alt_re_v2.rs` (~1029 lines).
- **Registered in:** `src/report/mod.rs` (`pub mod tearsheet_alt_re_v2;`).
- **CLI:** `src/main.rs` — `Command::TearsheetAltReV2 { out_dir }` (clap kebab-cases to `tearsheet-alt-re-v2`).
- **Deliverables:** `outputs/mcorp-tearsheet-alternative-re-v2.html` + `outputs/mcorp-tearsheet-alternative-re-v2.json`.
- **Quality:** 12 unit tests pass; clippy clean.
- **Uncommitted** — `git status` in the monorepo clone shows exactly these three (stage ONLY these):
  - `M tool-proforma-engine/src/main.rs`
  - `M tool-proforma-engine/src/report/mod.rs`
  - `?? tool-proforma-engine/src/report/tearsheet_alt_re_v2.rs`
  (Other untracked files in that clone — `direct_hold_*`, `pclp1_sensitivity_v8`, a `pclp1_proforma.rs`
  modification — are pre-existing and NOT part of this work.)

## Regenerate / test

```bash
cd /srv/foundry/clones/project-proforma/pointsav-monorepo/tool-proforma-engine
cargo test tearsheet                      # 12 tests
cargo clippy --lib
cargo run --bin tool-proforma-engine -- tearsheet-alt-re-v2 \
  --out-dir /srv/foundry/clones/project-proforma/outputs
```

Visual check (snap chromium can't read /srv — stage into ~/sandbox first):
```bash
cp outputs/mcorp-tearsheet-alternative-re-v2.html /home/jennifer/sandbox/x/v2.html
chromium-browser --headless --no-sandbox --disable-gpu --window-size=1280,4600 \
  --virtual-time-budget=12000 --screenshot=/home/jennifer/sandbox/x/full.png file:///home/jennifer/sandbox/x/v2.html
chromium-browser --headless --no-sandbox --disable-gpu --no-pdf-header-footer \
  --virtual-time-budget=12000 --print-to-pdf=/home/jennifer/sandbox/x/v2.pdf file:///home/jennifer/sandbox/x/v2.html
```

## Architecture

- **Six sections** (JW3 "Section N of 6" idiom), **10 single-message charts** (Chart.js 4.4.0 CDN),
  green = Legacy JV, blue = Direct-Hold, red = alerts only, bars for levels / lines for trajectories,
  US-Letter landscape print, left-gutter line numbers on the year table.
  1. Same $250M, two machines — Scale (grouped bar) + Leverage (2-line) + year table.
  2. Where Direct-Hold pulls ahead — Value/unit (line, $100 ref) + LTV (line, 65% ref).
  3. Where the Legacy JV pulls ahead — Cash to investors (grouped bar) + Coverage (line, 1.20× covenant).
  4. How it resolves — MOIC + IRR horizontal-bar scoreboard.
  5. The next decade (Years 11–20 illustration) — extended Scale + extended Coverage (Y11–20 shaded).
  6. Basis of preparation & FOFI.
- **Engine sources:** `spv::legacy_jv_proforma::forecast()` (D7, Y0–10) + `spv::pclp1_proforma::forecast()`
  (D2, Y0–10). MOIC/IRR via `spv::irr::xirr_annual` over replicated per-year investor cash-flow vectors.
- **Embedded data contract:** `const D = {...}` JSON injected at render time; the HTML does no financial
  math. Matching audit JSON emitted beside the HTML.

### Headline numbers (engine-truthful)
- Square footage: JV 2,298,150 vs DH 3,906,855 (**+41%** LP-denominator; the +1,608,705 sf delta).
- Leverage (debt ÷ $250M): JV flat **3.0×**; DH ramps to **~3.98×** then eases.
- **MOIC: JV 2.96× / DH 4.76×** (4.76× is the real computed value, NOT the legacy comparator's stale "~4.0×").
- **IRR: JV 13.1% / DH 17.5%** (pre-tax, IFRS-FV terminal, investor fraction 0.9 for DH).
- NAV/unit (IFRS FV): JV ~$204/share vs DH ~$392/unit (both indexed to $100).
- LTV (IFRS FV, both): JV flat **~59.5%** (750M ÷ 1,260M); DH peaks ~62% → declines ~47% via buyback.

## Years 11–20 extension (Section 5)

- `fn extend_y11_y20(y10: &Pclp1Year) -> Vec<ExtYear>` lives **in the tear-sheet module**. It is a forward
  projection seeded from the engine Y10 row — it does **NOT** modify or re-run `pclp1_proforma::forecast()`
  (which is hard-coded `for y in 1..=10` and consumed by many other reports).
- **Operator-selected profile: debenture-recycling / compounding.** Each year recycles a share of FFO as
  development equity and issues matched debentures at a target loan-to-cost; 1-yr lease-up; fair value on
  capitalised generating NOI + WIP at cost.
- **Tunable constants** (top of the extension block): `EXT_RECYCLE_FFO_PCT = 0.50`,
  `EXT_NEW_DEV_LTC = 0.60`, `EXT_COST_PER_SF ≈ $310`, `EXT_LAST_YEAR = 20`.
- **Result:** SF 3.91M → **8.64M** (+121%); interest coverage rises **2.57× → 2.96×** (min 2.57×, far above
  the 1.20× covenant); NAV/unit → ~$901; debentures $972M → **$1.85B**. Fully BCSC-hedged as an
  illustration, not a commitment. To soften the pace, lower `EXT_RECYCLE_FFO_PCT`.

## Conventions locked (do not regress)

- **Version stays V2** — deliberately matches the sensitivity-analysis companion. Do not bump unless told.
- **Both structures on IFRS fair value** — there is no ASPE-vs-IFRS asymmetry caveat anymore; LTV/NAV/MOIC/
  IRR are directly comparable. (Do not reintroduce the rising ASPE `ltv_book` line.)
- **Naming:** displayed entity is "D2 Direct-Hold Solutions" (never "Professional Centres Canada LP").
  Displayed code-ref is `pro-01_proforma`; the real Rust module is still `pclp1_proforma` and is kept as
  the audit JSON `source` for provenance. "Direct-Hold Solutions" on first/formal reference per section,
  "Direct-Hold" shorthand and in legends/cards/table. **Never call it a "fund."**
- Reader-facing text must NOT contain: "No Excel source", "Professional Centres Canada LP",
  "Traditional / JV", "Woodfine LP", "Direct-Hold (LP)", "accounting-basis", "ASPE". (A test enforces this.)

## Open items / next steps

1. **Commit** — `~/Foundry/bin/commit-as-next.sh "<msg>"` staging ONLY the three files above (CODE-*,
   self-contained). Then Stage 6 promote from the **Command session** (`bin/promote.sh`), not this Totebox.
2. Optional — tune the Y11–20 pace (`EXT_RECYCLE_FFO_PCT`) if 8.6M/+121% reads too aggressive.
3. Optional — emit a PDF deliverable into `outputs/` (currently HTML+JSON only). Reference print-to-PDF
   was generated at `/home/jennifer/sandbox/v2b/v2.pdf` (7 pages, landscape, clean breaks).

## Related

- Companion: `outputs/Woodfine Direct-Hold Solutions — Sensitivity Analysis_JW3.html` (internally V2;
  engine module `report/pclp1_sensitivity_v8` / `report/direct_hold_sensitivity`).
- Memory: `project_tearsheet_alt_re_v6` (operator-facing digest of this work).
- Superseded single-chart v6 draft module backed up at `/home/jennifer/sandbox/v6-superseded/`.
- Reference inputs: `inputs/CORPORATE_MCorp_Tear Sheet_Alternative Real Estate.pdf` (the original chart the
  early drafts reproduced; the metric later moved from "Debt to Contributions" to engine-truthful
  Debt-to-Equity, and then to this multi-chart redesign).
