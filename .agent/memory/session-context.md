# Session context — project-marketing

Rolling 3-session summary. Newest entry first. Keep 3 entries max; push oldest to session-context-archive.md.

---

## 2026-06-01 | totebox | claude-code

**Done this session:**
- Startup: role confirmed, lock written, 1 inbox message (H-1..H-10 rollout). Actioned and archived.
- Produced all outstanding artifacts for this cluster:
  - `GUIDE-provision-marketing-site.draft.md` — provision a new instance from scratch (customer leg)
  - `GUIDE-deployment-marketing-site.draft.md` — deploy/update binary on running instance (customer leg)
  - `TOPIC-app-mediakit-marketing.draft.md` — WordPress-leapfrog architecture background (wiki leg; condition met: MVP running)
  - All 3 staged to `.agent/drafts-outbound/` → gateway: project-editorial
- Prior design drafts remain staged: 3 in `.agent/drafts-outbound/` (icon-tab component + 2 tokens), 10 in `.claude/drafts-outbound/leapfrog-2030/` (Leapfrog 2030 batch → project-design).

**Pending / carry-forward:**
- Bootstrap deploys + certbot TLS still operator-gated (NEXT.md).
- Source-level `<title>` fix on next bundle rebuild (NEXT.md).
- `TOPIC-app-mediakit-marketing.draft.md` needs bilingual pair (`.es.md`) — flagged in frontmatter.

**Operator preferences surfaced:**
- No new preferences this session.

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

