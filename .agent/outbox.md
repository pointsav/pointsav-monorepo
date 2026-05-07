---
mailbox: outbox
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: task-project-intelligence
to: master
re: D4 image pipeline complete — 6 commits promoted to canonical at 0140176
created: 2026-05-07T12:00Z
---

D4 pipeline code is committed and promoted. All 177 tests pass.

**6 commits (5e4dc3f → 0140176):**

1. `fix(idle-monitor)` — `SLM_YOYO_METRICS_KEY` env var added to `IdleMonitorConfig`.
   Default: `llama_active_slots_total` (backward compat). Set to
   `vllm:num_requests_running` for Yo-Yo #1 vLLM.
2. `feat(compute)` — Packer template at `service-slm/compute/packer/yoyo-image.pkr.hcl`
   + `scripts/{provision.sh,vllm.service,nginx-yoyo.conf}`.
   Produces `slm-yoyo` image family in `pointsav-public`.
3. `feat(compute)` — OpenTofu at `service-slm/compute/opentofu/{main,variables,outputs}.tf`.
   Declares: nightly Instance Schedule (02:00 UTC), 100 GB SSD weights disk,
   g2-standard-4 Spot VM + L4, firewall (workspace-only port 9443), IAM for idle monitor stop.
4. `feat(scripts)` — `service-slm/scripts/start-yoyo.sh` + `stop-yoyo.sh`.
5. `docs(deploy)` — `service-slm/docs/deploy/deploy-yoyo-tier-b.md` (8-step runbook)
   + `local-doorman.env.example` updated with 5 new Tier B env vars.
6. `fix(idle-monitor)` — mutex added to idle monitor tests to prevent env-var race.

**Remaining operator-gated work (all in service-slm/NEXT.md):**
- Create `pointsav-public` GCP project + enable Compute Engine API + billing
- Request L4 GPU quota in `us-west1`
- Run `packer build` to produce the `slm-yoyo` image
- Run `tofu apply` to provision VM + disk + Instance Schedule
- Upload OLMo 3 32B-Think Q4 weights to the data disk
- Wire Doorman env vars in `/etc/local-doorman/local-doorman.env`
- Restart Doorman + verify `has_yoyo: true` in `/readyz`
- Smoke test nightly drain + idle-shutdown
- Re-enable apprenticeship (`SLM_APPRENTICESHIP_ENABLED=true`)
- Tier C auth (Anthropic API key)
- cmake + build-essential on workspace VM

---
from: task-project-intelligence
to: master
re: Monorepo housekeeping complete — 14 commits promoted to canonical
created: 2026-05-07T00:00Z
---

All auto-completable tasks from the session task list are now complete and
promoted to GitHub (canonical `pointsav/pointsav-monorepo`, commit `5e4dc3f`).

**Completed this session (13 commits + 1 merge commit):**

1. Inbox housekeeping — inbox.md + inbox-archive.md committed
2. `.gitignore` dedup — 3 duplicate ASP quarantine blocks removed
3. `service-market` registered as Reserved-folder (Doctrine claim #52)
4. `service-exchange` registered as Reserved-folder (Doctrine claim #52)
5. `app-orchestration-market` registered as Reserved-folder (Doctrine claim #52)
6. `app-orchestration-exchange` registered as Reserved-folder (Doctrine claim #52)
7. `app-console-market` registered as Reserved-folder (Doctrine claim #52)
8. `app-console-exchange` registered as Reserved-folder (Doctrine claim #52)
9. `app-orchestration-gis` registry drift closed — Reserved-folder row + directory created
10. `service-extraction/CLAUDE.md` created — Active-state conformance
11. `app-workplace-memo` activated — CLAUDE.md + NEXT.md added; Scaffold-coded → Active
12. `app-workplace-proforma/CLAUDE.md` committed — local-only header removed
13. Monorepo `NEXT.md` updated — closed items moved to "Recently closed (2026-05-07)"

**Remaining operator-gated work (all in service-slm/NEXT.md):**
- Tier C auth (Anthropic API key)
- cmake + build-essential on workspace VM
- D4 image pipeline + GCP project creation
- Yo-Yo VM deployments (#1, #2)
- Apprenticeship re-enable

No blocking items for Master. Registry now at 107 rows, Reserved-folder count 43, Active count 6.
