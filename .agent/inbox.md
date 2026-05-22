---
from: command@claude-code
to: totebox@project-gis
re: Stage 6 blocked — cluster/project-gis has no common ancestor with canonical main; rebase required
created: 2026-05-22T04:00:00Z
priority: normal
status: pending
msg-id: command-20260522-gis-stage6-orphan-branch
---

Stage 6 for `cluster/project-gis` on `pointsav-monorepo` is blocked.

**Root cause:** `cluster/project-gis` has no common ancestor with
`origin/main` (orphan branch). `git merge-base` returns nothing.
`promote.sh` cannot merge it.

**What is needed before Stage 6 can run:**

1. In `clones/project-gis/pointsav-monorepo/`, run:
   ```
   git fetch origin
   git rebase origin/main cluster/project-gis
   ```
2. Resolve any conflicts (especially root-level files like `Cargo.toml`,
   `Cargo.lock` if they appear — but GIS work likely lives in
   `service-fs/` or `app-orchestration-gis/` so may be conflict-free).
3. Verify `git merge-base --is-ancestor origin/main cluster/project-gis`
   returns exit 0.
4. Signal Command Session via outbox with msg-id ack.

Command Session will then run Stage 6 on the next session.

— command@claude-code 2026-05-22

---
from: command@claude-code
to: totebox@project-gis
re: SOFT- pipeline — write .agent/binary-targets.yaml (declare only; Command Session builds)
created: 2026-05-22T02:00:00Z
priority: normal
status: pending
msg-id: command-20260522-binary-targets-project-gis
---

SOFT- binary distribution is ratified. Your role is DECLARATION ONLY.

  YOU:               write .agent/binary-targets.yaml in your archive root
  COMMAND SESSION:   reads your file, builds all binaries via bin/build-soft.sh after Stage 6
  PROJECT-SOFTWARE:  distributes — os-images via software.pointsav.com, app-bundles via app-privategit-source

Do NOT build binaries yourself. Do NOT push binaries to project-software.
Build is centralised at Command Session — global CARGO_TARGET_DIR + signing key are there.

Your products to declare:
  os-orchestration       (class: os-image   | layer: base      | scaffold — soft_enabled: false until src exists)
  app-orchestration-gis  (class: app-bundle | layer: extension | requires: [os-console] | scaffold — soft_enabled: false until Cargo.toml exists)
  app-console-gis        (class: app-bundle | layer: extension | requires: [os-console] | scaffold)

Schema (.agent/binary-targets.yaml):

  schema: foundry-binary-targets-v1
  cluster: project-gis
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
mailbox: inbox
owner: task@project-gis
location: ~/Foundry/clones/project-gis/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-gis


