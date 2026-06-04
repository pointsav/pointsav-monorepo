# Session context — project-marketing

Rolling 3-session summary. Newest entry first. Keep 3 entries max; push oldest to session-context-archive.md.

---

## 2026-06-04 | totebox | claude-code

**Done this session:**
- Full mobile improvement sprint for both marketing sites. All changes committed and live.
- 4 commits landed this session:
  - `ee0e07e5` (Jennifer) — S5: hide Disclaimer/Contact us from mobile header; footer links remain
  - `78bf2890` (Peter) — S6: stack Manifest/BIM Library/Location Intelligence equally on Woodfine mobile
  - `114ec305` (Peter) — S7: collapsible `<details>/<summary>` disclaimer on both sites
  - `41c6b1cf` (Peter) — created `scripts/apply-mobile-fixes.sh` (idempotent; S1–S4, W1, P1, P2)
- Hyperscaler mobile research (Opus agent): Apollo, Brookfield, Carlyle, Blackstone, Prologis.
  Key patterns: single-column stacking, equal-card atomic unit, ≥16px body, hero imagery.
  Saved in plan file `/home/jennifer/.claude/plans/on-the-mobile-sites-compiled-coral.md`.
- `scripts/apply-mobile-fixes.sh` now covers S1–S7 + W1, P1, P2 — run it after any site rebuild.

**Pending / carry-forward:**
- UX audit (10 items from 2026-06-03): server-render HTML, font self-hosting, hero typos (Woodfine
  "AN real property developer", PointSav "F*KEYS CONSSOLE"/"DIGTIAL TWIN"), nav font min 14px, add h1.
  None of these were actioned this session — separate sprint needed.
- GSC sitemaps + Bing Webmaster Tools: operator-gated (operator must do in browser).
- NEXT.md and outbox.md are contaminated with content from project-bim/project-knowledge/
  project-intelligence archives. Need Command Session cleanup or a dedicated Totebox session.
- Future mobile improvements from hyperscaler research: hero photography, hamburger nav,
  full-width CTAs repeated under each card, persistent Enquire/click-to-call.

**Operator preferences surfaced:**
- Prefers auto mode (no confirmation pauses mid-task); confirmed this session.
- Likes Opus agents for research tasks ahead of implementation.
- Likes plan-then-execute flow with ExitPlanMode approval.

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


