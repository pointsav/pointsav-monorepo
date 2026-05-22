# Tier Architecture Policy — 2026

> Authored: 2026-05-14 task@project-intelligence (Opus deep-think)
> Status: Active — pending operator ratification of §5 as CONVENTION-*
> Companion: `.agent/plans/universal-ai-gateway.md`

---

## 1. Three-Tier Compute — Verdict

The Tier A/B/C split is correctly scoped. `route_yoyo_only()` for extraction is
structurally right. Hard rule confirmed: **Tier A must never handle entity extraction.**

### Tier routing table (authoritative)

| Task class | Correct tier | Rationale |
|---|---|---|
| Read file / summarise | Tier A (local 7B) | Zero cost, instant, within 7B quality threshold |
| Grep / search interpretation | Tier A (local 7B) | Doesn't need reasoning |
| Tool-call args (flat schema, haiku-tier) | Tier A (local 7B) | 7B reliable; 1B is not |
| Moderate code edit / refactor | Tier B "trainer" (Yo-Yo #1) | Good enough, cheap |
| Entity extraction / DataGraph | Tier B "trainer" (Yo-Yo #1) | Proven at 74 entities; 1B cannot do this |
| Complex debugging | Tier C (Claude) | Needs real reasoning |
| Architecture decisions | Tier C (Claude) | Needs real reasoning |
| Multi-step agent chains (5+) | Tier C (Claude) | Tool-use quality gap at Tier B |
| Batch graph extraction (grammar strict) | Tier B "graph" (Yo-Yo #2) | H100 + 70B + JSON grammar |

### Failure mode fix required

**Current:** `{deferred: true}` → log → return true → file stays in `processed_ledgers` → retry on next boot.
**Problem:** Tier B can be down for hours or days (vllm 0.12 issue is the proof). Per-boot retry is not appropriate.

**Required fix:** Persistent extraction queue. service-content marks each `CORPUS_*.json` with
`state: queued | extracting | extracted | deferred` in a sidecar ledger. On defer: flip to
`deferred + deferred_at timestamp`. Retry trigger: Yo-Yo-up notification from Doorman
(POST /v1/yoyo/up) or Doorman polling `/healthz`. ~50 LOC service-content + small Doorman extension.
Eliminates the per-boot retry storm by construction.

---

## 2. Tier A Model — Recommendation

**Upgrade to: OLMo 2 1124 7B Instruct Q4_K_M. MemoryMax=6G.**

| Option | Weights (Q4) | KV @ctx 4096 | MemoryMax | VM headroom (16GB) |
|---|---|---|---|---|
| OLMo 2 0425 1B (current) | ~0.7GB | ~0.4GB | 3G | Wide |
| OLMo 2 1124 7B (target) | ~4.5GB | ~0.5GB | 6G | Tight but OK |
| OLMo 2 1124 13B | ~7.5GB | ~0.7GB | 10G | Not viable |

Why 7B beats 1B for Tier A tasks:
- Read/summarise: 1B produces watery output; 7B produces coherent paragraph-level summaries
- Tool-call JSON args (haiku-tier shim for Sprint 0a): 1B unreliable; 7B handles flat-schema reliably — this is the gating capability for the Anthropic shim
- Apprenticeship signal quality: 1B pairs too noisy to use for training

**Never use "Think" variants at Tier A.** Think variants emit reasoning traces before answering — wrong shape for latency-sensitive tasks and wasted KV cache. Think = Tier B (GPU), Instruct = Tier A (CPU).

### Drift surfaced (requires immediate reconciliation)

`local-slm.service` loads `OLMo-2-0425-1B-Instruct-Q4_K_M.gguf` (1B).
`local-doorman.service` declares `SLM_LOCAL_MODEL=Olmo-3-1125-7B-Think-Q4_K_M.gguf` (7B Think).

These disagree. The `SLM_LOCAL_MODEL` env var affects Doorman audit ledger entries and capability
routing decisions. Fix: reconcile both files to point to the same model before any further changes.

---

## 3. mistralrs-server Migration — Timing

**Defer until Sprint 1.5 (between Sprint 1 canonical IR and Sprint 2 native Anthropic).**

Trigger: first LoRA adapter is trained + validated on Yo-Yo #1. At that point, hot-swap LoRA
at runtime is a first-class requirement — mistralrs-server provides it natively; llama-server
does not. Also: mistralrs ships native `/v1/messages` (Anthropic wire format), eliminating
format conversion at the inference engine layer once canonical IR is in place.

**Does not help Sprint 0a.** The Doorman-level shim in `slm-doorman-server/src/http.rs` is
engine-agnostic — migration doesn't reduce the ~305 LOC Sprint 0a scope.

---

## 4. Tier B Model — Recommendation

**Keep OLMo 3 32B Think on L4 (Yo-Yo #1 "trainer"). Do not downgrade.**

Economics: L4 GPU rental cost is dominated by the instance-hour, not model size. Running
7B Think leaves 19GB VRAM unused at the same price. The 32B Think produces ~95%+ valid JSON
arrays for extraction (proven at 74 entities); 7B Think is ~60-75% reliable.

**Tier B "graph" (Yo-Yo #2 / H100 / Llama 3.3 70B):**
Llama 3.3 70B is open-weight, closed-training-data. Permissible as a **process-only** backend:
output is structured facts, never generated prose or user-visible content. The audit ledger
records `tier: yoyo-graph` so regulatory review can distinguish OLMo-base extractions from
Llama-base extractions.

Fallback: OLMo 3 32B Instruct (non-Think) if Think variants are unstable under streaming.

---

## 5. BCSC Permissible Model Families

**Principle:** Sovereign Data Foundation compute substrate is sourced exclusively from
jurisdictions whose data-export and AI-governance regimes are reciprocally aligned with
Canadian continuous-disclosure expectations under NI 51-102. PRC-headquartered model labs
are excluded because PRC AI interim measures (CAC review, export controls) cannot be
represented as durably available, and the "Sovereign Data" claim materially weakens if any
inference-substrate component is subject to extraterritorial PRC AI regulation.

| Family | Maker | Open weights | Open training data | BCSC posture | Sovereign roadmap |
|---|---|---|---|---|---|
| OLMo 2/3 | AI2 (US 501c3) | Yes | **Yes (Dolma)** | **Permissible** | **Required** — only open-data base for fine-tune |
| Anthropic Claude | Anthropic (US PBC) | No | No | **Permissible** | Tier C passthrough only |
| Llama 3/4 | Meta (US) | Yes | No | Permissible w/ disclosure | **Process-only** (e.g. "graph" Yo-Yo) |
| Mistral / Mixtral | Mistral AI (FR) | Yes | No | Permissible w/ disclosure | Process-only |
| Gemma 2/3 | Google (US) | Yes (restrictive use terms) | No | Permissible w/ disclosure — read prohibited-use carefully | Avoid; use terms create disclosure risk |
| Phi 3/4 | Microsoft (US) | Yes | No (synthetic-heavy) | Permissible w/ disclosure | Skip — synthetic provenance hard to footnote |
| Qwen 2.5/3 | Alibaba (CN) | Yes | Partial | **Excluded** | Excluded |
| DeepSeek V3/R1 | DeepSeek (CN) | Yes | Partial | **Excluded** | Excluded |
| Yi 1.5 | 01.AI (CN) | Yes | No | **Excluded** | Excluded |
| GLM 4 | Zhipu (CN) | Yes | No | **Excluded** | Excluded |

**Anthropic (Tier C) and the BCSC constraint:** Not affected. Doorman's sanitise step (SYS-ADR-07)
ensures structured organisational data does not leave the boundary — Tier C receives prose only.
Describe Tier C in filings as "external prose-only inference, sanitised at the Doorman boundary"
not "data sent to Anthropic." Audit-ledger-verifiable.

---

## 6. Own-Model Roadmap — Gap Analysis

**What you have:** WORM ledgers, entity extraction → LadybugDB, apprenticeship endpoints
code-complete, 453 engineering + 137 apprenticeship tuples past threshold, OLMo open-data base.

**Gaps (ordered by correct fix sequence):**

| Priority | Gap | What's missing | Rough LOC |
|---|---|---|---|
| 1 | Evaluation harness | No held-out eval set; no regression test; no quality scoreboard | ~200 |
| 2 | Corpus quality gate | No acceptance criteria: min brief length, min diff size, dedup, PII scrub | ~150 |
| 3 | Diff capture closure | `/v1/shadow` receives `actual_diff` but it is empty — git post-commit hook not wired | ~50 |
| 4 | Adapter versioning + signing | ARCHITECTURE.md §Ring 3b says Sigstore-signed GCS object; code not written | ~300 |
| 5 | Graph-grounded training | Training pairs don't include LadybugDB graph context as signal | ~500 |

**Critical sequence:** Build eval harness (Gap 1) BEFORE first training run (Gap 3). Inverting
produces unmeasurable adapters. Gap 2 before Gap 3 prevents PII/noise from poisoning corpus.

---

## 7. Annual Tier A Refresh Policy

**Ratify as `conventions/permissible-model-substrate.md`.**

Required sections:
1. Permissibility criteria — two gates: (a) open weights + redistributable license; (b) jurisdictional alignment per §5
2. Open-data preference — Tier A and own-model base MUST be open-training-data subset
3. Permissible table — reproduce §5 above; update on change; never loosen
4. Upgrade trigger — new release passes: permissible list ✓ + regression test ≥98% of current ✓ + MemoryMax fits ✓ + 14-day soak ✓
5. Approval gate — operator signs `MODEL-CHANGE-*` ledger entry (same provenance as commits)
6. Deployment procedure — reference `bin/apply-model-change.sh` (to be written)
7. Tier-specific tables — Tier A, Tier B "trainer", Tier B "graph" (audit-flagged process-only), Tier C
8. Annual Q1 review cadence — document review even when no changes result

---

## 8. Strategic Verdict

Architecture is sound. The three-ring separation, WORM substrate, apprenticeship loop, and
OLMo-only-for-Tier-A reinforce each other into a coherent compound moat. The risk profile
is concentrated in plumbing, not strategy.

**Three immediate priorities (in order):**

1. **Reconcile Tier A model drift + upgrade to OLMo 2 1124 7B Instruct** — fixes haiku-tier
   Sprint 0a routing, improves apprenticeship signal, and removes false metadata in audit ledger.
   One day of work. Also verify: `SLM_APPRENTICESHIP_ENABLED=true` is in local-doorman.service
   but CLAUDE.md says "unset" — reconcile the docs.

2. **Replace per-boot retry with persistent queue + Yo-Yo-up notification** — eliminates the
   retry storm bug class by construction. Two to three days.

3. **Draft + ratify `conventions/permissible-model-substrate.md`** — locks BCSC posture, OLMo-only
   rule, and upgrade procedure as policy before any further model changes. One week.

After these three, universal-ai-gateway Sprint 0a ships into a sound, documented substrate.
