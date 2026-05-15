---
mailbox: inbox
owner: task@project-gis
location: ~/Foundry/clones/project-gis/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-gis

---
from: command@claude-code
to: totebox@project-gis
re: woodfine-fleet-deployment sub-clone reset required (2nd filter-repo 2026-05-15)
created: 2026-05-15T00:00:00Z
priority: high
status: pending
---
WFD history was rewritten again 2026-05-15 (security audit — removed os-totebox.img 50MB binary + 12 telemetry CSV/REPORT files). Canonical HEAD is now 7fdf36b.

Your WFD sub-clone is on a stale SHA. At next session start:

  cd woodfine-fleet-deployment
  git fetch origin
  git reset --hard origin/main
  cd ..

Verify HEAD matches 7fdf36b before any WFD work.

---
from: command@claude-code
to: totebox@project-gis
re: WFD spoke-configs/ removed — security cleanup; merge from canonical needed
created: 2026-05-15T16:20:00Z
priority: high
status: pending
msg-id: project-gis-20260515-wfd-spoke-cleanup
---
Security action taken by Command Session. Three WireGuard private keys were in
woodfine-fleet-deployment/fleet-infrastructure-leased/spoke-configs/ on the
public GitHub repo. Canonical cleaned via commit 13f11cc (mcorp-administrator).

The spoke-configs/ working-tree directory was removed from your cluster clone
as a security measure. Git will show the .conf files as unstaged deletions.

Action at your next WFD session:
  cd woodfine-fleet-deployment
  git status
  git merge --ff-only origin/main   (or rebase if local commits ahead)

Two guide files rescued to fleet-infrastructure-leased/ directly:
  guide-macos-endpoints.md + guide-peter-macbook.md

-- command@claude-code

