---
artifact: brief
schema: foundry-brief-v1
brief-id: project-editorial-working-plan-2026-06-19
title: "Editorial Working Plan — 2026-06-19"
status: active
owner: project-editorial
created: 2026-06-19
updated: 2026-06-19
parent: BRIEF-topic-guide-adaptation-plan.md
---

> ## 🟢 STATUS: AUTO RUN IN PROGRESS — 2026-06-19
> AUTO run approved: [x]
> When approved, execute Tracks 1–4 in order. Commit per track (or per article where noted).
> Surface only real blockers. Report completion summary at end of run.

---

## Context

Successor to `BRIEF-topic-guide-adaptation-plan.md` (§16 quality programme — now reference).
Three work streams have converged: §16 carry-forwards, a new inbox language-pass queue
(9 system TOPICs + 4 JOURNALs), and audit content repairs (C2/M2/M5/M6/M7/M8/M9) routed
to project-editorial from the 12-agent §7 audit.

## AUTO execution rules

- Commit per track (or per article for Track 2 language pass).
- Never `git add .` or `-A`; always stage specific files.
- Use `~/Foundry/bin/commit-as-next.sh "<message>"` for all commits.
- Editorial stripping applies to all TOPIC bodies: no personal names (Jennifer/Peter/Mathew),
  no localhost endpoints, no SSH port specifics, no systemd paths, no Doctrine claim numbers.
- Stage 6 promotion and `bin/sync-local.sh` are Command scope — route via outbox only.
- Surface a blocker (stop and ask) only when: source data is missing, deletion intent is
  ambiguous, or a content decision requires operator input that cannot be inferred from
  existing doctrine/conventions.

---

## Track 1 — §16 carry-forwards

Close out the predecessor BRIEF. Three items remain open.

### 1a — Restore Full Rankings tables

Files: `media-knowledge-projects/topic-top-400-regional-markets-eu.md`
       `media-knowledge-projects/topic-top-400-regional-markets-na.md`

Tables (ranks 26–400) were replaced with placeholder text during the top-25 test pass
(committed 0a9f8bc, 2026-06-17) pending live rendering verification.

**Action:** Read the current files. If placeholder text is present, restore the full
ranking tables from the session context (EU: rank 1 = Chemnitz 18.0; NA: rank 1 = Plano 25.5
— full tables must be reconstructed from the corpus or found in the prior session transcript).
Commit to media-knowledge-projects.

**Blocker condition:** If ranking data is not recoverable from file history or the corpus,
surface as blocker — do not fabricate rankings.

### 1b — Tier-index ledes

Files: `media-knowledge-projects/topic-tier-index-europe.md`
       `media-knowledge-projects/topic-tier-index-north-america.md`

Both flagged as mode2 (throat-clearing) ledes in the §16 audit. Lede-rewrite to
consequence-first pattern, matching the six place-profile rewrites already applied.

**Action:** Read both files; apply lede-rewrite. Commit to media-knowledge-projects.

### 1c — GUIDE-mk-* unstaged deletions

7 files reported deleted from filesystem but not yet `git rm`'d:
- `GUIDE-mk-corporate-content-operations`
- `GUIDE-mk-corporate-deployment`
- `GUIDE-mk-documentation-content-operations`
- `GUIDE-mk-documentation-deployment`
- `GUIDE-mk-documentation-editorial-content-sweep`
- `GUIDE-mk-projects-content-operations`
- `GUIDE-mk-projects-deployment`

**Action:** Run `git status` in the project-editorial archive to confirm deletion state.
If confirmed deleted and no other session is using them, `git rm` and commit with message
noting this was previously flagged in §16 carry-forward.

**Blocker condition:** If files still exist on disk (prior session report was stale),
investigate before deleting.

---

## Track 2 — Inbox language pass

Process all four pending language-pass requests in order. For each:
1. Read the draft file(s).
2. Check: banned vocabulary, BCSC compliance, personal names stripped, bilingual
   completeness (EN+ES pair required for TOPIC; JOURNALs are internal — EN only).
3. Route: TOPIC → commit to appropriate `media-knowledge-*` sub-clone.
   JOURNAL-NOTES / JOURNAL → language-cleared ACK back to source via outbox.
4. Archive the inbox message after actioning.

### 2a — 9 TOPIC drafts from project-system

Source: `clones/project-system/.agent/drafts-outbound/`

Drafts (from inbox preview):
1. `TOPIC-crypto-license-sales-architecture.draft.md`
2. `TOPIC-dual-tier-extraction-architecture.draft.md`
3. (+ 7 more — read full inbox message for complete list)

Route: read each draft; determine target wiki (documentation / projects / corporate)
from content; apply language pass; commit to correct `media-knowledge-*` sub-clone.
ACK to project-system via outbox with routing summary.

### 2b — JOURNAL-NOTES J3 + J6 from project-workplace

Source: `clones/project-workplace/.agent/drafts-outbound/`
- `JOURNAL-NOTES-j3-20260602.draft.md`
- `JOURNAL-NOTES-j6-20260602.draft.md`

These are internal notes — language pass only (no wiki commit). ACK to Command
(who relayed) and note forbidden_terms_cleared status.

### 2c — JOURNAL J7 + J8 from project-gis

Source: `clones/project-gis/JOURNAL/`
- `JOURNAL-urban-fringe-v0.1.stub.md` (J7 — Urban Fringe v0.3)
- J8 — Commuter v0.3 (confirm filename from inbox message)

Language pass. ACK to project-data (who relayed; msg-id:
`project-data-20260618-language-pass-request-journal-j7-urban-f`).

---

## Track 3 — Audit content repairs

Repairs from the 12-agent §7 audit routed to project-editorial
(msg-id: `command-20260614-content-repairs-app-mediakit-knowledge-a`).
Ordered by priority.

### 3a — C2: Tier semantics reconciliation (HIGH — CBRE blocker)

`topic-co-location-ranking-system.md` defines T1 = lowest (★ "Anchor Only").
Every other article (tier-nomenclature, archetypes, regional-markets-system) defines T1 = highest.

**Action:** Rewrite `co-location-ranking-system` "Quality Tiers" section to align with
the site-wide convention (T1 = highest, T5 = lowest). Add `[[co-location-tier-nomenclature]]`
wikilink as the authoritative cross-reference. Commit to media-knowledge-projects.

### 3b — M8: OSM ODbL attribution (HIGH — legal obligation)

Methodology articles rest on OSM + Wikidata. ODbL requires attribution in any published work.
No published article names OpenStreetMap or its license.

**Action:** Add a "Data Sources" section to each methodology/index article that uses OSM data
(at minimum: `topic-co-location-methodology`, `topic-co-location-ranking-system`,
`topic-regional-markets-system`). Section text must name © OpenStreetMap contributors / ODbL.
Commit to media-knowledge-projects.

### 3c — M5: Guide catalog hatnote (LOW)

Guide catalog presents ~80 WFD GUIDEs as live links; all 404 (GUIDEs live in
woodfine-fleet-deployment, not the wiki).

**Action:** Add hatnote to the guide catalog article: "These guides are accessible to
Woodfine operators; they are not public wiki articles." Do not present unresolvable guide
slugs as live links. Commit to media-knowledge-documentation.

### 3d — M7: Snapshot dating (LOW per article)

Inconsistent cluster counts across articles (7,594 vs 6,493) — different snapshots
with no dating, reads as errors.

**Action:** Add "Data as of YYYY-MM-DD build" stamp to each article carrying cluster counts
or geographic coverage claims. Read existing articles to determine correct snapshot dates
from their content. Commit to media-knowledge-projects.

### 3e — M6: TOPIC/GUIDE drift (HIGH)

`topic-vertical-warehouse` contains transient operational content ("Data collection plan",
"Priority additions", "test results as of 2026-06-01"). `topic-co-location-ranking-system`
straddles TOPIC/GUIDE (algorithm steps, build-config dates).

**Action:** Strip transient operational/test-result sections from both articles. If the
stripped content is substantive, stage it to `.agent/drafts-outbound/` as a GUIDE draft
for Command routing. Commit cleaned TOPICs to media-knowledge-projects.

### 3f — M2: Onboarding chips (MEDIUM)

"New here? Start with these" → 4× 404 on projects + corporate home instances.

**Action:** Read the chip lists for both instances. Repoint each dead chip to the closest
extant article slug. If no suitable article exists, remove the chip rather than leave it
dead. Commit to media-knowledge-projects and media-knowledge-corporate as needed.

### 3g — M9: EN/ES parity sweep (HIGH — L4 release blocker)

`topic-co-location-ranking-system.es.md` is ~25% of EN length. L4 requires parallel bilingual
coverage; parity lag is a release blocker.

Also: `topic-co-location-ranking-system.es.md` received an out-of-scope new content section
("Niveles de Calidad y Distribución") in the §16 pass — decide whether to keep or revert.

**Action:** Parity sweep across all EN/ES pairs. Bring lagging .es.md files to full parallel
coverage. For the out-of-scope section: keep if factually correct and parallel to EN;
revert if it introduces unsourced content. Commit to media-knowledge-projects.

---

## Track 4 — Housekeeping

Execute at start of AUTO run (before Tracks 1–3).

| # | Action |
|---|---|
| 4a | Archive inbox messages 3 (ACK — Phase G GUIDEs committed) and 5 (relay already processed, commit 39588c97) |
| 4b | Mark `BRIEF-topic-guide-adaptation-plan.md` `status: reference` (update frontmatter only) |
| 4c | Fix `briefs/README.md` header — says "project-knowledge" (contamination residue); correct to "project-editorial" |
| 4d | Update `briefs/README.md` active-briefs table: add this BRIEF, retire the §16 plan |
| 4e | Outbox sweep: confirm which of the 14 pending messages have been actioned by Command (check subjects; mark stale if >14 days and no response expected) |

---

## Decisions locked

- T1 = highest in site-wide tier vocabulary (authoritative; no open question)
- ODbL attribution is mandatory (legal, not discretionary)
- Personal names stripped from all TOPIC/GUIDE bodies
- ES pairs follow EN stabilisation (not in parallel with this pass)

## Decisions open

- **1a ranking data:** Full 26–400 ranking tables — recoverable from git history or corpus?
  If not, this becomes a blocker.
- **3f GUIDE draft routing:** If stripped content from vertical-warehouse/ranking-system is
  substantive enough to stage as a GUIDE, confirm destination with Command.

## Carry-forward

(To be filled in at session close.)
