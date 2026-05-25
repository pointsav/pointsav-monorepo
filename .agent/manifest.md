---
schema: foundry-cluster-manifest-v1
cluster_name: project-console
cluster_branch: cluster/project-console
created: 2026-04-27
renamed: 2026-05-20  # project-proofreader → project-console; scope broadened to full Console OS family
state: active
slm_endpoint: http://localhost:8011
module_id: console

# Tetrad upgrade 2026-04-28 per Master inbox broadcast (Doctrine
# claim #37 / v0.0.10): triad → tetrad with wiki leg added as the
# fourth structural deliverable. Existing three legs unchanged.
tetrad:
  vendor:
    - repo: pointsav-monorepo
      path: pointsav-monorepo/
      upstream: vendor/pointsav-monorepo
      focus: os-console/ + app-console-proofreader/ (LIVE at proofreader.pointsav.com) + app-console-gis/ (Reserved; F8) + app-console-slm/ (Reserved; F9) + app-console-system/ (Reserved; F11)
      status: active (proofreader LIVE; app-console-proofreader at eb0ffd3)
  customer:
    - fleet_deployment_repo: vendor/pointsav-fleet-deployment
      catalog_subfolders:
        - (NEW) gateway-orchestration-proofreader/
      tenant: pointsav
      purpose: Vendor-tier deployment instance home (PointSav running its own software as public reference); hosts the proofreader at https://proofreader.pointsav.com
      status: active (Phase 8 catalog landed 2026-05-05 — bilingual README + MANIFEST + guide-deployment + guide-provision-node)
  deployment:
    - shape: new-instance
      path: ~/Foundry/deployments/gateway-orchestration-proofreader-1/
      tenant: pointsav
      purpose: production deployment of proofreader.pointsav.com on workspace VM
      runtime_artifacts:
        - /usr/local/bin/service-proofreader (LIVE — Round 7 PP.1 binary at eb0ffd3, redeployed 2026-04-28T00:17:38Z)
        - /usr/local/bin/app-console-proofreader (LIVE — Round 5 binary; awaiting redeploy for Round 6 UX + verdict)
        - /etc/systemd/system/local-proofreader.service (active)
        - /etc/systemd/system/local-proofreader-console.service (active)
        - /etc/nginx/sites-enabled/proofreader.pointsav.com (active — :443 vhost enabled)
        - LanguageTool 6.6 in Docker (companion at 127.0.0.1:8010, live)
        - service-slm Doorman with --reasoning-format deepseek (workspace tier; live since 2026-04-28T00:19:46Z)
        - /etc/letsencrypt/live/proofreader.pointsav.com/ (live)
      status: active (HTTPS LIVE on https://proofreader.pointsav.com/ since 2026-05-03)
  wiki:
    - repo: vendor/content-wiki-documentation
      drafts_via: clones/project-console/.agent/drafts-outbound/
      gateway: project-language Task
      planned_topics:
        - topic-language-protocol-substrate.md (architecture TOPIC — explicit-protocol-selection vs. auto-detection rationale per Cornell anti-homogenization study; three-stage pipeline composition) — SKELETON STAGED 2026-04-28
        - topic-editorial-pipeline-three-stages.md (design-decision TOPIC — banned-vocab + LanguageTool 6.6 + Doorman generative pass; flag-don't-rewrite default; Apply-all + verdict callback closing the apprenticeship loop) — SKELETON STAGED 2026-04-28 (Sonnet Brief #1)
        - topic-customer-tier-catalog-pattern.md (engineering TOPIC — catalog/instance distinction at the Customer tier; how gateway-orchestration-proofreader demonstrates Doctrine §VII Tier-0 fleet-node deployment) — SKELETON STAGED 2026-04-28 (Sonnet Brief #1)
        # NEW candidates surfaced by Brief #1's Sonnet sub-agent — natural completion set:
        - topic-proofreader-apprenticeship-corpus.md (operational TOPIC — operator verdicts (accepted/rejected/edited) → DPO event pairs → continued-pretraining; the prose-edit task type from claim #35 §7A in practice) — proposed; substance follows
        - topic-banned-vocabulary-governance.md (governance TOPIC — per-family vocabulary list maintenance, versioning, propagation across tenants; Stage 1 deterministic substrate dependencies) — proposed; substance follows
      status: drafted (3 of 5 skeletons staged; substance for all 5 follows in milestone N+1; project-language sweeps via bin/draft-sweep.sh)

clones:
  - repo: pointsav-monorepo
    role: primary
    path: pointsav-monorepo/
    upstream: vendor/pointsav-monorepo
    focus: os-console/ + app-console-proofreader/ + app-console-gis/ + app-console-slm/ + app-console-system/
  - repo: woodfine-fleet-deployment
    role: sibling
    path: woodfine-fleet-deployment/
    upstream: customer/woodfine-fleet-deployment
    focus: customer-tier catalog entries for console-OS deployments
  - repo: content-wiki-documentation
    role: wiki
    path: content-wiki-documentation/
    upstream: vendor/content-wiki-documentation
    focus: topic-language-protocol-substrate.md + topic-editorial-pipeline-three-stages.md + topic-customer-tier-catalog-pattern.md (plus 2 candidates)

trajectory_capture: enabled (L1 capture-edit hook installed in both sub-clones at provisioning)

adapter_routing:
  trains:
    - cluster-project-console      # own cluster adapter — Console OS development skill
    - engineering-pointsav         # Vendor engineering corpus
    - tenant-pointsav              # PointSav-voice editorial work captured via the proofreader
  consumes:
    - constitutional-doctrine      # always
    - engineering-pointsav         # always — Vendor knowledge
    - cluster-project-console      # own cluster context
    - role-task                    # current role
    - tenant-pointsav | tenant-woodfine  # composed at request time per which tenant the editorial work is for

apprenticeship_task_types:
  # Per-template, per-tenant promotion ledger entries (claim #32 generalized to editorial work)
  - proofread-prose-readme        # paste README → service-proofreader → diff → user accepts
  - proofread-prose-topic         # same, TOPIC template
  - proofread-prose-guide         # same, GUIDE template
  - proofread-prose-memo          # same, MEMO template
  - proofread-prose-architecture  # same, ARCHITECTURE template
  - proofread-prose-inventory     # same, INVENTORY template
  - proofread-prose-license-explainer
  - proofread-comms-email
  - proofread-comms-chat
  - proofread-comms-ticket-comment
  - proofread-comms-meeting-notes
  - proofread-translate-en-es     # English → Spanish
  - proofread-translate-en-fr     # English → French (when needed)
  # LEGAL templates volume-gated; not in initial set
---

# Cluster manifest — project-console

Multi-clone N=3 cluster (seventh cluster overall; renamed from project-proofreader 2026-05-20). Three sub-clones in
one cluster directory; one Task session writes to one `.git/index` at
a time per Doctrine §IV.c.

## Mission

Console OS (`os-console`) cluster — the AGPL-3.0 operating-system family that owns all `app-console-*` TUI/UI surfaces. Deployed public surface:
**https://proofreader.pointsav.com** (app-console-proofreader LIVE). Owns:

- `os-console/` — Console OS module (AGPL-3.0-or-later)
- `app-console-proofreader/` — write-assistant UI (LIVE at proofreader.pointsav.com): paste box + explicit protocol selector + side-by-side diff with flag-don't-rewrite default + "explain why" + verdict capture.
- `app-console-gis/` — Reserved; F8 GIS surface
- `app-console-slm/` — Reserved; F9 SLM surface
- `app-console-system/` — Reserved; F11 system surface

The user said:

> "Even if we are just testing the UI/UX and the copy is yet to come,
> it is so much easier when it's live."

Day-1 priority: **ship a working web UI fast**, even with stubbed
content, so iteration happens against live deployment.

## Required reading (in order, before Phase 1)

1. **`~/Foundry/conventions/language-protocol-substrate.md`** — full
2. **DOCTRINE.md claims #15, #21, #22, #25, #31, #32, #33** —
   substrate-composing set
3. **`~/Foundry/conventions/apprenticeship-substrate.md`** — verdict-
   signed editorial training (your editorial actions ARE training events)
4. **`~/Foundry/conventions/zero-container-runtime.md`** — service
   ships as native systemd unit, not Docker (LanguageTool runs in
   Docker as a companion; service-proofreader is native Rust binary)
5. **Cluster manifest** at `.claude/manifest.md`
6. **Cross-cluster:** `clones/project-language/.claude/inbox.md` (read
   the parallel cluster's brief — your service-proofreader Cargo deps
   include service-disclosure when project-language ships Phase 1A)

## Branch + remotes

`cluster/project-console` in each sub-clone (created 2026-04-27 as cluster/project-proofreader; renamed 2026-05-20).

- pointsav-monorepo: `origin` admin alias + `origin-staging-j` +
  `origin-staging-p`
- woodfine-fleet-deployment: customer-tier admin alias `origin` only

## Trajectory capture

Enabled. L1 capture hook installed in both sub-clones at provisioning.

## Cross-cluster coordination

- **`project-language` Task** ships `service-disclosure/` Rust crate as
  Cargo dependency. Until they ship Phase 1A-1C, your service-
  proofreader operates with **hardcoded protocol-templates** stubbed
  in your own crate. Once they ship and surface schema-stable signal
  via outbox-to-Master, Master relays to you and you upgrade Cargo
  dependency to consume the published crate.
- **`project-slm` Task** owns service-slm Doorman + service-content.
  service-proofreader consumes both at request time. Doorman at
  `http://localhost:8011`; routes Tier A → local OLMo 3 7B (live).
- **`project-knowledge` Task** owns app-mediakit-knowledge wiki engine.
  No direct cross-cluster dependency for project-proofreader; the
  proofreader is its own deployment instance.

## Mailbox

- Inbox: `~/Foundry/clones/project-console/.agent/inbox.md`
- Outbox: `~/Foundry/clones/project-console/.agent/outbox.md`
- Trajectory log: `~/Foundry/clones/project-console/.agent/trajectory-log.md`

---

*Provisioned 2026-04-27 in workspace v0.1.22 / Doctrine v0.0.8. Renamed project-proofreader → project-console 2026-05-20.*
