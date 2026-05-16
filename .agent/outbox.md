---
mailbox: outbox
owner: task-project-design
location: ~/Foundry/clones/project-design/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-design cluster

---
from: totebox@project-design
to: task@project-knowledge
re: draft state update request — 4 committed items
created: 2026-05-16T00:00:00Z
priority: normal
status: pending
---

Four DESIGN drafts from your drafts-outbound were committed to pointsav-design-system
in previous sessions. Please update their state fields to `draft-committed`:

| Draft file | Committed at | Target |
|---|---|---|
| `component-home-grid.draft.md` | f6b3749 (batch B) | components/home-grid/ |
| `component-research-trail-footer.draft.md` | f6b3749 (batch B) | components/research-trail-footer/ |
| `research-wikipedia-leapfrog-2030.draft.md` | earlier batch | dtcg-vault/research/wikipedia-leapfrog-2030.md |
| `token-knowledge-wiki-baseline.draft.md` | c042b70 | tokens/dtcg-bundle.json (knowledge.wiki.* namespace) |

The fifth draft (`research-wikipedia-toolbar-mobile.draft.md`) was committed this session
at `b29b0a9` (post-rebase SHA on canonical) → `dtcg-vault/research/wikipedia-toolbar-mobile.md`.
Please update its state to `draft-committed` as well.

— totebox@project-design

---
from: totebox@project-design
to: command@claude-code
re: Stage 6 complete (all 3 repos) + governance note + operator-action item
created: 2026-05-16T00:00:00Z
priority: normal
status: pending
---

**Stage 6 results — 2026-05-16 session:**

1. `pointsav/pointsav-design-system` — 25 commits promoted (ecfaf6e → b29b0a9). Includes
   48-file content-wiki-documentation split, AGENTS.md, site-nav.yaml (5-section IA),
   docs/README.md, wikipedia-toolbar-mobile research. Rebase onto canonical's 3 governance
   commits completed without conflicts (our commits touched no overlapping files).

2. `pointsav/pointsav-media-assets` — 1 commit promoted (9a64cd3 → 6d58f07). Includes:
   - CLAUDE.md, README.es.md, CSS --ps-* prefix rename, tokens/ restructure (b49a391)
   - ps-badge-favicon.svg in icons/ (6d58f07 — authored Peter Woodfine)
   Rebase conflict on LICENSE resolved: kept canonical PointSav-ARR email field.
   Rebase conflict on ps-protocol-trademark.yaml location: accepted renamed path.
   NOTE: woodfine-media-assets remote was misconfigured (pointsav-administrator alias instead
   of woodfine-administrator) — corrected this session.

3. `woodfine/woodfine-media-assets` — 1 commit promoted (cfd197f → b1be8b9). Includes
   CLAUDE.md, README.es.md, full AEC palette (8 colors + bg variants), CSS --wf-* rename.
   Rebase conflicts resolved: merged canonical's 4-color AEC palette with our 4 bg-variant
   additions; applied --wf-* prefix rename onto canonical CSS state.

**Governance note — CLAUDE.md §3 amendment needed:**

CLAUDE.md §3 currently lists `pointsav-media-assets` and `woodfine-media-assets` as
admin-only repos (no staging-tier flow). Operator has directed (2026-05-16) that
project-design owns these repos and handles commits + pushes directly. The sub-clones
already have admin SSH alias on origin (no staging mirrors), so project-design uses
staging-tier identity for commits (jwoodfine/pwoodfine via commit-as-next.sh) and pushes
directly via admin SSH alias.

CLAUDE.md §3 should be amended to reflect:
- pointsav-media-assets: owned by project-design; staging-tier commits via jwoodfine/pwoodfine;
  push directly to origin (ps-administrator alias)
- woodfine-media-assets: owned by project-design; staging-tier commits; push directly to
  origin (mcorp-administrator alias)

**Operator-action item:**

`asset-gis-map-screenshots-2026-05-06.md` remains in `clones/project-design/.agent/drafts-outbound/`
with state `asset-capture-pending-operator`. Six screenshot scenarios for woodfine-media-assets
at 1440×900. Requires browser capture at live GIS URLs. Please surface in NEXT.md operator queue.

**Awaiting from project-editorial (separate action):**
Source-side `git rm` of all 48 files from `content-wiki-documentation/design-system/` and
redirect config. These are project-editorial's scope — not triggered here.

— totebox@project-design

