---
artifact: brief
status: archived
contamination_note: >-
  Contaminated in project-data; belongs to project-knowledge. Command: redistribute to clones/project-knowledge/.agent/briefs/
archived_date: 2026-06-01
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
- [x] BRIEF consolidation — Gemini handover archived; BRIEF-active-work created
- [x] Revert Gemini's threading — no revert needed; source was clean (Gemini never committed)
- [x] Wire `toc-persistence.js` into `wiki_chrome()` — DONE: file created + server.rs wired
- [x] UX-B.7: Woodfine SVG wordmark — DONE: WORDMARK_WOODFINE constant updated to SVG inline
- [x] Phase 10: Reading state progress bar — DONE: CSS + JS added to source (was already in binary)
- [x] Phase 9 CSS + JS: claim-rail — DONE: CSS + JS added to source (was already in binary)
- [x] Fix #p-views display — hidden; duplicate tab bar below article title removed
- [x] Recover binary-vs-source divergence — source now matches binary on Phase 9+10 + toc-persistence
- [ ] Fix links.rs `exists()` method — scoped to Phase 9 server.rs emit (claim-rail needs it)
- [ ] Phase 9: server.rs emit of `<aside class="claim-rail">` + links.rs exists() method
- [ ] Phase 11: `query_claims(topic, asof)` MCP method (mcp.rs + links.rs)
- [ ] cargo check → stage → commit (monorepo sub-clone) — in progress

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
