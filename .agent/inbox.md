---
mailbox: inbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Inbox — task-project-intelligence

---
from: command@claude-code
to: totebox@project-intelligence
re: service-content vocabulary refresh — 5 issues gating all 3 wiki relaunches
created: 2026-05-16T00:00:00Z
priority: high
status: pending
msg-id: project-intelligence-20260516-service-content-vocab-refresh
---

Routed from project-editorial outbox (2026-05-16T00:30Z). All three wiki relaunches
(documentation.pointsav.com, corporate.woodfinegroup.com, projects.woodfinegroup.com)
are editorially complete but gated on these service-content/ontology/ fixes.

**Issue 1 — `wiki_repo` field stale in topic CSVs**
`topics_corporate.csv` and `topics_projects.csv` have incorrect `wiki_repo` values.
Update to point at `content-wiki-corporate` and `content-wiki-projects` respectively.

**Issue 2 — `wiki_path` format stale**
CSV entries use old path format (`topics/topic-*.md`).
Update to `<category>/<slug>.md` per current content-contract.md §4.

**Issue 3 — Glossary v9 terms not applied to DataGraph CSVs**
The wiki glossaries received a v9 vocabulary pass; `service-content/ontology/` CSVs
have not. Every downstream DataGraph consumer is reading stale vocabulary.

**Issue 4 — ~251 articles unclassified; ~72 GUIDEs unregistered**
Editorial gaps affecting wiki discovery and search.

**Issue 5 — Domains.json / domain seed files — Bloomberg vocabulary audit**
Audit for Bloomberg vocabulary violations (no "Foundry", no internal paths, no
"Doctrine claim"). The project-editorial OPUS audit could not inspect these files
(monorepo sub-clone in project-editorial cluster is empty — use this cluster's clone).

**Action:** Fix Issues 1–3 minimum before wiki services restart after Stage 6.
Issues 4–5 can follow in a subsequent session. Log completion in outbox.

Reference: `clones/project-editorial/.agent/plans/overhaul-progress.md` (2026-05-15 entry)
for full editorial audit context.

— command@claude-code

---
from: command@claude-code
to: task@project-intelligence
re: comprehensive handoff — all outstanding project-intelligence work (2026-05-14)
created: 2026-05-14T00:00:00Z
priority: high
status: in-progress
---

This message consolidates all outstanding Totebox-scope work for project-intelligence.
Command Session is handing this off cleanly — nothing here requires Command action.

**Prior inbox messages — status:**
- `re: URGENT — rebuild + deploy service-content` (2026-05-13T17:58Z) — **COMPLETED.**
  Watcher fix (b8a70ee / 3e8c8a4) is deployed and confirmed working. Service has been
  stable since 2026-05-13T20:05Z. Archive this message.
- `re: investigate Doorman routing returning invalid JSON` (2026-05-13T23:30Z) — **OPEN.**
  Still needs investigation. See item 1 below.

---

## 1. Doorman extraction interface — investigation + fix (carry-forward from open inbox)

**STATUS (2026-05-15 session): CODE COMPLETE — `832db9c1`. Pending operational verification.**
`POST /v1/extract` wired; `route_yoyo_only("trainer")` in router; service-content updated.
`{deferred: true}` returned when Tier B unavailable — no retry storm.
Verification blocked on L4 stockout in europe-west4-a. Run startup sequence when capacity returns.

---

## 2. start-yoyo.sh line 340 — update_doorman_env on every Mode 1 success

**STATUS (2026-05-15 session): CODE COMPLETE — already unconditional in current code.**

---

## 3. Universal AI Gateway — Sprint 0a (Anthropic Messages shim)

**STATUS (2026-05-15 session): DONE — `fdd1a223` + hardening in `7cd9ca61`.**
`POST /v1/messages` live on workspace VM. Sprint 0b (real streaming + on-demand boot) is next.

---

## 4. Drafts outbound — notify project-editorial

**STATUS: DONE — outbox message sent 2026-05-15.**

---

## 5. Outbox — archive stale messages

**STATUS: DONE — 2026-05-15.**

---

## 6. Stage 6 — promote cluster branch to canonical main

**STATUS: DONE (2026-05-15). All commits on canonical origin/main.**

---

## 7. Yo-Yo — mask vllm.service before next boot

**STATUS: SUPERSEDED — europe-west4-a correction applied. vllm.service masked.**

---

## 8. Set SLM_YOYO_WEIGHTS_GCS_BUCKET in local-doorman.env

**STATUS: DONE — already set in `/etc/local-doorman/local-doorman.env`.**

---

## 9. Packer image rebuild + OLMo 3 32B weights upload (after item 7 complete)

**STATUS: OPERATOR-BLOCKED.** vllm.service mask on yoyo-tier-b-1 confirmed done (NEXT.md).
Packer rebuild + boot-disk snapshot are the remaining operator actions.

— command@claude-code
