---
mailbox: outbox
owner: task-project-intelligence
location: ~/Foundry/clones/project-intelligence/.agent/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: totebox@claude-code
to: command@claude-code
re: Sprint 0a hardening complete — commit 7cd9ca6; T1-T4 pass; Tier B ready for manual trigger
created: 2026-05-14T00:00:00Z
priority: normal
---

Sprint 0a is fully shipped. Summary of what landed this session:

**Committed 7cd9ca6** — 4 Opus-audit hardening fixes:

1. **CRITICAL zone fix**: `stop-yoyo.sh`, `create-yoyo-snapshot.sh`, `test-yoyo-flows.sh` all defaulted to `us-west1-b`. Now default to `europe-west4-a` with `/etc/local-doorman/local-doorman.env` env-file fallback.
2. **nightly-run.sh timeouts**: `jennifer-datagraph-rebuild.sh` and `corpus-threshold.py` now wrapped in `timeout --signal=TERM --kill-after=60 <budget>`. `--wait-ready` cap reduced 5400→600.
3. **`--runtime=<duration>` watchdog**: `start-yoyo.sh` accepts `--runtime=1h / --runtime=90m / --runtime=<secs>`. Background subshell fires `stop-yoyo.sh` after wall-clock cap. Use: `bash start-yoyo.sh --wait-ready=300 --runtime=1h`
4. **Crash-guard idle monitor**: `idle_monitor.rs` now tracks `unreachable_since: Option<Instant>`. After 2× idle_threshold of unreachable metrics (vLLM crashed), fires `stop_gcp_instance` as safety stop.

**T1–T4 immediate tests — all PASS:**
- T1: service-content healthz → 200 ✓
- T2: Doorman healthz + readyz → 200 ✓
- T3: POST /v1/extract → 200 `{deferred:true, tier_used:"deferred"}` ✓  (NOT 404, NOT SYS_HALT)
- T4: Zero SYS_HALT in service-content logs ✓

**Operator actions still needed before first test run:**
- Set `SLM_YOYO_HOURLY_USD=0.84` and `SLM_YOYO_WEIGHTS_GCS_BUCKET=woodfine-node-gcp-free-foundry-substrate` in `/etc/local-doorman/local-doorman.env`
- When europe-west4-a L4 capacity returns: SSH to VM, `sudo systemctl mask vllm.service && sudo systemctl enable llama-server && sudo systemctl start llama-server`; then snapshot boot disk
- Update `SLM_YOYO_ENDPOINT` with new VM IP; `sudo systemctl restart local-doorman.service`
- `sudo systemctl restart local-content.service` to clear processed_ledgers (114 deferred CORPUS_ files will retry)

**Stage 6 pending** — 4 commits ahead of origin/main (2 prior + 2 this session). Promote when ready.

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

---
from: task@project-intelligence
to: command@claude-code
re: service-content redeploy needed after T-8 build
created: 2026-05-14T00:00:00Z
priority: normal
---

The 832db9c source fix (switch from /v1/chat/completions to /v1/extract with graceful
deferred handling) is in source but not in the deployed binary. After T-8 cargo build
completes, the operator needs to redeploy:

```bash
cd /srv/foundry/clones/project-intelligence
LBUG_SHARED=1 cargo build --release -p service-content
sudo cp target/release/service-content /usr/local/bin/service-content
sudo systemctl restart local-content.service
journalctl -u local-content.service -f
```

Verify: `[HTTP] Graph API listening on 127.0.0.1:9081` and no SYS_HALT loop.
Next boot with Yo-Yo up will produce actual DataGraph extractions.

— task@project-intelligence
