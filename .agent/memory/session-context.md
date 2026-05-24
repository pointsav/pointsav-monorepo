# Session context — project-marketing

Rolling 3-session summary. Newest entry first. Keep 3 entries max; push oldest to session-context-archive.md.

---

## 2026-05-24 | totebox | claude-code

**Done this session:**
- Startup: role confirmed, lock written, 1 inbox message (binary-targets) + 1 new ACK from project-design.
- Wrote `.agent/binary-targets.yaml` — declared app-mediakit-marketing (FSL-1.1-ALv2, app-bundle, extension layer, requires os-console). Inbox actioned and archived.
- Applied SEO head blocks to both home pages: meta description, canonical, Open Graph, Twitter card, JSON-LD Organization schema. Verified live on home.pointsav.com (port 9101) and home.woodfinegroup.com (port 9102).
- Added robots.txt + sitemap.xml to both sites via nginx static location blocks (`/etc/nginx/sites-available/home.pointsav.com` + `home.woodfinegroup.com`). Verified live via HTTPS.
- Archived all 3 stale outbox messages (project-design ACK confirmed icon-tab + woodfine-blue-tint committed; Stage 6 confirmed resolved). Outbox is clean.
- NEXT.md cleared of all completed SEO items. session-start.md updated.

**Pending / carry-forward:**
- None. Archive is clean.
- Bootstrap deploys (operator-gated): both `media-marketing-landing-1/2` still awaiting certbot TLS.
- Source-level `<title>` fix on next bundle rebuild (NEXT.md).

**Operator preferences surfaced:**
- No new preferences this session.

---

## 2026-05-21 | totebox | claude-code

**Done this session:**
- Startup: role confirmed, lock written, inbox clear, no NOTAM blockers.
- Fixed browser tab title on `home.pointsav.com`: removed ", Inc." from `PointSav, Inc. — Home` → `PointSav — Home`. Change applied directly to gitignored deployment file `deployments/media-marketing-landing-2/content/index.html` (live immediately).
- Noted source-level fix needed on next bundle rebuild in NEXT.md (committed `10abb33`).

**Pending / carry-forward:**
- SEO head blocks applied this session (2026-05-24). ✓

**Operator preferences surfaced:**
- No new preferences this session.

---

## 2026-05-20 | totebox | claude-code

**Done this session:**
- Startup: role confirmed, lock written, inbox clear, no NOTAM blockers.
- Researched SEO gap for brand searches ("pointsav", "woodfinegroup").
- Drafted full SEO head blocks for both home pages. Operator approved preview; applied 2026-05-24.

**Pending / carry-forward:**
- All completed 2026-05-24. ✓

**Operator preferences surfaced:**
- No preferences flagged this session.
