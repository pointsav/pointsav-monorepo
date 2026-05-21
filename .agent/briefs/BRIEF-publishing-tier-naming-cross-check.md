---
artifact: brief
status: active
---

# Naming cross-check — proposed publishing-tier vs canonical taxonomy

## Context

In the conversation we sketched a "publishing VM" that would offload customer-facing websites (5 surfaces) from `foundry-workspace`, with a folder structure like `~/deployments/<site>-N/` matching one-to-one between Foundry and the new VM. I named the candidate folders ad-hoc:

- `documentation-pointsav-1`
- `projects-woodfine-1`
- `corporate-woodfine-1`
- `gis-woodfine-1`
- `proofreader-woodfine-1`
- VM name: `publishing-1` or `woodfine-publishing-1`

Operator asked to cross-check these against the Nomenclature Matrix (CLAUDE.md §5 #2) and the MEMO Development Overview (#3). After deep-thinking with the operator on whether new catalog folders were necessary, the plan resolved to a much simpler shape: one new catalog (proofreader migration to vendor side), one new convention (publishing-tier pattern), no new fleet-* catalog folder, and a freely-chosen GCE hostname. Five sites split cleanly across the existing vendor/customer doctrinal flow.

## Audit findings

### 1. The two cited authoritative documents do not exist as files

CLAUDE.md §5 names these as priority #2 and #3:
- `/srv/foundry/IT_SUPPORT_Nomenclature_Matrix_V8.md` — **does not exist**
- `/srv/foundry/MEMO-2026-03-30-Development-Overview-V8.md` — **does not exist**

Cross-references to them appear in `customer/woodfine-fleet-deployment/.agent/rules/project-registry.md` ("Deployment prefix taxonomy — see `IT_SUPPORT_Nomenclature_Matrix_V8.md` §4"). That citation is a dangling pointer.

The actual canonical taxonomy is **inferred from**:
- `/srv/foundry/MANIFEST.md` (workspace manifest)
- `/srv/foundry/deployments/` (live instance structure)
- `/srv/foundry/customer/woodfine-fleet-deployment/` (catalog folders)
- per-cluster `.agent/manifest.md` files

### 2. Canonical taxonomy (seven deployment prefixes)

From the project-registry citation and observed catalog/instances, the deployment-tier prefixes are:

| Prefix | Meaning | Examples (live) |
|---|---|---|
| `cluster-*` | Data fleet (totebox archives, multi-instance) | `cluster-totebox-corporate-1/-2/-3`, `cluster-totebox-personnel-1` |
| `gateway-*` | Orchestration gateway (workflow surface: input → orchestrate → output + state capture) | `gateway-orchestration-gis-1`, `gateway-orchestration-bim-1` |
| `media-*` | Content publication / distribution (one-way reader surfaces) | `media-knowledge-documentation-1`, `media-knowledge-projects-1`, `media-distribution-newsroom`, `media-marketing-landing` |
| `vault-*` | Source / secrets repository | `vault-privategit-source-1` (the workspace), `vault-privategit-design-1` |
| `fleet-*` | Compute pool tier | catalog: `fleet-infrastructure-cloud`, `-leased`, `-onprem` |
| `node-*` | **Console-OS runtime** (Type 2 hypervisor TUI on a host; the operator console surface) — NOT a generic "runtime host" prefix | catalog: `node-console-operator` |
| `route-*` | Network routing | catalog: `route-network-admin` |

**Important correction to my earlier audit:** I previously suggested `node-publishing-1` as a generic "runtime host" name. That's wrong. `node-*` is reserved for Console-OS surfaces (the secure-delivery TUI running on a host); the related app-tier surface is `app-console-*` (e.g., `app-console-proofreader`). A publishing host is NOT a node-* deployment.

**Number-suffix rule:** `-1`, `-2`, `-3` apply only to **deployment instances** under `~/Foundry/deployments/`. Catalog folders in `customer/woodfine-fleet-deployment/` carry no suffix. Source repos carry no suffix. The first instance is always `-1`. Multi-tenant cohabitation requires distinct numbers.

**Tenant-keying:** the canonical name does NOT carry tenant in the suffix. Tenant (pointsav vs woodfine) is a **property** of the deployment, not its name.

### 3. Site-by-site corrected naming (with vendor/customer split)

**Domain update (2026-05-05):** Proofreader is moving from `proofreader.woodfinegroup.com` to **`proofreader.pointsav.com`** — i.e., vendor-tier (pointsav.com), not customer-tier (woodfinegroup.com). This aligns with the "PointSav running its own software as open public reference" framing.

| Site domain | Tier | My proposal | Canonical name | Status today |
|---|---|---|---|---|
| documentation.pointsav.com | vendor | documentation-pointsav-1 | **`media-knowledge-documentation-1`** | ✅ instance + catalog exist; live |
| **proofreader.pointsav.com** | **vendor (newly moved)** | proofreader-woodfine-1 | **`gateway-orchestration-proofreader-1`** (Q1) | ⚠ no catalog folder; running as `local-proofreader.service` |
| projects.woodfinegroup.com | customer | projects-woodfine-1 | **`media-knowledge-projects-1`** | ✅ instance + catalog exist; live |
| corporate.woodfinegroup.com | customer | corporate-woodfine-1 | **`media-knowledge-corporate-1`** | ⚠ catalog exists; instance not yet realized |
| gis.woodfinegroup.com | customer | gis-woodfine-1 | **`gateway-orchestration-gis-1`** | ✅ instance exists; live |

The five sites now split **cleanly along the vendor → customer doctrinal flow:** 2 PointSav (vendor public reference) + 3 Woodfine (customer customer-facing).

### 4. Catalog placement — vendor/customer split already in force

**Correction to my earlier audit (the Phase 1 agent missed this):** `vendor/pointsav-fleet-deployment/` already exists as a separate GitHub-tracked repo, parallel to `customer/woodfine-fleet-deployment/`. The doctrinal vendor → customer split is operational at the catalog tier, not a future decision.

**vendor/pointsav-fleet-deployment/** (5 catalog folders):
- `media-knowledge-distribution/`
- `media-knowledge-documentation/`  ← documentation.pointsav.com
- `media-marketing-landing/`
- `vault-privategit-design-system/`
- `vault-privategit-source/`  ← cited by project-intelligence manifest for guide-doorman-deployment

**customer/woodfine-fleet-deployment/** (~14 catalog folders):
- `cluster-totebox-corporate/`, `cluster-totebox-personnel/`, `cluster-totebox-property/`
- `fleet-infrastructure-cloud/`, `-leased/`, `-onprem/`
- `gateway-interface-command/`
- `media-distrabution-newsroom/` (typo in folder name — `distrabution` should be `distribution`)
- `media-knowledge-corporate/`, `media-knowledge-documentation/` (DUP across vendor side), `media-knowledge-projects/`
- `media-marketing-landing/` (DUP across vendor side)
- `node-console-operator/`

**Drift to surface:**
- `media-knowledge-documentation/` exists in **both** catalogs. Cluster manifest (project-knowledge) cites the vendor side as authoritative — so the customer-side copy is stale and should be removed.
- `media-marketing-landing/` exists in both as well — same triage needed.
- `media-distrabution-newsroom/` is misspelled (should be `media-distribution-newsroom`).
- `gateway-orchestration-gis/` and `gateway-orchestration-bim/` catalog folders are **missing in both catalogs** despite live instances at `deployments/gateway-orchestration-{gis,bim}-1/`. project-gis manifest marks this as `leg-pending — catalog folder authoring`.

### 5. Updated site-by-site naming with correct catalog placement

| Site domain | Tier | Canonical instance | Catalog location | Status |
|---|---|---|---|---|
| documentation.pointsav.com | vendor | `media-knowledge-documentation-1` | `vendor/pointsav-fleet-deployment/media-knowledge-documentation/` | ✅ live (catalog ratified 2026-04-26) |
| **proofreader.pointsav.com** | **vendor (newly moved)** | **`gateway-orchestration-proofreader-1`** | **`vendor/pointsav-fleet-deployment/gateway-orchestration-proofreader/`** (NEW — needs creation) | ⚠ migration needed: delete woodfine-side catalog (9ede81f), create vendor-side, update project-proofreader manifest |
| projects.woodfinegroup.com | customer | `media-knowledge-projects-1` | `customer/woodfine-fleet-deployment/media-knowledge-projects/` | ✅ live |
| corporate.woodfinegroup.com | customer | `media-knowledge-corporate-1` | `customer/woodfine-fleet-deployment/media-knowledge-corporate/` | ⚠ catalog exists; instance not realized |
| gis.woodfinegroup.com | customer | `gateway-orchestration-gis-1` | `customer/woodfine-fleet-deployment/gateway-orchestration-gis/` (NEW — leg-pending) | ⚠ instance live; catalog folder needs authoring |

### 6. Publishing VM does not need a taxonomy slot

After deep-thinking with the operator: catalogs are GUIDEs for *kinds* of deployment instances. The new VM is not a new kind — it's another GCE host running already-cataloged deployment types. No new catalog folder needed. The VM gets a GCE hostname (operator picks); the seven-prefix taxonomy applies to the deployment instances on it, not the host.

### 7. Naming drift in my conversation responses (lessons)

I should have verified canonical names before sketching `documentation-pointsav-1` etc. Two lessons worth memorizing:

- **Derive deployment prefixes from role** (data fleet → cluster-*, wiki → media-knowledge-*, orchestration → gateway-*) by reading existing canonical names in `~/Foundry/deployments/` first, not from the site's tenant or domain.
- **`node-*` is reserved for Console-OS surfaces** (Type 2 hypervisor TUI for secure delivery on customer-owned hardware), not generic "runtime host." The related app-tier surface is `app-console-*`.
- **Don't propose new catalog folders unless the operational knowledge is genuinely new.** Check first whether the knowledge belongs in an existing catalog, a workspace convention, or a deployment instance MANIFEST.

## Recommended corrections to the publishing-VM architecture sketch

```
foundry-workspace VM (engineering)            <new-VM-hostname> (publishing)
─────────────────────────────────             ─────────────────────────────────
~/Foundry/deployments/                        ~/deployments/
├── media-knowledge-documentation-1/  rsync→  ├── media-knowledge-documentation-1/
├── media-knowledge-projects-1/       rsync→  ├── media-knowledge-projects-1/
├── media-knowledge-corporate-1/      rsync→  ├── media-knowledge-corporate-1/   (instance to be realized)
├── gateway-orchestration-gis-1/      rsync→  ├── gateway-orchestration-gis-1/
├── gateway-orchestration-proofreader-1/ rsync→ ├── gateway-orchestration-proofreader-1/
└── (engineering-only instances stay on Foundry)
```

Same path on every host. Per-site VM graduation later is a clean lift.

**No new `fleet-public/` catalog folder.** The VM is just compute under the existing `fleet-infrastructure-cloud/` catalog. Each deployment instance on it uses its own existing catalog. The cross-cutting publishing pattern (rsync mirror, atomic-swap, DNS cutover) lives in a workspace convention, not in a catalog.

## Decisions reached during this audit

| # | Question | Decision |
|---|---|---|
| Q1 | Proofreader instance name | `gateway-orchestration-proofreader-1` (cluster-ownership symmetry with project-bim/-gis; bidirectional orchestrated workflow, not one-way publication) |
| Q2 | Proofreader catalog placement | `vendor/pointsav-fleet-deployment/gateway-orchestration-proofreader/` — vendor side, matches the pointsav.com domain |
| Q3 | Publishing VM count | One mixed-tenant VM (vendor + customer surfaces co-resident) |
| Q4 | Publishing VM catalog | **NONE.** No new catalog folder is needed. The VM is GCE compute under existing `customer/woodfine-fleet-deployment/fleet-infrastructure-cloud/`; each deployment instance on it uses its own existing catalog. Cross-cutting publishing pattern (rsync mirror, atomic-swap, DNS cutover) lives in `conventions/publishing-tier-architecture.md` |
| Q5 | Publishing VM hostname | Operator picks a sensible GCE hostname (e.g. `pointsav-public-1`, `pointsav-publishing`, `woodfine-publishing-1`). Hostname is its own namespace; doesn't bind taxonomy. |

## Persistence — save this plan to a durable location FIRST

This file currently lives at `/home/mathew/.claude/plans/can-you-cross-check-starry-unicorn.md` — Claude's plan-mode scratch space, which may be cleaned up. **Step 0 of execution: copy this plan into the workspace at:**

```
/srv/foundry/.agent/plans/2026-05-05-publishing-tier-naming-cross-check.md
```

ISO date prefix + lowercase-ASCII-with-hyphens descriptive name per CLAUDE.md §14 file naming. A new `.agent/plans/` subdirectory is the natural home for in-flight Master execution plans (parallel to existing `.agent/sub-agent-queue.md`, `.agent/drafts-outbound/`, etc.). If `.agent/plans/` doesn't exist yet, create it with this file as the first inhabitant.

Once saved durably:

- Reference the file path from `NEXT.md` when this work is queued, instead of inlining the full plan
- The plan can outlive any one Claude session and be picked up by a future Master session (or the operator) without context loss
- Plan-mode scratch file at `/home/mathew/.claude/plans/...` can be discarded after the copy

## Files to modify (no implementation in this plan)

When the publishing-VM project lands as NEXT.md items, the work touches:

**Proofreader migration (vendor side):**
- `vendor/pointsav-fleet-deployment/gateway-orchestration-proofreader/` — NEW catalog folder (README, README.es, MANIFEST, guide-deployment, guide-provision-node, guide-operate). Author from project-proofreader Task scope.
- `customer/woodfine-fleet-deployment/<existing-proofreader-catalog>/` — DELETE (was committed at 9ede81f against the old woodfine.com domain)
- `clones/project-proofreader/.agent/manifest.md` — update `fleet_deployment_repo` to `vendor/pointsav-fleet-deployment`; update purpose strings (currently still cites woodfine.com)
- nginx + DNS — proofreader.pointsav.com vhost; cert renewal config (operator-presence)
- deployment instance: `~/Foundry/deployments/gateway-orchestration-proofreader-1/MANIFEST.md`

**Publishing-VM provisioning (no new catalog needed):**
- `conventions/publishing-tier-architecture.md` — NEW convention. Names the rsync mirror pattern, atomic-swap convention, DNS cutover sequence, ~/deployments/<canonical-name>/ symmetry between hosts.
- GCE VM provision (operator from iMac) — hostname picked by operator (e.g. `pointsav-public-1`); pd-balanced 30 GB; us-west1-a; e2-small.
- nginx + Let's Encrypt on the new VM for the 5 vhosts.
- Sync timer on foundry-workspace — systemd unit + per-site rsync to the new host.
- `PROJECT-CLONES.md` — add a publishing-tier note describing which deployment instances run where.
- `NEXT.md` — Master-scope items: convention authoring, VM provisioning, DNS cutover, sync timer.
- `MANIFEST.md` — version log entry noting publishing-tier introduction.

**Catalog drift cleanup (separate from publishing-VM project):**
- `customer/woodfine-fleet-deployment/media-knowledge-documentation/` — duplicate of vendor side; remove
- `customer/woodfine-fleet-deployment/media-marketing-landing/` — duplicate of vendor side; remove (or confirm one is canonical)
- `customer/woodfine-fleet-deployment/media-distrabution-newsroom/` — `git mv` to fix `distrabution` → `distribution` typo
- `vendor/pointsav-fleet-deployment/` should add: `gateway-orchestration-proofreader/` (above)
- `customer/woodfine-fleet-deployment/` should add: `gateway-orchestration-gis/`, `gateway-orchestration-bim/` (currently leg-pending per cluster manifests)

**Dangling-reference cleanup (separate):**
- `CLAUDE.md` §5 — references `IT_SUPPORT_Nomenclature_Matrix_V8.md` and `MEMO-2026-03-30-Development-Overview-V8.md` as priority-2/3 authoritative documents. Neither file exists in the workspace. Resolve by: (a) authoring/restoring them, (b) repointing to current authoritative sources (MANIFEST + customer/woodfine-fleet-deployment/.agent/rules/project-registry.md cite), or (c) removing the citations.
- `customer/woodfine-fleet-deployment/.agent/rules/project-registry.md` — same dangling reference: "see IT_SUPPORT_Nomenclature_Matrix_V8.md §4". Repoint or remove.

## Verification

This plan is an audit + naming correction, not an implementation. To verify the audit findings:

1. `ls /srv/foundry/vendor/pointsav-fleet-deployment/` → confirm vendor catalog has 5 folders + INVENTORY.yaml + standard root files (README, NEXT.md, SECURITY.md, TRADEMARK.md).
2. `ls /srv/foundry/customer/woodfine-fleet-deployment/` → confirm customer catalog has cluster-totebox-* / fleet-infrastructure-* / gateway-interface-command / media-knowledge-* / node-console-operator entries.
3. `ls /srv/foundry/deployments/` → confirm canonical instance names: `media-knowledge-documentation-1`, `media-knowledge-projects-1`, `gateway-orchestration-gis-1`, `gateway-orchestration-bim-1`, `cluster-totebox-corporate-{1,2,3}`, etc.
4. `grep -r "fleet_deployment_repo" /srv/foundry/clones/*/.agent/manifest.md` → confirm cluster manifests cite the correct catalog (vendor or customer) per their domain tier.
5. `head /srv/foundry/deployments/media-knowledge-documentation-1/MANIFEST.md` → confirm the manifest schema's `guide:` field points to `vendor/pointsav-fleet-deployment/media-knowledge-documentation` (vendor side, matching the pointsav.com domain).
6. `ls /srv/foundry/IT_SUPPORT_Nomenclature_Matrix_V8.md /srv/foundry/MEMO-2026-03-30-Development-Overview-V8.md 2>&1 | grep -c "No such"` → confirms these files don't exist (CLAUDE.md §5 dangling references).

All audit questions are resolved. Implementation can proceed with the names and placements above when the operator opens the publishing-VM project as a NEXT.md item.

## Execution order summary

When operator authorizes this plan post-exit:

1. **Save plan durably** — copy this file to `/srv/foundry/.agent/plans/2026-05-05-publishing-tier-naming-cross-check.md` (create `.agent/plans/` if needed). Master commits.
2. **Add NEXT.md item** — Master scope: "Publishing-tier separation per `.agent/plans/2026-05-05-publishing-tier-naming-cross-check.md`."
3. **Proofreader catalog migration** — project-proofreader Task creates `vendor/pointsav-fleet-deployment/gateway-orchestration-proofreader/`, Master coordinates the woodfine-side delete of the old (9ede81f) catalog and the cluster-manifest update.
4. **Catalog drift cleanup** (parallel, can run anytime) — remove duplicates, fix typo, author leg-pending gateway-orchestration-{gis,bim} catalogs.
5. **Author convention** — `conventions/publishing-tier-architecture.md` (Master scope).
6. **VM provision** — operator from iMac (gcloud).
7. **DNS cutover** — one site at a time, lowest-traffic first.
8. **Dangling-references cleanup** — separate NEXT.md item; CLAUDE.md §5 + project-registry.md.

No code or filesystem changes happen until operator approves and steps execute outside plan mode.

---

# EXECUTION PROGRESS LOG — 2026-05-05 → 2026-05-06

Tracking what's actually landed vs what remains. Operator authorized "let's do it all" 2026-05-05.

## Completed (15 of 26 tasks)

### Conventions authored (4)
- ✅ `conventions/nomenclature-taxonomy.md` — replaces dangling V8 reference
- ✅ `conventions/orchestration-architecture.md` — Model B ratified; hub-and-spoke topology
- ✅ `conventions/datagraph-access-discipline.md` — single-mode "everything via Doorman"; Tier 0 alignment; interim direct-access section added 2026-05-06
- ✅ `conventions/publishing-tier-architecture.md` — separation pattern, rsync mirror, atomic-swap, DNS cutover, per-site VM graduation

### Cleanup (2)
- ✅ Catalog drift cleanup: removed customer-side media-knowledge-documentation/ duplicate; authored leg-pending gateway-orchestration-{gis,bim} catalog skeletons on customer side; verified media-marketing-landing not actually duplicate (vendor + customer have legitimately different content)
- ✅ CLAUDE.md §5 + AGENT.md + customer/woodfine-fleet-deployment/.agent/rules/project-registry.md dangling references repointed to nomenclature-taxonomy convention; MEMO V8 reference dropped (DOCTRINE.md + per-project ARCHITECTURE.md cover this)

### Catalog authoring (1 + 1)
- ✅ vendor/pointsav-fleet-deployment/gateway-orchestration-proofreader/ — 4 files (README, README.es, guide-deployment, guide-provision-node) for proofreader.pointsav.com vendor-tier deployment
- ✅ ~/Foundry/deployments/gateway-orchestration-proofreader-1/MANIFEST.md — deployment instance for the proofreader public reference

### Mailbox broadcasts (1 task = 14 messages)
- ✅ DataGraph access pipeline OPEN broadcast to 10 cluster inboxes (project-bim, -bookkeeping, -data, -design, -editorial, -gis, -intelligence, -knowledge, -orgcharts, -proofreader, -system; project-command has no inbox yet)
- ✅ Targeted earlier messages to project-intelligence (queue Doorman endpoints), project-design (graph access reply), project-proofreader (rebase 9ede81f for vendor-side migration), project-gis (IPEDS fix + sprint promote-readiness)

### NEXT.md & PROJECT-CLONES.md (2)
- ✅ NEXT.md publishing-tier + orchestration architecture items added
- ✅ PROJECT-CLONES.md project-command 10th cluster registered

### project-command cluster (1)
- ✅ Cluster manifest authored at clones/project-command/.agent/manifest.md
- ✅ pointsav-monorepo sub-clone provisioned 2026-05-06 (~468 MB; cluster/project-command branch; 3 remotes); state moved from "leg-pending" to "active"; pointsav-design-system + vendor/pointsav-fleet-deployment sub-clones deferred (not needed for first MVP)

### Operator-presence (1)
- ✅ gdal-bin installed 2026-05-06 (GDAL 3.8.4 at /usr/bin/ogr2ogr); project-gis download-boundaries.sh now runnable

## Blocked / pending (11 of 26 tasks)

### Cluster Task work (4)
- ⛔ #10 Stage-6 promote project-intelligence — local main DIVERGED from staging mirrors (not just commits-ahead). 42 commits ahead of origin/main; staging at f943137 (ahead of canonical, has feature branches audit-layer-1-findings + service-extraction-v04); local at 19c652a; not a fast-forward. Cluster Task must commit pending state (.agent/inbox.md broadcast + settings) and decide rebase vs merge against staging mirrors before Master can promote.
- ⛔ #12 Doorman graph proxy endpoints — project-intelligence Task scope; surfaced via cluster inbox 2026-05-05
- ⛔ #17 First app-orchestration-command MVP — project-command Task scope; cluster ready
- ⛔ #24 Stage-6 promote project-gis — cluster has 176 commits ahead + 100+ dirty files of sprint work; same divergence-or-fast-forward question
- ⛔ #26 IPEDS URL fix — project-gis Task scope; surfaced via cluster inbox 2026-05-05

### Operator-presence (5)
- ⏳ #8 proofreader.pointsav.com nginx + DNS + cert
- ⏳ #11 service-content systemd install (depends on #10 promote landing first)
- ⏳ #13 service-content lockdown (depends on #11 + #12)
- ⏳ #19 publishing-VM provision
- ⏳ #20 publishing-VM nginx + Let's Encrypt
- ⏳ #22 DNS cutover

### Composite/dependent (2)
- ⏳ #21 rsync mirror timer (after VM)
- ⏳ #23 PROJECT-CLONES + MANIFEST update for publishing-tier (after cutover)

## Three commits this session

1. **Workspace** `92326f8` (2026-05-05) — 13 files / 1,350 insertions: 4 conventions + dangling-ref fixes + NEXT.md + PROJECT-CLONES.md + saved plan + project-command manifest + 3 cluster broadcast inboxes
2. **Customer fleet** `6d5cda2` + `09c4dd4` (2026-05-05) — 14 files: catalog cleanup + 2 new gateway-orchestration catalogs
3. **Vendor fleet** `ea3d0b9` (2026-05-05) — 4 files: gateway-orchestration-proofreader catalog
4. **Workspace** `194df8f` (2026-05-06) — 10 files / 1,000 insertions: DataGraph pipeline broadcast + convention amendment

## What's at the next session start

**Immediate Master actions available:**
- Save final state commit (this update)
- Sync project-command manifest changes (just done)

**Operator-presence list (priority order):**
1. proofreader.pointsav.com nginx + DNS + cert (#8) — 30 min
2. service-content systemd install (#11) — 30 min, depends on #10 cluster Task action
3. publishing-VM provisioning (#19, #20, #22) — multi-day project

**Cluster Task work (next time each cluster opens):**
- project-intelligence: handle promote divergence + author Doorman graph endpoints
- project-gis: commit sprint work + fix IPEDS URL
- project-proofreader: rebase 9ede81f + update manifest for vendor-side
- project-command: occupy cluster, author CLAUDE.md + first app-orchestration-command/ scaffolding

End of execution log.

---

# RELATED ARCHITECTURAL DEEP-THINK — appended 2026-05-05

# Orchestration architecture — production-first design with `app-orchestration-command` as hub

## Context

Conversation continued past the publishing-tier audit into a deeper architectural question raised by the operator: how should all clusters access service-content's DataGraph, and is this really a per-cluster access problem or a missing-architecture problem? Operator framing:

> "build software the exact same way it is to be deployed and used in the real world"

> "we need to also give permission or properly set it up so that all projects can access the DataGraph that service-content has in place... We need to get this right as it is going to be needed all the time now, it should be a two way street, all the projects are training the service-slm and service-content as we go now"

> "would it help if we looked at what we need now as `app-orchestration-slm` or `app-orchestration-content` where we have `projects-*` which are the same as the future Totebox Archives that need to be aggregated not only for user groups but also for the data... maybe these features are all part of the basic `app-orchestration-command` which is the base `app-orchestration-*` for `os-orchestration`?"

> "we connect `app-orchestration-command` to all the `projects-*` on one side (simulating Totebox Archives) and we connect two USERS on the other side Jennifer and Mathew (I'm not sure how Master fits in here, maybe Mathew is MASTER)"

## Core insight

The eleven `project-*` clusters are not just engineering containers. They are **operational shape-tests for the production architecture**. Each `project-*` simulates one Totebox Archive (a per-customer per-domain data fleet). Production topology = development topology, just at different scale and with real archives instead of project clones.

If true, the missing piece in development is the same as the missing piece in production: **a hub gateway that aggregates users and data across archives.** Today, there is no aggregation layer. Master/Root/Task session pattern is the dev-time scaffold for this; in production it must be a real software component.

## Topology

```
   Users                Command Gateway                  Data Archives (today: project-* clusters)
   ─────                ───────────────                  ─────────────

                                                        ┌── project-bim         (→ totebox-bim)
   Jennifer  ──┐                                        ├── project-gis         (→ totebox-gis)
               │                                        ├── project-bookkeeping (→ totebox-books)
               │                                        ├── project-knowledge   (→ totebox-wiki)
               │      app-orchestration-command         ├── project-design      (→ totebox-design)
               ├─→    ├── user aggregation         ─→  ├── project-editorial   (→ totebox-editorial)
               │      ├── data aggregation              ├── project-data        (→ totebox-data)
               │      ├── permission boundary           ├── project-system      (→ totebox-system)
               │      └── consumes service-slm          ├── project-orgcharts   (→ totebox-orgs)
   Mathew   ──┘            and service-content          ├── project-intelligence (→ totebox-intelligence)
                                                        └── project-proofreader  (→ totebox-proof)
```

**User aggregation** — each user has scoped access across multiple archives; command presents a unified view.
**Data aggregation** — single query federates across archives; results deduped + presented as one answer.

## Two architectural models

### Model A — `service-*` as dependencies; command as the only app surface
- `app-orchestration-slm` and `app-orchestration-content` don't exist as separate apps
- service-slm and service-content are backends consumed by command
- Pro: simplest, one gateway to operate
- Con: power-user surfaces (slm corpus inspector, content graph editor) get buried in command

### Model B — peer `app-orchestration-*` apps; command is one of them — RECOMMENDED
- `app-orchestration-slm` (slm operator console)
- `app-orchestration-content` (graph browser, schema editor)
- `app-orchestration-command` (user-facing aggregator)
- Each deploys to its own `gateway-orchestration-*-N` instance, parallel to existing `app-orchestration-bim` / `-gis`
- Command depends on slm + content as upstream services
- Pro: consistent with existing taxonomy, single-concern surfaces, clear ownership
- Con: more apps to develop

**Recommendation: Model B.** Preserves taxonomic consistency (`bim` / `gis` / `command` / `slm` / `content` are all sibling `app-orchestration-*` apps); preserves production-first (different audiences need different surfaces); doesn't conflate framework with app.

## Master ↔ Production user duality

| Dev session role | Production user identity | Notes |
|---|---|---|
| Master (Mathew) | admin user (Mathew) using `app-orchestration-command` in admin mode | Same person, same UI, elevated permissions |
| Root (per repo) | archive admin (governance over one archive) | Per-archive operator role within command |
| Task (per cluster clone) | regular user (Jennifer or others) doing work in one archive | Default user mode |

The development pattern (Master/Root/Task) is **isomorphic** to the production pattern (admin user / archive admin / regular user). Master is a development scaffold, not a production entity.

## What this means for the DataGraph access question — ONE MODE (revised after operator pushback)

Initial draft proposed two-mode access (dev = direct to service-content, prod = through command). Operator pushed back: *"should this just be one mode so there is no confusion for now?"* Reconsidered — operator is right; two-mode violates the production-first principle.

**Single mode: everything goes through the Doorman.** This aligns with Doctrine claim #43 (Single-Boundary Compute Discipline) already ratified for inference; extending it to graph access introduces no new principle.

```
                          ┌─→ service-slm    (inference)
   anything that needs    │
   the graph or inference ├─→ Doorman ──┼─→ service-content (graph)
   (clusters today, users │              └─→ external LLMs (Tier C)
   via command tomorrow)  │
```

- **Doorman (127.0.0.1:9080)** — the only boundary. Audits all access. Enforces module_id. Handles auth (future).
- **service-content (127.0.0.1:9081)** — backend service. Localhost-bound; nftables/systemd-restricted to Doorman process. Never called directly by clusters or users.
- **service-slm** — already inside this boundary.
- **app-orchestration-command** — when it exists, also goes through Doorman like any other caller.

Doorman gets two new endpoints (proxying service-content):
- `POST /v1/graph/query` — proxies to service-content's `/v1/graph/context`, audits as `event_type: graph-query` (or use existing `/v1/audit/proxy` with provider="service-content")
- `POST /v1/graph/mutate` — proxies to service-content's `/v1/graph/mutate`, audits as `event_type: graph-mutation`

`graph-mutation` already exists in audit-endpoints-contract v0.2.0. No new substrate — thin proxy on existing primitives.

### Why one-mode is actually simpler

| Concern | Two-mode (rejected) | One-mode (chosen) |
|---|---|---|
| Convention complexity | Dev mode + prod mode + graduation rule | "Everything via Doorman" |
| Audit logging | Different per mode | Universal |
| Tenant isolation (module_id) | Enforced twice in different ways | Enforced once at boundary |
| What clusters do today | Direct curl to 9081 | Curl Doorman 9080 — same effort |
| What clusters do in production | Different from today | Same as today |
| Operator/developer confusion | Real | None |

Effort difference for cluster scripts: change `:9081/v1/graph/context` to `:9080/v1/graph/query`. That's it.

### app-orchestration-command's scope clarified

Command's value is **user/data aggregation**, NOT graph-access middleware. service-content + Doorman already handle graph access cleanly under one-mode discipline. Command sits on top, focused on what users see (their archives, their entities, their work-in-progress) — not on plumbing graph queries.

### service-content lockdown — making one-mode enforceable

For one-mode to be a discipline (not just a convention), service-content's localhost binding needs to actually exclude direct cluster access:

- Systemd unit `local-service-content.service` runs as `local-service-content` user
- Bind to `127.0.0.1:9081` only
- nftables rule: only allow connections to 9081 from the `local-doorman` user's PID/cgroup
- OR: systemd socket activation with peer-cred check (Linux SO_PEERCRED)

The lockdown can land in a follow-up round; the convention starts with localhost binding + discipline. Direct curl by operator (debugging) is allowed via SSH/sudo, not via cluster Task.

### Tier 0 alignment — architecture works on a $7/mo node with zero AI

Operator sanity check raised: *"how this still would work if service-slm is on a $7 per month node and could not handle any AI, TIER 0 in the Totebox"*

The one-mode design is **stronger on Tier 0**, not weaker. Per the Compounding Substrate doctrine, *optional intelligence* is one of the five pillars: the substrate must work without AI. Tier 0 is the on-ramp, not a degraded mode.

**On a Tier 0 node (e2-small ~$14/mo, e2-micro free-tier ~$0/mo, no GPU, ~1 GB RAM):**

| Component | Status on Tier 0 |
|---|---|
| service-content (LadybugDB graph) | ✅ Runs — embedded DB, ~10K entities under 500 MB RAM, ~5-10 MB binary |
| slm-doorman-server (boundary) | ✅ Runs — `has_local=false, has_yoyo=false, has_external=false` is a first-class supported config; binary ~7 MB |
| service-slm Tier A (local llama) | ❌ Not installed — no GPU |
| service-slm Tier B (Yo-Yo GPU) | Optional — customer-configurable; costs nothing when not invoked |
| service-slm Tier C (external API) | Optional — customer brings their own key |
| app-mediakit-knowledge (wikis) | ✅ Light Rust binary + Tantivy index |
| app-orchestration-command | ✅ TUI/thin-web, no GPU need |
| All Totebox Archives | ✅ Data containers, CRUD only |

**Doorman endpoints under "zero AI tier configured":**

| Endpoint | Tier 0 with no AI | Why |
|---|---|---|
| `/healthz`, `/readyz`, `/v1/contract` | ✅ | Status only |
| `POST /v1/chat/completions` | ❌ → 503 TierUnavailable | Honest degraded response |
| `POST /v1/graph/query` | ✅ | **No AI involved — proxies to service-content** |
| `POST /v1/graph/mutate` | ✅ | **No AI involved — proxies to service-content** |
| `POST /v1/audit/capture` | ✅ | Local-only ledger write |
| `POST /v1/audit/proxy` | ❌ → 503 | Needs upstream provider configured |
| `POST /v1/shadow`, `/v1/verdict` | ✅ if enabled | Queue substrate, no inference required |

**Graph access through Doorman is fully functional on Tier 0 with zero AI tiers configured.** Only inference endpoints degrade gracefully (503 with clear error). No discipline is broken; one-mode holds.

### Tier 0 is the SMB on-ramp, not a degraded mode

Even with no AI configured, on Tier 0:

1. **Graph mutations land** — every entity Jennifer/Mathew add through command flows through Doorman → service-content → audit ledger captures `event_type: graph-mutation`. Substrate grows.
2. **Engineering corpus accumulates** — git commits produce training tuples at `~/Foundry/data/training-corpus/engineering/`. Substrate grows.
3. **Apprenticeship corpus accumulates** — prose edits produce DPO pairs (when SLM_APPRENTICESHIP_ENABLED=true). Substrate grows.
4. **Audit ledger records everything** — at `/var/lib/local-doorman/audit/<YYYY-MM>.jsonl`. Substrate grows.

Customer pays $7/mo, gets:
- Working CRUD on archives + entities through command
- Search and graph queries via Tantivy + LadybugDB
- Audit trail meeting BCSC posture
- Structured workflow surfaces (BIM, GIS, proofreader UI, knowledge wikis)
- Substrate that compounds value with every interaction
- Future option to add Tier C key OR schedule Tier B Yo-Yo bursts to unlock inference, with their accumulated context already in place

This is the actual go-to-market story for SMBs. Tier 0 is the **proof-of-value** before AI is paid for.

### Tier 0 implications for the convention

`conventions/datagraph-access-discipline.md` should explicitly state:
- The discipline holds when the customer has no AI tier configured
- Graph access does not depend on inference
- service-content + Doorman are mandatory; Tier A/B/C are optional
- Degraded modes are honest (503 with clear error), not silent failures

This is consistent with Doctrine claim #43 (Single-Boundary Compute Discipline) and the *optional intelligence* pillar of the Compounding Substrate doctrine. Tier 0 is the canonical case the architecture must serve — bigger nodes with AI tiers are extensions, not the baseline.

## Two-way street — every user action trains the substrate

Today's training capture:
1. **capture-edit hook** → engineering corpus tuples (every commit)
2. **shadow brief flow** → apprenticeship corpus (DPO pairs)

Missing third channel:
3. **graph mutation tuples** — user actions that add/edit graph entities

This third channel can't flow today because there's no `app-orchestration-command` to capture user actions from. Once command exists, every user action produces three training signals:

| Signal | Source | Sink | Used for |
|---|---|---|---|
| Engineering tuple | git commit | `~/Foundry/data/training-corpus/engineering/` | service-slm continued pretraining |
| Verdict tuple (DPO) | user edits a draft via command | `~/Foundry/data/training-corpus/apprenticeship/` | service-slm adapter LoRA training |
| Graph mutation | user adds entity via command | service-content + audit ledger | service-content classifier improvement |

This is the two-way street the operator named: every Jennifer/Mathew session compounds the substrate.

## Where to develop `app-orchestration-command`

Three options for cluster ownership:

1. **NEW `project-command` cluster** — owns `app-orchestration-command` source. Sibling to project-bim/-gis. Cleanest scoping. Recommended.
2. **Inside `project-system`** — if project-system is meant to be the kernel/substrate cluster, command might fit there. Depends on its current scope (not surveyed in this plan).
3. **Inside `project-design`** — design system produces UI components; command UI could live there. Less natural for non-UI orchestration logic.

**Default recommendation: Option 1 — new `project-command` cluster** (mirrors how project-bim/project-gis own their respective `app-orchestration-*` source).

## Implementation sequence

Ordered by what unblocks what:

1. **Decide Model A vs Model B.** (Recommendation: Model B.)
2. **Spin up `project-command` cluster.** Provision the clone, author CLAUDE.md + manifest, register in PROJECT-CLONES.md.
3. **Author `conventions/orchestration-architecture.md`** — hub-and-spoke topology, user/data aggregation, dev-vs-prod two-mode access, production-first principle.
4. **Author `conventions/datagraph-access-discipline.md`** — single-mode "everything via Doorman" specification. Endpoint table, module_id enforcement, audit-event-type mapping, service-content lockdown notes. Aligns with Doctrine claim #43.
5. **Promote service-content to workspace systemd unit** — `local-service-content.service` parallel to `local-doorman.service`. Master scope; operator-presence install. (Independent of orchestration model decision; needed regardless.)
6. **First `app-orchestration-command` MVP** — barebones TUI that lists archives, lets a user pick one, federates a query against service-content. Production-first means starting the surface at toy scale, not waiting for "complete" specs.
7. **Per-cluster module_id assignment + audit-write convention** — clusters write to service-content via Doorman audit-capture, regardless of dev or prod mode.

## How this relates to the publishing-tier work above

The publishing-VM project (above this section) is about **where customer-facing surfaces deploy**. This orchestration deep-think is about **what user-facing surface aggregates them**. Both are valid concurrent projects:

- Publishing-VM separates the customer surfaces from foundry-workspace
- app-orchestration-command provides the user aggregator across surfaces

In production, Jennifer/Mathew use `app-orchestration-command` to aggregate across the customer surfaces hosted on the publishing VM. The two projects compose:

```
Users → app-orchestration-command (one of many surfaces on the publishing VM)
                                     ↓
                         queries service-slm / service-content / archives
```

`app-orchestration-command` deploys to `gateway-orchestration-command-1`, hosted on `fleet-public-1` (the publishing VM), alongside the wikis and proofreader and gis gateway. Same VM, different surface.

## Open questions for operator (when this becomes a NEXT.md item)

- Q-A — Model A vs Model B (peer apps or single integrated app)?
- Q-B — Where does `app-orchestration-command` source live: new `project-command` cluster, existing `project-system`, or somewhere else?
- Q-C — Module-ID per-cluster assignment table — confirm the read/write authority per cluster (sketched in the DataGraph access discussion above).
- Q-D — First MVP scope: list-archives + query-one-entity, or richer first cut?
- Q-E — Authentication model: localhost trust for dev mode is fine; what's the prod auth between users and command? OAuth? mTLS? Magic link? (BCSC posture matters — claims about authentication must use planned/intended language until operational.)
- Q-F — Does `app-orchestration-command` itself become an `app-console-*` (Console-OS TUI) or a web UI or both? Per the earlier plan-mode discussion, `app-console-*` is the secure-delivery TUI tier; that may be the production target for command.

These are deferred — not blockers for the publishing-tier work, but should land before any significant `app-orchestration-command` development begins.

## Persistence note

This deep-think section is appended to the same plan file as the publishing-tier audit. It's a separate concern but related (both are deployment-tier architectural decisions). When this work is queued in NEXT.md, it can be referenced by the same plan path or split into its own plan file at that time.

End of appended section.
