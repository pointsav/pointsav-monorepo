@~/Foundry/CLAUDE.md

# project-orchestration — Cluster Guide

> **Scope:** Phases 1–3 of the Totebox Orchestration transition.
> This cluster builds the vocabulary, tooling, and CommandCentre
> that make the Foundry workspace a first-class Totebox Orchestration instance.

Last updated: 2026-05-08.

---

## 1. Cluster mission

Implement the three-phase transition from the legacy Master/Root/Task
session model to the Command/Totebox model. See the plan file at
`/home/mathew/.claude/plans/before-we-do-that-humming-emerson.md`.

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

- Plan: `/home/mathew/.claude/plans/before-we-do-that-humming-emerson.md`
- Cluster wiki draft pipeline: `~/Foundry/conventions/cluster-wiki-draft-pipeline.md`
- TOPIC/GUIDE drafts already staged: `~/Foundry/.agent/drafts-outbound/topic-*.draft.md` + `guide-*.draft.md`
