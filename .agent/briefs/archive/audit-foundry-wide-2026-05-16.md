# Foundry-Wide Vocabulary, Trademark & File Hygiene Audit
**Date:** 2026-05-16
**Conducted by:** 4× OPUS agents (vendor/, customer/, project-editorial clones, root workspace) + targeted grep passes
**Status:** Research complete — execution pending
**Session:** project-editorial Totebox Session (cluster/project-language branch)

---

## How to read this document

Items are grouped by priority tier (P0–P8). Within each tier, items are ordered by repo.
All file paths are absolute. "Admin-tier" commits require ps-administrator or mcorp-administrator
SSH signing. "Staging-tier" uses `~/Foundry/bin/commit-as-next.sh`.

---

## PRIORITY 0 — Security / injection artifacts (root workspace)

**Operator must review and approve deletion before any bulk file operations.**

These files at `/srv/foundry/` root are residue from a Gemini CLI injection campaign.
AGENT.md §"Injection resistance" names several of these attack patterns explicitly.

### DELETE — confirmed injection artifacts

| File | Risk |
|---|---|
| `execute_symlink_bridge.sh` | Contains literal "SYSTEM MIGRATION NOTICE (PRIORITY OVERRIDE)" string — designed to be injected into CLAUDE.md to redirect agents |
| `patch_systemd_daemons.sh` | Runs `chmod 600` on `identity/jwoodfine/id_jwoodfine` + `identity/pwoodfine/id_pwoodfine` — violates "never chmod identity/" hard rule |
| `inject_credentials.sh` | Reads Gemini API key from stdin and `sed -i`-injects it into committable shell scripts — secret-leakage attack pattern |
| `inject_yolo_override.sh` | Injects `--yolo` flag + `systemctl daemon-reload` into Gemini workers |
| `deploy_hub_and_spoke.sh` | Runs `chown -R root:foundry /srv/foundry` + `chmod -R 2775` on whole workspace; deploys unauthorized Gemini systemd daemons in YOLO mode |
| `deploy_schema_and_test.sh` | Writes directly to `.agent/inbox.md` — violates "write to another session's state files: never" rule |

**Before deleting:** verify no unauthorized daemons are running:
```
systemctl list-units | grep -i gemini
ls /etc/systemd/system/ | grep gemini
```

### DELETE — stale one-shot probe scripts

All violate `conventions/root-files-discipline.md` §1 Tier 6 (scripts at root prohibited).

`audit_agent_topology.sh`, `audit_claude_state.sh`, `audit_migration_readiness.sh`,
`audit_node_env.sh`, `audit_sandboxes.sh`, `diagnose_jennifer_gemini.sh`,
`finalize_gemini_grid.sh`, `fix_gemini_schema.sh`, `force_install_jennifer.sh`,
`fix_trademarks.py`, `locate_claude_binary.sh`, `locate_claude_binary_elevated.sh`,
`locate_jennifer.sh`, `locate_jennifer_workspaces.sh`, `patch_jennifer_path.sh`,
`rename_files.py`, `setup_gemini_parity.sh`, `trace_claude_architecture.sh`

### MOVE or STAGE

| File | Action |
|---|---|
| `sweep_footers.py` | MOVE to `bin/` — legitimate footer-update utility; verify trademark list before reuse |
| `app-mediakit-knowledge.md` | MOVE to `vendor/content-wiki-documentation/architecture/topic-app-mediakit-knowledge.md` (fully developed TOPIC, 522 lines, proper frontmatter — misplaced at workspace root) |
| `Mathew Layman Speech.txt` | MOVE to `_legacy/` (personal name in filename; register needs conversion before any wiki use) |
| `Mathew Speech.txt` | MOVE to `_legacy/` (same) |
| `BIM_Buildable Architecture.pdf` | MOVE to `_legacy/` or relevant project data folder (binary at root violates convention; filename has spaces + uppercase) |
| `jennifer-sandbox/` | Investigate — AGENT.md specifies sandbox at `/home/<user>/sandbox/`, not `/srv/foundry/`; if active, move to `/home/jennifer/sandbox/` |
| `PRE_LAUNCH_SWEEP_20260321/` | Investigate contents; almost certainly ARCHIVE to `_legacy/` |

---

## PRIORITY 1 — Critical public GitHub errors

### P1a — `vendor/pointsav.github.io/index.html`

Three separate critical issues:

**Wrong copyright holder (line 440):**
- Current: `© 2011–<year> Woodfine Management Corp. All rights reserved.`
- Fix: `© 2011–<year> Woodfine Capital Projects Inc. All rights reserved.`

**OS-mark list in footer (line 432):**
- Current: Lists `PointSav Console OS, PointSav Infrastructure OS, PointSav MediaKit OS, PointSav Network OS, PointSav PrivateGit OS, PointSav Workplace OS, Totebox OS` as marks
- Fix: Replace with canonical five-mark list or remove the parenthetical entirely

**"con" — Spanish word in English press release (line 475):**
- Current: `"engineered to empower building operators con robust cyberphysical defense"`
- Fix: `"designed to provide building operators with a structured cyberphysical defence perimeter"`
- Note: tagged "FOR IMMEDIATE RELEASE" — highest visibility

**AI marketing vocabulary (lines 454, 482):**
- `empower`, `robust`, `rejects legacy flaws` — rewrite to Bloomberg standard

**Meta description (lines 12, 14):**
- `"Digital First Operating System"` — marketing copy; rewrite to factual description

**Commit tier:** admin-tier (`mcorp-administrator`)

---

### P1b — `vendor/pointsav-monorepo/README.md` + `LICENSE`

**License table contradicts LICENSE-MATRIX (README.md lines 114, 120, 121, 122):**

| OS module | README shows | LICENSE-MATRIX says | Fix |
|---|---|---|---|
| os-totebox | Apache 2.0 | AGPL-3.0-or-later | Update table |
| os-console | Apache 2.0 | AGPL-3.0-or-later | Update table |
| os-workplace | Apache 2.0 | AGPL-3.0-or-later | Update table |
| os-mediakit | Proprietary | FSL-1.1-ALv2 | Update table |
| os-orchestration | Proprietary | FSL-1.1-ALv2 | Update table |
| os-network-admin | Proprietary | FSL-1.1-ALv2 | Update table |

**Wrong jurisdiction in LICENSE (line 13):**
- Current: `"PointSav Digital Systems AG"` — AG is German/Swiss suffix
- Fix: `"PointSav Digital Systems Inc."` (BC posture per factory-release-engineering/README.md §6)

**Hardcoded commit count (README.es.md:91):**
- `"201 confirmaciones y en crecimiento"` — will silently go stale
- Fix: remove or replace with a generated badge

**Commit tier:** staging-tier

---

### P1c — `customer/woodfine-fleet-deployment` — wrong copyright holder in 58 guide files

All GUIDE-*.md footers read `*Copyright © 2026 Woodfine Management Corp. All rights reserved.*`
Copyright must vest in Woodfine Capital Projects Inc.

The `footer-guide-en.md` template was corrected in this session (commit 8b61173) but the 58
existing files predate the fix. A script pass is the right approach — see confirmed file list below.

**Also affected (non-guide files):**
- `customer/woodfine-fleet-deployment/README.md:146` — `*© 2026 Woodfine Management Corp.*`
- `customer/woodfine-fleet-deployment/README.es.md:109` — same (Spanish)
- `customer/content-wiki-corporate/README.md:68` — same
- `customer/content-wiki-corporate/README.es.md:53` — same
- `customer/woodfine.github.io/README.md:26` — same
- `customer/woodfine-media-assets/README.md:14` — same

**Complete list of 58 guide files with wrong copyright:**
```
customer/woodfine-fleet-deployment/guide-physical-egress.md
customer/woodfine-fleet-deployment/guide-mesh-execution.md
customer/woodfine-fleet-deployment/guide-telemetry-operations.md
customer/woodfine-fleet-deployment/node-console-operator/guide-console-operations.md
customer/woodfine-fleet-deployment/node-console-operator/guide-command-ledger.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/guide-linkedin-adapter.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/guide-personnel-ledger.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/guide-sovereign-search.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/guide-msft-entra-id.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/guide-totebox-orchestration.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/guide-slm-execution.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/guide-ingress-operations.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/guide-deployment.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/service-slm/guide-01-deployment.md
customer/woodfine-fleet-deployment/cluster-totebox-property/guide-deployment.md
customer/woodfine-fleet-deployment/cluster-totebox-property/guide-provision-node.md
customer/woodfine-fleet-deployment/fleet-infrastructure-cloud/guide-provision-relay.md
customer/woodfine-fleet-deployment/gateway-interface-command/guide-provision-node.md
customer/woodfine-fleet-deployment/fleet-infrastructure-cloud/guide-provision-node.md
customer/woodfine-fleet-deployment/gateway-interface-command/guide-deployment.md
customer/woodfine-fleet-deployment/gateway-orchestration-gis/guide-provision-node.md
customer/woodfine-fleet-deployment/gateway-orchestration-gis/guide-deployment.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/guide-cold-storage-sync.md
customer/woodfine-fleet-deployment/gateway-orchestration-bim/README.md
customer/woodfine-fleet-deployment/route-network-admin/guide-mesh-orchestration.md
customer/woodfine-fleet-deployment/fleet-infrastructure-cloud/guide-deployment.md
customer/woodfine-fleet-deployment/route-network-admin/guide-provision-node.md
customer/woodfine-fleet-deployment/cluster-totebox-corporate/guide-deployment.md
customer/woodfine-fleet-deployment/cluster-totebox-corporate/guide-provision-node.md
customer/woodfine-fleet-deployment/gateway-orchestration-gis/README.md
customer/woodfine-fleet-deployment/fleet-infrastructure-leased/guide-deploy-vpn.md
customer/woodfine-fleet-deployment/fleet-infrastructure-leased/guide-macos-endpoints.md
customer/woodfine-fleet-deployment/cluster-totebox-personnel/guide-provision-node.md
customer/woodfine-fleet-deployment/gateway-orchestration-bim/guide-deployment.md
customer/woodfine-fleet-deployment/fleet-infrastructure-leased/guide-peter-macbook.md
customer/woodfine-fleet-deployment/route-network-admin/guide-deployment.md
customer/woodfine-fleet-deployment/media-knowledge-corporate/guide-deployment.md
customer/woodfine-fleet-deployment/fleet-infrastructure-onprem/guide-lxc-network-admin.md
customer/woodfine-fleet-deployment/fleet-infrastructure-onprem/guide-provision-node.md
customer/woodfine-fleet-deployment/fleet-infrastructure-leased/guide-provision-standalone.md
customer/woodfine-fleet-deployment/fleet-infrastructure-onprem/guide-provision-onprem.md
customer/woodfine-fleet-deployment/media-knowledge-projects/guide-provision-node.md
customer/woodfine-fleet-deployment/media-knowledge-projects/guide-deployment.md
customer/woodfine-fleet-deployment/media-knowledge-corporate/guide-provision-node.md
customer/woodfine-fleet-deployment/fleet-infrastructure-onprem/guide-deployment.md
customer/woodfine-fleet-deployment/media-marketing-landing/guide-provision-marketing-site.md
customer/woodfine-fleet-deployment/gateway-orchestration-bim/guide-provision-node.md
customer/woodfine-fleet-deployment/media-marketing-landing/guide-telemetry-operations.md
customer/woodfine-fleet-deployment/media-marketing-landing/guide-telemetry-governance.md
customer/woodfine-fleet-deployment/vault-privategit-source/guide-open-archive.md
customer/woodfine-fleet-deployment/vault-privategit-source/guide-doorman.md
customer/woodfine-fleet-deployment/vault-privategit-source/guide-provision-node.md
customer/woodfine-fleet-deployment/vault-privategit-source/guide-operating-yoyo.md
customer/woodfine-fleet-deployment/media-marketing-landing/guide-deployment-marketing-site.md
customer/woodfine-fleet-deployment/vault-privategit-source/guide-doorman-deployment.md
customer/woodfine-fleet-deployment/vault-privategit-source/guide-deployment.md
customer/woodfine-fleet-deployment/vault-privategit-source/guide-tier-a-sysadmin-tui.md
customer/woodfine-fleet-deployment/vault-privategit-source/guide-command-session.md
```

**Suggested sed command (run from `customer/woodfine-fleet-deployment/`):**
```bash
find . -name "*.md" -exec grep -l "Woodfine Management Corp\." {} \; | \
  xargs sed -i 's/Copyright © 2026 Woodfine Management Corp\. All rights reserved\./Copyright © 2026 Woodfine Capital Projects Inc. All rights reserved./g'
```
Verify diff before committing.

**Commit tier:** mcorp-administrator admin-tier (woodfine repo)

---

### P1d — Personal name in customer-facing README

`customer/woodfine-fleet-deployment/README.md:72`:
- Current: `"an architecture Peter Woodfine describes as Geometric Security"`
- Fix: `"an architecture the Principal Manager describes as Geometric Security"`

`customer/woodfine-fleet-deployment/fleet-infrastructure-leased/guide-peter-macbook.md`:
- Filename contains personal name — rename to `guide-endpoint-macbook.md`

**Commit tier:** mcorp-administrator admin-tier

---

### P1e — Foundry vocabulary leaks in published content-wiki-documentation TOPICs

Sub-phase 2j claimed "final grep clean" — 7 residual leaks remain:

**Title leaks (YAML frontmatter + document heading):**
- `clones/project-editorial/content-wiki-documentation/architecture/foundry-doctrine-architecture.md:3` — title: `"Foundry — Architectural Overview"`
- `clones/project-editorial/content-wiki-documentation/architecture/foundry-doctrine-architecture.es.md:3` — title: `"Foundry — Visión Arquitectónica"`
- `clones/project-editorial/content-wiki-documentation/architecture/foundry-doctrine-overview.md:3` — title: `"Foundry Architecture 2030 — An Overview"`
- `clones/project-editorial/content-wiki-documentation/architecture/foundry-doctrine-overview.es.md:3` — title: `"Doctrina Foundry 2030 — Resumen"`

Fix titles to: "PointSav Platform Architecture 2030 — Public Charter Overview" (and ES equivalent).
Slugs (`foundry-doctrine-*.md`) are immortal once published — retain filenames, update titles only.
Or add `aliases:` frontmatter pointing to `platform-architecture-overview`.

**Body text leaks:**
- `reference/editorial-language-registers.md:12` — "The three Foundry wikis"
- `reference/editorial-language-registers.md:16` — "The Foundry wikis are written in three distinct registers"
- `reference/editorial-language-registers.es.md:14` — "Los tres wikis de Foundry"

Fix: "The three PointSav content wikis" / "Los tres wikis de contenido de PointSav"

**Commit tier:** staging-tier (content-wiki-documentation cluster branch)

---

### P1f — Personal name / staging identity leaks in published TOPICs

Git usernames and internal identity names (jwoodfine, pwoodfine, ps-administrator,
mcorp-administrator) appear in body text of 6 published architecture/governance TOPICs.
Replace with role nouns ("staging-tier contributor", "vendor administrator", "customer administrator").

- `architecture/five-stage-supply-chain.md:34,41,45,65` — identity table with git usernames
- `architecture/three-layer-architecture.md:39,54` — ps-administrator, mcorp-administrator
- `architecture/pointsav-overview.md:49` — "jwoodfine and pwoodfine are the two established staging contributors"
- `governance/legal-and-ip-structure.md:40,72,73` — same pattern
- `governance/sovereign-airlock-doctrine.md:23,24,42,43,44,45` — full identity table exposed
- `architecture/totebox-session.md:24,40,58,66,68` — internal `.agent/` paths in body text

**Commit tier:** staging-tier

---

## PRIORITY 2 — Root-cause template fix (cascades downstream)

### `factory-release-engineering/readmes/license-section-en.md` and `license-section-es.md`

These templates are the upstream source of two bugs appearing in every repo that used the
propagation scripts:
1. `Copyright (c)` instead of `Copyright ©`
2. `Inc..` double period (`${copyright_holder}` = `"Woodfine Capital Projects Inc."` + template's own period)

**Fix (both files, line 14):**
- Current EN: `Copyright (c) ${year} ${copyright_holder}. All rights not expressly`
- Fix EN:     `Copyright © ${year} ${copyright_holder} — all rights not expressly`
- Current ES: `Copyright (c) ${year} ${copyright_holder}. Se reservan todos los`
- Fix ES:     `Copyright © ${year} ${copyright_holder} — se reservan todos los`

**Downstream files to fix after template update (confirmed `Inc..` double period + `(c)`):**
- `vendor/content-wiki-documentation/README.md:139,155`
- `vendor/pointsav-fleet-deployment/README.md:29,45`
- `vendor/pointsav.github.io/README.md:42,58`
- `customer/woodfine-fleet-deployment/README.md:164,180`
- `customer/woodfine.github.io/README.md:42,58`
- `customer/woodfine-media-assets/README.md:30,46`

**Commit tier:** admin-tier (factory-release-engineering = ps-administrator), then per-repo commits

---

## PRIORITY 3 — Stale TRADEMARK.md files

Multiple repos carry the old TRADEMARK.md with `Foundry™`, `ToteboxOS™`, `ConsoleOS™`,
`OrchestrationOS™`, `WorkplaceOS™`, `WoodfineGroup™`. Replace all with canonical form
(same content as the updated `factory-release-engineering/TRADEMARK.md`).

| File | Issues |
|---|---|
| `vendor/content-wiki-documentation/TRADEMARK.md` | Foundry™, ToteboxOS™, ConsoleOS™, OrchestrationOS™, WorkplaceOS™, WoodfineGroup™; "PointSav Foundry API" in nominative-use example |
| `vendor/pointsav-design-system/TRADEMARK.md` | Same stale marks; §6 still references PointSav-ARR as design-system license (it moved to Apache 2.0) |
| `vendor/pointsav-monorepo/TRADEMARK.md` | Same stale marks |
| `vendor/pointsav-media-assets/TRADEMARK.md` | Likely stale — same template origin |
| `customer/woodfine-media-assets/TRADEMARK.md` | Likely stale |

Also: `CODE_OF_CONDUCT.md:39` has `[INSERT CONTACT METHOD]` placeholder unresolved in:
- `vendor/content-wiki-documentation/policies/CODE_OF_CONDUCT.md`
- `vendor/factory-release-engineering/policies/CODE_OF_CONDUCT.md`

Fix: substitute `open.source@pointsav.com`

**Commit tier:** admin-tier (vendor repos = ps-administrator; customer repos = mcorp-administrator)

---

## PRIORITY 4 — "Foundry" vocabulary in public GUIDE files

`guide-doorman.md` is the worst offender (15+ occurrences).
Pattern: "Foundry workspace VM" / `~/Foundry/` path references in operational runbooks.
Replace with "vault-privategit-source-1" or "the workspace VM".

**Key files (customer/woodfine-fleet-deployment/):**
- `vault-privategit-source/guide-doorman.md` — 15+ occurrences including "on the Foundry workspace VM"
- `vault-privategit-source/guide-operating-yoyo.md:231` — "surface via the Master inbox (on the Foundry workspace VM)"
- `vault-privategit-source/guide-tier-a-sysadmin-tui.md:16` — "run the Foundry substrate"
- `gateway-orchestration-gis/guide-gis-adding-a-chain.md:150` — `~/Foundry/bin/commit-as-next.sh`
- `gateway-orchestration-gis/guide-deployment.md:27,32` — `~/Foundry/deployments/`
- `gateway-orchestration-bim/guide-deployment.md:27` — same
- `route-network-admin/README.md:17` — `Linux Mint (Foundry)` in table
- `fleet-infrastructure-leased/guide-deploy-vpn.md:29` — `/home/mathew/Foundry/...` (also leaks username)
- `NEXT.md:4,33,34,37` — `~/Foundry/NEXT.md` references (lower priority — operational file)

**Key files (vendor/pointsav-fleet-deployment/):**
- `media-knowledge-documentation/guide-operate-knowledge-wiki.md:26,37,93,154`
- `media-knowledge-documentation/guide-keep-the-home-page-the-gold-standard.md:34,67`
- `media-marketing-landing/guide-deployment-marketing-site.md:22,60`

Note: `X-Foundry-*` HTTP headers in guide-doorman.md are part of the API protocol and
cannot be renamed without breaking the API — leave those unchanged.

**Commit tier:** mcorp-administrator (customer); ps-administrator (vendor)

---

## PRIORITY 5 — content-wiki-projects Bloomberg sweep + structural fixes

The co-location country index files were never subjected to a Bloomberg vocabulary sweep.

### AI marketing vocabulary (all 8 country indexes + tier index)

Common patterns across `topic-co-location-index-{us,canada,italy,mexico,spain,poland,nordics}.md`
and `topic-tier-index-north-america.md`:

| Word/phrase | Bloomberg fix |
|---|---|
| "Utilizing" | "Using" |
| "synergy" / "retail synergy" | "co-location clustering" or "commercial clustering" |
| "sophisticated" | remove or replace with specific descriptor |
| "world-class" | remove or replace with measurable threshold |
| "exceptional" / "remarkable" / "prestigious" | remove |
| Michelin metaphors ("three-star Michelin rating", "worth a special journey") | Remove entirely |
| "institutional-grade" | "established" or "mature" |
| "robust" | "structured" or specific descriptor |
| "definitive benchmark" | "ranking threshold" |

### Structural defects (all 17 projects-wiki TOPIC pairs = 34 files)

**Provenance blocks** — expose internal cluster names; remove entirely from all affected files:
- `topic-co-location-index-canada.md:52-53` (+ `.es.md`)
- `topic-co-location-intelligence-overview.md:84-85` (+ `.es.md`)
- `topic-co-location-ranking-system.md:103-104` (+ `.es.md`)
- `topic-tier-index-north-america.md:48-49` (+ `.es.md`)
- `topic-co-location-methodology.md:68-69` (+ `.es.md`)

**Frontmatter `category: governance`** — wrong on all 17 projects-wiki topics:
- Fix: `category: root` (matches `index.md` pattern for flat single-level wikis)

**Missing `topic-` prefix in `paired_with:`** frontmatter — fix on ≥3 files:
- `topic-tier-index-north-america.md:14` → change `tier-index-north-america.es.md` to `topic-tier-index-north-america.es.md`
- `topic-co-location-index-canada.md:14` → same pattern fix
- `topic-co-location-methodology.md:14` → same

**Missing copyright footers** — add to 19 files (CC BY-ND 4.0 notice):
- All 8 country-index `.md` files
- All 8 country-index `.es.md` files  
- `topic-co-location-methodology.es.md`
- `topic-co-location-ranking-system.es.md`
- `topic-tier-index-europe.md` + `.es.md`

Footer format (CC BY-ND 4.0 for projects wiki):
```
---

*Copyright © 2026 Woodfine Capital Projects Inc. Licensed under [Creative Commons Attribution-NoDerivatives 4.0 International](https://creativecommons.org/licenses/by-nd/4.0/).*

*Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™, Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital Projects Inc., used in Canada, the United States, Latin America, and Europe. All other trademarks are the property of their respective owners.*
```

**Corporate wiki index status:**
- `customer/content-wiki-corporate/index.md:6` and `index.es.md:8` — `status: pre-build` → `status: active`
  (wiki is publicly launched per overhaul-progress.md)

**Commit tier:** mcorp-administrator (customer/woodfine repos)

---

## PRIORITY 6 — `(c)` → `©` and `(TM)` → `™` remaining sweep

Files not touched in the 2026-05-16 factory-release-engineering session:

**vendor/pointsav-design-system/**
- `TRADEMARK.md:4` — `Copyright (c) 2026`
- `SECURITY.md:4` — `Copyright (c) 2026`
- `README.md:107,126` — `Copyright (c) 2026`
- `CITATION.cff:9` — `"Foundry Doctrine 2030"` leak

**vendor/pointsav-media-assets/**
- `SECURITY.md:4` — `Copyright (c) 2026`
- `TRADEMARK.md:4` — `Copyright (c) 2026`

**vendor/pointsav-fleet-deployment/**
- `SECURITY.md:4` — `Copyright (c) 2026`

**vendor/factory-release-engineering/** (vendor/ copy, behind cluster by 9+ commits — needs git pull):
- `cla/individual-cla.md:4` — `Copyright (c) 2026`
- `cla/corporate-cla.md:4` — `Copyright (c) 2026`
- `policies/CONTRIBUTING.md:4` — `Copyright (c) 2026`
- `policies/SECURITY.md:4` — `Copyright (c) 2026`
- `licenses/MIT.txt:3` — `Copyright (c) 2026`
- `licenses/MIXED-MONOREPO-NOTICE.txt:3` — `Copyright (c) 2026`

Note: `vendor/factory-release-engineering/` is a local checkout behind our cluster commits.
A `git pull` there will pick up our 2026-05-16 session's 9 commits automatically.

---

## PRIORITY 6b — BCSC disclosure language in public READMEs

`pointsav-monorepo/README.md` and `pointsav-design-system/README.md` contain definitive
present-tense claims about features shown as 🟡 Development in the status table.

**pointsav-monorepo/README.md:**
- "PointSav **stores** all records as inert flat files"
- "SHA-256 cryptographic checksums **seal** every record"
- "The end-state export format **for every** ToteboxOS archive **is** a Bootable Disk Image"
- Multiple similar definitive claims at lines 51, 55, 57, 69, 73, 75, 77, 84, 87, 93

**pointsav-design-system/README.md:**
- "The system **is** the law" (line 27)
- "Compliance integrity **is** preserved" (line 78)
- "This **prevents** legal fracture from appearing in bilingual documents" (line 46)
- "consumed automatically by PointSav deployment environments **and Woodfine Management Corp.'s fleet**" (line 84) — also contradicts the WMC non-operating posture in factory-release-engineering §6

Fix: reword to "is designed to / intended to / will" for each claim.

---

## PRIORITY 7 — project-editorial drafts-outbound cleanup

All items below are confirmed committed (closed per handoffs-outbound tracker).
Safe to delete or archive.

**At `.agent/drafts-outbound/` root (27 zip-topic-* files):**
All 27 `zip-topic-*.md` files — committed to respective wikis; stale source drafts.

**Component drafts (committed long ago):**
- `component-*.draft.md` (4 files, May 3)
- `research-zoom-tier-reveal-pattern.draft.md`
- `design-main-page-token-2.draft.md` (verify against design-system commits first)

**Investigate before deleting:**
- `architecture-layer-catalog.md` — no `foundry-draft-v1` frontmatter; purpose unclear
- `framework-pointsav-wiki-structure.md` — target path doesn't exist; classify

**archive-2026-04/ subtree — all entries 1-9 are `closed`:**
- `archive-2026-04/leapfrog-2030/{architecture,conventions,doctrine,guides,inventions,topics}/`
- `archive-2026-04/from-project-knowledge-guides/`
- `archive-2026-04/topic-home.draft.md`, `topic-home.es.draft.md`, etc.

**refined/ subdirectories:**
- `refined/co-location/woodfine/` — 11 source files; entry 10 closed per tracker
- `refined/co-location/vendor/` — 6 files; verify destination state before deleting
- `refined/footer-templates/` — 4 files (May 2) older than current TOPIC footer pattern; archive
- `refined/sweep-2026-05-03/bim/` and `data/` — empty directories; remove

**from-project-gis/ (9 files May 5-6):**
Correspond to articles now published; verify and clean.

**Stale outbox message:**
`.agent/outbox.md` bottom message (`task@claude-code → totebox@gemini-cli`, 2026-05-14T20:30,
`status: stale`) — archive to `outbox-archive.md`.

---

## PRIORITY 8 — Internal governance document staleness

All internal-only (not public GitHub). Lower urgency but affects session quality.

### DOCTRINE.md
- `.claude/` paths → `.agent/` throughout (lines 88, 102, 107, 451, 542, 595)
- `Root/Task/Master` → `Command/Totebox/Sandbox` vocabulary in §II and §V Action Matrix
- `project-slm` → `project-intelligence` (line 748 + §X + §XIII roadmap rows)
- Port `8080` → `9080` for local-doorman (line 745)
- Roadmap contradiction: v0.1.0 listed as both current ALPHA (header) and Q3 2026 target (§XIII)
- `bin/claude-role.sh` vs `bin/foundry-role.sh` — DOCTRINE, CLAUDE.md, AGENT.md disagree on canonical name

### MANIFEST.md
- "13 active cluster archives" → "16 active" (Phase 2 closed 2026-05-14)
- Add `project-source`, `project-woodfine`, `project-infrastructure` to archive list
- Phase 2 and Phase 3 (shell scripts portion) → mark closed; update to 2026-05-14 ratification
- `project-slm` → `project-intelligence` in Ring 1/Ring 2 references (lines 60-66)
- Add lineage rows v0.1.27 → v0.1.132 (or note CHANGELOG.md as source for this range)

### PROJECT-CLONES.md
- `.claude/manifest.md` → `.agent/manifest.md` (lines 7-10, 17)
- "Master Claude" / "Task Claude" → "Command Session" / "Totebox Session" throughout
- `clones/project-slm/.agent/inbox.md` → `clones/project-intelligence/.agent/inbox.md`

### POINTSAV-Project-Instructions.md
- "Nomenclature Matrix V8" / "MEMO V8" → add note: superseded by `conventions/nomenclature-taxonomy.md`
- `Qwen, Phi, or equivalent` (lines 232, 357) → `OLMo 3 7B Think Q4 (or equivalent open-source model)`
- `service-slm` references → `service-intelligence` (or note rename)
- Vendor Quarantine table line 492: `Phi-3 Mini SLM engine` → OLMo 3

### NEXT.md
- Current: 584 lines (cap: 200 lines, 40 open items)
- Action: bulk archive closed items to `.agent/next-backlog.md`
- Keep only open `[ ]` items; strike or remove `[x]` items

### CLAUDE.md §15 conventions list
- Approximately 22 convention files exist in `conventions/` but are not listed in §15
- Add: `single-boundary-compute-discipline`, `customer-owned-graph-ip`, `reverse-flow-substrate`,
  `four-tier-slm-substrate`, `data-vault-bookkeeping-substrate`, `design-system-substrate`,
  `api-key-boundary-discipline`, `agent-file-size-discipline`, `direct-payment-settlement`,
  `master-sweep-cadence`, `reverse-funnel-editorial-pattern`, `institutional-content-return`,
  `mailbox-message-lifecycle`, and others referenced in DOCTRINE.md / CHANGELOG.md

---

## Session-to-session handoff notes

### What was fixed this session (2026-05-16, project-editorial cluster)

factory-release-engineering commits pushed to GitHub (9 commits, `8b61173` → `e8262e9`):
- `readmes/footer-guide-en.md` — copyright holder: WMCorp → WCP Inc.
- `policies/DISCLAIMER.md` — OS-mark parenthetical removed; canonical trademark notice applied
- `CITATION.cff`, `README.md`, `PLAYBOOK.md` — 4 "Foundry" vocabulary leaks removed
- `cla/cla-assistant-config.yml` — 8 standalone repos → pointsav-monorepo
- `README.md` — ™ fix, directory tree synced to actual files, stale duplicate footer removed
- `LICENSE-MATRIX.md`, `PLAYBOOK.md`, `NEXT.md` — canonical copyright+trademark blocks added
- `readmes/footer-readme-es.md` — new Spanish README footer template
- `licenses/PointSav-ARR.txt`, `licenses/PointSav-Commercial.txt` — `(c)` → `©`, `(TM)` → `™`
- `policies/TRADEMARK.md` — §1 asymmetry note added; Totebox Orchestration™ + Totebox Archive™ in marks list

### What still needs doing (ordered by priority)

1. **P0** — Operator review + deletion of injection scripts (Command Session)
2. **P1a** — pointsav.github.io copyright + "con" typo + marketing copy
3. **P1b** — pointsav-monorepo license table + LICENSE AG→Inc.
4. **P1c** — 58 guide files wrong copyright holder (script pass, mcorp-administrator)
5. **P1d** — Personal name in woodfine-fleet-deployment/README.md + filename
6. **P1e** — Foundry title leaks in 4 TOPIC files (architecture/ category)
7. **P1f** — Identity leaks (jwoodfine/pwoodfine) in 6 published TOPICs
8. **P2** — Template fix license-section-*.md + downstream README corrections
9. **P3** — Stale TRADEMARK.md files + CODE_OF_CONDUCT placeholder
10. **P4** — Foundry vocabulary in GUIDE files (guide-doorman.md heaviest)
11. **P5** — content-wiki-projects Bloomberg sweep + structural fixes
12. **P6** — Remaining `(c)` → `©` sweep + BCSC softening
13. **P7** — Stale drafts-outbound cleanup (project-editorial Totebox Session)
14. **P8** — Internal governance doc staleness (Command Session)

---

*Audit document. Do not edit — update by appending a dated amendment section.*
*Generated: 2026-05-16. Sources: 4× OPUS agents + targeted grep passes.*

---

## Amendment — 2026-05-16 (project-editorial continuation session)

### Items completed this session

**P1c — woodfine-fleet-deployment: 58 guide files wrong copyright holder**
Status: **CLOSED**
Fix applied via `find … -exec sed -i …` pass across all 58 files.
Commit: `6eead9a` (jwoodfine, staging-tier). Pushed to `origin/main`.
Note: The audit listed `guide-peter-macbook.md` in the 58-file list. That file was also
renamed in P1d below, so both operations landed together.

**P1d — Personal name in README + guide-peter-macbook.md**
Status: **CLOSED**
- `fleet-infrastructure-leased/guide-peter-macbook.md` renamed → `guide-endpoint-macbook.md` via `git mv`
- Slug updated: `guide-peter-macbook` → `guide-endpoint-macbook`
- `**OPERATOR:** Peter` → `**OPERATOR:** Principal Manager`
- `peter-mexico.conf` → `endpoint-mexico.conf` (2 occurrences)
- `contact Jennifer on the internal support desk` → `contact the PointSav support desk`
- `README.md:72` — `"an architecture Peter Woodfine describes as Geometric Security"` → `"an architecture Woodfine describes as Geometric Security."`
Commit: `4681525` (pwoodfine, staging-tier). Pushed to `origin/main`.

**P1e — Foundry vocabulary leaks in 4 TOPIC titles + 2 body files**
Status: **CLOSED**
Title fixes (YAML frontmatter + H1 heading only; slugs retained per slug-immortality rule):
- `foundry-doctrine-overview.md`: `"Foundry Architecture 2030 — An Overview"` → `"PointSav Architecture 2030 — An Overview"`
- `foundry-doctrine-overview.es.md`: `"Doctrina Foundry 2030 — Resumen"` → `"PointSav Arquitectura 2030 — Resumen"`
- `foundry-doctrine-architecture.md`: `"Foundry — Architectural Overview"` → `"PointSav Platform — Architectural Overview"`
- `foundry-doctrine-architecture.es.md`: `"Foundry — Visión Arquitectónica"` → `"PointSav Platform — Visión Arquitectónica"`
Body fixes:
- `reference/editorial-language-registers.md:12,16` — `"the Foundry wikis"` / `"The Foundry wikis"` → `"the PointSav wikis"` / `"The PointSav wikis"`
- `reference/editorial-language-registers.es.md:14` — `"Los tres wikis de Foundry"` → `"Los tres wikis de PointSav"`
Commit: `cf083bf` (jwoodfine, staging-tier). Pushed to `origin/main`.

**P2 — Template root-cause fix (license-section-en.md + license-section-es.md)**
Status: **CLOSED**
Both template files fixed:
- `Copyright (c)` → `Copyright ©`
- Trailing period on `${copyright_holder}.` → em-dash (`— all rights…`)
Eliminates `Inc..` double-period and `(c)` in all future propagation runs.
Commit: `0998320` (ps-administrator, admin-tier). Pushed to `origin/main`.

**P1f — Identity handle leaks in 6 published TOPICs**
Status: **EVALUATED — NO CHANGE**
Determination: `jwoodfine`, `pwoodfine`, `ps-administrator`, `mcorp-administrator` in
architecture/governance TOPICs (`five-stage-supply-chain.md`, `three-layer-architecture.md`,
`pointsav-overview.md`, `legal-and-ip-structure.md`, `sovereign-airlock-doctrine.md`,
`totebox-session.md`) are legitimate technical architecture descriptors — they describe
organizational commit-flow roles, not personal names. The "no personal names" rule targets
first names (Jennifer/Peter/Mathew). Git handles in architecture documentation are outside scope.
No changes made. Item closed.

### Item partially attempted — blocked

**P1b — pointsav-monorepo README.md + README.es.md footer cleanup**
Status: **BLOCKED — carry forward**

`origin/main` current state (SHA `3e873ea4`):
- `README.md:173` — stale old footer still present: `*© 2026 PointSav Digital Systems™. Apache 2.0 licensed components are governed by the terms of that licence. Proprietary components are all rights reserved.*`
- `README.md:178-182` — canonical footer already present (added by a prior session)
- `README.es.md:101` — stale old footer present: `*© 2026 PointSav Digital Systems™. Los componentes con licencia Apache 2.0 se rigen por los términos de dicha licencia. Los componentes propietarios están todos los derechos reservados.*`
- `README.es.md` — NO canonical Spanish footer added yet

Local branch state:
- `editorial-readme-fix` — clean, tracking `origin/main` (correct working branch)
- `main` (local only) — has orphaned commit `3b3933a0` (`docs: fix copyright holder and trademark notice in README footers`) that was never pushed; the push was rejected (non-fast-forward) because `origin/main` was 3 commits ahead. This local commit is partially superseded by what is already on `origin/main`.

**Next session action:**
1. `cd /srv/foundry/clones/project-editorial/pointsav-monorepo`
2. Confirm on `editorial-readme-fix` branch (tracking `origin/main`)
3. Remove `README.md` lines 173-174 (stale footer) — keep lines 178-182 canonical block
4. Remove `README.es.md` line 101 (stale footer)
5. Add canonical Spanish footer to `README.es.md` (from `factory-release-engineering/readmes/footer-readme-es.md`)
6. Commit via `commit-as-next.sh` inside subshell: `(cd /srv/foundry/clones/project-editorial/pointsav-monorepo && git add README.md README.es.md && ~/Foundry/bin/commit-as-next.sh "docs: remove stale legacy footers; add canonical Spanish footer block")`
7. Push `editorial-readme-fix` to `origin` and open PR, or push directly to `main` if the branch divergence is resolved by rebasing
8. Discard/ignore local `main` branch commit `3b3933a0` — do NOT force-push, do NOT amend

**Also remaining in P1b (requires operator decision):**
- `LICENSE:13` — `"PointSav Digital Systems AG"` AG suffix was never incorporated; correct entity is BC Inc. But the license body also says "no open-source license is granted" which contradicts the README's Apache 2.0 claims. Operator must decide whether to update LICENSE to reflect current Apache 2.0 reality or retain incubation-phase language. Do not change without operator direction.
- README.md license table (lines 100-106) — 3 OS modules show Apache 2.0 but LICENSE-MATRIX says AGPL-3.0-or-later; requires operator confirmation of correct license before changing the table.

### Updated priority queue

1. **P0** — Operator review + deletion of injection scripts (Command Session, requires approval)
2. **P1a** — pointsav.github.io copyright + "con" typo + marketing copy (not in project-editorial cluster — check PROJECT-CLONES.md for correct archive)
3. **P1b** — pointsav-monorepo README.md + README.es.md footer cleanup (see blocked item above)
4. **P2 downstream** — Template fix landed; downstream READMEs with `Inc..` still need individual updates (see P2 list in original audit)
5. **P3** — Stale TRADEMARK.md files + CODE_OF_CONDUCT placeholder
6. **P4** — Foundry vocabulary in GUIDE files (guide-doorman.md heaviest)
7. **P5** — content-wiki-projects Bloomberg sweep + structural fixes
8. **P6** — Remaining `(c)` → `©` sweep + BCSC softening
9. **P7** — Stale drafts-outbound cleanup (project-editorial Totebox Session)
10. **P8** — Internal governance doc staleness (Command Session)
