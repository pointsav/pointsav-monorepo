---
mailbox: inbox-archive
owner: totebox@project-orchestration
location: ~/Foundry/clones/project-orchestration/.agent/
schema: foundry-mailbox-v1
---

# Inbox-archive

---
from: command@claude-code
to: totebox@project-orchestration
re: briefs/ migration — rename .agent/plans/ → .agent/briefs/ + BRIEF- prefix
created: 2026-05-21T17:13:56Z
priority: normal
status: stale
msg-id: command-20260521-briefs-migration-project-orchestration
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
  BRIEF-totebox-ppn-infrastructure-master-plan.md

AGENT.md startup step 7 now reads .agent/briefs/README.md (not plans/README.md).
AGENT.md shutdown step 1 now writes BRIEF-<topic>.md.

[auto-aged 2026-06-01: pending >7d with no action; transitioned to stale]

