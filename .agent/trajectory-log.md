# project-bookkeeping — trajectory log

This file is the cluster's session-trajectory write log per
DOCTRINE.md §XV and `conventions/trajectory-substrate.md`.
Every Task session appends a structured entry summarising what it
did, what it changed, and what surprised it.

`bin/capture-trajectory.sh` consumes this log to emit Stage-0
trajectory tuples into the apprenticeship corpus.

---
session: 2026-05-05 — Task Claude planning session
state: plan-complete; no code written
operator: jennifer
---

**What happened:** Full planning session for the PCLP 1 (Woodfine Professional Centres LP) bookkeeping engine. Session was entirely in Plan Mode — no files created, no code written. Jennifer served as domain expert, correcting the agent on several material financial modelling points.

**Verified proforma data:**
- PCLP 1: 20 annual periods (E=Y1..X=Y20); three construction phases (3+2+2 years); peak debt $987.2M at Y7
- Y7 DSCR = 1.39× (tightest; floor 1.20×); Y8 ending cash = $1.63M (distributions of $66.9M are paid out, not held)
- Debt headroom at Y8 = $1.08B at 1.20× floor; compound reinvestment possible via Phase 4 at Y9–Y11
- TitleCo X calibrated rent $41.06/sqft; TitleCo Y calibrated rent $35.22/sqft (both → 10.5% devYield)

**Key corrections made by Jennifer:**
1. Period structure: 20 annual periods, not quarterly — cascading error caught and fixed
2. Y8 cash: $66.9M = distributions paid (not available cash); actual ending cash $1.63M
3. JV comparison: must use same $250M equity and 5% rate — only structural difference (debt priority)

**Plan file:** `/home/jennifer/.claude/plans/1-can-you-read-sharded-owl.md` (18 sections; complete rebuild reference)
**Sandbox copy:** `/home/jennifer/sandbox/PCLP1-MASTER-SUMMARY.md`

**What surprised me:** Jennifer's financial model fluency is well ahead of a typical operator — she caught errors in DSCR, cash flow, and period structure that would have produced a materially wrong engine. The apprentice-with-domain-expert working pattern declared in `manifest.md` is accurate and load-bearing.

**Left pending:** All implementation. Next session approves plan and executes Step 0.

---
session: 2026-05-07 — Sprints 2–4 + draft-batch routing
state: complete; all four sprints shipped; crate production-ready
operator: jennifer
commits: 34e70d7 (Sprint 2 fix), 1988de9 (Sprint 2 base), [sprint3 commit], 1c176a3 (Sprint 4), 505ca26 (NEXT/registry), 0318dad (routing outbox)
---

**What happened:** Resumed mid-Sprint-2 (context carried from prior session). Completed
Sprint 2 (adj_nav_pu fix, 23 tests), then executed Sprints 3 and 4 in sequence.
Sprint 3: minijinja HTML report — 4 IFRS statements × 10 years, Notes 1–12, Bear/Base/Bull
scenarios, 5×5 NAV/DSCR sensitivity matrices, JV tear sheet, OSC 51-721 disclaimer.
Sprint 4: EY audit package — formula registry (18 formulas, SHA-256 fingerprints), full
audit trail JSON (127 entries per base-case run), reconciliation report (20/20 checks pass).
Session ended with cross-cluster draft-batch routing: 23 TOPIC/GUIDE files → project-editorial,
28 DESIGN files → project-design; BIM licensing defects and leapfrog-2030 body flagged for Master.

**Final state:** 36/36 tests pass. Four files in /home/jennifer/sandbox/outputs/.
Outstanding known gap: WC reserve formula (D34 × EBITDA) not modelled; Y8 ending
cash $27–28M vs. proforma $1.63M. Flagged for EY.

**What surprised me:** The SHA-256 formula registry design is sound for EY use —
each entry's hash is independently recomputable from the plaintext name/source/math
triple, making formula substitution detectable without inspecting the Rust source.
The minijinja template matrix loop required switching from `range(end=N)` (not
supported as a range arg in minijinja 2.x) to `{% for row in cells %}` +
`loop.index0` indexing — a non-obvious engine limitation.

**Left pending:** WC reserve formula modelling; Phase 4/5 reinvestment scenarios;
cluster Tetrad legs (customer guide, deployment instance, wiki TOPICs) all `leg-pending`.

---
