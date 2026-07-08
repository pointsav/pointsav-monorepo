---
schema: foundry-session-context-v1
archive: project-bim
---

# Session context — project-bim

Rolling 3-session summary. Newest entry first. Keep only 3; push oldest to session-context-archive.md.

---

## 2026-07-08 | totebox@claude-code | live-site verification (browser-in-the-loop) + mailbox triage

**Done:**
- **Startup**: role confirmed (Totebox, `cluster/project-bim`, correct branch), session brief pulled via
  MCP. Processed 2 pending inbox messages that were already resolved by prior-session work but never
  flipped to `actioned`: Command's v2-redesign-live ACK (mailbox-cleanup summary — all 3 sub-items
  independently confirmed already resolved). Updated [[project-bim-v2-redesign-status]] memory to drop
  "pending Command" language since the ACK confirmed the push actually landed.
- **Operator asked for real browser-in-the-loop verification** that Command pushed the correct v2 build to
  `bim.woodfinegroup.com` (not just trusting Command's report) — see [[feedback-verify-visually-in-browser]].
  Used Python Playwright (already installed; no MCP browser tool available) to load the live URL headless,
  captured a full-page screenshot + full body-text dump. Confirmed: HTTP 200, title "Woodfine BIM Library",
  Objects/Compositions taxonomy framing, 3-column restructured footer, "MCorp™" trademark line, real CC
  BY-ND 4.0 badge, collapsible Important Information band. **Didn't stop at "text matches"** — traced the
  exact rendered footer/trademark text back to `pointsav-monorepo/app-privategit-bim/src/render/shell.rs`
  in the local clone and ran `git log` on that file: last commit touching it is `3461856d`, the exact SHA
  Command cited as the cherry-pick tail — proof this is genuinely that commit's output, not a stale/partial
  deploy (the failure mode that hit project-marketing the same day per Command's carry-forward notes).
- Sent Command a detailed mailbox confirmation (`command-20260708-verified-bim-woodfinegroup-com-is-the-co`)
  with full method + findings, plus a side-flag on the public "AGPL-3.0-or-later" footer license claim.
  Command replied same session: independently re-verified with the same result, and separately found +
  fixed a missing `--delete` flag on shared `push-to-prod.sh`'s vault rsync (discovered auditing
  project-design; preemptively applied the same fix to `target_bim` even though it hadn't visibly bitten
  BIM yet). Marked that ACK actioned too — inbox is now fully clear, nothing pending.
- **NEXT.md**: struck 2 completed Hot items (Command's canonical merge + push-to-prod, and the canonical
  TRADEMARK.md amendment) — both confirmed done this session, replaced with resolution detail.
- **BRIEF-app-privategit-bim.md** + **cleanup-log.md**: appended closing entries recording the live
  confirmation and the shared push-to-prod.sh fix, so the v2 redesign's "pending Command" status doesn't
  linger stale in the durable record.

**Pending / carry-forward:**
- CC BY-ND 4.0 content licensing still not counsel-confirmed (unchanged from last session) — see
  [[project-bim-v2-redesign-status]].
- All other carry-forward items unchanged from 2026-07-06/08 entry below (JOURNAL /research render-contract
  deferral, search indexing gap, corporate wiki CSS 404s, 18 modified IFC files, Cargo.lock staleness,
  woodfine-bim-library staging-mirror gap, `ee089b2` admin-tier push, NOTAM permission — not re-verified
  this session, no reason to believe changed).

**Operator preferences surfaced:**
- Confirms the standing rule from [[feedback-verify-visually-in-browser]]: even when the other party
  (Command) reports a verified success, a specific operator ask to "check ... browser in the loop" means
  do the real independent check yourself, not just relay their report.

---

## 2026-07-06/08 | totebox@claude-code | v2 redesign (Objects/Compositions catalog) + footer/legal polish (largest session on this archive to date)

**Done:**
- **Outstanding-issues cleanup + BRIEF consolidation** (session start): verified the 2026-07-05
  "hard reset" on `pointsav-monorepo` lost nothing (3 dangling commits' content was already on
  `origin/main` under different SHAs — a prior reconciliation, not data loss); archived 10 resolved
  inbox messages; escalated 4 outbox messages stuck at attempts:25 to Command (root cause turned
  out to be a `mailbox-relay.sh` bug, fixed by Command later this session). Consolidated 3
  overlapping active BRIEFs to 2 active + 1 superseded.
- **Corrected a real product-framing bug**: research (NBS/buildingSMART/Uniclass 2015/Revit
  precedent) confirmed a Key Plan is a *Composition* assembled from BIM Objects, not a BIM Object
  itself — fixed the actual conflation in `plan-bim-objects.md`. Redefined `app-orchestration-bim`
  as a future BIM Editor/Viewer (kept its brief active, un-superseded it) — distinct from
  `app-privategit-bim` (the CMS) and `tool-keyplan` (the Composition compiler).
- **Denver Airport / Woodfine simulation exercise** (operator-directed, explicitly a Denver-content-
  free structural pass-through) produced a clean-sheet Woodfine v2 design candidate — see
  [[project-bim-v2-redesign-status]].
- **Built the v2 design as real Rust integration**, not a static push (operator: "go 100%... we are
  looking for something radically different" — see [[feedback-radical-redesign-when-asked]]): new
  unified Objects/Compositions catalog home page wired to real `woodfine-bim-library` data (7
  Steelcase furniture Objects, 23 real Key Plan Compositions — confirmed 23 is correct, not 24, by
  cross-checking the actual DTCG source file), full-site visual rebuild, `/key-plans`+`/furniture`
  redirects. Real Uniclass 2015 Pr codes added to all 7 furniture Objects (verified against the
  actual NBS Pr table), real `furniture_refs` generated for PO-1 via an actual `tool-keyplan` compile.
- **Follow-up polish, five rounds** — branding rename to "Woodfine BIM Library" (product-name level
  only), footer badge reposition + sourced-language pass (Taxonomy/Anatomy/Syntax, Use Case, Data
  Box — cited to real architect source documents), footer 3-column restructure, a real SPA-nav
  silent-failure bug fix (site-wide, not just the one link that surfaced it) + LICENSE/network link
  corrections, the Important Information band redone against Command's actual counsel-approved
  spec (found the real reference pattern already shipped on `project-knowledge`), a real CC BY-ND
  badge, and a trademark rename (Woodfine Management Corp → MCorp, explicit operator instruction
  after I flagged it wasn't previously an established mark). Full detail: [[project-bim-v2-redesign-status]].
- **Two real bugs found via browser-in-the-loop review that pure code-reading missed** — see
  [[feedback-verify-visually-in-browser]]. Both caught by personally reviewing screenshots/PDF
  output rather than trusting a subagent's "looks fine" summary.
- Self-service Stage 6 lite run twice (both times: personal staging mirrors rejected as stale/non-
  fast-forward — a known, already-diagnosed condition, not force-pushed through; the promote-queue
  entry + Command notification are the durable record). Final queued HEAD `3461856d`. **Not pushed
  to foundry-prod** — needs Command's canonical merge + `push-to-prod.sh`, pending at session end.

**Pending / carry-forward:**
- Command's canonical merge + `push-to-prod.sh bim` — operator has explicitly requested this go
  live; queued, not yet processed as of session end.
- `TRADEMARK.md`'s canonical source (`vendor/factory-release-engineering/policies/TRADEMARK.md`)
  still needs the MCorp amendment — only the downstream `pointsav-monorepo` copy was updated (admin-
  tier out of Totebox reach). See [[reference-trademark-canonical-source]]. Flagged to Command.
- CC BY-ND 4.0 content licensing is implemented but **not counsel-confirmed** — flagged directly to
  the operator, not silently asserted as settled. See [[project-bim-v2-redesign-status]].
- JOURNAL /research render-contract cross-surface work (`SPEC-journal-wiki-render-contract.md`
  §9/§10) consciously deferred, not built.
- Still open from prior sessions, untouched this session: search's property-set/compliance-text
  indexing gap, corporate wiki (port 9095) CSS 404s, the 18 modified IFC files in
  `woodfine-bim-library/key-plans/` (operator: leave for a separate content-work pass).

**Operator preferences surfaced:**
- See [[feedback-verify-visually-in-browser]] and [[feedback-radical-redesign-when-asked]] for the
  two strongest, most reusable signals from this session.
- Wants real fixes surfaced even when they contradict an already-completed plan step (e.g. asked
  "is this correct??" about CC BY-ND and got a direct "not actually confirmed" answer, not
  reassurance) — don't paper over an uncertain legal/compliance claim to make a task look done.
- Comfortable overriding a flagged risk once informed (the MCorp trademark rename) — the right move
  was surfacing the concern clearly once, then proceeding on explicit confirmation, not repeatedly
  re-litigating after the operator has already decided.

---

## 2026-07-03/04 | totebox@claude-code | bim.woodfinegroup.com — two full rebuild passes, real-data bug fix, family-continuity reversal

**Done:**
- **Live-site check + promote-truncation correction**: operator asked "is the new website up live?" —
  found it was serving the OLD wiki-shell, root-caused to a `tail -40` truncation bug on Command's side
  during `promote.sh` (saw 18 of the real 31 queued commits, silently dropping the newest 11 including
  the CMS-reposition tip). Escalated with exact evidence; Command re-diagnosed, corrected, redeployed —
  confirmed genuinely live this time.
- **Fresh full-site audit (2 parallel agents) found real bugs beyond the operator's named complaints**:
  a CSS layout bug (`.bim-shell`'s forced `flex:1`/`min-height:100vh` left ~1500px dead space before the
  footer), a missing `<h1>` on the homepage, doubled/garbled chip text ("IFC IfcSpatialElement" →
  "IFC IFCSPATIALELEMENT" under an uppercase transform), redundant breadcrumb+back-link, and confirmed
  fonts still weren't actually loading (system fallback) — the single biggest driver of "looks generic."
- **Operator rejected the Envelope-as-Navigation hero's core concept**, not just execution: "the three 3d
  boxes do not quite make any real sense... we don't have the massing for real BIM Objects to allow us
  to be more playful." Root cause worked out together: the diagram represented claim #41 (City Code as
  Composable Geometry), an explicit v0.0.2+ roadmap idea per the manifest's own scope section — not what
  the catalog does today.
- **Deep real-source research (multiple rounds, following the operator's own pointers into `inputs/`'s
  dated "Collaborators" folders and `cluster-totebox-jennifer`)** found: those "Collaborators" folders
  are internal Woodfine-family mail-merge email threads, not external architects; a real, hand-drafted
  CAD sheet for PO-1 ("Private Office — Small," `inputs/Sketches/DISCOVERY_MCorp_Sketches_Key
  Plans_Private Office.pdf`) with real dimensions (19'-8"/5.9944m depth × 13'-5" width, 325 SF, no Zone
  3); and — the most consequential find — that `key-plans.dtcg.json`'s private-office entries, despite
  being marked `status: "confirmed"`, had actually inherited the *Professional Office* use-type's zone
  depths by mistake. **Fixed the real data bug** (corrected to 5.9944/1.3716m across PO-1/2/3, dropped an
  unverifiable "Steelcase Leap" brand claim with no source anywhere).
- **Checked real terminology before inventing any** — verified "Bundle" doesn't appear anywhere in the
  actual methodology PDFs (it's a software/design-system import) before using it; the product's own real,
  established term is "Key Plan," and the real IFC relationship is `IfcRelContainedInSpatialStructure`.
- **Real, carefully-scoped positioning content added** (not overclaimed): a real, sourced Denver
  International Airport reference (17M SF, 93 buildings, proprietary Revit/CDE — used as a contrast
  point for "open standard, self-hostable," not a claim Denver would have chosen this), and a tie to
  PointSav's own already-published positioning pillars (home.pointsav.com's real icon-strip: "Business
  Administration, Record Keeping, Building Connectivity") rather than positioning BIM as a standalone
  competitor to Autodesk-style tools.
- **Legal disclosure rewritten by a dedicated Opus-model pass** (operator: "write it as if they were a
  hyperscaler lawyer at a big law firm") — `disclaimers.md` replaced with tightened, precise text;
  confirmed "Sovereign Data Foundation" isn't a real initiative and removed it entirely, not replaced
  with anything else; `important-information.md` deleted as confirmed-dead (footer already inlines
  `disclaimers.md` directly — **reversed 2026-07-07/08**, see current entry above: the file was
  re-created against Command's actual counsel-approved spec, which requires a dedicated short band
  distinct from the full disclaimers content). **Flagged, not silently resolved**: the CC BY-ND 4.0
  editorial-license claim has no backing LICENSE file anywhere in either repo.
- **Reversed the RD.7 Spectrum-chrome color/font direction for family continuity** (explicit operator
  call, not a default): `--bim-accent` changed from drafting-blue `#1A4480` to `#164679`, copied directly
  from `home.woodfinegroup.com`'s live tokens (not approximated); real self-hosted Inter/Source Serif
  4/Source Code Pro files copied byte-for-byte from `app-mediakit-marketing-2` (the exact files
  `home.woodfinegroup.com` currently serves) — finally real fonts, not another silent fallback.
  Differentiation now comes from the real-object content grounding, not a deliberately different palette.
- **Two full implementation passes, both verified (cargo build + clippy clean, local deploy, screenshots,
  hotspot click-tests) and committed**: pass 1 (envelope-as-navigation, later superseded) at
  `pointsav-monorepo` `dbb74ff8`; pass 2 (real "Anatomy of a Key Plan — PO-1" hero, header/footer redo,
  color/font continuity) at `pointsav-monorepo` `0d72def7`, `woodfine-bim-library` `ee089b2` (the
  zone-depth fix + legal text + home.md content), archive-root BRIEF commits after each pass.
- **`woodfine-bim-library`'s remote misconfiguration (flagged pass 1) was fixed by Command by pass 2**
  — now correctly points at `woodfine/woodfine-bim-library` — but it has no staging-mirror remotes at
  all, so it still needs Command's admin-tier push each time; flagged as a recurring bottleneck worth a
  proper staging-mirror setup, not just a one-off unblock.

**Pending / carry-forward:**
- Both passes' commits are staged/queued for Command (canonical merge + `push-to-prod.sh bim` for
  `pointsav-monorepo`; admin-tier push for `woodfine-bim-library`) — not yet live on foundry-prod as of
  this session's end.
- CC BY-ND 4.0 license-file gap — needs an operator decision before the new disclosure text is fully
  settled.
- `building-width-calculator.dtcg.json` (a third token file) still has its own, still-unreconciled set
  of Private Office zone-depth numbers, different again from both the old wrong "confirmed" value and
  the real CAD-sourced one now in `key-plans.dtcg.json` — same bug family, not yet fixed there.
- "Swiss air requirement" (operator-mentioned) — searched, not found in any text-based source this
  session; may be in a PDF not yet read, or may not exist. Don't fabricate it.
- Deferred design work (unchanged from pass 1, still logged in the BRIEF): drafting-sheet layout system,
  GUID-as-visual-mark, live constraint-composition tool, and the 3D-viewport decision gate (real
  AGPL-xeokit vs. MIT/MPL-@thatopen licensing tradeoff, needs explicit operator sign-off before any
  engineering starts — already flagged as an open question elsewhere in the project).

**Operator preferences surfaced:**
- **"No hype, must be real" is an operating standard, not a one-time note** — applies to visual concepts
  (rejected an invented diagram in favor of a real, already-shipped catalog entry), specific factual
  claims (rejected an unverifiable brand/chair claim), and terminology (checked "Bundle" against real
  sources before using it). When in doubt, go find the real primary source rather than write plausible
  copy — this operator will ask "how does this relate back to X" if a concept feels invented, and that
  question is a genuine signal to reconsider the concept, not just execute better.
- **Expects the agent to actually dig into real files the operator points at** (`inputs/`'s dated folders,
  `cluster-totebox-jennifer`), including reading PDFs/binary-ish sources directly, not just theorizing
  from summaries — and to keep digging across multiple redirects in the same session without losing
  the thread.
- **Comfortable reversing an earlier research-backed decision** (RD.7's deliberate color differentiation)
  when a more important goal (family continuity) emerges — don't over-anchor on a previous session's
  "confirmed" research conclusion if the operator gives new, explicit direction.
- **Values a dedicated high-effort pass for register-sensitive text** (asked for legal disclosure text
  to be written "as if they were a hyperscaler lawyer at a big law firm" — a specific, deliberate framing
  worth reusing for future legal/compliance copy needs on this archive).
- Sensitive personal/corporate archives (`cluster-totebox-jennifer`) contain real investor-relations and
  corporate-response material alongside useful general research — correctly scoped research agents to
  read the general/public material (industry reports, methodology PDFs) while explicitly not opening or
  summarizing anything that looked like confidential business content; this distinction mattered and
  should carry forward to any future research into personal Totebox directories.

