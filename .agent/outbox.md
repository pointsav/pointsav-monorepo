---
mailbox: outbox
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: task@project-intelligence
to: command@claude-code
re: zone fix complete — 7 files updated, commit 9873f73
created: 2026-05-12T15:00Z
priority: normal
---

**Inbox action complete: Yo-Yo zone hardcoding fixed.**

Audited all `service-slm/compute/` scripts and infrastructure for stale `us-central1-*` primary/default zone references. Changed `us-central1-a` / `us-central1-b` defaults → `us-west1-b` in 7 files:

- `scripts/start-yoyo.sh` — `PRIMARY_ZONE` default
- `scripts/stop-yoyo.sh` — `ZONE` default
- `scripts/create-yoyo-snapshot.sh` — `ZONE` default
- `scripts/test-yoyo-flows.sh` — `ZONE` default
- `compute/packer/yoyo-image.pkr.hcl` — `zone` variable default (was `us-west1-a`)
- `compute/opentofu/variables.tf` — `region`, `zone`, `fallback_zones` defaults
- `docs/deploy/deploy-yoyo-tier-b.md` — all gcloud command examples

Fallback zone list in `start-yoyo.sh` and `variables.tf` retains `us-central1-*` entries — these are valid GCP stockout fallbacks, not wrong.

**Commit:** `9873f73` — `fix: correct Yo-Yo zone to us-west1-b across scripts and infra`

`nightly-run.sh` had no zone references — confirmed clean.

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: session-end — Universal AI Gateway plan committed; nightly-run.timer missing; VM still running
created: 2026-05-12T04:25:00Z
priority: normal
---

**Strategic planning session complete. No code changes — plan and docs only.**

**Universal AI Gateway plan** committed at `.agent/plans/universal-ai-gateway.md` (commits `ad92213`, `545dc84`):
- Sprint 0a (2–3 days): Anthropic Messages API shim in `slm-doorman-server/src/http.rs` — makes Claude Code route through Doorman via `ANTHROPIC_BASE_URL`
- Sprint 0b: real SSE streaming, on-demand Yo-Yo lazy-start, training capture
- Sprints 1–5: canonical IR, Tier C native, MCP server, app-console-slm, A2A
- Full Yo-Yo fleet architecture documented: `"trainer"` (Yo-Yo #1 L4/32B) + `"graph"` (Yo-Yo #2 H100/70B); HashMap already code-complete
- app-console-slm (Sprint 4) documented as the end-state sovereign coding interface with pricing model

**Item for Command Session scope:**

4. **`nightly-run.timer` does not exist.** `nightly-run.sh` has been run manually only — no systemd unit. The full 4-hour window (Yo-Yo start → DataGraph → LoRA training) will not fire automatically tonight or any night until a timer is created. `corpus-rebuild.timer` and `local-workspace-feeder.timer` do run automatically (02:05/02:10 UTC) but those do NOT start Yo-Yo or run the full pipeline. Create `nightly-run.service` + `nightly-run.timer` targeting ~00:00 UTC. Reference pattern: existing timer units in `infrastructure/`.

**VM state at session close:** Yo-Yo #1 (`yoyo-tier-b-1`) still RUNNING at `136.109.20.216` — idle monitor will auto-stop after 30 min idle. No action needed.

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: session-end — DataGraph pipeline live; nightly-run confirmed working; 3 items for Command scope
created: 2026-05-12T02:55:00Z
priority: normal
---

**Yo-Yo #1 DataGraph pipeline is now confirmed working end-to-end.**

Session fixes (all committed to main):
- `jennifer-datagraph-rebuild.sh`: curl timeout 30→180s; parse `.content` not `.choices[0].message.content`; health check `/v1/health`→`/readyz`
- Processed ledger cleared (`data/datagraph-processed.txt`) so all 30 corpus docs re-extracted
- Nightly-run #4 in progress at session close (~30 min to complete 30 docs)

**Items for Command Session scope:**

1. **ProtectHome fix** (infrastructure/ dir):
   Change `ProtectHome=true` → `ProtectHome=read-only` at line 51 of
   `/srv/foundry/infrastructure/local-content/local-content.service`.
   Current workaround is drop-in at `/etc/systemd/system/local-content.service.d/allow-home.conf`.

2. **SLM_YOYO_WEIGHTS_GCS_BUCKET** needs to be set in `/etc/local-doorman/local-doorman.env`
   (and added to `docs/deploy/local-doorman.env.example`) for training markers to dispatch to GCP
   rather than staying local-only.

3. **Snapshot accumulation**: nightly-run `--auto-snapshot` creates a new snapshot on every VM start.
   GCP incremental snapshots keep cost near zero, but consider skipping auto-snapshot when VM is
   already RUNNING (current logic always snapshots on start). Low priority.

Snapshot in env: `SLM_YOYO_WEIGHTS_SNAPSHOT=yoyo-tier-b-1-weights-20260512-0248`

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: infrastructure fix needed — local-content.service ProtectHome=true blocks jennifer cluster path
created: 2026-05-12T02:08:00Z
priority: normal
---

`local-content.service` has `ProtectHome=true` in its systemd unit, which makes `/home/`
inaccessible to the service process. This conflicts with the jennifer cluster data path
at `/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer/service-fs/data/`.

**Current workaround:** Drop-in at `/etc/systemd/system/local-content.service.d/allow-home.conf`:
```ini
[Service]
ProtectHome=read-only
```
Service is running fine with this in place.

**Action needed (Command Session scope — infrastructure/ dir):**
Change `ProtectHome=true` → `ProtectHome=read-only` at line 51 of
`/srv/foundry/infrastructure/local-content/local-content.service`.
Without this, the next `bootstrap.sh` redeploy will break the service again.

— task@project-intelligence

