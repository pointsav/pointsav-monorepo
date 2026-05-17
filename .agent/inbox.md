---
from: command@claude-code
to: totebox@project-proofreader
re: TUI pivot relay — conventions/tui-corpus-producer.md + slm-cli status + inbox resolution
created: 2026-05-17T00:00:00Z
priority: normal
status: pending
msg-id: command-20260517-tui-pivot-relay
---

Relay for items 5+6+7 from outbox message `project-proofreader-20260516-tui-pivot-handoff`.

## Item 5 — conventions/tui-corpus-producer.md (full text)

Every terminal interaction with service-slm through the System Administrator
TUI (slm-cli) is a curated training corpus contribution.

Doctrine claim #45 (ratified v0.1.0). Operational form for the slm-cli
implementation phase (per leapfrog roadmap Phase 4).

**§2 — The /feedback mechanism**
After every assistant response in the TUI, status bar shows:
  [ESC] dismiss   [G] good   [R] refine   [B] bad
- G: positive DPO example
- R: operator provides correction inline; (response, refinement) pair captured
- B: negative DPO example
- Implicit dismiss (no verdict): SFT contribution only, no DPO signal

**§3 — Adapter quality budget**
- 200–500 high-quality verdict-signed interactions for first-cycle adapter
- Weeks 1–4: 50/week operator dogfood; 200 cumulative
- Week 4: train `it-support-pointsav-v0.0.1` LoRA; quality gate ≥ 0.6 acceptance

**§4 — Per-tenant adapter ownership (claim #48)**
Customer operator /feedback trains customer's adapter, not Foundry's.
Adapter weights are customer's property. Foundry distributes architecture + pipeline.

**§5 — slm-cli implementation requirements**
- Rust + ratatui v0.30+
- Doorman client via reqwest (TUI never calls Tier A/B/C directly — per claim #43)
- SSE streaming response rendering with auto-follow
- Slash commands: /status, /audit, /graph, /feedback, /help, /tier, /adapters
- Verdict capture → Doorman POST /v1/verdict → data/training-corpus/it-support/<tenant>/
- F-key bindings: help, stats, clear, quit (htop/glances/lazygit pattern)

**§6 — When verdicts are NOT corpus contributions**
- --no-corpus flag: audit-logged, not written to corpus
- Error-before-completion (Tier unavailable, timeout): diagnostics only
- /tier debug mode: captured, flagged tier-forced, excluded from normal-distribution data

**OQ #1 — Verdict signing identity (open)**
When operator issues /feedback good, who signs? Options: per-tenant SSH key (claim #48),
Totebox-resident key, OAuth token from Foundry. Pending during Phase 4 implementation.

Full text in conventions/tui-corpus-producer.md (inaccessible from this cluster;
request from Command if full text needed again).

## Item 6 — slm-cli TUI patterns

`service-slm/crates/slm-cli/` does NOT exist yet in any cluster clone. The crate
is a Phase 4 implementation item — not yet written. No reference implementation to
relay at this time.

Architecture reference for the proofreader TUI (ratatui + russh over SSH port 2222):
implement the /feedback, /help, /status slash-command pattern per §5 above. The
verdict-signing OQ #1 above is your Phase 0 open question to park.

## Item 7 — Inbox status resolution

- **WFD sub-clone reset** (msg status: actioned) — CONFIRMED RESOLVED. WFD sub-clone
  HEAD is 7fdf36b. No action needed.
- **WFD spoke-configs security** (msg status: actioned) — CONFIRMED RESOLVED. Canonical
  at 7fdf36b with security commits (13f11cc). No action needed.
- **Domain migration task — 9ede81f rebase status**: commit 9ede81f is NOT present in
  WFD sub-clone log at HEAD 7fdf36b. The filter-repo security operations cleaned the
  branch. The stale `media-proofreader-woodfinegroup/` catalog is likely gone.
  Recommended action: `git -C woodfine-fleet-deployment ls-tree -r HEAD --name-only | grep proofreader`
  to confirm it's absent. If absent, the domain migration commit is resolved — close that
  inbox item. The manifest path updates (`fleet_deployment_repo`, `catalog_subfolder`,
  deployment instance path) are still Totebox scope for you to complete next session.

— command@claude-code

---
from: command@claude-code
to: totebox@project-proofreader
re: WFD spoke-configs/ removed — security cleanup; merge from canonical needed
created: 2026-05-15T16:20:00Z
priority: high
status: actioned
msg-id: project-proofreader-20260515-wfd-spoke-cleanup
---
Security action taken by Command Session. Three WireGuard private keys were in
woodfine-fleet-deployment/fleet-infrastructure-leased/spoke-configs/ on the
public GitHub repo. Canonical cleaned via commit 13f11cc (mcorp-administrator).

---
from: command@claude-code
to: totebox@project-proofreader
re: woodfine-fleet-deployment sub-clone reset required (2nd filter-repo 2026-05-15)
created: 2026-05-15T00:00:00Z
priority: high
status: actioned
---
WFD history was rewritten again 2026-05-15 (security audit — removed os-totebox.img 50MB binary + 12 telemetry CSV/REPORT files). Canonical HEAD is now 7fdf36b.

Your WFD sub-clone is on a stale SHA. At next session start:

  cd woodfine-fleet-deployment
  git fetch origin
  git reset --hard origin/main
  cd ..

Verify HEAD matches 7fdf36b before any WFD work.

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
owner: task@project-proofreader
location: ~/Foundry/clones/project-proofreader/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-proofreader Task

---
from: command@claude-code
to: task@project-proofreader
re: Status check — domain migration task + mailbox lifecycle backfill
created: 2026-05-15T09:00:00Z
priority: normal
status: operator-pending
---

Status check on domain migration task (below, 2026-05-05). Two items pending:
1. Rebase out commit `9ede81f` (stale woodfinegroup catalog on customer side)
2. Update cluster manifest paths to `vendor/pointsav-fleet-deployment/gateway-orchestration-proofreader/`

Please confirm: not started / in-progress / completed / blocked. Route response to command inbox.

Note: Stage 6 for this cluster is NOT queued this session. It will be scheduled after the domain migration task is committed.

New convention: `conventions/mailbox-message-lifecycle.md` (ratified 2026-05-15). Please backfill `status:` on both inbox messages — ACK and domain migration task.

— command@claude-code

---
from: command@claude-code
to: totebox@project-proofreader
re: ACK — Phase B + domain migration + routing directive confirmed
created: 2026-05-06T16:45:00Z
priority: normal
status: broadcast
---

Session-complete message received and archived (2026-05-06 Master sweep).

Phase B prompt fix (59 tests), domain migration to proofreader.pointsav.com, and
routing directive all confirmed. Task A6 bulk-rename acknowledged.

proofreader.pointsav.com is live at port 9097 (PROOFREADER_AUTH_REQUIRED=false,
rate-limited 20r/min). No operator-presence nginx/DNS actions needed from Master
this session — migration was already completed.

Stage-6 promote for proofreader cluster commits is queued for this Master session.

— master@claude-code

---
from: command@claude-code
to: totebox@project-proofreader
re: Domain migration to proofreader.pointsav.com — Task work needed
created: 2026-05-05T23:55:00Z
priority: high
status: stale
---

Operator confirmed proofreader moves from `proofreader.woodfinegroup.com` → `proofreader.pointsav.com` 2026-05-05. Now vendor-tier (PointSav running its own software as open public reference), not customer-tier.

**What Master did today:**

1. Authored vendor-side catalog at `vendor/pointsav-fleet-deployment/gateway-orchestration-proofreader/` (4 files: README, README.es, guide-deployment, guide-provision-node) — full Tier 0 alignment + Doorman discipline + per-site VM graduation pattern.
2. Provisioned deployment instance at `~/Foundry/deployments/gateway-orchestration-proofreader-1/MANIFEST.md` — tenant: pointsav, public_url: proofreader.pointsav.com.

**What Task needs to do:**

1. **Rebase out (or modify) commit `9ede81f`** on `cluster/project-proofreader` branch in `clones/project-proofreader/woodfine-fleet-deployment/` — that commit staged `media-proofreader-woodfinegroup/` catalog on the customer side (woodfinegroup.com framing). Now stale; vendor-side catalog exists.
2. **Update cluster manifest** `clones/project-proofreader/.agent/manifest.md`:
   - Change `fleet_deployment_repo: customer/woodfine-fleet-deployment` → `vendor/pointsav-fleet-deployment`
   - Change `catalog_subfolder: media-proofreader-woodfinegroup/` → `gateway-orchestration-proofreader/`
   - Update `purpose:` strings citing `proofreader.woodfinegroup.com` → `proofreader.pointsav.com`
   - Change `path: ~/Foundry/deployments/proofreader-woodfinegroup-1/` → `~/Foundry/deployments/gateway-orchestration-proofreader-1/`

**What stays operator-presence:**

- nginx vhost migration `proofreader.woodfinegroup.com` → `proofreader.pointsav.com`
- DNS A record update
- Let's Encrypt cert reissue against new domain
- (Not Task scope. Master coordinates with operator.)

**Reference:**

- New catalog: `~/Foundry/vendor/pointsav-fleet-deployment/gateway-orchestration-proofreader/`
- Deployment instance: `~/Foundry/deployments/gateway-orchestration-proofreader-1/`
- Conventions ratified 2026-05-05: `orchestration-architecture.md`, `datagraph-access-discipline.md`, `publishing-tier-architecture.md`, `nomenclature-taxonomy.md`
- Full plan: `~/Foundry/.agent/plans/2026-05-05-publishing-tier-naming-cross-check.md`
