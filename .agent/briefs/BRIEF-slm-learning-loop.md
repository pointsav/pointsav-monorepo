---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-slm-learning-loop
title: "SLM Learning Loop — DPO Training Pipeline"
status: active
owner: project-intelligence
created: 2026-05-29
updated: 2026-06-13 (session 10c — Phase 4b ledger root-cause fix: mark_sweep_sha_complete() in service-content + Phase 4b shell)
author: totebox@project-intelligence (claude-sonnet-4-6)
companion: BRIEF-slm-substrate-master.md
grounds_in:
  - service-slm/ARCHITECTURE.md §Apprenticeship Substrate
  - conventions/apprenticeship-substrate.md
  - service-slm/crates/slm-doorman-server/src/main.rs (shadow drain worker)
  - DOCTRINE.md claims #49, #54
  - Anthropic ToS §2.c (hard boundary)
research_sources_2026_05_31:
  - "Rethinking DPO: The Role of Rejected Responses in Preference Misalignment" arxiv 2506.12725
  - "What Matters in Data for DPO?" arxiv 2508.18312
  - "An Empirical Study of SFT-DPO Interaction in Small LMs" arxiv 2603.20100
  - "CodeDPO: Aligning Code Models with Self Generated and Verified Source Code" arxiv 2410.05605
  - "More is Less: Synthetic Data Pitfalls in DPO" arxiv 2504.02193
---

# BRIEF — SLM Learning Loop

> **Companion to BRIEF-slm-substrate-master.md.** That BRIEF covers substrate operations
> (Yo-Yo VM, DataGraph, tier routing). This BRIEF covers the training pipeline and the
> sovereign coding agent architecture that feeds it.

---

## §1 — ToS boundary (hard constraint, non-negotiable)

```
Claude (Tier C) outputs → NEVER → OLMo fine-tuning
OLMo (Tier A/B) outputs → apprenticeship queue → LoRA fine-tuning ✓
Claude Code text → CORPUS → OLMo entity extraction → LadybugDB ✓ (extraction only)
```

**Rationale:** Anthropic ToS §2.c prohibits using Claude outputs to train a competing model.
The constraint applies to model training only — using Claude Code session text as a *source
document* for OLMo entity extraction is not training and has no ToS conflict. Same pattern as
ingesting any other CORPUS document (emails, project files).

Tier C stays unconfigured by deliberate design. No Anthropic API key will be set.

---

## §1b — DPO corpus quality finding (2026-05-29)

**591 degenerate DPO tuples have accumulated in the corpus.** These look like valid
training pairs but carry no meaningful signal.

**Root cause:** `pick_tier_for_brief` (apprenticeship.rs:515–522) routes all large briefs
(size > threshold) to Tier B for distillation. With Tier B TERMINATED and 1,460+ circuit
failures, the drain worker falls back to Tier A. OLMo 7B handles the request but sets
`attempt.escalate=true` and returns `attempt.diff=""` (nothing to show — the model escalated
rather than producing a concrete diff). `write_shadow_tuple` records this as a valid tuple.
`export-dpo.sh` maps it as `chosen: .actual_diff`, `rejected: .attempt.diff=""`.

A rejected sample with an empty string is not a rejected sample — it is noise. Training on
these tuples teaches the model nothing about preferring good diffs over bad ones.

**Engineering SFT tuples are NOT affected.** The 1,410 SFT tuples from `git-post-commit-hook.sh`
via `capture-edit.py` capture actual committed diffs and are completely independent of Tier B
availability. These are valid training signal regardless of circuit state.

`corpus-threshold.py` currently conflates both populations in its count, reporting a combined
total that includes the 591 degenerate tuples. The true honest DPO count is ≤ 5.

**Fixes (Sprint 1A + 2C):**

- `export-dpo.sh` (line 86–88): add `select(.attempt.diff != null and .attempt.diff != "")` before
  the chosen/rejected mapping. Filters existing degenerate files from future export runs.
- `corpus-threshold.py` (line 32–45): content-filter DPO files — open each before counting,
  skip those where `attempt.diff == ""` or null.
- `write_shadow_tuple` (apprenticeship.rs:319–396): guard at top — if `attempt.escalate == true
  && attempt.diff.is_empty()`, log WARN and return Ok(()) without writing a file.

After Sprint 1A: `python3 service-slm/scripts/corpus-threshold.py` will report honest DPO count ≤ 5.

---

## §2 — Apprenticeship substrate (code-complete, partially wired)

### What is code-complete

- **`/v1/brief` endpoint** — accepts `ComputeRequest` for apprenticeship capture; writes
  to `data/apprenticeship/queue/` as JSONL tuples
- **`/v1/shadow` endpoint** — accepts `actual_diff` field; writes shadow capture to queue
- **Shadow drain worker** — `main.rs:199-298`; background task; dispatches shadow briefs
  to Tier B for distillation; 30s backoff on `Retry` outcome (commit `d835cab5`)
- **corpus-threshold.py** — Sunday 02:00 UTC systemd timer; checks if tuple count exceeds
  threshold (453 engineering + 137 apprenticeship already above threshold)
- **`data/apprenticeship/queue/`** — current state: 491 poison briefs (pre-backoff-fix
  tight-loop artifacts); 539 queue-done briefs; 1,995 training corpus tuples

### What is NOT yet wired

- **Git post-commit hook** — ~~design complete; script not written~~ **SCRIPT WRITTEN** (`service-slm/scripts/git-post-commit-hook.sh`, commit `1d819d7c`). Install per archive: `cp service-slm/scripts/git-post-commit-hook.sh .git/hooks/post-commit && chmod +x`. No archive has the hook installed yet — Command Session action needed.
- **Claude Code CORPUS bridge** — ~~design complete; script not written~~ **SCRIPT WRITTEN** (`service-slm/scripts/claude-session-bridge.py`, commit `1d819d7c`). Needs `local-claude-bridge.service` systemd unit + `CORPUS_WATCH_DIR` pointed at service-content's watched directory.

### Queue state (2026-05-31 session 13)

- queue/: 5 pending
- queue-done/: 550 briefs
- queue-poison/: 78 files — up from 0 at session 11 close (590 prior briefs were quarantined
  in sessions 10–11). Newest entries (May 31 04:47–04:58 UTC, post-Fix-B) have `actual_diff: ""`
  and no `response_raw` — never dispatched to OLMo. Root cause: either pre-Fix-A carry-forward
  briefs (H1, likely) or hook still broken for some commits (H2). Investigate before quarantining.
  See BRIEF-project-intelligence-active-work.md §1 for investigation procedure.

With Sprint 3C (drain worker pause), new briefs will be held in queue/ rather than moved
to queue-poison/ during extended Tier B outages. This prevents future accumulation.

---

## §3 — Sovereign coding agent architecture

### Two parallel agents

```
Claude Code (Max Pro)              Goose (AAIF/block, v1.36.0+)
  ↓                                  ↓
Anthropic direct                   ANTHROPIC_HOST=http://127.0.0.1:9080
(no proxy — ToS constraint)        ANTHROPIC_API_KEY=foundry-local (dummy — no auth)
  ↓                                  ↓
~/.claude/projects/**/*.jsonl      service-slm Doorman :9080
  ↓                                  ├── Tier A (OLMo 7B) — always-on primary
[CORPUS bridge → OLMo extraction]  ├── Tier B (Yo-Yo OLMo-3-32B-Think) — nightly bonus
  ↓                                  └── /v1/shadow (apprenticeship capture)
LadybugDB entity graph                   ↓
                                    LoRA fine-tuning (Sunday 02:00 UTC)
```

**Why Claude Code is not proxied:** Anthropic ToS. Max Pro web login also does not flow
through the gateway. This is correct behavior, not a gap.

**Why Goose:** Rust, Apache-2.0, AAIF (Linux Foundation, Dec 2025), 46k+ stars. Already
has `ANTHROPIC_HOST` env var — zero fork needed for initial testing. Native Anthropic
Messages API + MCP. Provider abstraction is a clean Rust trait — fork later to strip
24 non-Anthropic providers.

**No Anthropic API key needed:** Doorman's `/v1/messages` does zero auth validation;
`x-api-key` header is silently ignored.

### Goose operator setup (after Sprint 1)

```bash
export ANTHROPIC_HOST=http://127.0.0.1:9080
export ANTHROPIC_API_KEY=foundry-local
export GOOSE_MODEL=claude-haiku-4-5-20251001   # → Tier A (OLMo 7B, always-on)
goose session

# Nightly Yo-Yo window (2:00–3:00 AM UTC):
export GOOSE_MODEL=claude-sonnet-4-6   # → Tier B if up; Tier A fallback
```

---

## §4 — Sprint 1: tool_use shim — CODE COMPLETE (commit `1b47d3eb`, Jennifer)

**File:** `service-slm/crates/slm-doorman-server/src/http.rs`

Implemented 2026-05-29. 51/51 http_test pass, 102/102 slm-doorman tests pass.
Summary of 7 changes (all implemented):

1. Add `tools`, `tool_choice`, `top_p`, `top_k` to `AnthropicMessagesBody` (line ~1214)
2. Tool-turn thinking suppression: when `tools.is_some()`, `reasoning_budget = 0` — workaround
   for llama.cpp #20345 (grammar constraints disabled when thinking is active; unfixed upstream)
3. Tool_use SSE blocks in `anthropic_sse_body` — parse OLMo OpenAI-format `tool_calls`,
   emit as Anthropic `tool_use` content blocks with `stop_reason: "tool_use"`
4. `POST /v1/messages/count_tokens` new route — `len(bytes) / 4` heuristic (~20 LOC)
5. Confirm `#[serde(deny_unknown_fields)]` NOT set — `cache_control`, `anthropic-beta` must not 400
6. `GET /v1/models` new route — returns haiku + sonnet model IDs (~15 LOC)

---

## §5 — Sprint 2: training pipeline wiring (Peter)

### 2a. Git post-commit hook (30 LOC shell)

**New file:** `service-slm/scripts/git-post-commit-hook.sh`

Every commit in any Totebox archive triggers a diff capture to `/v1/shadow`. This is the
mechanism by which Goose's coding work enters the training queue.

```bash
#!/bin/bash
DIFF=$(git diff HEAD~1 HEAD --unified=3 2>/dev/null || git show HEAD --unified=3)
curl -s -X POST http://127.0.0.1:9080/v1/shadow \
  -H "Content-Type: application/json" \
  -H "X-Foundry-Module-ID: git-hook" \
  -d "{\"actual_diff\": $(echo "$DIFF" | python3 -c 'import json,sys; print(json.dumps(sys.stdin.read()))')}" \
  > /dev/null 2>&1 &
```

Install per archive: `cp service-slm/scripts/git-post-commit-hook.sh .git/hooks/post-commit && chmod +x .git/hooks/post-commit`

### 2b. Claude Code CORPUS bridge (80 LOC Python)

**New file:** `service-slm/scripts/claude-session-bridge.py`

Watches `~/.claude/projects/**/*.jsonl`, extracts assistant text turns, writes
`CORPUS_claude_<session>_<turn>.json` to the service-content watched dir.
Runs as `local-claude-bridge.service` systemd unit.

**ToS compliance:** OLMo extracts entities FROM Claude's text (Claude is the source document).
OLMo is not being supervised on Claude outputs. Same as ingesting any external document.

---

## §6 — Leapfrog 2030 strategic context

Five Opus 4.7 research agents confirmed the following (2026-05-29):

**Competitive threat:** Microsoft Azure Local Disconnected — 18–30 month window before
Copilot coding ships in a genuine air-gap substrate. Google (cloud-only by strategy) and
AWS (GovCloud, not disconnected) are NOT the threat.

**Three structural moats hyperscalers cannot replicate:**
1. **OLMo verifiable training provenance (OLMoTrace)** — Full Dolma 3 dataset published;
   any inference traceable to training data. Cite in regulated-sector procurement.
2. **Per-inference Ed25519 signed audit trail** — California SB 942 (Jan 2026), EU AI Act
   Article 12 (Aug 2026), OCC SR 11-7. F12 signed commit chain is the foundation; per-inference
   signing is the extension. 24-month first-mover window before it's table stakes.
3. **Customer-owned compounding LoRA weights** — apprenticeship corpus written to
   customer-controlled storage; LoRA adapter handed over with integrity proof. Microsoft
   telemetry returns to Microsoft by design.

**Market framing that wins:** "audit-grade AI coding" (not "sovereign AI"). Map capabilities
to specific regulatory clauses. Target mid-regulated mid-market (50–500 engineers in
insurance, regional banking, healthcare SaaS, AEC, legal) — 90-day procurement cycles,
unserved by Tabnine or Copilot SaaS.

**Open-weight quality trajectory:** OLMo 3.1 (+20 IFBench vs 3.0, better tool-use reliability,
18.6GB Q4_K_M fits L4 24GB) is upgrade path for Tier B. Not blocking Sprint 1.

---

## §7 — Definition of done (learning loop)

The learning loop is operational when ALL of the following pass:

1. `curl -s http://127.0.0.1:9080/readyz | python3 -m json.tool` → `has_local: true` (Tier A live)
   **STATUS: VERIFIED ✓** (`has_local: true, has_yoyo: true`)
2. `ANTHROPIC_HOST=http://127.0.0.1:9080 ANTHROPIC_API_KEY=foundry-local goose session` → chat round-trips
   **STATUS: VERIFIED ✓** (2026-05-29T04:10Z — Goose v1.36.0 round-tripped; OLMo replied "Hello! The result
   of 2+2 is 4."; Doorman log: `dispatching ... tier="local"`; fix required: `system`-as-blocks deserialization
   in `http.rs` — committed `74ba6da0`)
3. Goose file tool (Read/Write) → `sudo journalctl -u local-doorman -f | grep -i tool_use` confirms routing
   **STATUS: PARTIAL — tool format shim verified (104 unit tests); LIVE SSE TEST: OLMo 7B returned text**
   **(stop_reason: end_turn) instead of invoking the tool. OLMo 7B is not fine-tuned for tool use;**
   **tool_use SSE blocks require Tier B (OLMo 3 32B-Think) or a tool-use-tuned Tier A model.**
4. `sudo journalctl -u local-content -f | grep 'entities extracted'` → Claude Code CORPUS bridge feeding LadybugDB
   **STATUS: BLOCKED — extraction requires Tier B (Yo-Yo VM). Circuit breaker OPEN.**
   CORPUS bridge writes files correctly; service-content routes `/v1/extract` → Doorman → Tier B;
   Tier B circuit open because Yo-Yo VM not provisioned. Deferred until Yo-Yo VM up.
   Workaround option: route extraction to Tier A (OLMo-7B) — separate sprint decision.
5. `git commit` in any Totebox archive → `/v1/shadow` hit confirmed in Doorman log
   **STATUS: VERIFIED ✓** (shadow brief enqueued at queue_position 2-5 on every commit)
6. Sunday 02:00 UTC: `corpus-threshold.py` reports above threshold → LoRA training job starts
   **STATUS: PENDING — depends on §7.4 being unblocked**

### Additional verified endpoints (not in original §7)
- `/v1/messages/count_tokens` → `{"input_tokens": N}` ✓
- `/v1/shadow` with full ShadowWire payload → `202 ACCEPTED` + queue entry ✓
- `/v1/messages` with tools → LIVE TEST COMPLETE 2026-05-29: tool format shim works (no "Missing tool type"
  error from llama-server); OLMo 7B returned `stop_reason: end_turn` with text response rather than a
  `tool_use` content block. Model does not reliably invoke tools; this is a model capability limit, not a shim bug.

### Blockers summary (2026-05-29)

- **§7.3**: OLMo 7B does not invoke tools (responds with text). Tool_use requires Tier B (OLMo 3 32B-Think).
- **§7.4**: Tier B circuit OPEN — VM terminated, europe-west4-a L4 stockout. Restart when capacity returns.
- **§7.6**: Downstream of §7.4 (LoRA training requires valid DPO tuples).

**DPO corpus quality gate (new, from §1b):**
Before any LoRA run, verify `export-dpo.sh` filtered output contains ≤ 5 tuples (after Sprint 1A fix).
591 existing degenerate tuples must NOT be included in a training run — they will degrade the model.
The 1,410 engineering SFT tuples are always valid and may be used independently.

---

## §8 — Apprenticeship prompt audit + fixes (session 12, 2026-05-31)

Full audit of the call chain from `dispatch_shadow()` → OLMo 7B and the live training corpus.

### Two critical gaps found and fixed

**Fix A — `actual_diff: ""` in all 554 corpus entries (DEPLOYED, commit `a0649002`)**

Root cause: `/srv/foundry/bin/capture-edit.py` (the installed hook) used the pattern:

```bash
DIFF=$(git diff HEAD~1 HEAD ...)
PAYLOAD=$(python3 - "$COMMIT_MSG" <<'PYEOF'
diff_text = sys.stdin.read()   # BUG: stdin is already consumed by the heredoc script source
```

When `python3 -` is invoked with `<<'PYEOF'`, the heredoc IS stdin — reading it provides the
script source. `sys.stdin.read()` inside the script then returns `""` because stdin was already
exhausted. The bash variable `$DIFF` was captured correctly but never reached Python.

Fix: pass via env var — `HOOK_DIFF="$DIFF" python3 -` then `os.environ.get('HOOK_DIFF', '')`.
Applied to both `service-slm/scripts/git-post-commit-hook.sh` and workspace `/srv/foundry/bin/capture-edit.py`.

Verification: three newest queue entries after fix have non-empty `actual_diff` (2–3.5 KB each).
All future commits now produce complete DPO pairs with a real gold label.

**Fix B — 100% escalation rate from OLMo preamble before `---` (DEPLOYED, commit `a0649002`)**

Root cause: `APPRENTICE_SYSTEM_PROMPT` (apprenticeship.rs line ~446) contained terms OLMo
has never seen ("Doctrine claim #32", "Master/Root/Task Claude", "Foundry apprentice"). OLMo
responded with substantive reasoning but wrote natural-language preamble before the YAML
frontmatter. The `extract_frontmatter()` regex (`r"(?s)\A\s*---\s*\n(.*?)\n---\s*\n"`) requires
`\A` (start of string); any preamble = no frontmatter parsed = fallback defaults:
`self_confidence: 0.0, escalate: true`.

Fix: rewrote `APPRENTICE_SYSTEM_PROMPT` with OLMo-compatible plain instructions:
- Explicit "Do not write any introductory text before the opening `---`"
- Concrete format example showing exact YAML + ## Reasoning + ## Diff block
- Rules in plain language (no internal Foundry terminology)

All corpus entries before Fix B: `escalate: true`, `self_confidence: 0.0`. After Fix B the
next drain cycle will produce the first genuine attempt results.

### What is NOT yet done (Fix C — deferred)

**Fix C: Add GBNF grammar to `dispatch_shadow()` in `apprenticeship.rs`**

Currently both `dispatch_shadow()` calls (lines 181 and 279) pass `grammar: None`.
Adding `grammar: Some(GrammarConstraint::Gbnf(APPRENTICE_GBNF_GRAMMAR))` would constrain
OLMo output at the token level, eliminating format-failure escalations entirely. The wiring
is already in place (`LocalTierClient::complete()` handles GBNF); only the constant and
wiring call are missing.

Deferred: Fix B (system prompt rewrite) may be sufficient to reduce escalation rate to
acceptable levels without the complexity of a GBNF grammar. Observe the next 5–10 drain
cycles before implementing Fix C. If OLMo still preambles after Fix B, implement Fix C.

**Session 13 update — 78 poison entries accumulated post-Fix-B:** Newest entries have
`actual_diff: ""` and no `response_raw` (never dispatched to OLMo). This is likely
pre-Fix-A carry-forward briefs being reclassified by the Sprint 2C degenerate-tuple guard,
NOT a Fix B failure. Investigation required before concluding Fix C is needed. Fix C
addresses preamble-before-`---`; it does NOT help when `actual_diff` is empty.

### Key inference timing fact (for future planning)

OLMo 7B on this CPU VM runs at approximately **2 tokens/second**. With `max_tokens=2048`,
a single shadow brief takes 17–60 minutes wall-clock time. The 2048-token budget was set
for Tier B (GPU) inference. For Tier A (CPU) primary mode, consider reducing `max_tokens`
to 512–768 for shadow briefs to bring per-brief latency under 10 minutes, at the cost of
shorter diffs in OLMo's output. This is a separate configuration decision.

---

## §9 — Corpus audit findings + revised architecture (2026-05-31 session 14)

**What we found after a full corpus audit:**

### Corpus reality check

| Path | Count | Quality |
|---|---|---|
| `training-corpus/engineering/*/` | 1,410 edit tuples | `actual_diff: ""` — ALL pre-Fix-A, all empty. USELESS for LoRA SFT. |
| `training-corpus/apprenticeship/shadow-capture/` | 548 tuples | `attempt.diff: ""` — OLMo produced empty diffs. POTENTIALLY HARMFUL for DPO. |
| `queue/` (pending, post-Fix-A) | 77 entries | Real diffs (Fix A working). NOT YET PROCESSED. **This is the only real signal we have.** |

The 77 pending entries in `queue/` are the first genuinely useful data. They have real
`actual_diff` content (Fix A deployed 2026-05-31). They are waiting for GPU processing.

### Why CPU DPO is wrong (research-backed)

Five research papers (2024-2026) confirm the current approach is counterproductive on CPU:

1. **Empty rejected samples are HARMFUL, not neutral.** DPO loss is dominated by minimising
   rejected-sample probability. With empty diffs as rejected samples, the loss function
   optimises toward nothing — or degrades the chosen samples. The 548 existing shadow-capture
   tuples should be EXCLUDED from any training run, not just ignored.
   (Source: arxiv 2506.12725 "Rethinking DPO")

2. **At 1,410 samples, SFT alone outperforms SFT+DPO.** Empirical study on small models:
   DPO adds instability without benefit below ~5,000 sample threshold.
   DPO becomes effective only when: dataset >>2K AND model has large capacity AND
   rejected samples have ≥20% quality gap from chosen.
   (Source: arxiv 2603.20100 "SFT-DPO Interaction")

3. **OLMo 7B cannot critique its own code reliably.** The apprenticeship system asks OLMo
   to generate "what would you do differently" — a circular critique requiring the model to
   be better than it is to give useful feedback. At 7B with Q4_K_M quantisation, the model
   lacks the reasoning capacity for valid code critique on complex Rust diffs.

4. **CPU drain backlog math:** 77 pending × 30 min avg = 38 hours. New commits add
   5–10 briefs/day. The backlog grows faster than CPU can drain it — indefinitely.

### Revised architecture (what to do instead)

**Phase 1 — SFT LoRA (GPU, when Yo-Yo is available, ~2 hours)**

The 77 pending entries are direct SFT training data as-is. No OLMo processing needed:
- `brief.body` + `brief.scope` = instruction/context
- `actual_diff` = desired output (the real human-authored code change)

Extract these as SFT pairs, run LoRA fine-tuning directly:
```
rank=16, alpha=32, target q/k/v/up_proj/down_proj
5–10 epochs, early stop on held-out loss
Expected gain: +3–8% on codebase-specific tasks
```
This requires no OLMo shadow drain at all — the data is already captured.

**Phase 2 — GPU-gated DPO via CodeDPO (requires Yo-Yo OLMo 3 32B-Think)**

When Yo-Yo is running, use execution-based validation instead of OLMo self-critique:
1. OLMo 3 32B-Think generates 3–5 candidate diffs per brief (~30s each on GPU)
2. Run `cargo test` or `cargo check` on each candidate
3. Chosen = passes tests (or the actual human diff)
4. Rejected = fails tests (clear, objective signal)
5. Quality threshold: ≥40% pass-rate gap between chosen and rejected
6. Target: 200–500 high-confidence DPO pairs before first LoRA run

**Phase 3 — Continuous improvement**
- SFT refresh monthly with new post-Fix-A commits (domain drift correction)
- DPO expansion with each Yo-Yo session (execution-validated pairs only)
- Never train on: empty diffs, self-evaluated pairs, or cross-model inconsistent pairs

### What changed operationally (updated 2026-06-01)

- `SLM_HOLD_THRESHOLD_SECS` changed from `3600` → `1` (2026-05-31) — aggressive hold
- **`SLM_DRAIN_PAUSED=false` set 2026-06-01** — drain is now **ACTIVE**; routing all pending briefs
  through Tier A (OLMo 2 7B). Flow verified: `tier="local"` confirmed in dispatch log; 0 poison
  after initial drain pass (76 empty-diff ghost commits skipped cleanly; 209 real-diff briefs queued)
- SFT capture via post-commit hook continues (new commits still write to `queue/`)
- The 285 pending briefs (209 real-diff) are draining via Tier A; GPU drain via Yo-Yo is not blocked

### Fix C — decision deferred indefinitely

Fix C (GBNF grammar) was addressing OLMo format failures. With CPU drain paused and
the architecture shifting to GPU-only drain, Fix C is no longer urgent. Deprioritise.
When GPU drain resumes, OLMo 3 32B-Think handles format constraints more reliably.

### LoRA fine-tuning — when ready

Checklist before first LoRA run:
- [ ] Yo-Yo GPU available
- [ ] Extract 77 post-Fix-A queue entries as SFT pairs (script to write)
- [ ] Validate `actual_diff` non-empty for all 77 (spot-check 10%)
- [ ] Run CodeDPO on 50–100 entries (GPU) to generate validated DPO pairs
- [ ] Exclude all 548 existing shadow-capture tuples from training set
- [ ] LoRA: rank=16, alpha=32, 5–10 epochs, early stop

---

## §10 — Forward operating model (researched 2026-05-31; THE comprehensive plan)

> This section is the durable answer to "how do we run the learning loop going forward."
> Grounded in: corpus audit (§9), web research (8 papers + GPU/cost benchmarks, sources
> in frontmatter), and the EXISTING pipeline (`scripts/lora-update.sh` + `docs/yoyo-training-
> substrate-and-service-content-integration.md`). Read this before any training work.

### §10.1 — What already exists (do NOT rebuild)

`service-slm/scripts/lora-update.sh` is a complete 9-step orchestrator, HARD DISABLED behind
two gates (`SLM_LORA_AUTO_ENABLE=true` + operator-signed `data/training-approved/<id>.tag`):

```
export-dpo.sh → snapshot → push to Yo-Yo trainer VM → ssh trigger Unsloth DPOTrainer
→ poll for adapter (≤4h) → pull adapter → eval-adapter.sh → register in data/adapters/registry.yaml
```

It already implements (matching research best practice):
- **Adapter versioning** — `data/adapters/registry.yaml` (research §3: version adapters separately)
- **Eval gate** — `eval-adapter.sh` scores against held-out set before registration (research §2)
- **F12 human gate** — SYS-ADR-10 compliant; no automated training without operator tag (Doctrine)
- **Unsloth trainer** — ~40% faster than vanilla PEFT on L4 (research: 2–3h for 1,500 samples)

**Gap:** the pipeline is DPO-first (`export-dpo.sh`). Research says SFT-first at our scale.
The single missing piece is an `export-sft.sh` that emits `{instruction, output}` JSONL from
queue entries (brief.body+scope → instruction; actual_diff → output). ~40 LOC.

### §10.2 — The drain pause (immediate decision)

The CPU shadow drain produces empty OLMo diffs (harmful — §9). It must pause until GPU returns.
No existing env var pauses it cleanly because the Sprint 3C hold is bypassed when
`SLM_TIER_A_FIRST=true` (router.rs:223). Three levers, in order of preference:

| Lever | Stops CPU drain | Keeps SFT capture | Routing impact | Effort |
|---|---|---|---|---|
| **`SLM_DRAIN_PAUSED=true`** (NEW — recommended) | ✅ unconditional | ✅ yes | none | ~10 LOC + rebuild |
| `SLM_TIER_A_FIRST=false` + `HOLD=1` | ✅ after ~90s | ✅ yes | High-complexity → dead Yo-Yo first (negligible for dev traffic) | env only, immediate |
| `SLM_APPRENTICESHIP_ENABLED=false` | ✅ unconditional | ❌ /v1/shadow returns 404 | none | env only, immediate |

**Recommendation:** ship `SLM_DRAIN_PAUSED` as the proper lever (drain loop checks it before
dequeue, sleeps, continues — fully decoupled from routing and capture). Until that binary
ships, bridge with `SLM_TIER_A_FIRST=false` to keep capture alive. Avoid disabling
apprenticeship — losing the SFT capture stream is the one thing we cannot afford.

### §10.3 — The sustainable GPU-gated loop (target operating model)

Per-cycle cost ~$1.74, monthly ~$10–15 on L4 spot. Two GPU jobs, run as SEPARATE spot launches
(both don't fit one 1–2h window; Job B alone is ~2h):

```
TRIGGER (whichever first):  weekly  OR  ≥100 new post-Fix-A briefs in queue/

JOB A — DPO candidate generation [L4 spot, ~45 min, vLLM batch]:
  For each of 50–100 briefs: OLMo 3 32B-Think generates 3–5 candidate diffs
  → run `cargo check`/`cargo test` on each → chosen=passes, rejected=fails
  → keep only pairs with ≥40% pass-rate gap (CodeDPO; arxiv 2410.05605)

JOB B — LoRA training [L4 spot, ~2h, Unsloth]:
  SFT set (export-sft.sh: queue actual_diffs) + replay buffer (100–500 historical, 2:1 new:old)
  + validated DPO pairs from Job A (only if ≥200 exist)
  → LoRA rank=16, alpha=32, lr=2e-4 warmup-stable-decay, weight_decay=0.01, 5–10 epochs
  → checkpoint every 500 steps to GCS (spot preemption survival)

EVAL GATE [CPU, eval-adapter.sh]:
  Held-out: 10–30 diffs stratified by language/change-type, never in training set
  Metric: pass@5 on held-out. Promote ONLY if pass@5 ≥ current adapter (no regression).

PROMOTE [llama.cpp hot-swap]:
  GGUF LoRA adapters stay separate from base; swap via /lora-adapters endpoint (~1s).
  Canary 10% of requests → if no regression, promote 100%; else rollback (registry.yaml).
```

### §10.4 — Catastrophic-forgetting guardrails (mandatory for repeated narrow fine-tunes)

Repeated LoRA on one codebase erodes general capability. Apply all three (research §4):
1. **Replay buffer:** 100–500 historical commits mixed at 2:1 new:old per run
2. **Conservative LR:** 2e-4, warmup-stable-decay (10/80/10), `weight_decay=0.01` (L2-LoRA)
3. **General-data interleave:** every 3rd cycle, ~20% public Rust/Python in the batch

Expected: retain ~90–95% general capability, +5–8% on Foundry-specific tasks.

### §10.5 — Phased rollout (concrete next steps)

- **Phase A (next coding session):** add `SLM_DRAIN_PAUSED` flag (~10 LOC, main.rs drain loop);
  write `export-sft.sh` (~40 LOC). Both ship in one Stage 6.
- **Phase B (first Yo-Yo window):** run Job B SFT-only on the 77+ queue briefs (no DPO yet).
  Register adapter v1, eval against held-out, do NOT auto-promote — manual review first run.
- **Phase C (second Yo-Yo window):** add Job A CodeDPO generation; accumulate ≥200 validated
  pairs before first DPO-augmented run.
- **Phase D (steady state):** arm `lora-update.timer` (weekly) with the operator approval-tag
  gate; canary+promote loop runs hands-off except the F12 tag.

### §10.6 — What this means for the local OLMo (the operator's question)

The local 7B's best uses, in priority order:
1. **Interactive Goose sessions** — short prompts, code navigation, quick drafts (its sweet spot)
2. **Entity extraction** for service-content (short structured output — handles well)
3. **The model being improved** — it is the *target* of the LoRA loop, not the *generator* of
   training data. The 32B-on-GPU is the generator/teacher; the 7B-on-CPU is the student.

It should NOT be: a batch DPO-critique generator on CPU. That was the mistake — fixed here.

---

## §11 — Daily Yo-Yo enrichment cycle (deployed 2026-06-09)

### What was built

`/srv/foundry/bin/yoyo-daily-cycle.sh` — a fully automated daily batch script:

```
TERMINATED VM
  → gcloud instances start
  → wait for llama-server health (~170s consistent)
  → wait for Doorman Tier B circuit to close
  → 80% of budget window: DataGraph enrichment drain
  → corpus-threshold.py check + training markers
  → SSH stop llama-server
  → gcloud instances stop
  → verify TERMINATED
```

**Systemd automation:**
- Service: `/etc/systemd/system/local-yoyo-daily.service`
- Timer: `/etc/systemd/system/local-yoyo-daily.timer`
- Schedule: `*-*-* 17:00 UTC` = 10:00 AM PDT (Vancouver daytime for monitoring)
- Planned move: `02:30 UTC` after 1-2 weeks once operator confirms stability
- Next fire: 2026-06-09 10:01 AM PDT

### 3-cycle test results (2026-06-09 00:04–00:24 UTC)

| Cycle | Duration | Entities | Enrichment DPO pairs | VM final |
|---|---|---|---|---|
| 1 | 10m 43s | 8239→8246 (+7) | 12→18 (+6) | TERMINATED ✓ |
| 2 | 9m 12s | 8246→8254 (+8) | 18→22 (+4) | TERMINATED ✓ |
| 3 | 10m 38s | 8254→8276 (+22) | 23→31 (+8) | TERMINATED ✓ |

GPU captured in cycle 3: 99% utilization, 16151/23034 MB VRAM, 73°C L4 — confirmed working.

Total elapsed may slightly exceed 600s test cap because the VM stop sequence adds ~90s after
the enrichment window ends. Enrichment stops at the cap; stop sequence is outside the cap.
This is expected and acceptable.

### Enrichment DPO pair format (TRL-compatible, verified)

```json
{
  "prompt": "<document chunk — 1,711 chars average>",
  "chosen": "[{\"classification\":\"Project\",\"entity_name\":\"service-slm\"}]",
  "rejected": "[]",
  "source_type": "datagraph-enrichment"
}
```

- `chosen` = Tier B (OLMo 32B, L4) entity extraction — richer, more complete
- `rejected` = Tier A (OLMo 7B, CPU) entity extraction — often empty or fewer entities
- Pair is only written when Tier B finds entities AND Tier A result differs
- `run-dpo-training.py::load_feedback_files()` reads both `enrichment-*.jsonl` and
  `apprenticeship-*.jsonl` with the same TRL field format — confirmed compatible

### Corpus state as of 2026-06-09

| Adapter | Count | Threshold | Status |
|---|---|---|---|
| engineering-pointsav (SFT) | 1,410 tuples | 50 | READY |
| apprenticeship-pointsav (DPO) | 225 valid tuples | 50 | READY |
| enrichment DPO pairs | 31 files | (feeds apprenticeship run) | ACCUMULATING |

Training markers (local, today-dated): `engineering-pointsav-2026-06-09.json` +
`apprenticeship-pointsav-2026-06-09.json` — both written. `SLM_YOYO_WEIGHTS_GCS_BUCKET`
not set → markers are local only; no auto-trigger to yoyo-batch yet.

### Cycle log location

```
/srv/foundry/data/yoyo-cycle-logs/cycle-YYYYMMDD-HHMMSS.log
```

Real-time diagnostics (when cycle is running):
```bash
journalctl -u local-yoyo-daily          # cycle service output
journalctl -u local-doorman             # Tier B circuit events
journalctl -u local-content             # entity extraction + enrichment writes
ssh -i ~/.ssh/google_compute_engine mathew@10.128.0.24 nvidia-smi  # GPU stats
```

---

## §12 — ML libraries installation plan (NEXT: complete the training pipeline)

### Current gap

`run-dpo-training.py` requires HuggingFace ML libraries that are **not installed** on
the yoyo-batch VM. The daily cycle drains enrichment and writes training markers, but
cannot trigger actual LoRA training until these are in place:

```
Required on yoyo-batch:
  pip install trl>=0.8 peft>=0.10 transformers>=4.40 datasets bitsandbytes
```

These install once to the persistent disk and survive VM stop/start cycles. Total ~6 GB
disk, ~8-10 min first install.

### Also note: `lora-update.sh` config drift

`lora-update.sh` references `TRAINER_INSTANCE=yoyo-tier-b-1` and `TRAINER_ZONE=europe-west4-a`.
The actual VM is `yoyo-batch` in `us-central1-a`. Must be corrected before any training run.

### Phased plan — in order

**Step 1 — Install ML libs on yoyo-batch (one-time, ~10 min)**

Start the VM, SSH in, install libraries, verify with dry-run, stop:

```bash
# Start VM
gcloud compute instances start yoyo-batch --zone us-central1-a

# Wait for SSH to be ready (~60s after start)
ssh -i ~/.ssh/google_compute_engine mathew@10.128.0.24 \
  "pip install --quiet trl>=0.8 peft>=0.10 transformers>=4.40 datasets bitsandbytes \
  && echo 'INSTALL OK'"

# Dry-run — verifies imports and corpus loading without training
ssh -i ~/.ssh/google_compute_engine mathew@10.128.0.24 \
  "python3 /srv/foundry/clones/project-intelligence/service-slm/scripts/run-dpo-training.py \
   --dry-run \
   --corpus /home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer/service-fs/data/training-corpus/feedback/"

# Stop VM
gcloud compute instances stop yoyo-batch --zone us-central1-a
```

Expected dry-run output: corpus scan, pair count, hyperparameter summary, no actual training.

**Step 2 — Fix lora-update.sh config (~5 min)**

Update `TRAINER_INSTANCE` and `TRAINER_ZONE` defaults in `lora-update.sh` to match
actual VM (`yoyo-batch`, `us-central1-a`).

**Step 3 — First real training run (manual, operator-supervised, ~2h on L4)**

Requires an operator session while the VM is running. Set:
```bash
SLM_LORA_AUTO_ENABLE=true
echo "first-run" > /srv/foundry/data/training-approved/coding-lora-$(date +%Y-%m-%d).tag
```

Then run the training directly via SSH to yoyo-batch:
```bash
ssh -i ~/.ssh/google_compute_engine mathew@10.128.0.24 \
  "python3 /srv/foundry/clones/project-intelligence/service-slm/scripts/run-dpo-training.py \
   --corpus <feedback-dir> \
   --base-model allenai/OLMo-2-1124-7B-Instruct \
   --max-runtime-seconds 5400"   # 90 min hard cap
```

Monitor: `journalctl -u local-doorman` for circuit state; SSH to yoyo-batch for nvidia-smi.

**Step 4 — Wire training trigger into the daily cycle (~30 min code)**

Add a Phase 7 to `yoyo-daily-cycle.sh`: after the threshold check, if training markers
exist AND the ML libs are confirmed installed, SSH-trigger `run-dpo-training.py` on yoyo-batch
with a time budget equal to remaining cycle time (up to 90 min total cap).

The VM is already running at this point in the cycle — no extra cost or start latency.

**Step 5 — Eval gate + adapter registration (deferred until after first training run)**

`eval-adapter.sh` and `data/adapters/registry.yaml` are already designed; wire up after
the first adapter is produced and manually inspected.

### Dependency summary

| Step | Dependency | Time estimate |
|---|---|---|
| 1. Install ML libs | yoyo-batch must be RUNNING | 10 min |
| 2. Fix lora-update.sh config | None (edit only) | 5 min |
| 3. First training run | Step 1 + operator session | 2h VM time |
| 4. Wire training into daily cycle | Step 1 complete | 30 min code |
| 5. Eval gate | Step 3 adapter produced | 1 session |

### One thing to decide (operator)

The GCS bucket approach (`SLM_YOYO_WEIGHTS_GCS_BUCKET`) vs. SSH-direct:
- **SSH-direct (recommended for now):** no GCS setup needed; the daily cycle already SSHes
  into yoyo-batch; training runs on the same VM, adapter written to local disk, then `scp`
  back to workspace. Simpler, fully operator-observable.
- **GCS:** needed if training runs asynchronously (VM continues while workspace is offline)
  or if adapters need to be accessible from multiple machines. Can be added later.

For Phase 1, SSH-direct is the right call. GCS is a Phase 2+ concern.

---

## §13 — As-Built Record

> **Purpose:** A living snapshot of what is actually deployed and working as of the date
> shown. Updated whenever a component is added, changed, or removed. Not a plan — a record
> of physical reality. Reconcile against this when something breaks before checking the
> design sections.
>
> **Last updated:** 2026-06-12 (session 8) — Continuous service: timer removed; `Restart=always`+`RestartSec=600` is the day-boundary mechanism; Phase 4b DataGraph sweep fallback (fires when corpus stalls, ingests git history → /v1/ingest); training Phase 6 fixed: rsync script+corpus to remote VM before SSH; receipt only written on rc=0

---

### System diagram (current state)

```
WORKSPACE VM (foundry-workspace, vault-privategit-source-1)
  │
  ├── local-doorman.service          :9080  Tier A (OLMo 2 7B, llama-server :8080)
  │     └── Tier B circuit → yoyo-batch (CLOSED when VM running; OPEN when TERMINATED)
  │
  ├── local-content.service          :9081  service-content (LadybugDB entity graph)
  │     └── enrichment-*.jsonl → feedback dir (continuous write when Tier B is up)
  │
  ├── local-yoyo-daily.service ──────────── continuous (Type=simple; Restart=always; RestartSec=600s)
  │     └── yoyo-daily-cycle.sh             THE single VM lifecycle controller [REWRITTEN 2026-06-11]
  │           Day-budget ledger: /srv/foundry/data/yoyo-budget/<date>.consumed (dynamic date; midnight-safe)
  │           main() outer loop: reads ledger; retries if budget remains; stops when spent → systemd restarts
  │           start_vm_with_retry(): STOCKOUT → ks_sleep(600s) → retry indefinitely; kill-switch aware
  │           run_stint(budget_secs) → Phases 1–8:
  │             Phase 1:  start_vm_with_retry (STOCKOUT retry internal)
  │             Phase 2:  wait llama-server health; anchor ENRICHMENT_END to VM:READY (not stint start)
  │             Phase 3:  wait Doorman Tier B circuit close
  │             Phase 4:  enrichment drain + stall/preemption detector (return 8 if VM vanishes)
  │             Phase 4b: [NEW] DataGraph sweep — fires on STINT_RETURN=7 (stall); ingests git commit
  │                        history (90 days, 200 commits/repo, 2 repos) to /v1/ingest → Tier A+B
  │                        extraction → DPO pairs on disagreement; SHA ledger prevents re-processing;
  │                        resets STINT_RETURN=0 so Phase 5+ runs normally
  │             Phase 5:  corpus-threshold.py → training markers
  │             Phase 6:  LoRA training: rsync script+corpus to yoyo-batch → SSH training → receipt
  │                        ONLY written on rc=0; failure logs and clears path for next cycle retry
  │             Phase 7:  GCS sync (when SLM_YOYO_WEIGHTS_GCS_BUCKET set)
  │             Phase 8:  SSH stop llama-server → gcloud stop → debit_seconds(VM-on time)
  │           Return codes: 0=clean, 7=stall-exit, 8=preempted→main recovers, 9=VM unavailable
  │     systemd: Type=simple; Restart=always; RestartSec=600; RuntimeMaxSec=28800; KillMode=control-group; ExecStopPost=gcloud stop (sole independent authority)
  │     env:    YOYO_DAY_BUDGET_MIN=120
  │
  ├── [DELETED 2026-06-11 session 8] local-yoyo-daily.timer — removed; Restart=always replaces it
  ├── [DELETED 2026-06-11 session 8] yoyo-idle-monitor source files — bin/yoyo-idle-monitor.sh,
  │     infrastructure/yoyo-manual/yoyo-idle-monitor.{timer,service}, yoyo-idle-check.sh
  │     REASON: idle-monitor was disabled session-7; source files deleted to prevent future confusion
  │
  └── local-corpus-threshold.timer   ────── MASKED (→ /dev/null)
        was: 02:00 UTC daily corpus check + VM start
        now: permanently disabled; daily cycle owns this work

YOYO-BATCH VM (us-central1-a, g2-standard-4, L4 24GB)
  └── llama-server.service       :8080  OLMo-3-32B-Think (loaded at boot, inference-ready)
        started by: startup-script (systemctl start llama-server.service)
        stopped by: yoyo-daily-cycle.sh Phase 8 (SSH sudo systemctl stop)
        cost:       ~$0.71/hr running; TERMINATED = $0.00

KILL SWITCH
  file: /srv/foundry/data/yoyo-disabled
  scope: checked by main() outer loop, ks_sleep() (30s granularity), and every 10s in Phase 4 loop
  activate:   touch /srv/foundry/data/yoyo-disabled
  deactivate: rm /srv/foundry/data/yoyo-disabled
  effect:     VM lifecycle stops within 30s; in-flight stint exits cleanly

DAY-BUDGET LEDGER
  file: /srv/foundry/data/yoyo-budget/<UTC-date>.consumed  (integer seconds)
  written: after each stint's VM:STOP (debit_seconds = vm_stop_epoch - vm_start_epoch)
  reset:   natural (date in filename; new day → new file)
  safety:  if consumed >= DAY_BUDGET_SECS-300s → exit; no VM started; prevents overspend on retries

TRAINING AUTHORIZATION
  Autonomous: touch /srv/foundry/data/training-approved/AUTONOMOUS_ENABLED  (set once; persists)
  Manual:     echo 'supervised' > /srv/foundry/data/training-approved/coding-lora-$(date -u +%Y-%m-%d).tag
  Receipt:    written to coding-lora-<date>.ran after each training run (SYS-ADR-19 traceability)
  Double-run guard: receipt checked before Phase 6; today's run skipped if receipt exists
```

---

### Deployed components inventory

| Component | Path | Status | Notes |
|---|---|---|---|
| `yoyo-daily-cycle.sh` | `/srv/foundry/bin/` | **UPDATED 2026-06-12 session-8** | Phase 4b DataGraph sweep (commit `c89e78e`): fires on stall, ingests git history to /v1/ingest, SHA ledger `/srv/foundry/data/yoyo-datagraph-sweep.ledger`; Phase 6 training fixes (commit `78ce725`): rsync script to `/home/mathew/run-dpo-training.py` + corpus to `/home/mathew/training-corpus/feedback/` on remote before SSH; receipt only on rc=0; earlier session-8: `budget_file()` dynamic date; `RETRY_DEADLINE_SECS` removed; `Restart=always` day-boundary mechanism; set-e fix (commit `9341778`) |
| `local-yoyo-daily.service` | `/etc/systemd/system/` | **UPDATED 2026-06-11 session-8** | Type=simple; Restart=always; RestartSec=600; RuntimeMaxSec=28800; KillMode=control-group; ExecStopPost; YOYO_DAY_BUDGET_MIN=120; enabled via WantedBy=multi-user.target |
| `local-yoyo-daily.timer` | — | **DELETED 2026-06-11 session-8** | Removed from source + /etc/; `Restart=always` in service replaces it |
| `yoyo-idle-monitor.sh` | — | **DELETED 2026-06-11 session-8** | Was: bin/yoyo-idle-monitor.sh; idle-monitor source files in infrastructure/yoyo-manual/ also deleted; installed units archived to /srv/foundry/data/yoyo-idle-monitor-archive/ |
| `local-corpus-threshold.timer` | `/etc/systemd/system/` | **MASKED** | → /dev/null; backup at `.timer.bkp` |
| `corpus-threshold.py` | `service-slm/scripts/` | **DEPLOYED** | kill switch added; commit `5ca1e6e0` |
| `lora-update.sh` | `service-slm/scripts/` | deployed (disabled) | Fixed VM/zone defaults; commit `5ca1e6e0` |
| `git-post-commit-hook.sh` | `service-slm/scripts/` | deployed | Install per archive; no archive has it yet |
| `capture-edit.py` | `/srv/foundry/bin/` | **ACTIVE** | Fix A deployed; real diffs in queue |
| `run-dpo-training.py` | `service-slm/scripts/` | **code-complete** | Quality fixes commit `135ce9ac`: LR 1e-4→5e-6; beta 0.1→0.5; output_dir -wip (fixes --resume); enrichment-only corpus loader (no apprenticeship mix); empty-rejected filter; ML libs INSTALLED in ~/training-venv on yoyo-batch |
| `export-sft.sh` | `service-slm/scripts/` | **COMPLETE** | Already existed; exports Alpaca SFT JSONL; `--dry-run` supported |
| `SLM_DRAIN_PAUSED` env var | `slm-doorman-server/src/main.rs` lines 244–290 | **DEPLOYED** | Drain loop checks unconditionally; already in production |
| `service-content` binary | `/usr/local/bin/service-content` | **REDEPLOYED 2026-06-09** | Prompt-injection fix + schema normalization commit `62df887e`; sha256 `89c219d9`; 10/10 tests pass; 9,692 entities healthy |
| `start-yoyo.sh` | `service-slm/scripts/` | **HARDENED 2026-06-11** | KILL_SWITCH var + kill_switch_aware_sleep() (30s poll); kill-switch check at retry-loop top; post-start race fix (stop VM if kill switch activated while gcloud start in-flight) |
| `local-yoyo-daily.service` | `/etc/systemd/system/` | **HARDENED 2026-06-11** | --max-minutes 120 (2hr budget); TimeoutStartSec=7800 (hard kill for oneshot — RuntimeMaxSec has no effect); KillMode=control-group; TimeoutStopSec=120; ExecStopPost=/snap/bin/gcloud stop (independent kill authority — fires on OOM/hang/SIGKILL) |
| `yoyo-daily-cycle.sh` | `/srv/foundry/bin/` | **HARDENED 2026-06-11** | Phase 4 curl --max-time 5 (lines 132,150,179); Phase 6 SSH: timeout+ServerAliveInterval=30/CountMax=3; Phase 8: exit 3 + final gcloud stop on non-TERMINATED; pre-gate check: 90% enrichment when training gates skip |
| Logging (all 3 scripts) | `start-yoyo.sh`, `stop-yoyo.sh`, `yoyo-daily-cycle.sh` | **ADDED 2026-06-11 session-6** | LIFECYCLE_LOG → `/srv/foundry/data/yoyo-lifecycle.log` (was /var/log/ root:root unwritable); `[PHASE:START elapsed=]`; `[RATE entities=+N/60s pairs=+N/60s]`; `[TRAIN:STEP step= loss= epoch=]`; `[VM:START/READY/STOP online_secs= cost_est=]`; `[RETRY:ATTEMPT cycle= elapsed= result=]`; `[STOP:SSH type=CLEAN\|SSH_CONN_FAIL\|SSH_OTHER]` |

---

### Training corpus inventory (as of 2026-06-09)

| Dataset | Location | Count | Quality | Use |
|---|---|---|---|---|
| Engineering SFT tuples | `data/training-corpus/engineering/*/` | 1,410 | pre-Fix-A diffs empty — DO NOT USE raw | needs filter via `export-sft.sh` |
| Apprenticeship DPO tuples | `data/training-corpus/apprenticeship/shadow-capture/` | 225 valid | post-Fix-A; some still empty-diff | filter before use |
| Enrichment DPO pairs | `feedback/enrichment-*.jsonl` | **0 — corpus reset** | All 91 prior pairs were prompt-injection contaminated (86% had Tier B extracting prompt examples not document entities). Deleted 2026-06-09. Clean accumulation starts on next cycle with fixed service-content binary. |
| Training markers | `data/training-pending/*.json` | 14 (local) | today-dated; idempotent | waiting for GCS bucket + ML libs |
| Poison/degenerate | `queue-poison/` | ~78 | empty diffs, never dispatched | EXCLUDE from all training runs |

---

### What is NOT yet built (next steps, in order)

| # | What | Blocks |
|---|---|---|
| 1 | ~~Install ML libs on yoyo-batch~~ — **DONE** (2026-06-09): `~/training-venv` with trl 1.5.1 + peft 0.19.1 + transformers 5.10.2 + bitsandbytes; added ephemeral external IP for pip/HuggingFace internet access | ~~All training runs~~ |
| 2 | Create approval tag + first supervised training run (Phase 6 already wired; gates on tag + ≥50 genuine pairs) | Adapter v1 |
| 3 | ~~Uncomment `SLM_YOYO_WEIGHTS_GCS_BUCKET`~~ — **DONE**: already set in `/etc/local-doorman/local-doorman.env` | ~~GCS sync~~ |
| 4 | Move timer to 02:30 UTC (Command Session `/etc/` scope) | Night-time operation |
| 5 | ~~Eval gate + adapter registration (`eval-adapter.sh` + `data/adapters/registry.yaml`)~~ — **SCAFFOLDED 2026-06-10**: both files created; full pass@5 GPU eval in Phase 2 | ~~Safe promotion~~ |
| 6 | ~~Install `git-post-commit-hook.sh` in each active Totebox archive~~ — **DONE**: symlink at `.git/hooks/post-commit → /srv/foundry/bin/capture-edit.py` | ~~SFT capture~~ |

---

### Cost and budget reference

| Item | Rate | Notes |
|---|---|---|
| yoyo-batch RUNNING | ~$0.71/hr | g2-standard-4 + L4 spot, us-central1-a |
| Daily 90-min cycle | ~$1.07/day | when running; $0 when TERMINATED |
| Monthly (daily cycles) | ~$32/month | 30 days × $1.07 |
| Kill switch `yoyo-disabled` | $0 | instant cost control; one file |
| Unexpected 02:00 UTC run (pre-fix) | ~$0.85/event | root cause: corpus-threshold.timer (now masked) |

---

## §14 — Testing + Quality Audit Record

> **Purpose:** Document what was tested, what passed, what failed, and what was fixed. Entries
> are added at session end whenever quality work is done. Read before any training run to understand
> the current health of the pipeline. Newest entries on top.

---

### 2026-06-12 — Session 8: Continuous service model; Phase 4b; training path fixes

**Changes shipped:**
- `local-yoyo-daily.timer` deleted from source and `/etc/`; `Restart=always RestartSec=600` replaces it
- `RETRY_DEADLINE_SECS` (22h cap) removed; `start_vm_with_retry()` now retries STOCKOUT indefinitely
- `budget_file()` made dynamic (re-evaluates UTC date on each call; midnight-crossing safe)
- Five idle-monitor source files deleted (were already disabled session-7; source cleanup prevents confusion)
- Phase 4b added: DataGraph sweep fallback when corpus stalls — ingests 90-day git commit history from
  2 repos to `/v1/ingest`; SHA ledger `/srv/foundry/data/yoyo-datagraph-sweep.ledger` prevents double-ingest
- Phase 6 bug 1 fixed: `set -e` killed `main()` on STINT_RETURN=7 before `rc=$?` captured it; fixed with
  `run_stint "$stint_cap" && rc=$? || rc=$?` (commit `9341778`)
- Phase 6 bug 2 fixed: training SSH ran remote path `/srv/foundry/...` which doesn't exist on yoyo-batch;
  fixed by rsyncing script+corpus to remote before SSH (commit `78ce725`)
- Phase 6 bug 3 fixed: receipt written even on SSH failure (pipe always exits 0 from `while` loop);
  fixed with `{ ssh... } | while... && training_rc=0 || training_rc=$?`; receipt only on rc=0

**Test result (2026-06-12 00:14 UTC, PID 1085006):**
- Phase 4 stall detected correctly at 1570s elapsed (12 zero-delta intervals)
- Phase 4b did NOT fire — script loaded from pre-commit version; confirmed expected (bash compiles
  function defs at startup; Phase 4b was committed AFTER this PID launched)
- Phase 5+6 ran normally; training attempted but SSH failed (script not on remote) — no receipt (fixed)
- Receipt deletion: false `coding-lora-2026-06-12.ran` manually deleted; next cycle will retry

**Current state (2026-06-12 00:52 UTC, PID 1137652):**
- New PID loaded fully fixed script (Phase 4b + rsync fixes)
- STOCKOUT on us-central1-a L4; retrying every 600s indefinitely
- Budget: 1738s consumed / 7200s daily; ~5462s remaining for Jun 12

---

### 2026-06-12 — Session 8 Quality Audit: first full automation cycle (00:52 stint, PID 1137652)

**Cycle verdict: PARTIAL.** The first complete yoyo-batch automation cycle ran CLEAN end-to-end — no crash, no force-kill, graceful `TERMINATED` shutdown — but produced near-zero training value: **+22 entities** (all from the ingestion sweep, not enrichment), **0 enrichment DPO pairs, 0 training runs**, for **$0.804 and ~68min billable GPU time**. The plumbing works; the yield does not. The system paid for a GPU and never trained on it.

**Two brief premises VERIFIED FALSE this session:**
- **Enrichment pairs are NOT zero.** 418 valid DPO pairs exist at `/srv/foundry/data/training-corpus/feedback/apprenticeship-git-commit-*.jsonl`. The brief's `find` for `enrichment-*.jsonl` returned 0 because **that filename prefix does not exist anywhere on the box** (verified: 0 files).
- **Markers are NOT zero.** `corpus-threshold.py` wrote fresh markers for BOTH adapters today — engineering **1410 SFT tuples @02:12Z**, apprenticeship **418 DPO tuples @02:17Z**. **20 markers** now accumulate in `training-pending/`.

**Phase 4b results — 400 commits ingested, 0 pairs (BY DESIGN, not a sweep bug):**
- Sweep ingested **400 commits** (foundry 1–200, project-intelligence 201–400); wrote 400 SHAs to `/srv/foundry/data/yoyo-datagraph-sweep.ledger`.
- service-content returns **202 Accepted immediately**; the Tier A→Tier B enrichment cascade runs **asynchronously** afterward via the file watcher.
- With Tier B Terminated, logged `[TIER-B] Circuit open — using Tier A results` ×5. A bare Tier A baseline is not a preference pair → **0 DPO pairs BY DESIGN**.
- **Two compounding code faults:**
  1. `yoyo-daily-cycle.sh:171` writes SHA to the ledger on the **202-QUEUE ACK**, not enrichment success — all 400 commits are permanently marked swept and will **NEVER re-enrich when Tier B returns**.
  2. **60s wait-for-pairs** (6×10s) far too short for an async cascade at ~1.7–2 tok/s — `new_pairs=0` is meaningless.

**Training result — SKIPPED (cause is NOT "zero markers"):**
- Phase 6 of the 00:52 stint got **0s budget** — earlier 1738s + slow boot + dead 25min enrichment + 31min sweep exhausted the 7200s/day window before Phase 6. Two adapters READY (1410 SFT + 418 DPO, 8–28× over threshold) and nothing trained.
- Separate 02:32 training cycle hit **3× STOCKOUT** (02:36/02:48/03:00, 600s backoff).
- **`SLM_YOYO_WEIGHTS_GCS_BUCKET` unset everywhere in `infrastructure/`** (verified 0 grep hits) — `corpus-threshold.py` takes the `marker local only` branch; 20 markers pile up with no consumer even without stockout.

**Root cause — zero pairs:** Tier B (yoyo-batch L4) was Terminated the entire window. Doorman logged `consecutive_failures=102`, `route_yoyo_only: circuit not allowing request`. A Tier-A-only sweep **cannot produce DPO pairs by design**. The `/readyz` `circuit: closed` field is misleading — `health_up` is false.

**Additional findings:**
- Stall early-exit fired at **12/6, not 6/6** — ~15min burned past stall point on dead GPU (0% util, `metrics:n/a`).
- `entities=?` read failure on cycle log line 34 — unguarded healthz parse error.
- 8080 drain-worker upstream fault: `/health` ok but `/metrics` + `/v1/chat/completions` fail; **737-entry quarantine, 9 poison** blocking apprenticeship shadow-capture.
- **709/1127 apprenticeship tuples (63%) degenerate** (empty `attempt.diff`, Tier B unavailable at capture).
- VM `timedatectl` is America/Vancouver (PDT/-0700) — `journalctl --since` UTC queries return "No entries"; recurring 7h off-by analysis trap.

**Action items:**
1. **[BLOCKER — operator/Command]** Restore Tier B GPU endpoint. No DPO pairs and no training without it. No `--enable-zone-fallback`.
2. **[HIGH]** Fix Phase 4b ledger (`yoyo-daily-cycle.sh:171`) to write SHA on enrichment **success**, not 202-queue ACK — 400 commits poisoned.
3. **[HIGH]** Set `SLM_YOYO_WEIGHTS_GCS_BUCKET` or document that training dispatch is manual pre-D4.
4. **[HIGH]** Reserve a training-budget floor when adapters reach READY.
5. **[MED]** Fix stall early-exit to fire at 6/6; guard `entities=?` read failure.
6. **[MED — Command]** Fix 8080 drain-worker; audit 737-entry quarantine for replayability.
7. **[S improvement]** Expand `SWEEP_REPOS` to glob the 23 `clones/*/.git` (project-bim ~2532 commits, etc.).
8. **[S improvement]** Decouple Phase 4b `new_pairs` from the meaningless 60s wait (fire-and-forget).
9. **[S improvement]** Set VM clock to UTC (kills the recurring 7h off-by journalctl trap).

**Next-cycle forecast:** Another low-yield clean run — 0 DPO pairs (Tier B Terminated), 0 training (markers pile to 22+, no GCS bucket, stockout), ~$0.80 wasted. Entity yield will **shrink** (400 SHAs now permanently skipped). To make the next cycle productive, ALL THREE must hold: (1) yoyo-batch up in an L4-capacity zone; (2) 202-queue ledger bug fixed; (3) `SLM_YOYO_WEIGHTS_GCS_BUCKET` set OR training-budget floor reserved.

---

### 2026-06-11 — Session 7: Opus deep audit + preemption-resilient rewrite

**Method:** Opus agent performed full on-disk audit of all scripts (not from prompts/summaries).
Key corrections from the audit:

#### Ground-truth corrections from Opus audit (vs prior session summaries)

| Prior belief | Reality on disk (Opus verified) |
|---|---|
| start-yoyo.sh retry loop deployed | NOT in bin/; lives only in service-slm/scripts/ (never installed) |
| Two separate "worlds" to reconcile | No separate runner existed; one cycle script + one unmentioned idle-monitor |
| "0 DPO pairs" is a pipeline bug | Correct behavior: Tier A/B agree after normalization → no signal; fix = more Tier-B-up minutes |
| No source↔installed unit drift | Unit files were byte-identical; no drift |
| Idle-monitor was not mentioned | **CRITICAL FIND: yoyo-idle-monitor.timer ACTIVE, firing every 5min**, racing Phase 4 enrichment |

#### Five root causes found and fixed

| # | Problem | Fix |
|---|---|---|
| 1 | STOCKOUT fatal, not retried — 24h gap on failure | `start_vm_with_retry()`: ks_sleep(600s) + retry loop up to 22h |
| 2 | No day-budget ledger — retries would overspend ($4.26 for 3 attempts) | `/srv/foundry/data/yoyo-budget/<date>.consumed`; debited after each stint |
| 3 | No preemption recovery — mid-cycle preemption burns budget with no work | Phase 4 stall+VM-status detector (return 8); `main()` recovers with remaining budget |
| 4 | Idle-monitor races Phase 4 — between-batch lull triggers VM kill mid-enrichment | Disabled + archived; replaced by in-cycle stall detector (STALL_LIMIT=6) |
| 5 | Training gate blocks autonomous training — daily tag never auto-created | `AUTONOMOUS_ENABLED` flag (set once) + daily receipt for SYS-ADR-19 traceability |

#### Session-7 cycle output

- 02:31 UTC Jun 11 timer: STOCKOUT; 0 work. `cycle-20260611-023119.log` (45-min old unit).
- No yoyo-batch VM started successfully today; STOCKOUT since at least Jun 10 19:31 UTC.
- Retry loop (start-yoyo.sh --retry-until-epoch, PID 852424) was running but starts VM only — no enrichment.
- Tonight's timer (02:31 UTC Jun 12) will use the new rewritten script.

#### Verified post-implementation

| Test | Result |
|---|---|
| `bash -n yoyo-daily-cycle.sh` | PASS |
| Depleted budget → exits before gcloud | PASS (`Day budget spent 7500s/7200s`) |
| Kill switch → exits immediately | PASS |
| Unit file `systemd-analyze verify` | PASS |
| Timer still active after changes | PASS (next fire 19:30 PDT Jun 11 = 02:30 UTC Jun 12) |
| Only `local-yoyo-daily.timer` in systemctl list-timers | PASS (idle-monitor gone) |

---

### 2026-06-11 — Session 5: Opus shutdown audit + Fable holistic review

**Method:** Opus agent audited shutdown safety; Fable agent reviewed end-to-end pipeline correctness
against BRIEF design intent. All 5 Opus showstoppers fixed this session. Fable open items tracked below.

#### 5 showstoppers found and fixed (shutdown safety)

| # | Showstopper | Fix |
|---|---|---|
| 1 | No hard kill timeout — OOM/hang leaves VM running indefinitely | `TimeoutStartSec=7800` + `KillMode=control-group` (RuntimeMaxSec has no effect on oneshot — TimeoutStartSec is the correct mechanism) |
| 2 | No independent stop authority — OOM-killed script means Phase 8 never runs | `ExecStopPost=/snap/bin/gcloud compute instances stop yoyo-batch --zone us-central1-a --quiet` (absolute path mandatory for snap) |
| 3 | Phase 4 curl hang — 3 `$CONTENT/healthz` calls had no `--max-time` | `--max-time 5` added to lines 132, 150, 179 |
| 4 | Phase 6 training SSH hang — no client-side wall clock; NCCL/GPU deadlock blocks forever | `timeout $((TRAINING_SECS+120)) ssh -o ServerAliveInterval=30 -o ServerAliveCountMax=3` |
| 5 | Phase 8 silent exit 0 — non-TERMINATED VM indistinguishable from success | Retry gcloud stop + `exit 3`; ExecStopPost fires as backstop |

#### Fable pipeline review — open items (service-content + Phase 2 scope)

- **First broken link — Tier A 0-entity extraction** (OPEN): `write_enrichment_dpo_pair()` guard (commit
  `62df887e`) skips pairs when `tier_a_raw.is_empty()`. Tier A returns 0 entities every cycle. Combined:
  zero enrichment DPO pairs written → `run-dpo-training.py` loads 0 pairs → Phase 6 always skips.
  Root cause same class as apprenticeship fix `b84f8310` (format compliance). Fix in service-content scope:
  Tier A extraction prompt needs pre-fill or grammar constraint. Track in `service-content/NEXT.md`.

- **Corpus transport gap** (OPEN): Phase 6 SSH passes `--corpus $FEEDBACK_DIR` (workspace VM path). Nothing
  copies pairs to yoyo-batch before training; GCS sync is Phase 7 (after Phase 6). Add pre-training rsync
  step OR confirm path is network-mounted (currently undocumented).

- **Adapter-to-serving link not built** (OPEN — Phase 2): No PEFT→GGUF conversion, no llama-server
  `/lora-adapters` hot-swap wiring. Trained adapters land in `~/adapters/` on yoyo-batch and go nowhere.
  Also: `LORA_TARGET_MODULES` uses LLaMA-style names — verify against OLMo-2 architecture before first run.

- **Budget split corrected** (ACTIONED): Daily default changed to 90% enrichment / 10% overhead when
  training gates fail (implemented as dynamic pre-gate check). Weekly dedicated training day: 45/47/8 split.
  Training daily on a tiny corpus wastes spot time on load/checkpoint overhead; weekly `--resume` runs
  are strictly more efficient.

- **Signal starvation risk** (OPEN — strategic): At 4–8 enrichment pairs/cycle, corpus accumulation rate
  may not reach ~2K quality pairs by month 12. Automated quality gate between capture and training is a
  prerequisite for a sustained flywheel. app-orchestration-slm is NOT needed for the daily cycle —
  it is a multi-tenant broker for a later phase.

---

### 2026-06-10 — Session 4 Quality Audit (apprenticeship + enrichment corpus inspection)

**Method:** Live inspection of all corpus directories, log analysis, root cause tracing.

#### What was tested

| Test | Method | Result |
|---|---|---|
| Apprenticeship git-commit corpus count | `ls data/training-corpus/apprenticeship/git-commit/ \| wc -l` | 401 tuples present |
| Apprenticeship diff quality | Python quality check on 20 recent files | 143/300 DPO pairs have real diffs (48%) |
| OLMo 7B format compliance | Read corpus tuple + check reasoning/diff/escalate | **PARTIAL FAIL: reasoning present, diff often missing** |
| Corpus write guard | Trace `write_shadow_tuple` with escalate/diff state | **FAIL: `escalate && diff.is_empty()` guard was silently dropping tuples** |
| DPO feedback pairs | `ls data/training-corpus/feedback/*.jsonl \| wc -l` | 300 pairs |
| run-dpo-training.py dry-run | `--dry-run --corpus data/training-corpus/feedback/` | **FAIL: 0 pairs loaded (enrichment-only; apprenticeship files not loaded)** |
| Enrichment pairs | `ls .../feedback/enrichment-*.jsonl` | **0 files** |
| service-content extraction | `journalctl -u local-content.service` | **FAIL: `[TIER-A] 0 entities extracted` every cycle** |
| yoyo-batch availability | `gcloud compute instances start yoyo-batch --zone us-central1-a` | **STOCKOUT** |

#### Apprenticeship corpus tuple drop — FIXED

Root cause: OLMo 2 7B Instruct writes `## Reasoning` but skips the YAML frontmatter.
`parse_attempt_content` defaults `self_confidence=0.0` → `escalate=true` → blanks diff.
`write_shadow_tuple` guard: `escalate && diff.is_empty()` → skips corpus write.
Result: 100% of recent tuples silently dropped.

**Fix: commit `b84f8310`** — three-part fix:
1. Assistant pre-fill `---\n` → model continues from frontmatter
2. Remove diff-blanking when escalate is threshold-forced
3. Change write_shadow_tuple guard: skip only if reasoning AND diff are both empty

Also: max_tokens 512 → 1024 (7B uses ~230 tokens for reasoning; 512 was insufficient for diffs).

#### Enrichment corpus 0 entities — OPEN BLOCKER

`[TIER-A] 0 entities extracted` every cycle. Cause: OLMo 7B entity extraction prompt
produces no valid entities when Tier A runs the extraction via the Doorman.
When Tier B (yoyo-batch) is available, extraction works (4 entities written at 10:15 UTC).
But Tier B is STOCKOUT, and Tier A fallback gives 0 entities → no enrichment DPO pairs.

**This blocks all enrichment training.** The `run-dpo-training.py` only trains on `enrichment-*.jsonl`
files. With 0 files, Phase 6 will report 0 pairs and skip training.

**Investigation needed (service-content scope):**
- Why does OLMo 7B return 0 entities for service-content extraction requests?
- Is it the same format-compliance issue as apprenticeship? (missing JSON/YAML structure)
- Can the extraction request use assistant pre-fill or grammar constraint to force valid JSON?

**Workaround:** Wait for yoyo-batch L4 capacity to return; Tier B extraction works correctly.

#### Approval tag

Created: `data/training-approved/coding-lora-2026-06-10.tag` (143 good apprenticeship pairs).
NOTE: This tag is premature for enrichment training (0 enrichment pairs). Phase 6 will skip.
Useful for future: once enrichment pairs accumulate, tag is already in place.

---

### 2026-06-09 — Session 3 Quality Audit (adversarial agent review + corpus inspection)

**Method:** Adversarial agents (Opus) independently reviewed the training pipeline for defects.
Manual inspection of 10 corpus sample pairs. Dry-run execution on yoyo-batch.

#### What was tested

| Test | Method | Result |
|---|---|---|
| Dry-run corpus count on yoyo-batch | `run-dpo-training.py --dry-run` (pre-fix) | 91 pairs loaded — appeared healthy |
| Corpus content quality | Manual inspection of 10 random pairs | **FAIL: 86% prompt-injected** (see below) |
| Empty-rejected rate | Count of `rejected="[]"` across 91 pairs | **FAIL: 71/91 (78%) degenerate** |
| Corpus type separation | Check if apprenticeship-*.jsonl was loaded | FAIL: both types loaded (opposing gradients) |
| Learning rate | Review hyperparameters in run-dpo-training.py | FAIL: 1e-4 (20× too high for DPO) |
| beta parameter | Review with empty-rejected context | FAIL: 0.1 (causes hallucination bias with empty rejected) |
| output_dir for --resume | Trace output path with --resume flag | FAIL: dated dir breaks accumulation |
| VRAM budget for Phase 6 | Check llama-server VRAM vs training VRAM | FAIL: 16,151 MB already used; OOM on training |
| venv path in daily cycle | Check python3 call path in Phase 6 | FAIL: system python3, no trl |

#### Critical defect: EXTRACTION_SYSTEM_PROMPT prompt injection

The original prompt contained six named entity examples embedded directly in the instructions:
```
Person — named human individual. Example: "Jane Smith".
Company — registered organisation or business. Example: "Woodfine Management Corp.".
Project — named initiative, programme, or system. Example: "service-slm".
Location — geographic place or address. Example: "Vancouver".
```

When Tier B (OLMo 32B) received a document for extraction, it extracted these example names
FROM THE PROMPT INSTRUCTIONS rather than from the document text. Because the system prompt
prefix is identical across all documents, Tier B consistently returned "Jane Smith", "service-slm",
and "Vancouver" as extractions. All 91 pairs were contaminated — the chosen/rejected contrast
was between "examples + real entities" (Tier B) vs "real entities only" (Tier A), not between
better vs worse extraction. Training on these pairs would have taught the model to hallucinate
known example entities into every extraction.

**Fix committed:** `service-content/src/main.rs` commit `62df887e` — removed all named examples;
replaced with structural descriptions only; added explicit omit rule for terms appearing only
in the instructions.

#### Degenerate empty-rejected pairs

71/91 pairs (78%) had `rejected="[]"` — Tier A (OLMo 7B on CPU) found no entities in the document.
These are not genuine preference pairs. DPO loss on these pairs minimizes rejected-sample probability
toward an empty output, teaching the model verbosity preference rather than extraction accuracy.
Research confirms empty-rejected pairs degrade rather than improve model output (arxiv 2506.12725).

**Fix committed:**
- `run-dpo-training.py` commit `135ce9ac`: filter at corpus load time, log skipped count
- `service-content/src/main.rs` commit `62df887e`: guard in `write_enrichment_dpo_pair()` — skip pair
  if `tier_a_raw.is_empty()`, preventing degenerate pairs from being written to disk in future

#### Schema normalization fix

Tier A output includes extra hydration fields (`role_vector`, `location_vector`, `contact_vector`)
absent in Tier B's raw JSON response. Without normalization, nearly all pairs appeared to differ
on schema structure alone, not on extraction quality. The pairs were comparing different data
formats rather than different entity recognition results.

**Fix committed:** `service-content/src/main.rs` commit `62df887e` — `tier_a_raw` is normalized
to `{classification, entity_name}` only before comparison and serialization.

#### Actions taken

1. `EXTRACTION_SYSTEM_PROMPT` rewritten (commit `62df887e`) — structural descriptions, no examples
2. `write_enrichment_dpo_pair()` fixed — empty-rejected guard + schema normalization
3. `run-dpo-training.py` quality fixes (commit `135ce9ac`): LR 5e-6, beta 0.5, -wip output_dir, enrichment-only corpus loader, empty-rejected filter
4. 91 contaminated enrichment-*.jsonl pairs deleted; corpus = 0 on clean start
5. service-content redeployed with new binary (sha256 `89c219d9`); service healthy at 9,692 entities
6. Phase 6 VRAM fix: `yoyo-daily-cycle.sh` stops llama-server before training (commit `6d749df`)
7. Phase 6 venv fix: `yoyo-daily-cycle.sh` uses `~/training-venv/bin/python3` (commit `2f5c672`)
8. ML libs installed to `~/training-venv` on yoyo-batch: trl 1.5.1, peft 0.19.1, transformers 5.10.2

#### Post-fix dry-run result (yoyo-batch, after 135ce9ac + corpus reset)

```
[corpus] loaded 0 DPO pairs (0 format-skipped, 0 empty-rejected filtered)
[ERROR] No valid DPO pairs found — check corpus path and field names
```

Expected: corpus is empty after deletion. First genuine pairs will accumulate on the next daily
cycle (tonight at 17:00 UTC) using the fixed service-content binary.

#### Verified passing (post-fix)

| Test | Status |
|---|---|
| service-content 10/10 unit tests | PASS |
| service-content binary deployed + healthy (9,692 entities) | PASS |
| run-dpo-training.py --dry-run (empty corpus, correct error) | PASS (expected behavior) |
| Phase 6 VRAM path: llama-server stopped before training | PASS (code verified) |
| Phase 6 venv path: ~/training-venv/bin/python3 | PASS (code verified) |
| Empty-rejected pairs filtered at corpus load | PASS (code verified) |
| output_dir fixed to -wip (--resume accumulates correctly) | PASS (code verified) |

#### What to verify on NEXT cycle (2026-06-09 17:00 UTC)

1. New enrichment-*.jsonl pairs appear in the feedback dir (count > 0)
2. Spot-check one new pair: `prompt` field should contain only document text, not example names
3. Spot-check one new pair: `rejected` field should not be `"[]"` (the guard now prevents writing)
4. Dry-run after the cycle: `run-dpo-training.py --dry-run` should report N genuine pairs (N ≥ 1)
5. If 10+ good pairs accumulate, arm Phase 6 approval tag for first real training run

#### Remaining quality concern: LoRA target module names

`LORA_TARGET_MODULES = ["q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj"]`

These are standard LLaMA-architecture module names. OLMo-2 uses a different architecture.
**Not yet verified** that these module names exist in `allenai/OLMo-2-1124-7B-Instruct`.
Before the first real training run, confirm with:
```python
from transformers import AutoModelForCausalLM
model = AutoModelForCausalLM.from_pretrained("allenai/OLMo-2-1124-7B-Instruct", ...)
[n for n, _ in model.named_modules() if any(k in n for k in ["q_proj", "gate_proj"])]
```
If the list is empty, the LoRA will train zero parameters — a silent no-op. OLMo-2 may use
`att_proj`, `ff_proj`, or similar. Verify before committing the first training run.

---

### Standing protocol for YOYO sessions

Run at session start AND end (per operator instruction, 2026-06-09):

```bash
# DataGraph flow health
curl -s http://127.0.0.1:9080/readyz | python3 -c "import sys,json; d=json.load(sys.stdin); print(f\"tier_a={d.get('has_local')} tier_b={d.get('has_yoyo')}\")"
curl -sf http://127.0.0.1:9081/healthz | python3 -c "import sys,json; d=json.load(sys.stdin); print(f\"entities={d.get('entity_count')}\")"

# Corpus state
ls /home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer/service-fs/data/training-corpus/feedback/enrichment-*.jsonl 2>/dev/null | wc -l

# Training markers
ls /srv/foundry/data/training-pending/*.json 2>/dev/null | wc -l

# VM status (fast check, no start)
gcloud compute instances describe yoyo-batch --zone us-central1-a --format="get(status)" 2>/dev/null || echo "unknown"
```

---

## §15 — Session 9 research findings + as-built (2026-06-12)

### Research findings (3-agent parallel audit — pre-code-changes)

**OLMo-2-7B LoRA target modules — CONFIRMED CORRECT:**
Session 8 §14 flagged "not yet verified". Confirmed correct via HF Transformers source and
community testing: `q_proj, k_proj, v_proj, o_proj, gate_proj, up_proj, down_proj` are the
right module names for `allenai/OLMo-2-1124-7B-Instruct`. These match the standard attention
+ MLP projection names in the OLMo-2 architecture. LoRA will not be a silent no-op. ✓

**STINT_DEADLINE root cause of RC_1 at 02:32 and 04:04 UTC (Jun 12):**
The 02:32 and 04:04 RC_1 failures were NOT caused by slow startup. Root cause confirmed:
`STINT_DEADLINE` was computed at `run_stint()` start. After 6–8 STOCKOUT retries (~3600s
cumulative backoff), the deadline had already expired when the VM finally started. Phase 2
saw `NOW >= STINT_DEADLINE` on the first check and aborted with RC_1. The VM ran for 0
useful seconds while still consuming spot cost. Fix B addresses this.

**run-dpo-training.py glob pattern — the training BLOCKER:**
Line 65 used `glob("enrichment-*.jsonl")` which matched 0 files. All 435 real pairs use
the prefix `apprenticeship-git-commit-*.jsonl`. The `enrichment-` prefix was a forward-looking
design from session 5; no files with that prefix exist. Every training call since deployment
exited with "No valid DPO pairs found". Fix A resolves this permanently.

**tokenizer.padding_side = "right":**
With OLMo-2 and TRL DPOTrainer, left-padding (the default) causes position embedding
misalignment in the causal mask during DPO loss computation. This can produce NaN loss
silently on some batch shapes. Confirmed in TRL documentation and OLMo-2 usage notes.
Setting `padding_side = "right"` is the standard fix. Low-risk — does not affect inference,
only training collation.

**OLMo-2-32B on L4 (24GB VRAM):**
32B model needs ~20–22GB at bfloat16 inference. During training, gradient storage and
optimizer states push total VRAM above 24GB without `gradient_checkpointing=True`.
With gradient checkpointing + `per_device_train_batch_size=1` + `max_length=512`,
estimated VRAM usage: ~20–22GB — within L4 limit.

### 9 queue-poison entries — purged 2026-06-12

All 9 entries at `data/apprenticeship/queue-poison/` had `attempts=0`. The Doorman
pre-screened and moved them before any inference attempt. All were ops/docs commits
(MCP config deployments, JOURNAL edits, manifest rewrites) with no code training signal.
Purged in session 9. Queue-poison is now empty. New entries will only appear when
inference genuinely fails after retry exhaustion.

### As-built (session 9 — code changes)

**Commit:** session-9 (see git log)

**Files modified:**
- `service-slm/scripts/run-dpo-training.py` — Fix A (glob), Fix C (auto_verdict), Fix D (padding_side), Fix F (32B config)
- `bin/yoyo-daily-cycle.sh` — Fix B (STINT_DEADLINE propagation), Fix E (remote corpus pre-check)

**What was done:**
- Fix A: glob now matches both `apprenticeship-*.jsonl` + `enrichment-*.jsonl` — training will
  find all 435+ pairs on next cycle (**training blocker resolved**)
- Fix B: `STINT_DEADLINE` passed as arg to `start_vm_with_retry()`; STOCKOUT path checks if
  remaining budget supports a useful session before sleeping; returns 9 early if not
  (**RC_1 on STOCKOUT-exhausted deadline resolved**)
- Fix C: `auto_verdict` filter — only accepts `True` or absent; logs accepted/rejected count
- Fix D: `tokenizer.padding_side = "right"` — prevents potential NaN loss on OLMo-2
- Fix E: remote corpus count pre-check via SSH before issuing training call; skips if < 10 pairs
- Fix F: 32B conditional config — `gradient_checkpointing=True`, `batch_size=1`, `max_length=512`
  when `--base-model` contains "32B"

**What is NOT done (carry-forward):**
- Stage 6: promote `6a377cc` + session-9 commit — Command Session scope
- `SLM_YOYO_WEIGHTS_GCS_BUCKET` env var: still unset; training markers have no consumer
- Manual validation on yoyo-batch (operator): boot VM, rsync, dry-run, confirm ≥ 400 pairs found
- OLMo-3-32B-Think as teacher model (Job A CodeDPO): deferred until ≥ 200 enrichment pairs accumulate

---

## §16 — Session 10: Opus audit findings + code fixes (2026-06-13)

### Overnight run summary (Jun 13 UTC — 5 stints, no receipt)

| Stint | Phase 6 budget | Outcome |
|---|---|---|
| 1 (00:00) | 3163s | Phase 4b consumed 47min BEFORE training; training crashed: `max_prompt_length` TypeError |
| 2 (01:41) | 1128s | Crashed: `--resume True` ValueError (no checkpoint dir) |
| 3 (02:05) | ~300s | Budget fragment |
| 4 (02:33) | 413s | Model loaded, tokenization started, budget exhausted (150+ mismatch warnings fired) |
| 5 (02:56) | 158s | Skipped (< useful time) |

Day budget consumed: 7,018s / 7,200s. No receipt. Total yoyo-batch cost: ~$1.39.

### Key corpus state (Jun 13)

- 473 `apprenticeship-git-commit-*.jsonl` + 11 `enrichment-*.jsonl` = 484 corpus files
- run-dpo-training.py loads **472 DPO pairs** (1 format-skipped, 0 empty-rejected, 0 verdict-rejected)
- DataGraph: 10,654 entities (pre-cycle); entities grew to 10,662 (+8)
- Apprenticeship queue (live Doorman, Jun 13): 289 pending / 2,093 done / 1 poison / 737 quarantine
- Phase 4b: ingested 1,281 commits in 47 min (big sweep). Only 2 enrichment pairs produced (9→11).
  1,281 corpus files now backlogged in service-content watcher; pairs accrue over subsequent days.

### Four-agent Opus audit findings

**Agent A — Training quality:**

| Finding | Severity | Fix | Status |
|---|---|---|---|
| Tokenization mismatch: ~150+ TRL warnings; raw-string DPO format causes EOS token divergence in standalone vs concatenated tokenization; DPO loss boundary detection fails for ~35% of pairs | **P0** | Convert prompt/chosen/rejected to conversational message lists — TRL applies OLMo-2 chat template correctly in list format | **COMMITTED** `a6ccdf04` |
| BETA=0.5 stale: justification was empty-rejected pairs, now filtered; with clean corpus, 0.5 + 5e-6 LR + 47 steps = near-zero learning | **P1** | BETA 0.5→0.1 (DPO default); LR 5e-6→1e-5 | **COMMITTED** `a6ccdf04` |
| No `save_total_limit`: 9 checkpoints accumulate on spot disk (~1 GB each) | **P1** | `save_total_limit=2` | **COMMITTED** `a6ccdf04` |
| RuntimeCapCallback cap-exit receipt flow: confirmed CORRECT — trainer.train() returns normally, trainer.save_model() runs, script exits rc=0, receipt is written | OK | No change | — |

**Agent B — Enrichment throughput:**

| Finding | Severity | Fix | Status |
|---|---|---|---|
| SHA ledger written on 202-ingest-ACK (before Tier B runs): 1,237 of 1,281 commits permanently marked-done without Tier B processing; will never produce enrichment pairs | **P0 (architectural)** | Stop writing SHA to ledger in Phase 4b until per-document Tier-B completion is confirmed; OR use a separate pending→done reconciliation ledger | OPEN — tracked below |
| Sequential Tier B watcher: 44 docs/47min throughput ceiling (~63s per document); 2 pairs from 1,281 ingests is expected given this bottleneck | P1 | Extend Tier B uptime (primary fix) | Operator scope |
| String comparison for disagreement: `tier_a_json == tier_b_json` is array-order sensitive → false pairs; should compare canonical entity sets | P1 | service-content: sort + normalize entity arrays before comparison | OPEN |
| Phase 3 entity freeze unverified: +3 then flat could be empty queue or watcher hang; need corpus_dir count in log | P2 | Add `corpus_dir` file count to Phase 3 progress log | OPEN |

**Agent C — Cycle script:**

| Finding | Severity | Fix | Status |
|---|---|---|---|
| Phase 4b before Phase 6: overnight run lost 47min to sweep; training gets scraps | **P0** | Gate Phase 4b off when `_TRAINING_SECS > 0`; defer to enrichment-only stints | **COMMITTED** workspace `0a0e9f9` |
| No Phase 6 minimum budget guard: 158s/413s stints attempted training, failed fast, wrote no receipt, costing another full boot | **P1** | Skip Phase 6 if `_TRAINING_SECS < 600` | **COMMITTED** workspace `0a0e9f9` |
| No fast retry on training failure: each failed attempt costs a full boot+stall cycle (~13min overhead) | **P1** | Same-VM fast retry if rc!=0 and >=600s remaining and VM reachable; `--resume` for continuity; capped at 2 attempts | **COMMITTED** workspace `0a0e9f9` |
| `num_train_epochs=1` + `--resume` stops accumulating after epoch 1 completes: subsequent resumes are no-ops | Design | Decision needed: bump `num_train_epochs` (long run, fixed corpus) OR fresh-run-on-corpus-growth (N epochs over current corpus) | OPEN — operator decision |
| Corpus frozen-per-checkpoint constraint: if corpus changes mid-checkpoint-lineage, `train_test_split(seed=42)` shifts, max_steps mismatch with checkpoint | Design | Document; freeze corpus path for a full epoch before accepting new pairs OR use fresh-run approach | OPEN |

**Agent D — BRIEF audit:**

| Finding | Action | Status |
|---|---|---|
| BRIEF-slm-learning-loop.md was in archive/ (NOT deleted) — 1,337 lines, full history | Reactivated: `git mv` archive/ → briefs/, `status: active`, session-10 entry added | **DONE this session** |
| active-work BRIEF: service state table stale (2026-06-01 data) | Update §§ current state + §2 next items + §5 what-not-to-do | DONE this session |
| Misrouted BRIEFs (5): project-editorial, knowledge, data, software | Route to Command via outbox; do not self-resolve (cross-archive scope) | Outbox message pending |
| Technical direction: DPO at 472 pairs is below threshold where it beats SFT (arxiv 2603.20100: SFT beats DPO below ~1,410 pairs); format-contrast rejected not content-contrast | SFT-first is higher EV at current scale; DPO after Tier B restored + CodeDPO pairs generated | Design decision |

### Technical direction assessment (Opus audit verdict)

The learning loop pipeline engineering is mature. The training recipe has three issues:

1. **Wrong tool for scale:** DPO needs ~5K+ pairs for reliable signal; at 472 pairs, SFT on chosen diffs is higher EV. `export-sft.sh` is code-complete (§13). Recommendation: run SFT first; switch to DPO after Tier B restored + 200+ enrichment pairs accumulated.

2. **Format-contrast rejected:** chosen = full unified diff; rejected = short YAML/simplified fragment. Model learns "emit long diff-shaped text" not "prefer correct code." Genuine preference pairs need content-contrast (same format, different content). CodeDPO path (§10.3: cargo test validates chosen/rejected) is the fix — GPU-gated.

3. **Under-specified prompt:** commit message only (no file context). OLMo-2-7B cannot generate correct unified diffs from descriptions alone. Future: include touched-file slice in prompt.

**Jun 14 milestone is a plumbing milestone** (training completes without crashing). Quality milestone requires SFT-first or CodeDPO pairs.

### Commits this session

- `a6ccdf04` — project-intelligence: tokenization fix + BETA/LR + save_total_limit
- workspace `0a0e9f9` — Phase 4b gate + min budget guard + fast retry
- BRIEF reactivation + active-work update (this commit)

### Carry-forward (open items)

- [ ] **Stage 6**: promote `2b48bc75`, `06435048`, `3eed6cc4`, `a6ccdf04`, `2f8a6e9a`, `c2116094`, `159bbecd` (project-intelligence) + workspace `d94043f`, `0a0e9f9`, `917871f` — Command Session scope
- [x] ~~**Phase 4b ledger root-cause fix**: SHA written on 202-ACK before Tier B runs~~ — DONE: `mark_sweep_sha_complete()` in service-content (`159bbecd`) + Phase 4b shell fix (`917871f` workspace); ledger now written only after enrichment succeeds; needs `SERVICE_CONTENT_SWEEP_LEDGER` env var in `local-content.service` (Command outbox sent)
- [ ] **service-content binary rebuild + deploy**: after Stage 6; enrichment canonical sort fix (`2f8a6e9a`) not live until binary redeployed — Command scope (`deploy-binary.sh`)
- [ ] **SFT-first path**: deferred by operator (session 10b Q&A: plumbing milestone first); revisit after first DPO receipt + eval-adapter.sh baseline shows signal
- [ ] **Misrouted BRIEFs**: Command Session to relocate 6 foreign BRIEFs from this archive to owning archives (outbox sent)
- [ ] **`SLM_YOYO_WEIGHTS_GCS_BUCKET`**: set in service unit when GCS bucket ready
- [ ] **Tier B GPU restoration**: Tier B offline; enrichment pairs accrue slowly without GPU; primary constraint on enrichment quality and DPO pair volume
- [x] ~~**Enrichment string comparison**: fix canonical entity-set comparison in service-content/src/main.rs (sort before `==`)~~ — DONE commit `2f8a6e9a`
- [x] ~~**num_train_epochs decision**: operator to decide: bump epochs vs fresh-run-on-corpus-growth~~ — DONE: bumped to 3, commit `2f8a6e9a`; fresh-run-on-corpus-growth deferred until ~200+ enrichment pairs

### Session 10b — Operator Q&A decisions (2026-06-13)

Five outstanding design decisions resolved via structured Q&A with recommendations:

| Decision | Choice | Rationale |
|---|---|---|
| Phase 4b SHA ledger (1,281 commits blocked from enrichment) | **Clear it** — `rm /srv/foundry/data/yoyo-datagraph-sweep.ledger` | Ingest via `service-content` is idempotent (`worm_id` dedup); re-ingest is safe; commits become re-eligible for Tier B enrichment on next cycle |
| `num_train_epochs` accumulation (epoch 1 completes → `--resume` is a no-op) | **Bump to 3** — `NUM_EPOCHS = 3` in `run-dpo-training.py` | After epoch 1, `global_step == max_steps`; Trainer returns immediately on every subsequent nightly resume; 3 epochs = 3× gradient exposure same corpus; checkpoint lineage continues |
| SFT vs DPO at current corpus scale (472 pairs) | **Stay DPO** — deferred SFT | Plumbing milestone first; validate receipt + eval before switching training strategy; SFT path (`export-sft.sh`) remains code-complete and available |
| Enrichment canonical sort (false DPO pairs from array-order variance) | **Fix now** — committed `2f8a6e9a` | Only 11 enrichment pairs exist; cheap to fix before accumulation; normalization + sort prevents "same entities, different order" from producing spurious training signal |
| Misrouted BRIEFs (6 foreign BRIEFs in this archive) | **Leave to Command** | Cross-archive scope; outbox sent; Totebox must not write to other archives' `.agent/` |

**Epoch accumulation strategy (locked):** checkpoint lineage continues to epoch 3 via nightly `--resume`; trigger fresh-run-on-corpus-growth when enrichment pairs reach ~200+ (prevents `train_test_split(seed=42)` max_steps mismatch mid-lineage).
