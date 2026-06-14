@~/Foundry/AGENT.md

# project-gis — Archive Guide

> **State:** active | **Last updated:** 2026-06-14
> **Cluster manifest:** `.agent/manifest.md`
> **Workspace AGENT.md takes precedence on conflict.**

---

## Cluster mission

Location Intelligence GIS cluster. Builds and deploys:
- Co-location archetype system: **PRO** (retail, 6,493 clusters), **VWH** (vertical
  warehouse / urban fringe, 6,368 clusters), **PKS** (parking structures / commuter,
  6,953 clusters)
- Top 400 Regional Markets dataset (NA + EU)
- AEC enrichment layers: Köppen, ecoregion, GHI, seismic, flood, wildfire
- Journal research programme: J1 Retail Co-location (v0.5), J7 Urban Fringe, J8 Commuter

Primary surface: `gis.woodfinegroup.com` (gateway-orchestration-gis-1).
All pipeline scripts: `pointsav-monorepo/app-orchestration-gis/`.

## Tetrad

See `.agent/manifest.md` `tetrad:` block.

- **vendor:** `pointsav-monorepo` → `app-orchestration-gis` (Python pipeline)
- **customer:** `woodfine-fleet-deployment/gateway-orchestration-gis-1/`
- **deployment:** `gateway-orchestration-gis-1` on `vault-privategit-source-1`
- **wiki:** `content-wiki-projects` (TOPIC-* artifacts)

## At session start

Per `~/Foundry/AGENT.md` §startup:

1. Confirm role: `~/Foundry/bin/foundry-role.sh` (Totebox Session expected)
2. Write session lock: `.agent/engines/<engine-id>/session.lock`
3. Read `.agent/manifest.md` — cluster mission + tetrad
4. Call `get_session_brief(role="totebox", archive="project-gis")` — replaces
   inbox, NOTAM, session-context reads
5. Read `~/Foundry/NOTAM.md` only if `notam_active: true` returned from step 4
6. Read `.agent/rules/*.md` if present

## Nightly pipeline schedule (system timezone: America/Vancouver)

| Time | Script | Log |
|------|--------|-----|
| 22:00 PDT daily | `nightly-rebuild.sh` | `nightly-rebuild.log` |
| 23:00 PDT Monday | `build-aec-global.sh` | `aec-global.log` |
| 23:00 PDT Tuesday | `build-aec-seismic.sh` | `aec-seismic.log` |
| 23:00 PDT Wednesday | `build-aec-flood.sh` | `aec-flood.log` |

nightly-rebuild.sh runs: build-clusters.py → build-tiles.py --layer 2 →
build-vwh-clusters.py → build-pks-clusters.py. AEC scripts patch
`clusters-meta.json` via coordinate-based merge (≈300m tolerance).

## Hard rules

`~/Foundry/AGENT.md` §Hard rules — identity store immutable, never chmod;
preview before writing; edit in place (no _V2 files); one session per repo;
Bloomberg standard; BCSC posture; SYS-ADR-07/10/19.
`~/Foundry/CLAUDE.md` §Size discipline — per-archive CLAUDE.md ≤ 150 lines.

## Commit + promote

Commits via `~/Foundry/bin/commit-as-next.sh "<message>"`.
Stage 6 promotion via `~/Foundry/bin/promote.sh` from Command Session.

## Conflicts

Surface conflicts via outbox to Command Session — do not silently override.

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-gis")` replaces manually
reading inbox.md, outbox.md, NOTAM.md, session-context.md. Call it first.
`send_mailbox_message()` replaces hand-editing YAML frontmatter.

| Tool | When to use |
|---|---|
| `get_session_brief` | **First call at startup** |
| `send_mailbox_message` | Send any mailbox message |
| `query_mailbox` | Sweep archives — scope="all" |
| `get_doorman_status` | Tier A/B/C + circuit state |
| `query_datagraph` | Entity lookup before answering about people/projects |
| `ask_local` | OLMo 7B local inference — free, SYS-ADR-07-safe |

## Artifact types — bright-line rules

TOPIC = explains WHAT/WHY; bilingual EN+ES; survives decommission.
GUIDE = instructs HOW-NOW; woodfine-fleet-deployment/; English-only.
JOURNAL = academic paper draft; `foundry-journal-v1` schema; named authors only.
DATA = pipeline output; commit directly; no drafts-outbound needed.
