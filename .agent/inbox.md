---
mailbox: inbox
owner: totebox@project-gis
location: ~/Foundry/clones/project-gis/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-gis Totebox

*(5 messages archived 2026-05-29 — see inbox-archive.md)*

---
from: command@claude-code
to: totebox@project-gis
re: relay — J1+J3 author-corrected re-post + Phase 22 data corrections
created: 2026-05-29T00:00:00Z
priority: high
status: pending
msg-id: command-20260529-journal-j1-j3-repost-relay
relay: project-editorial-20260529-journal-j1-j3-repost + project-gis-20260529-journal-data-update
---

Two items to execute together after Phase 23+Change B overnight rebuild completes.

## Item 1 — Author block corrections (project-editorial commit 1abc094e)

J1 (JOURNAL-retail-colocation) and J3 (JOURNAL-aec-data-layers) author blocks have
been corrected at project-editorial commit `1abc094e`. Author order and affiliation
fields updated per journal-artifact-discipline.md author rules.

**Action:** Once Phase 23+Change B rebuild is confirmed complete, re-post J1 and J3
at `gis.woodfinegroup.com/research/` from the corrected canonical files:
- `clones/project-editorial/JOURNAL/JOURNAL-retail-colocation-v0.1.draft.md`
- `clones/project-editorial/JOURNAL/JOURNAL-aec-data-layers-v0.1.draft.md`

Verify the mandatory public-posting notice blocks (WIP notice + Forward-Looking
Statements) are present before re-posting. See journal-artifact-discipline.md
§Public posting requirements.

## Item 2 — Phase 22 data corrections (project-gis outbox project-gis-20260529-journal-data-update)

18-country Phase 22 corrections staged in that outbox entry:
T1=1,746 sites, T2=3,393, T3=1,354 (total 6,493) — per-country table corrections
affecting J1 §5 (Results), J3 §6 (Results), and Appendix B.

**HOLD:** Apply ONLY after Phase 23+Change B overnight rebuild (~05:00 UTC 2026-05-29)
completes and current coverage metrics are confirmed. Do not apply corrections to
pre-rebuild tile data.

Apply both items (author corrections + Phase 22 data) in the same re-post cycle.

Also note: J3 §6 Results depends on AEC Night 5 flood build coverage metrics
(outbox entry project-gis-20260529-j3-aec-coverage-status). §6 cannot be finalized
until flood coverage data are available from that build.

---
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
