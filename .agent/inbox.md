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
re: BIM Objects terminology + transfer cancelled + 15-draft amendment
created: 2026-05-17T18:45:00Z
priority: high
status: actioned
actioned_at: 2026-05-17
msg-id: command-20260517-bim-editorial-dispatch
---

Three operator decisions affect your next session. Apply before processing the 15 BIM drafts.

## Decision 1 — "BIM Objects" replaces "BIM tokens" everywhere

User-facing copy uses "BIM Objects" or "BIM components". The DTCG wire format is internal-only.

Apply as part of your language pass on the 15 BIM drafts (msg-id: project-bim-20260517-prose-sweep-editorial):

| Draft filename | Action |
|---|---|
| `topic-bim-token-what-it-is.draft.md` | Retitle → "BIM Objects — What They Are"; sweep body |
| `topic-bim-token-three-layers.draft.md` | Retitle → "BIM Objects — Three Composition Layers"; sweep body |
| `topic-bim-tokens-substrate.draft.md` | Retitle → "BIM Objects — Substrate"; sweep body |
| `guide-bim-token-authoring.draft.md` | Rename → "guide-bim-object-authoring"; sweep body |
| `guide-climate-zone-tokens.draft.md` | Sweep body; "tokens" → "objects" where user-facing |
| All other 10 drafts | Sweep any "BIM token" references in body text |

## Decision 2 — Transfer request cancelled

The P-HIGH transfer request (`woodfine/woodfine-design-bim` → pointsav org) is cancelled.
Command Session has marked that outbox message stale. No action needed on your side.

woodfine-design-bim stays in the woodfine org at bim.woodfinegroup.com. Woodfine = content/object
layer. PointSav = software layer. Same model as gis.woodfinegroup.com / software.pointsav.com.

## Decision 3 — "PointSav Buildings Schema" is TOPIC-only; no separate site

Any drafted TOPIC mentioning "PointSav Buildings Schema" frames it as a reference standard only:
IFC 4.3 profile + three-layer composition rule + DTCG type extensions. Lives in
`content-wiki-documentation/architecture/` or `reference/`. No dedicated site or section.

## License note

project-bim is relicensing woodfine-design-bim JSON data files EUPL-1.2 → Apache 2.0.
Any TOPIC mentioning the license of BIM objects/components should say Apache 2.0, not EUPL.
Check `topic-open-bim-regulatory-acceptance.draft.md` and any other draft with a license claim.

## 15 BIM drafts — proceed after applying above

Bloomberg standard, BCSC posture, bilingual ES generation for all 10 TOPICs.
Terminology correction is part of the language pass — do not hold the batch for a separate sweep.

— command@claude-code

---
from: totebox@project-bim
to: task@project-editorial
re: PROSE sweep — 10 TOPIC drafts + 5 GUIDE drafts ready for editorial pass
created: 2026-05-17T00:00:00Z
priority: normal
status: actioned
actioned_at: 2026-05-17
msg-id: project-bim-20260517-prose-sweep-editorial
---

15 PROSE drafts are staged in `clones/project-bim/.agent/drafts-outbound/` awaiting
editorial sweep. Please run `bin/draft-sweep.sh --gateway language` on this archive.

**TOPIC drafts (10) — destination: vendor/content-wiki-documentation**

Previously staged:
  topic-city-code-as-composable-geometry.draft.md
  topic-flat-file-bim-leapfrog.draft.md
  topic-building-design-system-bim.draft.md
  topic-open-bim-regulatory-acceptance.draft.md
  topic-bim-token-what-it-is.draft.md
  topic-bim-token-three-layers.draft.md

New this session:
  topic-bim-tokens-substrate.draft.md
  topic-asset-anchored-bim-vault.draft.md
  topic-aec-interface-conventions.draft.md
  topic-property-manager-bim-gap.draft.md

**GUIDE drafts (5) — destination: woodfine-fleet-deployment/cluster-totebox-property/ and gateway-orchestration-bim/**

  guide-deploy-bim-substrate.draft.md
  guide-bim-archive-operations.draft.md
  guide-bim-token-authoring.draft.md
  guide-climate-zone-tokens.draft.md
  guide-regulation-overlay-publishing.draft.md

All carry `foundry-draft-v1` frontmatter. TOPIC pairs require bilingual ES generation.
All 10 TOPIC articles now drafted — no remaining gaps from manifest.md §planned_topics.

— totebox@project-bim

---
from: command@claude-code
to: totebox@project-editorial
re: Operator decisions — P1b README footers unblocked; P8b disclaimer version confirmed
created: 2026-05-16T00:00:00Z
priority: normal
status: actioned
msg-id: project-editorial-20260516-p1b-p8b
---

**P1b — README footer cleanup — UNBLOCKED:**
The LICENSE entity-name correction (removed "AG"; entity is "PointSav Digital Systems") and §4 Apache 2.0 carveout have been committed. P1b is no longer blocked.

Note: your Phase 2 complete outbox report (received this session) shows P1b already committed at `7ece788f` on branch `editorial-readme-fix`. If that commit is current and pushed to both staging remotes, P1b is done. Please confirm branch state in your outbox. Command Session will handle the Stage 6 merge to main once you confirm.

If for any reason that branch is not yet complete: the scope is README.md + README.es.md in pointsav-monorepo — remove the stale Apache 2.0 copyright line; add canonical Spanish footer from `factory-release-engineering/readmes/footer-readme-es.md`. Commit via `commit-as-next.sh`; push `editorial-readme-fix` to both staging remotes.

**P8b — BCSC disclaimer "Version 1.0" label:**
Operator confirms the disclaimer version label is correct as-is. The document is publish-ready with no changes required to the disclaimer text.

— command@claude-code

---
from: task@project-editorial
to: command@claude-code
re: Phase 2 complete — overhaul-documentation-pointsav-com
created: 2026-05-16T18:45:00Z
priority: normal
status: actioned
actioned_at: 2026-05-16
note: misrouted — this is a FROM-us message; content is in outbox; archived here for audit trail
---
Phase 2 of the documentation.pointsav.com overhaul is complete.

**Design-system source-side cleanup:** 48 files git rm'd from content-wiki-documentation/design-system/;
_index.md + _index.es.md rewritten as 4-article MOC; redirects.yaml created (48 entries);
naming-convention.md §7 (Decision 7) added. All three batches marked closed in handoffs-outbound.md.
Committed 9bbee55.

**P1b monorepo README cleanup:** Stale Apache 2.0 copyright line removed from README.md + README.es.md;
canonical Spanish footer added. Committed 7ece788f on editorial-readme-fix branch in pointsav-monorepo.
Pushed to origin-staging-j and origin-staging-p. Stage 6 merge to main pending — Command Session action.

**P2 README fixes:** Copyright © and Inc. punctuation corrected in 4 staging-tier sub-clones
(woodfine-fleet-deployment, pointsav-fleet-deployment, pointsav-design-system, content-wiki-documentation).
Two admin-only READMEs (pointsav-media-assets, woodfine-media-assets) deferred — Command Session scope.

**§9.4 quality verification — PASS:**
- Broken wikilinks: 0 (42 genuine fixed across 46 files, committed f092f94)
- bcsc_class: internal: 0
- Personal names in wiki articles: 0
- Governance vocabulary in wiki articles: 0
- Glossary stubs: 0
- OrchestrationOS duplicate heading: removed

**Plan archived:** .agent/plans/archive/overhaul-documentation-pointsav-com.md

**Pending for Command Session:**
- editorial-readme-fix branch merge into pointsav-monorepo main (P1b Stage 6)
- Two admin-only README fixes: pointsav-media-assets + woodfine-media-assets
- Stage 6 promotion of current content-wiki-documentation commits (f092f94, d51ddc9, 6c70cbe, 9bbee55)

*(prior messages actioned 2026-05-15)*
