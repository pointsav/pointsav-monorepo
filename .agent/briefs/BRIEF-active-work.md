---
artifact: brief
status: active
created: 2026-05-23
consolidates:
  - BRIEF-institutional-chrome-sprint.md (archived 2026-05-23)
  - BRIEF-github-presence-elevation.md (archived 2026-05-23)
  - BRIEF-publishing-tier-naming-cross-check.md (archived 2026-05-23)
---

# project-editorial — Active Work Queue

> Single consolidated brief. Replaces the three action briefs above.
> Research reference: `BRIEF-framework-pointsav-products-services.md` (standing — not archived).
> Knowledge-platform plan: `BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md` (execution complete; archival operator-gated per §9).

---

## STATUS — 2026-05-24 (session 3)

| Area | State |
|---|---|
| Knowledge-platform editorial overhaul | **COMPLETE** (project-editorial scope). Stage 6 pending Command. |
| Wiki rebuild lint blockers | **COMPLETE this session.** CWP 9 errors → 0 (aa26ddd). CWC 5 errors → 0 (01ea8a7). |
| ES governance pages — corporate | **COMPLETE this session.** about.es.md, contact.es.md, disclaimers.es.md authored (01ea8a7). |
| Stage 6 — all 4 repos | **Pending Command.** Outbox message updated with final ranges. |
| Serving-clone pull + service restart | **Pending Command** (after Stage 6). |
| Nightly build — cargo PATH fix | **Pending Command** — `foundry-nightly-build.service` needs `Environment=PATH=...` line. |
| Nightly build — app-mediakit-knowledge | **Build-request in outbox.** Prerequisites: Stage 6 + cargo PATH fix first. |
| Chrome sprint E1 / E3 / E4 | **Blocked — awaiting Stage 6 + project-knowledge build.** |
| GitHub presence elevation | **Research complete.** Remainder Command/admin-tier. |
| Publishing-tier naming | **All project-editorial items done.** Brief archived. |
| BCSC README sweep | **COMPLETE** — clean, no changes needed. |
| Post-pass lint gate | **COMPLETE** — 0 errors, 246 sentence-length warnings (pre-existing). |

---

## AUTO-approved work queue — 2026-05-23 — COMPLETE ✓

All 6 items executed and committed this session. See STATUS table above.

### 1. A4 — Design-system stub aliases (pre-emptive) ✓

24 slugs were moved from `content-wiki-documentation/design-system/` to
`pointsav-design-system/` (foundations + components). `redirects.yaml` already
handles URL-level navigation. **No wiki article currently links to these slugs
via wikilinks** — so this is pre-emptive insurance, not active red-link repair.

Approach: add `aliases:` frontmatter to the surviving 5 articles in
`design-system/` (design-philosophy, design-primitive-vocabulary, brand-family-swatch,
brand-typography + ES pairs) and/or add minimal stub files for the 4 moved
foundation docs (`design-color`, `design-typography`, `design-spacing`,
`design-motion`) so that future wikilinks resolve cleanly.

Slugs from redirects.yaml:
- Foundations (4): `design-color`, `design-typography`, `design-spacing`, `design-motion`
- Components (20): `guide-component-badge`, `guide-component-breadcrumb`,
  `guide-component-button`, `guide-component-checkbox`,
  `guide-component-citation-authority-ribbon`, `guide-component-freshness-ribbon`,
  `guide-component-home-grid`, `guide-component-input-text`, `guide-component-link`,
  and remaining component guides in redirects.yaml

Commit: `content(design-system): add aliases for moved foundation + component slugs`

---

### 2. A2 — Title normalisation (content-wiki-documentation)

Sentence-case audit on `title:` frontmatter across all EN + ES articles.

Rules:
- EN: sentence case — only first word + proper nouns capitalised.
- ES: same discipline — only first word + proper nouns capitalised.
  Many ES titles are currently full Title Case ("Almacenamiento Inmutable y
  Respaldo Seguro" → "Almacenamiento inmutable y respaldo seguro").
- Preserved proper nouns: WORM, BCSC, GIS, PointSav, Woodfine, seL4, Tier A/B/C,
  DTCG, WCAG, DataGraph, LadybugDB, OLMo, IFC, BIM. Acronyms always uppercase.
- Also catch any stale G1/G2 product names (e.g. "Totebox OS", "interface-os")
  in titles — normalise to canonical `os-totebox`, `os-orchestration` per
  `BRIEF-framework-pointsav-products-services.md` §1.

Scope: all categories (architecture, substrate, patterns, services, systems,
applications, governance, infrastructure, reference, design-system).

Commit: one commit per category batch, or a single commit if changes are minor.

---

### 3. Glossary ES — complete translation

File: `reference/glossary-documentation.es.md`
Committed as a minimal stub at `583f642` to satisfy the bilingual-pair linter check.
Full translation against EN source (`reference/glossary-documentation.md`) and
the term list in `glossary-documentation.csv`.

Commit: `content(reference): complete glossary-documentation.es.md translation`

---

### 4. README cross-link fix (content-wiki-corporate)

`content-wiki-corporate/README.md` does not link to `content-wiki-projects`.
The reverse link exists. Add a one-line entry to the cross-links section.

Source: GitHub presence elevation brief item 8.

Commit: `docs(readme): add content-wiki-projects cross-link to corporate README`

---

### 5. BCSC language sweep (content-wiki-* READMEs)

Quick pass on `README.md` files in all three content-wiki-* sub-clones.
Check for forward-looking claims missing "planned / intended / may / target"
qualifiers. In-place edits only.

Source: GitHub presence elevation brief item 11.

Commit: fold into item 4 commit if changes are minor, or separate commit if
multiple files change.

---

### 6. Post-pass lint gate

Run `editorial-lint.py` across content-wiki-documentation after items 1–3.
Target: 0 errors, 0 warnings.

Command:
```
python3 /srv/foundry/clones/project-editorial/.agent/scripts/editorial-lint.py \
  /srv/foundry/clones/project-editorial/content-wiki-documentation/
```

Fix any surfaced issues. Record gate result in session context at shutdown.

---

## Blocked — awaiting project-knowledge build

These cannot run until Stage 6 lands and project-knowledge rebuilds + restarts
the three wiki services (`local-knowledge-documentation`, `-projects`, `-corporate`).

| Item | What | Unblocked by |
|---|---|---|
| E1 | `/wanted` endpoint audit — top 20 missing slugs; stub articles for planned content | Stage 6 + build |
| E3 | Category count verification — all 10 categories ≥5 articles after `company`/`help` removal | Stage 6 + build |
| E4 | Title QA spot-check — 20 articles across 5 categories for sentence-case compliance | After A2 + Stage 6 |

---

## Pending — Command Session / operator

| Item | What | Owner |
|---|---|---|
| Stage 6 | content-wiki-documentation, content-wiki-projects, content-wiki-corporate (all three wikis including commits from 2026-05-22 + 2026-05-23) | Command |
| Stage 6 | woodfine-fleet-deployment (BIM GUIDEs) | Command |
| Stage 6 | pointsav-monorepo branch `readme-fixes-2026-05-16` → merge + build + service restart | Command |
| Org profile READMEs | `pointsav/.github/profile/README.md` + `woodfine/.github/profile/README.md` — drafts in github presence brief | Command (admin-tier) |
| `pointsav-media-assets` README rewrite | Internal vocab → institutional register | Command (admin-tier) |
| `woodfine-media-assets` README.es.md | Missing bilingual pair | Command (admin-tier) |
| `USER_GUIDE_2026-03-30_V2.md` deletion | Read + content-forward first; then `git rm` | Command |
| ARCHITECTURE.md | pointsav-monorepo, pointsav-design-system, woodfine-fleet-deployment | Respective sessions |
| Plan archival + §9 deletion | knowledge-platform plan archive + 7 superseded brief files | Operator go-ahead, post-ship |
| D5 apprenticeship loop | Verdict-signing for apprenticeship corpus | Operator signing identity |
| E2 / E3 / E5 / E-claim / E-rename | Cross-cluster handshakes + GitHub repo rename | Operator / cross-cluster |
| A1 review pass | Main Page lede prose review when project-knowledge branches each Main Page | project-knowledge trigger |

---

## Reference notes

- **Canonical product names:** `BRIEF-framework-pointsav-products-services.md` §1 — use G3 names
  (`os-totebox`, `os-orchestration`, `service-slm`, etc.); flag G1/G2 variants as stale.
- **Design-system moved slugs:** 24 entries in `redirects.yaml` (foundations + components).
  Only 5 articles remain in `design-system/` category; 2 design-philosophy/primitive-vocabulary
  + 2 brand articles (EN+ES each).
- **Chrome sprint critical files:** `pointsav-monorepo/app-mediakit-knowledge/src/server.rs`,
  `static/style.css`, `static/fonts/` — all in branch `readme-fixes-2026-05-16`.
- **GitHub presence elevation drafts:** `pointsav` + `woodfine` org profile README drafts
  are in `BRIEF-github-presence-elevation.md` §§ "Draft content" — not reproduced here
  to avoid duplication; read that brief (archived) for the copy.
