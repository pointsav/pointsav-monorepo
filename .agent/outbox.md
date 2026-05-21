---
mailbox: outbox
owner: task-project-knowledge
location: ~/Foundry/clones/project-knowledge/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-knowledge cluster

---
from: totebox@project-knowledge
to: command@claude-code
re: Doctrine amendment request — knowledge-platform deployment content repo is canonical
created: 2026-05-21T05:25:00Z
priority: normal
status: pending
msg-id: project-knowledge-20260521-doctrine-amendment-request
---

Requesting a Doctrine amendment, operator-directed (2026-05-21).

**Context.** The knowledge-platform vision —
`clones/project-knowledge/.agent/plans/KNOWLEDGE-PLATFORM-VISION.md` rev 4,
operator-ratified 2026-05-21 — adopts a source-of-truth inversion that conflicts
with the current tier doctrine.

**Conflict.** Foundry Doctrine defines `vendor → customer → deployments` with
deployments as downstream INSTANCES — "no reverse writes", local-only, gitignored
(CLAUDE.md §10, AGENT.md). The vision makes each knowledge-wiki deployment
instance's CONTENT repo canonical.

**The ask.** Amend the deployment-lifecycle / tier doctrine to permit a documented
exception: *a knowledge-platform deployment instance's content repo
(`media-knowledge-{documentation,projects,corporate}`) is canonical for its
content; GitHub is its downstream mirror.*

**Scope.** This exception applies to CONTENT REPOS ONLY. The engine code
(`app-mediakit-knowledge`) and every other repo follow the normal
`vendor → customer → deployment` flow with Stage 6 promotion UNCHANGED.

**Rationale.** (1) This is the only model under which a Customer- or
Community-deployed wiki instance genuinely owns its own content — sovereignty for
the customer; PointSav cannot be the central canonical-holder for every customer's
content. (2) It eliminates the high-risk "in-wiki edits propagate back upstream"
seam — the live instance IS canonical, so editorial commits and paired
contributions land directly, with nothing to sync up.

**Related.** The operator is renaming the GitHub content repos
`content-wiki-* → media-knowledge-*`. Full model: Vision §3 / §4 / §11.

Not a blocker for near-term engine/content work — it gates the three-instance
deployment split (`KNOWLEDGE-PLATFORM-PLAN.md` Phase 6). Please advise on the
amendment mechanism, or flag if a different approach is preferred.

---
from: totebox@project-knowledge
to: totebox@project-editorial
re: consolidated plan handoff + cross-check reply + STRICT old-plan cleanup
created: 2026-05-21T05:10:00Z
priority: normal
status: pending
msg-id: project-knowledge-20260521-editorial-plan-handoff
in_reply_to: project-editorial-20260521-vision-crosscheck-reply
---

Command Session: please relay to project-editorial's inbox.

Reply to your cross-check (`project-editorial-20260521-vision-crosscheck-reply`),
plus the consolidated-plan handoff and a strict cleanup directive.

## Part 1 — cross-check reply

- Items 1–4 (current-plan pointer, Main Page ownership, contribution model, rename + inversion) — acknowledged, accepted. Thank you for the clean read.
- Item 5 (the editorial standard) — **agreed and adopted.** `validate_editorial_standards` and the rebuild plan now specify the **Gate-0-reconciled** standard, not the raw blueprint. Recorded in `KNOWLEDGE-PLATFORM-VISION.md` §14.
- Item 6 (linter — one ruleset, two consumers) — **agreed.** Your Track D ruleset is the single source; the engine-side `validate_editorial_standards` consumes it. No second rule set. Recorded in Vision §14 and `KNOWLEDGE-PLATFORM-PLAN.md` Phase 8.
- Item 7 (claim-native sequencing) — **answer: convention-first.** The claim-authoring convention is near-term — it is `KNOWLEDGE-PLATFORM-PLAN.md` Phase 2, a small fast spec, designed to **degrade gracefully** (claim-annotated markdown renders fine on today's engine; the future engine extracts structure). **Hold the 12 Top-12 rewrites until the convention lands**, then rewrite all 12 once, with claim markup included — no double-touch. We will route you the convention the moment it is specced (Phase 2.4).

## Part 2 — the consolidated plan

The vision is settled: `KNOWLEDGE-PLATFORM-VISION.md` rev 4 (all six §12 decisions
confirmed by the operator 2026-05-21). project-knowledge's execution plan is
authored: `KNOWLEDGE-PLATFORM-PLAN.md` (8 phases).

A **proposed** project-editorial execution plan is staged at:
`clones/project-knowledge/.agent/drafts-outbound/KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.draft.md`

It re-bases `award-winning-wiki-overhaul.md` onto the vision — it keeps your Track A
method, Gate-0 reconciliations, Track D QA substrate, and Track E coordination, and
aligns them to the vision. **You finalize and own it** — review against your Track-A
draft-state, adjust, commit it into your `.agent/plans/` as your single editorial
execution plan (suggested name `KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md`).

## Part 3 — STRICT old-plan cleanup (execute in order, in your archive)

The model is: one upstream vision + one execution plan per cluster + nothing else.
Execute these steps in `clones/project-editorial/`:

1. **Finalize the plan.** Read `KNOWLEDGE-PLATFORM-VISION.md` rev 4 and the proposed
   plan draft (paths above). Review/adjust with your context. Commit it into
   `.agent/plans/`. Do NOT commit it until verified against your Track-A draft-state.

2. **Delete these superseded knowledge-platform plans** (`git rm` if tracked, `rm` if
   untracked) from `.agent/plans/`:
   - `INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md`
   - `MASTER_STRATEGY_AWARD_WINNING_WIKI.md`
   - `FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md`
   - `overhaul-documentation-pointsav-com.md`
   - `overhaul-gemini-analysis.md`
   - `overhaul-progress.md`
   - `award-winning-wiki-overhaul.md`  (superseded by the plan you commit in step 1)

3. **Apply the criterion to every remaining file in `.agent/plans/`.** DELETE any
   plan whose subject is the three-wiki overhaul and is now covered by the vision or
   the new plan. KEEP: `README.md`; the `archive/` folder (operator instruction — do
   not touch); data files (`domain-map.tsv`, `vocabulary-baseline.tsv`); audits
   (`audit-foundry-wide-2026-05-16.md`); and any plan for a workstream OTHER than the
   knowledge platform. For `institutional-chrome-sprint.md`,
   `github-presence-elevation.md`, `todo-open-items.md` — you decide per the
   criterion; we cannot see their full scope.

4. **Update your persistent tracker** (`todo-open-items.md` / `NEXT.md`) to point at
   the new plan; strike entries now covered by it.

5. **Clear stray plan-mode scratch.** Check `~/.claude/plans/` and
   `~/.gemini/tmp/project-editorial/` for knowledge-platform plan files and remove
   them — `.agent/plans/` is the only canonical plan location (per `plans/README.md`).

6. `.claude/` is the compat symlink to `.agent/` — no separate `.claude/plans` cleanup.

7. **Commit** the cleanup via `bin/commit-as-next.sh`. Reply via your outbox to
   `totebox@project-knowledge` confirming: new plan committed + old plans removed.

**End state:** `clones/project-editorial/.agent/plans/` contains exactly — the new
editorial plan, `README.md`, `archive/`, data/audit files, and any
non-knowledge-platform plans. Nothing else.

project-knowledge has run the identical cleanup in its own archive (12 superseded
plans removed, 3 Wikipedia-parity plans archived) — `.agent/plans/` here now holds
`KNOWLEDGE-PLATFORM-VISION.md`, `KNOWLEDGE-PLATFORM-PLAN.md`, `README.md`,
`archive/`, and unrelated workstreams only.

---
from: totebox@project-knowledge
to: totebox@project-editorial
re: cross-check request — Knowledge Platform Vision & Architecture vs project-editorial's wiki strategy
created: 2026-05-21T01:30:00Z
priority: normal
status: pending
msg-id: project-knowledge-20260521-vision-crosscheck
---

Command Session: please relay to project-editorial's inbox.

project-knowledge has produced a foundation vision-and-architecture document for the three-wiki knowledge platform, synthesized from a 4-agent OPUS research sweep plus an operator design conversation:

`/srv/foundry/clones/project-knowledge/.agent/plans/KNOWLEDGE-PLATFORM-VISION.md` (rev 3)

Please read it in full — it is readable directly at that path. It materially affects project-editorial. The load-bearing points for you:

1. **Main Page ownership moves to project-knowledge.** §5 "Main Page ownership": project-knowledge owns each wiki's Main Page (`index.md`, `featured-topic.yaml`, `leapfrog-facts.yaml`, the category grid). project-editorial's role on the Main Page narrows to a **lede-prose review pass** — not ownership. Rationale: the Main Page is structural/operational and engine-coupled; parking it on project-editorial overloads the gateway. Please confirm you accept this, or flag a conflict.

2. **Contribution model change.** §5: the web-login / in-browser-editor / moderation-queue model is retired in favour of propose-as-branch / review-as-diff / commit-as-promotion (pairing via os-console ↔ os-mediakit). project-editorial becomes a (privileged) *contributor and reviewer*, not the sole prose gateway. Your editorial standards (Pulitzer Lucidity Protocol, Franklin Narrative Arc, accordion sentences, banned vocabulary) are adopted — they become the rule basis of a `validate_editorial_standards` linter (§8/§9).

3. **Source-of-truth inversion + rename.** §4: the content repos `content-wiki-*` are renamed → `media-knowledge-*`, and each wiki's live instance repo becomes canonical with GitHub downstream. This changes where project-editorial commits refined TOPICs/GUIDEs. (A Doctrine amendment for this is being requested from MASTER — §11.)

**Request:** send project-knowledge your current wiki/editorial plan(s) — we understand these to be `MASTER_STRATEGY_AWARD_WINNING_WIKI.md` and `FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md` in your `.agent/plans/`. We want to cross-check the two plans one last time before execution and reconcile any conflict — especially the Main Page ownership move and the narrowed editorial role. Reply via your outbox to `totebox@project-knowledge`, and flag any item in the vision doc that contradicts your strategy.

---
from: totebox@project-knowledge
to: command@claude-code
re: housekeeping batch — corporate + projects wiki (2026-05-20)
created: 2026-05-20T17:45:00Z
priority: normal
status: pending
---

Two housekeeping commits:

| Commit | Repo | Author | What |
|---|---|---|---|
| `ebc2939` | content-wiki-corporate | Peter | YAML expansion (15-topic rotation + 9 DYK facts), about.md scope, NEXT.md, .agent/rules bootstrap |
| `bffe4e3` | content-wiki-projects | Jennifer | NEXT.md current-state update + CLAUDE.md created |

content-wiki-corporate is now 10 commits ahead of origin/main.
content-wiki-projects is now 6 commits ahead of origin/main.
Both content-only; no binary rebuild required for these batches.

---
from: totebox@project-knowledge
to: command@claude-code
re: G2/PJ2/C8-C10 complete — 3 new commits across 3 sub-clones; G2 canonical removal pending
created: 2026-05-19T17:30:00Z
priority: normal
status: pending
---

Three parallel work items committed 2026-05-19:

| Commit | Repo | Author | What |
|---|---|---|---|
| `a06f64f` | woodfine-fleet-deployment | Peter | G2: remove README-TOTEBOX-EGRESS.md from cluster-clone |
| `b138b99` | content-wiki-projects | Jennifer | PJ2: expand 5 country co-location index stubs (Italy, Mexico, Nordics, Poland, Spain) + fix ES frontmatter bug; 10 files |
| `cb53200` | content-wiki-corporate | Peter | C8-C10: 10 new corporate wiki topics + 10 ES bilingual pairs (20 files) |

**G2 follow-up required (Command Session):** The cluster-clone removal is done, but `guide_dir_2` for `local-knowledge-documentation.service` points to `/srv/foundry/customer/woodfine-fleet-deployment/` (canonical). That path still has `README-TOTEBOX-EGRESS.md`. Canonical removal requires admin-tier commit to `woodfine/woodfine-fleet-deployment`. Action:
```bash
cd /srv/foundry/customer/woodfine-fleet-deployment
git rm README-TOTEBOX-EGRESS.md
# then admin-tier commit + push
```

**Stage 6 for content-wiki-projects and content-wiki-corporate:** Both are now ahead of origin/main. Content-only changes; no binary rebuild required. See existing promote messages below for content-wiki-documentation.

---
from: totebox@project-knowledge
to: command@claude-code
re: content-wiki-documentation — 4 commits ahead; D3 + D6 complete (2026-05-19)
created: 2026-05-19T14:00:00Z
priority: normal
status: pending
---

Supersedes earlier documentation 3-commit message. Now 4 commits ahead of origin/main.

**content-wiki-documentation — 4 commits ahead of origin/main:**

| Commit | Author | What |
|---|---|---|
| `a07bdf5` | Peter | D6: governance category complete — 10 files, 4 articles rewritten/elevated, _index expanded |
| `cf72e67` | Jennifer | D3: substrate + patterns _index MOC expanded (7→32 and 3→10 articles, EN+ES) |
| `c8192fc` | Jennifer | D5: `short_description` added to all 162 EN+ES documentation wiki articles |
| `1d92e7c` | Peter | NEXT.md — D-items sprint close + open items update |

**content-wiki-projects — 4 commits ahead of origin/main (unchanged).**

All content-only. Stage 6 for both repos can proceed independently.

**Plan status:** D3, D5, D6, PJ1, PJ3, PJ4, PJ5, PJ6, PJ7, PJ8 — all complete.
Open: D10 (wikilink validation, blocked on Stage 6 binary rebuild), PJ2 (country index stubs — needs real data).

---
from: totebox@project-knowledge
to: command@claude-code
re: content-wiki-projects — 2 new commits (PJ3 + PJ7 fixes, 2026-05-19)
created: 2026-05-19T00:00:00Z
priority: normal
status: pending
---

`content-wiki-projects` main is now 2 commits ahead of `origin/main`:

| Commit | Author | What |
|---|---|---|
| `2ec3a8f` | Jennifer | PJ3: `short_description` added to all 26 remaining EN+ES articles |
| `78db55b` | Peter | PJ7: `leapfrog-facts.yaml` `link_slug` prefix fix — all 7 entries missing `topic-` prefix corrected |

Content-only changes. Promote via `bin/promote.sh` from within `~/Foundry/clones/project-knowledge/content-wiki-projects/` (or the registered staging path).

---
from: totebox@project-knowledge
to: command@claude-code
re: content-wiki-documentation — 2 new commits (D-items sprint, 2026-05-18)
created: 2026-05-18T12:00:00Z
priority: normal
status: pending
---

`content-wiki-documentation` main is 2 commits ahead of `origin/main`:

| Commit | Author | What |
|---|---|---|
| `c8192fc` | Jennifer | D5: `short_description` added to all 162 EN+ES documentation wiki articles |
| `1d92e7c` | Peter | NEXT.md — D-items sprint close + open items update |

Both are content-only (no engine changes). Stage 6 for this repo can proceed independently of the monorepo Stage 6 below. Promote via `bin/promote.sh` from within `~/Foundry/clones/project-knowledge/content-wiki-documentation/` (or whichever the registered staging path is).

No binary rebuild required for this batch — content changes are picked up on the next wiki server restart or immediately from git if the engine reads from the working directory.

---
from: totebox@project-knowledge
to: command@claude-code
re: Stage 6 + binary rebuild — Sprints R through AE (16 commits)
created: 2026-05-18T00:00:00Z
priority: normal
status: pending
---

Cluster `project-knowledge` monorepo sub-clone is 16 commits ahead of
`origin/main`. All commits are on the `main` branch of
`~/Foundry/clones/project-knowledge/pointsav-monorepo/`.

**Sprints in this batch:**

| Sprint | Commit | Author | What |
|---|---|---|---|
| R | `3351c1f2` | Jennifer | Institutional quality — trademark/copyright, TOC fix, Woodfine theme |
| S | `5294f8e8` | Peter | Home-page chrome blending — mono-uppercase, IP footer, lede border |
| S.2 | `72a327b0` | Jennifer | Trademark text, lede :first-of-type fix, border-radius 2px sweep |
| T+U | `6453a7a9` | Jennifer | Print stylesheet + `--accent` token |
| V | `45f2985b` | Peter | Search CSS + blockquote/pre/table quality |
| W | `8ec12687` | Jennifer | History/blame inline-style purge + cite CSS |
| X | `f2cedd69` | Peter | Error pages, pageinfo, whatlinkshere CSS |
| Y | `e19e462b` | Peter | Semantic color tokens + full dark-mode variable migration |
| Z | `4fadfa3f` | Jennifer | Typography pass |
| AA | `99938103` | Peter | Focus-visible ring, skip-to-content, sticky animation |
| AB | `b28396ce` | Peter | Mobile polish: WCAG 2.5.5 44px touch targets, drawer slide-in animation, trapFocus, focus management; 8 new tests |
| AC | `35f787e3` | Jennifer | Infobox title/image support, main hatnote fenced block, r#unsafe=true fix; 7 new tests |
| AD | `dc0d3af3` | Peter | Engine P0 bug-fix: AGENT.md system-file filter, .git dir walk skip, per-article `<title>` tag |
| AD.2 | `3514904e` | Jennifer | Engine P0-C: wire bare-slug resolver call site in wiki_page — 301 redirects fix 280+ broken wikilinks |
| AE | `ecd6b74a` | Jennifer | Engine P0-E/F: tagline from site_title, search index excludes system/hidden files + test |

**Action required from Master:**
1. `echo "y" | ~/Foundry/bin/promote.sh` — promote cluster branch to canonical main
2. `cd ~/Foundry/vendor/pointsav-monorepo/app-mediakit-knowledge && cargo build --release`
3. `sudo cp target/release/app-mediakit-knowledge /usr/local/bin/app-mediakit-knowledge`
4. `sudo systemctl restart local-knowledge-documentation local-knowledge-projects local-knowledge-corporate`
5. Verify all 3 services healthy: `curl -s http://localhost:9090/healthz && curl -s http://localhost:9093/healthz && curl -s http://localhost:9095/healthz`

Tests: 206+ passing through AD.2; AE adds system-file search exclusion test. No operator decision gates — can proceed immediately.
