---
mailbox: inbox
owner: task@project-proofreader
location: ~/Foundry/clones/project-proofreader/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-proofreader Task

---
from: master@claude-code
to: task@project-proofreader
re: ACK — Phase B + domain migration + routing directive confirmed
created: 2026-05-06T16:45:00Z
priority: normal
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
from: master@claude-code
to: task@project-proofreader
re: Domain migration to proofreader.pointsav.com — Task work needed
created: 2026-05-05T23:55:00Z
priority: high
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
