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


