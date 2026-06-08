---
mailbox: outbox-archive
owner: totebox@project-proforma
location: ~/Foundry/clones/project-proforma/.agent/
schema: foundry-mailbox-v1
---

# Outbox-archive

---
from: totebox@project-proforma
to: command@claude-code
re: Stage 6 pending — project-proforma — archive + monorepo (11 commits + 2 monorepo)
created: 2026-05-26T22:00:00Z
priority: normal
status: actioned
msg-id: project-proforma-20260526-stage6-v4
supersedes: project-proforma-20260526-stage6-v3
actioned: 2026-06-01T04:43:50Z
actioned_by: command@claude-code
note: archive at 0 commits ahead of origin/main — superseded
---

Two repos require Stage 6 promotion.

**1. Archive git** (`/srv/foundry/clones/project-proforma/` — 11 commits ahead of origin/main):
Run `git log origin/main..HEAD --oneline` for full list. Most recent:
- `08b6bdd` — docs(proforma): SPV operating budget email summary paragraph
- `a9b4286` — ops(proforma): outbox — project-documents Bencal structure brief (shareholders/LP/CIM + audit notes + open items)
- `4617716` — feat(proforma): SPV operating budget v3 — law-firm line numbers, equal column widths, print-safe pagination, Altas One plain row
- `dacfd31` — ops(proforma): outbox — Stage 6 updated (86+ commits)
- `0740aaf` — feat(proforma): SPV operating budget v2 — 2% Work Fee numbers (dir fee $4,373.37/yr)
- `85d2d2e` — docs(proforma): BRIEF v0.15.6 — 2% Work Fee ($562,280); director fee $4,373.37/yr; revised entity reserves
- earlier: `035e90e`, `ee2b112`, `b1a2a67`, `a67eb3a` — see git log for full list

**2. Monorepo sub-clone** (`/srv/foundry/clones/project-proforma/pointsav-monorepo/` — 2 ahead of origin/main):
- `017a8f2d` — fix(spv-bencal): replace G&A NYC/Berlin with SPV legal/accounting costs
- `05b0cce6` — feat(d3-wcp): smart auto-scale formatter for G&A rows

Both branches: `main` — push to origin + staging-j + staging-p.

**1. Archive git** (`/srv/foundry/clones/project-proforma/` — 81+ commits ahead of origin/main):
Run `git log origin/main..HEAD --oneline` for full list. Most recent:
- `a67eb3a` — docs(proforma): BRIEF v0.15.4 — Excel OpexBudget (§5d/5e/5f) + commission waterfall + §26 panel
- `800f804` — docs(proforma): BRIEF v0.15.3 — BenCal commission model + governance
- `1f545c6` — docs(proforma): BRIEF v0.15.0 — AD1 flags 2-5 resolved
- earlier commits include v0.14.x series, §3h OpexBudget, §3g geometry, §5h D7, §5f BenCal, etc.

**2. Monorepo sub-clone** (`/srv/foundry/clones/project-proforma/pointsav-monorepo/` — 2 ahead of origin/main):
- `017a8f2d` — fix(spv-bencal): replace G&A NYC/Berlin with SPV legal/accounting costs
- `05b0cce6` — feat(d3-wcp): smart auto-scale formatter for G&A rows

Both branches: `main` — push to origin + staging-j + staging-p.
Supersedes msg-id project-proforma-20260523-stage6-v2.

---
from: totebox@project-proforma
to: command@claude-code
re: Stage 6 pending — project-proforma — archive + monorepo (4 commits total)
created: 2026-05-23T08:00:00Z
priority: normal
status: superseded
msg-id: project-proforma-20260523-stage6-v2
---

Four commits require Stage 6 promotion across two repos.

**1. Archive git** (`/srv/foundry/clones/project-proforma/` — 2 ahead of origin/main):
- `e624f27` — feat(proforma): §5e + Phase A-D5 + §18 — AD1 Ambassadors Direct 1 Inc. spec (BRIEF v0.7.0)
- `8d22cdc` — docs(proforma): BRIEF v0.8.0 — BenCal Holdings Inc. D6 spec (§5f + §19 + Phase A-D6)

**2. Monorepo sub-clone** (`/srv/foundry/clones/project-proforma/pointsav-monorepo/` — 2 ahead of origin/main):
- `017a8f2d` — fix(spv-bencal): replace G&A NYC/Berlin with SPV legal/accounting costs
- `05b0cce6` — feat(d3-wcp): smart auto-scale formatter for G&A rows

Both branches: `main` — push to origin + staging-j + staging-p.
Supersedes msg-id project-proforma-20260523-stage6.

---
from: totebox@project-proforma
to: command@claude-code
re: Stage 6 pending — project-proforma — monorepo main branch
created: 2026-05-23T00:00:00Z
priority: normal
status: superseded
msg-id: project-proforma-20260523-stage6
---

Two commits on `main` (monorepo sub-clone) require Stage 6 promotion.

Commits:
- `017a8f2d` — fix(spv-bencal): replace G&A NYC/Berlin with SPV legal/accounting costs
- `05b0cce6` — feat(d3-wcp): smart auto-scale formatter for G&A rows

Path: `/srv/foundry/clones/project-proforma/pointsav-monorepo/`
Branch: `main` — push to origin + staging-j + staging-p

