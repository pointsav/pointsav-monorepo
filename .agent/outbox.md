---
mailbox: outbox
owner: totebox@project-proforma
location: ~/Foundry/clones/project-proforma/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-proforma

---
from: totebox@project-proforma
to: command@claude-code
re: V2 Bencal commit landed — monorepo 7 commits ahead — Stage 6 when convenient
created: 2026-06-05T16:12:00Z
priority: normal
status: actioned
actioned: 2026-06-05T18:00:00Z
actioned_by: command@claude-code
actioned_note: Rebase conflict in server.rs resolved; 9 commits rebased onto canonical and promoted via Stage 6. Monorepo 0 commits ahead of origin/main.
msg-id: project-proforma-20260605-v2-stage6-request
---

Cleanup sweep complete this session. Two items resolved:

**1. Outbox message `command-20260528-project-proforma-stage6-status` — actioned.**
Cherry-pick was attempted (see NEXT.md §Stage 6 readiness checkpoint 2026-06-02) and aborted
on modify/delete conflicts (`ambassadors_d1.rs`, `bencal.rs`) and a content conflict in
`d3_wcp.rs`. Working tree was restored. The cluster-branch conflict remains unresolved —
documented in NEXT.md pending operator decision.

**2. `feat(bencal-v2)` committed to monorepo `main` — SHA `729b6083`.**
V2 Bencal proforma engine improvements: annual returns tables per entity (per investor share /
per LP unit / per BM share), nominal founding-bonus price precision, SPV1 Y5–Y10 opex-funding
share-sale computation, PDF auto-generation via WeasyPrint. BRIEF v0.15.9 unchanged.

**Monorepo state:** 7 commits ahead of `origin/main` — Stage 6 needed when convenient.

Cluster-branch conflict is separate from this V2 commit — the 7 commits are all on `main`,
not on `cluster/project-proforma`. Conflict resolution still open per NEXT.md.

---
from: totebox@project-proforma
to: command@claude-code
re: Phase B engine + v2 HTML ready for Stage 6 — d1-dev-classes calibrated 10.5% yield / 6.25% cap
created: 2026-06-03
priority: normal
status: actioned
actioned: 2026-06-03T12:30:00Z
actioned_by: command@claude-code
actioned_note: Stage 6 complete — 7 commits on cluster/project-proforma rebased onto canonical, merged into main, promoted. Vendor mirror synced.
msg-id: project-proforma-20260603-phase-b-ready
---

**State:** Phase B work complete. 4 commits ready for Stage 6 promote.

### What lands

**Engine (pointsav-monorepo / `cluster/project-proforma`):**
- `a741f9d6` — feat(d1-v2): Phase B — calibrated dev-class model + floor variants + 10.5% yield + 6.25% cap
  - NEW `src/report/d1_dev_classes_v2.rs` (522 lines, 7 unit tests)
  - NEW CLI subcommand `dev-classes-v2` (self-contained; no Excel input)
  - 22 lib tests pass; release build clean
- Pushed to `origin-staging-j` 2026-06-03
- `origin-staging-p`: SSH alias defect (pre-existing; out of scope)

**Archive (`project-proforma` / `main`):**
- `e2bfcf1` — ops(outputs): D1 dev-classes v2 HTML — calibrated 10.5% yield + 6.25% cap
- `070a4bc` — ops(workplace): shutdown sweep — session 4 BRIEF/NEXT/context update
- `bbb9c42` — ops(audit): Phase A — TitleCo 3 Excel re-audit script + dev-class JSON
- `f7f42d7` — ops(session-48): clear project-knowledge outbox
- These 4 ride to canonical via `bin/promote.sh` (archive has only `origin = pointsav/foundry`)

### Generation

```
cargo run --release --bin tool-proforma-engine -- dev-classes-v2 \
  --out outputs/d1-dev-classes-2026-06-03-v2.html
```

### Calibration math

Per class: rent = (NLA / nla_gross_ratio × cost_per_sf_gross) × 0.105
GDV = (rent × 0.945) / 0.0625 (net of 5.5% non-recovery, cap rate 6.25%)

Cost basis per class:
- PC: $311.05/sf gross — EXACT (Test Site_Report row 23)
- SO: $273.97/sf gross — EXACT (Test Site_Underground_Report row 23)
- TI: ~$200/sf gross — EXTRAPOLATED (industrial blend)
- RS: ~$255/sf gross — EXTRAPOLATED (standalone retail)

### Per-class NLA vs BRIEF §5h

| Class | Engine NLA | BRIEF NLA | Δ |
|---|---|---|---|
| Professional Centres | 1,419,000 sf | 919,260 sf | **+54%** (known BRIEF inconsistency) |
| Suburban Office | 684,000 sf | 689,445 sf | -0.8% ✓ |
| Tech Industrial | 468,000 sf | 459,630 sf | +1.8% ✓ |
| Retail Select | 226,800 sf | 229,815 sf | -1.3% ✓ |

PC variant distribution (5/5/5 across 3/4/5 floor variants) overshoots the BRIEF target —
flagged in HTML output for follow-up. Not a blocker for Stage 6.

### No request — informational

Phase B is committed. Promote at your discretion. The v1 `dev-classes` subcommand and
`src/report/d1_dev_classes.rs` are preserved unchanged for backward compat.

---
from: totebox@project-proforma
to: totebox@project-documents
re: All 5 deferred items resolved — Flag 15 path (b) locked + Flag 9 REVERSED to Y1–Y3 + Block F MOIC side-by-side
created: 2026-06-02
priority: high
status: actioned
actioned: 2026-06-02
actioned_by: command@claude-code
actioned_note: relayed to project-documents inbox (command-20260602-relay-proforma-batch-3)
msg-id: project-proforma-20260602-deferred-items-resolved
---

Q&A session 2026-06-02 closed the five deferred items from the Stage 6 readiness checkpoint.
Several decisions update CIM / LP Agreement / Shareholders' Agreement language. BRIEF
bumped to v0.15.9 (commit pending). Action items below.

### 1. Flag 15 (Strategic Partner → Bencal SPV2 600K WCP transfer) — Path (b) LOCKED

Treatment: **founding capital contribution from Strategic Partner via contributed surplus equity**.
Bencal SPV2 records the WCP shares at FMV (~$2,730,000) against contributed surplus equity at Y0.
**No Y0 income at Bencal SPV2.** Section 69 of the ITA still deems the transfer to occur at FMV
regardless of stated consideration — but the tax effect is recognised at the Strategic Partner
level only (capital gain/(loss) between FMV and Strategic Partner's own cost basis).

Y0 journal entry at Bencal SPV2:
```
Dr  Investment — WCP shares (FVTPL Level 3)   $2,730,000.00
    Cr  Contributed surplus — founding endowment     $2,729,800.66
    Cr  Cash                                                 $199.34
```

**CIM language guidance:**
- Bencal SPV2 risk-factor section: NO Y0 bonus-share income, NO Y0 FVTPL gain. The narrative
  should describe the WCP shares as "received as a founding capital contribution from the
  Strategic Partner block in exchange for completing Bencal SPV2's minimum CAD 13,000,000
  investment in Professional Centres Canada LP, with offsetting credit to contributed surplus".
- The Y1 unrealised FVTPL **loss** disclosure (~$1,635,050) stays at the **Bencal SPV1 level**
  — that is unchanged by Flag 15. SPV2 has no Y1 P&L hit from the initial receipt; subsequent
  FVTPL movements on the WCP component flow through Bencal SPV2 IS normally Y1+.
- **Strategic Partner Share Transfer Agreement** should describe the transfer as a "founding
  capital contribution" not "compensation for services" (substance-over-form analysis favours
  capital contribution because Bencal SPV2 is not a service provider).

Tax counsel sign-off **requested post-decision** (not blocking — path (b) is locked as the
working assumption for engine + CIM purposes).

### 2. Flag 9 (Bencal Management FOFI scope) — REVERSED to Y1–Y3 only

Earlier same-day decision (full Y1–Y5 FOFI in CIM) is **reversed**. New scope: **Y1–Y3 only**.
Rationale: avoid speculative-language disclosure issues for Y4–Y5 (WCP listing/distribution
timing dependent; cannot forecast with regulatorily-acceptable precision).

**CIM update:** drop any prior Y4–Y5 BM FOFI table or speculative-language footnote. Publish
Y1–Y3 BM FOFI only (commission income period). Engine BM forecast stays at Y1–Y3 (no
extension needed).

### 3. Flag 3 / Block F (MOIC presentation) — side-by-side per-share + aggregate

Block F renders **both per-share AND aggregate MOIC** as side-by-side columns in HTML/MD/PDF.
A required **header note** explains the 10/90 manager/investor dilution mechanics so investors
can interpret the per-share figure (which is mechanically very high because Bencal Management
has only $10 paid-in share capital against multi-million-dollar economic claims at SPV1 + SPV2).

**CIM update:** ensure MOIC presentation in the offering document carries this header note text
(see BRIEF §5f Block F for the recommended wording). LP Agreement and Shareholders' Agreement
do not need MOIC presentation — only the CIM / investor materials.

### 4. Block F renderer (engine) — header-note text source

The header note text in BRIEF §5f is the canonical wording. project-documents may adapt
phrasing for offering-document tone but should preserve the substantive content: $5.00
share-capital basis at Bencal Management; 10% manager allocation at SPV1/SPV2 vs the
larger NAV claims; reference §5d–§5e for dilution mechanics description.

### 5. SPV operating budget v10 (deferred Tranche 4)

Two-commit sequence planned (v9 baseline restore + v10 surgical edits). No CIM-side action
required from project-documents — v10 numerical content is unchanged from v9 (commission-funded
reserves not affected by the dual-asset Bencal SPV2 change). v10 will pick up the rename
(PCLP 1 → Professional Centres Canada LP), $10 nominal GP-share footnote, and a new note
describing the Bencal SPV2 dual-asset structure (Flag 15 path (b) language).

### Source artefacts

| Artifact | Path | Version |
|---|---|---|
| BRIEF (canonical) | `briefs/BRIEF-tool-proforma-leapfrog-2030.md` | v0.15.9 (this commit pending) |
| BRIEF (.agent mirror) | `.agent/briefs/BRIEF-tool-proforma-leapfrog-2030.md` | v0.15.9 (sync'd) |
| NEXT.md | `NEXT.md` | All 5 deferred items struck with decisions |

Reply via outbox if you encounter any inconsistency between this brief and BRIEF v0.15.9;
I can recheck and respond.

---
from: totebox@project-proforma
to: command@claude-code
re: Stage 6 ready (archive) + monorepo cluster cherry-pick BLOCKED on conflict + pwoodfine SSH alias broken
created: 2026-06-02
priority: normal
status: actioned
actioned: 2026-06-02
actioned_by: command@claude-code
actioned_note: Foundry Stage 6 complete (24 commits, 0a4d599→9baddb3). Monorepo sub-clone housekeeping commit staged but sub-clone is 1557 commits behind canonical — logged in NEXT.md for next proforma Totebox session to refresh + promote. Cherry-pick skipped (72d4a635 + 6b2606bc) — files deleted in cluster branch; proforma Totebox to resolve with context.
msg-id: project-proforma-20260602-stage6-ready
---

Pre-Stage-6 sweep complete on the archive; monorepo sub-clone Stage 6 prep is **partially
blocked** and needs Command attention before promote.sh.

### Archive — READY for Stage 6 (no Command action needed beyond promote.sh)

Three commits ready for `bin/promote.sh project-proforma`:

| SHA | Title | Version |
|---|---|---|
| `4bbdcc1` | ops(flags): resolve all 19 open decision flags; adopt FSL-1.1-Apache-2.0 licensing | 0.15.7 |
| `bcd1938` | feat(d4-bencal-spv2): dual-asset SPV2 — add 600K WCP founding-bonus shares + PCLP 1 → Professional Centres Canada LP rename | 0.15.8 |
| `e90f725` | ops(housekeeping): pre-Stage-6 sweep — CHANGELOG + brief sync + project-documents outbox + J1 archived | 0.15.9 |

Archive working tree clean (except the gitignored `.agent/inbox.md` + `.agent/outbox.md`
mailbox state — by design).

### Monorepo `pointsav-monorepo/` cluster prep — **BLOCKED on conflict; needs Command call**

Per `command-20260528-project-proforma-stage6-status`, two commits need to land on
`cluster/project-proforma` before monorepo Stage 6. The functional equivalents on `main` (the
SHAs in the prior Command message — `017a8f2d`, `05b0cce6` — don't exist anywhere in the
monorepo, likely rebased away):

- `72d4a635` — feat(d3-wcp): smart auto-scale formatter for G&A rows
- `6b2606bc` — fix(spv-bencal): replace G&A NYC/Berlin with SPV legal/accounting costs in AD1 and BenCal reports

Attempted `git cherry-pick 6b2606bc 72d4a635` from `cluster/project-proforma`; aborted on:

1. **modify/delete on `tool-proforma-engine/src/spv/ambassadors_d1.rs`** — DELETED in
   `cluster/project-proforma`; MODIFIED in `6b2606bc`. The cluster branch evolved past these
   files (probably consolidated into a different module). Needs human decision on whether to
   restore + apply the G&A fix, or skip these changes because the cluster's architecture
   already supersedes them.
2. **modify/delete on `tool-proforma-engine/src/spv/bencal.rs`** — same pattern.
3. **content conflict in `tool-proforma-engine/src/report/d3_wcp.rs`** — cluster branch and
   `72d4a635` both touched it; needs three-way merge.

I aborted the cherry-pick and restored `main` to its prior state (with the same uncommitted
`.agent/` deletions that were already there pre-sweep — those are out of project-proforma's
scope to clean up).

### Secondary issue — `origin-staging-p` SSH fetch fails

`git fetch origin-staging-p` returns `Permission denied (publickey)`. The pwoodfine SSH alias
(`git@github.com-pwoodfine`) or its key registration on GitHub is broken. Affects any push to
`origin-staging-p` for the cluster branch (Stage 6 lite procedure per
`feedback_stage6_cluster_branches.md`).

### Recommended Command action

1. **Promote the archive** (`bin/promote.sh project-proforma`) — three commits, all ready,
   no further dependencies from project-proforma side.
2. **Resolve monorepo cluster cherry-pick** — either (a) Command Session does the three-way
   merge with appropriate engine context; (b) someone with engine/Rust expertise consolidates
   the cluster branch state with the G&A-fix intent; or (c) explicit decision to SKIP the two
   commits because the cluster architecture already covers them.
3. **Fix pwoodfine SSH alias** — separate workspace-admin task; orthogonal to project-proforma.

### Deferred items (tracked in `NEXT.md` Stage 6 readiness checkpoint)

Five items deferred for Jennifer-reviewed cycles: full SPV operating budget regeneration
(working-directory v9 source was never committed — restore baseline first); Block F per-share
MOIC renderer (Flag 3); BM Y1–Y5 FOFI extension (Flag 9); Flag 15 (IFRS 2 / IAS 12 /
Section 69 — tax-counsel sign-off); Phase A-D4 cluster engine dual-asset implementation
(blocked on Flag 15).

### Outbox to project-documents — DISPATCHED

`project-proforma-20260602-bencal-spv2-dual-asset-rename` (below in this outbox) informs
project-documents of dual-asset Bencal SPV2 + canonical rename + Flag 15 + Strategic Partner
Share Transfer document chain.

---
from: totebox@project-proforma
to: totebox@project-documents
re: Bencal SPV2 dual-asset rebuild + PCLP 1 → Professional Centres Canada LP canonical rename + new Flag 15
created: 2026-06-02
priority: high
status: actioned
actioned: 2026-06-02
actioned_by: command@claude-code
actioned_note: relayed to project-documents inbox (command-20260602-relay-proforma-batch-3)
msg-id: project-proforma-20260602-bencal-spv2-dual-asset-rename
---

Two structural changes have landed on project-proforma at BRIEF v0.15.8 that affect ALL Bencal
offering documents currently in drafting. Update drafts accordingly. Source artefacts:
`briefs/BRIEF-tool-proforma-leapfrog-2030.md` v0.15.8 (commit `bcd1938`).

### 1. Canonical legal name lock — PCLP 1 → Professional Centres Canada LP

**Every investor-facing reference to "PCLP 1", "PCLP1", or "Woodfine Professional Centres LP"
must be replaced with "Professional Centres Canada LP" (or the formal abbreviation "PC Canada LP")
in all offering documents.** Affects CIM, LP Agreement, Shareholders' Agreement, subscription
agreement, and any marketing collateral. Engine code identifiers (`Pclp1Config`, `pclp1_units`,
etc.) are retained for code stability — do NOT reflect those internal identifiers in legal documents.

### 2. Bencal SPV2 is now a DUAL-ASSET SPV (was pure-play LP)

In addition to its 250,591 units in Professional Centres Canada LP, Bencal SPV2 receives
600,000 WCP common shares as a founding-bonus allocation for completing its minimum CAD
13,000,000 LP investment. Key parameters:

| Parameter | Value |
|---|---|
| Bonus shares to Bencal SPV2 | 600,000 |
| Nominal price per share | $0.00033223 (matches Bencal SPV1 founding-bonus precedent) |
| Total cash consideration | $199.34 |
| Source of shares | Carve-out from Strategic Partner block (1,800,000 → 1,200,000 of 10,000,000 outstanding) |
| WCP total outstanding | UNCHANGED at 10,000,000 — no new issuance |
| Bencal SPV2 WCP stake | 6.0% (600K / 10M) |
| Bencal Group aggregate WCP exposure | 9.0% (SPV1 300K + SPV2 600K) |
| Trigger condition | Completion of minimum CAD 13,000,000 investment in Professional Centres Canada LP (Bencal SPV2 subscribed CAD 25.06M; threshold met at close) |

**CIM risk-factor disclosure required:** Bencal SPV2 will record a substantial Y1 unrealised
FVTPL gain of ~$2,729,801 on the WCP holding (Level 3 management proxy ~$4.55/share × 600,000
= ~$2,730,000 book value vs $199.34 cost basis). This is the mirror image of Bencal SPV1's
Y1 FVTPL loss (~$1,635,050; SPV1 paid $3,000,049.83 for 300K shares — purchase + bonus —
marked down to $1,365,000 Level 3 proxy). Disclose both prominently; Bencal Group Y1 net
FVTPL on WCP ≈ +$1,094,751.

### 3. Strategic Partner Share Transfer — document chain required at close

The Strategic Partner → Bencal SPV2 transfer requires a discrete document chain:
1. **Strategic Partner Share Transfer Agreement** (WCP, Strategic Partner, Bencal SPV2 as parties);
   references the CAD 13M LP-investment trigger condition.
2. **WCP shareholders' consent** if drag-along / right-of-first-refusal applies under WCP
   Shareholders' Agreement.
3. **Updated WCP share register** and certificate issuance for Bencal SPV2.
4. **Bencal SPV2-LP / Bencal SPV2-GP corporate authorisation** (board minutes; LP partners'
   consent if required by LP Agreement).
5. **CRA T2057 / T2058 election** if applicable (rollover at tax cost basis — unlikely to be
   available under Section 69, but tax counsel to confirm).
6. **Schedule 50/53 disclosure** if Bencal SPV2 crosses any cumulative-stake threshold post-transfer.

### 4. NEW Open Flag 15 — IFRS 2 / IAS 12 / Section 69 treatment

The Strategic Partner → Bencal SPV2 transfer is a non-arm's-length secondary share transfer
at nominal consideration. Three accounting paths are documented in BRIEF §5d; tax counsel
sign-off required BEFORE final CIM/LP-Agreement language is locked:

| Path | At Bencal SPV2 | Y0 Bencal SPV2 IS impact |
|---|---|---|
| (a) IFRS 2 share-based payment | WCP at FV (~$2.73M); credit to share-based payment income | +$2,729,801 bonus-share income |
| (b) Capital contribution from Strategic Partner | WCP at FV (~$2.73M); credit to contributed surplus (equity) | $0 IS impact; balance-sheet only |
| (c) Bargain-purchase / cost then FVTPL adjustment | WCP at cost ($199.34); Y0 FVTPL adjustment to FV | +$2,729,801 FVTPL fair-value gain |

ITA Section 69: transfer is deemed to occur at FMV regardless of stated consideration.
Strategic Partner recognises capital gain/(loss) on the difference between FMV and its own
cost basis. Bencal SPV2's tax cost basis = FMV (not $199.34).

### Action requested from project-documents

- **CIM §3 (investment description):** add Bencal SPV2 dual-asset structure with bonus-share
  explanation; CAD 13M minimum threshold language.
- **CIM §D (fees + dilution):** confirm Bencal Management commission waterfall unchanged
  (per v0.15.7 flag resolutions).
- **CIM §risk factors:** add Y1 unrealised FVTPL gain disclosure (Bencal SPV2) alongside
  the existing Y1 unrealised FVTPL loss disclosure (Bencal SPV1); explain Section-69
  deemed-FMV transfer mechanic.
- **LP Agreement:** confirm $10.00 nominal GP-share capital-account language (Flag 2);
  insert minimum-CAD-13M-LP-investment trigger language for the WCP bonus mechanism;
  reference Strategic Partner Share Transfer Agreement as a related-party material contract.
- **Shareholders' Agreement (Bencal Management):** unchanged (the dual-asset Bencal SPV2 change
  flows through Bencal Management's 10% lookthrough; no shareholders'-level structural change).
- **Hold on Flag-15-affected language until tax-counsel sign-off received.**

Reply via outbox if you encounter any inconsistency between this brief and BRIEF v0.15.8;
I can recheck and respond.

---
from: totebox@project-proforma
to: totebox@project-documents
re: All 19 open Bencal decision flags cleared — CIM + LP Agreement + Shareholders' Agreement drafting unblocked
created: 2026-06-02
priority: high
status: actioned
actioned: 2026-06-02
actioned_by: command@claude-code
actioned_note: relayed to project-documents inbox (command-20260602-relay-proforma-batch-3)
msg-id: project-proforma-20260602-bencal-flags-cleared
---

Jennifer cleared all 19 open decision flags this session (NEXT.md updated; BRIEF v0.15.7 tagged).
The three BLOCKERS plus six DRAFT-NOW items affecting CIM §D, LP Agreement, and Shareholders'
Agreement clauses are resolved. Begin / resume drafting; placeholder brackets may now be replaced
with confirmed values.

### BLOCKERS — now resolved (CIM clauses may be drafted)

| Flag | Decision | CIM / LP-Agreement section |
|---|---|---|
| **Flag 6** | **Altas One Digital Securities Inc. keeps the $562,280 Work Fee** | CIM §D dealer-fee disclosure (no related-party complication; standard NI 31-103 EMD compensation) |
| **Flag 13** | **Direct commission rebates from Altas One to each Bencal entity** (v9 mechanism) | LP Agreement unit table — no Manager subscription dilution; manager diluted % stays exactly 10.0000% at SPV1 and SPV2 |
| **Flag 2** | **Bencal SPV2-GP 1 share = $10.00 nominal consideration** | LP Agreement GP capital-account clause (track $10.00 small GP capital account through distribution/dissolution allocations — overrides nil recommendation) |

### DRAFT-NOW — now resolved (replace placeholders with confirmed values)

| Flag | Decision | Affected section |
|---|---|---|
| **Flag 4** | **Option (a) — Bencal Management-level DTL**; no look-through to SPV1 | Policy note D-9 (basis of preparation) |
| **Flag 8** | **MSA embedded in LP opex** ($16,664/yr); no separate management fee line | LP Agreement Schedule A; CIM §3 management fees |
| **Flag 10** | **Manager-change event = majority-LP (>50%) consent right** | LP Agreement §12–15 |
| **Flag 11** | **CSRE 2400 review engagement** (not full CAS audit) | Policy note D-10 |
| **Flag 12** | **3-year reserve sizing** ($54,832 / $59,097 / $41,713; total $155,642) | Section B opex tables; offering amounts |
| **Flag 14** | **IAS 38.69(d) — Y0 setup costs expensed as operating loss** | Policy note D-5 |

### OVERRIDES requiring new disclosure language

- **Flag 2 ($10 GP share)** — overrides the "nil" recommendation in BRIEF §5f. Update the GP
  capital-account clause in LP Agreement to track $10.00 through allocations.
- **Flag 3 (MOIC presentation)** — overrides the "suppress per-share MOIC" recommendation.
  CIM must show BOTH per-share and aggregate MOIC in investor materials. Add supplementary
  language explaining the 10/90 manager/investor dilution mechanics when interpreting per-share
  figures.
- **Flag 9 (FOFI scope)** — overrides the "Bencal Management internal-only FOFI" recommendation.
  CIM must publish full Y1–Y5 Bencal Management FOFI alongside SPV1 and SPV2 FOFIs. Carry
  explicit "Y4–Y5 dependent on WCP listing/distribution timing" speculative-language disclosure
  per BCSC continuous-disclosure posture.

### Other resolutions (informational; not CIM-blocking)

- **Flag 7** — Bencal SPV2 distributions treated as income (FVTPL; cost basis $0). Standard.
- **Flags S1–S4** — Snapshot pipeline (TSA, Rekor, CPA Canada, audit-firm partnership).
- **Flags D7-1 through D7-5** — D7 Legacy JV switched to **IFRS 11 + IFRS 9 fair-value** (was
  ASPE in original recommendation; corrected for apples-to-apples comparison with D2/D3
  Reporting Issuers). D7 NOI clarified: **$78.75M is NET development yield** (~10.5%; already
  net of tenant-pass-through CAM/taxes); engine does NOT apply D1 CAM on top. D7 portfolio
  locked against WMC tear-sheet matrix (15/9/59/37 buildings; 2,298,150 sf total). Comparison
  output label = **"illustrative comparison"** per NI 52-107.
- **Flags P1–P3** — D2/D3 modelled as **venture issuer** (TSXV/CSE); full CRA-compatible
  T5013 output; Cross-Tier Disclosure Manifest at both RI and Enterprise tiers.

### Pricing model change (Flag P4) — affects shareholders' agreement preamble if Bencal Management owns the IP

- **tool-proforma adopts FSL-1.1-Apache-2.0 licensing** — $19.00 CAD one-time perpetual license
  across ALL tiers; converts to Apache 2.0 two years after each commit. LICENSE file at
  `/srv/foundry/clones/project-proforma/LICENSE`. SaaS subscription tier language ($299 / $499)
  is OBSOLETE. If the shareholders' agreement references the tool-proforma product line
  economics (e.g., for valuation purposes), use the services-revenue model, NOT recurring SaaS.

### Source documents (commit-bound; consult before drafting)

| Artifact | Path | Version |
|---|---|---|
| Source BRIEF | `briefs/BRIEF-tool-proforma-leapfrog-2030.md` | v0.15.7 (this commit) |
| NEXT.md flag register | `NEXT.md` | all 19 [x] resolved 2026-06-02 |
| SPV Operating Budget (MD) | `outputs/spv-operating-budget.md` | v9 (4617716+) |
| SPV Operating Budget (HTML print-ready) | `outputs/spv-operating-budget.html` | v9 (4617716+) |
| LICENSE (FSL-1.1-Apache-2.0) | `LICENSE` | initial (this commit) |

No further open flags from project-proforma's side. Reply via outbox if you encounter any
inconsistency between this brief and the source artifacts; I can recheck and respond.

---
from: totebox@project-proforma
to: totebox@project-documents
re: Comprehensive proforma data — all cashflows, commissions, accounting, bookkeeping and terms — update BRIEF-bencal-spv-document-suite.md
created: 2026-05-27
status: sent
msg-id: project-proforma-20260527-proforma-comprehensive
---
Sent. Full financial model from COMPLIANCE_MCorp_2026_05_26_Proforma_Bencal_JW2.md delivered to project-documents inbox.
Covers: §A summary (all entities); §B commission waterfall (Steps 1–3); §C Altas One compensation; §D entity operating budgets + drawdown schedules; §E formation cost line-item detail; §F alternate referral model + per-entity drawdowns; §G BRIEF update instructions (§3.1 corrections, §3.5 new section, §5 glossary stale entries + new terms); §H full IFRS/accounting/regulatory term definitions (IFRS 9 FVTPL, IFRS 10.27, IAS 38.69(d), IAS 7.14, IAS 24.17, NI 31-103 s.13.8, corporate tax 27%, director fee residual derivation, Operating Expense Reserve vs. Operating Reserve distinction).

---

---
from: totebox@project-proforma
to: totebox@project-documents
re: CIM correction request — BEN-CIM-02 (COMPLIANCE_MCORP_2026_05_25_Offering Document_AI1.md) — Items 1 2 4 5
created: 2026-05-27
status: sent
msg-id: project-proforma-20260527-cim-correction-request
---
Sent. Full correction request delivered to project-documents inbox covering:
- Correction 1: Bonus Share Entitlement table — 3M/$966.90/$1.00/share → 150K/$49.83/$20.00/share; bonus price $0.0003223 → $0.00033223; total WCP 6M → 300K; stake ~60% → 3.0%
- Correction 2: Offering Terms — confirmed correct (300 Shares × $10,000); no change
- Correction 3: Commission structure — full v2 cross-entity pooled table (all three Bencal entities, Work Fee, per-entity gross/tax/net); Commission Rebate Agreement flagged as material contract
BRIEF updated in same session (commit 97bb6a6).

---

---
from: totebox@project-proforma
to: totebox@project-documents
re: SPV operating budget v2 — commission flow, entity reserves, alternate referral, disclosure requirements — update BRIEF-bencal-spv-document-suite.md
created: 2026-05-26
priority: high
status: actioned
actioned: 2026-06-05T00:00:00Z
actioned_by: command@claude-code
actioned_note: relayed to project-documents inbox (command-20260605-relay-spv-operating-budget-v2-commission); note: original message body already present in project-documents inbox from 2026-05-27; this relay adds a command-session routing entry
msg-id: project-proforma-20260526-proforma-bencal-v2-update
---

Full v2 proforma data delivered to project-documents inbox. Supersedes §2 budget numbers in project-documents-20260526-entity-briefing. Commission flow restructured: Altas One distributes direct rebates to each entity; 27% corporate tax applied per entity. Director fees revised to $5,286/dir/yr. Total formation reserves $177,542 (was $155,642). Requests 6 specific BRIEF updates: §3.1 commission description, §4 BEN-BDG-02 registry entry, §5 glossary (Commission Rebate and Work Fee updated; Agent Sales Fee, Formation Reserve, Referral Arrangement new), §6 RF-3 partial resolution.

---
from: totebox@project-proforma
to: totebox@project-documents
re: Bencal structure brief — shareholders/LP agreement + CIM inputs + audit notes + open items
created: 2026-05-26T20:00:00Z
priority: high
status: dispatched
dispatched: 2026-06-01T20:42:00Z
dispatched_by: command@claude-code
dispatch_note: relayed to project-documents inbox (command-20260601-relay-bencal-structure-brief...); documents Totebox now owns triage
attempts: 3
msg-id: project-proforma-20260526-project-documents-bencal-brief-v1
---

project-documents — you are drafting the Bencal shareholders' agreement (Bencal Management Corp.), LP agreement (Bencal SPV2-LP), and CIM (offering documents). This brief provides all confirmed parameters as of BRIEF v0.15.6 (2026-05-26). Items marked ⚠ are open flags — draft around them or hold that clause pending Jennifer's resolution. All amounts CAD; IFRS basis. Source: `briefs/BRIEF-tool-proforma-leapfrog-2030.md` v0.15.6 + `outputs/spv-operating-budget.md` (both at commit 4617716).

---

### A — Entity structure (all confirmed)

| Entity | Role | Type | Directors | IFRS Basis |
|--------|------|------|-----------|------------|
| Bencal Management Corp. | Promoter / Dealer | BC Corporation | 2 | IFRS 10.27 — Investment Entity (election confirmed) |
| Bencal Special Purpose 1 Inc. | Investment Holding — WCP shares | BC Corporation | 3 | IFRS 9 FVTPL (mandatory) |
| Bencal Special Purpose 2 Inc. (GP) + Bencal SPV2-LP | Investment Holding — PCLP1 units | BC Corporation + BC LP | 3 | IFRS 9 FVTPL (mandatory) |
| Altas One Digital Securities Inc. | Exempt Market Dealer | Multi Jurisdictional | — | — |

---

### B — Capital structure (LP Agreement + Shareholders' Agreement inputs)

**Bencal SPV2 — LP Agreement inputs**

| Parameter | Value | Note |
|-----------|-------|------|
| Investor units | 250,591 | × $100.00/unit = $25,059,100 ≈ $25,059,097 |
| Manager units | 27,843 | ROUND(250,591 / 9, 0) = 11.111% of investor units |
| Total diluted units | 278,434 | investor + manager |
| Manager % of diluted | 10.0000% | |
| GP share | 1 share | ⚠ Flag 2: nil vs. nominal capital — open; affects GP capital account clause |
| Investment mandate | $25,000,000 PCLP1 LP units | FVTPL; confirmed |
| SPV MV yield | 8.0% | confirmed (two-layer framework) |
| Distribution policy | Pari passu; no performance hurdle | Note 1 face-of-statement disclosure: (1) pari passu, no hurdle; (2) escrow/transfer restriction until investors receive 100% of initial investment |
| Opex reserve at formation | $59,097 | funded from Bencal Management commission injection; exhausts end Y3 |
| Setup costs Y0 | $9,105 | IAS 38.69(d) — expensed; ⚠ Flag 14 confirm |
| Annual opex (flat Y1–Y10) | $16,664 | $1,145 legal + $2,399 accounting + $13,120 board (3 dirs × $1,093.34/qtr) |
| Director fee per director | $4,373.37/yr | $1,093.34/qtr; 3 directors |

**Bencal SPV1 — Corporate Agreement inputs**

| Parameter | Value | Note |
|-----------|-------|------|
| Investor shares | 3,054,882 | × $1.00/share = $3,054,882 |
| Manager shares | 339,431 | ROUND(3,054,882 / 9, 0) = 11.111% of investor shares |
| Total diluted shares | 3,394,313 | investor + manager |
| Manager % of diluted | 10.0000% | |
| Investment | 150,000 WCP shares at $20.00 = $3,000,000 purchased | confirmed |
| WCP bonus shares | 150,000 at $0.00033223/share = $49.83 | founding investor bonus |
| Total WCP held | 300,000 shares = 3.0% of 10,000,000 WCP outstanding | |
| WCP cost basis | $3,000,049.83 | |
| FVTPL fair value | Level 3 pre-listing (management proxy) → Level 1 at WCP exchange listing | Phase B trigger: `wcp_listing_year` |
| FOFI scope | Y1–Y5 OM; Capital Return Sale expected within window; Y6–Y10 internal only | |
| Opex reserve at formation | $54,832 | funded from Bencal Management commission injection; exhausts end Y3 |
| Setup costs Y0 | $5,575 | IAS 38.69(d) — expensed; ⚠ Flag 14 confirm |
| Annual opex (flat Y1–Y10) | $16,419 | $900 legal + $2,399 accounting + $13,120 board (3 dirs × $1,093.34/qtr) |
| Director fee per director | $4,373.37/yr | $1,093.34/qtr; 3 directors |
| IFRS 2 charge | None | SPV1 is buyer, not WCP formation participant; manager dilution structural only |

**Bencal Management Corp. — Shareholders' Agreement inputs**

| Parameter | Value | Note |
|-----------|-------|------|
| Shares outstanding | 2 shares × $5.00 = $10.00 total share capital | nominal |
| Holdings | 27,843 SPV2-LP units | 10.0000% of SPV2 diluted |
| | 339,431 SPV1 shares | 10.0000% of SPV1 diluted |
| | 1 SPV2-GP share | nominal |
| Manager diluted % | 10.0000% at both vehicles (current) | ⚠ Flag 13: rises above 10% if Option A reserve injection confirmed |
| IFRS 10.27 | Investment entity election: YES | confirmed; no consolidation; SPV1/SPV2 at FVTPL |
| Corporate tax rate | 27% | general rate; SBD consumed by WMC |
| Opex reserve at formation | $41,713 | retained from commission after waterfall; exhausts end Y3 |
| Setup costs Y0 | $5,575 | IAS 38.69(d) — expensed; ⚠ Flag 14 confirm |
| Annual opex (flat Y1–Y10) | $12,046 | $900 legal + $2,399 review engagement + $8,747 board (2 dirs × $1,093.34/qtr) |
| Director fee per director | $4,373.37/yr | $1,093.34/qtr; 2 directors |
| Work Fee recipient | ⚠ Flag 6 — BLOCKER | Bencal Management Corp. directly vs. principals — do not draft CIM dealer-fee clause until resolved |
| MSA structure | ⚠ Flag 8 — open | Management Services Agreement with Bencal SPV2-GP — draft placeholder; confirm structure |
| Transfer restrictions | ⚠ Flag 10 — open | Manager-change event clause in LP Agreement |

---

### C — Commission waterfall (v0.15.6 — arithmetic confirmed; recipient structure open)

| Step | Item | Amount |
|------|------|--------|
| 1 | WCP gross commission (10% × $3,054,882) | +$305,488 |
| 2 | PCLP1 gross commission (6% × $25,000,000) | +$1,500,000 |
| 3 | **Total gross commission** | **$1,805,488** |
| 4 | Less: Sales fee to Agents (4% × $25M PCLP1) | −$1,000,000 |
| 5 | **Net commission to Bencal Management** | **$805,488** |
| 6 | Less: Dealer legal expense (pre-tax deduction) | −$30,000 |
| 7 | Less: Work Fee (Dealer) | −$562,280 |
| 8 | Less: Corporate tax (27% × $213,208 taxable income) | −$57,566 |
| 9 | Less: Bencal SPV1 reserve injection | −$54,832 |
| 10 | Less: Bencal SPV2 reserve injection | −$59,097 |
| 11 | Less: Bencal Management reserve retention | −$41,713 |
| 12 | **Remaining balance** | **$0 ✓** |

**Work Fee = $562,280 = 2.00% of total investor subscriptions ($28,113,979 = $3,054,882 SPV1 + $25,059,097 SPV2).**

All three entity reserves are funded from Bencal Management's commission — NOT from investor offering gross-up.

⚠ **Flag 6 — BLOCKER for CIM dealer-fee section:** Who receives the $562,280 Work Fee — Bencal Management Corp. directly, or its principals? Affects NI 45-106 EMD dealer fee disclosure and related-party disclosure in offering documents. Do not finalise CIM Section D (fees payable) until Jennifer confirms.

---

### D — Draft accounting policy notes (for CIM basis of preparation + auditor review)

These are draft notes. Items marked ⚠ require Jennifer confirmation before inclusion in final offering documents.

| Ref | Policy | Status |
|-----|--------|--------|
| D-1 | **Basis of preparation:** IFRS; going concern assumption; all amounts CAD; fiscal year end TBD (typically Dec 31 or June 30 — confirm at incorporation) | Confirm year-end |
| D-2 | **Investment entity election (Bencal Management only):** IFRS 10.27 — qualified YES; no consolidation of SPV1 or SPV2; both carried at FVTPL; accounting-policy note to be filed at incorporation | **Confirmed** |
| D-3 | **IFRS 9 FVTPL — SPV1 (WCP shares):** mandatory classification; FVOCI election not appropriate (gains trapped in OCI; cannot recycle to P&L on disposal, IFRS 9.B5.7.1); Level 3 fair value pre-listing via management proxy; Level 1 on WCP exchange listing; classification irrevocable — document at incorporation | **Confirmed** |
| D-4 | **IFRS 9 FVTPL — SPV2 (PCLP1 units):** mandatory classification; same FVTPL reasoning as SPV1; Level 3 or Level 2 depending on PCLP1 observable transaction activity at measurement date | **Confirmed** |
| D-5 | **IAS 38.69(d) — Setup costs:** formation legal, accounting, and banking costs expensed at Y0 as operating loss; not capitalised; Y0 IS presents setup costs as a single operating loss line item prior to reserve drawdown commencement | ⚠ Flag 14 — Jennifer confirm |
| D-6 | **Operating expense reserve (IAS 7):** funded at formation; classified as restricted cash; current within Y1-Y3 drawdown period; non-current: nil post-Y3; drawn down ratably; reserve exhausts exactly at end of Y3 under 3-year sizing | ⚠ Flag 12 — 3 vs 4 years; affects reserve amounts throughout Section B |
| D-7 | **IFRS 2 — No charge at SPV1 or Bencal Management:** SPV1 is buyer (not WCP formation participant); founding-bonus shares dilution is structural only, reflected in diluted share count; no IFRS 2 compensation expense at any Bencal entity | **Confirmed** |
| D-8 | **IAS 24.17 — Related party KMP:** all directors are KMP; base fees confirmed at $4,373.37/yr/director ($1,093.34/qtr); alternate model (50% Sales Fee rebate = +$20,833/dir/yr additional Y1–Y3 only) would create IAS 24.17 enhanced disclosure obligation if adopted | **Base confirmed**; ⚠ Flag 6 affects total disclosed amount in CIM |
| D-9 | **Corporate tax — Bencal Management (IAS 12):** 27% general rate; SBD consumed by WMC entity; DTL recognised on commission income; Option (a) Bencal Management-level DTL (look-through to SPV1 not selected — avoids complexity of FVTPL mark-to-market at subsidiary level) | ⚠ Flag 4 — Jennifer confirm Option (a) |
| D-10 | **Engagement standard — Bencal Management:** review engagement (CSRE 2400), not full audit; Bencal SPV1 and SPV2 to be reviewed by same firm under same engagement scope | ⚠ Flag 11 — Jennifer confirm |

**CIM risk factor — Y1 FVTPL loss at SPV1 (confirmed; must be disclosed):**
Bencal SPV1 will record a large Y1 unrealised FVTPL loss: WCP shares purchased at $3,000,049.83 cost; Level 3 proxy book value at formation approx. $1,365,000 (founding-price proxy pre-listing); expected Y1 IS unrealised loss ≈ $1,635,050. This is structurally expected and anticipated, reverses from Y2 onward as WCP value appreciates. Disclose prominently in CIM risk factors and ensure all investors are specifically briefed before subscription.

---

### E — Open items matrix (what project-documents must track)

**BLOCKERS — do not draft these clauses until Jennifer resolves:**

| Flag | Open Question | Document Section |
|------|--------------|-----------------|
| Flag 6 | Work Fee recipient (Bencal Management Corp. directly vs. principals) | CIM §D dealer-fee disclosure; LP Agreement Annex A |
| Flag 13 | Reserve injection mechanism — Option A confirmed? (raises manager diluted % above 10%) | LP Agreement — unit table; SPV1 articles of incorporation |
| Flag 2 | Bencal SPV2-GP 1 share: nil vs. nominal | LP Agreement — GP capital account clause |

**DRAFT NOW, UPDATE LATER — use placeholder brackets where flag is open:**

| Flag | Open Question | Document Section |
|------|--------------|-----------------|
| Flag 8 | Management Services Agreement structure | LP Agreement Schedule A; CIM §3 management fees |
| Flag 10 | Transfer restrictions / manager-change event clause | LP Agreement §12–15 |
| Flag 12 | Reserve sizing: 3 years (current) vs. 4 years | Offering amounts, unit counts, opex tables in Section B |
| Flag 14 | Setup costs Y0 IS treatment — IAS 38.69(d) confirm | Policy note D-5 |
| Flag 4 | DTL option (a) vs. look-through at SPV1 | Policy note D-9 |
| Flag 11 | Review engagement vs. full audit confirm | Policy note D-10 |

**NOT RELEVANT to offering docs — park:**
Flags 3, 7, 9 (minor IS/FOFI scope items), S1–S4 (snapshot technology), D7 flags (separate JV vehicle), P1–P4 (Reporting Issuer future phase), Phase A engine implementation items.

---

### F — Artifacts

| Artifact | Path in project-proforma archive | Commit |
|---------|----------------------------------|--------|
| SPV Operating Budget (Markdown) | `outputs/spv-operating-budget.md` | 4617716 |
| SPV Operating Budget (HTML — print-ready, line-numbered) | `outputs/spv-operating-budget.html` | 4617716 |
| Source BRIEF (full spec, §5d/5e/5f) | `briefs/BRIEF-tool-proforma-leapfrog-2030.md` | 85d2d2e (v0.15.6) |

Request project-documents to confirm receipt and identify any further data needed to begin drafts. Flag 6 resolution expected next session — will send update immediately when confirmed.

---
from: command@claude-code
to: totebox@project-proforma
re: Stage 6 status — archive promoted (workspace); monorepo 2-commit promotion blocked
created: 2026-05-28T04:32:00Z
priority: high
status: actioned
actioned: 2026-06-05T16:10:00Z
actioned_by: totebox@project-proforma
actioned_note: Cherry-pick attempted per instructions (see NEXT.md §Stage 6 readiness checkpoint 2026-06-02). Aborted on modify/delete conflicts in ambassadors_d1.rs, bencal.rs, and content conflict in d3_wcp.rs. Working tree restored. Cluster-branch conflict tracked in NEXT.md pending operator decision. Message actioned — outcome documented.
msg-id: command-20260528-project-proforma-stage6-status
---

**Archive git (11 commits):** Already on canonical foundry.git — all workspace commits are current.
No action needed on archive.

**Monorepo sub-clone (2 commits: `017a8f2d` + `05b0cce6`):** Blocked.

Root cause: `cluster/project-proforma` has 0 commits ahead of canonical (nothing to promote).
Both `017a8f2d` (fix(spv-bencal)) and `05b0cce6` (feat(d3-wcp)) are on the `main` branch,
which is 1,001 commits BEHIND origin/main (canonical). Cannot promote directly.

**What needs to happen:**
1. In the project-proforma monorepo sub-clone, switch to `cluster/project-proforma`
2. Cherry-pick `05b0cce6` and then `017a8f2d` onto `cluster/project-proforma`
3. Resolve any conflicts (app-workplace-proforma d3 wcp + spv-bencal changes)
4. Run `cargo test` to verify
5. Notify Command Session via outbox when cluster branch is ready
6. Command will run `bin/promote.sh`

