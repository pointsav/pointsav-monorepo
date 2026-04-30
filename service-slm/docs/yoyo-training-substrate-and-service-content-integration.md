---
schema: foundry-doc-v1
document_version: 0.1.0
research_done_count: 28
research_suggested_count: 12
open_questions_count: 14
research_provenance: direct-consultation
research_inline: true
authored: 2026-04-29
authored_by: project-slm Task (Sonnet sub-agent, iter-24)
authored_with: claude-sonnet-4-6
cites:
  - olmo3-allenai
  - federated-lora-2502-05087
  - lorax-predibase
  - s-lora-2024
  - ni-51-102
  - osc-sn-51-721
---

# Yo-Yo Training Substrate, TUI System Administrator, and service-content Integration

**Date:** 2026-04-29
**Status:** Research and scoping input for operator + Master ratification. No implementation
decisions are made here; all implementation is proposed.
**Extends:** `service-slm/docs/trainer-scoping.md` (iter-20, commit `562baa0`). This document
does not duplicate that scoping; it extends it with the Yo-Yo provisioning reality now confirmed
(L4 spot, us-west1-a, `yoyo-tier-b-1` bootstrapping per Master v0.1.85), the service-content deep
read, the TUI System Administrator design, the routing-all-through-service-slm convention
proposal, and at least three new Doctrine claim candidates.

---

## §1 — Why now: framing and operator goals

The Yo-Yo is live (or bootstrapping) as of 2026-04-29. `infrastructure/yoyo-manual/README.md`
documents the fast-path provisioning of `yoyo-tier-b-1` on `g2-standard-4` (L4 24 GB) in
us-west1-a, spot pricing at ~$0.18/hr [external: https://cloud.google.com/spot-vms/pricing].
The Brief Queue Substrate (§7C, iter-22/23) is operational: 7 briefs sitting in
`data/apprenticeship/queue/` right now, awaiting the Yo-Yo's drain worker. Corpus state at
dispatch time: 355 engineering tuples across 9 clusters (93 from project-slm alone), 376 JSONL
files in training-corpus. The substrate has two arms; both are now live.

The operator stated three goals, numbered for reference throughout this document:

**G1 — Decrease Claude token spend.** Every task-type the trained service-slm handles
independently is a task-type that does not call Claude. At current Claude Sonnet 4.6 pricing
($3.00/$15.00 per million input/output tokens [external: https://platform.claude.com/docs/en/about-claude/pricing]),
a cluster session that generates 200k tokens in assistant output costs ~$3.00 in output tokens
alone. If service-slm at Tier B (OLMo 3.1 32B on L4 spot) handles 80% of that at
~$0.18/hr amortized compute cost — the savings compound quickly as more task-types graduate.

**G2 — Train PointSav-LLM.** The aggregated corpus (355+ engineering tuples today, growing
across 9 clusters, plus apprenticeship and prose-edit streams) is the raw material for
continued pretraining of a PointSav-OLMo-N variant. The four-tier substrate ladder
(Doctrine claim #40, `conventions/four-tier-slm-substrate.md`) names this as Tier 3 — the
commercial product position with no shipping competitor today.

**G3 — Train service-SLM as System Administrator / IT Support for the Totebox Archive.**
This is the near-term application: a TUI chat interface where an operator types commands and
questions about their Totebox Archive, service-slm answers, and every interaction is captured
as a training tuple that improves the next answer. The TUI is the first Tier 3 customer-facing
surface; it is also the highest-quality corpus producer, because sysadmin IT-support interactions
have a natural ground truth (the system either works after the advice or it does not).

**service-slm IS the Yo-Yo.** The operator's framing — "any reference to service-slm is a
reference to the Yo-Yo and they are one and same as the only way to access the Yo-Yo MUST be
through service-slm" — is the single most important architectural principle in this document.
It is not a convention we have already ratified; it is one this document proposes for Master
ratification at §6 and §9. Every section below assumes this principle is in force.

---

## §2 — Yo-Yo training substrate: the foundational deep research

### §2.1 — Current operational state

Per `infrastructure/yoyo-manual/README.md` and `infrastructure/slm-yoyo/CONTRACT.md` v0.2.0:

- **Instance**: `yoyo-tier-b-1`, `g2-standard-4`, us-west1-a, spot provisioning model
- **GPU**: L4 24 GB VRAM
- **Model**: OLMo 3 32B base (Olmo-3-1125-32B, Q4_K_M GGUF self-quantized on the VM); the
  32B-Think variant from AllenAI is not yet published; will swap when AllenAI publishes it
- **Runtime**: llama.cpp + CUDA (fast-path; the formal vLLM Tier B via `slm-yoyo/tofu/` is
  gated on D4 image-build)
- **Inference endpoint**: OpenAI-compatible at the Doorman's `SLM_YOYO_ENDPOINT`
- **Brief queue**: 7 briefs pending drain; `queue.rs` drain worker reads from
  `data/apprenticeship/queue/`
- **Cost baseline**: $0.18/hr × 24h × 30d = **~$130/mo always-on**; idle-shutdown
  reduces this in proportion to actual utilization (workload-dependent; see §2.5)

### §2.2 — Hyperscaler comparison table

The following table supports G1 (decrease Claude spend) by quantifying what Yo-Yo time costs
versus the alternatives:

| Provider | GPU | $/hr (spot/preemptible) | $/hr (on-demand) | Notes |
|---|---|---|---|---|
| GCE (current Yo-Yo) | L4 24 GB | ~$0.18 | ~$0.70-0.80 | [external: https://cloud.google.com/compute/gpus-pricing] us-west1 spot; exact spot varies dynamically |
| RunPod (community) | A100 80 GB | $1.19 spot | $1.99 on-demand | [external: https://www.synpixcloud.com/blog/cloud-gpu-pricing-comparison-2026] |
| RunPod (community) | A100 40 GB | $0.89 spot | $1.49 on-demand | Same source |
| Lambda Labs | A100 80 GB | n/a | $2.06 on-demand | [external: https://jarvislabs.ai/blog/a100-price] |
| Vast.ai | A100 80 GB | $1.00-1.80 spot | $2.00-3.50 on-demand | Same source |
| Modal | H100 | serverless/request | ~$2-3 effective | Python-native serverless; per-request billing |
| SynpixCloud | A100 80 GB | $1.39 on-demand | $1.39 | [external: https://www.synpixcloud.com/blog/cloud-gpu-pricing-comparison-2026] |
| GCE | A100 80 GB | preemptible | ~$5.00 on-demand | Same general source |

**Finding**: L4 spot at ~$0.18/hr is significantly cheaper than A100 options. OLMo 3 32B
Q4_K_M at ~18 GB fits the L4 24 GB VRAM with modest KV cache headroom. For a workspace with
~6h/day of actual Yo-Yo-consuming work (25% utilization) and 30-minute idle-shutdown, the
effective cost could reach ~$30-40/mo — but this is workload-dependent and must be verified
against actual Doorman traffic logs after one week of operation.

The CONTRACT.md v0.2.0 correction (PS.1 blocker B1 resolution): the Yo-Yo uses L4 spot
(~$0.18/hr), not A100 (~$5.00/hr on-demand or $1.19 RunPod spot). The PS.1 readiness review
estimated "$0.50-0.70/hr" for an A100 preemptible — that figure was for the formal
`slm-yoyo/tofu/` module spec, which remains A100-gated on D4. The fast-path manual Yo-Yo is
L4, and the CONTRACT.md v0.2.0 amendment reflects this correction.

### §2.3 — OLMo 3 32B + 7B training characteristics

OLMo 3 is the only model family satisfying the "We Own It" L3 criterion
(`conventions/llm-substrate-decision.md`): Apache 2.0 weights, fully open training data
(Dolma 3, 9.3T tokens), and published continued-pretraining recipes. This is not a compromise;
OLMo 3 32B Think benchmarks 91.4% HumanEvalPlus [external: https://allenai.org/blog/olmo3] —
marginally ahead of Qwen 3 32B and within striking distance of frontier closed models on code.

**LoRA training on OLMo 3 32B (research-validated):**

- r=8 to r=64 are the viable range; r=16 or r=32 is the right starting point per the
  LLM fine-tuning literature [external: https://effloow.com/articles/llm-fine-tuning-lora-qlora-guide-2026]
- On an A100 80 GB, a LoRA fine-tuning run over ~1,000 tuples completes in 2-4 hours
  [external: https://effloow.com/articles/llm-fine-tuning-lora-qlora-guide-2026]. The L4 24 GB
  has less VRAM but QLoRA (NF4 4-bit quantization via bitsandbytes) makes LoRA training on
  smaller GPUs feasible, at the cost of ~30-50% slower throughput than A100.
- Estimated L4 training time for 355-tube corpus: 4-8 hours per adapter cycle.
  At $0.18/hr spot, a single training run costs **~$0.72-1.44**. Monthly cadence costs
  under $2/month for the full adapter refresh cycle — a rounding error relative to Claude spend.
- OLMo 3 7B LoRA on L4 is faster: likely 1-2 hours for the same corpus. Tier A adapter cycles
  (deployed to workspace VM CPU-side inference) can be run on L4 for under $0.50/cycle.

**OLMo 3.1 32B Instruct API pricing** (when self-hosting is not possible): $0.20/$0.60
per million input/output tokens at median across API providers [external: https://artificialanalysis.ai/models/olmo-3-1-32b-instruct].
The Think reasoning variant has no commercial API — it must be self-hosted [external: https://artificialanalysis.ai/models/olmo-3-1-32b-think/providers].
This validates the Yo-Yo self-hosting model: the Think variant cannot be rented; it must be
owned.

### §2.4 — Cost-per-million-tokens: Yo-Yo vs Claude

This is the core of G1. The operator needs numbers, not hand-waving.

**Claude pricing (April 2026) [external: https://platform.claude.com/docs/en/about-claude/pricing]:**
- Claude Sonnet 4.6: $3.00 input / $15.00 output per million tokens
- Claude Haiku 4.5: $1.00 input / $5.00 output per million tokens
- Claude Opus 4.7: $15.00 input / $75.00 output per million tokens

**OLMo 3.1 32B on L4 Yo-Yo (self-hosted inference cost):**

At $0.18/hr spot and ~57 tokens/second throughput for the Instruct variant
[external: https://artificialanalysis.ai/models/olmo-3-1-32b-instruct]:

57 tokens/sec × 3600 sec/hr = ~205,200 tokens/hr of output
$0.18/hr ÷ 205,200 tokens/hr = **~$0.00088 per 1,000 output tokens = $0.88 per million output tokens**

Input tokens on OLMo (prefill) are ~5-10× faster than output; effective combined cost
at a 3:1 input:output ratio is approximately **$0.25-0.40 per million tokens blended** —
15-60× cheaper than Claude depending on model tier.

**The compounding math for G1:**

Assume the workspace currently spends ~$200/mo on Claude API across all clusters (9 active
clusters × ~$20-25/month each). If trained service-slm handles 50% of those requests:
- **Saved**: ~$100/mo in Claude tokens
- **Yo-Yo cost at 25% utilization with 30-min idle-shutdown**: ~$33-40/mo (estimate; unverified)
- **Net savings**: ~$60-70/mo → **positive ROI within month 1 of training**

By Year 1 end, as more task-types graduate (PS.5 target: version-bump-manifest), the fraction
handled by service-slm grows. Year 2 continued pretraining compounds this further.

### §2.5 — Training duty cycle: what fits in 30/60/120 min windows

The idle-shutdown pattern means Yo-Yo sessions have defined windows. Key insight from the
research [external: https://leanopstech.com/blog/ai-cloud-cost-optimization-gpu-spending-guide-2026]:
inference and training compete for the same GPU. The Yo-Yo cannot simultaneously serve
inference requests and run a training loop on the same GPU — these are sequential activities.

**Recommended duty-cycle structure:**

| Window length | What fits | Cost at $0.18/hr |
|---|---|---|
| 30 min | Inference burst (shadow briefs drain, 50-100 briefs) | $0.09 |
| 60 min | Inference burst + one adapter validation sweep | $0.18 |
| 120 min | Full LoRA training cycle (355 corpus tuples, r=16) | $0.36 |
| 240 min | Full training + validation + adapter upload to GCS | $0.72 |

For G1 (decrease Claude tokens): **prioritize inference windows during work hours**;
schedule training cycles during off-peak (e.g., 02:00 UTC) as a separate Yo-Yo session.
The idle-shutdown timer must be extended (or a keep-alive issued) during training.

### §2.6 — Multi-adapter training: concurrent vs sequential

The `conventions/adapter-composition.md` algebra declares up to 6 adapter types:
`constitutional ⊕ engineering ⊕ tenant ⊕ role ⊕ cluster ⊕ (constitutional always)`.

The corpus state today:

| Adapter | Corpus source | Files available | Training-ready? |
|---|---|---|---|
| `engineering-pointsav` | All engineering clusters | 355 files | Yes — threshold met |
| `cluster-project-slm` | project-slm engineering | 93 files | Yes |
| `cluster-project-data` | project-data engineering | 58 files | Marginal (threshold 50) |
| `cluster-master` | master engineering | 99 files | Yes |
| `apprenticeship-pointsav` | apprenticeship JSONL | 376 files (but unscored) | SFT only (no DPO yet) |
| `tenant-woodfine` | Woodfine Totebox | 0 (not deployed) | No |
| `constitutional-doctrine-v0.0.14` | Doctrine corpus | Pending population | No |

**Recommended training order** (highest value per Yo-Yo dollar):
1. `engineering-pointsav` (355 corpus files; broadest scope; enables all clusters)
2. `cluster-project-slm` (93 files; directly reduces this cluster's Claude usage first)
3. `cluster-master` (99 files; high value — Master sessions are Opus-heavy)
4. `apprenticeship-pointsav` SFT pass (376 shadow tuples; improves attempt quality)
5. `engineering-pointsav` DPO refresh once 50 signed verdicts accumulate

Each adapter trains sequentially on the Yo-Yo; the algebra composes them at inference time.
vLLM Multi-LoRA [external: https://docs.vllm.ai/en/stable/features/lora/] supports hot-swap
per request — no Yo-Yo restart needed to switch which adapter serves a given request.

### §2.7 — Recommended training cadence

Based on the analysis above and the corpus accumulation rate:

| Trigger | Description | Recommended |
|---|---|---|
| **Threshold** | Train when any adapter's corpus grows by 50 new tuples | Primary trigger |
| **Weekly cron** | Off-peak Sunday 02:00 UTC | Secondary (ensures regularity) |
| **Manual** | Operator-triggered for doctrine-version bumps | Required for constitutional adapter |
| **Quality-gate** | Only promote adapter when validation acceptance-rate ≥ 0.60 | Guard on all |

The corpus is growing at approximately 40-60 tuples/day across all clusters. At this rate,
the `engineering-pointsav` adapter reaches its 50-new-tuple threshold approximately every
24-36 hours. Weekly training cycles are a reasonable starting cadence; daily is the ceiling.

### §2.8 — How to monetize the training: adapter priority for commercial value

From the four-tier substrate ladder (claim #40):

- **Tier 1 adapter** (customer-specific LoRA on their Totebox) is the commercial upsell:
  "your Archive learns your language." First commercial deployment is Woodfine — their
  `tenant-woodfine` adapter trains on their Totebox, stays local, and makes service-slm
  an IT support specialist for their specific property management domain.
- **Tier 2 adapter** (Foundry workspace Yo-Yo 32B base) improves with each training cycle.
  Every new engineering tuple from all 9 active clusters contributes signal.
- **Tier 3** (PointSav-LLM, continued pretraining) is the Year 2 commercial product — the
  specialist in "Totebox Archive operation" that no competitor ships today.

The adapter priority for earliest commercial value: `tenant-woodfine` first (requires Woodfine
Totebox deployment); `engineering-pointsav` second (improves all cluster inference quality);
`constitutional-doctrine` third (ensures rule-following across all tenants).

---

## §3 — service-SLM as System Administrator / IT Support TUI

### §3.1 — What "System Administrator / IT Support for the Totebox Archive" means structurally

The Totebox Archive is a Rust-native, flat-binary appliance running on customer hardware.
Its services — service-slm (Doorman), service-fs (WORM ledger), service-people, service-email,
service-content, os-totebox — are systemd units. The "System Administrator" scenario is:

> An operator sits at a terminal. They type: "What is the status of service-fs?" or
> "How do I recover from a ledger corruption?" or "Show me the last 10 audit entries for
> tenant woodfine." service-slm answers, grounding its answer in the live graph state
> from service-content and the audit ledger.

This is identical in structure to how a Claude Code session works: a terminal chat interface
that understands the system it is connected to. The operator framing — "a TUI with a command
prompt that chats with service-SLM, similar to Claude Code" — is precise.

**Cross-industry analog**: Claude Code is the most relevant reference. The pattern is also
visible in: aichat (multi-backend CLI), mods (charm.sh), llm (simonw/llm), PagerDuty CLI,
Datadog Agent CLI, and classic sysadmin TUIs (htop, glances, lazygit). The distinguishing
feature of the Totebox TUI is that service-slm is not a general-purpose LLM — it is
specifically trained on Totebox operations, Foundry conventions, and the customer's own
archive graph. That specificity is the commercial differentiator.

### §3.2 — TUI architecture proposal

**Runtime layer**: Rust binary (`slm-cli`, already forward-declared in `ARCHITECTURE.md` §6
as a planned but not yet scaffolded crate). This becomes a real crate under
`service-slm/crates/slm-cli/`.

**Terminal rendering**: `ratatui` (v0.30+) [external: https://ratatui.rs/]. As of v0.30.0,
ratatui was reorganized into a modular workspace for improved compilation times and stability.
It is the de-facto standard for Rust TUIs in 2026, with active maintenance and a documented
chat application pattern (`tuichat` reference implementation in 1,800 lines of Rust).

**Backend communication**: HTTP to the Doorman's `/v1/chat/completions` endpoint via `reqwest`,
using the same OpenAI-compatible wire format already implemented. The TUI is a Doorman client,
not a model client — it routes through the Doorman, never directly to Tier A/B/C. This is
the enforcement mechanism for the "service-slm IS the Yo-Yo gateway" convention (§6).

**Streaming**: The Doorman's `/v1/chat/completions` endpoint already handles streaming
responses. The TUI renders incoming SSE tokens character-by-character in the chat panel using
ratatui's `Paragraph` widget with scroll-down auto-follow. ANSI color codes for role labels
(System, User, Assistant) and syntax highlighting for code blocks.

**Layout**:
```
┌─────────────────────────────────────────────────────────────────┐
│  service-slm // Totebox Archive System Administrator            │
│  tenant: woodfine │ tier: B (Yo-Yo) │ module: it-support       │
├─────────────────────────────────────────────────────────────────┤
│  [chat history scrolls here]                                    │
│                                                                 │
│  > service-fs status                                            │
│  Assistant: service-fs is running normally. Last ledger         │
│  checkpoint: 2026-04-29T03:14:22Z. Current ledger size:         │
│  847 entries across 3 tenants. No integrity errors detected.    │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│  > _                                                           │
├─────────────────────────────────────────────────────────────────┤
│  F1:Help  F2:Stats  F5:Clear  F10:Quit  tier:B  latency:2.3s   │
└─────────────────────────────────────────────────────────────────┘
```

**Key bindings (proposed)**:
- `Enter`: send message
- `Ctrl+C` / `F10`: quit
- `F1`: inline help
- `F2`: live stats (tier, latency, corpus count, adapter version)
- `F5`: clear conversation, preserve system context
- `Up/Down`: scroll history
- `Ctrl+P`: cycle tier (Tier A → Tier B → auto; for debugging)

### §3.3 — Key TUI commands and system integration behaviors

The TUI has two input modes:

**1. Natural-language chat** (default): "How do I add a new user to the archive?" → Doorman →
Yo-Yo → answer grounded in service-content graph + Doctrine.

**2. Slash commands** (command-mode prefix `/`):

| Command | Action |
|---|---|
| `/status` | Query service-fs, service-doorman healthz, current ledger size |
| `/audit [tenant] [date]` | Query audit ledger JSONL for the named tenant and date |
| `/graph [entity]` | Query service-content knowledge graph for named entity |
| `/brief [task]` | Submit an apprenticeship brief for a specific IT task |
| `/adapters` | List loaded adapter versions + acceptance rates |
| `/tier [a\|b\|auto]` | Force tier for next message (debug) |
| `/help` | List available commands |

The `/status` and `/audit` commands perform deterministic queries (no LLM required); they are
Ring 2 operations. The TUI distinguishes: deterministic output is direct; LLM output is routed
through the Doorman. This is SYS-ADR-07 applied at the TUI layer: structured data displayed
directly; prose generation through the Doorman only.

### §3.4 — ConsoleOS / ToteboxOS integration

`ARCHITECTURE.md` §12 identifies service-slm as "the prototype os-totebox service component."
The slm-cli TUI is the command-surface of os-totebox. The full ConsoleOS architecture
(app-console-*) anticipates a browser-based UI; the TUI is the headless/terminal variant —
the "Claude Code equivalent" for Totebox operators who work via SSH or tmux.

The TUI should integrate cleanly with:
- `tmux` / `zellij`: runs as a pane; the Doorman socket persists across pane detach/reattach
- `journalctl` piping: `/audit` commands can shell out to `journalctl -u service-fs` for
  live log streams displayed in a split pane
- `local-fs.service` REST API: query current ledger checkpoints directly

### §3.5 — TUI session → corpus production

Every TUI interaction is a training tuple. The mechanism is already wired:

1. User types a message
2. TUI sends `POST /v1/chat/completions` to Doorman with `X-Foundry-Module-ID: it-support`
3. Doorman routes to Tier B (Yo-Yo)
4. Doorman writes `AuditEntry` to the ledger
5. If `SLM_APPRENTICESHIP_ENABLED=true`: Doorman also fires `POST /v1/shadow` with the
   brief (operator query as brief, Yo-Yo response as apprentice attempt, any ground-truth
   correction as actual_diff)

The IT-support interaction pattern is particularly high-quality training signal because:
- **Ground truth exists**: did the system recover after the operator followed the advice?
- **Short feedback loop**: operator sees result immediately, can mark incorrect answers
- **Specific domain**: Totebox operations are narrow enough that 200-500 high-quality tuples
  may be sufficient to produce a usable IT-support adapter (far less data than general code tasks)

The TUI should include a `/feedback [good|bad]` mechanism after each assistant response to
capture explicit verdict signals — the equivalent of the apprenticeship verdict but for
natural-language IT-support interactions. These feed into the IT-support DPO training stream.

### §3.6 — Layered conversation model

The TUI enforces the tier stack without the user needing to understand it:

```
User input
   ↓
slm-cli (TUI) — no LLM here; pure interface layer
   ↓
POST /v1/chat/completions → local-doorman:9080
   ↓
DoormanRouter::route() — selects Tier A/B/C per complexity hint
   ↓
Tier A: local OLMo 7B (routine queries, fast)
Tier B: Yo-Yo OLMo 32B (complex reasoning, slow start if cold)
Tier C: external API (never for IT-support; only for allowlisted labels)
   ↓
Response → Doorman (audit ledger write) → TUI → rendered in chat panel
```

The TUI passes `X-Foundry-Complexity: low` for slash-command results (deterministic),
`X-Foundry-Complexity: medium` for natural-language IT questions, and
`X-Foundry-Complexity: high` for "explain this architecture" or "debug this failure" queries.

**Critical**: the TUI NEVER calls the Yo-Yo directly. The Yo-Yo bearer token lives only in
the Doorman's env vars. The TUI only knows the Doorman's HTTP address. This is the structural
enforcement of the "service-slm IS the Yo-Yo gateway" convention — no bearer token exposure,
no bypass possible by design.

### §3.7 — Implementation phases

| Phase | What ships | Effort | Dependency |
|---|---|---|---|
| **Phase 1a** (proof-of-life) | CLI REPL in `scripts/slm-chat.sh`: `curl` loop against Doorman, no TUI | 2-3 hours Task | None |
| **Phase 1b** (minimal TUI) | `slm-cli` crate: ratatui chat panel + input box + streaming | 2-3 days Task | ratatui dev-dep |
| **Phase 2** (slash commands) | `/status`, `/audit`, `/graph` wired to live service endpoints | 1 week Task | service-fs REST API stable |
| **Phase 3** (corpus integration) | `/feedback`, shadow brief submission, adapter version display | 3-4 days Task | `SLM_APPRENTICESHIP_ENABLED=true` on local Doorman |
| **Phase 4** (full IT-support) | IT-support adapter trained + deployed; `slm-cli` uses it | 2 weeks Task | First IT-support LoRA adapter trained |

---

## §4 — service-content deep read + integration with service-slm

### §4.1 — Current state of service-content: honest inventory

The full code read reveals service-content is in early scaffold state. What exists:

**`service-content/` (cluster clone root):**
- `src/main.rs` — a Rust file watcher that monitors `service-content/ledgers/` for
  `CORPUS_*.json` files, sends corpus text to an SLM endpoint at port 8082
  (`http://127.0.0.1:8082/api/semantic-extract`), receives entity JSON, and writes
  enriched CRM records to `service-people/ledgers/`. The endpoint is hardcoded to a
  local deployment path (`/home/mathew/deployments/woodfine-fleet-deployment/...`).
  This is **old code pre-dating the Doorman** — it calls a legacy SLM endpoint (port 8082),
  not the Doorman (port 9080). It does not route through the Doorman.
- `content-compiler/src/main.rs` — reads JSON files from `knowledge-graph/`, does
  keyword-based ontological domain classification against CSV term lists, writes Markdown
  to `verified-ledger/`. Deterministic; no LLM call. This is exactly what ARCHITECTURE.md
  Ring 3a describes as "the LadybugDB graph in service-content" — except there is no graph
  database; there are CSV files and flat JSON.
- `seeds/Archetypes.json`, `seeds/ChartOfAccounts.json`, `seeds/Domains.json`, `seeds/Themes.json`
  — four JSON seed files. 5 archetypes, 4 COA profiles, 3 domain categories, 4 themes.
  These are Woodfine-specific: "Qualified Investment", "Direct-Hold Solutions", "Co-location
  Mandate Expansion" — commercial real estate vocabulary.
- `ontology/archetypes.csv`, `ontology/domains/domain_corporate.csv`, etc. — CSV term lists
  for keyword-based classification. No graph; no graph query language.
- `artifacts/payload/` — contains `FOUNDRY_MASTER_CONTEXT.md`, `OPERATIONAL_DOCTRINE.md`,
  media releases, and LinkedIn posts. These are the content artifacts the compiler processes.

**Critical finding: there is no graph database.** `ARCHITECTURE.md` §2 Ring 3a describes
`LadybugDB` (the successor to KuzuDB) as the long-term semantic memory. The code that exists
today uses flat JSON files and CSV keyword-matching. The graph is a forward declaration.

**Second critical finding: service-content does not route through the Doorman.** The main.rs
hardcodes `http://127.0.0.1:8082/api/semantic-extract` — a pre-Doorman SLM endpoint that
may no longer exist in the current deployment. The operator's goal "start looping in
service-content on ALL the interaction with service-SLM and the Yo-Yo" requires
service-content to be refactored to route all LLM calls through `http://127.0.0.1:9080`
(the Doorman).

**Third finding: KuzuDB is abandoned** [external: https://www.theregister.com/2025/10/14/kuzudb_abandoned/].
KuzuDB was acquired by Apple in October 2025 and the open-source project archived.
`ARCHITECTURE.md` §5.4 lists `kuzu` as the graph client crate. This is a dependency defect
to surface to Master. LadybugDB [external: https://ladybugdb.com/] is the explicit successor
(MIT license, Cypher API, Rust SDK, embedded columnar), but it is early-stage. The
`github.com/russellromney/graphd` project provides a Kuzu/LadybugDB Rust server. This
dependency situation requires operator awareness before investing in graph database integration.

### §4.2 — What datagraph(s) service-content intends to be

From `ARCHITECTURE.md` Ring 3a, `service-content`'s intended purpose is to be:

1. **The long-term semantic memory of the Totebox Archive** — a property graph where nodes
   are entities (people, companies, projects, documents, financial accounts) and edges are
   relationships (works-for, owns, governs, references, derived-from).
2. **The source of grounding context for service-slm** — every LLM call assembles context
   from a graph traversal, not from flat text retrieval. This is the GraphRAG pattern applied
   to Totebox data.
3. **A multi-tenant graph** — the `moduleId` field scopes traversals to the right tenant
   partition. Woodfine's property graph is isolated from PointSav's internal graph.

The current flat-JSON implementation is a valid bootstrap. The seeds (`Archetypes.json`,
`ChartOfAccounts.json`, `Domains.json`) represent the ontological schema in miniature —
the graph schema that would need to be formalized into Cypher `CREATE CONSTRAINT` / `CREATE
INDEX` declarations once a graph database is selected.

### §4.3 — Cross-industry knowledge graph patterns

Four patterns from industry research:

**Pattern 1 — GraphRAG (Microsoft Research, 2024)**: build a knowledge graph from unstructured
text using LLMs, apply community detection (Leiden algorithm) to cluster entities into thematic
groups, generate community summaries, use summaries for corpus-level QA
[external: https://pub.towardsai.net/graphrag-explained-building-knowledge-grounded-llm-systems-with-neo4j-and-langchain-017a1820763e].
30-40% reduction in hallucinations versus flat RAG. The Totebox equivalent: each document
ingested by service-extraction becomes graph nodes/edges in service-content, grounding all
future service-slm responses.

**Pattern 2 — Glean / Perplexity "workspace context"**: enterprise knowledge graph built from
connected data sources (email, documents, tickets); every LLM answer cites specific graph nodes
as sources. The Totebox analog: service-slm answers cite `entity_id` from the LadybugDB graph,
making answers auditable.

**Pattern 3 — ServiceNow CMDB as single-source-of-truth**: before any configuration item (CI)
is used in an IT decision, it must exist in the CMDB. The CMDB is the gatekeeper; every tool
that needs asset data reads the CMDB
[external: https://www.suretysystems.com/insights/servicenow-configuration-management-database-surety-systems/].
The Totebox analog: service-content graph is the CMDB for the Totebox Archive. Before service-slm
answers a question about the archive, it queries the graph. The graph is the authoritative source.

**Pattern 4 — IT knowledge bases (Zendesk, ServiceNow Knowledge)**: structured knowledge
articles tagged to entity categories, surfaced by the AI based on the user's query context.
The content-compiler already produces a `verified-ledger/` of Markdown files — these are
the knowledge articles. The missing layer is the vector index + graph hybrid that serves them
to service-slm at query time.

### §4.4 — service-content ↔ service-slm integration architecture

The integration has two directions:

**Direction A — service-content as context provider for service-slm (read path):**

Before routing to Tier A/B/C, the Doorman queries service-content for relevant graph context:

```
ComputeRequest arrives at Doorman
   ↓
GraphContextAssembler::assemble(module_id, query_text, max_nodes=20)
   → POST http://127.0.0.1:9100/v1/graph/context
     { module_id, query_embedding OR keyword_terms, max_depth: 2, max_nodes: 20 }
   ← { nodes: [...], edges: [...], context_summary: "..." }
   ↓
context_summary injected into system prompt: "Known context: {summary}"
   ↓
Doorman routes to Tier A/B/C with enriched prompt
```

This turns every inference call into a grounded response. The graph prevents hallucination
about entities the system knows about (customers, accounts, projects).

**Direction B — service-slm as graph builder for service-content (write path):**

After an extraction or editorial run, service-slm's output flows into service-content:

```
Doorman rehydrate() called with LLM response
   ↓
GraphDeltaExtractor parses structured response (JSON Schema validated)
   ↓
POST http://127.0.0.1:9100/v1/graph/mutate
  { module_id, operations: [{ upsert_node | upsert_edge | delete_node }] }
   ↓
service-content applies mutations to graph
   ↓
AuditCapture event "graph-mutation" sent to Doorman for ledger
```

This is the "datagraph develops hand-in-hand with training" thesis operationalized: every
inference result that is accepted feeds back into the graph, improving the grounding for the
next inference. The graph and the training data grow together.

### §4.5 — Per-tenant graph isolation

The `moduleId` field on every node in the graph (per ARCHITECTURE.md §4) is the isolation
mechanism. The service-content graph HTTP API enforces `moduleId` on all read and write
operations — no cross-tenant traversal is possible without explicitly crossing moduleId
boundaries.

For Woodfine: `moduleId = woodfine` partitions the Woodfine entity graph. Their Chart of
Accounts (real estate; leasing; compliance; construction) and Archetypes (The Executive, The
Guardian, etc.) are stored as Woodfine-scoped nodes. PointSav's internal graph uses
`moduleId = pointsav`. A single LadybugDB (or SQLite-backed graph) instance serves both,
with all queries scoped by moduleId. This matches the multi-tenant vector database patterns
used in Glean and Perplexity workspace.

### §4.6 — GraphRAG / knowledge-graph-grounded generation

The proposed integration implements a lightweight GraphRAG pattern:

1. **Ingest**: service-extraction parses documents → service-content stores entities/edges
2. **Query-time assembly**: Doorman asks service-content for a 2-hop subgraph around query
   terms → assembles a context string
3. **Generate**: OLMo 32B generates a response grounded in the subgraph context
4. **Verify + Cite**: the structured output from OLMo names entity IDs from the graph →
   Doorman verifies the entity IDs exist → response includes `citations: [entity_id, ...]`
5. **Rehydrate**: Doorman writes confirmed graph updates back to service-content

This pattern reduces hallucination (entities referenced must exist in the graph), produces
auditable answers (cited entity IDs are traceable), and builds the knowledge base over time.
The research literature reports 30-40% hallucination reduction for GraphRAG vs flat RAG
[external: https://pub.towardsai.net/graphrag-explained-building-knowledge-grounded-llm-systems-with-neo4j-and-langchain-017a1820763e].

### §4.7 — The "datagraph develops hand-in-hand with training" thesis

The operator's framing — "one, or multiple datagraphs that we are also developing
hand-in-hand with the training" — names something structurally important: the graph and the
adapter are not independent artefacts. They co-evolve.

**How they co-evolve:**

- New entities discovered by service-slm during inference go into the graph (Direction B above)
- New graph nodes provide better context for the next inference (Direction A above)
- Training tuples capture (graph-context, query, response) triples — not just (query, response)
- Adapters trained on graph-grounded tuples learn to generate graph-coherent responses
- A graph-incoherent response (names an entity not in the graph) becomes a negative DPO example

This is the **Knowledge-Graph-Grounded Apprenticeship** pattern — proposed as Doctrine claim #44
in §7.

### §4.8 — service-content as the canonical knowledge surface

The structural claim the operator intends: service-content is the single authoritative knowledge
surface for the Totebox Archive. service-slm queries it on every substantive request. Just as:

- ServiceNow CMDB is the single source of truth for IT assets
- Splunk Universal Forwarder is the only path to log aggregation
- Istio/Linkerd sidecar is the only path to service-to-service calls in a service mesh

service-content graph is the only path to structured Totebox knowledge for any LLM.

This means:
1. `service-extraction` → `service-content` (ingest path)
2. `service-slm Doorman` → `service-content` at context-assembly time (query path)
3. No LLM in service-slm should answer questions about archive entities without first
   querying the graph

The mechanism already exists structurally (ARCHITECTURE.md Ring 3a defines service-content
as "read-only from here"). What is missing is the actual HTTP client in the Doorman that calls
service-content before routing to inference.

### §4.9 — Wire format: how service-slm hits service-content

Given the existing stack: HTTP, same as all other Ring 2 ↔ Ring 3 communication.

**Proposed service-content HTTP API** (to be specified in a future contract document):

```
POST http://127.0.0.1:9100/v1/graph/context
{
  "module_id": "woodfine",
  "query_terms": ["Qualified Investment", "Flow-Through Taxation"],
  "max_depth": 2,
  "max_nodes": 20
}
→
{
  "nodes": [{ "id": "...", "label": "...", "properties": {...} }],
  "edges": [{ "from": "...", "to": "...", "relationship": "..." }],
  "context_summary": "Woodfine structures Qualified Investments via..."
}

POST http://127.0.0.1:9100/v1/graph/mutate
{
  "module_id": "woodfine",
  "operations": [
    { "op": "upsert_node", "id": "ARTHUR_PENDELTON", "type": "Person", "properties": {...} },
    { "op": "upsert_edge", "from": "ARTHUR_PENDELTON", "to": "COMP_001", "relationship": "works-for" }
  ]
}
→ { "applied": 2, "errors": [] }
```

Port 9100 is already used by `local-fs.service`. Service-content should bind a different port
(e.g., 9101 or configurable via `SERVICE_CONTENT_ENDPOINT` env var). The Doorman adds
`SLM_GRAPH_ENDPOINT` env var (or falls back to unset, disabling graph context assembly for
backward compatibility).

---

## §5 — Routing ALL traffic through service-slm

### §5.1 — Current state: what fraction of MASTER/ROOT/TASK calls actually hit service-slm

**Honest assessment**: effectively zero in current practice.

The Brief Queue Substrate (iter-22/23) wires the `capture-edit.py` post-commit hook to
submit shadow briefs to the queue. This means every commit in a tracked cluster generates
a shadow brief that eventually routes through the Doorman. However:

- Shadow briefs go through the queue asynchronously, not synchronously during the session
- The session itself (Claude as Master/Root/Task) generates all in-session LLM output directly
  via the Claude API (not through the Doorman)
- The Doorman has no in-session routing role today — it is a post-commit corpus producer

**The operator's goal** is to change this: service-slm should be the first responder on
code-shaped + IT-support-shaped requests DURING the session, not only post-commit.

This is a larger architectural change than corpus production. It requires:
1. A convention change (§5.2, §5.3) stating explicitly which request types route to service-slm
2. A quality-gate mechanism (§5.6) for service-slm to decline gracefully when unable
3. A bypass-prevention mechanism (§5.4) to ensure no session goes around the Doorman

### §5.2 — Convention proposal: service-slm as first responder

Proposed convention text for Master ratification (§9 item 2):

> **service-slm first-responder rule**: For any code-shaped or IT-support-shaped request
> during a cluster Task session, the session MUST submit the request as an apprenticeship
> brief to the Doorman (`POST /v1/brief`) BEFORE generating output via Claude. If the
> Doorman's apprentice response has `self_confidence >= 0.5` and `escalate = false`, the
> session uses the apprentice response as its output draft, reviews it, and either accepts
> or submits a verdict to the Doorman (`POST /v1/verdict`). If `escalate = true` or the
> session cannot accept the draft, the session proceeds with Claude as normal, capturing
> the Claude output as a shadow brief afterward.
>
> Exceptions: architectural decisions, doctrine drafting, cross-layer coordination (per
> CLAUDE.md §11 ≥80% confidence gate).

This is the operational form of Doctrine claim #32 (Apprenticeship Substrate) applied to
ALL clusters, not just project-slm.

### §5.3 — How Master should re-instruct Tasks/Roots

The instruction mechanism is cluster `CLAUDE.md` files. Master action items (§9):

1. Add a "service-slm first-responder rule" section to the workspace `CLAUDE.md` §6
   (Rules of Engagement). This makes it visible to every session.
2. Add a note to each cluster's `CLAUDE.md` `NEXT.md` Right-now section: "Check whether
   service-slm can answer this before calling Claude."
3. Update `bin/commit-as-next.sh` to fire a shadow brief for every commit even in clusters
   not currently wired — the hook should call `POST /v1/shadow` on the Doorman regardless
   of whether the commit was code or documentation.

The broadcast mechanism for cross-cluster convention changes is the inbox broadcast pattern
(Master writes to each cluster's `.claude/inbox.md`). The Tetrad upgrade (iter-17) used this
pattern successfully — the same broadcast should carry the service-slm first-responder rule
to all 9 active clusters.

### §5.4 — How to prevent bypass: structural and cultural

**Structural prevention** (already partly in place):
- The Yo-Yo bearer token lives ONLY in `local-doorman.service` env vars, loaded from
  `/etc/slm-doorman/doorman.env`. No cluster session has direct access to it.
- The Yo-Yo firewall rule (per Master v0.1.85 18:50Z) restricts Yo-Yo access to the
  workspace VM's internal IP — external bypass is structurally impossible.
- The only path to Tier B/C from a Task session is `POST /v1/chat/completions` to the
  Doorman. There is no SDK, no environment variable, no config file that gives a Task session
  direct Yo-Yo access.

**Cultural prevention** (convention-level):
- "service-slm IS the Yo-Yo gateway" convention text (proposed claim #43, §6): naming it
  explicitly makes the bypass visible as a violation, not just a convenience shortcut
- Quality-gate mechanism (§5.6): if service-slm can gracefully escalate, there is no
  practical reason to bypass it

**Anti-patterns to prevent**:
- A Task session calling `http://127.0.0.1:8080/v1/chat/completions` (Tier A llama-server)
  directly, bypassing the Doorman and its audit ledger. Structural prevention: the llama-server
  should be firewalled to accept connections ONLY from the Doorman process (iptables or
  systemd socket activation with AF_UNIX socket).
- A session loading Tier C provider API keys from environment. Prevention: keys live ONLY
  in the Doorman's env file; cluster sessions have no access to the provider key env vars.

### §5.5 — Fallback discipline

Service-slm is not Claude. It will be wrong in ways Claude is not. The fallback must be clean:

| Condition | Action |
|---|---|
| `escalate = true` in apprentice response | Use Claude; shadow-capture the Claude output |
| `self_confidence < 0.5` | Treat as implicit escalation |
| Doorman returns 503 (Tier B unavailable, Tier A cold) | Fall through to Claude directly |
| Brief cache miss (Doorman 410) | Re-submit brief once; if second miss, fall through to Claude |
| `task_type` not registered in promotion ledger | No apprentice involvement; proceed with Claude |

Fallback events should be captured as `AuditCapture` events (`event_type: "escalation"`) so
the rate can be tracked. If the escalation rate drops below 20% for a task-type over 30 days,
that task-type is ready for `autonomous` promotion.

### §5.6 — Quality gate: how service-slm declines and yields to Claude

The `escalate` field in `ApprenticeshipAttempt` is already the quality gate:
- `self_confidence < 0.5` → `escalate = true`
- Session receives the attempt, sees `escalate = true`, proceeds with Claude
- Claude's output is submitted as `POST /v1/shadow` for corpus capture

The quality gate improves over training cycles: as the adapter improves, `self_confidence`
distributions shift upward and the escalation rate drops. The compounding effect is measurable
as a KPI (escalation-rate-per-task-type over rolling 30 days).

### §5.7 — How this composes with PS.5 graduated task types

PS.5 (NEXT.md) is "AS-6/AS-7 P1 production routing on version-bump-manifest task type." This
is the first task-type to reach `autonomous` stage — meaning service-slm handles it without
senior review. The routing-all-through-service-slm convention (§5.2) prepares all clusters
for PS.5 graduation: when version-bump-manifest goes `autonomous`, every cluster is already
wired to submit briefs to the Doorman first, so the routing change is a threshold update
in the promotion ledger, not a session behavior change.

---

## §6 — Convention proposal: service-slm IS the Yo-Yo gateway

### §6.1 — The assertion

The operator's framing is precise and should be the ratified convention text:

> **service-slm IS the Yo-Yo gateway.** Any reference to service-slm in any Foundry
> document, session, or code comment refers equally to the Yo-Yo node. The Yo-Yo has
> no separate identity as a computing resource to Foundry's operational layer; it is
> the Tier B implementation of the Doorman protocol. The ONLY path to the Yo-Yo — from
> any session, any cluster, any deployment — MUST be through the service-slm Doorman.
> Direct access to the Yo-Yo inference endpoint from outside the Doorman is a security
> and audit violation, not a convenience.

### §6.2 — Why this matters

**Auditing**: if the Yo-Yo can be called directly, requests go unlogged. The WORM ledger
and the audit substrate (iter-15/16) assume ALL inference calls pass through the Doorman.
A single direct Yo-Yo call is an audit gap in a BCSC-sensitive environment.

**Corpus capture**: shadow briefs and apprenticeship tuples are produced at the Doorman
boundary. Direct Yo-Yo calls produce no training signal. Every direct call is a lost
training tuple.

**Cost control**: the budget cap (`monthly_cap_usd` in `infrastructure/slm-yoyo/tofu/`)
and the kill-switch Cloud Function (PS.1-5) operate at the Doorman level. Direct Yo-Yo
calls bypass the budget cap.

**Sovereignty**: the bearer token for the Yo-Yo is the most sensitive operational secret
in the deployment. Keeping it exclusively in the Doorman's env file is the minimal surface.
The moment a second process holds the bearer token, the secret surface doubles.

### §6.3 — Cross-industry analogs

Three structural analogs support the "single gateway" pattern:

**1. ServiceNow CMDB** [external: https://www.suretysystems.com/insights/servicenow-configuration-management-database-surety-systems/]:
All IT asset data must pass through the CMDB reconciliation engine before use. No tool
reads infrastructure state directly from servers; it reads from the CMDB. The CMDB is the
only path to authoritative IT asset state — just as service-slm Doorman is the only path
to authoritative inference compute.

**2. Splunk Universal Forwarder**: all log data flows through the Universal Forwarder to
the Splunk indexer. A server that writes logs directly to the indexer bypasses retention,
masking, and alerting rules. The Doorman is the Universal Forwarder for LLM requests: all
inference traffic flows through it so the audit ledger is complete.

**3. Kubernetes service mesh (Istio/Linkerd)**: ALL service-to-service traffic routes through
sidecar proxies. The sidecar enforces mTLS, rate limits, and observability. A service that
bypasses the sidecar gets traffic through but loses encryption, tracing, and policy
enforcement. The Doorman is the LLM service mesh: enforce it universally or the audit surface
has holes.

### §6.4 — Proposed Doctrine claim #43

**Claim title**: "Single-Boundary Compute Discipline"

**Claim text (proposed)**:
> The Doorman (service-slm) is the single boundary point for all AI inference compute in
> every Foundry deployment. No process, session, or service accesses an inference tier
> (local, Yo-Yo, or external API) except through the Doorman. The Yo-Yo node, the local
> llama-server, and external API endpoints are implementation details of the Doorman's
> routing logic; they have no operational identity outside the Doorman. Bearer tokens,
> API keys, and compute endpoint URLs for all inference tiers live exclusively in the
> Doorman's configuration and are never distributed to callers. This discipline enables:
> complete audit coverage (no shadow inference paths); mandatory corpus capture (all
> inference generates training signal); unified cost control (one budget cap covers all
> tiers); minimal secret surface (one process holds all inference credentials).

**What makes this "special"** (per operator framing): the industry pattern for LLM gateway
is to route for load balancing (LiteLLM, Portkey, Helicone). Foundry's claim is stronger:
the gateway is the ONLY path, not a preferred path. The enforcement is structural (bearer
token inaccessible outside Doorman), not policy-only. This makes the audit guarantee
cryptographic rather than procedural.

### §6.5 — Enforcement mechanisms

Current enforcement (in place):
- Bearer token in `doorman.env` — no cluster session has access
- Firewall: Yo-Yo accepts connections from workspace VM internal IP only (per Master v0.1.85)
- llama-server bound to 127.0.0.1:8080 — only Doorman can reach it from localhost

Proposed additional enforcement:
- `iptables` rule on Tier A: allow port 8080 only from the Doorman process UID
  (via `--uid-owner` iptables extension for `OUTPUT` chain filtering)
- Doorman health check: verify `SLM_YOYO_BEARER` is present and non-empty at startup;
  refuse to start without it — prevents Yo-Yo bypass by misconfiguration
- Convention text in every cluster's `CLAUDE.md`: "service-slm IS the Yo-Yo gateway —
  never call inference endpoints directly"

---

## §7 — Inventions / Doctrine claim candidates

This section proposes three new Doctrine claims for Master ratification.

### Invention 1 — Proposed Claim #43: Single-Boundary Compute Discipline

**Full claim text**: as stated in §6.4 above.

**Structural argument**: the claim is not just a policy rule; it is an architectural
invariant. The bearer token structure (exclusive to Doorman) and the firewall rule
(Yo-Yo only from Doorman's VM) are non-policy enforcement mechanisms that make bypass
structurally difficult rather than merely prohibited. The claim names this structural
quality explicitly so future architecture changes are evaluated against it: "does this
change create a second path to inference outside the Doorman?" If yes, the claim is
violated and the change requires doctrine-level approval.

**What it unlocks**: complete audit coverage makes the audit ledger legally admissible
as a record of all AI inference activity. For a BCSC-sensitive deployment, "we have a
complete record of every AI inference call, with module_id, cost, and latency, because
no inference call bypasses the ledger" is a material capability. It also means the
cost-per-call data in the ledger is reliable for pricing and billing purposes.

**What makes it "special"**: existing LLM gateway products (LiteLLM, Portkey, Helicone)
are preferred-path, not exclusive-path. "Exclusive path with structural enforcement"
is unclaimed as an explicit principle in the 2026 LLM gateway literature
[external: https://docs.litellm.ai/docs/simple_proxy].

---

### Invention 2 — Proposed Claim #44: Knowledge-Graph-Grounded Apprenticeship

**Claim title**: "Knowledge-Graph-Grounded Apprenticeship"

**Claim text (proposed)**:
> service-slm consults the service-content knowledge graph before routing every substantive
> inference request to Tier A/B/C. The (query, graph-context, apprentice-response, senior-verdict)
> tuple is the atomic unit of the Apprenticeship Substrate (claim #32) when graph context
> is present. Training on graph-grounded tuples produces adapters that generate graph-coherent
> responses: references to entities that exist in the graph, relationships that are consistent
> with the graph, and structured outputs that extend the graph on write-back. The datagraph
> and the adapter co-evolve: each accepted graph mutation increases the grounding quality of
> the next inference; each training cycle produces an adapter better at querying the graph.
> The Knowledge-Graph-Grounded Apprenticeship loop is the mechanism by which Foundry's
> sovereign knowledge base and its sovereign AI capabilities compound together rather than
> in parallel.

**Structural argument**: the GraphRAG literature [external: https://arxiv.org/html/2502.13247v3]
and Microsoft's GraphRAG research establish that grounded generation reduces hallucination by
30-40%. But the published GraphRAG pattern is stateless: the graph is built once from static
documents; the inference model does not feed back into the graph. Foundry's claim is a
dynamic extension: the inference model's output updates the graph on each accepted call,
and the training loop rewards graph-coherent outputs. This creates a positive-feedback system
between knowledge accumulation and model quality that has no direct analog in published
GraphRAG work.

**What it unlocks**: measurable grounding quality over time (percentage of model responses
citing known graph entities, tracked in the audit ledger). For Woodfine's property management
domain: every document processed by service-extraction adds nodes/edges to the Woodfine
partition of the service-content graph; service-slm's IT-support answers for Woodfine improve
in specificity as the graph grows. This is the commercial argument for the Tier 3 PointSav-LLM
product: "a model that knows your specific archive better than any general-purpose model."

**What makes it "special"**: the co-evolution loop (graph updates model quality, model quality
updates graph) has not been proposed as a first-class architectural principle in the 2026
literature. Existing GraphRAG implementations treat the graph as input; Foundry's claim treats
it as a co-evolving output. The audit ledger provides the substrate for tracking whether the
co-evolution is working (graph nodes cited per response, trending over rolling 30-day windows).

---

### Invention 3 — Proposed Claim #45: TUI-as-Corpus-Producer

**Claim title**: "TUI-as-Corpus-Producer"

**Claim text (proposed)**:
> Every terminal interaction with service-slm through the System Administrator TUI
> (slm-cli) is a curated training corpus contribution. Sysadmin and IT-support interactions
> are uniquely high-quality training signal because: (a) the ground truth is verifiable
> (the system either responds to an administrative action correctly or it does not); (b) the
> interaction domain is narrow (Totebox Archive operations, Foundry conventions, customer
> archive specifics); (c) the operator providing the feedback is the domain expert (no
> proxy labeler). The TUI implements a lightweight feedback mechanism (`/feedback [good|bad]`)
> that captures explicit verdicts without a formal apprenticeship brief/verdict cycle, producing
> (query, response, verdict) DPO triples suitable for IT-support adapter training. The IT-support
> adapter is expected to reach production quality with 200-500 high-quality (explicit-verdict)
> TUI interaction triples — an order of magnitude less data than code-generation tasks because
> the domain is narrow and the ground truth is unambiguous.

**Structural argument**: the research consensus (from `conventions/apprenticeship-substrate.md`
§1, citing the RLHF/DPO literature) is that interaction tuples train an order of magnitude more
efficiently than observation tuples per tuple. The TUI provides the highest-density interaction
tuples in the system: the operator is the domain expert, the feedback is immediate, and the
task domain (Totebox sysadmin) is narrow enough that few tuples produce a high-quality adapter.
This is the application of the "quality over volume" principle from
[external: https://effloow.com/articles/llm-fine-tuning-lora-qlora-guide-2026] to the specific
IT-support use case.

**What it unlocks**: the TUI is the first customer-facing surface that produces training data
while delivering value. For Woodfine as the first customer: every time a Woodfine employee
asks service-slm an IT question and marks the response as good, they are contributing to the
`tenant-woodfine` IT-support adapter — making the next response better. This closes the
customer-value ↔ model-quality loop in a form that customers can understand and participate in.

**What makes it "special"**: existing AI sysadmin products (Datadog AI, PagerDuty AI) do not
make the corpus-contribution loop visible or controllable to the operator. The operator is
a passive beneficiary. Foundry's claim is that the operator is an active corpus contributor
whose feedback directly improves their specific deployment's model. This is the
"customer-owned substrate" principle (claim #28 Designed-for-Breakout) applied to AI
training data: the customer's feedback is the customer's property.

---

### Potential Invention 4 — service-content as Temporal Knowledge Graph

This is an exploratory proposal, not a formal claim candidate without further research.

The service-content graph accumulates changes over time. Every graph mutation is timestamped
in the audit ledger. This creates a temporal knowledge graph: not just "what is the current
state of the archive" but "what was the state at time T." For Woodfine's property management
use case: "who owned unit 4B in Building 3 in Q3 2025?" is a temporal query that flat RAG
cannot answer but a temporal graph can.

The WORM ledger (Doctrine Invention #7) is already designed for this: append-only with
timestamps. If service-content mutations are also append-only (insert new node versions,
never overwrite old ones), the graph becomes a temporal record. This composes with the
BCSC continuous-disclosure posture: the state of the knowledge graph at any past point
in time is recoverable and auditable.

This would be a significant design constraint on service-content implementation (no
in-place graph node updates; all mutations are versioned inserts). The claim is
exploratory — whether it is worth the implementation cost is an operator decision.

---

## §8 — Phased implementation roadmap

### Phase 0 — Immediate (now, cluster-Task scope, 0 days)

**What ships**: this document. An outbox message to Master summarizing the §9 proposals.

**Who**: project-slm Task (current session; this iter-24 dispatch).

**Dependencies**: none.

**Effort**: iter-24 sub-agent session (~4-5 hours).

---

### Phase 1 — Proof-of-life CLI + service-content Doorman refactor (1-2 weeks, Task)

**What ships**:
1. `scripts/slm-chat.sh` — a 50-line bash REPL that submits messages to the Doorman via curl
   and streams responses. No ratatui dependency. Validates the Doorman-as-TUI-backend pattern.
2. service-content `src/main.rs` refactored to route SLM calls through
   `http://127.0.0.1:9080/v1/chat/completions` (the Doorman) instead of the legacy port 8082
   endpoint. This closes the "service-content does not route through the Doorman" finding.

**Who**: project-slm Task (Sonnet sub-agent, 2-3 days each).

**Dependencies**: Doorman running (already live).

**Effort**: ~3-4 days Task.

---

### Phase 2 — service-content graph foundation (2-4 weeks, Task + Master)

**What ships**:
1. A decision on LadybugDB vs SQLite-backed graph for the short term (Master tier — requires
   operator awareness of the KuzuDB abandonment finding before committing to LadybugDB).
2. service-content HTTP server exposing `/v1/graph/context` and `/v1/graph/mutate` endpoints.
3. Doorman gains `GraphContextAssembler` — queries service-content before routing to Tier B.
4. Graph seeds loaded: Woodfine's `ChartOfAccounts.json`, `Archetypes.json`, `Domains.json`
   imported as initial graph nodes.

**Who**: Task (service-content HTTP server + Doorman integration); Master (LadybugDB decision).

**Dependencies**: LadybugDB or SQLite-graph decision. Port assignment for service-content
(e.g., `SERVICE_CONTENT_ENDPOINT=http://127.0.0.1:9101`).

**Effort**: ~2 weeks Task for the implementation; 1 day Master for the decision.

---

### Phase 3 — Routing convention enforcement + first LoRA training cycle (2-3 weeks)

**What ships**:
1. Master outbox broadcast to all 9 active clusters with the service-slm first-responder
   convention (§5.2) — clusters amend their `CLAUDE.md` to include the rule.
2. First LoRA training cycle on the Yo-Yo: `engineering-pointsav` adapter (355 corpus files),
   trained on L4, validated, signed, stored at `data/adapters/`.
3. Doorman adapter loader (`slm-doorman/src/adapter_loader.rs`) — reads adapter manifest,
   verifies signatures, registers adapters. Follow-on to `trainer-scoping.md` §7.

**Who**: Master (convention broadcast); Task (Doorman adapter loader); operator (training run
approval, adapter signing).

**Dependencies**: Python training script (`train_adapter.py`) from `trainer-scoping.md`
Phase 0 completed first.

**Effort**: Master: ~1 hour. Task: ~5 days. Training run: ~$1-2 on Yo-Yo.

---

### Phase 4 — TUI System Administrator (3-4 weeks, Task)

**What ships**:
1. `slm-cli` crate under `service-slm/crates/slm-cli/` — ratatui chat TUI with streaming.
2. Slash commands: `/status`, `/audit`, `/graph`, `/feedback`.
3. TUI shadow brief submission after each interaction (corpus production from §3.5).
4. Deployed to Woodfine dogfood Totebox as `slm-cli` binary alongside `local-doorman.service`.

**Who**: Task.

**Dependencies**: Phase 1 (Doorman validated as TUI backend); Phase 2 (graph context for
grounded responses).

**Effort**: ~3 weeks Task.

---

### Phase 5 — Full IT-support corpus loop (4-6 weeks, Task + operator)

**What ships**:
1. 200-500 IT-support TUI interactions captured with explicit `/feedback` verdicts.
2. `it-support-woodfine` LoRA adapter trained on those interactions.
3. Adapter deployed to Woodfine Totebox Doorman; IT-support TUI answers from the adapter
   for known task types.
4. Escalation rate tracked: target <30% escalations for "service-fs status" and "audit
   query" task types within 2 months.

**Who**: operator + Woodfine users (corpus production); Task (adapter training + deployment).

**Dependencies**: Phase 4 (TUI deployed); Woodfine Totebox provisioned.

**Effort**: operator usage (daily), Task for training cycles (~weekly).

---

### Phase 6 — PointSav-LLM v0 (Year 2 target)

**What ships**:
1. Continued pretraining of OLMo 3 32B base on Foundry's full aggregated corpus
   (engineering + apprenticeship + IT-support interaction tuples) — per `four-tier-slm-substrate.md` §5.
2. PointSav-OLMo-N first cut: the Tier 3 commercial product with no shipping competitor.
3. Federated LoRA marketplace (claim #14) — customers contribute adapters, Foundry aggregates.

**Who**: operator (compute procurement decision); Master (training infrastructure).

**Dependencies**: ~100B domain-specific tokens accumulated; H100 cluster access (or RunPod
burst at ~$1.49/hr spot × 1000 hours = ~$1,500 for one CPT cycle on a small H100 pod).

**Effort**: Year 2 timeline; operator capital decision required.

---

## §9 — Master-instruction proposals

These are concrete proposals for Master to ratify at the next operator-presence pass.

### Proposal 1 — Ratify Doctrine claims #43, #44, #45

See §7 for full claim texts. Suggested ratification format:

Claim #43: "Single-Boundary Compute Discipline" — add to `DOCTRINE.md` + create
`conventions/single-boundary-compute-discipline.md`

Claim #44: "Knowledge-Graph-Grounded Apprenticeship" — extend `conventions/apprenticeship-substrate.md`
with a §8 section on graph-grounded tuples; separately create
`conventions/knowledge-graph-grounded-apprenticeship.md`

Claim #45: "TUI-as-Corpus-Producer" — add to `conventions/apprenticeship-substrate.md` §7D
(following §7C Brief Queue Substrate); separately create `conventions/tui-corpus-producer.md`

### Proposal 2 — service-slm IS the Yo-Yo gateway convention

Add to workspace `CLAUDE.md` §6 (Rules of Engagement):

> **service-slm IS the Yo-Yo gateway.** Any reference to service-slm in Foundry documents
> and sessions refers equally to the Yo-Yo node. The ONLY path to the Yo-Yo from any session
> or deployment is through the service-slm Doorman. Direct access to inference tier endpoints
> (local llama-server, Yo-Yo vLLM, external API) from outside the Doorman is an audit and
> cost-control violation. Bearer tokens and API keys for all inference tiers live exclusively
> in the Doorman's configuration. Convention source: claim #43. First articulated: iter-24,
> 2026-04-29.

Add an identical note to each cluster's `CLAUDE.md` as a broadcast (Master inbox action).

### Proposal 3 — service-slm first-responder convention

Add to workspace `CLAUDE.md` §6 (Rules of Engagement):

> **service-slm first-responder rule.** For code-shaped or IT-support-shaped requests
> during a cluster Task session, submit as an apprenticeship brief to the Doorman BEFORE
> generating output via Claude. Accept the apprentice response if `self_confidence >= 0.5`
> and `escalate = false`; submit verdict. Proceed with Claude if `escalate = true` or the
> draft is unacceptable; shadow-capture the Claude output. Exceptions: architectural
> decisions, doctrine drafting, cross-layer coordination (CLAUDE.md §11 ≥80% confidence
> gate). Convention source: Apprenticeship Substrate claim #32 applied workspace-wide.

### Proposal 4 — service-content scope ratification: Tetrad wiki leg

Formally absorb service-content into the project-slm cluster's Tetrad manifest
(`clones/project-slm/.claude/manifest.md`). The integration design in §4 generates three
TOPIC drafts for the wiki leg:

- `topic-service-content-knowledge-graph.md` — what the datagraph is and how it co-evolves
  with training
- `topic-single-boundary-compute-discipline.md` — the service-slm-IS-the-Yo-Yo convention
- `topic-tui-system-administrator.md` — the slm-cli design and corpus production pattern

Stage in `.claude/drafts-outbound/` per §11 convention after operator green-light.

### Proposal 5 — KuzuDB abandonment surface to Master

The `kuzu` crate in `service-slm/ARCHITECTURE.md` §5.4 depends on a project abandoned
in October 2025 (acquired by Apple). LadybugDB is the Cypher-compatible successor (MIT,
Rust SDK) but is early-stage and community-backed only. Before investing implementation
effort in graph database integration (Phase 2), the operator should:

1. Confirm whether `kuzu` crate (lib.rs) still receives security patches from the
   community fork or LadybugDB
2. Evaluate LadybugDB (ladybugdb.com) for maturity against the Phase 2 requirements
3. Consider SQLite + recursive CTEs as a fallback for the Phase 2 graph if LadybugDB
   is not yet production-ready

This is a dependency risk that should be explicitly accepted or mitigated before Phase 2 starts.

### Proposal 6 — Yo-Yo training cadence ratification

Based on §2.7 analysis, the proposed training cadence for Master ratification:

- **Trigger**: threshold-triggered (50 new tuples in any adapter's corpus bucket)
- **Off-peak schedule**: Sunday 02:00 UTC as secondary trigger
- **First run**: `engineering-pointsav` adapter (355 files, all clusters); ~4-8 hours on L4;
  ~$1-2 cost
- **Quality gate**: promotion only when validation acceptance-rate ≥ 0.60 per
  `trainer-scoping.md` §6 Step 7

---

## §10 — Open questions for operator decision

Each question is answerable in a sentence and shapes Phase 1-3 implementation.

1. **service-content route-through-Doorman**: Confirm that port 8082 (the legacy SLM endpoint
   hardcoded in service-content `src/main.rs`) no longer exists in the current deployment,
   and confirm the refactor to port 9080 (Doorman) is authorized.

2. **LadybugDB vs SQLite-graph for Phase 2**: Is the operator comfortable proceeding with
   LadybugDB (early-stage, MIT, Cypher API) as the graph store given the KuzuDB abandonment
   finding, or should Phase 2 use SQLite + recursive CTEs as an interim graph until LadybugDB
   matures?

3. **Training timeline**: Is the first LoRA training cycle (Phase 3, ~$1-2, 4-8 hours Yo-Yo)
   authorized to proceed once the Python training script is written?

4. **service-content multi-service integration**: service-content's `main.rs` also processes
   data from `service-people/ledgers/`. Should the Phase 1 refactor scope include
   service-people integration, or is service-content-to-Doorman routing sufficient for Phase 1?

5. **Temporal graph**: Is the "temporal knowledge graph" exploratory proposal (§7 Invention 4)
   worth pursuing in Phase 2, or is the simpler mutable graph (overwrite nodes in place)
   acceptable for the first deployment?

6. **TUI deployment target**: Should `slm-cli` Phase 1a (proof-of-life `slm-chat.sh`) be
   deployed to the workspace VM (operator self-test) or to a Woodfine staging Totebox?

7. **IT-support adapter scope**: Should the IT-support LoRA adapter be Woodfine-specific
   (`tenant-woodfine` moduleId) or generic Totebox operations (`engineering-pointsav`)?
   The answer determines whether Phase 5 requires the Woodfine Totebox to be deployed first.

8. **Yo-Yo idle-shutdown timer**: The current manual Yo-Yo (`yoyo-tier-b-1`) runs without
   idle-shutdown (Step 8 in the manual runbook is marked "optional follow-up"). Should idle-
   shutdown be added now (stops cost bleed at ~$130/mo always-on) or after the first training
   run validates the infrastructure?

9. **Claims #43/#44/#45 naming**: The proposed claim names are working titles. Operator
   preference on naming? "Single-Boundary Compute Discipline", "Knowledge-Graph-Grounded
   Apprenticeship", and "TUI-as-Corpus-Producer" are precise but compound. Shorter names
   (e.g., "Gateway Discipline", "Grounded Apprenticeship", "Operator Corpus") may be
   preferable for DOCTRINE.md.

10. **OLMo 3 32B-Think availability**: The yoyo-manual README notes AllenAI has not yet
    published OLMo 3 32B-Think (only 7B has the Think variant). When this is published,
    should the Yo-Yo be upgraded to the Think variant? This is the inference quality jump;
    the Think variant benchmarks higher on reasoning. The upgrade requires re-quantizing on
    the Yo-Yo VM — a manual operator action.

11. **Broadcast timing**: The service-slm first-responder rule (§5.2) and service-slm-IS-
    the-Yo-Yo-gateway convention (§6.1) should be broadcast to all 9 active cluster inboxes.
    Should this broadcast happen before or after the Doctrine claims are formally ratified?

12. **service-content Cargo.toml**: service-content is currently NOT in the monorepo
    workspace Cargo.toml and uses `reqwest = "0.11"` (outdated; current stable is 0.12+).
    Does the refactor in Phase 1 also include updating the Cargo.toml to reqwest 0.12+ and
    aligning with the monorepo workspace?

13. **Inventory decision: Woodfine-specific ontology**: The service-content seeds
    (Archetypes, ChartOfAccounts, Domains, Themes) are Woodfine-specific commercial real
    estate vocabulary. Is this the right schema for the PointSav-side graph as well, or
    does PointSav need a separate ontology schema for its internal knowledge graph?

14. **The `cognitive-forge` / `content-compiler` wire-format reconciliation**: The cleanup-log
    records that `cognitive-forge` (writes `.md`) and `content-compiler` (reads `.json`) are
    inconsistent. Phase 1 service-content refactor is an opportunity to close this defect.
    Confirm scope inclusion?

---

## §11 — Research trail

### Done — what informed this document

- `service-slm/ARCHITECTURE.md` — full read; three-ring model, moduleId discipline, tier
  routing, apprenticeship substrate, audit ledger, LadybugDB reference in §5.4
- `service-slm/docs/trainer-scoping.md` (iter-20) — full read; corpus state, adapter types,
  deployment options, corpus-to-adapter pipeline; this document extends rather than duplicates
- `service-slm/DEVELOPMENT.md` — skim; phase roadmap, planned crates
- `service-slm/docs/audit-endpoints-contract.md` v0.2.0 — skim; wire contract for audit proxy
- `service-slm/crates/slm-doorman-server/src/queue.rs` — head section read; file-backed
  durable brief queue; 7 briefs in queue dir at dispatch time
- `service-content/` — full end-to-end read:
  - `src/main.rs` (file watcher → legacy SLM port 8082 → enriched CRM ledger)
  - `content-compiler/src/main.rs` (deterministic ontological classifier → verified-ledger)
  - `content-compiler/Cargo.toml` (standalone crate; serde_json + chrono)
  - `seeds/Archetypes.json`, `ChartOfAccounts.json`, `Domains.json` (Woodfine-specific)
  - `ontology/archetypes.csv`, `domains/domain_corporate.csv` (CSV term lists)
  - `scripts/forge-seeds.sh` (seed generation script; hardcoded deployment paths)
  - `README.md` (service description)
  - `Cargo.toml` (package; `notify`, `serde`, `reqwest 0.11`, `serde_json`)
- `conventions/four-tier-slm-substrate.md` — full read; four-tier ladder, API key boundary,
  per-tier training differentials, CPT vs LoRA thresholds, two market gaps
- `conventions/llm-substrate-decision.md` — full read; OLMo 3 choice, three compute tiers,
  Year 1-5 continued pretraining roadmap
- `conventions/apprenticeship-substrate.md` — head section read; three stages, brief/verdict
  format, promotion thresholds
- `conventions/adapter-composition.md` — head section read; composition algebra,
  multi-LoRA serving primitives
- `infrastructure/yoyo-manual/README.md` — full read; fast-path provisioning, L4 spot at
  ~$0.18/hr in us-west1, always-on ~$130/mo baseline, idle-shutdown note
- `infrastructure/slm-yoyo/CONTRACT.md` v0.2.0 — head section; L4 24 GB, OLMo 3 32B
  Q4_K_M, dual-purpose role, Brief Queue compatibility
- `data/training-corpus/` — live corpus state: 355 engineering files across 9 clusters,
  376 apprenticeship JSONL, 7 queue briefs pending
- Web research: GCE L4 spot pricing, A100/H100/L4 hyperscaler comparison, OLMo 3 32B
  inference metrics, Claude Sonnet/Haiku pricing, vLLM Multi-LoRA docs, KuzuDB abandonment
  + LadybugDB successor, ratatui chat TUI pattern, GraphRAG pattern, LiteLLM gateway,
  ServiceNow CMDB single-source-of-truth, LoRA training time on A100, effloow.com LoRA guide

### Suggested — what should be consulted next

- [external: https://ladybugdb.com/] — Priority high before Phase 2. Evaluate maturity,
  Rust SDK stability, and whether the embedded graph server meets Phase 2 requirements.
  Check for a stable release, not just a marketing page.
- [external: https://github.com/russellromney/graphd] — Priority high. This is the Rust
  graph server built on Kuzu/LadybugDB. Review for production-readiness before adopting.
- [external: https://docs.vllm.ai/en/stable/features/lora/] — Priority medium. When the
  formal vLLM Tier B via `slm-yoyo/tofu/` replaces llama.cpp, verify the hot-swap
  Multi-LoRA API before writing the Doorman adapter composition code.
- [external: https://github.com/allenai/OLMo] — Priority medium. Check when OLMo 3.1 32B
  Think is published; this is the inference quality upgrade for the Yo-Yo.
- [external: https://huggingface.co/docs/trl/sft_trainer] — Priority high before Phase 3
  training script. Verify TRL SFTTrainer compatibility with OLMo 3 tokenizer.
- [external: https://ratatui.rs/tutorials/] — Priority low (Phase 4 implementation).
  Verify ratatui 0.30+ streaming output patterns for the TUI chat panel.
- GCE L4 spot price verification — check `gcloud compute machine-types list` directly from
  the workspace VM before committing to the ~$0.18/hr figure in project budgets. Spot prices
  are dynamic.
- `conventions/zero-container-runtime.md` — Confirm that the llama.cpp fast-path (yoyo-manual)
  is compliant with the zero-container convention, or whether the manual setup needs an
  additional compliance note.
- `infrastructure/slm-yoyo/tofu/` (full read of compute.tf, iam.tf, variables.tf) — When D4
  unblocks, the formal Tofu module is the production path; understand it before Phase 3
  training cycles that assume vLLM rather than llama.cpp.
- Cost verification: run `gcloud billing budgets list --billing-account=...` on the workspace
  VM to see current Yo-Yo spend vs the $0.18/hr baseline. Do this after one week of operation.
- [external: https://arxiv.org/html/2502.13247v3] — Grounding LLM Reasoning with Knowledge
  Graphs (February 2026). Priority medium before Phase 2 graph integration design.
- BCSC review: the IT-support adapter (Phase 5) will produce model output for an operational
  customer deployment. Confirm with operator whether a BCSC review pass is required before
  deploying an AI model output to customer infrastructure.

### Open questions

- Questions 1-14 from §10 above.
- Is the KuzuDB `kuzu` Rust crate (currently in `ARCHITECTURE.md` §5.4) still receiving
  security patches from either the Kineviz bighorn fork or the LadybugDB team?
- Does the service-content `src/main.rs` watcher assume a specific deployment directory
  structure (`/home/mathew/deployments/...`) that would need to be generalized for any
  deployment path before the Phase 1 refactor?
- Should the graph context assembly in the Doorman be synchronous (adds latency to every
  Tier B request) or async (pre-assembled context in a brief cache updated on document
  ingest events)?
- At what graph size does a SQLite-backed graph become a performance bottleneck for
  2-hop traversals on a Totebox-class machine (e2-standard-4, 16 GB RAM)? This bounds
  the Phase 2 SQLite fallback option.

---

## References

Per `~/Foundry/citations.yaml` and inline citations:

- [olmo3-allenai] — OLMo 3 announcement and tech report (allenai.org/blog/olmo3)
- [federated-lora-2502-05087] — federated LoRA framework paper (arxiv.org/abs/2502.05087)
- [lorax-predibase] — LoRAX multi-adapter inference server (github.com/predibase/lorax)
- [s-lora-2024] — S-LoRA: Adapter isolation per dynamic computation (proceedings.mlsys.org)
- [ni-51-102] — BCSC continuous-disclosure obligations (bcsc.bc.ca)
- [osc-sn-51-721] — OSC forward-looking information disclosure (osc.ca)

External sources consulted (not yet in citation registry):

- [external: https://cloud.google.com/compute/gpus-pricing] — GCE GPU pricing reference
- [external: https://cloud.google.com/spot-vms/pricing] — GCE spot VM pricing
- [external: https://www.synpixcloud.com/blog/cloud-gpu-pricing-comparison-2026] — GPU cloud pricing comparison 2026
- [external: https://jarvislabs.ai/blog/a100-price] — A100 pricing across providers
- [external: https://platform.claude.com/docs/en/about-claude/pricing] — Claude API pricing April 2026
- [external: https://artificialanalysis.ai/models/olmo-3-1-32b-instruct] — OLMo 3.1 32B Instruct performance and pricing
- [external: https://artificialanalysis.ai/models/olmo-3-1-32b-think/providers] — OLMo 3.1 32B Think: no commercial API; must self-host
- [external: https://allenai.org/blog/olmo3] — OLMo 3 benchmarks and announcement
- [external: https://effloow.com/articles/llm-fine-tuning-lora-qlora-guide-2026] — LoRA/QLoRA fine-tuning guide 2026
- [external: https://docs.vllm.ai/en/stable/features/lora/] — vLLM Multi-LoRA serving documentation
- [external: https://blog.vllm.ai/2026/02/26/multi-lora.html] — vLLM multi-LoRA serving with Amazon SageMaker
- [external: https://www.theregister.com/2025/10/14/kuzudb_abandoned/] — KuzuDB abandoned, Apple acquisition
- [external: https://ladybugdb.com/] — LadybugDB successor to KuzuDB (MIT, Cypher, Rust SDK)
- [external: https://ratatui.rs/] — Ratatui Rust TUI library
- [external: https://github.com/nonscalar/tuichat] — tuichat: TUI chat in Rust with ratatui and tokio
- [external: https://pub.towardsai.net/graphrag-explained-building-knowledge-grounded-llm-systems-with-neo4j-and-langchain-017a1820763e] — GraphRAG pattern explanation
- [external: https://arxiv.org/html/2502.13247v3] — Grounding LLM Reasoning with Knowledge Graphs
- [external: https://docs.litellm.ai/docs/simple_proxy] — LiteLLM AI Gateway documentation
- [external: https://www.suretysystems.com/insights/servicenow-configuration-management-database-surety-systems/] — ServiceNow CMDB as single source of truth
- [external: https://leanopstech.com/blog/ai-cloud-cost-optimization-gpu-spending-guide-2026/] — AI cloud cost optimization 2026
- [external: https://www.finout.io/blog/anthropic-api-pricing] — Anthropic API pricing 2026

---

*Forward-looking statements in this document regarding PointSav-LLM trajectory, training
timelines, cost estimates, and commercial product positions carry "planned"/"intended"/
"proposed" language per `conventions/bcsc-disclosure-posture.md`. This document is
internal-operational and has not been reviewed for public distribution.*
