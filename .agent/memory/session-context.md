---
schema: foundry-session-context-v1
archive: project-bim
---

# Session context — project-bim

Rolling 3-session summary. Newest entry first. Keep only 3; push oldest to session-context-archive.md.

---

## 2026-07-09/13 | totebox@claude-code | Round 10 voice/diagram polish + Rounds 11-13 Spanish translation, LIVE on foundry-prod, real static-asset sync bug found post-push

**Done:**
- **Round 10** (voice/diagram work, largely predates this session's visible context window):
  sitewide third-person→first-person voice rewrite; Method-page diagram redesign (containment model
  → nested/concentric frames; cross-section craft pass); real dark-mode contrast fix
  (`--bim-pen-primary`/`--bim-pen-secondary`); real motion-animation bug fix (staggered
  `transition-delay` getting cancelled by style recalcs — switched to JS `setTimeout`). Follow-ups:
  diagram text legibility + a caught-and-fixed `letter-spacing` clipping regression; mobile header
  right-justification; renamed a cross-section label "Interior"→"CENTRELINE" after a multi-round
  grilling landed on the real underlying concept (mirror-symmetry geometry).
- **Rounds 11-13 — Spanish (`/es/*`) translation, now live on foundry-prod.** Operator: "+50% of our
  audience in Mexico." Reference implementation (`app-mediakit-marketing-2`) traced with real
  file:line citations before any code. Real Mexican-Spanish AEC/BIM terminology glossary researched
  via a dedicated Opus agent before translating anything (Key Plan/Tile/Magazine kept English —
  Mexican AEC practice borrows English BIM vocabulary directly; Habitat→Hábitat, Corridor→Corredor,
  Floor Plate→Placa de Piso, Object→Objeto).
  - Round 11 (plan-mode scoped, Tier 1): home/method/disclaimers/24 category ledes/UI chrome.
    Operator's own explicit decision: "I draft it, flagged for verification" — not routed to
    project-editorial, applied uniformly including the legal disclaimers content.
  - Round 12: operator broadened scope — "keep translating everything except the Research essays and
    the Key Plans pages themselves." Objects/Search pages + full DTCG entity-description dataset (221
    entities across 22 files, `key-plans.dtcg.json`/`amenity-key-plan.dtcg.json` excluded). One real
    boundary case caught on review and reverted (a Key-Plan-specific subtree nested inside an
    otherwise in-scope file). One real bug caught by browser click-through, not curl: object cards
    silently linked to the English detail page.
  - Round 13: operator clarified further — Key Plans/Research nav should read in Spanish even though
    their *content* stays English. A browser-in-the-loop check (operator-requested) confirmed clicking
    those nav links from `/es` was a genuine dead end — landed on all-English pages with the language
    switch entirely absent, no way back to Spanish. Fixed with chrome-only-translated routes. Also
    caught a second bug while there: nav active-page highlighting had been silently broken on every
    Spanish page since Round 11 (path-prefix matching never accounted for `/es/`).
  - **Deployed to foundry-prod 2026-07-13** via the normal Command handoff (this archive doesn't push
    to foundry-prod directly — see [[project-bim-deploy-model]]). Command ran `push-to-prod.sh bim`,
    confirmed 200s. **Same-day, operator asked to verify visually** ("check the live site... browser
    in the loop") — found the "LIVE, health check passed" report was misleading: a real, severe
    static-asset sync gap on foundry-prod (`bim-planroom.css`, the core stylesheet, 404s entirely;
    3 other assets stale/undersized) that predates this session's own work and wasn't caught because
    `/healthz` never checks static assets. Escalated to Command high-priority with exact diagnostics
    (byte-size diffs, 404 confirmation, Playwright screenshots) — **not yet confirmed fixed as of
    session end.**
- Full detail for all of the above: `.agent/briefs/BRIEF-app-privategit-bim.md` (updated this
  session, now ~1,200 lines — consider whether it needs splitting if it keeps growing at this rate).

**Pending / carry-forward:**
- **foundry-prod static-asset sync gap** — awaiting Command's fix + re-verification. Highest-priority
  item for next session's startup: check Command's reply first.
- SEO gaps on bim.woodfinegroup.com — a real, ready-to-apply draft from project-editorial
  (`SEO-bim-woodfinegroup.draft.html`), zero of ~10 signals present, 3 open questions need real
  answers. Not started; deserves its own session.
- Spanish translation's remaining exclusions (Key Plan/Composition technical data, Research essay
  content, SVG diagram inner labels) are deliberate, not "not yet gotten to" — don't reopen without a
  new operator decision.
- Binary-ledger sha256 refresh (Command-owned, low priority) and two forward-looking cross-property
  proposals from Command (footer-structure standardization, legal-tokens runtime consumer) — optional,
  logged, no action needed unless BIM ships a public-facing footer redesign or migrates trademark text
  off hardcoded strings.
- Pre-existing, untouched this session: the 18 modified `.ifc` files in `woodfine-bim-library/
  key-plans/` (operator: leave for a separate content-work pass), Cargo.lock staleness, JOURNAL-merge
  decision still sitting `status: pending` in inbox.md (whether to hand off enriched research-essay
  text for Command to merge, or pre-merge it — genuinely still open, not actioned this session since
  it deserved real attention, not a shutdown-sweep rush).

**Operator preferences surfaced:**
- **"Browser in the loop" is a standing verification instruction, invoked repeatedly this session** —
  every round got a real Playwright click-through before being called done, and it caught two real
  bugs (object-card links, nav active-state) that curl/HTTP-status checks alone would have missed.
  When the operator asks to "check ... browser in the loop," that means launch a real check, not
  relay another party's report — reconfirms [[feedback-verify-visually-in-browser]].
- **Scope decisions arrive incrementally and get refined, not stated once and fixed** — "translate
  everything except Research/Key Plans" (Round 12) later got refined to "except the Research
  essays and the Key Plans pages *themselves*" (Round 13, meaning chrome CAN translate even where
  content can't) via a follow-up clarification, not a full restatement. Don't over-interpret an
  earlier scope boundary as more restrictive than the operator's latest wording actually says —
  ask or re-derive from the newest message if a boundary case is ambiguous (the
  `building-width-calculator.dtcg.json` Key-Plan-subtree case this session is the concrete example:
  correctly caught as still-excluded on inspection, not because the file was excluded, but because
  its *content* was).
- **Wants real production issues escalated immediately and precisely** — when the operator reported
  the live site "doesn't look like the preview," the right move was to drop the in-progress shutdown
  sweep, diagnose with real byte-level evidence before reporting anything, and hand Command a
  complete, actionable report (exact file, exact diff, exact ask) rather than a vague "something's
  wrong, please look."
- Confirmed again this session: operator is comfortable with multi-round scope negotiation on a single
  feature (3 rounds of "how much should translate") rather than wanting the full scope locked up front
  — each round's request built on the verified state of the previous one.

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

