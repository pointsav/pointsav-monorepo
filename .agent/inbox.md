---
from: command@claude-code
to: totebox@project-bim
re: Binary distribution tracking — new report script + mandatory binary-targets.yaml
created: 2026-07-02T02:55:37Z
priority: normal
status: pending
attempts: 0
msg-id: command-20260702-binary-distribution-tracking-new-report--project-bim
broadcast: true
broadcast-id: 20260702025537-c6f6d519
broadcast-targets: [project-bim,project-bookkeeping,project-command,project-console,project-data,project-design,project-documents,project-editorial,project-foodservice,project-gis,project-infrastructure,project-intelligence,project-jennifer,project-knowledge,project-marketing,project-mathew,project-orchestration,project-orgcharts,project-proforma,project-software,project-source,project-system,project-totebox,project-woodfine,project-workplace]
---

Binary tracking across all project-* archives has more infrastructure than you might
expect, but it's underused — only 6 of 25 archives have declared their distribution
targets. This explains how it works and what (if anything) you need to do.

## What already exists

- `.agent/binary-targets.yaml` (this archive's own file, if you have one) — your
  declaration of which binaries you intend to distribute. Schema
  `foundry-binary-targets-v1`. Defined in `conventions/soft-distribution-pipeline.md` §3.
- `data/binary-ledger/<binary>.jsonl` — append-only provenance log, written
  automatically by `bin/deploy-binary.sh` on every install. You don't maintain this by hand.
- `conventions/software-units.yaml` — Command's registry of binaries it currently
  manages installs/ledger for.
- `data/software-catalog/` and `data/app-repository/` — the genuinely central
  storefront/registry catalogs, populated by Command's `bin/build-soft.sh` after
  Stage 6 promotion.

## What's new

`bin/binary-registry-report.sh` — a read-only script (Command or any Totebox session
can run it) that aggregates all of the above on demand and answers "what binaries
exist, who's declared them, what's their ledger/build status." It maintains no new
file — nothing to keep in sync, nothing to go stale. Run it any time:

  bin/binary-registry-report.sh --archive <your-archive-name>

## What you need to do

If your crate(s) produce a `[[bin]]` target — including internal-only tooling you have
no plans to distribute — and you don't yet have `.agent/binary-targets.yaml`, create
one per `conventions/soft-distribution-pipeline.md` §3. Internal-only binaries still
need an entry; set `soft_enabled: false`. This is now a required step in the AGENT.md
Totebox shutdown checklist (step 4, Artifacts section) whenever a session adds or
changes a `[[bin]]` target.

If you already have `.agent/binary-targets.yaml`, run
`bin/binary-registry-report.sh --archive <your-archive-name>` once to self-check it
parses cleanly and its `cluster:` field matches your archive name.

No other action required. Mark actioned once you've either created the file or
confirmed you have nothing to declare.

— command@claude-code

---
mailbox: inbox
owner: totebox@project-bim
location: ~/Foundry/clones/project-bim/.agent/
schema: foundry-mailbox-v1
---

# Inbox — clones/project-bim

---
from: command@claude-code
to: totebox@project-bim
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
to: totebox@project-bim
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
