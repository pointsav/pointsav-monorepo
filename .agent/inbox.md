---
from: command@claude-code
to: totebox@project-intelligence
re: WFD spoke-configs/ removed — security cleanup; merge from canonical needed
created: 2026-05-15T16:20:00Z
priority: high
status: pending
msg-id: project-intelligence-20260515-wfd-spoke-cleanup
---
Security action taken by Command Session. Three WireGuard private keys were in
woodfine-fleet-deployment/fleet-infrastructure-leased/spoke-configs/ on the
public GitHub repo. Canonical cleaned via commit 13f11cc (mcorp-administrator).

The spoke-configs/ working-tree directory was removed from your cluster clone
as a security measure. Git will show the .conf files as unstaged deletions.

Action at your next WFD session:
  cd woodfine-fleet-deployment
  git status
  git merge --ff-only origin/main   (or rebase if local commits ahead)

Two guide files rescued to fleet-infrastructure-leased/ directly:
  guide-macos-endpoints.md + guide-peter-macbook.md

-- command@claude-code
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
re: AGENTS.md retro-add — 10 monorepo crates (batch)
created: 2026-05-14T22:34:22Z
priority: low
status: pending
---

Add `AGENTS.md` (vendor-neutral pointer file, `root-files-discipline.md` Tier 2) to the
following 10 crates in `vendor/pointsav-monorepo/`. Follow the pattern at
`system-ledger/AGENTS.md` or `moonshot-toolkit/AGENTS.md` — brief header, quick-reference
block pointing to `CLAUDE.md` at that directory, and workspace navigation links.

Crates missing AGENTS.md (confirmed 2026-05-14):
- `app-console-bookkeeper/`
- `app-console-bim/`
- `app-mediakit-knowledge/`
- `app-orchestration-bim/`
- `app-workplace-bim/`
- `app-workplace-memo/`
- `app-workplace-proforma/`
- `service-bim/`
- `service-extraction/`
- `service-slm/`

Commit staging-tier; push to staging mirrors. Stage 6 can batch with other commits.

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

During the 2026-05-13 startup scan, all 114 CORPUS_ files returned
`[SYS_HALT] Doorman response was not a valid entity JSON array`. Watcher fix is working
(each file attempted exactly once, no hang). But every extraction failed because Tier A
(local OLMo 7B) was the only backend and cannot produce a structured JSON array via
`/v1/chat/completions`. DataGraph has zero extractions from corpus since redeployment.

---

## 2. start-yoyo.sh line 340 — update_doorman_env on every Mode 1 success

**STATUS (2026-05-15 session): CODE COMPLETE — already unconditional in current code.**
`update_doorman_env` is called at line 388 regardless of zone change.

---

## 3. Universal AI Gateway — Sprint 0a (Anthropic Messages shim)

**STATUS (2026-05-15 session): DONE — `fdd1a223` + hardening in `7cd9ca61`.**
`POST /v1/messages` live on workspace VM. Sprint 0b (real streaming + on-demand boot) is next.

Full plan at `.agent/plans/universal-ai-gateway.md`. Sprint 0a is the immediate next
feature for the cluster. Implement `POST /v1/messages` Anthropic shim in
`crates/slm-doorman-server/src/http.rs` (~305 LOC).

---

## 4. Drafts outbound — notify project-editorial

11 drafts are staged at `.agent/drafts-outbound/` with status `draft-pending-language-pass`.
Send an outbox message to `project-editorial` flagging them for pickup.

| Draft | Type | Language |
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

---

## 5. Outbox — archive stale messages

The project-intelligence outbox has 5 messages accumulated from 2026-05-12 and 2026-05-13.
All have been read and actioned by Command Session. Archive them to `outbox-archive.md`.

---

## 6. Stage 6 — promote cluster branch to canonical main

**STATUS: DONE (2026-05-15 Command Session). 3 commits promoted to canonical.**

---

## 7. Yo-Yo — mask vllm.service before next boot

**STATUS: SUPERSEDED by item in newer inbox message. See europe-west4-a correction.**

---

## 8. Set SLM_YOYO_WEIGHTS_GCS_BUCKET in local-doorman.env

**STATUS: DONE — already set in `/etc/local-doorman/local-doorman.env`.**

---

## 9. Packer image rebuild + OLMo 3 32B weights upload (after item 7 complete)

**STATUS: OPERATOR-BLOCKED.** vllm.service mask on yoyo-tier-b-1 confirmed done (NEXT.md).
Packer rebuild + boot-disk snapshot are the remaining operator actions.

— command@claude-code
