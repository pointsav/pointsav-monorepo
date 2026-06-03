# NEXT.md — project-knowledge (app-mediakit-knowledge)

Hot open items. ≤200 lines. Backlog at `.agent/next-backlog.md`.

---

## Pending Command Session

- [ ] **Binary deploy — UX batch** — 7 commits on canonical; run from Command workspace:
      `~/Foundry/bin/deploy-binary.sh app-mediakit-knowledge --note "UX batch: Phase 5/4/M1/Phase 2/Phase 3/wikilink-parser/check (39f4dcd1)"`
      Services: local-knowledge-documentation (9090), local-knowledge-projects (9093), local-knowledge-corporate (9095)
      [2026-06-01 totebox@claude-code]
- [ ] **GUIDE placement** — `GUIDE-location-intelligence-data-collection.draft.md` from project-gis drafts-outbound
      → `woodfine-fleet-deployment/gateway-orchestration-gis-1/guide-location-intelligence-data-collection.md`
      Blocked by pretool-scope-check.sh hook; requires admin-tier commit from Command
      [2026-06-02 totebox@claude-code]

## Pending project-editorial

- [ ] **Content audit triage** — 17 dead wikilinks + 6 missing-slug guides in documentation corpus
      Report: `.agent/drafts-outbound/CONTENT-AUDIT-dead-links-2026-06-01.md`
      Missing targets: `[[os-totebox]]`, `[[regional-name-resolution-architecture]]`,
      `[[topic-knowledge-wiki-home-page-design]]`; stray-backslash links in 2 files
      [2026-06-01 totebox@claude-code]
- [ ] **`check --strict`** CI gate — wire as pre-promote gate once dead-link count reaches 0
      [2026-06-01 totebox@claude-code]

## Pending project-design

- [ ] **DESIGN-wiki-institutional-redesign** — staged in `.agent/drafts-outbound/DESIGN-wiki-institutional-redesign.draft.md`; **`master_cosign:` required** before token changes committed (color #0E3A66, body 18px, nav 14px) [2026-06-03 totebox@claude-code]
- [ ] **DESIGN-docs-sidenav-component** — staged in drafts-outbound; routed via outbox
      [2026-06-01 totebox@claude-code]
- [ ] **DESIGN-doc-header-component** — staged in drafts-outbound; routed via outbox
      [2026-06-01 totebox@claude-code]

## Institutional UX — P0 implementation (project-knowledge owns)

From Opus browser audit 2026-06-03. Full brief: `.agent/drafts-outbound/DESIGN-wiki-institutional-redesign.draft.md`

- [ ] **Build-time link checker** — add internal href validation to Rust render binary; fail build on any 404 in chrome or featured slots. Catches corporate slug bugs + projects footer 404s in CI [2026-06-03 totebox@claude-code]
- [ ] **projects.woodfinegroup.com vendor brand leak** — nav/login/footer leaks pointsav.com domains + "PointSav Knowledge" title onto a Woodfine customer site; make tenant brand authoritative across all templates [2026-06-03 totebox@claude-code]
- [ ] **Author Disclaimer + Contact pages for projects.woodfinegroup.com** — both footer links 404 on 44+ pages; remove /wiki/pointsav-media-kit [2026-06-03 totebox@claude-code]
- [ ] **Category-template redesign** — drop empty TOC rail from category.html; collapse to card grid `repeat(auto-fill, minmax(258px,1fr))`; add designed empty state per §4 Decision 3 of design brief [2026-06-03 totebox@claude-code]
- [ ] **corporate.woodfinegroup.com featured-article slug fix** — -mechanics/-obligations → -model/-mandate [2026-06-03 totebox@claude-code]

## Code fix needed

- [ ] **Doorman stub routes missing** — `tests/doorman_test.rs` expects `POST /api/doorman/complete`
      and `POST /api/doorman/instruct` to return 501; both return 404 because the routes don't
      exist in server.rs. Add two minimal stub handlers returning `StatusCode::NOT_IMPLEMENTED`.
      Pre-existing gap; not a regression from UX batch. [2026-06-02 totebox@claude-code]

## Standing deferred

- Phase 12 (AI marginalia): gated on BP5 + SYS-ADR-07/10/19 review [2026-06-01]
- Phase 5.1+ (ACLs/OIDC/webhooks): gated on BP5 clearance [2026-06-01]
- Phase 6 (three-instance deployment split): gated on GitHub renames + Doctrine amendment [2026-06-01]
- claim-rail nightly URL validator: gated on server infrastructure [2026-06-01]
