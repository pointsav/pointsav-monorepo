# NEXT.md — project-proforma

> Hot open items. Backlog at `.agent/next-backlog.md`.
> Completed items struck or removed. Attribution format: `[YYYY-MM-DD role@engine]`

---

## V2 Bencal proforma outputs (2026-06-05)

- [x] `feat(bencal-v2)` — `729b6083` — committed to monorepo `main` (author: pwoodfine).
  Annual returns tables (per investor share / per LP unit / per BM share), nominal
  founding-bonus prices, SPV1 Y5–Y10 opex-funding share-sale model, PDF auto-gen via
  WeasyPrint. BRIEF v0.15.9 unchanged — all changes are engine-level implementation of
  existing spec (§3h, §5d–§5f). `[2026-06-05 totebox@claude-sonnet-4-6]`
- Monorepo `main` now **7 commits ahead of `origin/main`** — Stage 6 needed (Command Session).
- Cluster-branch conflict still unresolved — see Stage 6 readiness checkpoint below.

---

## Stage 6 readiness checkpoint (2026-06-02)

- **Archive ready:** commits `4bbdcc1` (v0.15.7 flag resolutions + FSL licensing), `bcd1938`
  (v0.15.8 dual-asset Bencal SPV2 + PCLP 1 rename), and `e90f725` (v0.15.9 housekeeping sweep)
  staged for Command Session `bin/promote.sh project-proforma`. Archive working tree is clean
  (gitignored mailbox state aside).
- **Monorepo sub-clone — BLOCKED on cluster-branch conflict:** Auto-mode cherry-pick of
  `72d4a635` (d3-wcp formatter) + `6b2606bc` (spv-bencal G&A fix) aborted on:
  (a) modify/delete in `tool-proforma-engine/src/spv/ambassadors_d1.rs` (cluster branch
  deleted; commit modified); (b) same in `tool-proforma-engine/src/spv/bencal.rs`;
  (c) content conflict in `tool-proforma-engine/src/report/d3_wcp.rs`. Cluster branch architecture
  evolved past the file layout these commits assume — needs human decision on consolidation vs
  skip. Working tree restored to pre-sweep state. Originally referenced SHAs `017a8f2d` /
  `05b0cce6` from `command-20260528-project-proforma-stage6-status` don't exist anywhere in the
  monorepo (likely rebased away).
- **Secondary issue:** `origin-staging-p` SSH fetch fails (`Permission denied (publickey)`);
  pwoodfine SSH alias / GitHub key out of sync. Orthogonal workspace-admin task.
- **Project-documents informed:** outbox msg-id `project-proforma-20260602-bencal-spv2-dual-asset-rename`
  dispatched announcing dual-asset SPV2 + canonical rename + new Flag 15 + document-chain requirements.
- **Command notified:** outbox msg-id `project-proforma-20260602-stage6-ready` dispatched with
  full punch-list of the monorepo blocker + recommended actions.
- **Inbox clean:** all messages actioned; J1 journal relay archived (connection registered).
- **Outstanding Jennifer-side / Command-side items:** see monorepo blocker above + the
  pwoodfine SSH alias. Archive itself has no blockers for `promote.sh`.

## Deferred-items resolution log (2026-06-02)

All 5 items resolved in Q&A session 2026-06-02. Decisions captured; implementation tranches
sequenced (Tranche 1 documentation done; Tranches 2–4 staged).

- [x] **Item 4 — Flag 15 resolution: Path (b) capital contribution** `[2026-06-02 totebox@claude-opus-4-7]`.
  Bencal SPV2 records 600K WCP at FMV (~$2,730,000) with offsetting credit to contributed surplus
  (equity); no Y0 IS impact at Bencal SPV2. Strategic Partner recognises deemed disposal at FMV
  under ITA s.69 (tax effect at Strategic Partner level only). Tax counsel sign-off requested
  post-decision; D4 cluster-engine block lifted.
- [x] **Item 2 — Block F per-share + aggregate MOIC: side-by-side columns** `[2026-06-02 totebox@claude-opus-4-7]`.
  Engine adds `moic_aggregate` + `moic_per_share` fields to `BlockF` output type; renderer emits
  single table with both columns + header note on 10/90 manager/investor dilution mechanics;
  HTML/MD/PDF uniform.
- [x] **Item 3 — Bencal Management FOFI scope: REVERSED to Y1–Y3 only** `[2026-06-02 totebox@claude-opus-4-7]`.
  Overrides earlier Flag 9 decision (originally "publish full Y1-Y5"). CIM publishes Y1–Y3 only;
  Y4–Y5 NOT published. Engine BM forecast stays at Y1-Y3 — no struct extension needed. Saves
  engine work; reduces CIM speculative-language regulatory surface.
- [x] **Item 5 — Cluster engine D4 dual-asset: Phase A-D4 full scope** `[2026-06-02 totebox@claude-opus-4-7]`.
  Single cycle: `Ad2Config` schema additions; `Ad2Data` outputs (dual-asset NAV + Y0 capital-
  contribution JE per Flag 15 path (b)); Block A-F valuation-matrix WCP column; cargo test;
  regenerate `bencal.{md,html,json}`. Lands on cluster/project-proforma AFTER Command resolves
  the existing cluster-branch conflict (orthogonal).
- [x] **Item 1 — SPV operating budget v10: restore v9 + incremental v10** `[2026-06-02 totebox@claude-opus-4-7]`.
  Two-commit sequence — Commit 1 restores v9 source as tracked baseline (search order: git show
  0c9d15e → local backups → Google Drive / mac copy → engine regen). Commit 2 applies surgical v10
  edits: rename PCLP 1 → Professional Centres Canada LP; Note 3 $10 GP nominal; new dual-asset
  Bencal SPV2 note (Flag 15 path (b) capital-contribution language); MOIC table side-by-side;
  BM FOFI Y1-Y3 only. Numerical values UNCHANGED.

---

## Structural change in flight (2026-06-02; v0.15.8)

- [ ] **D4 Bencal SPV2 — dual-asset rebuild** (LP units + 600,000 WCP founding-bonus shares).
  Jennifer instruction 2026-06-02: Bencal SPV2 receives 600,000 WCP common shares for nominal
  consideration ($199.34 = $0.00033223/share) on completing its minimum CAD 13,000,000 investment
  in **Professional Centres Canada LP** (canonical legal name; formerly PCLP 1). Carve-out from
  Strategic Partner block (1,800,000 → 1,200,000 of 10,000,000 outstanding). WCP cap table updated
  in BRIEF §5c; dual-asset Bencal SPV2 spec in BRIEF §5d; Bencal Group total WCP exposure 9.0%.
  Engine D4 implementation blocked pending Flag 15 resolution. `[2026-06-02 totebox@claude-opus-4-7]`

- [ ] **Flag 15 (NEW; Bencal SPV2 §5d)** — IFRS 2 / IAS 12 / Section 69 treatment of Strategic
  Partner → Bencal SPV2 600K WCP transfer. Three defensible paths: (a) IFRS 2 share-based
  payment received for service (Y0 bonus-share income ~$2.73M at Bencal SPV2); (b) capital
  contribution from Strategic Partner via contributed surplus (no Y0 IS impact at Bencal SPV2);
  (c) bargain-purchase / cost-basis with subsequent FVTPL adjustment (Y0 FVTPL gain ~$2.73M).
  Tax counsel sign-off required before D4 Phase A implementation; affects Y0 IS, Y1 FVTPL,
  CIM risk-factor disclosure of unrealised gain. `[2026-06-02 totebox@claude-opus-4-7]`

- [ ] **PCLP 1 rename ripple** — canonical legal name locked as **Professional Centres Canada LP**
  (or "PC Canada LP" abbreviation). Internal struct names (`Pclp1Config`, `pclp1_units`, etc.)
  retained for code stability. Rename ripple needed in: SPV operating budget MD/HTML outputs
  (`outputs/spv-operating-budget.md` + `.html` v9+), all proforma deliverables, CIM/LP-Agreement
  drafting at project-documents (outbox follow-up), all per-investor materials. `[2026-06-02 totebox@claude-opus-4-7]`

---

## Immediate blockers (Jennifer decision required)

- [x] **Bencal SPV1 Flag 1** — Cash reserve: Option 1 (reserve at close) via OpexBudget §3h.
  Reserve ≈ $240,842 (Y1–Y3 only; income onset Y4 via WCP share sales); WCP held at 150,000 shares.
  Offering = $3M + reserve = ~$3,240,842; diluted ≈ 3,600,936. Phase A-D5 blocker cleared. `[2026-05-24 totebox@claude-sonnet-4-6]`
- [x] **Bencal SPV1 Flag 5** — 300K WCP shares (150K at $20 + 150K bonus at $0.00033223 = $49.83); total cost $3,000,049.83;
  scale=3.0%; Bencal SPV1 does NOT invest in PCLP1. All Bencal SPV1 flags resolved; D5 blocker cleared. `[2026-05-25 totebox@claude-sonnet-4-6]`
- [x] **Bencal Management Flag 1** — Investment entity election (IFRS 10.27) confirmed YES.
  Bencal SPV2-LP + Bencal SPV1 at FVTPL; no consolidation. Accounting-policy note at incorporation. `[2026-05-25 totebox@claude-sonnet-4-6]`
- [x] **Bencal Management Flag 5** — Cash funding gap resolved (mechanism updated 2026-05-25): commission income
  funds Y1–Y3 reserve. Share price = $5.00/share nominal ($10 total capital). After 2% work fee +
  26.5% tax, Bencal Management retains ≈ $188,095 → covers $161,092 opex + $27K buffer. `[2026-05-25 totebox@claude-sonnet-4-6]`
- [x] **Bencal Management Flag 6** — Work Fee ($562,280) recipient: Altas One Digital Securities Inc. keeps the Work Fee
  (cleanest NI 31-103 EMD path; registered dealer compensation; no related-party disclosure burden in CIM §D). `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Bencal Management Flag 13** — Reserve injection mechanism: Option C — direct commission rebates from Altas One
  to each Bencal entity (already implemented in v9 of SPV operating budget; manager diluted % stays exactly 10.0000% at SPV1 and SPV2). `[2026-06-02 totebox@claude-opus-4-7]`

## All Jennifer decision flags (from BRIEF §5d, §5e, §5f)

### Bencal SPV2 flags (§5d)
> ⚠ Bencal SPV2 restructured 2026-05-24: from $30M embedded-WC to $25M net PCLP1 + additive OpexBudget reserve.
> Manager units: 27,843 (v0.15.6; 2% Work Fee constraint). Diluted: 278,434.
> Bencal SPV2 OpexBudget formation costs TBD — confirm from formation invoices before Phase A-D4.
- [x] **Flag 1** — SPV MV yield resolved: 8.0% (two-layer framework; same rate as PCLP 1;
  discount falls out from fee drag). `[2026-05-24 totebox@claude-sonnet-4-6]`
- [x] **Flag 2** — Fully diluted (333,333) everywhere; no dual-column. Unit count corrected:
  33,332→33,333 (Bencal SPV2), 333,332→333,333 (Bencal SPV1), 3,333,332→3,333,333 (Bencal SPV1 diluted), 2,777,777→2,777,778 (PCLP1).
  Formula: `ROUND(investor_units / 9, 0)`; issuance = 1/9 = 11.111111̄%. `[2026-05-24 totebox@claude-sonnet-4-6]`
- [x] **Flag 3** — No-hurdle disclosure resolved: face-of-statement Note 1 with two components —
  (1) pari passu distributions, no performance hurdle; (2) escrow/transfer restriction until investors
  receive 100% of initial investment (mirrors PCLP 1 LPA Benetti clause). PCLP 1 Benetti numbers
  confirmed from EY FOFI: 277,777 / 2,777,777 (LPA-locked; do not derive via ROUND(n/9, 0)). `[2026-05-24 totebox@claude-sonnet-4-6]`

### Bencal SPV1 flags (§5e)
- [x] **Flag 1** — Cash reserve resolved: Option 1, reserve at close via OpexBudget. `[2026-05-24 totebox@claude-sonnet-4-6]`
- [x] **Flag 2** — FVTPL IS basis: Book Value = Level 3 proxy pre-listing; Level 1 at WCP exchange listing (Phase B). Large Y1 FVTPL loss expected (book $1,365K vs cost $3,000,050). `[2026-05-25 totebox@claude-sonnet-4-6]`
- [x] **Flag 3** — IFRS 2: NO charge at Bencal SPV1 level. Bencal SPV1 is buyer, not WCP formation participant. Manager dilution structural only. `[2026-05-25 totebox@claude-sonnet-4-6]`
- [x] **Flag 4** — FOFI truncation: Y1-Y5 OM; Capital Return Sale within window; Y6-Y10 internal only. `[2026-05-25 totebox@claude-sonnet-4-6]`
- [x] **Flag 5** — Bencal SPV1 holds 300,000 WCP shares: 150K purchased at $20 ($3M) + 150K founding bonus at $0.00033223 ($49.83). Total cost $3,000,049.83. scale_factor=0.030. Bencal SPV1 does NOT invest in PCLP1. `[2026-05-25 totebox@claude-sonnet-4-6]`

### Bencal Management flags (§5f)
- [x] **Flag 1** — Investment entity election IFRS 10.27: YES (cleared 2026-05-25). `[2026-05-25 totebox@claude-sonnet-4-6]`
- [x] **Flag 2** — Bencal SPV2-GP 1 share: **$10.00 nominal consideration** (overrides nil recommendation; small GP capital account tracked through LP Agreement distribution/dissolution allocations). `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 3** — MOIC optics: **show BOTH per-share and aggregate MOIC** in Block F renderer (overrides suppress recommendation; CIM needs supplementary text on manager/investor 10/90 dilution mechanics). `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 4** — DTL computation: **Option (a) Bencal Management-level DTL** (no look-through to SPV1; consistent with IFRS 10.27 investment-entity election; DTL on commission income + realised FVTPL gains at Bencal Management level). `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 5** — Cash funding gap resolved v0.15.6: Bencal Management reserve = $41,713 (from commission; no surplus; director fee $4,373.37/yr/director). `[2026-05-26 totebox@claude-sonnet-4-6]`
- [x] **Flag 6** — Work Fee ($562,280) recipient: **Altas One keeps the Work Fee** (registered EMD compensation; cleanest NI 31-103 path; no IAS 24.17 enhanced disclosure burden). `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 7** — Distributions from Bencal SPV2 treated as **income (cost basis = $0)** in Bencal Management IS (FVTPL classification; no return-of-capital treatment). `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 8** — Management Services Agreement (Bencal SPV2-GP): **MSA embedded in LP opex** ($16,664/yr = $1,145 legal + $2,399 accounting + $13,120 board); no separate management fee line. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 9** — FOFI scope: **REVERSED 2026-06-02 → publish Y1–Y3 ONLY** (overrides earlier same-day decision to publish Y1–Y5). Rationale: avoid speculative-language disclosure issues for Y4–Y5 (WCP listing/distribution timing dependent). Engine BM forecast stays at Y1–Y3; no struct extension needed; CIM publishes Y1–Y3 only. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 10** — Transfer restrictions / manager-change event: **majority-LP consent right** required for GP transfer or change of control of Bencal Management Corp.; parallels existing Benetti escrow clause in PCLP 1 LPA. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 11** — Engagement standard: **CSRE 2400 review engagement** (not full CAS audit); same firm reviews SPV1, SPV2, and Bencal Management; audit may be triggered later by listing/issuer obligations. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 12** — Reserve sizing: **3 years** (current sizing — SPV1 $54,832; SPV2 $59,097; BM $41,713; total $155,642); matches Y4 income onset from WCP share-sales window. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 13** — Reserve injection mechanism: **Option C — direct commission rebates from Altas One** to each Bencal entity (already implemented in v9 SPV operating budget; manager diluted % stays exactly 10.0000%). `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag 14** — Setup costs Y0 treatment: **IAS 38.69(d) — expense at Y0 as operating loss** (mandatory; formation costs are not intangibles; Y0 IS shows clean operating-loss line equal to setup costs). `[2026-06-02 totebox@claude-opus-4-7]`

### Snapshot flags (§3e)
- [x] **Flag S1** — TSA provider: **FreeTSA (dev) → DigiCert (production)** two-tier; dev cost savings without compromising audit acceptance. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag S2** — Rekor anchoring cadence: **nightly batch + manual ad-hoc** anchor available for time-critical deliverables. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag S3** — Submit snapshot schema to CPA Canada: **Yes, post-Phase B** (when v1.0 ships with 2–3 audit-firm pilot deployments validating it). `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag S4** — Audit-firm partnership: **own dedicated sprint post-Phase A** (after at least one PCLP1 proforma delivered end-to-end). `[2026-06-02 totebox@claude-opus-4-7]`

### D7 Legacy JV flags (§5h)
- [x] **Flag D7-1** — Accounting framework: **IFRS 11 + IFRS 9 fair-value** (overrides ASPE 3061 recommendation; rationale: apples-to-apples comparison with D2/D3 direct-hold solutions which are Reporting Issuers and IFRS-mandated; ASPE would create framework mismatch). `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag D7-2** — Construction draw S-curve: **20 / 50 / 30** (Y1 / Y2 / Y3); per-deal override available via DevClassConfig TOML. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag D7-3** — Portfolio class mix: **lock against WMC tear-sheet matrix** (15 Professional / 9 Suburban Office / 59 Tech Industrial / 37 Retail buildings; 919K/689K/460K/230K sf; 2,298,150 sf total; from `DUE DILIGENCE_MCorp_Tear Sheet_Alternative Real Estate_FIN.xlsx` V3 2026-01-06); the 40/30/20/10 percentages emerge from this geometric matrix; JvConfig TOML references canonical matrix. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag D7-4** — NOI basis: **$78.75M is NET development yield (~10.5%)**, already net of building-level CAM/taxes/operating costs (tenant pass-through); engine MUST NOT apply D1 CAM opex on top (would double-deduct); engine may still apply portfolio-level overhead (asset mgmt, audit, governance). Dual-NOI: "gross NOI" = tenant payment (base + CAM recovery); "net NOI" = base rent − non-recoverable opex; $78.75M sits at net-NOI line. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag D7-5** — Comparison output label: **"illustrative comparison"** (NI 52-107 reserves "pro forma" for specific accounting contexts). `[2026-06-02 totebox@claude-opus-4-7]`

### Reporting Issuer tier flags (§3f)
- [x] **Flag P1** — Issuer status modelled as **Venture issuer (TSXV/CSE)** initially; non-venture is post-graduation event. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag P2** — T5013 output: **full CRA-compatible T5013 Partnership Information Return + per-partner T5013 slip + per-unit capital-account rollforward (ACB, at-risk amount, withdrawals, allocations)**. `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag P3** — Cross-Tier Disclosure Manifest: **available at BOTH Reporting Issuer and Enterprise tiers** (overrides RI-only recommendation; forward-looking-information governance valuable to private issuers planning future RI transition). `[2026-06-02 totebox@claude-opus-4-7]`
- [x] **Flag P4** — Pricing: **$19.00 one-time license under FSL-1.1-Apache-2.0** (replaces $299/$499 SaaS recommendation). Source-available perpetual license; converts to Apache 2.0 two years after each commit. Applies uniformly across ALL product tiers (D2/D3 direct-hold, Enterprise, RI); tier differentiation becomes documentation/onboarding/services depth, not pricing. Materially different economic model: recurring SaaS revenue replaced by one-time license + services revenue. `[2026-06-02 totebox@claude-opus-4-7]`

---

## Phase A implementation queue (after flags cleared)

- [ ] **D1** — Phase A items A1–A12 (CAM itemisation, DevClassConfig TOML, dual NOI, report summary)
- [ ] **D2** — Phase A-D2 items 1–17 (PCLP1 rebuild, Y7 solver, AA12:AN35 summary renderer)
- [ ] **D3** — Phase A-D3 items 1–17 (WCP rebuild, LP cascade, offering costs, G&A ramp, 4 valuation methods)
- [ ] **D4 Bencal SPV2** — Phase A-D4 items 1–15 (pending Jennifer Flags 1-3)
- [ ] **OpexBudget struct** — Phase A-OpEx: implement §3h struct + reserve_sizing + Format A summary table (gates D4/D5/D6)
- [ ] **D5 Bencal SPV1** — Phase A-D5 items 1–17 (pending Jennifer Flag 5 only; Flag 1 cleared)
- [ ] **D6 Bencal Management** — Phase A-D6 items 1–15 (pending Jennifer Flags 1 + 5 + 6)

---

## Stage 6 pending (Command Session action)

- [ ] **Monorepo sub-clone** (`/srv/foundry/clones/project-proforma/pointsav-monorepo/`):
  push `017a8f2d` + `05b0cce6` to origin + staging-j + staging-p `[2026-05-23 totebox@claude-sonnet-4-6]`
- [ ] **Archive git** (`/srv/foundry/clones/project-proforma/`):
  push all commits to origin (9+ commits ahead — run `git log origin/main..HEAD` for exact list) `[2026-05-24 totebox@claude-sonnet-4-6]`

---

## Deferred / backlog

- Number audit: engine-computed values vs Excel tab-by-tab (all 10 years, every row) — Phase B
- Bencal SPV2 standalone SPV costs (d2_direct_hold format still uses scaled PCLP admin_compliance)
- D1 entity name override (TitleCo source entity vs target report header)
- WORM audit ledger — Phase B
- Sensitivity analysis (OAT tornado, 2-variable grid) — Phase C
- Signed PDF bundle (typst) — Phase C
- axum server + Svelte SPA — Phase D
