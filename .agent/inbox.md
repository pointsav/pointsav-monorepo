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
re: Rename complete + website update in scope + path corrections
created: 2026-05-17T21:00:00Z
priority: high
status: in-progress
msg-id: command-20260517-bim-rename-complete
note: path corrections applied last session; B5 (Rust source) deferred — next task
---

**Repo renamed** (2026-05-17): `woodfine/woodfine-design-bim` → `woodfine/woodfine-bim-library`.

Update all paths in your work:
- Sub-clone path: `/srv/foundry/clones/project-bim/woodfine-bim-library/` (was `woodfine-design-bim/`)
- Remote URL already updated by Command: `git@github.com-woodfine-administrator:woodfine/woodfine-bim-library.git`
- `customer/` path: `/srv/foundry/customer/woodfine-bim-library/` (was `woodfine-design-bim/`)
- Cleanup-log entry (2026-05-16) references `woodfine-design-bim` — update to `woodfine-bim-library` when you touch that file.

**Website update now in scope (B5):**

`bim.woodfinegroup.com` currently serves a compiled binary (v0.0.2, built 2026-05-07) with these strings to change:
- Title: "BIM Token Catalog" → "BIM Object Library"
- Hero: "Woodfine BIM Token Catalog" → "Woodfine BIM Object Library"
- Nav: "What are BIM Tokens?" → "What are BIM Objects?"
- Nav: "Browse All Tokens" → "Browse the Library"
- Nav: "About BIM Tokens" → "About BIM Objects"
- Body: sweep all "BIM token" / "BIM tokens" → "BIM Object" / "BIM Objects" in user-facing text

The Rust source for `app-orchestration-bim` is a Reserved-folder (no src/ committed anywhere).
Your job: write the Rust source at `pointsav-monorepo/app-orchestration-bim/` on the cluster branch,
commit it, build with `cargo build --release -p app-orchestration-bim` in the monorepo sub-clone,
then write "Stage 6 pending — app-orchestration-bim" to your outbox. Command will run `sudo bootstrap.sh` to install + restart.

Also reflect the two-tier access model in UI copy:
- Public: "No account required. Generic BIM objects — parking, corridors, staircases, standard finishes."
- Remove any gated/subscription/lease-attestation language.

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
