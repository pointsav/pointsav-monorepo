# Session context archive — project-marketing

Entries pushed from session-context.md when rolling window exceeds 3.

---

## 2026-05-28 | totebox | claude-code

**Done this session:**
- Resumed from context summary (prior session had built BIM Library + Location Intelligence pages, applied SEO, robots/sitemap).
- Generated landscape PDFs of both home pages (home.woodfinegroup.com, home.pointsav.com) using WeasyPrint — extracted inner HTML from bundler JSON string to work around JS rendering. Rendered at 1400×900px.
- Created `outputs/` folder: added `.gitkeep` + gitignore rule (`outputs/*`, `!outputs/.gitkeep`). Committed as `6fa271b` (Peter Woodfine).
- Deleted both PDFs per operator request (folder remains tracked in git).

**Pending / carry-forward:**
- Bootstrap deploys + certbot TLS still operator-gated.
- Source-level `<title>` fix on next bundle rebuild (NEXT.md).

**Operator preferences surfaced:**
- WeasyPrint available on VM at `/usr/bin/weasyprint` (v61.1). Use for future HTML→PDF needs.

---

## 2026-05-24 | totebox | claude-code

**Done this session:**
- Startup: role confirmed, lock written, 1 inbox message (binary-targets) + 1 new ACK from project-design.
- Wrote `.agent/binary-targets.yaml` — declared app-mediakit-marketing (FSL-1.1-ALv2, app-bundle, extension layer, requires os-console). Inbox actioned and archived.
- Applied SEO head blocks to both home pages: meta description, canonical, Open Graph, Twitter card, JSON-LD Organization schema. Verified live on home.pointsav.com (port 9101) and home.woodfinegroup.com (port 9102).
- Added robots.txt + sitemap.xml to both sites via nginx static location blocks. Verified live via HTTPS.
- Archived all 3 stale outbox messages (project-design ACK confirmed icon-tab + woodfine-blue-tint committed; Stage 6 confirmed resolved). Outbox is clean.
- NEXT.md cleared of all completed SEO items. session-start.md updated.

**Operator preferences surfaced:**
- No new preferences this session.

---

## 2026-05-21 | totebox | claude-code

**Done this session:**
- Fixed browser tab title on `home.pointsav.com`: `PointSav, Inc. — Home` → `PointSav — Home`. Applied directly to gitignored deployment file (live immediately).
- Noted source-level fix needed on next bundle rebuild in NEXT.md.

**Operator preferences surfaced:**
- No new preferences this session.

## 2026-05-20 | totebox | claude-code

**Done this session:**
- Startup: role confirmed, lock written, inbox clear, no NOTAM blockers.
- Researched SEO gap for brand searches ("pointsav", "woodfinegroup").
- Drafted full SEO head blocks for both home pages. Operator approved preview; applied 2026-05-24.

**Pending / carry-forward:**
- All completed 2026-05-24. ✓

**Operator preferences surfaced:**
- No preferences flagged this session.
