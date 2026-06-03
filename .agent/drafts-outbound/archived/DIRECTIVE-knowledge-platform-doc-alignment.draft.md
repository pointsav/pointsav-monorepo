---
schema: foundry-draft-v1
artifact_type: DIRECTIVE
language_protocol: PROSE-DIRECTIVE
status: staged
created: 2026-06-01
created-by: totebox@project-knowledge
destination: project-editorial → media-knowledge-documentation (+ fleet-deployment guides)
reference: ".agent/briefs/BRIEF-knowledge-platform-master.md (project-knowledge) — SOURCE OF TRUTH"
research_done_count: 4
research_suggested_count: 0
open_questions_count: 1
research_provenance: "Consolidated from the 2026-06-01 research swarm (premium-docs UX: Stripe/Vercel/Linear/Apple/Tailwind/Supabase; editorial craft: Economist/Stripe Press/GOV.UK/Tufte/Butterick; mobile UX: Apple HIG/Material/GOV.UK/Minerva/web.dev) + current-state, mobile, and content-federation audits. Full record in the master brief."
research_inline: true
paired_with: "ES pairs required for each PUBLIC article updated (not for this internal directive)"
---

# DIRECTIVE — align knowledge-platform TOPIC/GUIDE/design docs to the master brief

**For:** project-editorial (owns committing into `media-knowledge-*` + fleet-deployment).
**Reference:** `BRIEF-knowledge-platform-master.md` in project-knowledge `.agent/briefs/` — read it
first; it is the single source of truth. This directive lists the precise doc changes it implies.

**BCSC posture (critical):** Phase 0 (federation engine) is **not yet built**. Describe federation,
mounts, blueprints, Cmd+K, and the new mobile chrome with **planned / intended / will** language
until the engine ships. State only already-shipped facts as current. Do not over-hedge shipped items.

**Bilingual:** every PUBLIC article updated needs its `.es.md` sibling updated in the same change.

---

## A. Urgent correction — typography drift (do this first; it's a factual error today)

`design-system/wiki-typography-system.md` documents **IBM Plex Sans/Mono**, which was **never live**.
The live engine used Oswald/Nunito/Roboto Slab; the ratified direction (master brief §7, new L8) is
**Inter (UI + headings) + Source Serif 4 (long-form reading body) + system mono**. Three truth
sources disagree — collapse all to **Inter + Source Serif 4 + system mono**:
- `design-system/wiki-typography-system.md` — rewrite the font stack, type scale, measure
  (one `--measure: 68ch`), and CSS-token names to semantic (`--ps-wiki-font-body`, not `-plex`).
- `design-system/design-primitive-vocabulary.md` + `brand-typography.md` — restate the type-split
  rationale (Inter for UI/headings; Source Serif 4 for reading; mono for code).

## B. The engine TOPIC — federation model

`applications/app-mediakit-knowledge.md` (+ `.es`): replace the single-binary monolithic-render
description with the **content-federation model** (planned): declarative `knowledge.toml` mounts +
content-type blueprints (`topic`/`guide` built-in; `regional-market`/`adr`/`changelog` pluggable);
per-instance isolation by disjoint mounts; provenance + edit-routing. Add: mobile-first, Inter,
zero-dead-links. Keep "We Own It" + git-native framing.

## C. New PATTERN topic — federation architecture

New `patterns/federation-via-content-mounts.md` (+ `.es`): the mount/blueprint/provenance
architecture as a reusable substrate pattern; how external customers/community federate their own
git repos; the hybrid model (curated editorial funnel internally + declarative mounts for others).

## D. Linking model + zero dead links

- `patterns/knowledge-wiki-leapfrog-architecture.md` — reframe the roadmap **mobile-first**; align
  fonts; replace "Wikipedia parity" framing with "Wikipedia information-model, Stripe/Linear craft."
- `.agent/rules/content-contract.md` + `naming-convention.md` (these live in the
  media-knowledge-documentation repo, editorial-owned): formalize **`type: guide`**; add a blueprint
  reference; add mount/provenance frontmatter fields; document the **zero-dead-links discipline**
  (every `[[ ]]` resolves within the instance's federated namespace; build gate; no red-links);
  document **slug normalization** (strip `topic-` prefix); document the **TOPIC↔GUIDE cross-link rails**
  (a TOPIC shows "How-to guides" from typed backlinks; a GUIDE shows "Background / concepts").
- `contribute.md` (+ `.es`) — add TOPIC vs GUIDE authoring guidance + the no-dead-links rule.

## E. Design-system docs — mobile-first + premium craft

`design-system/_index.md`, `wiki-component-library.md`, `wiki-dark-mode.md`, `design-philosophy.md`:
introduce Inter as the foundation; add **mobile-first** as a foundational principle (the 9 M-defects
and breakpoint ladder are in master §10); components must declare responsive/touch states (≥44px
targets, tap-not-hover, safe-area, `svh`/`dvh`).

## F. GUIDE docs — mount manifest + fonts + mobile ops

In the fleet-deployment repos (editorial-routed): `guide-deployment.md` (×3 instances),
`guide-operate-knowledge-wiki.md`, `guide-knowledge-wiki-sprint-roadmap.md`,
`guide-wiki-design-tokens.md`: add the planned `knowledge.toml` mount-manifest config (it replaces
`WIKI_CONTENT_DIR`/`WIKI_GUIDE_DIR`), Inter fonts, and mobile-first operational notes. Note that the
manifest, written with canonical `media-knowledge-*` paths, also resolves the current stale-path/
behind-canonical wiring (master §11).

---

## Open question for project-editorial

Slug normalization (strip `topic-` prefix) touches existing corporate/projects article filenames and
their inbound wikilinks. Confirm whether to run `migrate_corpus.py` across corporate + projects (as
was done for documentation) in the same pass, or stage it separately. The engine can also strip
`topic-` at render as a transitional measure — pick one and record it in `content-contract.md`.
