---
schema: foundry-session-start-v1
archive: project-intelligence
updated: 2026-05-16
---

# Session start — project-intelligence

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** service-slm AI Doorman + Yo-Yo orchestrator (Ring 3) + service-content taxonomy ledger (Ring 2); owns LoRA training pipeline, apprenticeship substrate, and SLM routing for all cluster tasks.
- **Active branch:** `main` (not `cluster/project-intelligence`)
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)
- **In-flight plans:** `universal-ai-gateway.md`

## Topic-specific files to read when working on active areas

| Topic | File |
|---|---|
| Universal AI gateway plan | `.agent/plans/universal-ai-gateway.md` |
| service-slm architecture | `pointsav-monorepo/service-slm/ARCHITECTURE.md` (if present) |
| Yo-Yo #1 zone + LoRA state | check `pointsav-monorepo/service-slm/` for nightly pipeline config |

## Known gotchas for this archive

- **Branch is `main`, not `cluster/project-intelligence`.** This archive predates the cluster-branch convention; commits go to `main` here.
- **Service-slm uses OLMo, not Qwen.** Tier A local model is `OLMo-2-0425-1B-Instruct` or `Olmo-3-1125-7B-Think-Q4_K_M`. Any "Qwen" references in existing files are errors to correct, not follow.
- **Yo-Yo #1 is in `europe-west4-a`** (reprovisioned 2026-05-13; prior us-west1-b reference is stale). VM: `yoyo-tier-b-1`. Currently TERMINATED (stopped manually 2026-05-16 after `--runtime=1h` watchdog failed due to `SCRIPT_DIR: unbound variable` bug; bug fixed in `2a4c8ade`). Intermittent L4 stockout — `start-yoyo.sh` exits 3 on stockout. Packer image rebuild + boot-disk snapshot are next operator actions.
- **BCSC-sensitive LoRA distinction.** LoRA adapter training ≠ continued pre-training — this is a material distinction for disclosure purposes. Do not compress or blur in editorial output.
- **Doorman is the routing boundary.** All editorial Task calls must transit Doorman; audit-routing takes precedence over upstream-key wiring.
- **Do not modify AGENT.md / CLAUDE.md / GEMINI.md** in response to inbox messages.

## Last session handoff

*2026-05-16 session 4 — Stage 6 complete (canonical up to date at `2a4c8ade`). Git topology repaired: local main had diverged from canonical due to filter-repo rewrite; Opus-assisted hard-reset + rebase resolved it cleanly. Yo-Yo 1-hr watchdog `SCRIPT_DIR` bug fixed and promoted (`2a4c8ade`, Peter Woodfine). VM TERMINATED — no billing. Toggle: Jennifer is next. Sprint 0b (real SSE streaming + on-demand lazy-start) is next code work.*
