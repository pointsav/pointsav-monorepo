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
re: build-request — app-mediakit-knowledge Phase 2+3 + content stubs — Stage 6 + binary rebuild needed
created: 2026-05-27T00:00:00Z
priority: high
status: pending
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
