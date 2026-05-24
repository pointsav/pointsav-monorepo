# Open items — project-editorial

> Persistent TODO tracker. Updated at session end. Last updated: 2026-05-24.
> Completed items remain here (checked) for one session before archiving to `archive/`.

---

## DONE — 2026-05-23 session

- [x] **Category placement fix** — capability-ledger-substrate + merkle-proofs-as-substrate-primitive moved from `architecture/` to `substrate/`; slug, frontmatter, `_index.md` + `_index.es.md` MOC entries corrected (`69c6030`)
- [x] **Wikilink audit** — 0 broken links across all 3 wikis (post-category-fix verification)
- [x] **featured-topic.yaml candidates list** — added `capability-ledger-substrate` + 11 other rotation-pool candidates
- [x] **Claim-validation linter pass** — `editorial-lint.py` now carries claim-authoring-convention §9 checks: id/confidence required, id uniqueness, cites non-empty unless structural, cites resolution against citations.yaml, depends_on resolution, projected-language check

---

## DONE — 2026-05-22 session

- [x] **Briefs migration** — `.agent/plans/` → `.agent/briefs/`, `BRIEF-` prefix + frontmatter + README (`e5bd2514`)
- [x] **Editorial plan AUTO block (items 1–10)** — E1, E4, A0, D1, D2, D3, D4, D6, A1, E-ruleset committed; editorial-QA substrate built at `.agent/editorial-qa/`
- [x] **A2 — twelve flagship TOPIC rewrites (EN+ES)** — Bloomberg lede + Gate-0 + claim markup per claim-authoring-convention #54 (`d71f0c3`…`63d133a`)
- [x] **A4 close-out** — wikilink audit 0 broken across all 3 wikis; `wikilink-audit.py` added (`19c64001`); plan §12 close-out recorded; Stage 6 publish request routed to Command

---

## DONE — 2026-05-21 session

- [x] **Knowledge-platform plan** — built `award-winning-wiki-overhaul.md` (5-track, 7 research agents), then adopted project-knowledge's proposed plan; committed self-contained `KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md` (`b8c19dfd`)
- [x] **Cross-check with project-knowledge** — reconciled against `KNOWLEDGE-PLATFORM-VISION.md` rev 4; 2 outbox replies + SITUATION message to Command
- [x] **Gate 0 ratified** — 5 Lucidity-Protocol-vs-Bloomberg-standard reconciliations

---

## DONE — 2026-05-20 session

- [x] **Drafts-outbound cleanup** — `refined/co-location/woodfine/` (11 files, closed batch) git-removed; empty dirs cleared
- [x] **Phase E design draft** — `design-phase-e-bilingual-routing.draft.md` written; outbox message to project-knowledge added
- [x] **leapfrog-facts.es.yaml** — Spanish DYK content produced and committed to all 3 wikis (documentation d9d2bc1, projects 825fab2, corporate a5a062b)

---

## DONE — 2026-05-19 session (continuation)

- [x] **Phase 1c resolved** — topic- prefix confirmed canonical; outbox message sent to unblock Command Stage 6
- [x] **Vendor co-location drafts cleared** — location-intelligence-ux TOPIC pair committed to applications/; vendor refined/ drafts removed (commit `01379316`)
- [x] **Corporate wiki lede** — DataGraph-aligned lede applied (commit `188dabd`); index.es.md updated + BCSC disclosure added; featured-topic rotated
- [x] **GIS Batch 1** — topic-gis-nordic-uk-coverage.md + .es.md committed to content-wiki-projects (commit `a9d5325`)
- [x] **GIS Batch 2** — all 4 tech TOPICs confirmed already committed in prior session
- [x] **from-project-gis/ drafts cleared** — all 9 drafts removed; Italy stub bounced to handoffs-outbound (commit `f82b4ee6`)
- [x] **E1 wikilink fixes** — 13 files: delink reverse-funnel-editorial-pattern + service-minutebook/bookkeeper; fix sel4-foundation→sel4-microkernel-substrate in 7 ES files; remove broken cross-category _index links (`19cd854`)
- [x] **E3 status field** — brand-family-swatch + brand-typography (all 4 EN+ES files) — added `status: active` (`80de908`)
- [x] **E4 title case** — foundry-services-slice-model sentence case; slug drift logged in cleanup-log.md (`d8d82cf`)
- [x] **Outbox pruned** — 12 actioned/stale messages archived; routing message added for 5 design drafts → project-design (`487d1da3`)

---

## DONE — 2026-05-24 (session 4, Command)

- [x] **Stage 6: media-knowledge-documentation** — promoted (outbox actioned)
- [x] **Stage 6: media-knowledge-projects** — promoted (outbox actioned)
- [x] **Stage 6: media-knowledge-corporate** — promoted (outbox actioned)
- [x] **Stage 6: woodfine-fleet-deployment** — promoted (outbox actioned)
- [x] **app-mediakit-knowledge nightly build** — sha256 `14bdb9a3`, smoke_test: pass; services restarted
- [x] **Local directory rename** — content-wiki-* → media-knowledge-* (actioned by Command)

## PENDING — Command Session

- [ ] **pointsav-monorepo branch merge** — `readme-fixes-2026-05-16` → main (Phase D Rust code)
- [ ] **project-knowledge services restart** — after monorepo main merge; cargo build --release + restart 3 services
- [ ] **Admin README fixes** — pointsav-media-assets + woodfine-media-assets (admin-tier commits)
- [ ] **Route design drafts** — forward project-editorial outbox routing message to project-design inbox (5 DESIGN-RESEARCH + component drafts)
- [ ] **Staging mirror rename** — jwoodfine/content-wiki-* + pwoodfine/content-wiki-* → media-knowledge-* on GitHub (operator action first)
- [ ] **NOTAM stale disk>95% entries** — archive 9 entries (10:28Z–15:58Z 2026-05-24); disk now at 54%

## PENDING — project-editorial

- [ ] **Editorial plan — A4 close-out complete.** Track A (project-editorial execution) + Track D (except D5) + E1/E4/E-ruleset done; recorded in `BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md` §12. Plan kept `status: active` — archival is operator-gated per §9.
- [ ] **D5 — apprenticeship verdict-signing loop** — joint with project-intelligence; needs an operator signing identity. Operator action.
- [x] **Claim-validation linter pass** — added claim-authoring-convention §9 checks to `editorial-lint.py` (Track D complete; `2026-05-23`).
- [ ] **E2 / E3 / E5 / E-claim / E-rename** — cross-cluster handshakes; E-rename gated on the operator's `content-wiki-* → media-knowledge-*` GitHub rename.
- [ ] **A1 review pass** — review Main Page lede prose when project-knowledge branches each Main Page.
- [ ] **Old-plan deletions parked** — execute after the overhaul ships, on operator go-ahead (delete set in `BRIEF-KNOWLEDGE-PLATFORM-EDITORIAL-PLAN.md` §9)

---

## Key file paths

| Path | Role |
|---|---|
| `.agent/drafts-outbound/design-home-chrome-v2.draft.md` | Phase D design spec for home_chrome() |
| `content-wiki-documentation/reference-invariants.yaml` | "From the engineering record" panel data |
| `content-wiki-projects/reference-invariants.yaml` | "Reference geometry" panel data |
| `content-wiki-corporate/reference-invariants.yaml` | "Holding structure" panel data |
| `pointsav-monorepo/app-mediakit-knowledge/src/server.rs:1150` | Hardcoded "From the doctrine" block to replace |
| `pointsav-monorepo/app-mediakit-knowledge/src/server.rs:1271` | Hardcoded sister surfaces to replace |
| `.agent/briefs/BRIEF-overhaul-progress.md` | Broader wiki overhaul progress tracker |
