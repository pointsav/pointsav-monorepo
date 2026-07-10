---
schema: foundry-cleanup-log-v1
archive: project-bim
---

# Cleanup log — project-bim

Decisions, deferred items, and architectural notes that belong in
the record but are not NEXT.md action items.

---

## 2026-07-10 — 2 of 3 BIM research essays enriched from internal design-response deck; seeding request sent to project-editorial

**Trigger:** operator asked whether `CONSTRUCTION_2025_10_31_Design Slides_Openstudio_Woodfine Response.docx`
(confirmed byte-identical to the already-fully-read "copy 2" version used in the v3 redesign workflow)
contained material to strengthen the 3 site research essays, ahead of proposing them as JOURNAL seeds to
project-editorial.

**Findings applied:**
- `bim-design-philosophy.md` — added a "Why build, not buy" section quoting Woodfine's own internal
  rationale (vendor heterogeneity, cost model, control argument) and a sourced "100+ years" building-lifespan
  figure alongside the existing unsourced "50+ years" (not silently overwritten — both stated).
- `flat-file-vs-cloud-bim.md` — added the actual specified deployment topology (each Woodfine Building on
  its own independent VM, not a shared platform) and a sourced cost-model quote under Pricing models.
- `aec-muscle-memory-rationale.md` — **no changes.** Exhaustive grep across the full 9,674-line source doc
  found zero material about AEC software interface conventions or muscle memory; the two "Revit" string
  matches were false positives ("revitalize"). Confirmed nothing to add rather than stretching a tenuous
  match.
- No competitor names (Autodesk/Bentley/Trimble/Planon/Tririga/Archibus/iTwin/Tandem/Nemetschek/dTwin) exist
  anywhere in the source document — the existing citations in `flat-file-vs-cloud-bim.md` are not
  corroborated or extended by this source; left as-is.

**Side finding, NOT folded into the essay work:** a separate research thread (Opus + Fable browser-in-loop
audit of the PointSav Design System v3 mockup + bim.woodfinegroup.com, re: "BIM Objects as a product other
companies adopt for their own library") surfaced a real license conflict — the live footer shows both
"Object data licensed Apache-2.0" and a "LICENSED CC BY-ND 4.0" badge, and CC BY-ND forbids the derivative
use this positioning idea requires. See `BRIEF-bim-server-external-positioning.md` for full detail — this
needs explicit operator re-scoping, not a default inherited from the 2026-07-09 CC BY-ND confirmation
(which was made in a different context and not explicitly tested against this use case).

---

## 2026-05-16 — P8c render.rs-only decision

**Artifact:** `design-component-bim-regulation-rs1.md`
**Decision:** Operator confirmed `render.rs`-only for the regulation overlay component.
`recipe.html` template approach deferred until the user-facing surface ships and the
rendering approach can be validated against real data.
**Source:** inbox message `project-bim-20260516-p8c-regulation-component`
(command@claude-code, 2026-05-16).
**Action taken:** Decision relayed to project-design via outbox (they are holding
the draft pending this answer).

---

## 2026-07-08 — v2 redesign confirmed live; shared push-to-prod.sh `--delete` gap fixed by Command

**Verification:** operator asked for a browser-in-the-loop check that Command pushed the correct v2
redesign build to `bim.woodfinegroup.com`. Ran headless Playwright/Chromium against the live URL,
screenshotted, and traced the rendered footer/trademark text back to
`pointsav-monorepo/app-privategit-bim/src/render/shell.rs` in the local clone — `git log` on that file
confirmed HEAD `3461856d` (the exact SHA Command cited as the cherry-pick tail). Sent a detailed
confirmation to Command (`command-20260708-verified-bim-woodfinegroup-com-is-the-co`); Command
independently re-verified the same result.
**Side finding (not project-bim's action item):** while auditing the shared `push-to-prod.sh` for
project-design, Command found `target_design`'s vault/templates/static rsync calls were missing
`--delete` — deleted source content never got removed from foundry-prod's disk. Fixed, and
preemptively added `--delete` to `target_bim`'s vault sync too (same root-cause gap, hadn't visibly
manifested for bim yet). Verified via dry-run; `bim.woodfinegroup.com` unaffected either way. No
action needed here — noting for the record in case a future session sees foundry-prod vault content
actually get pruned on next push where it previously wouldn't have.

---

## 2026-07-09 — plan-completeness audit: NEXT.md resolved items closed, 5 operator decisions resolved via interrogation

**Trigger:** operator asked whether "the plan" for project-bim was 100% finished and wanted
BRIEFs consolidated + NEXT.md brought to a clean state before moving on. Ran a one-by-one
AskUserQuestion interrogation (with recommendations) rather than guessing at any of these.

**Resolved items struck from NEXT.md's Hot section (already `[x]`, moved here for the historical
record — NEXT.md now holds open work only):**
- v2 redesign canonical merge + `push-to-prod.sh bim` — confirmed live 2026-07-08 (see the
  2026-07-08 entry above for full detail).
- Canonical `TRADEMARK.md` "Woodfine Management Corp" → "MCorp" amendment — closed 2026-07-08,
  admin-tier commit `062b29e`.
- 5 excluded commits from the 2026-07-05 branch reset — verified 2026-07-06 as already
  reconciled on canonical under different SHAs; no data loss, no action needed.

**5 real-world data decisions (the "Opus army synthesis" backlog, open since 2026-05-17)
— operator resolved directly, one at a time:**
1. **Academic Small area** — 105 m² (V3 Master Summary) confirmed authoritative (this was
   actually already settled by project-editorial's same-day sweep reply, independently of this
   interrogation). On checking `woodfine-bim-library/tokens/bim/professional-office-subtypes.dtcg.json`
   directly, the fix was **already applied on 2026-07-03** (105.0 m² / 1,131 SF, with a full
   reconciliation note citing the source conflict) — NEXT.md's "token file needs update" framing was
   itself stale. No edit needed; NEXT.md item closed as already-resolved, not newly fixed.
2. **Civic zone depths** — no real source exists; operator confirmed to leave synthesised/flagged
   rather than fabricate a number. Stays open in NEXT.md pending a real field-research pass.
3. **Professional Office Z2/Z3** — no real figures available; stays TBD in NEXT.md.
4. **Business Building Width** — operator chose option C/C (27.27 m, balanced) over the
   previously-live A/A (32.29 m, widest). Applied this session; local preview rebuilt and
   redeployed to `local-bim.service` (127.0.0.1:9096) for operator review — **not pushed to
   foundry-prod**, that step is Command's per the normal deploy model.
5. **End-cap tile sizing** — operator confirmed the token file's 2,700 SF is correct; the V12
   Methodology diagrams (3,500–5,500 SF) are illustrative, not authoritative. No data change;
   discrepancy note closed.

**Other decisions from the same interrogation:**
- **CC BY-ND 4.0 licensing** — operator confirmed directly as the correct license choice; this is
  an operator business call, not a formal counsel sign-off. Closes the item that had recurred
  unresolved for 3 sessions (first surfaced 2026-07-04).
- **3 untracked `inputs/--- <date> -- Collaborators #NN ---/` folders** (appeared ~2026-05-20,
  flagged as unclassified at this session's startup per the AGENT.md business-admin caution rule
  — project-bim is not on the confirmed business-admin archive list) — operator confirmed these
  are real BIM collaborator/project correspondence, in scope for this archive. `git add`ed and
  committed this session.
- **`cluster-totebox-property-1` deployment leg** (per-property IFC archive; pre-created
  2026-04-28, zero progress since) — operator confirmed this is deprioritized for now, not
  abandoned. Noted explicitly in `.agent/manifest.md`'s tetrad block rather than left as a silent
  stalled "leg-pending".
- **BRIEF-app-orchestration-bim.md** (BIM Editor/Viewer, forward-scoped 2026-07-06, software not
  started) — operator confirmed this stays active as correctly sequenced; no status change.

**Also this session:** reclaimed a misfiled brief (`BRIEF-bim-website-pipeline.md`, was sitting in
project-editorial's briefs dir) into this archive's own briefs dir with `status: superseded`; see
`.agent/briefs/README.md`'s Superseded section. Also struck NEXT.md's "Artifact dispatch status"
section (all 5 rows already `[x]`) — today's inbox message
(`command-20260709-editorial-sweep-complete-26-draft-batch-`) independently confirms the full
2026-05-17 TOPIC/GUIDE/DESIGN dispatch batch is published and swept; nothing left to track there.

---

## 2026-05-16 — building-width-calculator.dtcg.json migrated to woodfine-bim-library

**Source:** `pointsav-design-system/tokens/bim/building-width-calculator.dtcg.json`
**Destination:** `woodfine-bim-library/tokens/bim/building-width-calculator.dtcg.json`
**Commit:** 443a231 (pwoodfine, cluster/project-bim)
**Note:** All 10 BIM DTCG files are now in woodfine-bim-library (repo renamed from
woodfine-design-bim per command-20260517-bim-rename-complete, 2026-05-17). The copies
in `pointsav-design-system/tokens/bim/` are pending admin-tier removal by Command
(mcorp-administrator identity). Stage 6 push to woodfine-bim-library origin pending.
