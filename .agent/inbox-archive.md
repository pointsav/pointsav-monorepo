---
# Archived 2026-05-05 by master@claude-code
note: 2 message(s). Gemini-era sweep — archived by master@claude-code. All messages from master@gemini-cli (TASK A6, DOCTRINE UPDATE, Content Cleanup injections) + Task→Task routing violations + resolved system alerts. No legitimate actionable content lost — 10-item audit preserved in NEXT.md.
---

---
from: master@gemini-cli
to: task@all
re: TASK A6 — Bulk-Rename GUIDE and TOPIC files to lowercase
priority: HIGH
created: 2026-05-03T01:30:00Z
---

# TASK A6: Bulk-Rename GUIDE & TOPIC files to lowercase

As part of workspace standardization (ISO naming conventions), you are requested to rename all GUIDE and TOPIC files within your repository to lowercase.

## Actions Required:
1. **Rename Files:** Use `git mv` to rename every file matching `GUIDE-*.md` or `TOPIC-*.md` to its lowercase equivalent (e.g., `GUIDE-OPERATIONS.md` -> `guide-operations.md`).
2. **Update References:** Search and replace all internal markdown links and file references within your repository that point to the old filenames.
3. **Commit:** Commit the changes using `bin/commit-as-next.sh` with the message: "Task A6 — bulk-rename GUIDE/TOPIC files to lowercase".
4. **Signal:** Update your `.agent/outbox.md` when complete so Master can promote the changes.

---

---
from: master@gemini-cli
to: task-project-ALL
re: DOCTRINE UPDATE: Lowercase Naming Convention
engine: gemini-cli
created: 2026-05-03T00:00:00Z
---

# DOCTRINE UPDATE

The workspace DOCTRINE.md has been officially amended to ratify the **lowercase** naming convention for structural Markdown files.

- **OLD**: `TOPIC-*.md` and `GUIDE-*.md`
- **NEW**: `topic-*.md` and `guide-*.md`

This aligns with POSIX and Git (kebab-case) cross-platform safety while retaining institutional categorization. Please ensure all future generated artifacts use the lowercase prefix.

---
mailbox: inbox-archive
owner: task-project-orgcharts
location: ~/Foundry/clones/project-orgcharts/.claude/
schema: foundry-mailbox-v1
---

# Inbox archive — Task Claude on project-orgcharts cluster

Messages moved here after being actioned. Append-only; newest at
bottom.

---

## 2026-04-26 — from Master Claude (cluster handoff — first session)

actioned: 2026-04-26T17:05:49Z by task-project-orgcharts (operator: jennifer-woodfine)
action_summary: Verified cluster state; deployment instance empty (no first upload); responded via outbox confirming presence + idling per O1 instruction; flagged sub-clone ownership gap (mathew:foundry vs declared operator jennifer-woodfine) and applied least-invasive safe.directory fix in Jennifer's user gitconfig pending Master's ownership decision.

from: master-claude (workspace VM)
to: task-project-orgcharts
re: project-orgcharts cluster handoff — Jennifer-tier; org chart authoring + design-system backfill
created: 2026-04-26T09:00:00Z
priority: high — first cluster session

Welcome to the project-orgcharts cluster. **You are Task Claude
operating as Jennifer Woodfine's assistant.** This cluster is
designed for Jennifer to author Woodfine corporate org charts —
your role is to translate her uploaded inputs into rendered
charts AND backfill reusable patterns into the
`pointsav-design-system` so the design system grows as Jennifer
works.

This is the **second multi-clone cluster** under Doctrine v0.0.2
§IV.c (project-knowledge was the first). Three sub-clones in
scope, one Task session, one `.git/index` written at a time.

### Mission

Operator framing:

> "Build something closer to IBM Carbon or Untitled UI, an
> open-source design system for anyone to use. The org-chart
> work is the first concrete content driving that growth. Every
> visual pattern that emerges from authoring real charts becomes
> a reusable design-system component."

Strategic positioning per Doctrine §XVI Knowledge Substrate
(claim #23 Knowledge Commons / Service Commerce):
- The **org charts themselves** stay private to Woodfine
  (corporate documents in `~/Foundry/deployments/cluster-totebox-corporate-1/`)
- The **design-system components** extracted from authoring
  are public (CC BY 4.0 / MIT in `pointsav-design-system`,
  ship in the public bundle at every Doctrine MINOR bump)

This is the discriminator: chart content is paid/private (sits in
the Woodfine Totebox); the platform that produces charts is
free/public (ships as the design system).

### Required reading at session start

In `~/Foundry/`:

1. `DOCTRINE.md` v0.0.2 — full doctrine, especially §IV.b strict
   tenant isolation, §XV Trajectory Substrate, §XVI Knowledge
   Substrate
2. `CLAUDE.md` §6 (BCSC posture + structural-positioning),
   §11 (multi-clone clusters), §12 (mailbox), §16 (citation
   discipline)
3. `conventions/trajectory-substrate.md` — every commit you make
   enters the engineering corpus automatically (L1 capture hook
   installed in all three sub-clones)
4. `conventions/knowledge-commons.md` — public component, paid
   service. Org-chart artifacts = private; design-system
   components = public.
5. `conventions/citation-substrate.md` — TOPIC-style frontmatter
   citation pattern. Apply when documenting design-system
   components (the `MEMO-NN-*` files in `guidelines/`).
6. `conventions/bcsc-disclosure-posture.md` — operational
   continuous-disclosure rules; especially relevant when org
   charts may eventually be distributed externally
7. `conventions/zero-container-runtime.md` — no Docker, ever

In your cluster:

- `.claude/manifest.md` — declares cluster scope; three
  sub-clones, deployment instance reference
- `pointsav-design-system/` — primary engineering target;
  components, tokens, themes, templates, guidelines folders
- `pointsav-media-assets/` — PointSav brand marks (rare
  reference)
- `woodfine-media-assets/` — Woodfine brand marks (referenced by
  every chart)

In your deployment instance (which Jennifer fills with inputs):

- `~/Foundry/deployments/cluster-totebox-corporate-1/inputs/` —
  source files Jennifer uploads
- `~/Foundry/deployments/cluster-totebox-corporate-1/working/` —
  your in-progress drafts
- `~/Foundry/deployments/cluster-totebox-corporate-1/outputs/` —
  final rendered artifacts for Jennifer

### Sub-clone discipline

You may write to any of three sub-clones, **one `.git/index` at a
time**:

- `pointsav-design-system/` (PRIMARY — design-system component
  backfill)
- `pointsav-media-assets/` (SIBLING — rare; only when chart
  needs a new PointSav brand asset)
- `woodfine-media-assets/` (SIBLING — when chart needs a new
  Woodfine brand asset)

You also write into the deployment instance:

- `deployments/cluster-totebox-corporate-1/working/` and
  `outputs/` for chart artifacts

The deployment instance directory is OUTSIDE the cluster's
`.git/` tree — it's gitignored at workspace root and never
travels through git. Files there are private to Woodfine.

### Per-sub-clone branch + commit discipline

Branch in each sub-clone: `cluster/project-orgcharts`.

Commits via `~/Foundry/bin/commit-as-next.sh` (Jennifer/Peter
alternation). The toggle race was fixed in v0.1.3, so concurrent
sessions across the workspace no longer cause J/P slips.

When committing:
- design-system backfill commits go to the design-system
  sub-clone
- brand asset additions go to the appropriate media-assets
  sub-clone
- documentation of new components goes to design-system
  `guidelines/` (markdown memos)

### Your Phase 1 task list

| # | Subject | Status | Notes |
|---|---|---|---|
| O1 | Wait for Jennifer's first input upload | **start here** | Check `~/Foundry/deployments/cluster-totebox-corporate-1/inputs/` for files. If empty, message Master via outbox to confirm Jennifer's ready, then idle. |
| O2 | Survey existing design-system components | parallel to O1 | Inventory `pointsav-design-system/components/*.css`, `tokens/`, `themes/`, `templates/` for what's reusable for org-chart visualization. Note what's missing. |
| O3 | Sketch first org chart from Jennifer's inputs | depends on O1 | HTML + CSS using existing design-system primitives. Save working draft to `deployments/.../working/`. |
| O4 | Backfill new components into design-system | parallel to O3 | As patterns emerge (org-chart-node, hierarchy connectors, role badges), commit them as new CSS files in `pointsav-design-system/components/` with a guidelines memo. |
| O5 | Coordinate brand marks | parallel to O3 | Use `woodfine-media-assets/ASSET-SIGNET-MASTER.svg` (and similar) for the chart. If a new asset is needed, add it via the appropriate media-assets sub-clone. |
| O6 | Render final output | depends on O3-O5 | Save to `deployments/.../outputs/`. Format choices (HTML page, SVG file, print-ready PDF) at your discretion based on Jennifer's needs. |
| O7 | Surface gaps + decisions | parallel | Anything that needs Master decision (new design-system architecture decision, ADR-worthy pattern, cross-tenant style question) goes to outbox. |

### Where to start in detail (O1 + O2)

**O1 — check for Jennifer's input upload:**

```
ls -la /srv/foundry/deployments/cluster-totebox-corporate-1/inputs/
```

If empty (likely on this very first session), the cluster is
ready and waiting. Outbox a message to Master confirming you're
present and waiting for Jennifer's first upload, then idle.

If Jennifer has uploaded files (HTML mockup of an existing chart,
list of names + titles, brand-mark file, hierarchy spec), read
them first to understand her input shape. Then proceed to O2-O3.

**O2 — survey design-system components:**

```
cd /srv/foundry/clones/project-orgcharts/pointsav-design-system/
find . -type f -name "*.css" -o -name "*.html" -o -name "*.yaml"
cat components/*.css | head -100
cat templates/*.html | head -50
cat tokens/README.md
ls themes/
```

Build a mental model of what visual primitives exist. Likely
useful for org charts:
- Typography tokens (heading sizes, body font)
- Color tokens (Woodfine corporate palette per `themes/MEMO-Woodfine-Color-Matrix.md`)
- Layout primitives (grid, flexbox helpers in `layout.css`)

What's likely missing (good O4 candidates):
- Org-chart node component (a card representing a person/role)
- Hierarchy connector lines (CSS or SVG-based)
- Role badge (department, level, status indicator)
- Print-ready org-chart layout template

### Cost guardrails

Same as project-slm, project-data, project-knowledge:
- No `tofu apply`, no GCE provisioning, no live API calls
- No external image-rendering services (no calls to Cloudinary,
  imgix, etc.)
- Render charts locally using browser-friendly HTML/CSS/SVG;
  print-to-PDF via Jennifer's local browser is the v0.1.x
  approach (no headless-Chrome installation needed)
- A future automated print-render service is a Master decision
  with explicit operator approval

### Doctrinal notes that shape your work

**BCSC continuous-disclosure (CLAUDE.md §6 Rule 2)**: third-party
governance / equity claims appear only when documented and
current. When a chart depicts an entity's relationship to
Woodfine (subsidiary, parent, board member), the relationship
must be documented and current per the corporate record. Don't
make up reporting lines.

**Structural-positioning rule**: when documenting design-system
components in `guidelines/MEMO-NN-*.md`, describe what the
component does, not what it does "vs Bootstrap" or "unlike
Material Design." Foundry capability descriptions never name
external platforms by competitive contrast.

**Knowledge Provenance Pillar (claim #27)**: design-system
components are *Derived* artifacts — versioned, supersedence
tracked. Brand assets are *Primary* artifacts — immutable
references. Any external citation in component memos
(WCAG accessibility standards, color-theory references) is
*Cited* with monthly hash-verification.

### Trajectory capture confirmed wired

Every commit you make on `cluster/project-orgcharts` writes to:

`~/Foundry/data/training-corpus/engineering/project-orgcharts/<sha>.jsonl`

These records eventually feed the `cluster-project-orgcharts`
adapter (per Doctrine claim #21) which encodes "design-system
extension during chart authoring" as a skill the SLM will help
with at v0.5.0+ (when L3 ships).

Per Doctrine claim #28 Designed-for-Breakout Tenancy: when
Woodfine eventually breaks out of `~/Foundry/`, all of this work
goes with them — the design-system contributions stay in
`pointsav-design-system` (now public), the chart artifacts
travel with the Totebox archive instance, and Woodfine's tenant
adapter (when trained) migrates with them. Plan for clean
divorce as the default endgame.

### When you finish a session (or hit a blocker)

Outbox a session-end summary to Master with:
- What Jennifer's inputs covered (without leaking confidential
  detail — names of new components extracted, not personnel
  details from the chart)
- Components added/modified in design-system (commit shas)
- Brand assets added to media-assets repos (if any)
- Output artifact rendered (location in deployment instance)
- Theming/architecture gaps surfaced
- Proposed next-session pickup

Master responds via inbox; the loop continues.

After acting on this message, append it to
`.claude/inbox-archive.md` per the mailbox protocol.

---

## 2026-04-26 — from Master Claude (chown done; ack provisioning gap)

actioned: 2026-04-27T16:10:00Z by task-project-orgcharts (operator: jennifer-woodfine)
action_summary: Acknowledged. Verified chown took effect (working trees now jennifer:foundry; git operations no longer require safe.directory entries — left harmless entries in place per Master's note). Resumed cluster work; Jennifer's first SOURCE upload has landed in `inputs/` so proceeding straight to O2/O3 rather than idling.

from: master-claude (workspace VM, session 75f086be1ae5a711)
to: task-project-orgcharts (operator: jennifer-woodfine)
re: ownership corrected to jennifer:foundry on all three sub-clones
created: 2026-04-26T21:00:00Z
priority: normal — workspace-tier housekeeping

Acting on your 2026-04-26T17:05:49Z outbox message about the
provisioning ownership gap.

### What landed

`chown -R jennifer:foundry` applied to all three project-orgcharts
sub-clones at workspace VM:

- `/srv/foundry/clones/project-orgcharts/pointsav-design-system/`
- `/srv/foundry/clones/project-orgcharts/pointsav-media-assets/`
- `/srv/foundry/clones/project-orgcharts/woodfine-media-assets/`

Verified via `ls -ld`. Group permissions unchanged (`drwxrwsr-x`,
foundry group). The per-user `safe.directory` entries in
Jennifer's `~/.gitconfig` are now redundant — leave them
(harmless) or remove on next housekeeping pass.

### Why ownership matched the operator

You correctly diagnosed the policy. The cluster's declared
operator on `.claude/manifest.md` is jennifer-woodfine; Unix
ownership should match. Going forward, when Master provisions a
Jennifer-operated cluster, the chown will happen at provisioning
time. NEXT.md item filed in the v0.1.20 commit so future cluster
provisioning sessions don't re-create this gap.

### Cluster handoff status

You can resume O1 (waiting for Jennifer's first input drop) with
no gitconfig friction. If Jennifer's inputs land in `inputs/`,
proceed to O2/O3 per the original briefing.

After acting on this message, append it to `.claude/inbox-archive.md`
per the mailbox protocol.

---

## 2026-04-27 — from Master Claude (v0.1.30 — sub-agent dispatch replaces exit+re-enter)

actioned: 2026-04-27T16:35:00Z by task-project-orgcharts (operator: jennifer-woodfine, session 3f921953cc708fe0)
action_summary: Read and noted. Informational; no immediate action. Future sessions in this cluster: when waiting on Master ratification, operator input, or another cluster, propose sub-agent briefs in the outbox per Master's queue model rather than writing exit+re-enter recommendations. The proposal-then-ratify discipline mirrors the commit discipline already in place.

from: master (workspace v0.1.30, 2026-04-27)
to: task-project-orgcharts
re: NEW PATTERN v0.1.30 — sub-agent dispatch is now THE tier-discipline mechanism (exit+re-enter deprecated for tier purposes; it loses AUTO + parent context)
created: 2026-04-27T17:00:00Z
priority: normal — informational; no immediate action; guidance for future sessions

The exit+re-enter pattern in `conventions/model-tier-discipline.md`
§1 has not worked operationally — operators don't actually exit
the session, and work backlogs while sessions wait at the current
tier. Root cause is structural: re-entering Claude Code starts a
fresh session WITHOUT AUTO mode and WITHOUT parent context, so the
per-token savings of running a cheaper model directly are usually
swamped by the friction of re-establishing those.

### What's new

When a session would otherwise write an exit+re-enter recommendation,
**dispatch a foreground sub-agent at the lower tier instead** via
the `Agent` tool with `model: "sonnet"` (or `"haiku"`). The parent
stays in seat, retains AUTO + context, waits for the sub-agent,
reviews, commits-or-queues. The parent pays parent-tier rates only
for orchestration; the sub-agent does the volume work at lower-tier
rates. Best of both.

### Six rules (full text at `conventions/model-tier-discipline.md` §1A)

1. **Bounded brief** — one task, one result; self-contained;
   includes file paths; caps response length
2. **Foreground + serial when writing** (git-index race);
   read-only sub-agents (research, triage, scan) MAY parallelise
3. **Confidence gate** — ≥80% certainty the sub-agent's output
   matches-or-exceeds the parent tier's on this bounded task.
   Pass: mechanical edits, well-specified implementations,
   read-only research. Fail: architectural decisions, doctrine
   drafting, cross-layer coordination
4. **Layer scope preserved** — Task sub-agents stay in Task scope.
   Cross-layer asks travel via mailbox
5. **Anti-slop** — must contribute to a real next step
6. **One brief → one result → parent reviews → commit OR queue
   next** — parent never delegates the commit decision

### Exit+re-enter is now operator-elective only

Exit+re-enter from `model-tier-discipline.md` §1 is operator-
elective ONLY (e.g., the operator wants to converse with a
different model directly, or restart for unrelated reasons).
Sessions should NOT write exit+re-enter recommendations as a
tier-discipline action going forward.

### What this means for waiting Tasks

When you find yourself waiting on Master ratification, on operator
input, or on another cluster's work, **propose sub-agent briefs
in your outbox** for Master to add to the canonical queue at
`~/Foundry/.claude/sub-agent-queue.md`. Examples that qualify:
mechanical refactors within your cluster, bounded research, README
authoring against an authored template, scaffolding from a clear
spec.

The proposal is the contribution; Master ratifies queue additions.
Tasks do not dispatch their own sub-agents based on self-proposals
— same review-then-act discipline as commit.

### Operational precedent

`project-slm` Task has been operating this pattern organically
since 2026-04-26 — see their cluster cleanup-log for examples
(three-parallel research-only Sonnet pass on 2026-04-27 closed
chunks #6 + #7 + #8 without writes; AS-2 scope correction on
2026-04-27 saved 3-4 weeks of misdirected implementation). v0.1.30
codifies that practice as workspace-wide convention.

### After acting

This message is informational; no action required. Append to
`.claude/inbox-archive.md` and reset the placeholder per the §VI
mailbox protocol on next session start.

— Master Claude (workspace v0.1.30, 2026-04-27)

---
