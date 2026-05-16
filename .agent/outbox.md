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
status: pending
---

Two commits landed this session:

1. `3e873ea4` — dispatch-clock fix: `last_yoyo_dispatch` AtomicU64 in `AppState` prevents idle monitor from misfiring when the 5-min poll granularity catches a slot=0 between-request gap. The monitor now rewinds `last_active` to the most recent Tier B dispatch on every cycle.

2. `b93f745b` — preemption auto-restart: when `/metrics` is unreachable and `stop_sent=false`, the idle monitor calls GCP `instances.start` automatically. Rolling `RestartBudget` caps at 3/hr. 90-second boot-grace window suppresses the next poll. `parse_metric` prefix-collision bug fixed (was matching `llama_active_slots_total_avg` when key was `llama_active_slots_total`). 22 new tests; total 198/198.

Both 30-minute test loops completed via `/v1/messages` Anthropic shim:
- Trainer: finished
- Graph: 318 requests / 30 min — GCP preempted mid-test; recovered manually; auto-restart now handles this in production.

VM is TERMINATED (`europe-west4-a`, `woodfine-node-gcp-free`). No billing.

Stage 6 pending for both commits — local only.


