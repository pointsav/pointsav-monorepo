---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-intelligence
target_repo: woodfine-fleet-deployment
target_path: cluster-intelligence/
target_filename: guide-yo-yo-nightly-pipeline.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-GUIDE
authored: 2026-05-11
authored_by: command@claude-code (session 2026-05-11)
authored_with: claude-sonnet-4-6
references:
  - service-slm/scripts/nightly-run.sh
  - service-slm/scripts/jennifer-datagraph-rebuild.sh
  - service-slm/scripts/test-yoyo-flows.sh
  - service-slm/compute/packer/yoyo-image.pkr.hcl
notes_for_editor: |
  This is an operator-facing guide. Paths and commands should be literal and
  accurate. No editorial polish needed on command syntax. Prose sections can
  be tightened.
  The Packer image rebuild and lora-training.service activation steps are
  intended operator actions — use "intended" language there. All other
  described behaviour is current-fact (code complete, 177/177 tests pass).
  No ES pair needed for guides.

  UPDATE NOTE (2026-05-12): The following corrections apply to this draft
  before language pass — verify these are reflected in the body text:
  1. jennifer-datagraph-rebuild.sh curl timeout is 180s (not 30s as originally
     scripted — 32B Think model needs 30–120s per inference).
  2. Doorman response envelope: `.content` field, not `.choices[0].message.content`.
     Scripts parsing the raw OpenAI wire format will silently return zero entities.
  3. Doorman health check endpoint is `/readyz` (GET /v1/health returns 404).
  4. Processed ledger (data/datagraph-processed.txt): entries written on "no entities"
     as well as on success — clear the ledger if a parsing bug corrupted a run.
---

# GUIDE — Yo-Yo #1 Nightly Pipeline

## Prerequisites

Two services must be running on the workspace VM before the nightly pipeline
can complete successfully:

- **local-doorman.service** — Doorman endpoint on `127.0.0.1:9080`. Verify
  with: `curl -sf http://127.0.0.1:9080/v1/health`
- **local-content.service** — service-content endpoint on `127.0.0.1:9081`.
  Verify with: `curl -sf http://127.0.0.1:9081/healthz`

If either service is not responding, the DataGraph rebuild will abort at the
health check stage. Do not start the pipeline until both return HTTP 200.

The pipeline also requires `jq` and `python3` on the workspace VM PATH. Both
are present in the standard workspace image.

## Running the Pipeline

All commands run from the `service-slm/` directory of the pointsav-monorepo
cluster.

**Normal mode** — boots Yo-Yo #1, runs the full 4-hour pipeline (2h DataGraph
+ 2h Training):

```
./scripts/nightly-run.sh
```

**Workspace-only mode** (`--no-yoyo`) — skips the Yo-Yo #1 VM lifecycle and
runs against local Tier A (OLMo 3 7B Q4 on llama-server). Entity extraction
uses the smaller model. Useful for testing the DataGraph phase without incurring
Yo-Yo #1 VM costs:

```
./scripts/nightly-run.sh --no-yoyo
```

**Test mode** (`--test-mode`) — reduces phase budgets to 60 seconds each
(60s DataGraph + 60s Training). Processes only the first few documents before
the time budget is exhausted. Use this to verify the pipeline wiring without
running a full cycle:

```
./scripts/nightly-run.sh --test-mode
```

Phase budgets can be overridden independently via environment variables:

```
DATAGRAPH_SECONDS=300 TRAINING_SECONDS=120 ./scripts/nightly-run.sh
```

## Interpreting DataGraph Results

After Phase 1 completes, the pipeline writes a health summary to:

```
$FOUNDRY_ROOT/data/datagraph-health.json
```

Default path: `/srv/foundry/data/datagraph-health.json`

The file contains four fields:

| Field | Meaning |
|---|---|
| `timestamp` | UTC ISO 8601 timestamp of the health probe |
| `entity_count` | Total entity count reported by service-content at end of run |
| `delta` | Change in entity count since the previous run |
| `new_entities_this_run` | Entities written during this specific run |

**HEALTHY** — `delta` is zero or positive. Entity count held steady or grew.
This is the expected outcome on most nightly runs after the initial population.

**WARN** — `delta` is negative. The entity count shrank. This typically means
service-content restarted and lost in-memory graph state between the prior run
and the current health probe. Inspect service-content logs and verify LadybugDB
persistence. The DataGraph rebuilds automatically on the next nightly run.

To read the health file:

```
jq . /srv/foundry/data/datagraph-health.json
```

## Interpreting Training Results

`corpus-threshold.py` writes marker files to:

```
$FOUNDRY_ROOT/data/training-pending/
```

Default path: `/srv/foundry/data/training-pending/`

Each marker is a JSON file named `<adapter>-<YYYY-MM-DD>.json`. The file
progresses through three states:

| Filename suffix | State |
|---|---|
| `.json` (no suffix) | Dispatched — waiting for Yo-Yo #1 pickup |
| `.json.claimed` | In progress — `lora-training.sh` has claimed the marker and training is running |
| `.json.claimed.completed` | Finished — adapter written and adapter-publish triggered |

To list all pending markers:

```
ls /srv/foundry/data/training-pending/
```

To inspect a marker:

```
jq . /srv/foundry/data/training-pending/<marker-name>.json
```

If no marker files are present after a Phase 2 run, neither corpus bucket
reached its 50-tuple threshold. The run is not an error — the pipeline logged
"No adapters at threshold."

## Packer Image Rebuild (Intended Operator Action)

The QLoRA training stack (peft, bitsandbytes, trl, accelerate) and the OLMo 3
7B Think safetensor weights are intended to be baked into the Yo-Yo #1 VM
image via Packer. Until the image is rebuilt, `lora-training.sh` falls back
to a pip install on first run, which adds several minutes to Phase 2 startup
and is not suitable for a reliable nightly schedule.

To rebuild the image:

```
cd service-slm/compute/packer
packer build yoyo-image.pkr.hcl
```

This requires Packer installed on the workspace VM and appropriate GCP
credentials. The build creates a new GCE machine image. After the build
completes, update the `yoyo_machine_image` variable in the OpenTofu
infrastructure configuration and run `tofu apply` to roll the new image
to Yo-Yo #1.

## Enabling Training After Image Rebuild

Once the image is rebuilt and deployed to Yo-Yo #1, enable the training
service on the Yo-Yo VM:

```
sudo systemctl enable --now lora-training.service
```

This activates the training watcher loop. From that point forward, any
`.json` marker in the pending directory will be claimed and processed
automatically. The adapter output will be written to
`/data/weights/adapters/<tenant>/<role>/v<N>/` on the Yo-Yo VM and uploaded
to GCS by `adapter-publish.service`.

## Test Matrix

Run the Yo-Yo flow tests to verify the DataGraph and training paths without
running a full nightly cycle:

```
./scripts/test-yoyo-flows.sh
```

Key tests for the nightly pipeline:

| Test | What it verifies |
|---|---|
| Test 10 — DataGraph REST API | Full round trip: Doorman extraction → service-content graph/mutate → health check |
| Test 11 — Training marker claim | Corpus threshold check → marker write → `.claimed` rename progression |

All other tests in the suite verify Doorman routing, Tier A fallback, and
apprenticeship substrate wiring. A passing Test 10 and Test 11 indicates
the nightly pipeline is correctly wired end-to-end. The full test suite
runs in under two minutes against local services only — no Yo-Yo #1 VM
required.
