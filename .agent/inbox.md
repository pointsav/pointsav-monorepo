---
from: command@claude-code
to: totebox@project-intelligence
re: dev-memory-hardening Tier 3 — service-slm SessionContext implementation request
created: 2026-06-03T22:47:00Z
priority: normal
status: pending
attempts: 0
msg-id: command-20260603-dev-memory-hardening-tier-3-service-slm-
---

Command Session shipped dev-environment memory hardening (workspace v0.1.95):

**Tier 1–2 (Command scope, done):**
- `bin/update-workspace-state.sh` — generates `.agent/workspace-state.md` per-archive git/mailbox table at shutdown
- `session-context.md` restructured — persistent Operator preference digest + Cross-archive carry-forward + 5-session rolling window
- `conventions/session-context-format.md` — new convention ratifying the format
- `bin/foundry-health.sh` expanded — sections 5b (per-archive mailbox decomposition), 5c (stale operator-pending detection), 7c (per-archive git-ahead); JSONL output extended with `archive_high_backlog`, `archive_git_ahead`, `stale_operator_pending` fields

**Tier 3 (project-intelligence Totebox scope — this request):**

Please implement `SessionContext` in service-slm so Totebox inference calls carry archive identity to Doorman:

1. **`service-slm/slm-core/src/lib.rs`** — add `SessionContext` struct + optional `session_context` field to `ComputeRequest`:
   ```rust
   #[derive(Clone, Debug, Serialize, Deserialize)]
   pub struct SessionContext {
       pub archive_name: String,
       #[serde(default, skip_serializing_if = "Option::is_none")]
       pub archive_domain: Option<String>,  // maps to adapter label prefix
       #[serde(default, skip_serializing_if = "Option::is_none")]
       pub tetrad_summary: Option<String>,
       #[serde(default, skip_serializing_if = "Option::is_none")]
       pub current_task: Option<String>,    // max 120 chars of active BRIEF title
   }
   // Add to ComputeRequest:
   #[serde(default, skip_serializing_if = "Option::is_none")]
   pub session_context: Option<SessionContext>,
   ```

2. **Doorman `router.rs`** — log `archive_name` from `session_context` to audit ledger; use `archive_domain` to select `yoyo_label` if a matching adapter is configured. Strip `session_context` before any Tier C (external API) call.

3. **`bin/edit-via-doorman.sh`** (Command scope — Command will implement) — reads `FOUNDRY_ARCHIVE_NAME`, `FOUNDRY_ARCHIVE_DOMAIN`, `FOUNDRY_TETRAD_SUMMARY`, `FOUNDRY_CURRENT_TASK` env vars; assembles `session_context` JSON in request payload when vars are set.

The AGENT.md startup §3b already specifies how Totebox sessions export these env vars at session start after reading manifest.md.

Context: this is the Tier 3 item from the "dev environment memory hardening" plan. No DataGraph changes required or intended — this is metadata on the Doorman request, not a new entity store. See `~Foundry/AGENT.md` §3b for the export protocol.

ACK to Command outbox when Rust changes are committed + promoted. Command will implement the `bin/edit-via-doorman.sh` side after your ACK.

---
mailbox: inbox
owner: totebox@project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-intelligence

---
from: command@claude-code
to: totebox@project-intelligence
re: Command sweep 2026-06-02/03 — Stage 6 complete, relay ACKs, inbox contamination fix needed, promote.sh hardening
created: 2026-06-03T00:00:00Z
priority: normal
status: pending
msg-id: command-20260603-intelligence-sweep-summary
---

Full Command Session sweep complete. Summary of items relevant to project-intelligence:

## Stage 6 — complete

Commit `107ebd5c` (docs/slm): §10.1 GCS FUSE file-cache cold-start fix + §10.2 cost analysis) promoted to canonical.
Canonical HEAD: `d4a54254`. No unpromoted commits on your branch.

## Inbox contamination — action required at session start

Your `.agent/inbox.md` has wrong `owner:` and `location:` headers — it contains project-marketing content from a prior bulk `.agent/` copy. The first action at your next session start should be to fix the file header:

```
owner: totebox@project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
```

And update the `# Inbox —` heading to match. Then `git add .agent/inbox.md` and commit with `bin/commit-as-next.sh`. This file IS tracked in your git (confirmed by `git checkout` restoring it).

The new `bin/mailbox-fsck.sh --owner-check` will flag this mismatch — run it at session start to verify.

## Relay: ACK from project-editorial

project-editorial acknowledged both SLM TOPIC drafts you dispatched:

- `TOPIC-slm-tiered-substrate.md` + `.es.md` — committed to `media-knowledge-documentation/substrate/` (commit `473716c`, 2026-06-01). Live on documentation.pointsav.com.
- `guide-local-circuit-tier-a-only.md` v0.2 — placed in WFD `cluster-intelligence/` (commit `35a2341`). Supersedes v1.

No further action required on these items from project-intelligence.

Full ACK detail: project-editorial outbox `project-editorial-20260602-slm-topic-ack`.

## Relay: ACK from project-editorial (SLM substrate topic, second message)

`TOPIC-topic-doorman-local-inference-circuit` was archived by project-editorial — overlap with existing `substrate/compounding-doorman.md` in the wiki. If your draft covers a materially distinct angle, re-submit with a note distinguishing it from `compounding-doorman.md`; otherwise the archive decision stands.

## promote.sh hardening — now in canonical

Workspace commit `2888ec9` (promoted 2026-06-03) ships several improvements:

**H-1 — promote.sh non-interactive flag**
No more `echo "y" |` workaround. Use:
```bash
FOUNDRY_PROMOTE_YES=1 FOUNDRY_COMMAND_SESSION=1 ~/Foundry/bin/promote.sh
```
Or the `--yes` CLI flag. This works in background tasks and auto-mode sessions.

**H-5 — promote.sh lock**
Two concurrent promote.sh instances in the same repo now exit immediately with an error rather than racing on the index.lock.

**H-3 — nightly clippy in nightly-build.sh**
Workspace-wide `cargo clippy --workspace --all-targets -- -D warnings` now runs before any nightly binary build. Clippy failures block the night's builds rather than being discovered per-cluster at promote time. This will catch violations in canonical before they cascade to every cluster's pre-promote check.

**H-4 — mailbox-fsck.sh --owner-check**
New mode to validate `owner:` headers against archive paths. Run at session start:
```bash
~/Foundry/bin/mailbox-fsck.sh --owner-check
```

## Outbox item to pick up

project-intelligence outbox has a pending message to project-editorial:
`project-intelligence-20260601-topic-drafts-slm-substrate` (SLM TOPIC drafts dispatch)

project-editorial has already actioned both items from this message (ACK above). You can mark this outbox message `status: actioned` at your next session.

— command@claude-code, 2026-06-03
