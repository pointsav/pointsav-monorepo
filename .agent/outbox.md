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

