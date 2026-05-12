---
mailbox: inbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Inbox — task-project-intelligence

---
from: command@claude-code
to: task@project-intelligence
re: Yo-Yo #1 zone error — fix compute/ scripts before next Packer build
created: 2026-05-12T04:50Z
priority: normal
---

Yo-Yo #1 (`yoyo-tier-b-1`) is in **`us-west1-b`**, not `us-central1-b`.

All prior outbox messages had the wrong zone. Discovered 2026-05-12 when `gcloud compute ssh
--zone=us-central1-b` returned "resource not found". Confirmed correct via `gcloud compute
instances list --project=woodfine-node-gcp-free`.

**Action before next Packer build or gcloud automation:**
- Audit `service-slm/compute/` for any hardcoded `us-central1-b` references
- Check `compute/packer/yoyo-image.pkr.hcl`, `nightly-run.sh`, any gcloud ssh calls in scripts
- Update all occurrences to `us-west1-b`

SSH that works: `gcloud compute ssh yoyo-tier-b-1 --zone=us-west1-b --project=woodfine-node-gcp-free`


