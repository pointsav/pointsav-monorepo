# Brief: Decompose PHASE-4-PLAN.md §1 into 8 queue-ready sub-agent briefs

**target**: Produce 8 individually dispatchable Sonnet sub-agent briefs for PHASE-4-PLAN.md Steps 4.1–4.8, each conforming to §1A's six rules, and stage them in `~/Foundry/.claude/sub-agent-queue.md` for Master ratification.
**target_files**:
- `/srv/foundry/.claude/sub-agent-queue.md` (create or append — queue destination)
- `/srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/docs/PHASE-4-PLAN.md` (read-only source)
- `/srv/foundry/clones/project-knowledge/pointsav-monorepo/app-mediakit-knowledge/docs/BP1-DECISION-PACKET.md` (read-only; Q1–Q7 decisions shape brief content)
**expected_output**: Eight formatted brief blocks appended to `sub-agent-queue.md`, each matching the 15-field shape defined in the Specification section below; a one-paragraph covering note per brief explaining which BP1 decisions are baked in vs. held as `[PENDING-BP1]` placeholders.
**max_response_lines**: 280
**model_tier**: sonnet
**parallelisable**: no (writes)
**confidence_gate_passes**: yes — BP1 cleared 2026-04-28 (operator answered all 7 via Master workspace pass v0.1.54); all 8 Phase 4 sub-step briefs can now be written to full specificity with concrete values (no `[PENDING-BP1-Q#]` tokens).
**layer_scope**: task
**anti_slop_check**: The queue entry for Brief 4.1 must name `src/git.rs`, `tests/git_test.rs`, and the J/P identity alternation requirement as concrete deliverables — not generic "implement git integration" prose.
**dependencies**: BP1 cleared (see BP1-DECISION-PACKET.md §9 for the 7 operator answers); the 8 Phase 4 sub-step briefs follow the plan's recommendations + the 4 operator decisions on Q1/Q3/Q4/Q7 (HTTP MCP transport / `--enable-mcp` off / outbox-first project-slm coordination / hand-author OpenAPI).

## Specification

The sub-agent reads PHASE-4-PLAN.md §1 in full and produces one brief per step. Each brief is a self-contained unit: it includes the exact file paths to create and modify, the required function signatures from the plan (e.g. `pub fn open_or_init`, `pub fn commit_topic`), the acceptance criterion verbatim from the plan (`cargo test` passes + the specific manual smoke described), and a `max_response_lines` cap that bounds the implementation to one commit's worth of output.

All 7 BP1 questions cleared 2026-04-28 by operator. The sub-agent writes all 8 briefs to full specificity with concrete values — no `[PENDING-BP1-Q#]` tokens. Concrete BP1 answers to embed: Q1 MCP transport = **HTTP on `/mcp`**; Q2 Git remote protocol = **smart-HTTP via the same axum server**; Q3 `--enable-mcp` default = **off**; Q4 Step 4.6 project-slm coordination = **outbox-first** (draft auth+rate-limit contract; outbox to project-slm Task; ~1 session round-trip; iterate; implement); Q5 gix-vs-git2 = **mixed: git2 write side, gix read side**; Q6 libgit2-dev = **bundle with libssl-dev in PK.3 single Master pass**; Q7 OpenAPI 3.1 = **hand-author for Phase 4**. Brief for Step 4.6 (MCP) embeds outbox-first sequencing — the first deliverable is the outbox message to project-slm Task with the auth+rate-limit contract draft, NOT immediate implementation. Brief for Step 4.8 (OpenAPI) specifies hand-author with no codegen tooling.

The dependency ordering from the plan must be reflected in the queue: Steps 4.1 is a prerequisite for 4.2, 4.2 for 4.3, 4.1 for 4.4, 4.4 for 4.5. Steps 4.6 and 4.7 depend on 4.1 for the `AppState` shape. Step 4.8 depends on all preceding steps because it documents their routes. Each brief's `dependencies:` field names these explicitly, not just "previous steps."

Each brief includes the `cargo check` discipline from the plan's §9: run from inside `app-mediakit-knowledge/`, not from workspace root, to avoid the `openssl-sys` drag. This is a mandatory line in every brief's specification, not an optional note. The cluster feature branch is `cluster/project-knowledge`; briefs confirm this rather than assuming main.

The queue entries are appended under a `## Phase 4 — app-mediakit-knowledge` heading in `sub-agent-queue.md`. If the file does not exist, the sub-agent creates it with a minimal header. Briefs are marked `[needs-master-ratification]` until the operator clears BP1 and Master reviews the `[PENDING-BP1-*]` tokens; Steps 4.1–4.5 and 4.7 are additionally marked `[ready-after-BP1-Q5-Q6-confirmed]` to signal they can be dispatched once those two low-controversy decisions are confirmed.

## Acceptance criteria

- All 8 briefs are present in `sub-agent-queue.md` under the Phase 4 heading.
- Each brief names at minimum: target, target_files (specific paths from the plan's §2 file map), expected_output, max_response_lines, model_tier (sonnet), parallelisable (no — all write), confidence_gate_passes, layer_scope (task), anti_slop_check (names a concrete deliverable), dependencies.
- Brief 4.1 names `git2 = "0.20"`, `src/git.rs`, `tests/git_test.rs`, and J/P alternation.
- Brief 4.6 names HTTP transport on `/mcp` (Q1), `--enable-mcp` off default (Q3), and outbox-first project-slm coordination as the FIRST deliverable (Q4) — auth+rate-limit contract drafted and outboxed before implementation begins.
- Brief 4.8 specifies hand-author OpenAPI 3.1 (Q7); no codegen tooling (e.g., utoipa) introduced in Phase 4.
- No brief omits the `cargo check` from `app-mediakit-knowledge/` discipline.
- The dependency graph in the briefs is consistent with the plan's sequencing (4.1 → 4.2 → 4.3; 4.1 → 4.4 → 4.5; 4.1 → 4.6; 4.1 → 4.7; all → 4.8).

## Risks / unknowns

- BP1-Q4 resolved as outbox-first 2026-04-28; Brief 4.6's first deliverable is the outbox draft to project-slm Task containing the MCP auth + rate-limit contract proposal. Implementation gates on project-slm reply.
- `rmcp` crate version listed as `"0.x"` in the plan — sub-agent should write `[PENDING-rmcp-version]` and Master patches at dispatch time with a `cargo search rmcp` check.
- The `gix-blame` crate version dependency in Brief 4.2 is listed as `"0.16"` in the plan with a parenthetical "or current"; sub-agent should carry this qualifier rather than hardcoding.
- The 8 produced sub-step briefs append to **this cluster's** sub-agent queue at `~/Foundry/clones/project-knowledge/.claude/sub-agent-queue.md` (per v0.1.30 §1A.4 layer-scope rule + Master's 2026-04-28 ratification correction), NOT Master's workspace queue. Append under existing `## Phase 4 — app-mediakit-knowledge` heading.
