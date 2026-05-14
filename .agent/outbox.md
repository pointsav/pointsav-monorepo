---
mailbox: outbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: task@project-intelligence
to: task@project-editorial
re: 11 drafts ready for language pass in drafts-outbound
created: 2026-05-14T00:00:00Z
priority: normal
---

11 drafts are staged at `.agent/drafts-outbound/` in the project-intelligence cluster with
status `draft-pending-language-pass`. Please pick up, refine, and route to the appropriate
wiki or fleet-deployment destination per the artifact routing table in `.agent/plans/README.md`.

| Draft | Type | Language |
|---|---|---|
| `guide-yo-yo-nightly-pipeline.md` | GUIDE | EN |
| `topic-apprenticeship-substrate.md` | TOPIC | EN |
| `topic-apprenticeship-substrate.es.md` | TOPIC | ES |
| `topic-doorman-protocol.md` | TOPIC | EN |
| `topic-doorman-protocol.es.md` | TOPIC | ES |
| `topic-jennifer-datagraph-rebuild.md` | TOPIC | EN |
| `topic-jennifer-datagraph-rebuild.es.md` | TOPIC | ES |
| `topic-yo-yo-lora-training-pipeline.md` | TOPIC | EN |
| `topic-yo-yo-lora-training-pipeline.es.md` | TOPIC | ES |
| `topic-zero-container-inference.md` | TOPIC | EN |
| `topic-zero-container-inference.es.md` | TOPIC | ES |

All drafts carry `foundry-draft-v1` frontmatter. The 5 newer drafts (jennifer-datagraph and
yo-yo-lora-training-pipeline EN+ES, plus guide-yo-yo-nightly-pipeline) have full research
trails; the 6 skeleton drafts (apprenticeship, doorman, zero-container-inference EN+ES) need
content infill before language pass.

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: operator actions required — Yo-Yo VM fix + local-doorman.env + Packer rebuild
created: 2026-05-14T00:00:00Z
priority: high
---

Three items from the inbox require operator SSH or sudo — excluded from this auto run.

**Item 7 — BLOCKING for next Yo-Yo restart: mask vllm.service on yoyo-tier-b-1**

When us-west1-b L4 capacity returns and the VM can start:
```bash
gcloud compute ssh yoyo-tier-b-1 --zone=us-west1-b --project=woodfine-node-gcp-free
sudo systemctl mask vllm.service
sudo systemctl enable llama-server.service
sudo systemctl start llama-server.service
```
Then snapshot the boot disk to lock in the fix.

**Item 8 — Set SLM_YOYO_WEIGHTS_GCS_BUCKET in local-doorman.env (requires sudo):**
```bash
sudo tee -a /etc/local-doorman/local-doorman.env <<'ENVEOF'
SLM_YOYO_WEIGHTS_GCS_BUCKET=woodfine-node-gcp-free-foundry-substrate
ENVEOF
sudo systemctl restart local-doorman.service
```
(Doc line added to `docs/deploy/local-doorman.env.example` this session — T-7.)

**Item 9 — Packer image rebuild + OLMo 3 32B weights upload (after item 7):**
```bash
cd /srv/foundry/clones/project-intelligence/service-slm/compute/packer
packer build yoyo-image.pkr.hcl
```
Then upload OLMo 3 32B-Think Q4 weights to the Yo-Yo VM (~20 GB) from GCS bucket.

— task@project-intelligence
