# Session Context — project-editorial

Rolling 3-session summary. Newest entry first. Push oldest to session-context-archive.md when >3 entries.

---

## 2026-05-24 (session 3) | totebox@claude-code | Sonnet 4.6

**Done this session (wiki rebuild readiness audit + lint fixes):**
- **Opus audit** — full rebuild readiness check: confirmed 3 systemd service content paths, identified unlicensed binary (`4f801fa3`, no ledger entry), confirmed cargo PATH bug in `nightly-build.sh`, confirmed 14 lint errors blocking CWP+CWC, confirmed vendor monorepo clean at origin/main (no new engine commits).
- **CWP lint — 9 errors → 0** — section-ordering fix in 9 co-location topic files (Provenance moved before See Also); `utilizes` → `uses` in `topic-co-location-ranking-system.md`. Committed `aa26ddd` (jwoodfine). Staging mirrors pushed.
- **CWC lint — 5 errors → 0** — `leverage` → `borrowing capacity` (×2 incl. short_description) in `topic-interest-coverage-ratio.md`; `facilitate` → `arrange` in `topic-investor-access.md`; authored `about.es.md`, `contact.es.md`, `disclaimers.es.md` (institutional finance register, strategic-adaptation style). Committed `01ea8a7` (pwoodfine). Staging mirrors pushed.
- **Outbox updated** — Stage 6 message updated with final commit ranges (CWP 2, CWC 4); build-request message for `app-mediakit-knowledge` added with prerequisites (Stage 6 → serving-clone pull → cargo PATH fix → queue build).
- **Plan updated** — plan file overwritten with full rebuild sequence (A→G).

**Pending / carry-forward:**
- **Stage 6** — CWD (11), CWP (2), CWC (4), WFD (1) — Command Session; outbox message `project-editorial-20260523-stage6-and-rename` (updated 2026-05-24).
- **Serving-clone pull + restart** — after Stage 6; Command Session.
- **Cargo PATH fix** — `foundry-nightly-build.service`: add `Environment=PATH=...` + daemon-reload; Command Session.
- **Nightly build** — `app-mediakit-knowledge`; after cargo fix; `nightly-build-plan.sh --add app-mediakit-knowledge --from project-knowledge`.
- **Staging mirror rename** — operator: rename `jwoodfine/content-wiki-*` + `pwoodfine/content-wiki-*` on GitHub to `media-knowledge-*`.
- **Local directory rename** — Command: `mv content-wiki-{documentation,projects,corporate} media-knowledge-*` + PROJECT-CLONES.md update.
- **E1/E3/E4** — blocked on Stage 6 + project-knowledge build.

**Operator preferences surfaced:**
- User wants rebuild readiness checked via Opus agent before planning; confirmed pattern.
- Build list + shutdown requested together at session end — combine into single response.

---

## 2026-05-23 (session 2) | totebox@claude-code | Sonnet 4.6

**Done this session (AUTO execution — 6-item queue):**
- **A4 — Design-system stubs** — 8 stub articles (4 EN+ES pairs: design-color, design-typography, design-spacing, design-motion) in `design-system/`, pointing to `design.pointsav.com/foundations/*` (`9fca6cd`).
- **A2 — Title normalisation** — 143 title fixes across all EN + ES articles in content-wiki-documentation; automated sentence-case script + 4 manual edge-case edits (`51e7724`).
- **Glossary ES** — full Spanish translation of glossary-documentation.es.md (694 lines, from 3-line stub) (`f82faeb`).
- **README cross-link** — `content-wiki-corporate/README.md` + `.es.md` now link to `projects.woodfinegroup.com` (`0fab5ad` in content-wiki-corporate).
- **BCSC sweep** — all 3 content-wiki README + index.md files checked; clean, no changes needed.
- **Lint gate** — 0 errors, 246 sentence-length warnings (pre-existing, not introduced this session).
- **BRIEF-active-work.md** — status table updated; AUTO queue marked complete.

**Pending / carry-forward:**
- Stage 6 promotion — content-wiki-documentation, content-wiki-projects, content-wiki-corporate + woodfine-fleet-deployment BIM GUIDEs (Command only).
- E1/E3/E4 — still blocked on project-knowledge build + Stage 6.
- Org profile READMEs, media-asset README rewrites — Command/admin-tier.
- Plan archival + §9 deletions — operator go-ahead post-ship.
- Yo-Yo files (service-slm-yoyo-operational, yo-yo-lora-training-pipeline) — title normalisation skipped; rename blocked on E4 triage.

**Operator preferences surfaced:**
- AUTO mode confirmed: execute all approved items autonomously; report at end.

---

## 2026-05-23 | totebox@claude-code | Sonnet 4.6

**Done this session (pre-build prep + wiki quality sweep):**
- **Category placement fix** — `capability-ledger-substrate` + `merkle-proofs-as-substrate-primitive` moved from `architecture/topic-*` → `substrate/*`; slug, frontmatter, `_index.md` + `_index.es.md` MOC entries corrected (`69c6030`).
- **Wikilink audit** — 0 broken links confirmed across all 3 wikis after category fix.
- **featured-topic.yaml candidates** — `capability-ledger-substrate` + 11 other rotation-pool articles added (`3f798bf`).
- **Banned vocab sweep** — 19 files across 6 categories: all `utilize*`, `robust`, `seamless`, `leverage` instances removed (`7cd8e3e`, `22face7`).
- **doorman-protocol claim fix** — `tier-a-verified` confidence changed from `established` → `structural` in EN+ES (`7cd8e3e`).
- **D1 linter hardening** — claim-validation §9 (id/confidence/uniqueness/cites/depends_on/projected-language), WARN-vs-ERROR fix for empty cites, ES projected-language skip (NameError fix), skip-dirs filter for non-article files (`d212863c`, `f64e279b`, `b8e1665e`).
- **glossary-documentation.es.md stub** — last missing bilingual pair; linter now reports 0 errors across 366 content-wiki-documentation articles (`583f642`).
- **NEXT.md dedup** — duplicate "Currently open" heading removed (`b8ddee0`).
- **Outbox addendum** — path-correction for capability-ledger-substrate (architecture/topic-* → substrate/*) prepended to project-system message (`cd7ae157`).

**Pending / carry-forward:**
- **Stage 6 promotion** — all three content-wiki-* repos; Command session only. Request in outbox since 2026-05-22.
- **D5** — operator signing identity; not project-editorial-executable.
- **E2/E3/E5/E-claim/E-rename** — cross-cluster / operator GitHub rename gated.
- **Plan archival + §9 old-plan deletion** — operator go-ahead, post-ship.
- **glossary-documentation.es.md** — stub only; full translation deferred.

**Operator preferences surfaced:**
- AUTO mode: execute autonomously on approved runs; surface only real blockers; report at end.
- Build-night goal: 0 lint errors before nightly build; achieved.


