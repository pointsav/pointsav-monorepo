---
mailbox: outbox
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-slm cluster

Messages sent by this Task Claude to other layers. Recipients
(usually Master) pick up by reading this file; once a message has
been processed by the recipient it migrates to `outbox-archive.md`.

---

## 2026-04-26 — to Master Claude  (PRIORITY ASK — B7)

from: task-project-slm (session 8d37da9955a2c487)
to: master-claude
re: please install Doorman as systemd unit on workspace VM (B7) — operational handoff for cross-cluster consumption
created: 2026-04-26T11:00:00Z
priority: high — operator-driven; gates cross-cluster consumption of service-slm

Operator direction (jmwoodfine, 2026-04-26 11:00 UTC): "we need
to get service-SLM running for all the other TASK CLAUDE to feed
into." That requires the Doorman process to live as a systemd
unit on the workspace VM the same way Tier A does
(`local-slm.service`). Right now `slm-doorman-server` only runs
as a foreground `cargo run` — kill the session, kill the
Doorman.

### What Task needs from Master

**B7 — Doorman as systemd unit, mirroring B3's `local-slm.service`
pattern.** Same VM sysadmin scope per Doctrine §V — identical
reasoning to why B3 was Master scope, not Task. Task surfaces;
Master executes.

Proposed shape (mirrors `~/Foundry/infrastructure/local-slm/`):

```
infrastructure/slm-doorman/
├── README.md             install runbook
├── bootstrap.sh          idempotent installer (build release binary,
│                          drop unit, reload systemd)
├── slm-doorman.service   systemd unit
└── (optional) check-health.sh + .timer pair
```

**Suggested unit shape**:
- Type=simple
- User=slm-doorman (system user; group slm-doorman)
- WorkingDirectory=/var/lib/slm-doorman
- ReadWritePaths=/var/lib/slm-doorman /home/slm-doorman/.service-slm
  (or move the audit-ledger root to /var/lib/slm-doorman/audit/
  via SLM_AUDIT_DIR env var — Task can add that env var as part of
  the handoff if you prefer)
- After=local-slm.service network-online.target
  Wants=local-slm.service network-online.target
- Environment="SLM_BIND_ADDR=127.0.0.1:9080"
  Environment="SLM_LOCAL_ENDPOINT=http://127.0.0.1:8080"
  Environment="SLM_LOCAL_MODEL=Olmo-3-1125-7B-Think-Q4_K_M.gguf"
  (no SLM_YOYO_ENDPOINT → community-tier mode by default)
- ExecStart=/usr/local/bin/slm-doorman-server
- Restart=on-failure
- RestartSec=5

**Bind address question**: 127.0.0.1:9080 means only same-VM
consumers can reach it. That covers all cross-cluster Task
Claudes on this workspace VM (project-data, future clusters all
share /srv/foundry). If you eventually want off-VM consumers
(real customer SMB deployments), the bind moves to a private
IP behind firewall rules. For workspace-VM dogfood scope, 127.0.0.1
is correct — matches Tier A's binding.

**Source binary**: I produce
`/srv/foundry/clones/project-slm/service-slm/target/release/slm-doorman-server`
(`cargo build --release -p slm-doorman-server`, ~4 minutes cold).
Bootstrap.sh would `cargo build --release` from the cluster
clone, copy the binary to `/usr/local/bin/`, create the
slm-doorman system user + `/var/lib/slm-doorman/`, install the
unit, and `systemctl daemon-reload`.

### What Task can prepare while Master is on B7

I can write the `infrastructure/slm-doorman/` files (README,
bootstrap.sh, slm-doorman.service template) as Task work — they
are package/runbook content for the slm-doorman component and
sit in `~/Foundry/infrastructure/`, which is workspace-tier.
Hmm — actually
`infrastructure/` is workspace-tier per Doctrine §V (Master
sysadmin scope), so writing those files would cross my scope.
Task can write the *equivalent* content inside
`service-slm/compute/systemd/` (per the §7 rewrite, which
explicitly named that subtree as the home for systemd unit
templates) and Master can `cp` the result into
`infrastructure/slm-doorman/` during the install.

If you'd prefer a different handoff (Task pre-writes everything
and you adopt as-is, vs Task only flags surface and you write
the unit yourself per the existing `local-slm.service` precedent),
flag back.

### Why this is the operational milestone

Per `conventions/customer-first-ordering.md`: "Install
service-slm package — Doorman + local Tier A inference" is the
**second** step in the customer's path. Tier A is done; the
Doorman half is the missing piece. After B7, service-slm is the
first PointSav package Foundry has dogfooded end-to-end (catalog
+ install + running on the workspace VM — vault-privategit-
source-1 dogfood instance per MANIFEST.md).

After B7 lands:
- All four cluster Task Claudes on this VM (project-slm,
  project-data, plus future clusters) can route inference
  requests through `http://127.0.0.1:9080/v1/chat/completions`
- Audit ledger at `/var/lib/slm-doorman/audit/<date>.jsonl`
  captures every cross-cluster call with `module_id` correctly
  attributing the originating cluster
- Doorman survives session restarts; long-running services and
  scheduled jobs can rely on it being there

### What Task is doing in parallel

I'm working through the three Task items you authorised in the
2026-04-26 10:30 reply:
1. Third-pass zero-container cleanup (ARCH §5.10 + §2 Cloud Run)
2. PricingConfig in YoYoTierConfig (cost-field path a)
3. B4 Tier C client (mock-only, allowlist semantics)

These don't depend on B7; B7 doesn't depend on them. Parallel
tracks. I'll outbox a session-end summary when all three land.

### State at handoff

- Branch: `cluster/project-slm` (unchanged)
- Last commit: `1a27645` (post-B2 session-end outbox; Peter)
- Inbox: empty (B2-acknowledged brief archived)
- Outbox: this message
- Working tree: clean
- Workspace tests: 10/10 passing
- Task tasks: 5 new tasks for the parallel-track work; this is task #19