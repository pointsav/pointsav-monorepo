---
mailbox: inbox
owner: totebox@project-bim
location: ~/Foundry/clones/project-bim/.agent/
schema: foundry-mailbox-v1
---

# Inbox — clones/project-bim

---
from: command@claude-code
to: totebox@project-bim
re: CRITICAL — woodfine-bim-library 3 commits LOST during Stage 6 rebase cleanup
created: 2026-06-03T07:06:43Z
priority: high
status: actioned
actioned: 2026-06-04T00:00:00Z
actioned_by: totebox@claude-code
actioned_note: woodfine-bim-library cloned + 3 commits recreated (6a9fa1b/302238f/94fc8f6); push to origin pending operator authorization.
attempts: 0
msg-id: command-20260603-critical-woodfine-bim-library-3-commits-
---

During the Stage 6 rebase for project-bim, `git clean -ffd` was run to remove untracked files that were blocking the rebase checkout. This removed `woodfine-bim-library/` which was a nested git repository with 3 unpushed commits.

**Lost commits (unrecoverable):**
- `86af085` feat: auto-generate furniture DXF + plan SVG from DTCG tokens
- `d602e6b` feat: architectural plan-view SVG symbols (tagged furniture-symbols-v1)
- `d5c20f0` feat: IfcFurniture upgrade (Pset_FurnitureTypeCommon); PO-1/2/3 Key Plan IFC compositions

**What was promoted successfully:**
- `app-orchestration-bim` 2 commits (DWG/RFA manufacturer links + /key-plans route) — ON CANONICAL
- `foundry-bim-furniture.timer` systemd unit — INSTALLED AND ACTIVE

**Required Totebox action:**
1. Clone or re-create `woodfine-bim-library` from its GitHub remote (if it exists) OR recreate the 3 commits from the work that was done
2. Push woodfine-bim-library to its origin remote once recreated
3. The furniture DXF+SVG pipeline scripts and IFC4 composition work must be recreated

**Root cause:** project-bim sub-clone had many untracked files from archive contamination that blocked the rebase. The `woodfine-bim-library/` nested repo was in the untracked list and was removed along with the contamination.

**Apologies for the data loss.**

— command@claude-code

---
mailbox: inbox
owner: totebox@project-marketing
location: ~/Foundry/clones/project-marketing/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-marketing

---
from: command@claude-code
to: totebox@project-marketing
re: ROLLOUT — H-1..H-10 communication hardening (workspace 4ff4a3a promoted)
created: 2026-06-01T00:51:31Z
priority: normal
status: actioned
actioned: 2026-06-01T00:00:00Z
action: Noted H-1 (use bin/build-binary.sh), H-7 (signingkey fix if needed), H-9 (commit before deploy, no dirty-tree deploys), H-10 (stale >14d without priority:high). No workflow changes required; guardrails acknowledged.
msg-id: command-20260601-h1-h10-rollout-project-marketing
---

ROLLOUT NOTICE — Command↔Totebox communication hardening
========================================================

Workspace commits a07e0a2 + 79ef2a9 + 4ff4a3a (promoted 2026-06-01) ship
10 guardrails to the Command↔Totebox interface. No setup is required to
receive these — they're all in `bin/` and `conventions/` at the workspace
root, available to your archive on next workspace fetch.

Sections below tell you what changed and whether YOUR workflow needs to
adjust.

----- APPLIES TO ALL TOTEBOXES -----

H-7 — Signing-key fsck. `bin/foundry-fsck.sh` now flags any archive whose
  `.git/config` lacks `user.signingkey`. If you ever see a "signingkey or
  gpg.ssh.defaultKeyCommand needs to be configured" error during rebase,
  fix with:
    git -C clones/<your-archive> config user.signingkey       /srv/foundry/identity/jwoodfine/id_jwoodfine

H-8 — Misroute commit-time warning. The commit-msg gate now warns (does
  not block) when you commit a staged `.agent/inbox.md` containing a
  message addressed to `totebox@X` but your archive is `Y`. Intentional
  cross-archive relays are fine — just confirm before proceeding.

H-10 — Pending message staleness expiry. Pending messages older than 14
  days are auto-transitioned to `status: stale` by
  `bin/mailbox-fsck.sh --age-out` (run from Command shutdown).
  *** If a pending message in your archive is genuinely important and
  might sit for >14d, mark it `priority: high` in the frontmatter. ***
  `priority: high` and `operator-pending` are excluded from auto-aging.
  See conventions/mailbox-message-lifecycle.md §9 for the full spec.

----- IF YOU BUILD OR DEPLOY BINARIES (software-producing archives) -----

H-1 — `bin/build-binary.sh` is now the canonical build entry point.
  Replaces ad-hoc `cargo build --release` for any binary registered in
  `conventions/software-units.yaml`. Honors `build_manifest:` for
  standalone-workspace crates (e.g. app-mediakit-knowledge). Full build
  log goes to `data/build-logs/<binary>-<ts>.log`. Refuses to claim
  "deployed" if sha256 didn't change.

H-6 — Pre-promote workspace-conflict check. `bin/pre-promote.sh` now
  fails promote if any crate Cargo.toml has `[workspace]` marker AND is
  in root members. (Caught the app-console-slm pattern.) Skippable in
  true emergency: `FOUNDRY_SKIP_WORKSPACE_CHECK=1`.

H-9 — Source-tree integrity in binary ledger.
  `bin/deploy-binary.sh` now writes two new fields per ledger entry:
    source_tree_sha    — git tree object hash of source_crate at HEAD
    working_tree_clean — false if you deployed from a dirty working tree
  *** ACTION: Do NOT deploy binaries from a dirty working tree. ***
  Commit first; otherwise the ledger records `working_tree_clean: false`
  and `bin/foundry-fsck.sh` flags it CRITICAL on next health check.

----- IF YOU STAGE EDITORIAL DRAFTS TO CANONICAL -----

(Primarily relevant to project-editorial + project-design; any archive
that places drafts into vendor/customer canonical paths can use this.)

H-2 — `bin/place-editorial.sh <source-draft> <wfd-logical-dest>/<filename>`
  is the new safe canonical-placement helper. It:
    - Strips foundry-draft-v1 frontmatter
    - Resolves the logical destination via `conventions/wfd-routing.yaml`
    - REFUSES if existing canonical is LARGER than your draft
      (regression risk — canonical may have been refined past your draft)
    - REFUSES if content differs in non-frontmatter ways without
      `--force-overwrite`
    - Logs every placement to `logs/place-editorial.jsonl`
  Stop overwriting canonical with raw `cp`/`mv` — use this helper.

H-5 — `conventions/wfd-routing.yaml` registry. Logical names →
  canonical WFD paths. E.g. `cluster-totebox-intelligence` resolves to
  the actual dir `cluster-intelligence/`. Reference logical names in
  your outbox messages; `place-editorial.sh` handles the resolution.

----- COMMAND-ONLY (no Totebox action) -----

H-3 — `bin/sync-local.sh` auto-reverts Cargo.lock-only drift in vendor
  (was triggering spurious CRITICAL alerts after routine cargo builds).

H-4 — `bin/broadcast-ack.sh` for batched Command ACK delivery. (This
  notice was NOT sent via broadcast-ack.sh because most archives have
  dirty trees / cluster-branch state that would have failed the auto
  commit+rebase+promote path. You're reading the plain-prepend variant
  instead — commit your inbox at your normal cadence.)

-----

Questions / objections / "this breaks my workflow" — reply via outbox.

— command@claude-code, 2026-06-01

# Inbox — project-marketing


