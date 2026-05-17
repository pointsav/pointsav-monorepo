---
mailbox: inbox
owner: task@project-bim
location: ~/Foundry/clones/project-bim/.agent/
schema: foundry-mailbox-v1
---

# Inbox — project-bim

---
from: command@claude-code
to: task@project-bim
re: Operator decisions locked — BIM Objects + two-tier + license fix + task brief
created: 2026-05-17T18:45:00Z
priority: high
status: pending
msg-id: command-20260517-bim-decisions-locked
---

Four operator decisions locked 2026-05-17. Execute all in next session.

## Decision 1 — Terminology: "BIM Objects" everywhere user-facing

Replace "BIM tokens" / "BIM token" with "BIM Objects" or "BIM components" in all user-facing copy.
The DTCG wire format stays internal. AEC professionals use "objects" and "components", not "tokens."

Affects:
- `woodfine-design-bim/` README.md + README.es.md
- All GUIDE drafts in `drafts-outbound/` referencing "BIM token authoring" etc.
- `topic-bim-token-what-it-is.draft.md` → retitle "BIM Objects — What They Are"
- `topic-bim-token-three-layers.draft.md` → retitle "BIM Objects — Three Composition Layers"
- `topic-bim-tokens-substrate.draft.md` → retitle "BIM Objects — Substrate"

Do NOT change internal code, variable names, or DTCG JSON file structures — user-visible copy only.

## Decision 2 — Two-tier access model (not three)

bim.woodfinegroup.com has exactly two tiers:

| Tier | Access | Content |
|---|---|---|
| Public | No login | Generic BIM objects: parking stalls, corridors, staircases, standard finishes |
| Operational | os-console only | Full archives, BCF coordination, IDS validation |

Gated tier (lease/AOR attestation login) is removed. Write the app-orchestration-bim Phase 1
architecture spec with this two-tier model. File: `.agent/plans/app-orchestration-bim-phase1.md`.

## Decision 3 — No org transfer; woodfine-design-bim stays in woodfine org

`woodfine/woodfine-design-bim` stays in the woodfine org. No rename, no transfer to pointsav.
The project-editorial P-HIGH transfer request has been marked stale by Command Session.

GIS parallel (canonical model going forward):
- software.pointsav.com = GIS engine → PointSav = BIM software (app-orchestration-bim)
- gis.woodfinegroup.com = map data → Woodfine = BIM objects (woodfine-design-bim, bim.woodfinegroup.com)

"PointSav Buildings Schema" lives only in content-wiki-documentation TOPICs. No separate site.

## Decision 4 — License fix (execute this session)

woodfine-design-bim JSON/data files (`*.dtcg.json`, schemas) → Apache 2.0.
EUPL-1.2 share-alike will scare off architects embedding objects in Revit deliverables.

Steps:
1. In woodfine-design-bim sub-clone: update `LICENSE` EUPL-1.2 → Apache 2.0
2. Add `NOTICE` file (Apache 2.0 §4(d) attribution — Woodfine Capital Projects Inc.)
3. Update `README.md` + `README.es.md` license section
4. Commit via `bin/commit-as-next.sh`
5. Push to origin (mcorp-administrator alias); include pending commit 443a231 in the same push batch

app-orchestration-bim Rust codebase: EUPL-1.2 remains correct — software, not data.

## Task scope

1. License fix commit + Stage 6 push to woodfine-design-bim origin (mcorp-administrator)
2. Terminology sweep: README + all drafts-outbound GUIDE drafts
3. Write `.agent/plans/app-orchestration-bim-phase1.md` (two-tier architecture spec)
4. Retitle the three affected TOPIC drafts; route corrected versions to project-editorial inbox

## Still operator-pending (unchanged)

DTCG accuracy errors (climate-zones, performance, materials) remain `status: operator-pending`.
Do not edit without confirmed source citations.

— command@claude-code

---
from: command@claude-code
to: task@project-bim
re: Status check — DTCG accuracy errors + mailbox lifecycle backfill
created: 2026-05-15T09:00:00Z
priority: normal
status: operator-pending
actioned_by: command@claude-code
actioned_at: 2026-05-16
note: citations are a blocking prerequisite for DTCG fixes; not started pending operator source research
---

Status check on the DTCG accuracy error message below (2026-05-13). Three items in `climate-zones.dtcg.json`, `performance.dtcg.json`, `materials.dtcg.json` are on hold pending source citations.

Please confirm current status: not started / research in progress / citations confirmed and ready to commit.

If citations are confirmed, route verified corrections to command inbox for review before committing.

New convention: `conventions/mailbox-message-lifecycle.md` (ratified 2026-05-15). Please backfill `status:` on inbox messages. The DTCG hold message should be `status: operator-pending` (citations are a blocking prerequisite, not a Totebox-only decision).

— command@claude-code

---
from: command@claude-code
to: task@project-bim
re: BIM token catalog — 3 data accuracy errors; do NOT edit until source citations confirmed
created: 2026-05-13T16:30:00Z
priority: normal
status: operator-pending
---

Three accuracy errors were identified in the BIM DTCG token catalog during the
Leapfrog 2030 session (2026-05-07). These have NOT been corrected because they
require confirmed source citations before any edit.

**Do not edit these files without resolving the citations first:**

1. `climate-zones.dtcg.json` — uses `ecoregion.arctic/temperate` keys; should be
   ASHRAE 90.1 climate zones 1-8. Also has fabricated bSDD URIs (not real references).
   Source required: ASHRAE 90.1 zone taxonomy + valid bSDD URI format.

2. `performance.dtcg.json` — `Pset_DoorCommon.FireExit` should be `IsFireExit`.
   Source: IFC4 Pset_DoorCommon property set definition.

3. `materials.dtcg.json` — `ThermalTransmittance` is an assembly-level property,
   not material-level. Source: IFC/ISO 10077 distinction between material and
   assembly thermal properties.

**Your action:** research each error with confirmed source citations, then fix in a
single commit with citations in frontmatter. Do not fix without sources — accuracy-
sensitive; these feed regulatory overlays at bim.woodfinegroup.com.

— command@claude-code
