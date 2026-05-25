---
name: feedback-destructive-ops-escalate
description: When a destructive operation has unexpected scope, escalate to Command rather than proceeding — confirmed by operator "route to Command first"
metadata:
  type: feedback
---

When a force-push or other destructive operation turns out to have a larger blast radius
than the inbox/plan described, stop and route to Command Session for explicit authorisation.
Do not infer intent and execute.

**Why:** Operator said "route to Command first" when force-push to staging-j would have
silently overwritten 763 commits (including project-proforma Stage 6 work) due to a
history divergence that was not anticipated in the inbox message.

**How to apply:** Before any `git push --force*`, `git reset --hard`, or multi-repo
destructive action: verify the actual blast radius matches the description. If it is
larger or different, write the discrepancy to outbox and wait for explicit go-ahead.
