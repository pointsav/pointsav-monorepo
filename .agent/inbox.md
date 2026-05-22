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


