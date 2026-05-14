---
mailbox: inbox
owner: task@project-editorial
location: ~/Foundry/clones/project-editorial/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-editorial Task

---
from: command@claude-code
to: task@project-editorial
re: ZIP research archive — framework + per-topic drafts staged (245 emails, Dec 2025–Apr 2026)
created: 2026-05-14
priority: normal
---
Staged at .agent/drafts-outbound/:
- framework-pointsav-wiki-structure.md — proposed wiki structure for documentation.pointsav.com
- zip-topic-*.md — 27 individual topic drafts from email research archive

Source: 245 development research emails archived by Mathew for the PointSav Documentation Wiki.
Most content is architectural/product research. Some is [STALE?] or [SUPERSEDED?] — flagged inline.

Requests from MASTER:
1. Use tables wherever possible to present product/service/architecture data.
2. Where ZIP content conflicts with current canonical names/conventions, prefer canonical and
   flag the old name with [SUPERSEDED BY: ...] rather than deleting.
3. Do NOT duplicate content already live in content-wiki-documentation/ — cross-reference only.
4. Route any content that is better suited as a GUIDE-* to woodfine-fleet-deployment context.
5. Internal document — full language + BCSC review before any public use.

Companion master framework (Command-Session-scope, not for editorial commit):
  /srv/foundry/.agent/plans/framework-pointsav-products-services.md

Flagged for operator attention (see framework §10 + per-draft notes_for_editor):
- [BCSC-REVIEW-REQUIRED] Sovereign Data Foundation language — email #017 claims 10% equity stake;
  current posture is planned/intended only
- [SECRET-EXPOSURE] Email #202 contains plaintext credentials; email #196 reports a committed
  Azure AD Application Secret — confirm rotation/revocation
- [POTENTIAL-DOCTRINE-CONFLICT] Email #163 (service-content + Gemini API autonomous publishing)
  and email #220 (Gemini-generated L4 taxonomy) — confirm human-in-the-loop verification per
  SYS-ADR-07 and SYS-ADR-19
- [LEGAL-ENTITY-CHECK] "PointSav Digital Systems AG" suffix appears in emails #017, #019,
  #022, #025; confirm current canonical legal entity name with operator
- [DESIGN-SYSTEM-DRIFT] Emails place BIM content under pointsav-design-system; project memory
  project_two_design_systems.md says BIM belongs in separate woodfine-design-bim → confirm

— command@claude-code


