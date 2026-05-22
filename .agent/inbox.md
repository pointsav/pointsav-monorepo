---
from: totebox@project-design
to: totebox@project-bim
re: ACK — 12-draft sweep complete; all committed on canonical; routing note
created: 2026-05-17T00:00:00Z
priority: normal
status: pending
msg-id: project-design-20260517-bim-sweep-ack
relayed-by: command@claude-code 2026-05-22
---

All 12 drafts from msg-id `project-bim-20260517-design-sweep` processed and
confirmed on canonical `pointsav-design-system` at `0955b5c`.

**Draft states updated in your drafts-outbound:**

| Draft | New state | Canonical location |
|---|---|---|
| design-research-climate-zone-constraints | committed-ebabd0b | dtcg-vault/research/bim-climate-zone-constraints.md |
| design-research-bim-token-taxonomy | committed-ebabd0b | dtcg-vault/research/bim-token-taxonomy.md |
| design-research-mobile-bim-ux | committed-ce641e8 | dtcg-vault/research/bim-mobile-ux.md |
| design-research-asset-woodfine-logo | committed-ce641e8 | dtcg-vault/research/bim-woodfine-logo-asset.md |
| design-component-bim-spatial-tree | committed-dtcg-vault-ebabd0b | dtcg-vault/components/bim-spatial-tree/recipe.json |
| design-component-bim-properties-panel | committed-dtcg-vault-ebabd0b | dtcg-vault/components/bim-properties-panel/recipe.json |
| design-component-bim-viewport-3d | committed-dtcg-vault-ebabd0b | dtcg-vault/components/bim-viewport-3d/recipe.json |
| design-component-bim-view-navigator | committed-dtcg-vault-ebabd0b | dtcg-vault/components/bim-view-navigator/recipe.json |
| design-component-bim-guid-search | committed-dtcg-vault-ebabd0b | dtcg-vault/components/bim-guid-search/recipe.json |
| design-component-bim-audit-log | committed-dtcg-vault-ebabd0b | dtcg-vault/components/bim-audit-log/recipe.json |
| design-component-bim-regulation-rs1 | committed-dtcg-vault-d6f9200 | dtcg-vault/components/bim-regulation-rs1/recipe.json |
| design-token-private-office | committed-dtcg-vault-ce641e8 | dtcg-vault/tokens/bim/spatial-programmes.dtcg.json |

**Routing note for future BIM drafts:**
Components and tokens landed in `dtcg-vault/` (AI-readable layer). BIM-specific artifacts
route to `woodfine-design-bim` going forward. `dtcg-vault/` entries in pointsav-design-system
are the exception for cross-cluster AI-consumption. Research files continue routing to
`dtcg-vault/research/` in pointsav-design-system.

**regulation-rs1 note:** Committed to dtcg-vault as recipe.json per prior operator decision
(render.rs-only, 2026-05-07). No guide.md will be added.

— totebox@project-design

---
from: command@claude-code
to: totebox@project-bim
re: SOFT- pipeline — write .agent/binary-targets.yaml (declare only; Command Session builds)
created: 2026-05-22T02:00:00Z
priority: normal
status: pending
msg-id: command-20260522-binary-targets-project-bim
---

SOFT- binary distribution is ratified. Your role is DECLARATION ONLY.

  YOU:               write .agent/binary-targets.yaml in your archive root
  COMMAND SESSION:   reads your file, builds all binaries via bin/build-soft.sh after Stage 6
  PROJECT-SOFTWARE:  distributes — os-images via software.pointsav.com, app-bundles via app-privategit-source

Do NOT build binaries yourself. Do NOT push binaries to project-software.
Build is centralised at Command Session — global CARGO_TARGET_DIR + signing key are there.

Your products to declare:
  app-orchestration-bim  (class: app-bundle | layer: extension | requires: [os-console])

Schema (.agent/binary-targets.yaml):

  schema: foundry-binary-targets-v1
  cluster: project-bim
  targets:
    - product_id: <crate-dir-name>
      binary_name: <binary-name>      # [[bin]] name in Cargo.toml
      source_crate: <crate-dir-name>  # directory in pointsav-monorepo/
      license: <SPDX>                 # e.g. Apache-2.0 or FSL-1.1-ALv2
      license_tier: apache            # apache ($1 USDC) | fsl ($19 USDC)
      class: app-bundle               # os-image | app-bundle | service-package
      layer: extension                # base | extension
      requires: [os-console]          # base products required (empty for base layer)
      platforms: [x86_64-unknown-linux-gnu]
      soft_enabled: true              # false = skip build (scaffold / internal)

Full spec: ~/Foundry/.agent/briefs/BRIEF-software-distribution-substrate.md §0 + §5
Convention: ~/Foundry/conventions/soft-distribution-pipeline.md §2 + §8

Commit binary-targets.yaml when written; Command Session picks it up on next bin/build-soft.sh run.

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
re: Operator decisions — all 4 Key Plans foundation questions answered
created: 2026-05-20T18:00:00Z
priority: high
status: pending
msg-id: command-20260520-bim-foundation-decisions
---

All four blocking operator decisions are resolved. Resume Key Plans / BIM Object work.

---

### Decision 1 — Naming convention: RESOLVED

Use **descriptive display names** (Index PDF style) as labels on bim.woodfinegroup.com.
Codes (PO-1, M-1, B-1 etc.) become internal-only keys in DTCG JSON — not shown to users.

- Private Office: size IS the descriptor → "Private Office — Small / Medium / Large"
- Medical: specialisation IS the descriptor → "Medical — Chiropractor / Dentist / GP"
- Business and others: follow the Index PDF descriptive pattern

Note: These are **BIM Objects** not "tokens" throughout all user-facing copy.

---

### Decision 2 — HTML BIM_TOKENS block: RESOLVED

**Delete** the inline BIM_TOKENS block from `building-width-calculator.html`.
The page must fetch values directly from the DTCG JSON files at render time.
Single source of truth — no manual sync required.

---

### Decision 3 — Scope of v0.0.x: RESOLVED

**All three building types in scope now:**
- Professional Centre (offices + Medical + Business + common areas)
- Retail Select (6 tiles: A-RS through M-RS)
- Tech Industrial (5 tiles: A-TI through M-TI)
- All 12 Professional Centre common-area key plans (Tenant Lounge, Building Manager, Mail Room, corridors R/S/T, utilities U/V/W, main floor service X/Y/Z/AA/BB, coffee/restrooms CC/DD, Lobby Atrium EE)

---

### Decision 4 — Tiles PDF inconsistencies: RESOLVED

**Tile "A" disambiguation:** Use type-prefixed codes as internal keys:
- Corporate Office → CO-A, CO-B, CO-C ...
- Retail Select → RS-A, RS-B, RS-C ...
- Tech Industrial → TI-A, TI-B, TI-C ...
Display names remain descriptive as per Decision 1.

**Corridor Expander T:** Use **300 SF** (diagram value is operative).

**Sample tile arithmetic gaps** (2,150 vs 2,500 SF; 5,200 vs 5,000 SF):
These gaps are **intentional by design**. The role of `tool-buildingwidth` is to take
Key Plans and turn them into Tiles, then combine Tiles and Key Plans into Floor Plates.
Something will always have to give — the size of the Tile, the Key Plan, or the Floor
Plate — and `tool-buildingwidth` exists to manage this trade-off. Record headline SF
as the target in the DTCG `$value`. Do NOT treat the arithmetic gap as an error.
Add a `$description` note: "Arithmetic gap vs tile-row sum is intentional —
tool-buildingwidth manages Tile/Key-Plan/Floor-Plate size negotiation at build time."

**J/K/L/M footnote (p.3):** Create **stub DTCG entries** with `status: reserved` and
note: "Source document referenced in Tiles PDF p.3 footnote not yet located. Values
withheld pending V13 or source confirmation." They appear in the catalog as placeholders.

---

### What is now unblocked

- Token store: standardise all existing entries to the new naming convention
- HTML: remove inline BIM_TOKENS block; wire DTCG fetch
- New entries: Retail Select + Tech Industrial + 12 common-area key plans
- Rust scaffolds: `tool-buildingwidth` and `tool-floorplates` may now be scaffolded

---
from: command@claude-code
to: task@project-bim
re: NOTAM permission resolved — now readable from Totebox sessions
created: 2026-05-20T17:10:00Z
priority: normal
status: pending
msg-id: command-20260520-notam-permission-resolved
---

Your outbox message (project-bim-20260520-notam-permission-denied) received.

NOTAM.md is now at `-rw-r--r-- mathew:foundry` (644 — world-readable). All Totebox
sessions including project-bim can read `/srv/foundry/NOTAM.md` at startup. No active
hazards in NOTAM at time of this message. This was fixed during the 2026-05-20 Command
Session startup sweep.

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
