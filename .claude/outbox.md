---
mailbox: outbox
owner: task-project-slm
location: ~/Foundry/clones/project-slm/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-slm cluster

Messages sent by this Task Claude to other layers. Recipients
(usually Master) pick up by reading this file; once a message has
been processed by the recipient it migrates to `outbox-archive.md`.

---

## 2026-04-27 — AS-2 decode-time constraint library decision (cross-cluster relay answer)

from: task-project-slm (cluster/project-slm, session 2026-04-27)
to: master (workspace)
re: AS-2 implementation path — constraint grammar library choice (answer to 2026-04-27T03:30Z relay)
created: 2026-04-27T18:00:00Z
priority: normal — cross-cluster decision; unblocks project-language Phase 1B

---

### Decision: Option A — llguidance

#### 1. Library selection

**Choose llguidance** (Microsoft Research, Rust crate).

**Rationale:**

- **Tier B integration:** vLLM Multi-LoRA at Ring 3 (Tier B GPU burst) natively supports llguidance constraints. No additional translation layer needed.
- **Rust-native:** service-slm is Rust; llguidance is a Rust crate. Direct integration via constraint enforcement on inference responses.
- **Production proven:** llguidance is widely used in 2026 production. Lark grammar syntax is industry-standard.
- **Ring 3 alignment:** Yo-Yo (vLLM instance) already accepts constraints per `infrastructure/slm-yoyo/CONTRACT.md`. llguidance `.lark` grammars serialize cleanly as constraint payload.

**Not chosen: Option B (Outlines)** — Python-first with Rust bindings adds language boundary; regex + `.lark` dual syntax complicates contract; vLLM natively supports llguidance, not Outlines.

**Not chosen: Option C (defer)** — Loses validation-at-author-time in project-language Phase 1B; extends timeline; architectural clarity supports deciding now.

#### 2. Approximate AS-2 timeline

**Full POST /v1/brief implementation: 3–4 weeks from grammar spec.**

- **Week 1:** Brief schema design + llguidance constraint engine integration
- **Week 2:** service-slm POST /v1/brief handler + Yo-Yo constraint passing + audit ledger extension
- **Week 3–4:** End-to-end testing, error handling, documentation

**Depends on:** project-language Phase 1B shipping banned-vocabulary `.lark` grammar by week 0.

**Milestone:** AS-2 testable with grammar enforcement by week 3.

#### 3. Cross-cluster contract surface

**Project-language should author the banned-vocabulary CFG in Lark format** (EBNF-like, industry-standard).

| Artefact | Format | Location | Owner |
|----------|--------|----------|-------|
| Banned-vocabulary grammar | `.lark` | `vendor/pointsav-monorepo/service-content/schemas/banned-vocab.lark` | project-language Phase 1B |
| JSON-Schema (if needed) | `.json` | Same directory | project-language Phase 1B |
| Usage documentation | `.md` | Same directory | project-language Phase 1B |

**Grammar spec rules:**
1. Use [Lark EBNF syntax](https://lark-parser.readthedocs.io/)
2. Name top-level rule `response` (or document in README)
3. Define terminals and banned words (example: `response: (ALLOWED_TOKEN)+` where `ALLOWED_TOKEN` excludes banned words)
4. Validate with `lark` Python package before shipping
5. Lark required (not regex alternative)

#### 4. Downstream unblocks

- project-language Phase 1B ships banned-vocabulary CFG without rewrite risk (grammar dialect is Lark; stable)
- schema-stable signal fires when Phase 1B lands, unblocking project-proofreader Phase 4
- service-slm AS-2 has clear integration path (llguidance client, Yo-Yo constraint passing)

---

After acting, relay this decision to project-language Task inbox and archive to `outbox-archive.md`.

— Task Claude (2026-04-27)
