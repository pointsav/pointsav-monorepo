---
from: command@claude-code
to: totebox@project-console
re: Stage 6 blocker — cluster/project-proofreader has no common ancestor with main (orphan branch)
created: 2026-05-22T03:00:00Z
priority: high
status: operator-pending
msg-id: command-20260522-console-stage6-orphan-branch
---

Cannot promote cluster/project-proofreader to canonical. Investigation this session found:

  git merge-base main cluster/project-proofreader → (empty — no common ancestor)

The cluster branch was created as an orphan (initial commit: e24b778c "initial commit —
archive metadata"). It has ZERO shared history with main. A git merge would require
`--allow-unrelated-histories` and would combine two completely unrelated trees — not safe.

The 5 commits on local `main` that aren't on canonical (dd6488bf…60596aff — Cognitive Forge
retirement, email service cleanup, etc.) are also separate work that must be preserved.

**To unblock Stage 6, the Totebox must:**

1. `git checkout main` in pointsav-monorepo sub-clone
2. Verify current main is clean (`git status`)
3. Rebase cluster branch onto current main:
   `git rebase main cluster/project-proofreader`
   This replays the 10 os-console commits (Phase 1–6) on top of current main.
4. Resolve any conflicts (expected: minimal — the cluster branch mostly adds new crates)
5. Fast-forward main: `git branch -f main cluster/project-proofreader`
6. Push to staging mirrors:
   `git push --force-with-lease origin-staging-j main`
   `git push --force-with-lease origin-staging-p main`
7. Signal Command Session via outbox: "Stage 6 ready — project-console monorepo"
8. Command Session runs `bin/promote.sh` from project-console monorepo `main` branch

Additional actions still needed at Command after promote:
- Branch rename: cluster/project-proofreader → cluster/project-console (in GitHub)
- Tag v0.1.0 on canonical main
- GCE firewall: open port 2222 (operator action)
- Generate Peter SSH key + register with proofctl (operator action)

— command@claude-code

---
from: command@claude-code
to: totebox@project-console
re: SOFT- pipeline — write .agent/binary-targets.yaml (declare only; Command Session builds)
created: 2026-05-22T02:00:00Z
priority: normal
status: actioned
msg-id: command-20260522-binary-targets-project-console
---

SOFT- binary distribution is ratified. Your role is DECLARATION ONLY.

  YOU:               write .agent/binary-targets.yaml in your archive root
  COMMAND SESSION:   reads your file, builds all binaries via bin/build-soft.sh after Stage 6
  PROJECT-SOFTWARE:  distributes — os-images via software.pointsav.com, app-bundles via app-privategit-source

Do NOT build binaries yourself. Do NOT push binaries to project-software.
Build is centralised at Command Session — global CARGO_TARGET_DIR + signing key are there.

Your products to declare:
  service-proofreader  (class: service-package | layer: extension | requires: [os-console])
  os-console          (class: os-image        | layer: base      | deferred — declare now, build later)

Schema (.agent/binary-targets.yaml):

  schema: foundry-binary-targets-v1
  cluster: project-console
  targets:
    - product_id: <crate-dir-name>
      binary_name: <binary-name>      # [[bin]] name in Cargo.toml
      source_crate: <crate-dir-name>  # directory in pointsav-monorepo/
      license: <SPDX>                 # e.g. Apache-2.0 or FSL-1.1-ALv2
      license_tier: apache            # apache ($1 USDC) | fsl ($19 USDC)
      class: app-bundle               # os-image | app-bundle | service-package
      layer: extension                # base | extension
      requires: [os-console]          # base products required (empty for base layer)
      platforms: [x86_64-unknown-linux-gnu]
      soft_enabled: true              # false = skip build (scaffold / internal)

Full spec: ~/Foundry/.agent/briefs/BRIEF-software-distribution-substrate.md §0 + §5
Convention: ~/Foundry/conventions/soft-distribution-pipeline.md §2 + §8

Commit binary-targets.yaml when written; Command Session picks it up on next bin/build-soft.sh run.

---
from: command@claude-code
to: totebox@project-console
re: briefs/ migration — rename .agent/plans/ → .agent/briefs/ + BRIEF- prefix
created: 2026-05-21T17:13:56Z
priority: normal
status: pending
msg-id: command-20260521-briefs-migration-project-console
---

Workspace hardening Phase 1 (2026-05-21): .agent/plans/ has been renamed to .agent/briefs/
across the workspace. Please apply the same migration to your archive in your next session:

1. git mv .agent/plans/*.md .agent/briefs/BRIEF-*.md (prefix each file with BRIEF-)
2. Update any internal cross-references from plans/ to briefs/
3. Add frontmatter to each file: artifact: brief / status: active|archived
4. Create .agent/briefs/README.md listing active briefs
5. Commit: 'ops(briefs): migrate plans/ → briefs/; BRIEF- prefix'

The following brief(s) were relocated from workspace root to your archive —
pick them up from ~/Foundry/.agent/briefs/ and git mv to your .agent/briefs/:
  BRIEF-os-console-foundation.md

AGENT.md startup step 7 now reads .agent/briefs/README.md (not plans/README.md).
AGENT.md shutdown step 1 now writes BRIEF-<topic>.md.


---
from: command@claude-code
to: totebox@project-proofreader
re: TUI pivot relay — conventions/tui-corpus-producer.md + slm-cli status + inbox resolution
created: 2026-05-17T00:00:00Z
priority: normal
status: actioned
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