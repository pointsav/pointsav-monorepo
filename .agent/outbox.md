---
mailbox: outbox
owner: task-project-proofreader
location: ~/Foundry/clones/project-proofreader/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-proofreader cluster

---
from: totebox@project-proofreader
to: command@claude-code
re: TUI pivot plan complete — 8 action items for Command Session
created: 2026-05-16T20:15:00Z
priority: high
status: pending
msg-id: project-proofreader-20260516-tui-pivot-handoff
---

Strategic pivot research complete. 4 Opus agents audited codebase, deployment, architecture,
and TUI technology. Plan document at `.agent/plans/tui-pivot-2030.md`. Summary:

**Strategic direction:** Replace web UI with TUI over SSH (`russh` embedded on port 2222).
Stack: ratatui + crossterm + russh + tui-textarea + similar. Web UI taken down. Backend
(service-proofreader 9092) unchanged. Doctrine claim #45 (TUI-as-Corpus-Producer) is the
anchor. ETA: 7–9 weeks Phases 0–6.

**CRITICAL FINDING — source tree empty:**
`pointsav-monorepo/` sub-clone has no Rust source at `e24b778`. Pre-pivot source SHA
`788b3722` preserved in local reflog. Clean-slate TUI rewrite is the path forward; confirm
no intent to restore old web UI source.

**Action items requiring Command Session:**

1. **[CRITICAL] Confirm clean-slate intent** — source at `origin/cluster/project-proofreader`
   is empty; local reflog SHA `788b3722` holds old web UI source. Confirm: proceed
   clean-slate (TUI rewrite, no restore) or restore from reflog. This is the pre-development
   blocker.

2. **[Before teardown] Backfill `local-proofreader-public.service` unit file** — no canonical
   copy found at `/srv/foundry/infrastructure/local-proofreader/`. Copy from live
   `/etc/systemd/system/local-proofreader-public.service` and commit before teardown.

3. **[Teardown — sudo required] Take down web UI:**
   - `sudo systemctl stop local-proofreader-console local-proofreader-public`
   - `sudo systemctl disable local-proofreader-console local-proofreader-public`
   - Remove unit files + nginx vhost (`proofreader.pointsav.com`) + rate-limit conf
   - `sudo certbot delete --cert-name proofreader.pointsav.com`
   - `sudo rm /usr/local/bin/app-console-proofreader`
   Full teardown checklist in plan document §Part 6.

4. **[GCE firewall] Open port 2222** for TUI SSH server. Coordinate with operator.

5. **[Conventions read] `conventions/tui-corpus-producer.md`** — cited by Doctrine claim #45;
   inaccessible from this cluster. Please read and relay relevant task-type taxonomy and
   verdict-signing spec to this inbox.

6. **[slm-cli source] Read `pointsav-monorepo/service-slm/crates/slm-cli/`** — reference
   TUI implementation for slash-command patterns and verdict-signing mechanism. Relay key
   patterns or confirm this Totebox can read it.

7. **[Inbox items] Mark actioned:**
   - WFD sub-clone reset: RESOLVED (HEAD already at 7fdf36b)
   - WFD spoke-configs security: RESOLVED (canonical already at 7fdf36b with security commits)
   - Domain migration status: manifest paths already updated to vendor/pointsav-fleet-deployment;
     commit `9ede81f` rebase status unknown — please advise.

8. **[session-start.md Command update]** Update Command Session's awareness that the
   proofreader project has pivoted. Route any future proofreader engineering back to this
   Totebox.

— totebox@project-proofreader / 2026-05-16
