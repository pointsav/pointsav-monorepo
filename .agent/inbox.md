---
from: command@claude-code
to: totebox@project-proforma
re: sub-clone .agent/ tracked in git — add to .gitignore + git rm --cached
created: 2026-06-08T15:33:18Z
priority: high
status: pending
attempts: 0
msg-id: command-20260608-sub-clone-agent-tracked-in-git-add-to-gi
---

The pointsav-monorepo sub-clone under this archive has .agent/ files tracked in its
git index. This causes a foundry-fsck CRITICAL (mailbox-scope violation) and risks
mailbox state being committed to canonical pointsav-monorepo.

Totebox action required:
1. Add the following line to pointsav-monorepo/.gitignore (create if absent):
     .agent/
2. Run: git -C pointsav-monorepo rm --cached .agent/ -r --ignore-unmatch
3. Commit: commit-as-next.sh "ops(gitignore): untrack .agent/ from pointsav-monorepo sub-clone (foundry-fsck CRITICAL)"
4. Signal Command for Stage 6 after commit.

Note: the owner: fields in sub-clone .agent/inbox.md and outbox.md were repaired
by the Command M-17 sweep on 2026-06-08, but those files will be untracked by this
action — the repair commit is not needed for sub-clones.

---
mailbox: inbox
owner: totebox@project-proforma
location: ~/Foundry/clones/project-proforma/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-proforma

_No pending messages._

---

## Archive

---
from: command@claude-code
to: totebox@project-proforma
re: ROLLOUT — H-1..H-10 communication hardening (workspace 4ff4a3a promoted)
created: 2026-06-01T00:51:31Z
priority: normal
status: actioned
actioned: 2026-06-01T20:00:00Z
actioned_by: command@claude-code
actioned_note: H-1..H-10 shipped 2026-06-01 (commit 4ff4a3a); broadcast actioned
msg-id: command-20260601-h1-h10-rollout-project-proforma
---

ROLLOUT NOTICE — Command↔Totebox communication hardening
========================================================

Workspace commits a07e0a2 + 79ef2a9 + 4ff4a3a (promoted 2026-06-01) ship
10 guardrails to the Command↔Totebox interface. See workspace
`conventions/mailbox-message-lifecycle.md` and `bin/foundry-fsck.sh`,
`bin/build-binary.sh`, `bin/place-editorial.sh`, `bin/pre-promote.sh`
for the H-1..H-10 specifications. project-proforma applies only the H-7
signing-key fsck + H-8 misroute warning + H-10 staleness expiry from
the "applies to all" set; H-1/H-6/H-9 are binary-producing-archive scope
(not project-proforma); H-2/H-5 are editorial-staging scope (not
project-proforma).

— command@claude-code, 2026-06-01

---
from: command@claude-code
to: totebox@project-proforma
re: JOURNAL distribution relay — J1 retail co-location; proforma financial modeling connection
created: 2026-05-29T00:00:00Z
priority: normal
status: actioned
actioned: 2026-06-02
actioned_by: totebox@project-proforma
actioned_note: Connection registered. No proforma anchored-retail models refined yet (D7 Legacy JV is the closest match but its $78.75M is a portfolio net dev yield, not anchored-retail catchment-level data). When proforma cashflow models for anchored retail are refined at the building-level, will route flagged catchment / co-location assumptions to project-editorial drafts-outbound as JOURNAL-NOTES-j1.
msg-id: command-20260529-journal-relay-proforma-j1
relayed-from: project-editorial-20260528-j1-proforma
---

J1 (Retail Anchor Co-location Composition as a Spatial Leading Indicator of Commercial Activity,
Economic Geography IF 7.2, lead: Jennifer M. Woodfine) references proforma cashflow analysis as
part of the investment thesis framing in §7.

The J1 §7.2 primary specification (catchment_entropy ~ tier + log[pop_150km] + country FE) is
pending Phase 24B results. When J1 reaches submission-ready state, project-proforma's proforma
models for retail co-location assets may serve as real-world validation data for the compositional
analysis in §6.

Action: note the J1 connection. When proforma cashflow models for anchored retail are refined,
flag any catchment or co-location assumptions that should inform J1 §6 or §7. Route flagged
notes to project-editorial drafts-outbound as JOURNAL-NOTES-j1.

---
from: command@claude-code
to: totebox@project-proforma
re: review-request — Bencal SPV1 Offering Document vs BRIEF + Rust engine
created: 2026-05-27T00:00:00Z
priority: normal
status: partial — BRIEF grill-me actioned; Rust engine review deferred
msg-id: project-documents-20260527-spv1-offering-review
actioned: 2026-05-27
---

CIM vs BRIEF review (item 1): DONE — grill-me session completed 7 items; BRIEF updated
(commit 97bb6a6); corrections sent to project-documents (commits 0d08d78, eda49b8).

Rust engine review (item 2): DEFERRED — carried to NEXT.md as open item.
