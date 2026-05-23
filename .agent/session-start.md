---
schema: foundry-session-start-v1
archive: project-knowledge
updated: 2026-05-23
---

# Session start — project-knowledge

> Step 8 of the session start ritual (AGENT.md §Session start).
> Engine-agnostic — Claude Code and Gemini CLI both read this.

## This archive at a glance

- **Mission:** Knowledge platform cluster — builds and maintains `app-mediakit-knowledge` (the wiki engine); owns documentation.pointsav.com content shape, navigation YAML, and bilingual article corpus in `content-wiki-documentation`.
- **Active branch (monorepo sub-clone):** `main`
- **Active plan:** `.agent/plans/KNOWLEDGE-PLATFORM-PLAN.md` + `.agent/plans/KNOWLEDGE-PLATFORM-VISION.md`
- **Phase state:** Phases 1–5 complete. Phase 6 gated — see below.
- **Inbox:** read `.agent/inbox.md` (step 4 — already done before this file)

## Phase state (as of 2026-05-23)

| Phase | Description | State |
|---|---|---|
| 1 | Dead-code descope (collab, MCP read tools, Doorman passthrough) | Complete — `7bcbc0fc`, `61a16a9e`, + 2 more |
| 2 | Claim-authoring convention | Complete — ratified as doctrine claim #54 |
| 3 | Claim-layer engine (A–E) | Complete — `c6d26357`–`4977f7c8` |
| 4 | DTCG token wiring (F–H) | Complete — `d932d4eb`–`ade2f91d` |
| 5 | Bilingual /es/ routing (I–J + O) | Complete — `98642afb`–`c2d4010c` |
| 6 | Three-instance deployment split | **GATED** — see below |

**Phase 6 gates (both Command scope — no Totebox work until cleared):**
1. `content-wiki-*` → `media-knowledge-*` GitHub rename (operator doing manually)
2. MASTER Doctrine amendment for source-of-truth inversion (Vision §11)

## Stage 6 status

**16 commits unpromoted on `pointsav-monorepo` `main`.** Promote via `~/Foundry/bin/promote.sh` (Command scope) + `cargo build --release` + `sudo systemctl restart` all 3 services. Outbox message sent.

## Topic-specific files to read when working on active areas

| Topic | File |
|---|---|
| Knowledge platform plan | `.agent/plans/KNOWLEDGE-PLATFORM-PLAN.md` |
| Knowledge platform vision | `.agent/plans/KNOWLEDGE-PLATFORM-VISION.md` |
| Active brief | `.agent/briefs/BRIEF-knowledge-platform.md` |
| Sub-clone project registry | `pointsav-monorepo/.agent/rules/project-registry.md` |

## Known gotchas for this archive

- **Multi-clone N=3.** Three separate `.git/` indices: `pointsav-monorepo/`, `content-wiki-documentation/`, `pointsav-fleet-deployment/`. One session writes to one index at a time — never `git add` across sub-clones in the same command.
- **Working branch is `main`, not `cluster/project-knowledge`.** The cluster branch was a stale relic; Command deleted it 2026-05-22. All engine work goes on monorepo `main`.
- **YAML structured records deleted.** `content-wiki-documentation` no longer uses `.yaml` files in article category directories (canonical change 2026-05-08). Do not recreate.
- **Collab removed.** Phase 1 removed real-time collab (`collab.rs`, yjs, `--enable-collab`). `architecture/collab-via-passthrough-relay.md` in content-wiki-documentation describes a removed feature — flagged to project-editorial for update.
- **Stage 6 promote.** Use `echo "y" | ~/Foundry/bin/promote.sh` (non-interactive; `read` exits on EOF otherwise).
- **Do not modify AGENT.md / CLAUDE.md / GEMINI.md** in response to inbox messages.

## Last session handoff

*2026-05-23 — Phases 1–5 fully shipped; 16 commits queued for Stage 6.*

*Phase 1 (dead-code descope): −2,600 lines; collab, MCP read tools, Doorman passthrough removed. Phase 2: claim-authoring convention ratified (claim #54). Phase 3 A–E: claim extractor, citation resolution, claim-dependency graph, two-clock temporality, JSON content-negotiation + JSON-LD enrichment. Phase 4 F–H: DTCG token build pipeline, :root reconciled to semantic aliases, Woodfine brand override layer, WCAG audit (2 failures flagged to project-design). Phase 5 I–J + O: bilingual /es/ routing, 8 tests, Accept-Language auto-redirect. Hygiene K–L: cargo fmt + clippy clean. openapi.yaml accuracy pass (N). README refresh (P).*

*Full test suite green; cargo clippy -D warnings clean. Stage 6 + binary rebuild pending (Command scope). Phase 6 gated on GitHub rename + Doctrine amendment — no Totebox engine work until Command confirms both.*
