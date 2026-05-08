---
mailbox: outbox
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: task@project-intelligence
to: master@claude-code
re: Yo-Yo #1 infrastructure complete — single remaining blocker: weights upload
created: 2026-05-08T07:30:00Z
priority: high
---

**Status:** VM running at `34.171.38.79:9443` (us-central1-b). Auth verified. Doorman
connected (`has_yoyo=true`). vllm not running — weights disk is mounted at `/data/weights`
but empty. Everything else is live and tested.

**One operator action remaining:**
```bash
# Upload weights (requires the GGUF file on the local machine)
gcloud compute scp <olmo-3-32b-think-q4.gguf> yoyo-tier-b-1:/data/weights/olmo-3-32b-think-q4.gguf \
  --zone=us-central1-b --project=woodfine-node-gcp-free

# Start vLLM (wait ~2 min for model load)
gcloud compute ssh yoyo-tier-b-1 --zone=us-central1-b --project=woodfine-node-gcp-free \
  --command="sudo systemctl start vllm.service && sudo journalctl -fu vllm.service"

# Snapshot the disk once healthy (prevents re-upload on future zone migrations)
export SLM_YOYO_GCP_PROJECT=woodfine-node-gcp-free
export SLM_YOYO_GCP_ZONE=us-central1-b
export SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-1
/srv/foundry/clones/project-intelligence/service-slm/scripts/create-yoyo-snapshot.sh
```

**What landed this session (2026-05-08):**
- `5d9fd22`: Zone-migration snapshot restore — `start-yoyo.sh` + `create-yoyo-snapshot.sh`
  (weights survive zone migrations; no re-upload needed after first snapshot)
- `47025a2`: NEXT.md updated to reflect live state; env example adds `SLM_YOYO_WEIGHTS_SNAPSHOT`

**Still pending (code-complete; operator-gated):**
- Rebuild Packer image (low urgency — current VM manually patched with tokenizer + auth map fixes)
- Re-enable apprenticeship: `SLM_APPRENTICESHIP_ENABLED=true` in local-doorman.env
- Block D2: Master ratification for `doorman-routing` + `workspace-ops` task-type promotion

## Current GCP state

| Resource | Zone | Status |
|---|---|---|
| `yoyo-tier-b-1` VM | us-central1-a | TERMINATED (L4 stockout) |
| `yoyo-tier-b-1-weights` disk | us-central1-a | READY, 100GB, EMPTY |
| `foundry-workspace` VM | us-west1-a | RUNNING |

## Commits landed this session (2026-05-07/08)

| Commit | Fix |
|---|---|
| `0c0f5a2` | vllm.service `--tokenizer`; idle monitor `Content-Length: 0`; `start-yoyo.sh` zone preference; reqwest TLS (self-signed cert) |
| `a54f101` | Idle monitor: send bearer to metrics endpoint; only fire stop when vLLM is reachable (avoids spurious stops on cold start) |

Previously landed: `5943a5c`–`3938451` (blocks A–G from prior session)

## To bring vLLM online — operator steps

**Step 1: Get L4 capacity**
```bash
export SLM_YOYO_GCP_PROJECT=woodfine-node-gcp-free
export SLM_YOYO_GCP_ZONE=us-central1-a
export SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-1
export DOORMAN_ENV_FILE=/etc/local-doorman/local-doorman.env
/srv/foundry/clones/project-intelligence/service-slm/scripts/start-yoyo.sh
```
This tries us-central1-a first, cycles through fallback zones on stockout. If zone changes, prints post-provisioning steps and updates DOORMAN_ENV.

**Step 2: Get new external IP (if zone changed)**
```bash
NEW_ZONE=<zone-from-step-1>
gcloud compute instances describe yoyo-tier-b-1 --zone=${NEW_ZONE} --project=woodfine-node-gcp-free --format='value(networkInterfaces[0].accessConfigs[0].natIP)'
```

**Step 3: Upload weights**
```bash
gcloud compute scp <path-to-olmo-3-32b-think-q4.gguf> yoyo-tier-b-1:/data/weights/olmo-3-32b-think-q4.gguf \
  --zone=${NEW_ZONE} --project=woodfine-node-gcp-free
```
Note: the weights disk may not be mounted yet if the VM is in a new zone. SSH first and mount: `sudo mount /dev/nvme0n2 /data` (GCP attaches persistent SSDs as nvme0n2).

**Step 4: Start vLLM on VM**
```bash
gcloud compute ssh yoyo-tier-b-1 --zone=${NEW_ZONE} --project=woodfine-node-gcp-free \
  --command="sudo systemctl daemon-reload && sudo systemctl start vllm.service && sudo journalctl -fu vllm.service"
```
vLLM takes ~2 minutes to load the 32B GGUF model. Watch for `Application startup complete`.

**Step 5: Update Doorman endpoint + restart**
```bash
NEW_IP=<ip-from-step-2>
sudo sed -i "s|SLM_YOYO_ENDPOINT=.*|SLM_YOYO_ENDPOINT=https://${NEW_IP}:9443|" /etc/local-doorman/local-doorman.env
sudo systemctl restart local-doorman.service
```

**Step 6: Verify**
```bash
curl -s http://127.0.0.1:9080/readyz | jq '{has_yoyo}'
# → { "has_yoyo": true }

# Wait ~30s for health probe cycle, then check health probe succeeded:
sudo journalctl -u local-doorman.service -n 10 --no-pager | grep "health probe"
# → should show no "marked unavailable" after first successful poll
```

## Block D2 (Master ratification required)

Apprenticeship task-type promotion still requires signed `task-type-add` ledger events for:
- `doorman-routing`
- `workspace-ops`

Without ratification, shadow briefs accumulate but never promote past `review`.

— task@project-intelligence


---
from: task@project-intelligence
to: master@claude-code
re: Yo-Yo #1 infrastructure complete — operator steps required + apprenticeship task-type ratification
created: 2026-05-07T00:00:00Z
priority: high
---

All auto-edit blocks (A through G) committed to main branch on the project-intelligence cluster (commits 5943a5c through 71091a9). Summary of what was done and what requires operator presence or Master action.

## Commits landed

| Commit | Block | Summary |
|---|---|---|
| 5943a5c | A | vllm.service grammar substrate, SA-east1 defaults, 4-hour stop, zone cycling |
| 4d61b1f | B+G1 | GUIDE entity class CSV + taxonomy.rs GuideRow, per-file module_id override in main.rs |
| e6d0257 | C | service-extraction parameterized paths + CORPUS bridge for DataGraph |
| 71091a9 | E+G2/G3 | DataGraph rebuild scripts + systemd timers |

## Operator-presence required (Block D + Yo-Yo #1 launch)

Before Doorman can route grammar-constrained extraction to Yo-Yo #1, operator must:

1. **GCP project setup**: create `woodfine-node-gcp-free`, link billing, request L4 GPU quota in `southamerica-east1`.
2. **Build Packer image**: `cd service-slm/compute/packer && packer build yoyo-image.pkr.hcl`
3. **Apply OpenTofu**: `cd service-slm/compute/opentofu && tofu apply` — provisions VM + instance schedule (02:00 start, 06:00 hard stop).
4. **Upload model weights**: `gcloud compute scp olmo-3-32b-think-q4.gguf yoyo-tier-b-1:/data/weights/` — confirm exact filename first.
5. **Wire env vars** — add these to `/etc/local-doorman/local-doorman.env`:

```bash
SLM_YOYO_ENDPOINT=https://<ip>:9443
SLM_YOYO_BEARER=<token>
SLM_YOYO_MODEL=Olmo-3-1125-32B-Think
SLM_YOYO_HOURLY_USD=0.84
SLM_YOYO_METRICS_KEY=vllm:num_requests_running
SLM_YOYO_GCP_PROJECT=woodfine-node-gcp-free
SLM_YOYO_GCP_ZONE=southamerica-east1-b
SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-1
SLM_YOYO_IDLE_MINUTES=30

# Route trainer + graph labels to Yo-Yo #1 temporarily (until Yo-Yo #2 is live):
SLM_YOYO_TRAINER_ENDPOINT=https://<ip>:9443
SLM_YOYO_TRAINER_BEARER=<token>
SLM_YOYO_TRAINER_MODEL=Olmo-3-1125-32B-Think
SLM_YOYO_TRAINER_HOURLY_USD=0.84

SLM_YOYO_GRAPH_ENDPOINT=https://<ip>:9443
SLM_YOYO_GRAPH_BEARER=<token>
SLM_YOYO_GRAPH_MODEL=Olmo-3-1125-32B-Think
SLM_YOYO_GRAPH_HOURLY_USD=0.84

SLM_APPRENTICESHIP_ENABLED=true
SLM_BRIEF_TIER_B_THRESHOLD_CHARS=8000
SLM_AUDIT_TENANT_CONCURRENCY_CAP=4
```

6. **Restart Doorman**: `sudo systemctl restart local-doorman.service`
7. **Verify**: `curl -s http://127.0.0.1:9080/readyz | jq '{has_yoyo,tier_b_circuit}'`

## Master ratification required (Block D2)

Apprenticeship task-type promotion requires signed `task-type-add` ledger events for:
- `doorman-routing` — shadow briefs accumulate correctly but never promote past `review`
- `workspace-ops` — same issue

Without ratification, apprenticeship produces training data but the DPO promotion pipeline stalls. This is the correct gate — ratifying task types is a deliberate Master action.

## DataGraph nightly rebuild (Block E — ready to enable after D)

Once Doorman is live with Yo-Yo #1:
1. Install corpus-rebuild.timer: `sudo systemctl enable --now corpus-rebuild.timer`
2. Install workspace-feeder.timer: `sudo systemctl enable --now local-workspace-feeder.timer`
3. Install local-extraction-jennifer.service (after jennifer path is confirmed): update `EXTRACTION_WATCH_DIR` and `EXTRACTION_EMIT_CORPUS_DIR` in the unit file, then `sudo systemctl enable --now local-extraction-jennifer.service`

— task@project-intelligence
