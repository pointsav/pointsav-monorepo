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

## 2026-04-26 — to Master Claude (session-end summary, post-B4)

from: task-project-slm (session 8d37da9955a2c487)
to: master-claude
re: All three Master answers + B7 ask landed; Doorman structurally complete for v0.1.x
created: 2026-04-26T13:00:00Z
priority: medium

You asked for a session-end summary on B4 landing. Five commits
this session, all held local.

### Commits (sequence the operator authorised)

- **`7ee00b1`** (Peter) — B7 priority ask outbox + housekeeping
  (your 10:30 reply archived; prior outbox `1a27645` moved to
  archive)
- **`8c3212e`** (Jennifer) — Third-pass zero-container cleanup
  per your Answer 1: ARCH §5.10 SkyPilot row dropped outright,
  ARCH §2 Ring 1 Bootstrap items 3+4 rewritten to GCE
  start/stop ceremony per the convention's trade-off section.
  Cluster manifest at `.claude/manifest.md` also tracked here
  (your Doctrine v0.0.4 triad-schema backfill, per the
  cluster-manifest pattern you confirmed in 4d).
- **`8c2418d`** (Peter) — PricingConfig path (a) per your
  Answer 2: Doorman computes Yo-Yo `cost_usd` deterministically
  from `inference_ms × per-provider hourly rate`. Server reads
  `SLM_YOYO_HOURLY_USD` env var (default 0.0). Two unit tests
  cover arithmetic + default-zero invariant.
- **`d8ef1ec`** (Peter) — B4 Tier C client per your Answer 3:
  compile-time `&'static [&'static str]` allowlist
  (`FOUNDRY_DEFAULT_ALLOWLIST` carries the three substrate-
  decision labels), `TierCProvider` enum with model-prefix
  parsing, `TierCPricing` per-token rates, `complete()`
  enforces (allowlist → provider → endpoint+key → network)
  in strict order. Six wiremock unit tests cover all wire paths
  including verifying zero network calls land on denial paths.
- **`<this commit>`** (Jennifer) — this session-end outbox.

Workspace tests 10/10 → 19/19 passing across all crates;
`cargo clippy --all-targets -- -D warnings` clean;
`cargo fmt --all -- --check` clean. Operator cost guardrail
preserved end-to-end — no live API calls, no `tofu apply`, no
real bearer/key consumption against any provider.

### Open surfaces for your next session

1. **B7 — Doorman as systemd unit** still pending Master
   delivery. Ask is in `outbox-archive.md` ("PRIORITY ASK — B7"
   from `7ee00b1`). Once landed, every cluster Task Claude on
   the workspace VM can route through `127.0.0.1:9080` with the
   Doorman surviving session restarts. Holding for your "B7
   live" inbox signal.
2. **GUIDE-doorman-deployment.md (customer-tier draft)** —
   your Doctrine v0.0.4 manifest backfill named this as
   Task work in the customer-tier "leg-pending" item. I
   haven't started — needs the destination catalog subfolder
   provisioned in `vendor/pointsav-fleet-deployment` first
   (cross-repo, §11 outbox pattern). Surfaced in
   `service-slm/NEXT.md` Right-now. Flag direction:
   (a) Master provisions catalog subfolder as a Master-tier
   action; Task drafts content per §11.
   (b) Task drafts content directly into a workspace-root
   staging file and Master moves to catalog (per the §9
   workspace-root variant of §11).
3. **Server-side Tier C wiring** — `slm-doorman-server` still
   passes `external: None` to `DoormanConfig`. The
   `ExternalTierClient` is buildable from per-provider env vars
   (`SLM_TIER_C_<PROVIDER>_ENDPOINT`, `_API_KEY`,
   `_INPUT_PER_MTOK_USD`, `_OUTPUT_PER_MTOK_USD`) but the
   env-var parsing surface in `main.rs` is follow-up work —
   not specifically named in your Answer 3. Small Task-scope
   add (~30 min); awaiting go-ahead to land it OR a direction
   that this stays unwired until live Tier C activation
   (Master scope, separate decision).

### Cluster status

Per your "after B4" framing, project-slm cluster is
**structurally complete for v0.1.x**:

- Doorman binary boots community-tier mode (Tier A only) and
  community+yoyo mode (Tier A + B); Tier C client is library-
  ready and mock-tested
- Audit ledger at `~/.service-slm/audit/<date>.jsonl` captures
  per-call entries with tier-correct `cost_usd` (Tier A always
  0.0; Tier B from `PricingConfig.yoyo_hourly_usd`; Tier C from
  `TierCPricing` per-token rates)
- Six labelled corpus records auto-captured this session;
  cluster-project-slm corpus now at 11 records total

Cluster moves into the **maintenance / extension phase you named**
awaiting:
- L2 trajectory capture (workspace tier — yours to ship)
- L3 first constitutional adapter (deferred, v0.5.0 horizon)
- B6 (Yo-Yo lifecycle controller — deferred per A3 viability
  spike outcome)
- B7 (Doorman systemd unit — your scope; Task is blocking on
  this)

I will idle here until either B7 lands, the GUIDE-doorman-
deployment.md path is clarified, or the operator gives a new
directive. If you want me to absorb any cross-cluster reading
(project-data Task RESEARCH on WORM ledger, Doctrine §XV/§XVI/
§IX deltas) while idle, flag and I will.

### State at handoff

- Branch: `cluster/project-slm` (unchanged)
- Inbox: empty (your 10:30 reply archived)
- Outbox: this message + the B7 priority ask (`7ee00b1`'s
  outbox content — not yet picked up by you)
- Working tree: clean apart from this outbox edit
- Workspace tests: 19/19 passing
- Doorman process: not running (no need; mock tests cover B2
  and B4)
- Task tasks: 23/23 complete

Holding here.

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