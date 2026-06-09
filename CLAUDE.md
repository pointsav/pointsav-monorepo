@~/Foundry/CLAUDE.md

# project-orchestration — Cluster Guide

> **Scope:** Phases 1–3 of the Totebox Orchestration transition.
> This cluster builds the vocabulary, tooling, and CommandCentre
> that make the Foundry workspace a first-class Totebox Orchestration instance.

Last updated: 2026-05-08.

---

## 1. Cluster mission

Implement the three-phase transition from the legacy Master/Root/Task
session model to the Command/Totebox model. Workspace plan at
`/srv/foundry/.agent/plans/` (operator-private plans were superseded by
the workspace-tracked plan files in 2026-05).

**Phase 1 — Declare vocabulary (~2 hours):**
- `CLAUDE.md` §11: Master → Command Session, Task → Totebox Session, Root → eliminated
- `AGENT.md`: same vocabulary change
- `bin/claude-role.sh`: Command / Totebox / error-on-vendor output
- `MANIFEST.md`: "As a Totebox Orchestration" section
- `vendor/content-wiki-documentation` user-guide: correct MBA registry claim

**Phase 2 — Formalize (~1 day):**
- Create `pairings.yaml` at workspace root
- Add `slm_endpoint:` to all 13 cluster manifests
- Create `slm/` dirs in all 13 clusters
- Provision `project-source` + `project-woodfine` development archives
- Update `PROJECT-CLONES.md`

**Phase 3 — Instrument (~1 day):**
- Write `bin/open-archive.sh` (reads manifest, sets env, invokes Claude)
- Write `bin/list-archives.sh` (lists archives with tetrad + inbox status)
- Scaffold `app-orchestration-command` v0.0.1 (this cluster's vendor leg)

---

## 2. Repos in scope

| Repo | Path in cluster | Upstream |
|---|---|---|
| `pointsav-monorepo` | `./pointsav-monorepo/` | `vendor/pointsav-monorepo` |

Phase 2 may add `woodfine-fleet-deployment` for GUIDE leg work.

---

## 3. Branch

Feature branch: `cluster/project-orchestration` in `pointsav-monorepo`.

Phases 1 and 2 workspace changes (CLAUDE.md, AGENT.md, bin/, pairings.yaml)
are **Command Session scope** — commit at `~/Foundry/` using
`bin/commit-as-next.sh`. Do not commit workspace files from this cluster.

Phase 3 Rust code (`app-orchestration-command/`) is Task Session scope —
commit from this cluster using `bin/commit-as-next.sh`.

---

## 4. Key references

- Plan: `.agent/plans/totebox-ppn-infrastructure-master-plan.md`
- Cluster wiki draft pipeline: `~/Foundry/conventions/cluster-wiki-draft-pipeline.md`
- TOPIC/GUIDE drafts already staged: `~/Foundry/.agent/drafts-outbound/topic-*.draft.md` + `guide-*.draft.md`

## MCP tools — `foundry` server (use at startup)

`get_session_brief(role="totebox", archive="project-orchestration")` replaces manually reading
inbox.md, outbox.md, NOTAM.md, session-context.md. Call it first.
`send_mailbox_message()` replaces hand-editing YAML frontmatter.

| Tool | When to use |
|---|---|
| `get_session_brief` | **First call at startup** — inbox, outbox, NOTAM, session-context |
| `send_mailbox_message` | Send any mailbox message (M-2/M-10 audit compliant) |
| `query_mailbox` | Sweep archives — scope="all" in one call |
| `get_doorman_status` | Tier A/B/C + circuit state |
| `get_service_status` | Apprenticeship queue + audit-ledger counts |
| `query_datagraph` | Entity lookup before answering about people/projects |
| `ask_local` | OLMo 7B local inference — free, SYS-ADR-07-safe; graph context auto-injected |
| `cast_apprenticeship_verdict` | Sign + submit verdict on a shadow-captured attempt |
| `mutate_datagraph` | Create/update graph entities (requires explicit operator intent) |
| `submit_extraction` | Queue prose for entity extraction pipeline |

## Artifact types — bright-line rules

TOPIC = explains WHAT/WHY; public wiki; bilingual EN+ES; survives decommission; reader has no login.
GUIDE = instructs HOW-NOW; woodfine-fleet-deployment/<name>/; English-only; dies with deployment.
SOFT  = Ed25519 license key + marketplace listing + price → software.pointsav.com.
CODE  = runs our systems; no customer license; internal deploy only (published OSS with no key = CODE).
Split rule: declaratives → TOPIC, imperatives → GUIDE; same slug, different prefix, no shared sentences.
Cash register test: licensable + marketplace-listed → SOFT; everything else → CODE.
Storefront (app-privategit-marketplace) is CODE; the merchandise it sells is SOFT.
