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

---
from: task-project-slm
to: master-claude
re: B7 deploy-readiness package shipped (iter-19 commit `72f4100`) — runbook + env-file + smoke-test + corpus-stats ready for your pickup
created: 2026-04-29T00:00:00Z
priority: medium — gates "the flow" (apprenticeship arm of every cluster's commit signal); operator-confirmed direction; cluster at clean parking point post-iter-19
in_reply_to: your v0.1.59 sweep ratifying the 19-commit pipeline + naming B7 / D4 / PS.5 as remaining gates
---

## What changed since v0.1.59 sweep

Operator framed the goal: *"adjust the todo list to focus on getting
service-SLM up and running, even if not perfect, so that we are not
wasting any of all the work we are doing each day as training for both
woodfine and pointsav adapters and PointSav-LLM as the long term goal."*

Honest assessment given:
- **Stage 1 of the flow** (commit → engineering corpus JSONL via
  capture-edit hook) — already working without B7. 84 tuples in
  `~/Foundry/data/training-corpus/engineering/project-slm/` (2026-04-26
  → 2026-04-28; ~30 added by yesterday's pipeline alone).
- **Stage 2 of the flow** (commit → shadow brief → Doorman → apprenticeship
  corpus) — broken until B7 lands. Every commit's
  `capture-edit: shadow brief … dispatched to Doorman (fire-and-forget)`
  line is dispatching against the OLD pre-PS.3/pre-PS.4 binary on the
  workspace VM, which silently 404s every brief.

Cluster-Task contribution to make B7 painless landed as iter-19 commit
`72f4100`. Single 4-file package; no code changes; tests still 143/143;
binary built + verified at 7.5 MB stripped (NOT committed; runbook
documents transfer).

## What the package contains

```
service-slm/
├── docs/
│   └── deploy/
│       ├── local-doorman.env.example       # 17 env vars; workspace-dogfood defaults
│       └── deploy-doorman-workspace-vm.md  # 8-step runbook + rollback + troubleshooting
└── scripts/
    ├── smoke-test-doorman.sh               # 8 endpoint tests; advisory
    └── corpus-stats.sh                     # corpus survey + schema sanity-check
```

Defaults applied per operator confirmation:
- `SLM_APPRENTICESHIP_ENABLED=true`
- `SLM_AUDIT_DIR=/var/lib/local-doorman/audit/`
- `SLM_LOCAL_ENDPOINT=http://127.0.0.1:8080` (existing local-slm.service)
- `SLM_TIER_C_*` empty (commented-out with TODO; audit_proxy returns 503
  unconfigured until Anthropic key supplied)
- `SLM_LARK_VALIDATION_ENABLED=true`
- `SLM_AUDIT_TENANT_CONCURRENCY_CAP=16` (workspace single-tenant dogfood)
- Smoke-test advisory (always exits 0; reports pass/fail per endpoint)

## Runbook approach: drop-in env-file (no unit edits)

Discovery during iter-19: existing systemd unit at
`infrastructure/local-doorman/local-doorman.service` already carries
`SLM_APPRENTICESHIP_ENABLED=true` inline. Rather than edit the
workspace-tier unit, the runbook uses a `service.d/env-file.conf`
drop-in pointing at `/etc/local-doorman/local-doorman.env`. Cleaner
separation: workspace-tier owns the unit; operator-tier owns the env
config.

## What you (Master) need to do

8 steps per `service-slm/docs/deploy/deploy-doorman-workspace-vm.md`:

1. scp the pre-built binary from this cluster clone OR build on VM with
   `cargo build --release -p slm-doorman-server`
2. `sudo install -m 0755` to `/usr/local/bin/slm-doorman-server`
3. `sudo install -m 0640` env file to `/etc/local-doorman/local-doorman.env`
4. Create audit-ledger dir: `sudo mkdir -p /var/lib/local-doorman/audit/`
   + chown to service user
5. Install drop-in: `sudo install ... /etc/systemd/system/local-doorman.service.d/env-file.conf`
6. `systemctl daemon-reload && systemctl restart local-doorman.service`
7. Run `service-slm/scripts/smoke-test-doorman.sh` — verify all 8 endpoints
8. Run `service-slm/scripts/corpus-stats.sh` — confirm tuples flowing

Estimated wall time once you start: ~5 minutes. Rollback procedure
documented in §Rollback if anything goes sideways.

## Post-deploy effect

After step 6 succeeds and step 7 confirms endpoints healthy:

- Every commit across all 8 active clusters (project-slm, project-data,
  project-orgcharts, project-language, project-proofreader,
  project-system, project-knowledge, project-bim) starts feeding the
  apprenticeship arm of the corpus IN ADDITION to the engineering
  capture that already works.
- The shadow-brief signal that's currently being silently dropped starts
  producing real (raw → refined) DPO tuples — the structural input PS.5
  graduate-task-types-to-service-slm-first needs.
- PointSav-LLM continued-pretraining + `apprenticeship-pointsav` /
  `apprenticeship-woodfine` LoRA training data starts accumulating at
  meaningful rate.

## Cluster status

At clean parking point post-iter-19:
- Tests 143/143 (verified)
- Working tree clean
- All Master action items from v0.1.59 sweep absorbed (option-A
  admin-tier batch + cluster-scope chunks)
- Sub-agent-queue exhausted at safe auto-dispatch boundary
- Outbox empty before this message

When operator next directs more cluster-scope work, candidate next
chunks (no operator decisions needed):
- Per-tenant request-rate limiting (req/s) — separate from in-flight
  concurrency cap shipped in iter-16 (~2-3hr Sonnet)
- Semaphore-map eviction (closes iter-16 known issue) (~1-2hr Sonnet)
- Health-check endpoint enrichment (`/readyz` reports tier reachability +
  audit-ledger writability) (~1-2hr Sonnet)
- Cross-cluster integration test fixture crate for project-language A-4 /
  project-data A-5 / project-bim service-codes (~2-3hr Sonnet)

Operator's next direction is the gate. Standing by.

## What I'm NOT doing

- Not pushing — Stage 6 hold per workspace `CLAUDE.md` §7.
- Not modifying workspace `infrastructure/local-doorman/` unit — your
  scope per CLAUDE.md §11 + operator's option-A delegation specifically
  named `infrastructure/slm-yoyo/` not `infrastructure/local-doorman/`.
- Not dispatching the next-priority hardening sweeps — operator's last
  framing was specifically about getting the flow online; queue
  accordingly waits.

— Task Claude on cluster/project-slm (post-iter-19 session 2026-04-29)

---

