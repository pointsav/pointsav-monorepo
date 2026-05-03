---
schema: foundry-doc-v1
document_version: 0.1.0
research_done_count: 14
research_suggested_count: 9
open_questions_count: 13
research_provenance: direct-consultation
research_inline: true
cites:
  - federated-lora-2502-05087
  - olmo3-allenai
  - lorax-predibase
  - s-lora-2024
---

# Trainer Scoping — cluster-side input to the trainer substrate conversation

**Date:** 2026-04-29
**Author:** project-slm Task (Sonnet sub-agent, iteration 20)
**Status:** Research and scoping input for operator + Master decision. No
implementation decisions are made here.

The apprenticeship corpus began flowing at 2026-04-29T00:22:25Z when
B7 landed with `apprenticeship_enabled=true`. At corpus-stats time,
86 engineering tuples and 14 apprenticeship tuples were present. The
number will grow monotonically from this point. Without a trainer that
consumes the corpus and produces adapter weights, the corpus accumulates
forever and never closes the feedback loop. This document scopes the
trainer substrate question comprehensively so the operator and Master
can make informed decisions in one pass.

---

## §1 — Why we need a trainer

The Doorman is now a corpus producer. Every code-shaped commit across
all 8 active clusters (project-slm, project-data, project-orgcharts,
project-language, project-proofreader, project-system, project-knowledge,
project-bim) fires a shadow brief through `POST /v1/shadow`. The
Doorman asks the local OLMo 3 7B model what it would have done, and
records the (brief, attempt, actual-diff) triple in
`data/training-corpus/apprenticeship/<task-type>/`. The prose-edit
pipeline is also producing tuples: 14 apprenticeship entries on day
zero, split across `prose-edit` (13) and `design-edit` (1).

Per [conventions/apprenticeship-substrate.md] §1, "every signed
verdict is a training tuple; every graduated task-type eliminates
Claude tokens monotonically." The elimination does not happen by
accumulation alone. It requires a training cycle that:

1. Reads the accumulated tuples from the corpus.
2. Formats them into the appropriate training format (SFT, DPO, or
   continued pretraining depending on the target).
3. Runs a training loop that adjusts OLMo 3's weights — or adds
   a LoRA adapter on top of the frozen base.
4. Validates the resulting adapter against a held-out set.
5. Stores the adapter at `~/Foundry/data/adapters/` (declared in
   [conventions/trajectory-substrate.md] §4 as the target library,
   but the directory does not exist yet).
6. Loads the adapter in the Doorman at inference time per
   [conventions/adapter-composition.md] §1.

**None of steps 1–6 are wired today.** The corpus is accumulating;
the adapter library directory does not exist; the Doorman does not
load adapters; and there is no trainer codebase in the workspace that
does this work. This document scopes what it would take.

### The compounding thesis

The strategic case for closing this loop is in
[conventions/llm-substrate-decision.md] "The continued-pretraining
path (Year 2+)": Year 1 is "OLMo 3 7B local + 32B Think Yo-Yo; build
LoRA library on Foundry corpus." We are now in Year 1. The corpus is
the raw material; the trainer is the missing process.

Without the trainer, the substrate has one arm (accumulation) and
cannot fire the other (improvement). The loop stays open.

---

## §2 — What the trainer must do (functional requirements)

### Inputs

| Input | Description | Current state |
|---|---|---|
| Engineering corpus | Edit-tuples from `data/training-corpus/engineering/<cluster>/` | 87 files in `project-slm/` alone; 11 active cluster subdirectories |
| Apprenticeship corpus | (brief, attempt, actual-diff) triples from `data/training-corpus/apprenticeship/<task-type>/<tenant>/` | 14 files; `prose-edit` dominant; shadow-only (no verdicts yet) |
| Feedback corpus | (rejected, corrected, doctrine-violation-tag) DPO triples from `data/training-corpus/feedback/` | Empty today; populated when verdicts with `refine`/`reject` start accumulating |
| Doctrine corpus | Doctrine clause × role × scope tuples from `data/training-corpus/doctrine/` | Directory present; not yet populated at scale |
| Adapter version manifest | Which doctrine version to train against; which tenant scope | Embedded in each corpus record as `doctrine_version` |

### Outputs

Per [conventions/adapter-composition.md] §1, the Doorman at request
time composes:

```
base_model[OLMo-3-1125-7B-Q4]
  ⊕ constitutional_adapter[doctrine_vM.m.p]
  ⊕ engineering_adapter[pointsav_vN]?
  ⊕ tenant_adapter[<tenant>_vK]?
  ⊕ role_adapter[master | root | task]
  ⊕ cluster_adapter[<cluster-name>_vJ]?
```

This implies the trainer must produce (at minimum) the following
adapter classes, in order of value and corpus readiness:

| Adapter | Corpus source | Training method | Readiness today |
|---|---|---|---|
| `engineering-pointsav-vN.lora` | Engineering corpus (`data/training-corpus/engineering/`) | Continued pretraining or SFT | 87 engineering tuples in project-slm alone; moderate volume |
| `cluster-project-slm-vJ.lora` | Engineering corpus (`data/training-corpus/engineering/project-slm/`) | SFT on cluster-specific edit patterns | Same 87 tuples; sub-corpus of the above |
| `apprenticeship-pointsav-vN.lora` | Apprenticeship corpus (`data/training-corpus/apprenticeship/`) | DPO on (attempt, actual-diff) pairs | 14 tuples; shadow-only (no scored verdicts yet) |
| `constitutional-doctrine-v0.0.12.lora` | Doctrine corpus | Supervised on clause × role × scope | Pending doctrine-corpus population |
| `tenant-woodfine-vK.lora` | Per-tenant corpus inside Customer Totebox | Per-tenant SFT or DPO | Must stay inside Woodfine Totebox per doctrine §IV.b isolation |

The cluster manifest at `.agent/manifest.md` declares this cluster
trains: `cluster-project-slm`, `engineering-pointsav`,
`apprenticeship-pointsav`. Those are the three adapters this cluster's
work is intended to produce.

### Invariants

- **Signed verdicts only in scored training data.** Shadow tuples
  (no verdict) are valid for SFT; DPO training requires a signed
  verdict (`refine`/`reject`) to produce the preferred/rejected pair.
  Per [conventions/apprenticeship-substrate.md] §5, verdicts are
  SSH-signed with `ssh-keygen -Y sign -n apprenticeship-verdict-v1`.
- **Doctrine-version pinning.** Per [conventions/adapter-composition.md]
  §3, each adapter carries a `doctrine_version` field; the Doorman
  loads only adapters whose `doctrine_version` matches the deployment.
  Doctrine MINOR bumps trigger a retrain.
- **BCSC posture preserved.** Sanitize-outbound discipline
  (`conventions/bcsc-disclosure-posture.md`) is applied at corpus
  capture time, not at training time. Training input is already
  redacted. Forward-looking claims in prose-edit corpus carry
  `bcsc_class: forward-looking` and must not flow into adapter
  weights that are used for public editorial work without a
  conformance review.
- **Tenant isolation.** Per [conventions/trajectory-substrate.md]
  §1, tenant-private corpus records stay inside the Customer Totebox.
  The Vendor workspace never trains on tenant-private data unless
  the tenant has explicitly opted into the federated marketplace
  (Doctrine claim #14; not yet implemented).
- **Adapter signatures.** Per [conventions/adapter-composition.md]
  §3, every adapter is SSH-signed by the trainer's identity and the
  signature is verifiable against `identity/allowed_signers`.
  Unsigned adapters are not composed.

### Integration points

- **Doorman adapter loading** — `service-slm` today has no adapter
  loading code. `service-slm/memory/adapters/` is described in
  `ARCHITECTURE.md` Ring 3b as the intended home, but the directory
  does not exist.
- **vLLM Multi-LoRA on Tier B** — per [conventions/adapter-composition.md]
  §2, LoRAX/S-LoRA/vLLM Multi-LoRA all serve thousands of concurrent
  adapters with hot-swap per request. Tier B (vLLM ≥ 0.12 on the
  Yo-Yo GCE node, when D4 ships) is the natural inference-time
  composition surface.
- **Verdict feedback loop** — `POST /v1/verdict` is wired in the
  Doorman (AS-3, per ARCHITECTURE.md §11). The verdict path
  writes the tuple and checks promotion thresholds. The trainer
  must consume the verdict-signed tuples separately.

---

## §3 — Existing trainer artefacts in the workspace

This section is an honest inventory. Nothing has been invented.

### `service-slm/router-trainer/` in `vendor/pointsav-monorepo`

The path
`/srv/foundry/vendor/pointsav-monorepo/service-slm/router-trainer/`
exists and contains the following:

| File / Directory | Contents |
|---|---|
| `README.md` | Describes "TOOL-COGNITIVE-FORGE" — a knowledge distillation script for training a 0.5B routing model on email-classification tasks |
| `engine/llamafile` | A 35 MB llamafile binary |
| `engine/weights/qwen2.5-coder-1.5b.gguf` | 1.07 GB Qwen2.5 Coder 1.5B GGUF weights |
| `engine/engine.log` | Log from a prior training run |
| `scripts/distill_knowledge.py` | A Python script that reads `transient-queues/*.txt` email skeletons, submits them to a local model API, and appends `{instruction, output}` training pairs to a JSONL file |
| `scripts/ignite_teacher.sh` | Shell script that boots the llamafile model, runs the Python distillation script, then kills the engine |
| `datasets/training_dataset.jsonl` | 15 instruction/output training pairs from a prior email-classification run |
| `src/` | Empty directory |

**Assessment of this artefact:** This is a pre-framework knowledge
distillation script aimed at a specific old task (email skeleton →
routing JSON using Qwen2.5 Coder 1.5B on an iMac). It uses a
different model family, a different corpus format (`{instruction,
output}` not the trajectory-substrate JSONL schema), a different task
type (email routing not code editing), and was designed for a machine
that no longer applies (iMac reference in README). It is not directly
reusable as-is for the current corpus.

It does establish one important precedent: the workspace has
successfully run a local distillation loop before, using llamafile as
the serve engine and a Python script for the generation pass. That
pattern is reproducible.

The path `service-slm/router-trainer/` also has a partial clone in
the `project-data` cluster at
`/srv/foundry/clones/project-data/service-slm/router-trainer/` with
only the `README.md` and `datasets/` and `scripts/` directories
(no engine or weights).

### `service-slm/memory/adapters/` — declared but absent

`ARCHITECTURE.md` §2 Ring 3b describes `service-slm/memory/adapters/`
as the long-term skill store. The directory does not exist in the
cluster. It is a forward declaration in the architecture document.

### `~/Foundry/data/adapters/` — declared but absent

[conventions/trajectory-substrate.md] §4 states the adapter library
lives at `~/Foundry/data/adapters/`. The directory does not exist.
No adapters have been trained.

### What is genuinely absent

There is no:
- Modern SFT or DPO training script for OLMo 3
- Hugging Face TRL / PEFT / Axolotl configuration file
- Conversion pipeline from the trajectory-substrate JSONL schema to a
  trainer-expected format
- Validation harness
- Adapter storage and signing pipeline
- Doorman code that loads adapters at inference time

The gap is real and the work required is non-trivial but well-bounded.

---

## §4 — Deployment options (where the trainer runs)

### Option A — Workspace VM CPU (e2-standard-4)

**Capabilities:** The workspace VM (`foundry-workspace`) is the
e2-standard-4 instance that currently runs `local-slm.service` (OLMo
3 7B Q4) at ~44 seconds per inference. CPU-only LoRA training on a
7B model with a Q4 quantisation is feasible via `llama.cpp`'s
training mode or HuggingFace's PEFT with CPU offloading, but is slow.
Rough estimate: one gradient step every 5–20 minutes on CPU-only.

**Cost:** Sunk in VM cost (~$50-60/month for e2-standard-4). No
incremental spend.

**Constraints:** Training on CPU occupies the machine. `local-slm.service`
and `local-doorman.service` would compete for RAM and CPU during a
training run. A training run of even 100 steps could take 8-24 hours.
CPU training of a 7B model (even with LoRA restricting trainable
parameters to ~0.1-1% of weights) requires holding the full model in
RAM for forward/backward passes unless gradient checkpointing is used.

**Time-to-first-adapter:** 1-3 days of operator setup + 8-24 hours of
training time per cycle on small corpora.

**Sovereignty posture:** Fully sovereign; no data leaves the VM.

**BCSC implications:** Training is internal-operational; no disclosure
required.

**What blocks it today:** No training script; no corpus-to-trainer
format conversion; no adapter storage; no Doorman adapter loading.
These are all cluster-scope implementation items (not Master-tier).

**Verdict:** Viable for Phase 0 proof-of-life. Not viable for regular
production training cycles.

### Option B — Yo-Yo GPU (GCE A100 80GB, D4-gated)

**Capabilities:** The `infrastructure/slm-yoyo/tofu/` OpenTofu module
provisions a preemptible GCE instance with an A100 80GB GPU. This is
the intended compute substrate for heavy workloads including LoRA
training. A LoRA fine-tuning run on OLMo 3 7B on an A100 at a small
corpus scale (~1,000 tuples) would complete in minutes to a few hours.

**Cost:** ~$0.50-0.70/hr preemptible (per PS.1 readiness review).
A training run of 2-4 hours costs ~$1-3. Monthly training cadence
(one run per month) is ~$2-6/month.

**Constraints:** Gated on D4 (the image-build pipeline, a Master-tier
action). `tofu apply` fails immediately because the
`data "google_compute_image" "yoyo"` lookup on `pointsav-public`
returns "resource not found" — the GCP project has never been created.
No estimated D4 timeline is available to this cluster.

**Time-to-first-adapter:** D4 lead time (unknown) + 1 day of trainer
setup + a few hours of training.

**Sovereignty posture:** Data stays on PointSav-controlled GCE
infrastructure. Preemptible instances do not provide data-at-rest
guarantees across preemptions; training data should be loaded at boot
and not persisted to disk in cleartext.

**BCSC implications:** Same as Option A; internal-operational.

**What blocks it today:** D4 (Master-tier). Every Yo-Yo path is gated
on D4 per the PS.1 readiness review.

**Verdict:** The right production training substrate once D4 ships.
Cannot be targeted for Phase 0 or Phase 1 until D4 is resolved.

### Option C — Customer GPU (hardware procurement)

**Capabilities:** An operator-owned consumer or prosumer GPU (e.g.,
RTX 4090 24GB VRAM, ~$1,700 CAD; or an H100 PCIe 80GB, ~$30,000 CAD)
installed on-premises or in a rented colo.

**Cost:** RTX 4090 can train OLMo 3 7B LoRA adapters faster than
Option A but slower than A100. H100 is comparable to Option B.

**Constraints:** Capital expense. Procurement, provisioning, and
connectivity to the workspace VM. The Two-Bottoms Sovereign Substrate
(Doctrine claim #34) envisions customers eventually running training
on their own metal — this would be the first instance of that pattern.

**Time-to-first-adapter:** 2-8 weeks (procurement + setup) + hours
of training.

**Sovereignty posture:** Highest — weights and data never leave
operator-controlled hardware.

**What blocks it today:** Procurement decision and budget approval.

**Verdict:** Appropriate for later phases (Year 2 continued
pretraining) or when a specific Customer (Woodfine) wants to run
per-tenant adapter training on their own hardware. Not appropriate
for Phase 0 or Phase 1.

### Option D — Hyperscaler fine-tuning API (Modal / RunPod / OpenAI)

**Capabilities:** Modal and RunPod both offer GPU-backed training
environments with Python runtimes and storage. OpenAI fine-tuning API
accepts JSONL directly. These services remove infrastructure burden
from the operator.

**Cost:** Modal: ~$0.30/GPU-hour on H100; a small fine-tuning run
costs ~$3-15. OpenAI fine-tuning: ~$8/M tokens. RunPod: ~$0.35-0.50/hr.

**Constraints:** The sovereignty framing of [conventions/llm-substrate-decision.md]
and the BCSC posture are the primary tension here. Training corpus data
would leave the PointSav GCP perimeter and be processed by a third
party. The operator should make this call explicitly. OpenAI fine-tuning
is incompatible with the OLMo 3 model (OpenAI only trains OpenAI models).
Modal and RunPod can run arbitrary Python, so OLMo 3 LoRA training via
HuggingFace TRL/PEFT is feasible.

**Time-to-first-adapter:** 1-2 days of setup + hours of training.

**Sovereignty posture:** Reduced. Corpus data transits to a third-party
compute provider. Fine-tuned weights (if stored there) carry residual
third-party custody risk.

**What blocks it today:** Operator decision on sovereignty trade-off.

**Verdict:** Operationally simplest; sovereignty trade-off requires
explicit operator call. Not the long-term substrate choice given the
zero-container-runtime and sovereignty conventions, but could be
acceptable for Phase 0 proof-of-life if the operator is comfortable
with the posture.

### Option E — Federated training (multi-Customer)

**Capabilities:** Doctrine claim #14 (federated marketplace) and
[conventions/apprenticeship-substrate.md] §9 envision a future where
Customer Totebox instances contribute per-tenant adapters to a
commons. Federated LoRA merging (weighted averaging of adapters,
e.g., via FedAvg or the federated-lora approach in [federated-lora-2502-05087])
would combine per-tenant adapters without sharing raw training data.

**Constraints:** No current Customer deployments other than the
Woodfine dogfood instance. The Woodfine Totebox does not yet exist
as a deployed instance. Federated training requires at least two
participants to be meaningful.

**Time-to-first-adapter:** Year 2+ (per [conventions/llm-substrate-decision.md]).

**What blocks it today:** No Customer Totebox instances; no federated
marketplace infrastructure; Doctrine claim #14 is a long-horizon item.

**Verdict:** Long-term architecture goal; not relevant to near-term phases.

---

## §5 — Trigger options (when the trainer runs)

### A — Continuous (after every N new tuples)

Train after every N tuples accumulated in a given task-type bucket.
N might be 50 for the first adapter cycle, then 100+.

**Latency-to-fresh-adapter:** Lowest; new corpus signal incorporated
as it accumulates.
**Operator burden:** Low once automated; requires a trigger service
(e.g., a watch process or cron with a tuple-count check).
**Cost predictability:** Variable; depends on tuple accumulation rate.
**Risk:** Training on very small increments (N=1, N=5) produces noisy
adapters. Minimum viable batch likely 20-50 tuples per task-type.

### B — Threshold-triggered (when K tuples per task-type)

Train when the corpus for a specific task-type reaches K total tuples.
K = 50 is the promotion threshold from `review` → `spot-check` per
[conventions/apprenticeship-substrate.md] §2. Natural alignment:
run the trainer when the corpus is large enough to justify the
promotion gate check.

**Latency-to-fresh-adapter:** Moderate; depends on how fast each
task-type accumulates tuples.
**Operator burden:** Low; event-driven.
**Cost predictability:** Moderate; each task-type triggers one run.
**Risk:** Task-types with low volume (e.g., `template-author`,
`schema-validate`) may never reach the threshold without a minimum
corpus injection.

**This option aligns cleanest with the promotion ledger logic.**
The trainer cycle and the promotion gate check could share a trigger.

### C — Time-triggered (nightly / weekly cron)

Run the trainer on a schedule regardless of corpus size. A nightly
cron on the workspace VM could check corpus size and skip if below
a minimum threshold.

**Latency-to-fresh-adapter:** High; up to 24 hours (nightly) or
7 days (weekly) between cycles.
**Operator burden:** Low; standard cron.
**Cost predictability:** High; predictable cost per cycle.
**Risk:** Adapter may be stale between training runs if the corpus
is growing rapidly.

**Practical first choice for Phase 1** given simplicity and
cost predictability.

### D — Manual (operator runs a make-trainer-cycle command)

The operator explicitly triggers a training run. No automation.

**Latency-to-fresh-adapter:** Operator-dependent.
**Operator burden:** Highest.
**Cost predictability:** High (operator-controlled).
**Risk:** Training runs get skipped under workload pressure.

**Appropriate for Phase 0** (proof-of-life; operator wants full
control) and during initial calibration. Not appropriate as a
permanent regime.

### E — Quality-gated (verdict-accept-rate / diversity thresholds)

Train only when the corpus reaches minimum quality thresholds:
accept-rate ≥ 0.5 over rolling 50 verdicts; at least 3 distinct
clusters represented; at most 1 task-type constituting >80% of
the corpus.

**Latency-to-fresh-adapter:** Longest; depends on quality metrics
being met.
**Operator burden:** Requires quality-gate monitoring.
**Cost predictability:** Lowest; gate may never fire if quality is
poor.
**Risk:** Mismatched quality metrics could permanently gate training.

**Appropriate as an additional guard on top of threshold or cron
triggers, not as the primary trigger.** Especially relevant for
the first DPO training run (requires signed verdicts, which
accumulate more slowly than shadow tuples).

---

## §6 — Corpus-to-adapter pipeline

This section describes the concrete pipeline steps that would need
to be implemented. No code exists today; this is the design of what
would need to exist.

### Step 1 — Filter

Read JSONL records from `data/training-corpus/` and apply:

- `doctrine_version` match (train against records matching the
  current doctrine version)
- `tenant` filter (Vendor corpus for `engineering-pointsav`;
  per-tenant for `tenant-woodfine`)
- `cluster` filter for cluster-specific adapters
- `tuple_type` filter (`apprenticeship` for DPO; `edit` for SFT)
- `stage_at_capture` filter: shadow tuples (no verdict) for SFT;
  tuples with `verdict: refine|reject` for DPO pairs
- `redaction_class` filter: exclude `tenant-private` from
  Vendor-side training

The engineering corpus has a different schema (`tuple_type: edit`)
from the apprenticeship corpus (`tuple_type: apprenticeship`). The
filter step normalises both into the training format below.

### Step 2 — Format conversion

The trajectory-substrate schema is not directly consumable by
HuggingFace TRL or PEFT. Conversion is needed:

**For SFT from engineering corpus:**
The `diff` field in each record maps to a completion; the commit
context (prior files, commit message) maps to a prompt. Approximate
format:
```json
{"prompt": "<system_prompt>\n<file_context>", "completion": "<unified_diff>"}
```

**For SFT from shadow apprenticeship tuples:**
The `attempt.diff` field is the apprentice's output; the `brief.body`
+ scope is the prompt. The `final_diff` (the actual human diff) is
the ground truth.
```json
{"prompt": "<apprentice_prompt>", "completion": "<final_diff>"}
```

**For DPO from verdict-signed apprenticeship tuples:**
When `verdict.verdict == "accept"`:
```json
{"prompt": "<apprentice_prompt>", "chosen": "<attempt.diff>", "rejected": null}
```
When `verdict.verdict == "refine"` or `"reject"`:
```json
{"prompt": "<apprentice_prompt>", "chosen": "<final_diff>", "rejected": "<attempt.diff>"}
```
The DPO library (TRL `DPOTrainer`, or similar) expects this shape.

### Step 3 — Training method

| Adapter target | Corpus | Method | Rationale |
|---|---|---|---|
| `engineering-pointsav` | Engineering edit-corpus | Continued pretraining or SFT | No preference signal yet; SFT on accepted edits teaches style and patterns |
| `cluster-project-slm` | Engineering edit-corpus (project-slm subset) | SFT | Same rationale; narrower scope |
| `apprenticeship-pointsav` | Apprenticeship corpus with signed verdicts | DPO | Requires (chosen, rejected) pairs; only meaningful once verdicts accumulate |
| `constitutional-doctrine` | Doctrine clause corpus | Supervised on clause × role | Trains rule-following |
| `tenant-woodfine` | Woodfine Totebox per-tenant corpus | SFT or DPO | Customer-side; requires Totebox deployment first |

**LoRA rank:** r=8 to r=64 are the production-viable range per the
[lorax-predibase] literature. Lower rank = smaller adapter, faster
training, less expressiveness. For an initial adapter on OLMo 3 7B
(7 billion parameters), r=16 or r=32 is a reasonable starting point.

**Target modules:** Attention (q, k, v, o projections) and feed-forward
(up, down, gate) are the standard LoRA targets.

**Tools:** HuggingFace TRL `SFTTrainer` / `DPOTrainer` + PEFT
`LoraConfig`. Both are Apache-2.0 and Foundry-compatible per the
licence policy. Axolotl (Apache-2.0) provides a higher-level YAML
configuration interface. All three are Python tooling.

The `router-trainer/` precedent in the workspace uses raw `llamafile`
as the teacher model and Python for orchestration. A modern TRL/PEFT
setup is the upgrade path.

### Step 4 — Validation

A held-out set should be reserved before training. Suggested split:
80% train, 10% validation (used during training for early stopping),
10% held-out for post-training evaluation.

Metrics:
- **Perplexity on held-out set** — standard language model metric;
  lower is better.
- **Acceptance-rate proxy** — run the trained adapter on the held-out
  shadow tuples and compute the fraction where the adapter's diff
  matches the human diff closely (edit-distance or semantic similarity
  threshold).
- **Task-specific evaluation** — for `version-bump-manifest`, the
  acceptance test in the brief is the ground truth; the adapter's
  output either passes it or fails.
- **Regression check** — verify the adapter does not degrade on a
  fixed set of reference prompts (prevents catastrophic forgetting).

No held-out set currently exists. It should be created before the
first training run.

### Step 5 — Versioning

Adapter versioning per [conventions/adapter-composition.md] §3:

```
{name}-{doctrine_version}-v{MAJOR}.{MINOR}.{PATCH}.lora
```

Example: `engineering-pointsav-doctrine-0.0.12-v0.1.0.lora`

Each adapter carries embedded metadata (name, version,
doctrine_version, provenance corpus shard identifiers, trained-at
timestamp, trained-by identity).

A Doctrine MINOR bump requires adapter retrain — the adapter
becomes incompatible with the new doctrine version. This is a
documented consequence of the versioning scheme.

### Step 6 — Storage

Target per [conventions/trajectory-substrate.md] §4:

```
~/Foundry/data/adapters/
  engineering-pointsav-v0.1.lora
  cluster-project-slm-v0.1.lora
  apprenticeship-pointsav-v0.1.lora
  ...
```

Each adapter should be:
1. SSH-signed by the trainer's identity
   (`ssh-keygen -Y sign -n foundry-adapter-v1`)
2. Sigstore Rekor-anchored per Doctrine claim #34
   (the `fs-anchor-emitter` Task scope; not yet implemented)
3. Stored with a companion `.sha256` and `.sig` file
4. Referenced in a version manifest at
   `data/adapters/manifest.yaml` so the Doorman can enumerate
   available adapters at startup

For Tier B deployment, adapters would also need to be uploaded to GCS
(`gs://pointsav-public/adapters/`) so the Yo-Yo instance can pull
them at boot.

### Step 7 — Promotion gate

An adapter version is "production-ready" when:

- Validation perplexity is below a threshold (to be determined;
  suggest starting with "not worse than base model by more than 20%")
- Acceptance-rate proxy ≥ 0.6 on held-out shadow tuples for the
  target task-type
- Zero catastrophic-forgetting regression failures on the reference
  set
- Master or Root has reviewed and co-signed the adapter release
  (human-in-the-loop requirement; `flock(2)` on the promotion ledger
  per [conventions/apprenticeship-substrate.md] §6)

The PS.5 graduation criteria from the operationalization plan are the
near-term benchmark: "quality gate project-language verdict
accept-rate ≥0.6 over rolling 50 → continue, below → abort."

---

## §7 — Doorman integration (what changes service-slm-side)

The Doorman today has no adapter loading code. The following code
paths would need modification or creation. **No implementation is
proposed here; this section identifies where the work would land.**

### Adapter loading at inference time

The Doorman would need to load adapters at startup (or hot-reload
on a file-watch signal). The natural home is
`service-slm/crates/slm-doorman/src/adapter_loader.rs` (new file).

At startup, the loader would read `data/adapters/manifest.yaml`,
verify each adapter's SSH signature against `identity/allowed_signers`,
and build an `AdapterRegistry` struct indexed by adapter name +
doctrine version.

### Adapter composition at request time

Per [conventions/adapter-composition.md] §1, the composition is
deterministic given the request context. The `DoormanConfig` struct
(in `slm-doorman/src/router.rs`) would gain an `adapter_registry:
Option<AdapterRegistry>` field. The `Doorman::route()` method would
consult the registry before dispatching to `LocalTierClient` /
`YoYoTierClient`, passing the composed adapter set in the
`ComputeRequest`.

For Tier A (`LocalTierClient`), the llama-server HTTP API supports
adapter loading via custom parameters (GGUF LoRA adapters are loaded
via `-lora` flags at server startup or via the `/v1/completions`
`lora_name` extension field in some llama-server builds — this
needs to be verified against the current llama-server version).

For Tier B, vLLM Multi-LoRA supports per-request adapter selection
via the `model` field extension per [conventions/adapter-composition.md]
§2. The adapter would need to be pre-loaded on the vLLM instance.

### Hot-reload vs restart

The current `local-slm.service` and `local-doorman.service` pattern
suggests restart-required for model changes. Hot-reload is possible
with vLLM (it supports dynamic LoRA loading) but is not available
in llama-server without patching. The initial implementation can
require restart; hot-reload is a later optimisation.

### Versioning in the audit ledger

Each `AuditEntry` in `slm-doorman/src/ledger.rs` would gain an
`adapter_composition: Vec<AdapterRef>` field (adapter name + version).
This is a MINOR bump to the audit contract (new optional field per
`service-slm/docs/audit-endpoints-contract.md` §5 versioning rules).

### Fallback

If adapter load fails (file missing, signature invalid, version
mismatch), the Doorman should fall back to the base model and log
the failure to `tracing` + the audit ledger. No hard panic; the
base model is usable.

### Verdict feedback loop

The `POST /v1/verdict` endpoint (AS-3, wired) already writes
apprenticeship corpus tuples and checks promotion thresholds. The
trainer would consume those tuples. The Doorman does not need further
changes for the verdict side; it already produces the training signal.

---

## §8 — Constitutional and governance considerations

### BCSC continuous-disclosure posture

Training runs are internal-operational events. They do not need to
be disclosed under NI 51-102 unless the training run produces a
"PointSav-OLMo-N" variant that is itself a material product event.
The initial LoRA adapter cycles are internal tooling improvements.

Once continued pretraining of a model variant begins (Year 2 per
the roadmap), that may constitute a material product development event
depending on PointSav's reporting-issuer status at the time. BCSC
disclosure posture applies: label as "planned" / "intended" with
material assumptions stated.

This scoping document itself carries forward-looking content (adapter
timelines, training cost estimates). It should not be published
externally without a BCSC review pass.

### Two-Bottoms Sovereign Substrate (Doctrine claim #34)

Per `conventions/system-substrate-doctrine.md`, the sovereign substrate
envisions adapter releases as inheritable cryptographic artefacts via
apex-cosigning. A trained adapter would be:

1. SSH-signed by the trainer identity at training time.
2. Anchored in Sigstore Rekor v2 via the `fs-anchor-emitter` binary
   (Task scope; the `infrastructure/local-fs-anchoring/` design stub
   names this as the missing component).
3. Optionally apex-cosigned by `ps-administrator` for production
   promotion (treating adapter releases on the same signature
   discipline as Doctrine MINOR bumps).

This is the long-term governance shape. The initial Phase 0-1
implementation can use SSH-signing only; Rekor anchoring is an
additive step.

### Customer-side training (tenant adapter)

Per [conventions/trajectory-substrate.md] §1, the `tenant-woodfine`
adapter must be trained inside the Customer Totebox and remain there.
This means:

1. The training pipeline must be deployable into the Woodfine Totebox
   instance (not yet provisioned).
2. The resulting adapter weights never leave the Woodfine deployment
   instance.
3. The federated marketplace opt-in (Doctrine claim #14) is the
   explicit mechanism for sharing tenant adapters across Customers.

The practical consequence: the `engineering-pointsav` and
`cluster-project-slm` adapters are Vendor-side and can be trained
on the workspace VM or Yo-Yo. The `tenant-woodfine` adapter requires
the Woodfine Totebox deployment first.

### Sovereign Data Foundation

Per the open-question recorded in `cleanup-log.md`, language treating
the Sovereign Data Foundation as a current equity holder or active
auditor must not appear in any document that may become public-facing
without a review pass. Any framing of adapter training as "audited
by the Foundation" would require verification that such a relationship
is documented and current.

---

## §9 — Open questions for operator decision

Each question is answerable in a sentence and changes the
implementation shape.

1. **Deployment option for Phase 0:** Should the first trainer cycle
   run on the workspace VM CPU (Option A: free, slow), or is there
   appetite to use a third-party compute provider (Option D: faster,
   sovereignty trade-off) for the proof-of-life?

2. **Corpus scope for Phase 0:** Which corpus bucket has the most
   value for the first training run — the engineering corpus
   (87 project-slm edit tuples) or the prose-edit apprenticeship
   corpus (13 tuples, all shadow/unscored)?

3. **Training method preference:** Should Phase 0 use continued
   pretraining (fine-tune base model weights directly) or LoRA
   (train a small delta adapter on top of frozen base)? LoRA is
   faster and more reversible; continued pretraining is more thorough
   but requires more compute.

4. **Verdict cadence:** The DPO training path requires signed
   verdicts from the senior (operator). Is there an expectation of
   a regular verdict-signing cadence (e.g., weekly batch at
   session-end), or is verdicts-per-session-end the model?

5. **D4 timeline:** Does the operator have a timeline for the
   D4 image-build pipeline (creating the `pointsav-public` GCP
   project and building the slm-yoyo GCE image)? This gates Phase
   1 on Tier B and all production training cycles.

6. **Adapter signing identity:** Should trained adapters be signed
   as `ps-administrator` (the workspace identity), or should there
   be a distinct `ps-trainer` identity that signs model artefacts?
   Using a distinct identity would make adapter signatures auditably
   separate from commit signatures.

7. **Storage location for adapter weights:** Should adapters live
   on the workspace VM at `~/Foundry/data/adapters/` (local-only,
   gitignored), or should they also be stored in GCS for
   Yo-Yo-accessible serving?

8. **Sovereignty limit for Phase 0:** Is the operator willing to
   use Modal or RunPod for Phase 0 training (Option D) to avoid
   blocking on CPU-only speed, accepting that corpus data transits
   to a third-party provider temporarily?

9. **Minimum viable adapter scope:** Should the first
   production-deployed adapter be `cluster-project-slm` (the
   narrowest scope; trained on this cluster's 87 edit-tuples) or
   `engineering-pointsav` (broader scope across all engineering
   clusters)?

10. **Llama-server LoRA support:** Does the currently deployed
    `local-slm.service` llama-server binary support runtime LoRA
    adapter loading (via `-lora` flags or the lora extension in
    completions requests)? If not, would the operator be willing to
    rebuild the binary with that support enabled?

11. **Training Python tooling:** Should the trainer be built with
    HuggingFace TRL/PEFT (the mainstream choice, well-documented,
    some dependency overhead), or should the team build on top of
    the `llamafile` + Python distillation pattern already demonstrated
    in `router-trainer/` (lighter dependency footprint)?

12. **Corpus minimum size:** Is 87 engineering tuples considered
    sufficient for a first SFT training run, or should the team
    wait for a specific corpus size threshold (e.g., 200 tuples
    per task-type)?

13. **Existing trainer code in other clusters:** Does Master know
    of any other trainer codebase in the workspace or in a sibling
    repo that is more aligned with the current OLMo 3 / TRL/PEFT
    substrate than the `router-trainer/` precedent?

---

## §10 — Phased recommendation

The following phases are ordered by cost and reversibility. Each
phase stands alone; Phase 1 does not require Phase 0 to have been
completed successfully, but Phase 0 de-risks Phase 1.

### Phase 0 — Proof-of-life ($0, 1 day, workspace VM CPU)

**Goal:** Verify that the corpus is well-formed enough to train
against and that a training loop can produce an adapter file.

**Operator green-light required:** Yes — specific decision on which
corpus bucket and which training tool.

**Steps:**
1. Create `~/Foundry/data/adapters/` directory.
2. Write a Python script (under `service-slm/scripts/train_adapter.py`)
   that reads `data/training-corpus/engineering/project-slm/*.jsonl`,
   extracts `(commit_context, diff)` SFT pairs, converts them to
   HuggingFace `Dataset` format, and runs `SFTTrainer` for 10-20 steps
   on CPU with a 4-bit quantised OLMo 3 7B (via `bitsandbytes`
   NF4 quantisation, which enables QLoRA training on CPU, albeit slowly).
3. Verify the script runs to completion and produces a `.lora` or
   PEFT-compatible adapter directory.
4. Do NOT deploy the resulting adapter to the Doorman.
5. Record the training perplexity curve; note whether it decreases
   (signal that the model is learning something).

**Dependencies:** Python 3.10+, `transformers`, `peft`, `trl`,
`bitsandbytes` (or CPU alternative). None of these exist in the
workspace Python environment today; they need to be installed.
The workspace VM has network access and can `pip install` them.

**Estimated effort:** 4-8 hours of implementation + 8-24 hours of
training wall time (background, does not need supervision).

**Expected outcome:** Either a successful adapter artefact (proceed
to Phase 1) or a clear error message showing what is missing
(informs Phase 1 scope).

### Phase 1 — First production-deployed adapter (1-2 weeks, Option A or B)

**Goal:** Deploy a LoRA adapter to the Doorman; verify the Doorman
loads and uses it; confirm the adapter does not degrade base
performance.

**Steps:**
1. Write the corpus-to-trainer format conversion script
   (building on Phase 0).
2. Write `service-slm/crates/slm-doorman/src/adapter_loader.rs`
   with adapter signature verification and registry construction.
3. Extend `DoormanConfig` and `Doorman::route()` with adapter
   composition.
4. Verify that the current llama-server binary supports LoRA
   loading (or rebuild with support enabled — Master scope for
   the systemd unit).
5. Run a full training cycle on the engineering corpus (all
   project-slm edit-tuples); validate against held-out set;
   sign the adapter; deploy to `data/adapters/`.
6. Restart `local-doorman.service` with `ADAPTER_PATH` configured;
   smoke-test a `POST /v1/chat/completions` request; verify the
   audit ledger records the adapter version.

**Dependencies:** Phase 0 completion; decision on llama-server LoRA
support; decision on adapter signing identity.

**Estimated effort:** 3-5 days of implementation + training time.

### Phase 2 — Formalise the training loop (2-4 weeks)

**Goal:** Threshold-triggered or cron-triggered training;
validation gates; promotion gate integration.

**Steps:**
1. Implement corpus-size monitor (check tuple count per task-type
   against the 50-tuple threshold from
   [conventions/apprenticeship-substrate.md] §2).
2. Automate training run on threshold-crossing.
3. Add validation harness (held-out perplexity +
   acceptance-rate proxy).
4. Wire promotion gate: only advance a task-type to `spot-check`
   if the adapter passes the validation threshold.
5. Document the adapter release procedure in
   `service-slm/docs/deploy/adapter-release-runbook.md`.

### Phase 3 — Multi-adapter composition and PS.5 graduation

**Goal:** Multiple adapter types deployed and composed at request
time; per-task-type dispatch; the PS.5 graduation criteria
("service-SLM first responder on version-bump-manifest")
operational.

**Steps:**
1. Train and deploy `apprenticeship-pointsav` adapter once
   sufficient signed verdicts have accumulated (target: 50 DPO
   pairs with accept-rate ≥ 0.6).
2. Implement adapter selection logic in the Doorman based on
   `task_type` in the brief + request context.
3. Verify that the multi-adapter composition path works
   end-to-end on Tier B (requires D4 and the Yo-Yo instance;
   vLLM Multi-LoRA is the intended serving stack).
4. Establish the `engineering-pointsav` adapter as a workspace-wide
   shared resource consumed by all clusters' Doormans.

---

## Research trail

### Done — what informed this draft

- [conventions/apprenticeship-substrate.md] — full spec for the
  Apprenticeship Substrate; promotion thresholds, verdict format,
  corpus paths, stage transitions; §7A prose-edit task type
- [conventions/adapter-composition.md] — composition algebra
  (`base ⊕ ... ⊕ cluster`), adapter typology, runtime composition,
  OS-of-AI metaphor; max 3 adapters per request constraint
- [conventions/trajectory-substrate.md] — the three corpora
  (constitutional / engineering / tenant-runtime); capture mechanics;
  adapter library at `data/adapters/`; L1 done / L3 pending
- [conventions/llm-substrate-decision.md] — OLMo 3 choice
  rationale; Year 1-5 continued-pretraining roadmap; Tier A/B/C
  description; Tier B names GCP Cloud Run (stale — Yo-Yo GCE per
  zero-container-runtime convention)
- [conventions/language-protocol-substrate.md] — 4-family adapter
  taxonomy; prose-edit task type; maximum 3 adapters per request
- [conventions/cluster-wiki-draft-pipeline.md] — prose-edit
  pipeline; DPO pairs per draft lifecycle
- [conventions/draft-research-trail-discipline.md] — frontmatter
  schema for this document; research-trail discipline
- [service-slm/docs/audit-endpoints-contract.md] v0.2.0 — what
  the Doorman's corpus pipeline produces; `verdict-issued`
  event_type in the audit capture vocabulary
- [service-slm/ARCHITECTURE.md] — Ring 3b LoRA adapter stack;
  current crate structure; `memory/adapters/` forward declaration;
  `adapter_composition` field in `ApprenticeshipAttempt`
- [service-slm/crates/slm-doorman/src/apprenticeship.rs] — AS-2
  implementation; shadow tuple write path; `data/training-corpus/apprenticeship/`
  paths are live
- [clones/project-slm/.agent/manifest.md] — `adapter_routing.trains`
  declares 3 adapters: cluster-project-slm, engineering-pointsav,
  apprenticeship-pointsav
- [vendor/pointsav-monorepo/service-slm/router-trainer/] — existing
  trainer artefact inspection; `distill_knowledge.py`,
  `ignite_teacher.sh`, 15-tuple `training_dataset.jsonl`
- [data/training-corpus/] — live corpus state: 14 apprenticeship
  tuples (13 prose-edit, 1 design-edit), 87 engineering tuples
  in project-slm, 0 adapters in `data/adapters/` (directory absent)
- [service-slm/NEXT.md] — B7 LIVE state; 143 tests; corpus stats
  at flow-online moment

### Suggested — what the next-leg gateway or operator should consult

- [external: huggingface.co/docs/trl/sft_trainer] — priority high;
  verify TRL SFTTrainer API is stable and compatible with OLMo 3
  tokenizer before writing the Phase 0 script
- [external: huggingface.co/docs/trl/dpo_trainer] — priority high;
  verify DPO pair format expectations; confirm OLMo 3 compatibility
- [external: github.com/axolotl-ai-cloud/axolotl] — priority medium;
  evaluate as higher-level trainer config option vs raw TRL/PEFT
- [external: docs.vllm.ai/en/latest/features/lora/] — priority high;
  verify current vLLM Multi-LoRA API (model field, pre-load vs
  dynamic-load per request) before writing Tier B adapter integration
- [external: allenai.org/olmo] — priority high; check whether AllenAI
  publishes official fine-tuning scripts or hyperparameter
  recommendations for OLMo 3 7B LoRA
- [infrastructure/slm-yoyo/tofu/] — priority medium; verify which
  Python environment ships in the planned Yo-Yo GCE image before
  writing a trainer that assumes specific Python package availability
- [external: github.com/ggerganov/llama.cpp docs on LoRA] — priority
  medium; verify whether the current `llama-server` binary deployed
  in `local-slm.service` supports runtime LoRA loading without
  a recompile; this blocks Phase 1 Option A path
- [external: sigstore.dev/rekor] — priority low; verify Rekor v2
  log-tile API for adapter anchoring (Doctrine claim #34); not
  blocking Phase 0-1
- [conventions/system-substrate-doctrine.md] — priority low;
  review the apex-cosigning mechanism for adapter releases when
  designing the Phase 2 promotion gate

### Open questions

(Cross-references §9 operator questions. These are the research-trail
form; §9 is the operator-decision form.)

- Questions 1-13 from §9 above.
- Is there a BCSC review required before the first adapter is
  deployed if the training corpus includes any `bcsc_class:
  forward-looking` editorial tuples?
- Does the `data/training-corpus/apprenticeship/prose-edit/woodfine/`
  tenant-private shard ever contain records that would need to be
  explicitly excluded from Vendor-side training? (The current 14
  files are all `pointsav` tenant; the `woodfine` directory has
  1 tuple. Confirm that tuple's `redaction_class` before the
  first cross-tenant filter pass.)
