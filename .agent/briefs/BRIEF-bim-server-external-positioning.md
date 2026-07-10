---
artifact: brief
schema: foundry-brief-v1
archive: project-bim
topic: bim-server-external-positioning
status: active
created: 2026-07-10
updated: 2026-07-10
---

# Brief — bim.woodfinegroup.com as a BIM server other companies use for their own BIM Library

## Context

Operator's framing (2026-07-10): think about bim.woodfinegroup.com the way project-design thinks about
design.pointsav.com — a platform other companies could adopt for their own use, the way people use IBM
Carbon Design System's tokens directly without thinking about the server behind them. Asked for a
browser-in-the-loop look at the local PointSav Design System v3 mockup (`127.0.0.1:8899`) using Opus and
Fable independently, to see how that positioning is (or isn't) achieved there, and whether the same idea
applies to BIM Objects.

## What the research found

**PointSav Design System v3 mockup (127.0.0.1:8899) — both Opus and Fable audited independently, real
headless-browser verification, converged on the same verdict:** the mockup currently inverts the Carbon
test. The homepage hero is literally `DESIGN-SYSTEM SERVER` / "The design-system server you run yourself"
— infrastructure is the headline, not the tokens. Interior pages (Foundations, the Button component page,
Writing, Paper) are genuinely Carbon-grade and token-forward. The `registry.json`-as-single-source-of-truth
device and the `extensions.html` page ("one registry, badged by origin") are the best "server recedes"
mechanisms already present on the site — under-used, not absent. Both audits recommend the same fix: rewrite
the hero/footer to lead with objects, demote "runs on your own hardware" to a secondary section. This is
IA/copy triage, not a rebuild. **Not project-bim's to act on — flagging for project-design's awareness only,
since it's their mockup.**

**bim.woodfinegroup.com (127.0.0.1:9096) — Fable's follow-on audit, browser + HTTP verified:** the live
site's front-of-house positioning is already closer to the target than the design mockup (hero = the
Object catalog; platform reduced to one footer line). But three concrete, verified gaps make the "other
companies build their own library on this" idea aspirational, not shipped, today:

1. **Downloaded IFC exports are stubs.** The Leap V2 object's `.ifc` download contains only
   `IFCBOUNDINGBOX` + spatial scaffolding — zero `IfcPropertySet` entities, 2.1 KB total. The site's own
   copy promises "what you specify is what every downstream tool receives" and cites fire-rating data as
   an example of what a BIM Object "already knows" — the actual downloadable artifact carries none of it.
2. **License signals conflict on the same page — the important one.** Footer text reads "BIM Object data
   licensed Apache-2.0 · platform code AGPL-3.0-or-later," while the adjacent license badge reads "LICENSED
   CC BY-ND 4.0" (linking to Creative Commons). CC BY-ND explicitly forbids derivative works. "Build your
   own BIM library from these Objects" is, by definition, a derivative use. **If CC BY-ND is meant to
   govern the Object data (not just site prose), it directly kills this entire positioning idea** — not a
   detail, a contradiction. This sits next to the operator's own confirmed CC BY-ND decision logged
   2026-07-09 in `.agent/rules/cleanup-log.md` — that decision was scoped as a licensing call in the
   moment, not explicitly re-scoped against "external companies redistributing/adapting Object data." It
   needs its own explicit answer, not a default inherited from the earlier confirmation.
3. **Catalog breadth doesn't match its own prose.** Homepage copy: "Objects are the parts — a wall type, a
   workstation, a luminaire." The live catalog is 7 objects, all furniture (5 Steelcase, 1 Carl Hansen &
   Son, 1 generic), no walls, no luminaires. Footer claims "24 BIM Object categories · 18 components"; the
   Objects page shows "7 of 7 objects."

**The honest Carbon analogy, and where it breaks:** Carbon components run in YOUR app; the BIM equivalent
would be BIM Objects loading into YOUR authoring tool (Revit/ArchiCAD/BlenderBIM via IFC) and living in YOUR
property archive. That mapping holds. It breaks on: (a) no update channel — a placed BIM Object is a frozen
copy, unlike `npm update`; nearest real comparables are NBS National BIM Library / bimobject.com, whose
adoption unit is the complete downloadable object — exactly the stub gap above; (b) jurisdiction coupling —
these Objects carry regulatory overlays tied to specific code jurisdictions, so a real offering needs a
base-object vs. jurisdiction-overlay separation (the design mockup's Extensions pattern is the right
architecture to port here); (c) third-party content rights — 5 of 7 current Objects are Steelcase products;
granting broad redistribution rights over manufacturer-derived specification data is a question for counsel,
not a design decision.

**Two distinct products the phrase conflates** (Fable's framing, useful going forward):
- **Product A — adopt the content** (the Carbon-like move; should be the foreground): versioned,
  checksummed Object-data releases; property-set-complete IFC exports; a Revit shared-parameter/type-catalog
  bridge + COBie sheets (no BIM tool consumes DTCG directly); a license that actually permits derivatives.
- **Product B — adopt the server** (the GitLab-self-managed move; should stay background): run
  `app-privategit-bim` with your own vault. Requires packaged binary distribution, vault-format docs,
  white-label/theming separation (Woodfine trademark chrome is currently baked into the shell), and an
  AGPL-vs-commercial answer for the platform code.

## Decisions open

- **License scope for Object data** — does the 2026-07-09 CC BY-ND confirmation cover Object data, or was it
  scoped to site prose only? These are in direct conflict as currently rendered on the live footer. Needs
  explicit operator re-confirmation before any positioning work proceeds — not something to resolve by
  silent inference either way.
- Whether/when to invest in IFC export completeness (real property sets) — a real engineering task, not a
  copy fix.
- Whether third-party (Steelcase) Object data can carry a redistribution-permitting license — a counsel
  question.

## Explicitly not done this session

Per the research workflow's own recommendation: this idea was **not** folded into the concurrent JOURNAL
essay-seeding request (see `.agent/rules/cleanup-log.md` 2026-07-10 entry). The three research essays are
grounded in sourced, already-shipped or explicitly-planned facts; this positioning idea is not yet backed by
a working artifact (stub IFC exports, unresolved license conflict) and folding it into published essay
material now would be a BCSC-disclosure-posture problem — describing aspiration as shipped capability.

## Work log

2026-07-10 — Opus + Fable browser-in-the-loop research completed (workflow `wf_12931276-c2a`). This BRIEF
created to hold the findings. No code, license files, or site content changed as part of this research pass.
