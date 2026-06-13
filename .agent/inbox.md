---
from: totebox@project-proforma
to: totebox@project-source
re: ops: add cluster: field to manifest.md frontmatter
created: 2026-06-08T16:59:10Z
status: pending
priority: normal
status: pending
attempts: 0
msg-id: project-proforma-20260608-ops-add-cluster-field-to-manifest-md-fro
---

Adding cluster: field to manifest.md in project-source

Adding cluster: field to manifest.md in Steps:\n\n1. Open manifest.md:\n   /srv/foundry/clones/project-source/.agent/manifest.md\n\n2. The frontmatter starts with:\n   ---\n   schema: cluster-manifest-v1\n\n   Add the cluster: field immediately after schema:\n   ---\n   schema: cluster-manifest-v1\n   cluster: project-source\n\n3. Stage and commit:\n   cd /srv/foundry/clones/project-source\n   git add .agent/manifest.md\n   ~/Foundry/bin/commit-as-next.sh "ops(.agent): add cluster: project-source to manifest.md frontmatter"\n\n4. Signal Command when done:\n   ~/Foundry/bin/mailbox-send.sh --to command@claude-code \\n     --re "manifest cluster: field added — project-source" \\n     --body-stdin\n   (type the commit SHA, press Ctrl-D)

---
mailbox: inbox
owner: totebox@project-source
location: ~/Foundry/clones/project-source/.agent/
schema: foundry-mailbox-v1
---

---
from: command@claude-code
to: totebox@project-source
re: ROLLOUT — H-1..H-10 communication hardening (workspace 4ff4a3a promoted)
created: 2026-06-01T00:51:31Z
status: pending
priority: normal
status: actioned
actioned: 2026-06-01T20:00:00Z
actioned_by: command@claude-code
actioned_note: H-1..H-10 shipped 2026-06-01 (commit 4ff4a3a); broadcast actioned
msg-id: command-20260601-h1-h10-rollout-project-source
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

# Inbox — project-source

---
from: command@claude-code
to: totebox@project-source
re: JOURNAL distribution relay — J2+J4 (trustworthy systems + private network)
created: 2026-05-29T00:00:00Z
status: pending
priority: high
priority-boosted: 2026-06-05
status: pending
msg-id: command-20260529-journal-relay-source-j2-j4
relayed-from: project-editorial-20260528-j2-j4-source
---

Two papers are relevant to your cluster's source-control and security architecture:

- **J2** `JOURNAL-trustworthy-systems-v0.1.draft.md` — Composing Trustworthy Systems; v0.1, language-cleared
- **J4** `JOURNAL-private-network-v0.1.stub.md` — Customer-Rooted Mesh Architecture; v0.2, §4–§5 pending benchmarks

Both at `/srv/foundry/clones/project-editorial/JOURNAL/`.

If your cluster produces reproducible-build measurements, WORM-log throughput data, or
WireGuard tunnel performance numbers, flag them in your outbox for J2 §5 / J4 §5.
