---
from: command@claude-code
to: totebox@project-workplace
re: Q5 — sub-clone .agent/ untracking — detailed instructions
created: 2026-06-08T16:29:42Z
priority: high
status: pending
attempts: 0
msg-id: command-20260608-q5-sub-clone-agent-untracking-detailed-i
---

SUPPLEMENTAL INSTRUCTIONS — Sub-clone .agent/ untracking (Q5)

This supplements the earlier terse relay (msg-id: command-20260608-sub-clone-agent-tracked-in-git-add-to-gi).
Execute the steps below in your first session.

Context: the pointsav-monorepo sub-clone at ./pointsav-monorepo/ has .agent/ files
tracked in its git index. This causes a foundry-fsck CRITICAL. .agent/ must be
untracked from the sub-clone git and added to .gitignore there.

EXACT COMMANDS — run in order from ~/Foundry/clones/project-workplace/

Step 1: Add .agent/ to sub-clone .gitignore
  Check if .gitignore exists:
    ls ./pointsav-monorepo/.gitignore
  If it exists, add the line (skip if already present):
    grep -q '^\.agent/$' ./pointsav-monorepo/.gitignore || echo '.agent/' >> ./pointsav-monorepo/.gitignore
  If it does NOT exist, create it:
    echo '.agent/' > ./pointsav-monorepo/.gitignore

Step 2: Untrack .agent/ from the sub-clone git index (files stay on disk)
    git -C ./pointsav-monorepo rm --cached .agent/ -r --ignore-unmatch

Step 3: Stage the .gitignore change
    git -C ./pointsav-monorepo add .gitignore

Step 4: Commit from the sub-clone directory
    (cd ./pointsav-monorepo && ~/Foundry/bin/commit-as-next.sh "ops(gitignore): untrack .agent/ from sub-clone git index (foundry-fsck CRITICAL — mailbox-scope violation)")

Step 5: Verify
    git -C ./pointsav-monorepo ls-files .agent/ 2>/dev/null | head -5
  Expected output: (empty)

Step 6: Signal Command Session when done
    ~/Foundry/bin/mailbox-send.sh \
      --to "command@claude-code" \
      --re "signal — sub-clone .agent/ untracking complete — project-workplace" \
      --priority high \
      --body-stdin
  (body: commit SHA from Step 4 + "Stage 6 pending — project-workplace sub-clone")

---
from: command@claude-code
to: totebox@project-workplace
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
owner: totebox@project-workplace
location: ~/Foundry/clones/project-workplace/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-workplace

Messages prepend (newest on top).
Archive actioned messages to `inbox-archive.md` after reading.

---
