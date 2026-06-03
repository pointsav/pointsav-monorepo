# Session context — project-marketing

Rolling 3-session summary. Newest entry first. Keep 3 entries max; push oldest to session-context-archive.md.

---

## 2026-06-03 | totebox | claude-code

**Done this session:**
- Read UX audit memo from project-knowledge (HIGH, msg-id: project-knowledge-20260603-ux-audit-memo). 10 findings logged to NEXT.md. Status: pending — needs dedicated sprint.
- Made plan + executed Google indexing work for both marketing sites:
  - Added `<noscript>` content blocks (company name, description, nav links) to both live HTML files so Google can index without JS execution
  - Added Google Search Console verification meta tags to both sites (Woodfine: `hLwqLQ2f_Mq0iD4j-S3vI1nXRTMYTQ2a0kqVKudXgSg`, PointSav: `QxxuzjyN_fGYt-QdoYq6pA7J1MdMKPioeBg_KHEkRwQ`)
  - Both tags confirmed live via curl against ports 9102 / 9101
- Both sites verified healthy (HTTP 200, viewport patch applied).

**Pending / carry-forward:**
- GSC: operator to submit sitemaps + request indexing in GSC (operator-gated — NEXT.md).
- Bing Webmaster Tools import (optional, NEXT.md).
- UX audit sprint: 10 items in NEXT.md — most impactful: server-render HTML, font self-hosting, hero typos, nav size.

**Operator preferences surfaced:**
- Prefers to paste verification codes directly into chat rather than running edit scripts manually.

---

## 2026-06-02 | totebox | claude-code

**Done this session:**
- Startup: role confirmed, lock written, 1 inbox (H-1..H-10 rollout). Actioned.
- Browser-in-the-loop audit: Playwright 1.60.0 + axe-core. 24 PNG screenshots, 36 JSON files → `outputs/audit-2026-06-02/`.
- Launched 3 competing Opus agents (parallel) — Alpha (accessibility), Beta (Leapfrog 2030 design), Gamma (mobile + performance).
- Synthesis document written from all three agent outputs.
- 4 DESIGN-RESEARCH drafts committed (`85099ed`, Peter Woodfine) + outbox to project-design.
- Key findings: keyboard trap (Level A), mobile nav unusable at 375px, 10/12 touch targets fail WCAG 2.5.5, 2.4 MB bundle (24× over budget), 4–5 axe violations per site, no H1, all SVGs missing title, no skip link.

**Pending / carry-forward:**
- project-design to sweep 4 DESIGN-RESEARCH drafts + 10 prior Leapfrog 2030 drafts.
- v0.0.2 sprint: ~17h closes all WCAG Level A/AA violations + mobile nav.
- Bootstrap deploys + certbot TLS operator-gated (NEXT.md).

**Operator preferences surfaced:**
- Requested competing Opus agents for design audit — will use this pattern again.

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



