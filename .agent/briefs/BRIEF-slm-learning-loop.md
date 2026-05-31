---
artifact: brief
status: active
title: SLM Learning Loop — Training Pipeline + Sovereign Coding Agent
created: 2026-05-29
author: totebox@project-intelligence (claude-sonnet-4-6)
companion: BRIEF-slm-substrate-master.md
grounds_in:
  - service-slm/ARCHITECTURE.md §Apprenticeship Substrate
  - conventions/apprenticeship-substrate.md
  - service-slm/crates/slm-doorman-server/src/main.rs (shadow drain worker)
  - DOCTRINE.md claims #49, #54
  - Anthropic ToS §2.c (hard boundary)
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
