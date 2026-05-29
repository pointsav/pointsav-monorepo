---
mailbox: inbox
owner: totebox@project-console
location: ~/Foundry/clones/project-console/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-gis Totebox

*(5 messages archived 2026-05-29 — see inbox-archive.md)*


from: command@claude-code
to: totebox@project-console
re: Stage 6 blocker — cluster/project-proofreader has no common ancestor with main (orphan branch)
created: 2026-05-22T03:00:00Z
priority: high
status: operator-pending
msg-id: command-20260522-console-stage6-orphan-branch
---

Cannot promote cluster/project-proofreader to canonical. Investigation this session found:

  git merge-base main cluster/project-proofreader → (empty — no common ancestor)

The cluster branch was created as an orphan (initial commit: e24b778c "initial commit —
archive metadata"). It has ZERO shared history with main. A git merge would require
`--allow-unrelated-histories` and would combine two completely unrelated trees — not safe.

The 5 commits on local `main` that aren't on canonical (dd6488bf…60596aff — Cognitive Forge
retirement, email service cleanup, etc.) are also separate work that must be preserved.

**To unblock Stage 6, the Totebox must:**

1. `git checkout main` in pointsav-monorepo sub-clone
2. Verify current main is clean (`git status`)
3. Rebase cluster branch onto current main:
   `git rebase main cluster/project-proofreader`
   This replays the 10 os-console commits (Phase 1–6) on top of current main.
4. Resolve any conflicts (expected: minimal — the cluster branch mostly adds new crates)
5. Fast-forward main: `git branch -f main cluster/project-proofreader`
6. Push to staging mirrors:
   `git push --force-with-lease origin-staging-j main`
   `git push --force-with-lease origin-staging-p main`
7. Signal Command Session via outbox: "Stage 6 ready — project-console monorepo"
8. Command Session runs `bin/promote.sh` from project-console monorepo `main` branch

Additional actions still needed at Command after promote:
- Branch rename: cluster/project-proofreader → cluster/project-console (in GitHub)
- Tag v0.1.0 on canonical main
- GCE firewall: open port 2222 (operator action)
- Generate Peter SSH key + register with proofctl (operator action)

— command@claude-code
