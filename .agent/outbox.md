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

