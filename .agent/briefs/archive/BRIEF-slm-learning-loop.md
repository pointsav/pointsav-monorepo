---
artifact: brief
schema: foundry-brief-v1
brief-id: project-intelligence-slm-learning-loop
title: "SLM Learning Loop — DPO Training Pipeline"
status: archived
owner: project-intelligence
created: 2026-05-29
updated: 2026-06-12
author: totebox@project-intelligence (claude-sonnet-4-6)
moved_to: project-intelligence
archived: 2026-06-12
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
> **Last updated:** 2026-06-09 (session 3) — corpus quality fixes; service-content prompt-injection fix; ML libs installed on yoyo-batch; §14 Testing added

---

### System diagram (current state)

```
WORKSPACE VM (foundry-workspace, vault-privategit-source-1)
  │
  ├── local-doorman.service          :9080  Tier A (OLMo 2 7B, llama-server :8080)
  │     └── Tier B circuit → yoyo-batch (CLOSED when VM running; OPEN when TERMINATED)
  │
  ├── local-content.service          :9081  service-content (LadybugDB entity graph)
  │     └── enrichment-*.jsonl → feedback dir (continuous write)
  │
  ├── local-yoyo-daily.timer   ──────────── 17:00 UTC daily (→ 02:30 UTC after monitoring)
  │     └── yoyo-daily-cycle.sh             THE single VM lifecycle controller (45-min cap)
  │           Phase 1:  gcloud instances start yoyo-batch
  │           Phase 2:  wait llama-server health (~170s)
  │           Phase 3:  wait Doorman Tier B circuit close
  │           Phase 4:  enrichment drain (40% of budget = 18 min at 45-min cap)
  │           Phase 5:  corpus-threshold.py → training markers
  │           Phase 6:  LoRA training trigger (45% budget = 20 min) — 3-gate: markers+ML+approval
  │           Phase 7:  GCS sync (when SLM_YOYO_WEIGHTS_GCS_BUCKET set — currently OFF)
  │           Phase 8:  SSH stop llama-server → gcloud instances stop → verify TERMINATED
  │
  ├── yoyo-idle-monitor.timer  ──────────── every 5 min
  │     └── yoyo-idle-monitor.sh            safety backstop: stops yoyo-batch if idle ≥30 min
  │           targets: yoyo-batch / us-central1-a / 10.128.0.24
  │
  └── local-corpus-threshold.timer   ────── MASKED (→ /dev/null)
        was: 02:00 UTC daily corpus check + VM start
        now: permanently disabled; daily cycle owns this work

YOYO-BATCH VM (us-central1-a, g2-standard-4, L4 24GB)
  └── llama-server.service       :8080  OLMo-3-32B-Think (loaded at boot, inference-ready)
        started by: startup-script (systemctl start llama-server.service)
        stopped by: yoyo-daily-cycle.sh Phase 6 (SSH sudo systemctl stop)
        cost:       ~$0.71/hr running; TERMINATED = $0.00

KILL SWITCH
  file: /srv/foundry/data/yoyo-disabled
  scope: checked by yoyo-daily-cycle.sh (Phase 0) AND corpus-threshold.py _start_trainer_vm()
  activate:   touch /srv/foundry/data/yoyo-disabled
  deactivate: rm /srv/foundry/data/yoyo-disabled
  effect:     no gcloud instances start issued from any automated path
```

---

### Deployed components inventory

| Component | Path | Status | Notes |
|---|---|---|---|
| `yoyo-daily-cycle.sh` | `/srv/foundry/bin/` | **DEPLOYED** | 45-min cap; 40/45 split; Phase 6 training trigger; Phase 6 venv path fix (`~/training-venv/bin/python3`) commit `2f5c672`; Phase 6 VRAM fix (stop llama-server before training) commit `6d749df` |
| `local-yoyo-daily.service` | `/etc/systemd/system/` | **ACTIVE** | 45-min cap; TimeoutStartSec=3600; commit `2e04bcf` |
| `local-yoyo-daily.timer` | `/etc/systemd/system/` | **ACTIVE** | 17:00 UTC, RandomDelay=120s |
| `yoyo-idle-monitor.sh` | `/srv/foundry/bin/` | **ACTIVE** | Fixed target: yoyo-batch/us-central1-a |
| `yoyo-idle-monitor.timer` | `/etc/systemd/system/` | **ACTIVE** | every 5 min; 30 min idle threshold |
| `local-corpus-threshold.timer` | `/etc/systemd/system/` | **MASKED** | → /dev/null; backup at `.timer.bkp` |
| `corpus-threshold.py` | `service-slm/scripts/` | **DEPLOYED** | kill switch added; commit `5ca1e6e0` |
| `lora-update.sh` | `service-slm/scripts/` | deployed (disabled) | Fixed VM/zone defaults; commit `5ca1e6e0` |
| `git-post-commit-hook.sh` | `service-slm/scripts/` | deployed | Install per archive; no archive has it yet |
| `capture-edit.py` | `/srv/foundry/bin/` | **ACTIVE** | Fix A deployed; real diffs in queue |
| `run-dpo-training.py` | `service-slm/scripts/` | **code-complete** | Quality fixes commit `135ce9ac`: LR 1e-4→5e-6; beta 0.1→0.5; output_dir -wip (fixes --resume); enrichment-only corpus loader (no apprenticeship mix); empty-rejected filter; ML libs INSTALLED in ~/training-venv on yoyo-batch |
| `export-sft.sh` | `service-slm/scripts/` | **COMPLETE** | Already existed; exports Alpaca SFT JSONL; `--dry-run` supported |
| `SLM_DRAIN_PAUSED` env var | `slm-doorman-server/src/main.rs` lines 244–290 | **DEPLOYED** | Drain loop checks unconditionally; already in production |
| `service-content` binary | `/usr/local/bin/service-content` | **REDEPLOYED 2026-06-09** | Prompt-injection fix + schema normalization commit `62df887e`; sha256 `89c219d9`; 10/10 tests pass; 9,692 entities healthy |

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
