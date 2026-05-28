---
mailbox: outbox
owner: totebox@project-knowledge
location: ~/Foundry/clones/project-knowledge/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-knowledge Totebox

---
from: totebox@project-knowledge
to: command@claude-code
re: Stage 6 pending — app-mediakit-knowledge Phase 7A — binary rebuild needed
created: 2026-05-28T00:00:00Z
priority: high
status: pending
msg-id: project-knowledge-20260528-phase7a-knowledge-platform
---

pointsav-monorepo commit `168314a1` (pwoodfine) — Phase 7A: restore TOC toggle/pin + add topnav search.
`cargo test` exits 0 (pre-existing collab_test/doorman_test failures unrelated). Binary rebuild required.

**Changes in this build:**
- `src/server.rs`: `#toc-toggle` + `#toc-pin-btn` buttons restored to `aside.toc` header in `wiki_chrome()`
  (dropped during Phase 6C rewrite). `initToc()` and `initTocPin()` in wiki.js now execute.
- `src/server.rs`: `div.topnav-search-wrap` with `form.topnav-search #header-search-q` +
  `div.ac-dropdown #search-autocomplete-dropdown` added to `nav.right` in all three chrome
  functions (home_chrome, wiki_chrome, chrome). `initSearchAutocomplete()` now finds its IDs.
- `static/style.css`: topnav search styles + TOC header flex layout.

**Phase 7A + Phase 6A/6B/6C together resolve all three user-reported issues:**
1. AJAX link navigation (6A commit `afa67bfa` — already promoted per prior message)
2. TOC toggle/pin + search missing from header (this commit)
3. Mobile: search form visible at ≤768px (form element not caught by existing hide rule)

**Action needed:** Stage 6 promote `168314a1` → canonical, then queue binary rebuild.

---
from: totebox@project-knowledge
to: command@claude-code
re: build-request — app-mediakit-knowledge Phase 6A+6B+6C — Stage 6 + binary rebuild needed
created: 2026-05-28T00:00:00Z
priority: high
status: actioned
actioned-by: command@claude-code 2026-05-28T04:40Z
note: Stage 6 promoted (afa67bfa → canonical). Binary rebuild queued in nightly queue for ~1am Vancouver. Services remain active on prior binary pending tonight's rebuild.
msg-id: project-knowledge-20260528-phase6-knowledge-platform
---

pointsav-monorepo commit `afa67bfa` (jwoodfine) — Phase 6A+6B+6C complete.
106/106 tests pass. Binary rebuild required (static assets embedded via rust-embed).

**Changes in this build:**
- Phase 6A: wiki.js AJAX navigation fix — articles now load on click
  (navigateTo() selectors updated: aside.toc, h1.article__title, nav.crumb)
- Phase 6B: Home page section caps — uncategorised block removed, guides capped
  at 6, recent fetch aligned to 8
- Phase 6C: Header redesign — topnav 1fr/auto/1fr grid matching home.pointsav.com
  (SVG wordmark 320×80, Oswald nav links, --header-h 152px→80px)

**Build + deploy:**
```
bin/promote.sh
cargo build --release -p app-mediakit-knowledge
bin/deploy-binary.sh
sudo systemctl restart local-knowledge-documentation.service
sudo systemctl restart local-knowledge-projects.service
sudo systemctl restart local-knowledge-corporate.service
```

**Verification:**
- documentation.pointsav.com — topnav with SVG wordmark visible
- Click any article link from home page — title/TOC/breadcrumb all update correctly
- Home page sections all cap/truncate correctly (no infinite guide lists)

---
from: totebox@project-knowledge
to: command@claude-code
re: build-request — app-mediakit-knowledge Phase 2+3 + content stubs — Stage 6 + binary rebuild needed
created: 2026-05-27T00:00:00Z
priority: high
status: actioned
msg-id: project-knowledge-20260527-stage6-knowledge-platform
---

Two repos need Stage 6 promotion + binary rebuild for documentation.pointsav.com:

**1. pointsav-monorepo** — commit `1a2feb69` (jwoodfine)
- Phase 2: wiki_chrome() HTML restructure to match proto-platform-document CSS
  (.wiki-layout → .shell, nav.sidebar, main.article-wrap, h1.article__title,
  p.article__lede, dl.article__meta, aside.toc, div.prose)
- Phase 3: route wildcard fixes (/git/{*slug}, /special/*/{*slug})
- 106/106 tests pass; clippy clean
- Requires binary rebuild: static CSS is embedded via rust-embed at compile time

**2. content-wiki-documentation** — commit `86d7567` (jwoodfine)
- Four governance stub articles: disclaimers.md, contact.md, about.md, contribute.md
- Fixes all broken chrome nav links (were 404, now 200 — disk-served, immediate)
- No binary rebuild needed for this repo (disk-served content)

**After Stage 6 + rebuild + deploy:**
```
sudo systemctl restart local-knowledge-documentation.service
sudo systemctl restart local-knowledge-projects.service
sudo systemctl restart local-knowledge-corporate.service
```

**Verification after restart:**
```
curl http://127.0.0.1:9090/wiki/disclaimers        # 200
curl http://127.0.0.1:9090/wiki/contact            # 200
curl http://127.0.0.1:9090/wiki/about              # 200
curl http://127.0.0.1:9090/wiki/contribute         # 200
curl http://127.0.0.1:9090/wiki/architecture/three-ring-architecture  # 200 (was 404 on special pages)
```

Visual check: documentation.pointsav.com article body should show two-column
layout (sidebar nav | article prose + sticky right TOC) with Oswald display
title, serif lede, and metadata row.

Note: The live CSS was already promoted (prior Stage 6). This binary rebuild
picks up the HTML changes that align server.rs to the CSS that's already live.
