---
artifact: brief
schema: foundry-plan-v1
slug: knowledge-platform-editorial-plan
status: active
phase: editorial
owner: project-editorial
created: 2026-05-21
last_edited: 2026-05-22
engines: [claude-code]
upstream: KNOWLEDGE-PLATFORM-VISION.md (project-knowledge)
sibling: KNOWLEDGE-PLATFORM-PLAN.md (project-knowledge)
adopted_from: project-knowledge/.agent/drafts-outbound/KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.draft.md (2026-05-21)
self_contained: true
supersedes:
  - award-winning-wiki-overhaul.md
  - INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md
  - MASTER_STRATEGY_AWARD_WINNING_WIKI.md
  - FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md
  - overhaul-documentation-pointsav-com.md
---

# Knowledge Platform — project-editorial Execution Plan

> **Owned by project-editorial.** Adopted 2026-05-21 from project-knowledge's
> proposed draft (`KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.draft.md`), finalized and
> committed by project-editorial as its single editorial execution plan. It
> re-bases the prior `award-winning-wiki-overhaul.md` onto
> `KNOWLEDGE-PLATFORM-VISION.md`.
>
> **Type:** execution plan (downstream). **Upstream:** `KNOWLEDGE-PLATFORM-VISION.md`
> (project-knowledge `.agent/briefs/`). **Sibling:** `KNOWLEDGE-PLATFORM-PLAN.md`
> (project-knowledge's engine execution plan). The three synchronize per
> Vision §14 — this plan references the vision by section, never restates it.
>
> **project-editorial finalization (2026-05-21):** the proposed draft was an
> overlay that referenced `award-winning-wiki-overhaul.md` for the Track-A
> method and the Track-D/E detail. That detail is now **inlined** here — this
> plan is self-contained and `award-winning-wiki-overhaul.md` is fully
> superseded. Two further adjustments to the draft: §3 operating posture
> re-inserted (a standing operator requirement) and §9 cleanup-timing note
> (deletions wait for explicit operator go-ahead).

---

## Status — 2026-05-23

**project-editorial's autonomous execution of this plan is COMPLETE.** Full
close-out record in §12; what follows is the at-a-glance state.

| Track | State |
|---|---|
| **A — editorial overhaul** | **Complete.** A0 (Gate-0 into style guides), A1 (3 recommended Main Page ledes drafted), A2 (12 flagship TOPIC rewrites, EN+ES, claim-annotated), A3 (acceptance criteria), A4 (close-out). Wikilink audit: **0 broken** across all 3 wikis. |
| **D — editorial QA substrate** | **Complete except D5.** D1 linter fully hardened 2026-05-23: claim-validation §9, banned-vocab sweep (19 files, 0 lint errors), ES projected-language fix, skip-dirs filter. D2/D3/D4/D6 done. D5 (apprenticeship verdict-signing) needs an operator signing identity. |
| **E — cross-cluster** | **Partial.** E1, E4, E-ruleset done. E2 / E3 / E5 / E-claim / E-rename are cross-cluster handshakes or operator-gated. |

**Pending — none of it project-editorial-executable:**
- **Stage 6 promotion** — Command (publish request in outbox, 2026-05-22).
- **D5** — operator signing identity.
- **E2 / E3 / E5 / E-claim / E-rename** — cross-cluster / the operator's GitHub repo rename.
- **A1 review pass** — when project-knowledge branches each Main Page.
- **Plan archival + §9 old-plan deletion** — operator go-ahead, post-ship.

The plan is kept `status: active` — archival is operator-gated (§9).

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

## 2. Gate 0 — editorial standard (RATIFIED 2026-05-21, canonical)

The five operator-ratified reconciliations stand and are the editorial contract:

| # | Rule |
|---|---|
| 1 | Sentence length: expansion sentences ≤ ~45 words; disclosure prose ≤ 25; accordion rhythm with mandatory short-punch sentences. |
| 2 | Prefer active verbs — present-fact mechanism description only; never for forward-looking claims (those keep planned/intended/may/target); no personification; no absolute `is/are/was` ban. |
| 3 | Analogy is a ceiling, not a quota — at most one per 300 words, optional. |
| 4 | The Bloomberg 4-paragraph lede is the nut graf (first ~10%); the Franklin arc (Crisis→Quest→Breakthrough) governs body-section order only. |
| 5 | The SaaS-marketing register is rejected for public content; "Liquid Glass" is an internal codename only. |

**This is the ruleset.** Track A0 encodes it into the style guides; Track D4
makes it the single machine-readable source — see §5.

## 3. Operating posture — services-optional, bypass by default

This plan has **no hard runtime dependency** on `service-content` or
`service-slm`. Both are enhancements, not gates. Every track operates on
git-tracked files and on-disk data; if a service is slow, hung, or down, the
work proceeds in degraded mode and the service interaction is queued — never
blocked.

- **service-content (Gravity Engine, `:9081`) — bypassed.** Track A reads and
  writes content files directly; vocabulary, glossary, and Domain checks use the
  on-disk sources, never a live HTTP call. The D1 linter is a pure file checker —
  no network calls.
- **service-slm (Doorman `:9080` / SLM `:8080`) — not in the editorial loop.**
  TOPIC rewrites are produced by the session against the §4 method. The SLM
  matters only for D5 (adapter training) and future automated validation —
  advisory, never gating.
- **Probe, don't wait.** Any optional service touch is a single
  `curl --max-time 3 .../healthz`; on no response, log "service unavailable —
  bypassed" and continue.
- **Queue, don't drop.** Service-dependent follow-ups go to the outbox or a
  handoff entry, actioned when the service returns. Plan progress is independent
  of service uptime.

*(Standing operator requirement, 2026-05-21.)*

## 4. Track A — Editorial overhaul (executable in project-editorial)

Scope: the three content repos (`media-knowledge-{documentation,projects,corporate}`
after rename; `content-wiki-*` until then). Every TOPIC commit is a bilingual
EN+ES pair; ES files are strategic adaptations, not 1:1 translations. Drafts
stage as `foundry-draft-v1` through the editorial gateway per
`cluster-wiki-draft-pipeline.md`; commit via `bin/commit-as-next.sh`.

### Method — getting better writing from AI

Every A1/A2 rewrite follows this pipeline (research basis: §11):

1. **Outline first.** Before prose, produce the section outline — heading +
   one-line intent + key claim per section. The Franklin arc governs section
   order (Gate 0 #4). Confirm the outline covers the source before drafting.
2. **Draft section by section** against the fixed outline — never one-shot.
3. **Gold exemplars.** Hold 3–5 already-strong, same-register articles as
   in-context reference passages; never mix registers in one set.
4. **Separate critic pass.** Review the draft as a copy editor — fresh role,
   against the §2 rules and the failure-mode registry (D2). Emit a structured
   defect list; revise; repeat at most twice.
5. **Deterministic lint before staging** — banned vocabulary, sentence-length
   budget, single body H1, frontmatter completeness (D1 linter).
6. **Positive-framed prompts**: state what to do, not what to avoid; place
   source text first and the instruction last.

### Track A items

- [ ] **A0 — Encode the standard.** Amend `reference/style-guide-topic.md` (+ `.es.md`) and `reference/editorial-language-registers.md` (+ `.es.md`) with the Gate-0 §2 rules; bump the style-guide `version:`. Do first — without it, A1/A2 drift.
- [ ] **A1 — Main Page ledes.** project-knowledge owns each Main Page. project-editorial *drafts recommended ledes* (Bloomberg 4-paragraph nut graf, Crisis-first, accordion rhythm; honour `guide-keep-the-home-page-the-gold-standard.md` anti-patterns) and hands them to project-knowledge as starting material; then *reviews* the lede prose when project-knowledge branches the Main Page. Not an editorial-owned rewrite.
  - [ ] A1.1 — documentation Main Page lede (EN + ES)
  - [ ] A1.2 — corporate Main Page lede (EN + ES) — check the `cluster-totebox-jennifer` DataGraph for corporate vocabulary first
  - [ ] A1.3 — projects Main Page lede (EN + ES)
- [ ] **A2 — Top-12 TOPIC rewrites — HELD pending the claim convention.** Wait for the claim-authoring convention (project-knowledge Phase 2.4 — near-term, degrades gracefully). Rewrite all 12 once, with claim markup included — no double-touch. One bilingual EN+ES commit per TOPIC; wikilink re-audit after each.

  | # | Article | Path (pre-rename) |
  |---|---|---|
  | 1 | compounding-substrate | `content-wiki-documentation/substrate/compounding-substrate.md` |
  | 2 | worm-ledger-design | `content-wiki-documentation/infrastructure/worm-ledger-design.md` |
  | 3 | machine-based-auth | `content-wiki-documentation/architecture/machine-based-auth.md` |
  | 4 | diode-standard | `content-wiki-documentation/architecture/diode-standard.md` |
  | 5 | apprenticeship-substrate | `content-wiki-documentation/substrate/apprenticeship-substrate.md` |
  | 6 | service-content | `content-wiki-documentation/services/service-content.md` |
  | 7 | service-slm | `content-wiki-documentation/services/service-slm.md` |
  | 8 | three-ring-architecture | `content-wiki-documentation/architecture/three-ring-architecture.md` |
  | 9 | doorman-protocol | `content-wiki-documentation/architecture/doorman-protocol.md` |
  | 10 | economic-model | `content-wiki-documentation/architecture/economic-model.md` |
  | 11 | topic-co-location-methodology | `content-wiki-projects/topic-co-location-methodology.md` |
  | 12 | topic-direct-hold-framework | `content-wiki-corporate/topic-direct-hold-framework.md` |

- [ ] **A3 — Acceptance criteria** per article, before staging: Bloomberg lede present; accordion rhythm, no expansion sentence > ~45 words; active-verb mechanism prose, forward-looking claims hedged; EN+ES pair committed together; wikilink re-audit clean; no SaaS-marketing register; no gold-standard anti-pattern violations.
- [ ] **A4 — Close-out** — final wikilink audit across all three wikis → 0 broken; outbox the publish request to Command (Stage 6, or the source-of-truth-inversion path once the Doctrine amendment ratifies); update this plan `status:` and archive it.

## 5. Track D — Editorial QA substrate (the ruleset + linter)

The cluster's enforcement layer is ~20% built — the manifest claims a
`service-disclosure` validator crate, frontmatter validators, ~16 genre
templates, and a closed apprenticeship loop that do not exist on disk. Track D
builds the layer that makes editorial quality mechanical and compounding.
Vision §14: the ruleset Track D produces is **the single source** consumed by
both the editorial linter *and* project-knowledge's engine
`validate_editorial_standards` — one ruleset, two consumers.

- [ ] **D1 — Editorial linter** (`.agent/scripts/editorial-lint.py`, self-contained, no network calls): frontmatter schema incl. the five research-trail fields, banned vocabulary, exactly one body H1, terminal section order (See also → References → External links), `.es.md` pair existence, sentence-length budget. Wire into `bin/draft-sweep.sh`.
- [ ] **D2 — Failure-mode registry** — versioned house list of AI-writing tells (sameness, hedging/false symmetry, negative parallelism, elevated filler, padding), each with example + fix. Doubles as the Track A critic checklist (§4 step 4). Do early.
- [ ] **D3 — Genre templates** — author the ~16 genre skeletons (template-topic, -guide, -readme, -memo, -architecture, …) with frontmatter stub + required-section headings + inline register reminders. Scaffold the `service-disclosure/` crate as their home.
- [ ] **D4 — Single canonical standard** — one home for the banned-vocabulary list + register rules; de-duplicate (currently defined twice). Write the missing `CORPUS-SCHEMA.md`. **This is the ruleset routed to project-knowledge** (Vision §14 / E-ruleset).
- [ ] **D5 — Apprenticeship loop** — joint with project-intelligence (their F12 gate + eval harness are built); run one verdict-signing session. Needs an operator signing identity — operator action; surface, do not self-execute.
- [ ] **D6 — Manifest revision** — `.agent/manifest.md`: `project-language → project-editorial`, `.claude/ → .agent/` paths, fix the stale `slm_endpoint: :8011 → :9080`, mark the `service-disclosure`/`service-language` legs as `leg-pending`.

### Target architecture — the editorial QA stack

Build in order; layers 0–2 deliver most value for least effort.

| Layer | Check | Gate or advisory |
|---|---|---|
| 0 | Frontmatter / research-trail schema | gate |
| 1 | Deterministic prose lint (banned vocab, H1, section order, sentence length) | gate on errors + advisory |
| 2 | Link + citation resolution | gate |
| 3 | Readability / sentence-variance metrics | advisory only — never gate on an absolute score |
| 4 | LLM-as-judge rubric scoring | advisory, human-ratified; pairwise framing; calibrated; abstains when unsure |
| 5 | Human editorial ratification | gate — honours SYS-ADR-19 (no automated AI publishing) |
| 6 | Periodic re-audit of the merged corpus | out-of-band — makes quality compound |

D1 is layer 1; layers 3–6 are larger follow-on builds, captured here as the target.

## 6. Track E — Cross-cluster coordination

Wiki content flows through two runtime services owned by **project-intelligence**
(renamed from `project-slm` 2026-05-05): `service-content` (the Gravity Engine)
and `service-slm` (Doorman + Tier A SLM). State 2026-05-21: `service-slm` is
healthy (`:9080`/`:8080`); `service-content` builds and deploys but is
runtime-hung (`:9081` not bound). These services are project-intelligence's
scope — this cluster does not fix them, and the overhaul does not depend on them
(see §3).

- [ ] **E1** — Outbox to project-intelligence: `service-content` (`local-content`) is runtime-hung — restart + diagnose. Informational; not a Track A blocker.
- [ ] **E2** — project-editorial owes project-intelligence a Do-Not-Use / taxonomy vocabulary ratification; three of their endpoints (RelatedTo edges, `POST /v1/editorial/grammar`, `/v1/editorial/seed`) are blocked on it. D4 produces it; E-ruleset routes it back.
- [ ] **E3** — Apprenticeship loop: project-intelligence has the F12 gate + eval harness; D5 is joint with them.
- [ ] **E4** — Inbound check: project-intelligence routed project-editorial 4 TOPIC drafts + 5 GUIDE specs + 4 CONVENTION proposals (commit `478c9465`). Locate and triage into Track A.
- [ ] **E5** — MCP reconciliation: `service-slm` already ships a live `slm-mcp-server`. The engine MCP is project-knowledge's; do not duplicate (Vision §8).
- [ ] **E-claim** — receive the claim-authoring convention from project-knowledge (their Phase 2.4); A2 proceeds against it.
- [ ] **E-ruleset** — route the Gate-0/D4 ruleset to project-knowledge for `validate_editorial_standards`.
- [ ] **E-rename** — adopt `media-knowledge-*` once the rename + Doctrine amendment land.

## 7. Engine & design — NOT editorial scope

`app-mediakit-knowledge` is project-knowledge's; design tokens are project-design's.
The old Track B/C handoff stubs are retired — see `KNOWLEDGE-PLATFORM-PLAN.md`
(project-knowledge) and route token questions to project-design. project-editorial
does not edit the engine or the design system.

## 8. Sequence

```
A0 (encode standard) → D1/D2 (linter + registry)
→ A1 (recommended Main Page ledes → project-knowledge)
→ [await claim convention] → A2 (Top-12, once, with claim markup)
→ A3/A4 → D3/D4/D6 → E-ruleset / E-claim
```

E1 + E4 (project-intelligence handoff + inbound triage) are cheap and can run
first, in parallel with A0.

## 9. Old-plan cleanup

This plan supersedes `award-winning-wiki-overhaul.md` and its blueprint inputs;
with the detail now inlined (§4–§6) the plan is self-contained and those files
carry no unique content.

**Timing — operator instruction:** superseded plans are deleted on explicit
operator go-ahead; the standing instruction is to delete after the overhaul
ships, not pre-emptively. The delete set, when authorized:
`award-winning-wiki-overhaul.md`, `INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md`,
`MASTER_STRATEGY_AWARD_WINNING_WIKI.md`, `FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md`,
`overhaul-documentation-pointsav-com.md`, `overhaul-gemini-analysis.md`,
`overhaul-progress.md`. The `archive/` folder and data/audit files are kept
(operator instruction).

## 10. Stop conditions

Stop and surface to the operator if: a Gate 0 rule does not cover an encountered
case; a TOPIC rewrite would require asserting a forward-looking claim as present
fact; a catalogue/link change would orphan inbound wikilinks with no clean
target; Track D work would require scaffolding a crate or touching another
cluster's scope without a handoff.

## 11. Research basis

The §4 method and Track D draw on a 2026-05-21 research sweep (multi-agent):

- **Eliciting better AI prose** — separate critic pass with a closed checklist
  (Self-Refine, ~20% measured lift); planning before prose; 3–5 gold exemplars;
  a named failure-mode registry; positive instruction framing; source-first
  prompt structure. Word-count targets are unreliable — control length
  structurally.
- **Measuring/enforcing quality** — a layered QA stack (§5). Deterministic lint
  gates defined defects; readability scores are trend-only, never an absolute
  gate; LLM-as-judge stays advisory, calibrated, pairwise, abstaining.
- **Substrate audit** — the manifest's `service-disclosure` crate, validators,
  16 templates, and closed apprenticeship loop are ~20% built; Track D closes
  the gap.

## 12. Execution record — close-out (2026-05-22)

project-editorial's autonomous execution of this plan is complete. What remains
is gated on other clusters or the operator.

**Track A — editorial overhaul: project-editorial execution complete.**
- A0 — Gate-0 standard encoded into the four style-guide files (`f646da2`).
- A1 — three recommended Main Page ledes (EN+ES) drafted and staged to
  `drafts-outbound/` for project-knowledge (`aefeee53`). The *review* half waits
  on project-knowledge branching each Main Page.
- A2 — all twelve flagship TOPICs rewritten (EN+ES), Bloomberg lede + Gate-0 +
  claim markup per claim-authoring-convention #54 (`d71f0c3`…`63d133a`).
- A3 — acceptance criteria applied inline per article.
- A4 — wikilink audit clean: **0 unresolved targets across all three wikis**
  (`wikilink-audit.py`, `19c64001`). Publish request routed to Command below.

**Track D — editorial QA substrate: complete except D5.**
- D1 editorial-lint.py, D2 failure-mode registry, D3 sixteen genre templates,
  D4 canonical standard + CORPUS-SCHEMA, D6 manifest revision — all committed.
- D5 (apprenticeship verdict-signing) — needs an operator signing identity;
  operator action, not project-editorial-executable.
- Follow-up: editorial-lint.py does not yet carry the claim-validation pass of
  claim-authoring-convention §9 — a Track D enhancement.

**Track E — cross-cluster:** E1, E4, E-ruleset done. E2 / E3 / E5 / E-claim /
E-rename remain — cross-cluster handshakes or operator-gated (the GitHub repo
rename).

**Publish + archival.** All A0/A2 commits sit on `main` in the three content
sub-clones, unpromoted; Stage 6 is Command's. **This plan is NOT archived.**
A4 of §4 says "archive it," but the operator's standing instruction (§9) gates
archival of superseded/closed plans on an explicit post-ship go-ahead. The two
conflict; per *surface conflicts, do not silently override*, the plan stays
`status: active` until Command issues the go-ahead.

---

*Adopted 2026-05-21 by project-editorial from project-knowledge's proposed
draft, re-basing `award-winning-wiki-overhaul.md` onto
`KNOWLEDGE-PLATFORM-VISION.md` rev 4; detail inlined for self-containment.
project-editorial owns this document. Track-A close-out recorded 2026-05-22.*
