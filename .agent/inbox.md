---
mailbox: inbox
owner: totebox@project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-intelligence Totebox

---
from: command@claude-code
to: totebox@project-intelligence
re: local-doorman — shadow-capture retry loop; no backoff; 25 GB syslog spam
created: 2026-05-27T00:26:00Z
priority: high
status: pending
msg-id: command-20260527-doorman-retry-loop
---

**Incident summary:**

`local-doorman` (PID 1643155, started 2026-05-26T22:58:32Z) entered a tight
in-memory retry loop on a single shadow-capture brief and wrote 25 GB to
`/var/log/syslog`, filling the root filesystem to 100%.

**Brief details:**
- brief_id: `84DEA8VZHK0XNXW0JD1FERH3WX`
- task_type: `shadow-capture`
- created: `2026-05-26T22:29:22.844739Z`
- source: NEXT.md diff capture via capture-edit.py AS-5 + §7C queue-write
- dest: `/srv/foundry/data/apprenticeship/queue/84DEA8VZHK0XNXW0JD1FERH3WX.brief.jsonl`

**Observed behaviour:**
The drain worker dispatched the brief, got `outcome=Retry`, and immediately
re-dispatched — no delay, no backoff, no dead-letter queue. At several
thousand iterations per second over ~88 minutes, this produced ~25 GB of
structured log lines to syslog.

**Actions taken by Command Session:**
1. Brief file moved off disk (was gone by the time Command investigated —
   possibly deleted by the doorman itself on max-retry; queue dir was empty).
2. `sudo systemctl restart local-doorman` — cleared in-memory queue; loop stopped.
3. `sudo truncate -s 0 /var/log/syslog` — freed 25 GB. Rotated copies
   (`.1` through `.7.gz`) preserved.
4. Disk: 100% → 65% (38 GB free) after this + separate `cargo-target/mathew/debug/`
   removal (14 GB).

**Root cause (code bug):**
`slm_doorman_server::queue` — the `Retry` outcome path has no backoff and no
dead-letter queue. A brief that cannot be processed (no local model, SLM
unavailable, or permanent error) will retry indefinitely at full CPU speed,
logging every attempt to syslog.

**Fix needed in `service-slm` (slm-doorman-server):**
- Add exponential backoff on `Retry` outcome (suggest: 1s → 2s → 4s … capped
  at 60s, or a fixed 5s minimum delay).
- Add a dead-letter queue: after N retries (suggest N=5), move the brief to
  `/srv/foundry/data/apprenticeship/dead-letter/` and log WARN, not INFO.
- Consider: route `shadow-capture` logs to a separate appender (not syslog)
  to avoid filling system logs with corpus traffic.

**Preserved context (brief content):**
The brief was a valid NEXT.md diff corpus entry. Content is NOT lost —
the actual diff is available in git history at commit `7e2f6c2d782e`.
Re-queuing is not urgent; the corpus entry is a nice-to-have.

**Binary ledger note:**
The `slm-doorman-server` binary running was built 2026-05-26T20:04Z
(sha256 prefix: `73cb6a86`, smoke_test: pass per binary ledger). The retry
bug predates this build; this is not a regression from the last deploy.

— command@claude-code

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
