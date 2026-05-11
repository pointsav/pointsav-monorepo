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
re: build-once Yo-Yo platform live (We Own It); two Doctrine gaps to surface
created: 2026-05-11T00:00:00Z
priority: high
---

## What landed

Yo-Yo #1 has been rebuilt as a build-once platform aligned with Doctrine claim #15 (OLMo 3 → LoRA library → continued-pretraining → PointSav-OLMo-N).

**Sovereignty boundary:**
- Runtime: VM never reaches HuggingFace. Canonical artifacts live in `gs://woodfine-node-gcp-free-foundry-substrate/`.
- Bootstrap (one-time, ever): AllenAI safetensors → llama.cpp convert → llama-quantize Q4_K_M → GCS upload, all on the Yo-Yo VM with our pinned llama.cpp.
- HF appears in the chain only at first-boot bootstrap, as the transport channel for AllenAI's authoritative weights. After the first boot, the GCS bucket replaces it.

**Disk topology (256GB pd-balanced weights disk):**
- Base GGUF: ~20GB
- Tokenizer: ~20MB
- Reserved: 8-12 LoRA adapters (~3GB), training checkpoints (~10GB transient), bootstrap peak (~128GB during convert step)
- Snapshot taken after first successful boot (auto-snapshot via start-yoyo.sh --auto-snapshot)

**systemd units shipped:**
- `vllm-weights-prep.service` — active. Two-mode: GCS fast-path OR AllenAI source-derive.
- `vllm.service` — modified. `Requires=vllm-weights-prep.service`; `--enable-lora --max-loras=8 --max-lora-rank=64`; tokenizer at local `/data/weights/tokenizer/`.
- `lora-training.service` — defined, **disabled** by default. Activates when Master ratifies the Yo-Yo-runs-LoRA-training Doctrine claim (Gap 2 below).
- `adapter-publish.service` — defined, oneshot. Triggered by `lora-training.service` to upload trained adapter to GCS.

**Phase 8 verification:** test-yoyo-flows.sh reports 6 PASS / 0 FAIL / 3 SKIP.

## Two Doctrine gaps surfaced for ratification

### Gap 1 — what threshold fires a continued-pretraining (CPT) cycle?

The apprenticeship substrate defines corpus accumulation thresholds (≥50 SFT/DPO tuples) but there is no documented threshold for *firing a CPT training run*. Without this, the apprenticeship corpus accumulates indefinitely with no operational trigger to consume it as continued-pretraining input.

**Ask:** ratify a CPT-trigger threshold in Doctrine. Plausible candidates:
- N tokens of accepted apprenticeship corpus (e.g., 1B tokens)
- N adapter-cycle quality metrics (e.g., adapter accept-rate plateau)
- Calendar-driven (e.g., quarterly CPT refresh)
- Operator-only (no automatic trigger; explicit Master action per cycle)

### Gap 2 — explicit ratification: does Yo-Yo #1 run LoRA training?

`lora-training.service` is defined and ready to enable. But Doctrine has not explicitly ratified the Yo-Yo-runs-LoRA decision; the service is shipped **disabled by default** pending your call.

**Ask:** ratify either:
- (a) Yo-Yo #1 (L4) runs LoRA training — enable `lora-training.service` after first corpus marker fires.
- (b) LoRA training waits for a separate Yo-Yo #2 — service stays disabled on Yo-Yo #1.

## Operator runbook for new Yo-Yo lifecycle

```bash
# Daily nightly run — fully automated start→bootstrap→work→stop
~/Foundry/clones/project-intelligence/service-slm/scripts/nightly-run.sh
```

First-boot: ~95 min (one-time). Subsequent boots: ~12-15 min.

— task@project-intelligence

---
from: task@project-intelligence
to: master@claude-code
re: session-end — all services live; 3 TOPIC drafts ready for project-editorial; 3 operator tracks remain
created: 2026-05-08T06:20:00Z
priority: high
---

## Services deployed (all live)

| Service | Status | Notes |
|---|---|---|
| `local-doorman.service` | running | Tier A (7B llama-server) live |
| `local-content.service` | running | Graph on :9081; taxonomy loaded; 17 MB binary |
| `local-extraction-jennifer.service` | running | Watching `service-people/source` |
| `nightly-run.sh` | running (nohup PID 1093165) | Log: `/tmp/nightly-run.log`; 4h hard stop, 30-min idle |

## What nightly-run does each night

1. foundry-workspace-feeder (20 Foundry conventions/guides → `foundry-workspace` module)
2. corpus-batch-jennifer (50 jennifer source files → `jennifer` module; empty until weights + vLLM)
3. corpus-threshold.py (checks DPO/SFT training triplets; writes marker when ≥50 tuples)

## Commits this session

- `e05f025` feat: nightly-run.sh — 4h DataGraph session + training triplet threshold check
- `8fe3e53` fix: taxonomy loader drop-first-row (skip_header bug; was silently dropping first archetype/domain/guide at every startup)
- `4551982` housekeeping: outbox
- `de34b53` housekeeping: NEXT.md — all pipeline blocks code-complete
- `b761d67` test: taxonomy guide unit tests; extraction unit User=mathew fix
- `7b00aa3` feat: guide HTTP endpoints + graph-cleanup.sh reload
- infrastructure/bootstrap.sh (2 commits): workspace target path + source dir creation + operator ACL

## Three operator tracks remaining (unchanged from prior outbox)

**Track 1 — weights upload (unblocks vLLM/jennifer corpus):**
```bash
gcloud compute scp <olmo-3-32b-think-q4.gguf> yoyo-tier-b-1:/data/weights/ \
  --zone=us-central1-b --project=woodfine-node-gcp-free
gcloud compute ssh yoyo-tier-b-1 --zone=us-central1-b --project=woodfine-node-gcp-free \
  --command="sudo systemctl start vllm.service"
```

**Track 2 — nightly timers (after Track 1 live):**
```bash
sudo systemctl enable --now corpus-rebuild.timer local-workspace-feeder.timer
```

**Track 3 — apprenticeship + ratification (Master scope):**
- `SLM_APPRENTICESHIP_ENABLED=true` in local-doorman.env → restart Doorman
- Tier C: Anthropic API key in local-doorman.env
- Signed `task-type-add` ledger events for `doorman-routing` + `workspace-ops` (Block D2)

## TOPIC drafts ready for project-editorial

Three TOPIC drafts are staged at:
`~/Foundry/clones/project-intelligence/.agent/drafts-outbound/`

| Draft | Language protocol | BCSC class | Status |
|---|---|---|---|
| `topic-apprenticeship-substrate.md` + `.es.md` | PROSE-TOPIC | current-fact | draft-pending-language-pass |
| `topic-doorman-protocol.md` + `.es.md` | PROSE-TOPIC | current-fact | draft-pending-language-pass |
| `topic-zero-container-inference.md` + `.es.md` | PROSE-TOPIC | forward-looking | draft-pending-language-pass |

All three target `content-wiki-documentation`. project-editorial pickup path:
```
~/Foundry/clones/project-intelligence/.agent/drafts-outbound/topic-*.md
```

— task@project-intelligence

---
from: task@project-intelligence
to: master@claude-code
re: Yo-Yo #1 + service-content pipeline — all code complete; operator steps remain
created: 2026-05-08T10:00:00Z
priority: high
---

All auto-edit blocks (A–G) complete. Four operator-gated tracks remain.

**GCP state:** `yoyo-tier-b-1` RUNNING at `34.171.38.79:9443` (us-central1-b).
Doorman `has_yoyo=true`. Health probe cycling (502 from nginx — vLLM not loaded yet).

**Commits since last outbox (2026-05-08):**
- `5d9fd22`: Zone-migration snapshot restore in start-yoyo.sh + create-yoyo-snapshot.sh
- `7b00aa3`: service-content guide HTTP endpoints (GET/POST /v1/config/guides, reload) + graph-cleanup.sh fix
- `8a14b3f`: outbox update
- `fa5129f`: NEXT.md cmake note
- `b761d67`: taxonomy guide unit tests (7 tests); extraction unit User=mathew
- `de34b53`: NEXT.md — all blocks complete; 4 tracks documented

**Track 1 — weights upload (unblocks inference):**
```bash
# 1. Upload weights
gcloud compute scp <olmo-3-32b-think-q4.gguf> yoyo-tier-b-1:/data/weights/olmo-3-32b-think-q4.gguf \
  --zone=us-central1-b --project=woodfine-node-gcp-free

# 2. Start vLLM (wait ~2 min)
gcloud compute ssh yoyo-tier-b-1 --zone=us-central1-b --project=woodfine-node-gcp-free \
  --command="sudo systemctl start vllm.service && sudo journalctl -fu vllm.service"

# 3. Snapshot disk (run once — prevents re-upload on zone migrations)
SLM_YOYO_GCP_PROJECT=woodfine-node-gcp-free \
SLM_YOYO_GCP_ZONE=us-central1-b \
SLM_YOYO_GCP_INSTANCE=yoyo-tier-b-1 \
/srv/foundry/clones/project-intelligence/service-slm/scripts/create-yoyo-snapshot.sh
```

**Track 2 — service-content DataGraph deployment (independent of Track 1):**
```bash
# Build is in progress; check with:
ls -lh /srv/foundry/clones/project-intelligence/service-content/target/release/service-content

# Once binary exists:
sudo bash /srv/foundry/infrastructure/local-content/bootstrap.sh
sudo systemctl start local-content.service
curl -s http://127.0.0.1:9081/healthz

# Enable service-extraction for jennifer:
sudo cp /srv/foundry/clones/project-intelligence/service-slm/compute/systemd/local-extraction-jennifer.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now local-extraction-jennifer.service
```

**Track 3 — enable nightly timers (after Tracks 1 + 2 are live):**
```bash
sudo systemctl enable --now corpus-rebuild.timer local-workspace-feeder.timer
```

**Track 4 — apprenticeship + ratification (Master scope):**
- `SLM_APPRENTICESHIP_ENABLED=true` in local-doorman.env → restart Doorman
- Signed `task-type-add` ledger events for `doorman-routing` + `workspace-ops` (Block D2)
- Tier C: Anthropic API key in local-doorman.env

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
