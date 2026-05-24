---
artifact: brief
status: active
topic: knowledge-platform — Phases 1–5 done; design competition in progress; Phase 6 gated; product strategy open
archive: project-knowledge
created: 2026-05-22
updated: 2026-05-24
owner: totebox@project-knowledge
---

# BRIEF — Knowledge Platform (consolidated)

## Mission

Build and ship `app-mediakit-knowledge` — a sovereign-data Wikipedia-pattern HTTP wiki engine.
Not MediaWiki (PHP, heavy, not sovereign). Not Hugo (static, no auth, no edit workflow, no search-as-you-type).
A Rust binary with Git-native content, Tantivy BM25 search, MCP JSON-RPC 2.0, bilingual routing,
claim-layer citations, and an edit review queue. Three live instances:
- `documentation.pointsav.com` → `local-knowledge-documentation.service` (port 9090)
- `projects.woodfinegroup.com` → `local-knowledge-projects.service` (port 9093)
- `corporate.woodfinegroup.com` → `local-knowledge-corporate.service` (port 9095)

## Implementation state — Phases 1–5 complete (2026-05-23)

All on `pointsav-monorepo` `main`. 17 commits promoted via Stage 6. Binary rebuilt. Services live.

| Phase | Status | Key commits |
|---|---|---|
| 1 — render + chrome | Shipped | Route /wiki/{slug}, TOC, hatnote, tabs |
| 1.1 — Wikipedia chrome | Shipped | Article/Talk/History tabs, language switcher, footer |
| 2 — edit (Steps 1-7) | Shipped | JSON-LD, atomic edit, CodeMirror 6, citation autocomplete |
| 3 — search + feeds | Shipped | Tantivy BM25, /feed.atom, /sitemap.xml, /llms.txt |
| 4 — Git sync + MCP | Shipped | git2, redb, blake3, MCP JSON-RPC 2.0, git smart-HTTP, OpenAPI 3.1 |
| 4 DTCG | Shipped | dtcg-bundle.json → dtcg-to-css.py → tokens.css + tokens-woodfine.css |
| 5 — auth + edit review | Core shipped | Cookie sessions, argon2id, edit review queue |
| 5.1+ ACLs / OIDC / webhooks | Deferred | Gated on BP5 clearance |

## Phase 6 — THREE-INSTANCE DEPLOYMENT SPLIT (gated)

**Gate 1 (operator, GitHub UI):** Rename jwoodfine/pwoodfine staging forks from `content-wiki-{documentation,projects,corporate}` → `media-knowledge-{documentation,projects,corporate}` (6 repos). Canonical remotes already updated.

**Gate 2 (Command Session):** MASTER Doctrine amendment — source-of-truth inversion for media-knowledge-* repos (Totebox clone = canonical; GitHub = downstream mirror).

**Gate 3 (after 1+2):** Command updates service unit `WIKI_CONTENT_DIR` for projects + corporate to read from Totebox clone paths (not `/srv/foundry/customer/` paths).

Nothing for Totebox to do until Command confirms gates 1+2 clear.

## Fleet catalog drift (outboxed to Command)

Two MANIFEST entries need correction:
- `media-knowledge-projects`: port 9091→9093, state planned→active
- `media-knowledge-corporate`: port 9092→9095, state planned→active

## Active work stream — UI/UX design competition

Three prior attempts failed because they worked incrementally within the existing CSS.
Current approach: fresh start, four competing philosophies, extract DTCG tokens backwards from the winner.

**Four HTML prototypes complete** (all in `.agent/drafts-outbound/`):

| File | Philosophy | Lines | Key trait |
|---|---|---|---|
| `DESIGN-COMPETITION-A-stripe-precision.html` | Authority through restraint | 1,486 | Dominant search bar, zero decorative borders, floating editor pill |
| `DESIGN-COMPETITION-B-wikipedia-evolved.html` | Encyclopedic register | 1,797 | Left-rail TOC, Zilla Slab serif lead, scroll-collapse header |
| `DESIGN-COMPETITION-C-enterprise-learn.html` | Metadata is content | 1,511 | Audience-routed tiles, tree sidebar, citation accordion |
| `DESIGN-COMPETITION-D-brand-continuity.html` | Zero visual discontinuity | 1,763 | wireframe-v2c token system, magazine layout, footnote citations |

**Jury agent:** Launched 2026-05-24 (OPUS). Output: `DESIGN-COMPETITION-JURY-REPORT.draft.md`.
**After jury:** Select winner or hybrid → implement in `src/server.rs` + `static/style.css` → extract tokens backwards into `scripts/dtcg-bundle.json`.

**Footer defect in prototypes (fix in implementation pass, not jury pass):**

Canonical footer (from `wireframe-home-header-v2c.html`):
```
© 2026 Woodfine Capital Projects Inc. All rights reserved.
Woodfine Capital Projects™, Woodfine Management Corp™, PointSav Digital Systems™,
Totebox Orchestration™, and Totebox Archive™ are trademarks of Woodfine Capital
Projects Inc. used in Canada, the United States, Latin America, and Europe. All other
trademarks are the property of their respective owners.
```

Designs A/B/C are missing parts or the entire trademark block. Design D adds a redundant
`PointSav is a trademark of PointSav Digital Systems` line not in the canonical text.
Implementation agent must use canonical text verbatim.

## "21st-century Wikipedia" design principle (mandatory in all prototypes)

The reading surface must look like a great magazine article, not a CMS admin panel.

| Reader state | Visible UI |
|---|---|
| Anonymous reader | Clean title + lead + body. Zero edit controls. Status badge = coloured dot (expandable). Citation ribbon at bottom, collapsed. |
| Logged-in contributor | Edit pencil on section hover. Full tools, not dominant. |
| Mobile | All toolbars behind "..." overflow. |

Talk/Discussion tabs: **never visible to anonymous readers.** History: accessible via "..." not a prominent tab.

## Product strategy open questions (2026-05-24)

These need answers before Phase 7+ scoping. Market research agent launched 2026-05-24.

**Why not MediaWiki?** PHP, MySQL, heavy ops, not sovereign-data, Wikipedia-branded not product-branded, no modern TypeScript frontend, no claim-layer.

**Why resist Hugo?** Hugo is fast and Git-native but fundamentally static: no search-as-you-type, no auth-gated content, no edit workflow, no revision history UI, no claim verification, no MCP API. "Never feels complete" because static = no server-side capability. Our engine provides all of that without giving up the flat-file + Git-native content model.

**Q4 Inc as competitor:** Q4 (Toronto; TSXV: QFOR) serves public-company IR teams — IR websites, earnings call webcast, investor CRM, analytics. Their gap: no sovereign-data claim layer, no bilingual structured content, no edit review queue for regulatory disclosure text. Our overlap: regulated-content management for public companies. Our differentiation: claim-layer citation verification, BCSC/OSC disclosure posture baked in, Git-native audit trail, no vendor lock-in on the content store.

**Google Cloud Console anti-pattern:** Feature-sprawl navigation — hundreds of product pages organized by product family, not by operator task. The anti-pattern is hierarchy depth: users must know the product name before they can find the capability. Our wiki engine organizes by topic/audience, not by product namespace.

**Leapfrog 2030 direction:** The combination of claim-layer (§3.6 MCP API + service-slm), BCSC-verified disclosure posture, Git-native audit trail, and bilingual routing creates a compliance-grade knowledge layer that Confluence/Notion/Gitbook cannot offer regulated-industry clients. The 2030 differentiation: `query_claims(topic, asof)` cross-cluster MCP API lets AI agents verify whether a disclosure claim has changed since a given date — a capability no current platform has.

## Key files

| File | Role |
|---|---|
| `app-mediakit-knowledge/src/server.rs` | Main HTTP handler, routing, AppState |
| `app-mediakit-knowledge/src/claim.rs` | Claim model + extractor (Phase 3 A/D) |
| `app-mediakit-knowledge/scripts/dtcg-bundle.json` | DTCG canonical token vault |
| `app-mediakit-knowledge/scripts/dtcg-to-css.py` | Token → CSS generator |
| `app-mediakit-knowledge/static/style.css` | Main stylesheet |
| `.agent/drafts-outbound/DESIGN-COMPETITION-*.html` | Four competing UI prototypes |
| `.agent/drafts-outbound/DESIGN-COMPETITION-JURY-REPORT.draft.md` | Jury output (pending) |
| `wireframe-home-header-v2c.html` | Ratified three-row header/footer pattern |

## Commit + promote path

`~/Foundry/bin/commit-as-next.sh "<msg>"` from `pointsav-monorepo/` sub-clone.
Stage 6 via Command Session `bin/promote.sh`. After promote: `bin/sync-local.sh --all` + `sudo systemctl restart local-knowledge-{documentation,projects,corporate}`.

## Open items for this BRIEF

- [ ] Jury report review → select winner/hybrid → implement
- [ ] Fix footer trademark in winning implementation (use canonical text above)
- [ ] Stage 6 staging fork renames (6 GitHub repos — operator action)
- [ ] MASTER Doctrine amendment (Command scope)
- [ ] §3.6 claim-record MCP API — waiting on project-intelligence reply
- [ ] §3.4 continuous citation verification — own sub-project, not yet scoped
- [ ] Phase 5.1+ (ACLs, OIDC, webhooks) — gated on BP5
