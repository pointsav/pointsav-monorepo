---
artifact: brief
status: active
topic: project-knowledge current work queue
archive: project-knowledge
created: 2026-05-31
owner: totebox@project-knowledge
---

# BRIEF — Active Work Queue (project-knowledge)

Current session and near-term work items. Updated at session end.

---

## Immediate — this session (2026-05-31)

- [x] Session startup: corporate wiki (9095) restarted — was down since 06:37Z
- [x] nginx proxy_read_timeout raised 30s → 90s on all three vhosts
- [ ] BRIEF consolidation — index synced, Gemini handover archived, outbox to Command
- [ ] Revert Gemini's unused link_graph threading in render.rs/server.rs
- [ ] Fix links.rs `exists()` method (broken key lookup — needs prefix scan)
- [ ] Wire `toc-persistence.js` into `wiki_chrome()` in server.rs
- [ ] UX-B.7: Inline Woodfine SVG wordmark from `woodfine-media-assets/ASSET-WORDMARK-WOODFINE.svg`
- [ ] Phase 9: Claim-rail freshness sidebar (server.rs emit + CSS + JS + links.rs method)
- [ ] Phase 10: Reading state progress bar (CSS + JS, client-only MVP)
- [ ] Phase 11: `query_claims(topic, asof)` MCP method (mcp.rs + links.rs)
- [ ] cargo check → stage → commit (monorepo sub-clone)

## Pending Command Session

- Stage 6 promotion for all outstanding commits (Leapfrog + this session)
- Binary rebuild + deploy to 9090/9093/9095
- Phase 6 gate: GitHub repo renames + Doctrine amendment + service unit WIKI_CONTENT_DIR updates
- BRIEF redistribution: cross-archive BRIEFs sitting in this archive need pickup

## Blocked on operator

- UX-B.7: SVG located — `woodfine-media-assets/ASSET-WORDMARK-WOODFINE.svg` — operator approved inline this session

## Standing deferred

- Phase 12 (AI marginalia): gated on BP5 + SYS-ADR-07/10/19 review
- Phase 5.1+ (ACLs/OIDC/webhooks): gated on BP5 clearance
- Phase 6 (three-instance deployment split): gated on GitHub renames + Doctrine amendment
