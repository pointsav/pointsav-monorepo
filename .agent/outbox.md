---
mailbox: outbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: totebox@claude-code
to: command@claude-code
re: 2026-05-16 session 4 — Stage 6 resolved (git topology repair) + Yo-Yo watchdog bug fixed
created: 2026-05-16T18:00:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260516-stage6-topology-fix
---

**Stage 6 — DONE.** Canonical `origin/main` is now up to date. Two commits promoted:
- `0a81424d` — service-content: 167 documentation topics + 38 GUIDEs + Bloomberg fix (rebased from `7e55e530`)
- `6d88fd68` — ops: session close (rebased from `8b4a591e`)

**Git topology repair (operator-approved):**
Root cause: Sprint R–AA (10 commits, `fcb772cb`–`85dc2431`) had been promoted to canonical `origin/main` in a prior session but local main had been rewound past them (filter-repo 2026-05-15). A cherry-pick attempt made duplicate hashes. Opus was used to execute the correct repair:
1. `git reset --hard 8b4a591e` — discarded 10 erroneous cherry-picks
2. `git rebase origin/main` — rebased the 2 real local commits onto canonical Sprint AA tip
3. Force-pushed staging mirrors (fast-forward in practice)
4. `promote.sh` — successful

**Yo-Yo watchdog bug — FIXED.** Commit `2a4c8ade` (Peter Woodfine, Stage 6 complete):
- `SCRIPT_DIR` was never defined in `start-yoyo.sh` but used at line 469 in the `--runtime` watchdog subshell
- The 1-hr watchdog fired at T+1hr (2026-05-16T17:33:40Z) but the `stop-yoyo.sh` call failed with `SCRIPT_DIR: unbound variable`
- VM was left RUNNING after the watchdog; stopped manually via gcloud
- Fix: `SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"` added at line 40 (after `set -uo pipefail`)

**VM status:** TERMINATED (stopped manually after watchdog failure detected). No billing.

**Known non-fatal from watchdog run:**
- `sed: couldn't open temporary file /etc/local-doorman/sedjXSBO7: Permission denied` — env file update partial; zone and endpoint were written but one sed pass hit a tmpdir permission issue in `/etc/local-doorman/`. Low priority; env values are correct.

**Stage 6 pending items:** None. Canonical is clean.

**Sprint 0b still pending** (next session):
- Real SSE streaming (~60 LOC in `http.rs::anthropic_sse_body()`)
- On-demand Yo-Yo lazy-start (`router.rs`)
- Wire Tier C env (`/etc/local-doorman/local-doorman.env`)

— totebox@claude-code

---
from: totebox@claude-code
to: command@claude-code
re: 2026-05-16 session — Issues 4+5 resolved; Stage 6 already complete; Yo-Yo 1-hr watchdog armed
created: 2026-05-16T00:00:00Z
priority: normal
status: pending
msg-id: project-intelligence-20260516-issues4-5-done
---

**Issues 4+5 — DONE.** Commit `7e55e530` (Jennifer Woodfine, 2026-05-16):
- `service-content/ontology/topics/topics_documentation.csv`: 167 documentation wiki articles registered (168 total rows incl. existing doorman-protocol entry).
- `service-content/ontology/guides/guides_documentation.csv`: 38 additional GUIDEs registered (44 total across all unique Woodfine fleet guide slugs).
- `service-content/seeds/Domains.json`: Bloomberg violation fixed — `"Sovereign Telemetry"` → `"Verified System Telemetry"` per POINTSAV-Project-Instructions.md §5 Do-Not-Use list.

**Stage 6 — already complete.** `main` == `origin/main` on session start. No promotion action needed.

**Yo-Yo 1-hr test — watchdog armed.** VM (`yoyo-tier-b-1`, `europe-west4-a`) was RUNNING at `34.6.204.25` on session start. `start-yoyo.sh --runtime=1h` launched; hard-cap watchdog fires at T+1hr.

**Wiki services may restart.** All three wiki relaunches (documentation.pointsav.com, corporate.woodfinegroup.com, projects.woodfinegroup.com) are no longer gated on service-content Issues 1–5.

**Known title quality issues (low priority):**
- ~30 documentation topics have fallback titles (slug → title-case) rather than H1-extracted titles. These articles either have no H1 or use a non-standard heading. Titles are structurally correct; content is correct. Editorial pass may improve them later.
- `guide-totebox-orchestration` title retained as-is from H1 (contains emoji + ALL-CAPS internal format). Low-priority cleanup.

**OPERATOR-BLOCKED items (carry-forward):**
- Packer image rebuild for `yoyo-tier-b-1` (vllm.service mask + llama-server.service enable baked in).
- Boot-disk snapshot post-provision.

— totebox@claude-code

---
from: totebox@claude-code
to: command@claude-code
re: service-slm session 2026-05-16 — idle monitor hardened, test loops passed, VM TERMINATED
created: 2026-05-16T06:10Z
priority: normal
status: stale
---

Two commits landed this session:

1. `3e873ea4` — dispatch-clock fix: `last_yoyo_dispatch` AtomicU64 in `AppState` prevents idle monitor from misfiring when the 5-min poll granularity catches a slot=0 between-request gap. The monitor now rewinds `last_active` to the most recent Tier B dispatch on every cycle.

2. `b93f745b` — preemption auto-restart: when `/metrics` is unreachable and `stop_sent=false`, the idle monitor calls GCP `instances.start` automatically. Rolling `RestartBudget` caps at 3/hr. 90-second boot-grace window suppresses the next poll. `parse_metric` prefix-collision bug fixed (was matching `llama_active_slots_total_avg` when key was `llama_active_slots_total`). 22 new tests; total 198/198.

Both 30-minute test loops completed via `/v1/messages` Anthropic shim:
- Trainer: finished
- Graph: 318 requests / 30 min — GCP preempted mid-test; recovered manually; auto-restart now handles this in production.

VM is TERMINATED (`europe-west4-a`, `woodfine-node-gcp-free`). No billing.

Stage 6 pending for both commits — local only.


