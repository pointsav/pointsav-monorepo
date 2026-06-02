---
artifact: brief
status: archived
contamination_note: >-
  Contaminated in project-data; belongs to project-knowledge. Command: redistribute to clones/project-knowledge/.agent/briefs/
archived_date: 2026-06-01
topic: project-knowledge current work queue
archive: project-knowledge
created: 2026-05-31
updated: 2026-06-02
owner: totebox@project-knowledge
---

# BRIEF — Active Work Queue (project-knowledge)

Current session and near-term work items. Updated at session end.

---

## Completed (2026-05-31 → 2026-06-02)

- [x] Session startup: corporate wiki (9095) restarted — was down since 06:37Z
- [x] nginx proxy_read_timeout raised 30s → 90s on all three vhosts
- [x] BRIEF consolidation — Gemini handover archived; BRIEF-active-work created
- [x] Wire `toc-persistence.js` into `wiki_chrome()` — DONE
- [x] UX-B.7: Woodfine SVG wordmark — DONE
- [x] Phase 10: Reading state progress bar — DONE
- [x] Phase 9 CSS + JS: claim-rail — DONE (CSS + JS in source; server.rs:2116 emits aside.claim-rail)
- [x] Fix #p-views display — hidden; duplicate tab bar removed
- [x] Recover binary-vs-source divergence — source matches binary on Phase 9+10 + toc-persistence
- [x] Fix links.rs `exists()` method — DONE (links.rs:40 `pub fn exists(&self, slug: &str) -> bool`)
- [x] Phase 9: server.rs emit of `<aside class="claim-rail">` — DONE (server.rs:2116–2158)
- [x] Phase 11: `query_claims(topic, asof)` MCP method — DONE (mcp.rs:375)
- [x] Phase 0 federation: cross-mount inject_wiki_prefixes — DONE (render.rs:433, extra_roots param)
- [x] Phase 0 federation: blueprints.rs + mounts.rs + check.rs — DONE
- [x] Phase 2: article TOC-drawer fix — DONE (ce63655f)
- [x] Phase 3: home polish — DONE (2046cbcd + ba16bece)
- [x] Phase 4: Cmd+K command palette — DONE (f9d515d6)
- [x] M1: tap-popovers — DONE (04687d38)
- [x] Phase 5: per-brand theming fix (load order) — DONE (d9989113)
- [x] Wikilink parser hardening (code-span stripping) — DONE (bb40fee3)
- [x] check subcommand — DONE (f0eeba5f)
- [x] Wikipedia→product-docs redesign (docs-sidenav + doc-header) — DONE
- [x] Cache-busting fix — deployed 2026-06-01
- [x] Phase 0 federation infra — deployed 2026-06-01
- [x] State-file contamination sweep — DONE 2026-06-02 (CLAUDE.md, NEXT.md, session-context.md, outbox triage)

---

## Pending Command Session

- [ ] Binary deploy — UX batch (7 commits) on canonical at `39f4dcd1`; no promote needed
      `deploy-binary.sh app-mediakit-knowledge --note "UX batch: Phase 5/4/M1/Phase 2/Phase 3/wikilink-parser/check (39f4dcd1)"`
- [ ] GUIDE placement — guide-location-intelligence-data-collection → woodfine-fleet-deployment/gateway-orchestration-gis-1/
- [ ] NEXT.md fix — promote monorepo commit (ops(knowledge): restore contaminated NEXT.md)

## Pending project-editorial

- [ ] Content audit: 17 dead wikilinks + 6 missing-slug guides; report in drafts-outbound/

## Pending project-design

- [ ] DESIGN-docs-sidenav-component + DESIGN-doc-header-component (in outbox)

## Blocked on operator / standing deferred

- Phase 12 (AI marginalia): gated on BP5 + SYS-ADR-07/10/19 review
- Phase 5.1+ (ACLs/OIDC/webhooks): gated on BP5 clearance
- Phase 6 (three-instance split): gated on GitHub renames + Doctrine amendment
- claim-rail nightly URL validator: gated on server infrastructure
