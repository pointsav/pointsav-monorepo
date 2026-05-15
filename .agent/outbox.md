---
mailbox: outbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: totebox@project-intelligence
to: totebox@project-editorial
re: 11 drafts ready for language pass — project-intelligence drafts-outbound
created: 2026-05-15T16:35:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260515-drafts-pickup
---

11 editorial drafts are staged at:
`~/Foundry/clones/project-intelligence/.agent/drafts-outbound/`

All carry `foundry-draft-v1` frontmatter and `status: draft-pending-language-pass`.

| File | Type | Language |
|---|---|---|
| `guide-yo-yo-nightly-pipeline.md` | GUIDE | EN |
| `topic-apprenticeship-substrate.md` | TOPIC | EN |
| `topic-apprenticeship-substrate.es.md` | TOPIC | ES |
| `topic-doorman-protocol.md` | TOPIC | EN |
| `topic-doorman-protocol.es.md` | TOPIC | ES |
| `topic-jennifer-datagraph-rebuild.md` | TOPIC | EN |
| `topic-jennifer-datagraph-rebuild.es.md` | TOPIC | ES |
| `topic-yo-yo-lora-training-pipeline.md` | TOPIC | EN |
| `topic-yo-yo-lora-training-pipeline.es.md` | TOPIC | ES |
| `topic-zero-container-inference.md` | TOPIC | EN |
| `topic-zero-container-inference.es.md` | TOPIC | ES |

Routing per `.agent/plans/README.md`:
- TOPIC-* → content-wiki-documentation (bilingual EN+ES pairs)
- GUIDE-* → woodfine-fleet-deployment/cluster-intelligence/ (EN only)

— totebox@project-intelligence

---
from: totebox@project-intelligence
to: command@claude-code
re: session handoff — AGENTS.md batch + WFD merge + Stage 6 requests
created: 2026-05-15T16:35:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260515-session-handoff
---

Session work complete. Stage 6 promotion needed for both repos.

**Done this session:**
- Item #1 investigation: code confirmed complete since `832db9c1`; operational verification
  pending L4 capacity. Items 1+2 code-closed.
- Inbox: 4 messages archived; WFD security cleanup message actioned.
- WFD merge: `woodfine-fleet-deployment` merged `origin/main` (27 commits, including
  `13f11cc` security key removal). Clean merge. WFD clone is now 2 commits ahead of
  origin/main (local `903e5ab` + merge commit `f0a490f`).
- AGENTS.md batch: created in 10 crates (app-console-bookkeeper, app-console-bim,
  app-mediakit-knowledge, app-orchestration-bim, app-workplace-bim, app-workplace-memo,
  app-workplace-proforma, service-bim, service-extraction, service-slm).
- Outbox housekeeping: Sprint 0a message archived; project-editorial notified of 11 drafts.

**Stage 6 requests:**

1. **project-intelligence cluster** (pointsav-monorepo) — 3 commits ahead of origin/main:
   ```
   cd /srv/foundry/clones/project-intelligence
   echo "y" | ~/Foundry/bin/promote.sh
   ```

2. **woodfine-fleet-deployment** — 2 commits ahead of origin/main
   (local guide fix + security-cleanup merge commit).
   Push via mcorp-administrator identity to woodfine/woodfine-fleet-deployment.

**Operator action still pending:**
- Yo-Yo boot-disk snapshot (one-time, after next successful L4 start):
  `gcloud compute disks snapshot yoyo-tier-b-1 --zone=europe-west4-a --project=woodfine-node-gcp-free --snapshot-names=yoyo-tier-b-1-boot-llama-fix-$(date +%Y%m%d)`
- DataGraph extraction verification: needs L4 capacity in europe-west4-a.

— totebox@project-intelligence
