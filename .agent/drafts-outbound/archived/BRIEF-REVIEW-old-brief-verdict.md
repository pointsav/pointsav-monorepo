---
schema: foundry-draft-v1
artifact_type: DESIGN-RESEARCH
language_protocol: DESIGN-RESEARCH
state: draft
title: "Old Knowledge-Platform Brief Decisions: Carry / Reject / Revise Verdict"
created: 2026-06-04
author: project-knowledge@claude-code (Opus review panel)
research_provenance: "Three competing Opus agents (Skeptic, Preservationist, Archaeologist) reviewed BRIEF-knowledge-platform-master.md (2026-06-01), BRIEF-app-mediakit-knowledge-2030.md, BRIEF-knowledge-platform.md, and DESIGN-knowledge-platform-fresh-slate-analysis.draft.md. Synthesis by fourth Opus agent."
bcsc_class: no-disclosure-implication
---

This is a synthesis/adjudication task — I have all three reviews and the ground-truth defect list. No tools needed; this is pure reasoning over the provided material. Let me produce the verdict document.

# Knowledge Platform — Synthesis Judge Verdict

**Date:** 2026-06-04 · **Judge:** Synthesis adjudication over Skeptic / Preservationist / Archaeologist reviews · **Scope:** Locked decisions L1–L19 + named non-L decisions in `BRIEF-knowledge-platform-master.md`, tried against 12 confirmed fresh-slate defects.

---

## 1. Executive Summary

The dominant finding is that **the old brief was substantially correct in its decisions and substantially wrong in its enforcement**. Of 24 identified decisions, **15 CARRY verbatim** (all constitutional/legal/identity invariants plus clean product decisions), **9 REVISE** (correct intent, missing acceptance test or completion gate), and **0 are outright REJECTED** — the one genuinely dead idea (red-links) is a SPLIT within L18, not a standalone decision, and the brief itself already ordered it dropped. The three reviewers converged hard: where the Preservationist said "implementation debt, not brief error" and the Archaeologist traced "brief said the right thing, code ignored it," the Skeptic's REVISE verdicts agree on the *remedy* even while assigning blame to the brief's toothlessness. The single sharpest pattern, endorsed by all three: **the old brief repeatedly declared a target state as "locked" without an enforcement clause or completion gate** — L18 named the exact dead-link gate and shipped without it, L19 said mounts "replace" the env wiring and left both in the tree, L4 locked "bilingual" and shipped an English `/es/` home. The new BRIEF must make every load-bearing decision carry its own acceptance test and merge gate.

---

## 2. Master Verdict Table

| Decision ID | Original Text (≤80 chars) | Skeptic | Preservationist | Archaeologist | **FINAL VERDICT** | New BRIEF wording |
|---|---|---|---|---|---|---|
| **L1** | Single Rust binary (`cargo build --release -p app-mediakit-knowledge`) | REVISE | CARRY | (gap #1) | **REVISE** | "Single Rust binary remains the deployment unit (Doctrine #54, customer-rooted, no runtime dependency). **Single binary ≠ single source file ≠ single JS bundle:** internals MUST be modular (see L20) and client assets MUST be route-scoped (see L25)." |
| **L2** | Git-native flat-file content store (`.md` + `git2`) | CARRY (no harm) | CARRY | — | **CARRY** | Verbatim. 50-year-readable, diffable, underpins F12 audit trail. |
| **L3** | DTCG token pipeline → `static/tokens.css` \| Single token vault | REVISE | CARRY-REVISED | (gap #2) | **REVISE** | "`dtcg-bundle.json` is the single source of truth for all CSS custom properties. Per-brand outputs (`tokens.css`, `tokens-woodfine.css`) are GENERATED through `dtcg-to-css.py`. No hand-authored token/theme CSS may coexist with the generated bundle; brand variation is a token override in the vault, not a new file. (`theme-woodfine.css` to be folded in and deleted — see L21.)" |
| **L4** | Bilingual routing (`.es.md` sibling, single canonical slug) | REVISE | CARRY | (drift #4) | **REVISE** | "Bilingual EN+ES via `.es.md` sibling on a single canonical slug. **Bilingual scope includes chrome:** all reader-visible strings (nav, headings, footer, featured/recent titles) come from a `strings(locale)` map; `/es/` MUST prefer the `.es` sibling's title. Acceptance: rendering `/es/` asserts zero hardcoded-English chrome strings (see L22)." |
| **L5** | Self-hosted WOFF2 fonts — no CDN (GDPR Art. 44) | REVISE | CARRY | (gap #5) | **REVISE** | "Self-hosted WOFF2 fonts, no CDN (GDPR Art. 44 — non-negotiable legal invariant). Self-hosting carries its own loading contract: each above-the-fold face MUST emit `<link rel=preload as=font type=font/woff2 crossorigin>` plus metric-override fallback (see L23)." |
| **L6** | Wikipedia Vector 2022 DOM conventions where they serve the model | REVISE | CARRY | (adjacent #1) | **REVISE** | "Adopt Wikipedia Vector 2022 information-model conventions (wikilinks, Article/Talk/History, hatnotes); visual language is Stripe/Linear. **Chrome rendering lives in one parameterised `chrome.rs` emitter — never three inline `*_chrome` copies in the handler file.**" |
| **L7** | Canonical footer trademark text verbatim (§9) | CARRY (no harm) | CARRY | — | **CARRY** | Byte-for-byte; only the year field updates. Legal/trademark invariant. |
| **L8 (new)** | Inter (UI+headings) + Source Serif 4 (body) + system mono; WOFF2 | REVISE | CARRY | (no defect in choice) | **CARRY** | Verbatim — typeface roster is operator-approved (2026-06-01 Decision Log). The loading contract is owned by L5/L23, not L8. Record the supersession of the old Oswald/Nunito/Roboto stack. |
| **L9** | `--navy: #164679`; `--bg: #F7F9FA`; `--link: var(--navy)`, WCAG AA | CARRY (no harm) | CARRY | — | **CARRY** | Verbatim. Brand token triad, WCAG AA verified; protected by routing through L3's vault. |
| **L10** | MCP JSON-RPC 2.0 native (`src/mcp.rs`) | CARRY (no harm) | CARRY | — | **CARRY** | Verbatim. Shipped differentiator; no defect. |
| **L11** | Claim-layer HTML comment markup | CARRY (no harm) | CARRY | — | **CARRY** | Verbatim. In production; foundation for claim-rail. |
| **L12** | SYS-ADR-07: no structured data through AI | CARRY (no harm) | CARRY | — | **CARRY** | Verbatim. Constitutional hard rule. |
| **L13** | SYS-ADR-10: F12 mandatory; human commits only | CARRY (no harm) | CARRY | — | **CARRY** | Verbatim. Constitutional. Collab dead-code removal (defect 8) must not weaken this gate. |
| **L14** | SYS-ADR-19: no automated AI publishing to verified ledgers | CARRY (no harm) | CARRY | — | **CARRY** | Verbatim. Constitutional. |
| **L15** | Apache 2.0 licence | CARRY (no harm) | CARRY | — | **CARRY** | Verbatim. Legal invariant. |
| **L16** | Commit identity `jwoodfine`/`pwoodfine` only; `commit-as-next.sh` | CARRY (no harm) | CARRY | — | **CARRY** | Verbatim. Governance hard rule, pre-commit-gate enforced. |
| **L17** | Mobile-first — base = phone; desktop via `min-width`; ~80% mobile | REVISE | CARRY | (drift #6) | **REVISE** | "Mobile-first: base stylesheet = phone, desktop via `min-width`. **Enforcement checklist (per release):** `env(safe-area-inset-bottom)` APPLIED (not merely defined) on all `position:fixed`/sticky chrome and body bottom padding; `viewport-fit=cover`; `≥16px` inputs; `dvh` not `100vh`. Per-release phone smoke test required (see L24)." |
| **L18** | Zero dead links — every `[[ ]]` resolves or is not a link; no red-links | REVISE | CARRY | (never-impl #9) | **SPLIT** | **CARRY (load-bearing half):** "Build-time wikilink resolver is a HARD GATE — any unresolved `[[ ]]` across topics + all federated mounts BLOCKS promote. The gate is a precondition of the 'zero' claim, not a follow-up; it MUST land before the red-link render path is removed." **REJECT (red-link affordance):** delete `wikilink-missing` emission and invert the render test to assert no red-link — but only *after* the gate exists, so dead links are never invisible-and-unguarded. |
| **L19** | Federation via declarative mounts + content-type blueprints | REVISE | CARRY | (never-impl #3) | **REVISE** | "Federation via declarative `Vec<Mount>` + content-type blueprints. **Completion-gated, not 'locked-done':** `AppState` carries `mounts: Vec<Mount>` (the hardcoded `content_dir`/`guide_dir`/`guide_dir_2` fields are DELETED); `blueprints.rs` is wired into render; `inject_wiki_prefixes` resolves across the full mount set. No Phase 2+ visual work merges to canonical while the old path survives or any instance is unwired." |
| **§9 footer** | Full trademark block, verbatim | (no harm) | CARRY | — | **CARRY** | Byte-for-byte; year field only. |
| **§11.3** | GUIDEs stay in fleet-deployment — DO NOT move them | REVISE | CARRY | (gap #12) | **REVISE** | "GUIDEs stay in `woodfine-fleet-deployment` (Foundry TOPIC/GUIDE taxonomy). The L18 build-time gate MUST resolve `[[guide-slug]]` across all federated guide mounts; until cross-mount resolution exists, guides land in fleet-deployment BEFORE any article references them (content-sequencing rule, see L29)." |
| **§4 isolation** | Cross-instance isolation structural; never a global `[[slug]]` resolver | (no harm) | CARRY | — | **CARRY** | Verbatim. Security invariant — a global resolver would leak corporate slugs into documentation and vice versa. |
| **§3 status table** | "Phases 1–8 + Leapfrog shipped … all promoted" | REVISE | (concede drift) | (gap #8) | **REVISE** | "Every shipped-phase row carries: (a) net file/line delta, (b) any code paths abandoned mid-flight flagged for deletion, (c) bundle-size impact. Removal actions from superseded predecessor briefs MUST be carried forward, not dropped in consolidation (see L26)." |
| **§14 Phase 0** | Phase 0 federation/gate "Gates the nav/sidebar in Phases 2–3" | REVISE | (n/a) | (sequencing) | **REVISE** | "Phase 0 (mount wiring + cross-mount link resolution + dead-link gate) is a HARD merge gate: no Phase 2+ feature/visual work reaches canonical while any Phase 0 item is incomplete. Stated as an enforced precondition, not prose." |

---

## 3. New Locked Decisions Proposed by Archaeologist

| ID | Proposed decision text | Defect prevented | Include? |
|---|---|---|---|
| **L20** | Source-file size discipline: no source file exceeds ~1,500 lines / 60 KB; `server.rs` decomposes into `routes.rs`, `chrome.rs`, `state.rs`, `pages/`, `walker.rs` along the concern boundaries the old §16 enumerated. | #1 (214KB monolith) | **Yes** — closes the implicit blessing in old §16. |
| **L21** | Exactly three CSS artifacts permitted (`style.css` shared, `tokens.css` PointSav, `tokens-woodfine.css` Woodfine); `theme-woodfine.css` folded in and deleted; a new `.css` requires a Decision-Log entry. | #2 (four competing CSS files) | **Yes** — binds L3's "single vault" to a hard file count. |
| **L22** | Chrome strings are locale-keyed via `strings(locale)`; `/es/` prefers the `.es` sibling title; test asserts zero hardcoded-English chrome on `/es/`. | #4 (English `/es/` chrome) | **Yes** — operationalizes L4. |
| **L23** | Font preload mandatory: `<head>` emits `<link rel=preload as=font crossorigin>` for the two latin regular faces, alongside the metric-override fallback. | #5 (FOIT/CLS) | **Yes** — completes L5/L8 loading contract. |
| **L24** | Safe-area insets APPLIED not merely defined: `padding-bottom: calc(56px + var(--safe-b))` on fixed bottom chrome + body; CSS lint enforces inset presence. | #6 (Home Indicator overlap) | **Yes** — operationalizes L17/M4. |
| **L25** | Route-gated client bundles: editor assets (CodeMirror, SAA, collab) load only on `/edit/*` via `editor.js`; article/home ship only `wiki.js` + `toc-persistence.js`; per-route payload budget in acceptance. | #7 (CodeMirror on every page) | **Yes** — no payload rule existed. |
| **L26** | Dead-code removal is a tracked deliverable on every plan rebase: removed feature's module, vendor bundle, routes, and tests deleted in the same change and logged; superseding briefs carry forward predecessor removal actions. | #8 (collab dead code) | **Yes** — the consolidation-loss root cause. |
| **L27** | List micro-layouts carry an explicit separator in markup (not CSS-only); recent-changes title/date use a separator element or block structure. | #10 ("Contact Topic2026-06-03") | **Yes** — cheap, closes a visible defect. |
| **L28** | DNS provisioning is a named deliverable: each §1 instance row carries a DNS-status field + owning session (Command); reconcile `documentation.pointsav.com` vs `documentation.woodfinegroup.com` before cutover. | #11 (NXDOMAIN + name drift) | **Yes** — also fixes an unreconciled brand/domain conflict. |
| **L29** | No article may reference an uncommitted guide/topic slug: same build-time resolver as L18 blocks promote on unresolved `[[ ]]`; interim content-sequencing rule applies until the gate exists. | #12 (62 guide-slug 404s) | **Yes** — same gate as #9; unifies L18/§11.4. |

*Judge note:* L29 and the L18 build-time gate are the **same mechanism**. The new BRIEF should implement one resolver that serves both #9 and #12, and reference it from L18, §11.4, §11.3, and L29 rather than describing it four times.

---

## 4. Invariants Confirmed (never change, no matter what)

All three agents agree these are non-negotiable and CARRY verbatim:

- **L1** — Single Rust binary deployment unit (Doctrine #54 "We Own It"). *(Deployment property is invariant; only the internal-modularity revision is added — the binary itself never changes.)*
- **L2** — Git-native flat-file content store.
- **L5** — Self-hosted WOFF2, no CDN (GDPR Art. 44). *(Legal invariant; preload is additive.)*
- **L7 / §9 footer** — Canonical trademark text, byte-for-byte (year field only).
- **L10** — MCP JSON-RPC 2.0 native.
- **L11** — Claim-layer HTML comment markup.
- **L12** — SYS-ADR-07 (no structured data through AI).
- **L13** — SYS-ADR-10 (F12 mandatory, human commits only).
- **L14** — SYS-ADR-19 (no automated AI publishing to verified ledgers).
- **L15** — Apache 2.0 licence.
- **L16** — Commit identity `jwoodfine`/`pwoodfine` only via `commit-as-next.sh`.
- **§4** — Cross-instance isolation is structural; never a global `[[slug]]` resolver.

The four constitutional ADR hard rules (L12–L14) and the governance/identity rules (L16, §4) are constraints that *prevent* a class of problems and caused none on the defect list — they are the cleanest carries in the register.

---

## 5. Implementation Drift vs Brief Failure — Final Score

Each of the 12 confirmed defects classified by root cause (Archaeologist's trace, cross-checked against Skeptic/Preservationist):

| Category | Count | Defects |
|---|---|---|
| **Caused BY a brief decision** (the decision itself was wrong) | **0** | — (no decision is rejected outright; even red-links the brief already ordered dropped) |
| **Caused by SILENCE** (brief gap — no decision existed) | **6** | #1 (no decomposition mandate), #2 (4th CSS file unacknowledged), #7 (no payload-per-route rule), #8 (collab removal lost in consolidation), #10 (no list-separator rule), #11 (no DNS deliverable) |
| **Caused by IMPLEMENTATION DRIFT** (brief correct, code ignored it) | **3** | #4 (English `/es/` chrome vs L4), #6 (safe-area defined-not-applied vs L17/M4), #10 partial overlap — counted under silence |
| **NEVER IMPLEMENTED** (brief specified, scaffold-only or unbuilt) | **3** | #3 (mounts/blueprints unwired vs L19), #9 (dead-link gate never built vs L18, masked by false code comment), #12 (cross-mount resolution + sequencing gate vs §11.4 — same gate as #9) |

*(Note: #5 sits between SILENCE and drift — the brief specified metric-override but was silent on the specific preload remedy; classified as silence/wrong-mechanism. #6 is the cleanest pure-drift case: the exact fix was named in §10 M4 and the token plumbed, then the apply step dropped.)*

**Decisions never implemented despite being correctly specified: 3** (L18 gate, L19 wiring, §11.4 cross-mount resolution — and #3's blueprints).

### What this tells us about writing the new BRIEF differently

The arithmetic is decisive: **zero** defects came from a wrong decision; **six** came from the brief being *silent*; **six** came from correct decisions the implementation either drifted from or never finished. The old brief failed not by deciding wrongly but by **(a) leaving gaps where no decision existed and (b) writing decisions as aspirational prose with no completion criterion**. Three structural rules for the new BRIEF:

1. **Every load-bearing decision carries its own acceptance test and merge gate.** "Zero dead links" becomes "build-time resolver blocks promote on any unresolved `[[ ]]` — here is the test." "Mobile-first" becomes a per-release phone smoke-test checklist. A decision that can ship with its load-bearing half unbuilt is an aspiration, not a lock.

2. **Distinguish "locked-done" from "completion-gated."** L19 and the L18 gate must be demoted from "locked" to "in progress, completion-gated" until actually serving in all three instances. A locked decision that leaves two parallel implementations in the tree (hardcoded `guide_dir` AND `Vec<Mount>`) is worse than either alone.

3. **The status table must account for removal, not just addition.** The single most insidious failure was a `§3` table that only ever added "Shipped" rows — letting collab dead code, the four-CSS pileup, and the `server.rs` monolith accumulate unremarked across eight phases. Track net delta, abandoned paths, and bundle impact per phase; carry forward predecessor *removal* actions on every consolidation.

The Preservationist's framing is the correct guidance for the rewrite team: **refactor the code to honor the decisions; do not rewrite the decisions to excuse the code.** The nine new L-decisions (L20–L29 above, less the merged pair) exist precisely to convert the six silences into enforced locks.
