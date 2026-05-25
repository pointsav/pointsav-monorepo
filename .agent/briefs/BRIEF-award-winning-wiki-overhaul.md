---
artifact: brief
schema: foundry-plan-v1
slug: award-winning-wiki-overhaul
status: archived
phase: editorial
owner: project-editorial
created: 2026-05-21
last_edited: 2026-05-21
engines: [claude-code]
supersedes:
  - INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md
  - MASTER_STRATEGY_AWARD_WINNING_WIKI.md
  - FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md
  - overhaul-documentation-pointsav-com.md   # completed 2026-05-16; delete with the three above after this overhaul ships
---

# Award-Winning Wiki Overhaul — Execution Plan

> Single source of truth and todo list for the "A-grade knowledge platform"
> overhaul of the three public wikis. Replaces the three blueprint documents
> and the completed prior overhaul plan — those four files are deleted once
> this overhaul ships (see §8).
>
> **This is a cross-cluster program.** Only Track A (editorial) is executable
> by a project-editorial Totebox session. Tracks B and C are handoff stubs.

---

## 1. Mission

Lift `documentation.pointsav.com`, `corporate.woodfinegroup.com`, and
`projects.woodfinegroup.com` from a clean-but-plain technical archive to an
A-grade institutional knowledge environment — Wikipedia-calibre structure,
Bloomberg-grade prose, and agent-legible markup — for a high-trust audience
(institutional finance, platform engineering).

The three wikis already passed a Bloomberg-standard overhaul (completed
2026-05-16): zero banned vocabulary, zero broken links, full bilingual pairs.
This overhaul is the **quality layer on top of that clean baseline**.

## 2. Inputs and what this plan does with them

| Input | Role here |
|---|---|
| `INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md` | UI/UX targets — folded into Tracks B/C |
| `MASTER_STRATEGY_AWARD_WINNING_WIKI.md` | Editorial + visual + agentic strategy — folded into all tracks |
| `FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md` | 4-phase checklist — folded into Tracks B/C |
| `overhaul-documentation-pointsav-com.md` | Completed prior overhaul — baseline only |

### 2.1 Reused assets from `drafts-outbound/archive-2026-04/`

Per operator instruction the archive folder is preserved. Four files are
load-bearing for this plan; the rest of the archive (the 16-file
`leapfrog-2030/` subtree + `guide-operating-yoyo.md`) is superseded and ignored.

| File | Use |
|---|---|
| `topic-wikipedia-structure.draft.md` | **Primary structural reference.** Infobox schemas, 3-tier quality-badge system, 16-element article anatomy, 9-panel Main Page table. Concrete spec behind Tracks B/C. |
| `guide-keep-the-home-page-the-gold-standard.md` | **Acceptance-criteria guardrail.** Format-invariant hard rules + 6 named anti-patterns. Governs Track A home-page work; overrides blueprint where they conflict (see §3, conflict 5). |
| `topic-home.draft.md` | Canonical home-page content + ENGINE rendering directives. Track A index work starts here, does not re-derive. |
| `topic-home.es.draft.md` | Bilingual sibling — strategic-adaptation pattern for the Spanish home page. |

### 2.2 Operating posture — services-optional, bypass by default

The overhaul has **no hard runtime dependency** on `service-content` or
`service-slm`. Both are enhancements, not gates. Every track operates on
git-tracked files and on-disk data; if a service is slow, hung, or down, the
work proceeds in degraded mode and the service interaction is queued — never
blocked. This holds the cluster's permanent posture: the editorial gateway
already runs as hand-refinement; live services accelerate it, they do not
enable it.

Concrete rules — every track obeys these:

- **service-content (Gravity Engine, `:9081`) — bypassed.** Track A reads and
  writes content files directly. Vocabulary, glossary, and Domain cross-checks
  use the on-disk sources (`vocabulary-banned-*.yaml`,
  `glossary-documentation.md`, `seeds/Domains.json`, `ontology/*.csv`) — never a
  live HTTP call. The Track D1 linter is a pure file checker and makes **no
  network calls**.
- **service-slm (Doorman `:9080` / SLM `:8080`) — not in the editorial loop.**
  TOPIC rewrites are produced by the session against the §4 method; they do not
  route through the SLM. The SLM matters only for Track D5 (adapter training)
  and future automated validation — both advisory and follow-on, never gating.
- **Probe, don't wait.** Where a track *could* use a service, it probes once
  with a short timeout (`curl --max-time 3 .../healthz`); on no response it logs
  "service unavailable — bypassed" and continues. No step blocks on a service.
- **Queue, don't drop.** Service-dependent follow-ups (runtime glossary patch,
  domain re-classification, adapter training) are written to the outbox or a
  handoff entry, to be actioned when the service returns. Plan progress is
  independent of service uptime.

### 2.3 Reconciliation with the Knowledge Platform Vision

project-knowledge has published `KNOWLEDGE-PLATFORM-VISION.md` (rev 3,
2026-05-21) — a settled foundation document for the three-wiki platform that
supersedes the blueprint trio (§2) on engine and architecture, and changes
this plan:

- **Main Page ownership → project-knowledge.** Each wiki's Main Page
  (`index.md`/`.es.md`, `featured-topic.yaml`, `leapfrog-facts.yaml`, category
  grid) is theirs. project-editorial's role narrows to a lede-prose review
  pass — Track A1 is reframed accordingly.
- **Repo rename.** `content-wiki-{documentation,projects,corporate}` →
  `media-knowledge-{documentation,projects,corporate}` (same git history;
  operator renames the GitHub repos). Track A/E paths migrate to the new names
  once the rename lands; the `content-wiki-*` sub-clone names still apply
  locally until then.
- **Source-of-truth inversion.** Each wiki's live instance content repo
  becomes canonical; GitHub is a downstream mirror. Content stops Stage-6
  promoting vendor→customer (engine code is unaffected). Gated on a Doctrine
  amendment requested from Master — treat as pending until ratified.
- **Tracks B and C are largely superseded.** The engine roadmap is governed
  by the vision §8–§10, not the blueprint's 4 phases. Track B is reduced to a
  coordination stub.
- **The editorial standard feeds their linter.** project-knowledge will build
  a `validate_editorial_standards` checker from project-editorial's standards.
  Track D's standard (D4) + linter ruleset (D1/D2) are its single source.

**Flag returned to project-knowledge:** the standard they adopt is the
**Gate-0-reconciled** one (§3 + the Track A0 output), not the raw blueprint
Lucidity Protocol — the 60-word sentences, the absolute `to-be` ban, and the
analogy quota were rejected in Gate 0.

## 3. Gate 0 — conflict ratification (RATIFIED 2026-05-21)

Research surfaced five conflicts between the blueprints and the
operator-ratified standards in CLAUDE.md §5/§6 and the existing wiki style
guides. Per CLAUDE.md §6 a project-editorial session **must not silently
adopt the blueprint** — these needed an operator decision.

**Status: all five resolutions below ratified by the operator 2026-05-21.**
They are now the editorial contract for Track A. A0 (encode the standard)
applies them verbatim to the style guides.

| # | Conflict | Recommended resolution |
|---|---|---|
| 1 | Blueprint "Lucidity" wants 30–60 word sentences; `style-guide-topic.md` caps at ~30, corporate register at 25. | Keep the accordion rhythm. Permit "expansion" sentences up to **~45 words** (not 60); short-punch sentences mandatory. Forward-looking/disclosure sentences keep the 25-word discipline. |
| 2 | Blueprint bans `is/are/was` and mandates sensory verbs ("the data betrays"); collides with BCSC posture + Bloomberg standard. | Prefer active verbs for **present-fact mechanism description only**. Never for forward-looking claims (those keep planned/intended/may/target). **No personification.** Downgrade the absolute `to-be` ban to "prefer active." |
| 3 | Blueprint mandates one analogy per 300 words; institutional register rejects folksy metaphor. | Treat as a **ceiling, not a quota** — at most one analogy per 300 words, optional, precision first. |
| 4 | Franklin arc (Crisis→Quest→Breakthrough) delays the consequence; `style-guide-topic.md` requires a Bloomberg 4-paragraph lede up front. | **Reconcile:** the Bloomberg 4-paragraph lede stays and *is* the nut graf (first ~10%). The Franklin arc governs **body-section ordering only**. |
| 5 | Blueprints use SaaS-marketing vocabulary ("Agentic UX", "Liquid Glass", "Outcome-Driven", "time-to-value"). | Gold-standard anti-patterns win. **No marketing register in public content.** "Liquid Glass" is an internal engineering codename only — never appears in wiki text. |

**Gate 0 status:** ratified 2026-05-21 — all five resolutions accepted as
written. Track A is unblocked.

---

## 4. Track A — Editorial overhaul (project-editorial — EXECUTABLE HERE)

Scope: `content-wiki-documentation/`, `content-wiki-corporate/`,
`content-wiki-projects/`. Every TOPIC commit is a bilingual EN+ES pair.
All drafts staged as `foundry-draft-v1` through the editorial gateway per
`cluster-wiki-draft-pipeline.md`. Commit via `bin/commit-as-next.sh`.

### Method — getting better writing from AI

Every A1/A2 rewrite follows this pipeline (research basis: §12):

1. **Outline first.** Before prose, produce the article's section outline —
   heading + one-line intent + key claim per section. The Franklin arc
   governs section order (Gate 0 #4). Confirm the outline covers the source
   material before drafting. Planning-before-prose prevents drift and padding.
2. **Draft section by section** against the fixed outline — never one-shot.
   Each generation is then a bounded, focused task.
3. **Gold exemplars.** Hold 3–5 already-strong, same-register articles as
   in-context reference passages. Never mix registers in one exemplar set.
4. **Separate critic pass.** After drafting, review the draft as a copy
   editor — fresh role, against the §3 ratified rules and the failure-mode
   registry (Track D2). Emit a structured defect list (location + defect +
   fix); revise; repeat at most twice. Stop when the critic finds nothing
   substantive.
5. **Deterministic lint before staging** — banned vocabulary, sentence-length
   budget, single body H1, frontmatter completeness (Track D1 linter).
6. **Positive-framed prompts**: state what to do, not what to avoid; place
   source text first and the instruction last.

### A0 — Encode the standard (do first, after Gate 0)

Without this, A1/A2 drift — the monthly DataGraph sweep regenerates against
the old Bloomberg standard and undoes the work.

- [ ] A0.1 — Amend `content-wiki-documentation/reference/style-guide-topic.md`
      (+ `.es.md`) with the ratified §3 rules.
- [ ] A0.2 — Amend `reference/editorial-language-registers.md` (+ `.es.md`)
      to match.
- [ ] A0.3 — Bump style-guide `version:` frontmatter; log in `cleanup-log.md`.

### A1 — Home-page lede review pass (3 wikis)

Per the Knowledge Platform Vision §5, project-knowledge **owns** each Main
Page; project-editorial does a **lede-prose review pass** — not a rewrite or
commit. When project-knowledge proposes a Main Page branch, review its lede
prose against the §3-reconciled standard (Bloomberg 4-paragraph lede as nut
graf, Crisis-first framing, accordion rhythm) and the anti-patterns in
`guide-keep-the-home-page-the-gold-standard.md`. Return review notes — and,
where useful, a recommended lede draft — via outbox; project-knowledge
commits. `topic-home.draft.md` is the reference for the target shape.

- [ ] A1.1 — documentation Main Page lede review (EN + ES)
- [ ] A1.2 — corporate Main Page lede review (EN + ES) — check the
      `cluster-totebox-jennifer` DataGraph for corporate vocabulary first
- [ ] A1.3 — projects Main Page lede review (EN + ES)

If project-knowledge has not yet proposed the Main Page branches, the A1
deliverable is the recommended lede drafts, sent to their outbox.

### A2 — Top-12 TOPIC rewrites — HELD pending the claim convention

**Status: HELD (2026-05-21).** project-knowledge confirmed (msg
`project-knowledge-20260521-editorial-plan-handoff`) that the claim-authoring
convention is near-term (their Phase 2.4) and degrades gracefully — claim-
annotated markdown renders fine on today's engine. Per their "convention-first"
answer to cross-check item 7, the Top-12 rewrites **wait** for that convention,
then all 12 are rewritten **once** with claim markup included — no double-touch.
project-knowledge routes the convention when it is specced; A2 does not start
until it arrives.

One TOPIC = one bilingual commit. ES files are strategic adaptations, not 1:1
translations. After each: run the wikilink re-audit before staging.

| # | Article | Path | Done |
|---|---|---|---|
| 1 | compounding-substrate | `content-wiki-documentation/substrate/compounding-substrate.md` | [ ] |
| 2 | worm-ledger-design | `content-wiki-documentation/infrastructure/worm-ledger-design.md` | [ ] |
| 3 | machine-based-auth | `content-wiki-documentation/architecture/machine-based-auth.md` | [ ] |
| 4 | diode-standard | `content-wiki-documentation/architecture/diode-standard.md` | [ ] |
| 5 | apprenticeship-substrate | `content-wiki-documentation/substrate/apprenticeship-substrate.md` | [ ] |
| 6 | service-content | `content-wiki-documentation/services/service-content.md` | [ ] |
| 7 | service-slm | `content-wiki-documentation/services/service-slm.md` | [ ] |
| 8 | three-ring-architecture | `content-wiki-documentation/architecture/three-ring-architecture.md` | [ ] |
| 9 | doorman-protocol | `content-wiki-documentation/architecture/doorman-protocol.md` | [ ] |
| 10 | economic-model | `content-wiki-documentation/architecture/economic-model.md` | [ ] |
| 11 | topic-co-location-methodology | `content-wiki-projects/topic-co-location-methodology.md` | [ ] |
| 12 | topic-direct-hold-framework | `content-wiki-corporate/topic-direct-hold-framework.md` | [ ] |

Items 1–10 cover the documentation wiki and overlap the existing 12-article
Featured-rotation pool; 11–12 deliberately extend coverage to the other two
wikis so the "three wikis" overhaul is visibly real.

### A3 — Acceptance criteria (per article, before staging)

- [ ] Bloomberg 4-paragraph lede present; Franklin arc governs body order.
- [ ] Accordion rhythm; no expansion sentence > ~45 words.
- [ ] Active-verb mechanism prose; forward-looking claims hedged; no personification.
- [ ] EN+ES pair committed together; ES is a strategic adaptation.
- [ ] Wikilink re-audit clean for any slug touched.
- [ ] No SaaS-marketing register; no gold-standard anti-pattern violations.

### A4 — Close-out

- [ ] Final `bin/draft-sweep.sh`-equivalent wikilink audit across all three wikis → 0 broken.
- [ ] Outbox message to Command Session: publish the refined content — via
      Stage 6, or the source-of-truth-inversion path (§2.3) if the Doctrine
      amendment has ratified by then.
- [ ] Update this plan `status:` and archive it (see §10).

---

## 5. Track B — Engine coordination stub (project-knowledge)

`app-mediakit-knowledge` (the wiki engine) is owned by **project-knowledge**.
**Superseded:** project-knowledge's `KNOWLEDGE-PLATFORM-VISION.md` §8–§10 now
governs the engine roadmap and supersedes the blueprint's 4-phase engine plan
(claim layer, MCP re-founding, descope). Do **not** hand project-knowledge the
blueprint tool list. Track B's live content is the cross-check reply and the
linter coordination (see the outbox reply + Track D). The table below is
retained as blueprint-era reference only — superseded by the vision:

| Item | State | Delta to hand off |
|---|---|---|
| Token injection (`:root{}` from design tokens) | EXISTS (`tokens.rs`, `server.rs:1063`) | Verify token-file path contract vs design-system layout only. |
| JSON-LD `TechArticle` (`jsonld.rs`) | EXISTS, tested | None. |
| `home_chrome()` slot order | PARTIAL (`server.rs:1008`) | Move Recent/`#mp-itn` block to *after* the category grid; update `tests/home_test.rs`. |
| MCP endpoint `POST /mcp` (`mcp.rs`) | PARTIAL — endpoint shipped | Add 3 blueprint tools: `fetch_architectural_intent`, `resolve_link_graph`, `validate_editorial_standards`. Last one needs a Lucidity rule engine. |
| 4-blueprint `type:` template logic | ABSENT | Add `type` to `Frontmatter` (`render.rs:25`); branch `wiki_chrome()` into Strategic/Operational/Definitional/Procedural. Largest build. |
| "Liquid Glass" CSS/SVG | ABSENT | Greenfield: `structural-physics.css`, refraction SVG filter, `.wiki-card-glass`, GPU perf. Internal codename only. |

**MCP note:** `service-slm` (project-intelligence) already ships a live
`slm-mcp-server` with 6 Foundry MCP tools. The blueprint's wiki-engine MCP is a
separate surface — the handoff must ask project-knowledge to **reconcile with
the existing SLM MCP, not duplicate it** (see E5).

- [ ] B.1 — Outbox handoff to project-knowledge with the table above.

## 6. Track C — Design-token handoff stub (project-design)

Design tokens are owned by **project-design** (`pointsav-design-system` +
media-assets). Route via outbox handoff. Current state vs blueprint:

| Item | State | Delta to hand off |
|---|---|---|
| 3-tier DTCG (primitive/semantic/component) | EXISTS | Verify-only. |
| `component.home-grid` (9-card) | EXISTS | Verify-only. |
| Typography tokens (Georgia / Linux Libertine / IBM Plex Mono) | EXISTS | Verify-only. |
| `wiki.*` namespace / `wiki.freshness-ribbon` | **STALE** | Blueprint terminology is out of date: 2026-05-07 co-sign renamed `wiki.* → knowledge.*` and **removed** freshness-ribbon tokens. Do **not** request these — flag the blueprint as stale. |
| Two `dtcg-bundle.json` files | CONFLICT | `tokens/` (old, `wiki.*`) vs `dtcg-vault/tokens/` (new, `knowledge.*`, feeds `dist/tokens.css`). project-design must declare which is canonical. |
| Brand primitives `#09090B/#111827/#869FB9/#F7F9FA/#164679` | PARTIAL | Present only in legacy `token-global-color.yaml`, not the DTCG `primitive.color` tier. Two unreconciled token systems — reconciliation is project-design's call. |
| `tokens/css/` consolidation | PARTIAL | Holds one stale file; live output is `dist/tokens.css`. |

- [ ] C.1 — Outbox handoff to project-design with the table above, explicitly
      flagging the stale `wiki.*` terminology and the dual-bundle conflict.

---

## 7. Track D — Build a better project-editorial (substrate)

The substrate audit found the cluster's enforcement and tooling layer is
roughly 20% built. The manifest claims a `service-disclosure` validator
crate, frontmatter validators, ~16 genre templates, and a closed
apprenticeship training loop — **none of these exist on disk.** Today every
editorial standard is prose a human must remember and apply by hand; that is
why the prior overhaul needed multiple manual sweeps to clear ~120
vocabulary leaks. Track D builds the layer that makes editorial quality
mechanical and compounding instead of dependent on a careful human each pass.

- [ ] D1 — **Editorial linter.** Start as `.agent/scripts/editorial-lint.py`
      (self-contained, fast): checks frontmatter schema incl. the five
      research-trail fields, banned vocabulary, exactly one body H1, terminal
      section order (See also → References → External links), `.es.md` pair
      existence, sentence-length budget. Wire into `bin/draft-sweep.sh`.
      Highest-leverage missing piece. The ruleset it enforces is the **single
      source** for project-knowledge's `validate_editorial_standards` checker
      (Knowledge Platform Vision §5/§8) — one ruleset, two consumers; do not
      let a second ruleset form.
- [ ] D2 — **Failure-mode registry.** A versioned house list of AI-writing
      tells — sameness, hedging / false symmetry, negative parallelism
      ("not X, it's Y"), elevated filler, padding — each with an example and
      the fix. Doubles as the Track A critic checklist. Small; do early.
- [ ] D3 — **Genre templates.** Author the ~16 genre skeletons (template-topic,
      -guide, -readme, -memo, -architecture, …) as real files: frontmatter
      stub + required-section headings + inline register reminders. Scaffold
      the `service-disclosure/` crate (this cluster's declared vendor leg) as
      their home; migrate the stranded design-system `ds-protocol-*.yaml`
      content in.
- [ ] D4 — **Single canonical standard.** Declare one home for the
      banned-vocabulary list + register rules; de-duplicate (currently defined
      twice — `style-guide-topic.md` prose vs `vocabulary-banned-*.yaml`).
      Write the missing `CORPUS-SCHEMA.md`.
- [ ] D5 — **Close the apprenticeship loop once.** Run one verdict-signing
      session so the promotion ledger goes live and the captured `prose-edit`
      tuples become training pairs. Joint with project-intelligence (the F12
      gate + eval harness live there — see E3). **Needs a signing identity —
      operator action; surface, do not self-execute.**
- [ ] D6 — **Manifest revision.** `.agent/manifest.md`: `project-language →
      project-editorial`, `.claude/ → .agent/` paths, fix the stale
      `slm_endpoint: :8011` → `127.0.0.1:9080` (the real Doorman endpoint), and
      mark the `service-disclosure` / `service-language` legs honestly as
      `leg-pending` with the D1/D3 plan attached.

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
| 6 | Periodic re-audit of the merged corpus | out-of-band — this is what makes quality compound |

D1 is layer 1. Layers 3–6 are larger follow-on builds, captured here as the
target, not committed scope.

## 8. Track E — Upstream services (project-intelligence coordination)

Wiki content flows through two runtime services owned by **project-intelligence**
(the cluster renamed from `project-slm` on 2026-05-05): `service-content` (the
Gravity Engine — Domain classification, glossary sync, datagraph grounding) and
`service-slm` (the Doorman + Tier A SLM behind the editorial gateway and adapter
training). These services are project-intelligence's scope — this cluster does
**not** fix them, and the overhaul does **not** depend on them being up
(see the §2.2 bypass posture). Track E records the coordination. State
2026-05-21:

- **service-slm — WORKING.** Doorman healthy on `127.0.0.1:9080`, Tier A SLM on
  `:8080`. The manifest's `slm_endpoint: :8011` is stale — corrected under D6.
- **service-content — runtime-hung.** Builds, deploys, taxonomy data intact, but
  the process is wedged (D-state); port 9081 never bound; the Doorman logs
  continuous "service-content graph unavailable". The classification / glossary
  / datagraph API is offline at runtime.

- [ ] E1 — Outbox to project-intelligence: `service-content` (`local-content`)
      is runtime-hung — restart + diagnose. The stale `lbug-build-blocker.md`
      masks it. Until fixed, this overhaul does **not** depend on live runtime
      classification — Track A operates against content files on disk, not the
      live graph. Informational handoff; not a Track A blocker.
- [ ] E2 — **project-editorial owes a deliverable.** Three project-intelligence
      endpoints (RelatedTo-edges substrate, `POST /v1/editorial/grammar`,
      `/v1/editorial/seed`) are blocked on an editorial Do-Not-Use / taxonomy
      vocabulary ratification; their outbox carries a ratification request to
      us. Produce the ratified vocabulary set as a deliverable of **D4** (single
      canonical banned-vocabulary standard), then send it back. Closes both sides.
- [ ] E3 — Apprenticeship loop: project-intelligence has built the F12 promotion
      gate (`bin/promote-corpus.sh`) and eval harness (`bin/eval-adapter.sh`);
      ~495 apprenticeship tuples captured, zero signed verdicts. **D5 is a joint
      action with project-intelligence**, not a solo operator step.
- [ ] E4 — Inbound check: project-intelligence routed project-editorial 4 TOPIC
      drafts + 5 GUIDE data/endpoint specs + 4 CONVENTION proposals (Phase 4,
      commit `478c9465`). Locate them (inbox / drafts-outbound) and fold into
      Track A scope or triage.
- [ ] E5 — MCP reconciliation: `service-slm` already ships a live
      `slm-mcp-server` (6 Foundry MCP tools). The blueprint's Track B "MCP
      endpoint on the wiki engine" is a *separate* surface — the Track B handoff
      must ask project-knowledge to reconcile, not duplicate. An
      editorial-validation tool may belong on the SLM MCP, not a new wiki-engine
      endpoint.

## 9. Sequence

1. **Gate 0** — ratified 2026-05-21.
2. **E1 + E4** — handoff to project-intelligence (service-content hung) +
   locate the inbound drafts. Early, cheap, unblocks triage.
3. **D2 + D1** — build the failure-mode registry and editorial linter; Track A
   leans on both.
4. **A0** — encode the standard into the style guides.
5. **B.1 + C.1** — send the two handoff stubs (parallel with A0).
6. **A1** — three home-page ledes.
7. **A2** — Top-12 TOPIC rewrites — **HELD** until project-knowledge routes
   the claim-authoring convention; then all 12 once, with claim markup.
8. **A4** — audit, outbox Stage 6 request.
9. **D3 / D4 / D6** — substrate build-out (follow-on). D4 produces the E2
   vocabulary deliverable for project-intelligence.
10. **E2 / E3 / D5** — close the cross-cluster loops with project-intelligence.
11. Archive this plan (see §10).

## 10. Old-plan cleanup (after this overhaul ships)

Per operator decision, once Track A close-out (§A4) is done, delete:
`INPUT-KNOWLEDGE-PLATFORM-BLUEPRINT.md`,
`MASTER_STRATEGY_AWARD_WINNING_WIKI.md`,
`FINAL_AWARD_WINNING_WIKI_EXECUTION_PLAN.md`,
`overhaul-documentation-pointsav-com.md`, and `overhaul-gemini-analysis.md` /
`overhaul-progress.md`. The `drafts-outbound/archive-2026-04/` folder is
**kept** (operator instruction).

## 11. Stop conditions

Stop and surface to the operator if: a Gate 0 resolution does not cover an
encountered case; a TOPIC rewrite would require asserting a forward-looking
claim as present fact; a catalogue/link change would orphan inbound wikilinks
with no clean target; Track D work would require scaffolding a crate or
touching another cluster's scope without a handoff.

## 12. Research basis

The Track A method (§4), Track D, and Track E draw on a 2026-05-21 research
sweep (four agents). Key findings:

- **Eliciting better AI prose.** The highest-leverage techniques: a separate
  critic pass with a closed, concrete checklist (Self-Refine — ~20% measured
  quality lift); planning before prose (outline-first, section-by-section);
  3–5 curated gold exemplars for register control; a named failure-mode
  registry; positive instruction framing over negation; source text placed
  first in the prompt. Word-count targets are unreliable — control length
  structurally instead.
- **Measuring and enforcing quality.** A layered QA stack (§7). Deterministic
  lint gates the *defined defects* (banned terms, broken links, schema gaps);
  readability scores (Flesch-Kincaid) are trend-only and must never gate on an
  absolute threshold — they penalise necessary domain vocabulary. LLM-as-judge
  is viable for subjective dimensions but must stay advisory, calibrated
  against human labels, use pairwise framing, and abstain when unsure.
- **project-editorial substrate audit.** The manifest describes a substrate
  (`service-disclosure` crate, validators, 16 genre templates, a closed
  apprenticeship loop) that is ~20% built — Track D closes that gap.
- **Upstream services (project-intelligence).** `service-slm` (Doorman + Tier A
  SLM) is healthy; `service-content` (Gravity Engine) builds and deploys but is
  runtime-hung. A two-way dependency exists: three project-intelligence
  endpoints are blocked on an editorial vocabulary ratification this cluster
  owes — Track E records it.
