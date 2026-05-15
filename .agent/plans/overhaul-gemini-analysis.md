---
schema: foundry-analysis-v1
slug: overhaul-gemini-analysis
status: partial
authored_by: claude-code
phase: 1-streamlined
created: 2026-05-15
note: §2 and §9 complete. §1/§3–§8 are deferred stubs — populated during Phase 2 sub-phases as listed.
---

# Overhaul Analysis — documentation.pointsav.com

> Streamlined Phase 1 analysis authored by Claude Code (Gemini CLI not available).
> §2 (ZIP-to-TOPIC mapping) and §9 (domain map) are complete and sufficient to
> begin Phase 2 sub-phases 2a and 2c. Remaining sections are stubs — each
> specifies which Phase 2 sub-phase populates it.

---

## §1 — Catalogue proposal

[DEFERRED — populated during Phase 2 sub-phase 2c (catalogue actions). Claude reads
each TOPIC file in full during 2c and flags retire/consolidate/split/move actions at
that point. Known pre-committed actions from the master plan (§5.1–§5.4) are already
queued as Phase 1 light-work commits and do not require this section.]

---

## §2 — ZIP-to-TOPIC mapping

All 27 ZIP drafts evaluated. Disposition: `new:<slug>,<category>` places the content
as a new TOPIC; `merge-into:<slug>` integrates ZIP prose into an existing article.
BIM-flagged content follows §4 routing: summary + cross-reference only in documentation wiki.

| zip slug | disposition | target | notes |
|---|---|---|---|
| zip-topic-app-console-input-f12 | `new:app-console-input,applications` | — | SYS-ADR-10 / F12 surface; no existing article |
| zip-topic-archetypes-and-coa | `new:archetypes-and-chart-of-accounts,services` | — | Taxonomy substrate; distinct from existing service articles |
| zip-topic-bim-product-family | `new:bim-and-real-property-surfaces,applications` | — | §4: summarise + cross-reference only; full BIM content routes to woodfine-design-bim |
| zip-topic-competitive-positioning | `new:structural-positioning,reference` | — | Positioning philosophy; no existing article |
| zip-topic-compliance-disclosure | `new:compliance-and-continuous-disclosure,governance` | — | Continuous-disclosure posture; no existing article |
| zip-topic-deployment-patterns | `new:deployment-patterns,patterns` | — | Five-primitive universal companion pattern |
| zip-topic-design-system | `merge-into:design-typography` | `design-system/design-typography.md` | Brand + typography content fits existing design-typography article |
| zip-topic-hardware-research | `new:hardware-reference,reference` | — | Reference hardware profiles for os-workplace + os-console |
| zip-topic-leapfrog-2030 | `merge-into:leapfrog-2030-architecture` | `architecture/leapfrog-2030-architecture.md` | Thesis content supplements existing leapfrog architecture article |
| zip-topic-legal-ip-structure | `new:legal-and-ip-structure,governance` | — | Three-corp topology + squash-and-merge + air-gap |
| zip-topic-machine-based-authorization | `merge-into:machine-based-auth` | `architecture/machine-based-auth.md` | Pairing-as-permission concept; existing article covers same subject |
| zip-topic-microkernel-substrate | `new:sel4-microkernel-substrate,substrate` | — | seL4 formal-verification substrate; no existing article |
| zip-topic-os-console | `merge-into:console-os` | `systems/console-os.md` (post-rename) | os-console overview; existing article is the canonical home |
| zip-topic-os-family-overview | `new:os-family-overview,systems` | — | Eight-OS family overview; separate from per-OS articles |
| zip-topic-os-infrastructure-network | `merge-into:infrastructure-os` | `systems/infrastructure-os.md` (post-rename) | os-infrastructure narrative; os-network-admin stays separate |
| zip-topic-os-mediakit | `merge-into:mediakit-os` | `systems/mediakit-os.md` (post-rename) | Compliance appliance overview; direct merge |
| zip-topic-os-orchestration | `merge-into:os-orchestration` | `systems/os-orchestration.md` | Fleet aggregator overview; existing article |
| zip-topic-os-totebox | `merge-into:totebox-os` | `systems/totebox-os.md` (post-rename) | Vault + service host overview; direct merge |
| zip-topic-os-workplace | `merge-into:os-workplace` | `systems/os-workplace.md` | Sovereign desktop overview; existing article |
| zip-topic-pointsav-overview | `new:pointsav-overview,architecture` | — | Company + three-org overview; no existing overview article |
| zip-topic-service-content | `merge-into:service-content` | `services/service-content.md` | Gravity Engine overview; direct merge into existing |
| zip-topic-service-email-people | `merge-into:service-email` | `services/service-email.md` | Ingest + identity; email article is primary; people cross-linked |
| zip-topic-service-slm | `merge-into:service-slm` | `services/service-slm.md` | SLM overview; direct merge into existing |
| zip-topic-six-tier-sovereignty-matrix | `new:six-tier-sovereignty-matrix,architecture` | — | Monorepo tier taxonomy; no existing article |
| zip-topic-supply-chain-governance | `new:five-stage-supply-chain,governance` | — | Five-stage supply chain; no existing article |
| zip-topic-the-diode-standard | `new:diode-standard,architecture` | — | Unidirectional command flow; no existing article |
| zip-topic-three-layer-architecture | `new:three-layer-architecture,architecture` | — | Software/Showcase/Instances three-layer model |

**New articles:** 15  **Merges into existing:** 12

---

## §3 — Navigation audit

[DEFERRED — populated during Phase 2 sub-phase 2i (main page and category landing
rewrite). Claude reads each _index.md and root index.md in full at that sub-phase.]

---

## §4 — Cross-reference gap list

[DEFERRED — populated during Phase 2 sub-phase 2e (cross-reference gap fill).
The broken-link baseline at `audit/baseline-broken-links.tsv` covers wikilink gaps.
Bare-concept reference gaps are identified per-TOPIC during sub-phase 2g.]

---

## §5 — Reference proposals

[DEFERRED — populated during Phase 2 sub-phase 2d (reference additions). Claude
checks each TOPIC for external reference count (< 2 = flagged) during 2g and adds
suggestions inline.]

---

## §6 — reference/ sub-routing proposal

[DEFERRED — populated during Phase 2 sub-phase 2b (reference/ sub-routing). The 55
reference/ articles are reviewed and sub-bucket assignments made at that sub-phase.]

---

## §7 — Readability audit

[DEFERRED — populated during Phase 2 sub-phase 2g (readability pass). Claude audits
each TOPIC for lede defects, label-only headers, passive-voice density, and GUIDE
structure gaps as it processes each article. Findings are logged here at that point.]

---

## §8 — Article length audit

[DEFERRED — populated during Phase 2 sub-phase 2g. Split candidates (> 2,500 words)
and stub candidates (< 400 words) flagged per-TOPIC during the readability pass.]

---

### §9 — Domain map

Full TOPIC + GUIDE domain map committed as `domain-map.tsv` alongside this file.
296 rows: 223 TOPICs across 10 categories + 73 GUIDEs across both fleet repos.

Domain assignments:

| Domain | TOPIC count |
|---|---|
| platform-architecture | 37 |
| platform-substrate | 36 |
| reference | 55 |
| design-system | 35 |
| platform-services | 19 |
| governance | 20 |
| operating-systems | 11 |
| infrastructure | 10 |
| platform-patterns | 10 |
| applications | 7 (pre-overhaul, before duplicate removal) |

Under-served domains (< 3 articles after overhaul): none expected — all domains
will gain articles from the 15 new ZIP-draft TOPICs in sub-phase 2a.

GUIDE breakdown: 73 GUIDEs (Woodfine fleet-deployment + PointSav fleet-deployment).
Primary domain `operational-guide` for all GUIDEs; secondary domain assigned per-cluster
during Phase 2 sub-phase 2f if needed.
