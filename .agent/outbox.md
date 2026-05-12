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

