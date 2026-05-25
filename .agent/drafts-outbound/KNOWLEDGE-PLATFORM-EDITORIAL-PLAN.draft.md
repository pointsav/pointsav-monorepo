# Knowledge Platform — project-editorial Execution Plan

> **PROPOSED DRAFT** — authored 2026-05-21 by project-knowledge for project-editorial.
> project-editorial reviews, adjusts with its own Track-A draft-state context,
> and commits this as its single editorial execution plan (suggested filename
> `KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md`). It re-bases `award-winning-wiki-overhaul.md`
> onto `KNOWLEDGE-PLATFORM-VISION.md` — it keeps that plan's substance (Track A
> method, Gate-0 reconciliations, Track D QA substrate, Track E coordination)
> and aligns it to the vision.
>
> **Type:** execution plan (downstream). **Upstream:** `KNOWLEDGE-PLATFORM-VISION.md`
> (in project-knowledge `.agent/plans/`). **Sibling:** `KNOWLEDGE-PLATFORM-PLAN.md`
> (project-knowledge's engine execution plan). The three synchronize per
> Vision §14 — this plan references the vision by section, never restates it.

---

## 1. What changed vs. `award-winning-wiki-overhaul.md`

The vision (a re-architecture: claim-native data model, pairing contribution
model, source-of-truth inversion, three-instance split) supersedes the blueprint
layer. Editorial work is unaffected at its core but re-based:

- **Main Page** — project-knowledge owns the artifact (Vision §5). project-editorial
  supplies *recommended lede drafts* and *reviews* the lede prose. Track A1 is
  reframed from "rewrite the ledes" to "draft + review."
- **Claim-native model** — the Top-12 TOPIC rewrites wait for the claim-authoring
  convention (project-knowledge Phase 2) and are rewritten *once* against it.
- **Repo rename** — commits route to `media-knowledge-*` (was `content-wiki-*`)
  once the rename + Doctrine amendment land.
- **Editorial standard** — the Gate-0-reconciled standard is canonical and is the
  single ruleset behind both the editorial linter and the engine's
  `validate_editorial_standards` (Vision §14 — one ruleset, two consumers).
- **Contribution model** — project-editorial is a privileged contributor + reviewer
  under propose-as-branch / review-as-diff / F12 (Vision §5).

## 2. Gate 0 — editorial standard (RATIFIED 2026-05-21, unchanged, canonical)

The five operator-ratified reconciliations stand and are the editorial contract:
expansion sentences ≤ ~45 words (disclosure prose ≤ 25); prefer active verbs
(present-fact mechanism only, no personification, no absolute `is/are/was` ban);
analogy is a ceiling (≤ 1 per 300 words); the Bloomberg 4-paragraph lede is the
nut graf and the Franklin arc governs body-section order only; the SaaS-marketing
register is rejected for public content. **This is the ruleset** — see §4 (D).

## 3. Track A — Editorial overhaul (executable in project-editorial)

Scope: the three content repos (`media-knowledge-{documentation,projects,corporate}`
after rename; `content-wiki-*` until then). Method unchanged from
`award-winning-wiki-overhaul.md` §4 (outline-first; section-by-section; gold
exemplars; separate critic pass; deterministic lint before staging).

- [ ] **A0 — Encode the standard.** Amend the style guides with the Gate-0 §2 rules. Do first.
- [ ] **A1 — Main Page ledes.** project-knowledge owns each Main Page. project-editorial *drafts recommended ledes* (Bloomberg 4-paragraph nut graf, Crisis-first, accordion rhythm) and hands them to project-knowledge as starting material; then *reviews* the lede prose when project-knowledge branches the Main Page. Not an editorial-owned rewrite.
- [ ] **A2 — Top-12 TOPIC rewrites.** **Sequencing: wait for the claim-authoring convention** (project-knowledge Phase 2 — near-term). Rewrite all 12 once, with claim markup included, against the convention — no double-touch. One bilingual EN+ES commit per TOPIC.
- [ ] **A3 — Acceptance criteria** per article, before staging (Gate-0 rules; wikilink re-audit; EN+ES pair).
- [ ] **A4 — Close-out** — wikilink audit; outbox the Stage-6 request; archive this plan section.

## 4. Track D — Editorial QA substrate (the ruleset + linter)

Unchanged in intent from `award-winning-wiki-overhaul.md` Track D. Vision §14
adds: the ruleset Track D produces is **the single source** consumed by both the
editorial linter *and* project-knowledge's engine `validate_editorial_standards`.

- [ ] **D1 — Editorial linter** (`editorial-lint.py`) — frontmatter schema, banned vocabulary, sentence-length budget, single body H1, section order, `.es.md` pair check.
- [ ] **D2 — Failure-mode registry** — the AI-writing-tells checklist; doubles as the Track A critic checklist.
- [ ] **D3 — Genre templates** — the ~16 genre skeletons.
- [ ] **D4 — Single canonical standard** — one home for the banned-vocabulary list + register rules. **This is the ruleset routed to project-knowledge** (Vision §14). Deliverable.
- [ ] **D5 — Apprenticeship loop** — joint with project-intelligence; needs an operator signing identity.
- [ ] **D6 — Manifest revision.**

## 5. Track E — Cross-cluster coordination

Unchanged from `award-winning-wiki-overhaul.md` Track E (project-intelligence
service coordination). Plus:
- [ ] **E-claim** — receive the claim-authoring convention from project-knowledge (their Phase 2.4); A2 proceeds against it.
- [ ] **E-ruleset** — route the Gate-0/D4 ruleset to project-knowledge for `validate_editorial_standards`.
- [ ] **E-rename** — adopt `media-knowledge-*` once the rename + Doctrine amendment land.

## 6. Engine & design — NOT editorial scope

`app-mediakit-knowledge` is project-knowledge's; design tokens are project-design's.
The old Track B/C handoff stubs are retired — see `KNOWLEDGE-PLATFORM-PLAN.md`
(project-knowledge) and route token questions to project-design. project-editorial
does not edit the engine or the design system.

## 7. Old-plan cleanup

This plan supersedes `award-winning-wiki-overhaul.md` and its inputs. Execute the
strict cleanup procedure delivered with this draft (outbox message
`project-knowledge-20260521-editorial-plan-handoff`).

## 8. Sequence

```
A0 (encode standard) → D1/D2 (linter + registry)
→ A1 (recommended Main Page ledes → project-knowledge)
→ [await claim convention] → A2 (Top-12, once, with claim markup)
→ A3/A4 → D3/D4/D6 → E-ruleset / E-claim
```

*Proposed 2026-05-21 by project-knowledge, re-basing project-editorial's
`award-winning-wiki-overhaul.md` onto `KNOWLEDGE-PLATFORM-VISION.md` rev 4.
project-editorial finalizes and owns this document.*
