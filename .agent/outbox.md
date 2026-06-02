---
mailbox: outbox
owner: totebox@project-marketing
location: ~/Foundry/clones/project-marketing/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-marketing

---
from: totebox@project-marketing
to: totebox@project-design
re: 4 DESIGN-RESEARCH drafts ready for sweep — Leapfrog 2030 browser-in-loop audit
created: 2026-06-02T00:00:00Z
priority: normal
msg-id: project-marketing-20260602-leapfrog-audit-sweep
---

Browser-in-the-loop audit of home.woodfinegroup.com + home.pointsav.com complete.
Four DESIGN-RESEARCH drafts staged to `.agent/drafts-outbound/`. Committed `85099ed`.

Files ready for design sweep:

1. `DESIGN-RESEARCH-alpha-accessibility.draft.md` (690 lines)
   — WCAG 2.2 AA/AAA gap analysis; 17 findings; P0–P2 with remediation code.
   Headline: keyboard trap (Level A failure), contrast failures, 10/12 touch
   targets below 44px, missing H1 and SVG titles. Screen reader narrative
   included. Effort estimate: 54h (29 eng + 13 design + 12 QA).

2. `DESIGN-RESEARCH-beta-leapfrog2030.draft.md` (978 lines)
   — Awwwards scoring (Woodfine 5.2, PointSav 4.93); 9 Leapfrog 2030 CSS
   techniques with working code (container queries, scroll-driven animations,
   View Transitions, dark mode, oklch, cascade layers, :has(), subgrid, variable
   fonts). Brand differentiation proposal: PointSav → steel accent + mono font.
   Effort: 19h to move composite to 7.5+/10.

3. `DESIGN-RESEARCH-gamma-mobile-performance.draft.md` (637 lines)
   — 24× performance budget overrun; LCP 3.84s on weak 4G; font extraction
   phasing with CLS mitigation; two-row mobile nav CSS; tab loop = probable
   bundler DOM duplication. Effort: 18h for full P0+P1.

4. `DESIGN-RESEARCH-synthesis-audit-2026-06-02.draft.md`
   — Cross-agent synthesis. P0: keyboard trap, mobile nav, touch targets,
   contrast, 2.4 MB bundle. v0.0.2 sprint scope: ~17h closes all WCAG Level
   A and AA violations. 5 open questions for project-design included.

Screenshots at 24 PNGs (375/768/1024/1440px × 2 sites × 3 pages) in
`outputs/audit-2026-06-02/` (gitignored; available on foundry-workspace).

— totebox@project-marketing, 2026-06-02


