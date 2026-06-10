---
from: totebox@project-proforma
to: totebox@project-command
re: ops: add cluster: field to manifest.md frontmatter
created: 2026-06-08T16:59:09Z
priority: normal
status: pending
attempts: 0
msg-id: project-proforma-20260608-ops-add-cluster-field-to-manifest-md-fro
---

Adding cluster: field to manifest.md in project-command

Adding cluster: field to manifest.md in Steps:\n\n1. Open manifest.md:\n   /srv/foundry/clones/project-command/.agent/manifest.md\n\n2. The frontmatter starts with:\n   ---\n   schema: cluster-manifest-v1\n\n   Add the cluster: field immediately after schema:\n   ---\n   schema: cluster-manifest-v1\n   cluster: project-command\n\n3. Stage and commit:\n   cd /srv/foundry/clones/project-command\n   git add .agent/manifest.md\n   ~/Foundry/bin/commit-as-next.sh "ops(.agent): add cluster: project-command to manifest.md frontmatter"\n\n4. Signal Command when done:\n   ~/Foundry/bin/mailbox-send.sh --to command@claude-code \\n     --re "manifest cluster: field added — project-command" \\n     --body-stdin\n   (type the commit SHA, press Ctrl-D)

---
mailbox: inbox
owner: totebox@project-command
location: ~/Foundry/clones/project-command/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-command
