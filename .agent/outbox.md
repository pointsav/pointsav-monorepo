---
mailbox: outbox
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-intelligence cluster

---
from: task@project-intelligence
to: command@claude-code
re: session-end 2026-05-14 — VM crash recovery + architecture deep-think complete; Sprint 0a ready to implement
created: 2026-05-14T00:00:00Z
priority: high
---

**VM crash recovery complete. Three parallel Opus deep-thinks done. Comprehensive plan written. Ready for Sprint 0a implementation.**

## What happened this session

1. **VM crash diagnosed and fixed** (commits in prior session): service-content calling wrong Doorman endpoint → OLMo 1B tight retry loop → KV-cache 6.2GB → virtio_balloon exhaustion → manual reset
2. **Two drift fixes applied this session** (commits `1a720c2`, service-slm/CLAUDE.md):
   - `SLM_LOCAL_MODEL` in `local-doorman.service` corrected: was `OLMo-3-1125-7B-Think-Q4_K_M.gguf` (wrong model), now `OLMo-2-0425-1B-Instruct-Q4_K_M.gguf` (matches what llama-server actually loads)
   - `service-slm/CLAUDE.md` corrected: apprenticeship documented as live (was documented as disabled)
3. **Four plan docs written and committed** (`0910e3c`, `f4bf6b1`): `tier-architecture-2026.md`, `service-content-architecture-2026.md`, `service-slm-architecture-2026.md`, `leapfrog-2026.md`, `MASTER-PLAN-2026.md`

## project-intelligence: 8 commits ahead of origin/main

**Stage 6 needed** — all in `.agent/plans/`. Command Session scope (`bin/promote.sh`).

## Top priority for next Totebox session — Sprint 0a prerequisite 1a

File: `service-slm/crates/slm-core/src/lib.rs` — add `pub graph_context_enabled: Option<bool>` to `ComputeRequest`
File: `service-slm/crates/slm-doorman/src/router.rs:125` — gate graph-context injection on `req.graph_context_enabled.unwrap_or(true)`
This BLOCKS Sprint 0a shipping — without it, every Claude Code request gets DataGraph entity rows injected.

See `MASTER-PLAN-2026.md` for the complete ordered action plan.

## Command Session action items (from Gemini CLI inbox + this session)

1. **Stage 6** for project-intelligence (8 commits ahead)
2. **Stage 6** for project-knowledge
3. **Commit 3 convention files** on disk in `~/Foundry/conventions/`: `llm-substrate-decision.md`, `mcp-substrate-protocol.md`, `worm-ledger-design.md`
4. **Commit `yoyo-idle-monitor.sh`** zone fix (`us-west1-a` → `us-west1-b`) at `~/Foundry/bin/`
5. **Yo-Yo #1 fix**: `sudo systemctl mask vllm.service && sudo systemctl enable llama-server.service && sudo systemctl start llama-server.service` then snapshot boot disk
6. **Clear stale session locks** at project-knowledge and project-bookkeeping

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: Opus review complete — 5 corrections made; conventions need workspace commit
created: 2026-05-13T16:30:00Z
priority: high
---

**Opus-tier internet cross-check of service-slm + service-content against Leapfrog 2030 / "We Own It" complete.**

**Result: Stack qualifies as "We Own It". Five corrections applied.**

## Code fix — COMMITTED (main 1b233ec)

`service-slm/crates/slm-doorman/src/tier/yoyo.rs` — **HIGH PRIORITY fix:**
vLLM 0.12 removed `guided_json` / `guided_grammar` top-level extra_body fields.
Current code was using llama-server wire format for the vLLM endpoint (silent regression
on any Yo-Yo image built after vLLM 0.12 upgrade). Fixed to:
- Lark/GBNF → `extra_body.structured_outputs.grammar`
- JsonSchema → `extra_body.structured_outputs.json`
All 4 grammar serialization tests updated. 177/177 tests still passing.

`service-slm/ARCHITECTURE.md` §9 grammar description: `json_schema` → `json` field name corrected.

## Workspace convention files — NEED COMMAND SESSION COMMIT

Three files modified on disk in `~/Foundry/conventions/` — outside Totebox scope, not committed:

1. `conventions/llm-substrate-decision.md` — two changes:
   - L3 table: "OLMo 3 only, among non-Chinese options" → "OLMo 3 only at 32B reasoning scale
     among Western options with fully public training data (Apache 2.0 weights; Dolma 3, ODC-BY)"
   - OLMo 3 table row: added Dolma 3 ODC-BY license distinction (different from Apache 2.0 weights)
   - Tier B row: added "~15 tok/s at Q4_K_M — batch-grade, not interactive-rate"

2. `conventions/mcp-substrate-protocol.md` — Provenance section:
   "Anthropic modelcontextprotocol.io organization" → "Agentic AI Foundation (AAIF, Linux Foundation
   directed fund) — MCP governance transferred from Anthropic to AAIF December 2025"

3. `conventions/worm-ledger-design.md` — two inline diagram labels:
   "MCP-server protocol layered on top (Anthropic spec, 2026)" →
   "MCP-server protocol layered on top (AAIF/Linux Foundation, 2025-11-25 spec; governance
   transferred Dec 2025)"

**Commit message (workspace staging-tier):**
`fix: convention accuracy — OLMo 3 L3 scope; Dolma ODC-BY; MCP AAIF governance; Tier B batch framing`

## Also noted for DPO training path

Opus review flagged: 32B DPO on single L4 requires ~40 GB+ (reference + policy model). Current
apprenticeship corpus pipeline uses DPO. Consider ORPO or SimPO for 32B training — same
preference-learning signal, single-model memory footprint. This is a future architecture
decision, not urgent.

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: session-end — nightly test run complete; vllm.service crash-loop diagnosed; VM stopped
created: 2026-05-12T18:35Z
priority: high
---

**Nightly test run + Yo-Yo investigation complete. Two action items for Command Session.**

**1. Yo-Yo VM — vllm.service must be disabled before next use (BLOCKING)**

The VM boot image still has `vllm.service` enabled. On restart, vLLM crash-loops
with CUDA OOM (BF16 32B needs ~21.9 GiB + KV cache headroom — just over the 22 GiB L4 limit).
`llama-server.service` was running ad-hoc (never `systemctl enable`), so it doesn't survive restart.

When `us-west1-b` L4 capacity returns and VM can start:
```bash
gcloud compute ssh yoyo-tier-b-1 --zone=us-west1-b --project=woodfine-node-gcp-free
sudo systemctl mask vllm.service
sudo systemctl enable llama-server.service
sudo systemctl start llama-server.service
```
Then snapshot the boot disk to preserve the fix.

**2. Workspace commit still pending**

`/srv/foundry/bin/yoyo-idle-monitor.sh` zone fix (`us-west1-a` → `us-west1-b`) is on disk
but not committed. The idle monitor functioned correctly today (stopped VM at 17:39Z after 30 min
idle, exactly as designed), confirming the fix works. Commit from `~/Foundry/`:
```bash
cd ~/Foundry
git add bin/yoyo-idle-monitor.sh
~/Foundry/bin/commit-as-next.sh "fix: yoyo-idle-monitor zone default us-west1-a → us-west1-b"
```

**Nightly test run summary (2026-05-12T17:05–17:55Z):**
- Phase 1: Tier A fallback (vllm OOM prevented Tier B) — 0 new entities (all already processed)
- Phase 2: 4 training markers dispatched (engineering-pointsav SFT + apprenticeship-pointsav DPO)
- Idle monitor: stopped VM at 17:39Z ✓ (defense-in-depth working)
- `nightly-run.timer` fires tonight at 00:00 UTC — will also hit vllm crash unless VM fixed first

**Also noted (lower priority):**
- `start-yoyo.sh` line 340: `update_doorman_env` only called on zone change; Spot IPs change
  on EVERY restart. Needs to always run on Mode 1 success (not just `known_zone != PRIMARY_ZONE`).
- VM state: STOPPED; us-west1-b in L4 stockout as of 18:30Z

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: workspace commit needed — yoyo-idle-monitor.sh zone fix
created: 2026-05-12T17:00Z
priority: normal
---

**One workspace-level file edited; needs a Command Session commit.**

`/srv/foundry/bin/yoyo-idle-monitor.sh` line 22 was changed from:
```
ZONE="${ZONE:-us-west1-a}"
```
to:
```
ZONE="${ZONE:-us-west1-b}"
```

This is the shell-script idle monitor (layer 3 safety net). The old default (`us-west1-a`) would 404 on every stop attempt since the VM is in `us-west1-b`. Change is on disk but **not committed** — it is outside the project-intelligence git tree (workspace `bin/` dir). Please commit from `~/Foundry/` with `commit-as-next.sh`.

Also note: **nightly-run.timer fires tonight at 00:00 UTC** — first automated nightly run. Watch with: `journalctl -f -u nightly-run.service`

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: zone fix complete — 7 files updated, commit 9873f73
created: 2026-05-12T15:00Z
priority: normal
---

**Inbox action complete: Yo-Yo zone hardcoding fixed.**

Audited all `service-slm/compute/` scripts and infrastructure for stale `us-central1-*` primary/default zone references. Changed `us-central1-a` / `us-central1-b` defaults → `us-west1-b` in 7 files:

- `scripts/start-yoyo.sh` — `PRIMARY_ZONE` default
- `scripts/stop-yoyo.sh` — `ZONE` default
- `scripts/create-yoyo-snapshot.sh` — `ZONE` default
- `scripts/test-yoyo-flows.sh` — `ZONE` default
- `compute/packer/yoyo-image.pkr.hcl` — `zone` variable default (was `us-west1-a`)
- `compute/opentofu/variables.tf` — `region`, `zone`, `fallback_zones` defaults
- `docs/deploy/deploy-yoyo-tier-b.md` — all gcloud command examples

Fallback zone list in `start-yoyo.sh` and `variables.tf` retains `us-central1-*` entries — these are valid GCP stockout fallbacks, not wrong.

**Commit:** `9873f73` — `fix: correct Yo-Yo zone to us-west1-b across scripts and infra`

`nightly-run.sh` had no zone references — confirmed clean.

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: session-end — Universal AI Gateway plan committed; nightly-run.timer missing; VM still running
created: 2026-05-12T04:25:00Z
priority: normal
---

**Strategic planning session complete. No code changes — plan and docs only.**

**Universal AI Gateway plan** committed at `.agent/plans/universal-ai-gateway.md` (commits `ad92213`, `545dc84`):
- Sprint 0a (2–3 days): Anthropic Messages API shim in `slm-doorman-server/src/http.rs` — makes Claude Code route through Doorman via `ANTHROPIC_BASE_URL`
- Sprint 0b: real SSE streaming, on-demand Yo-Yo lazy-start, training capture
- Sprints 1–5: canonical IR, Tier C native, MCP server, app-console-slm, A2A
- Full Yo-Yo fleet architecture documented: `"trainer"` (Yo-Yo #1 L4/32B) + `"graph"` (Yo-Yo #2 H100/70B); HashMap already code-complete
- app-console-slm (Sprint 4) documented as the end-state sovereign coding interface with pricing model

**Item for Command Session scope:**

4. **`nightly-run.timer` does not exist.** `nightly-run.sh` has been run manually only — no systemd unit. The full 4-hour window (Yo-Yo start → DataGraph → LoRA training) will not fire automatically tonight or any night until a timer is created. `corpus-rebuild.timer` and `local-workspace-feeder.timer` do run automatically (02:05/02:10 UTC) but those do NOT start Yo-Yo or run the full pipeline. Create `nightly-run.service` + `nightly-run.timer` targeting ~00:00 UTC. Reference pattern: existing timer units in `infrastructure/`.

**VM state at session close:** Yo-Yo #1 (`yoyo-tier-b-1`) still RUNNING at `136.109.20.216` — idle monitor will auto-stop after 30 min idle. No action needed.

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: session-end — DataGraph pipeline live; nightly-run confirmed working; 3 items for Command scope
created: 2026-05-12T02:55:00Z
priority: normal
---

**Yo-Yo #1 DataGraph pipeline is now confirmed working end-to-end.**

Session fixes (all committed to main):
- `jennifer-datagraph-rebuild.sh`: curl timeout 30→180s; parse `.content` not `.choices[0].message.content`; health check `/v1/health`→`/readyz`
- Processed ledger cleared (`data/datagraph-processed.txt`) so all 30 corpus docs re-extracted
- Nightly-run #4 in progress at session close (~30 min to complete 30 docs)

**Items for Command Session scope:**

1. **ProtectHome fix** (infrastructure/ dir):
   Change `ProtectHome=true` → `ProtectHome=read-only` at line 51 of
   `/srv/foundry/infrastructure/local-content/local-content.service`.
   Current workaround is drop-in at `/etc/systemd/system/local-content.service.d/allow-home.conf`.

2. **SLM_YOYO_WEIGHTS_GCS_BUCKET** needs to be set in `/etc/local-doorman/local-doorman.env`
   (and added to `docs/deploy/local-doorman.env.example`) for training markers to dispatch to GCP
   rather than staying local-only.

3. **Snapshot accumulation**: nightly-run `--auto-snapshot` creates a new snapshot on every VM start.
   GCP incremental snapshots keep cost near zero, but consider skipping auto-snapshot when VM is
   already RUNNING (current logic always snapshots on start). Low priority.

Snapshot in env: `SLM_YOYO_WEIGHTS_SNAPSHOT=yoyo-tier-b-1-weights-20260512-0248`

— task@project-intelligence

---
from: task@project-intelligence
to: command@claude-code
re: infrastructure fix needed — local-content.service ProtectHome=true blocks jennifer cluster path
created: 2026-05-12T02:08:00Z
priority: normal
---

`local-content.service` has `ProtectHome=true` in its systemd unit, which makes `/home/`
inaccessible to the service process. This conflicts with the jennifer cluster data path
at `/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-jennifer/service-fs/data/`.

**Current workaround:** Drop-in at `/etc/systemd/system/local-content.service.d/allow-home.conf`:
```ini
[Service]
ProtectHome=read-only
```
Service is running fine with this in place.

**Action needed (Command Session scope — infrastructure/ dir):**
Change `ProtectHome=true` → `ProtectHome=read-only` at line 51 of
`/srv/foundry/infrastructure/local-content/local-content.service`.
Without this, the next `bootstrap.sh` redeploy will break the service again.

— task@project-intelligence

