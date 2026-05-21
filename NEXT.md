# NEXT — project-orchestration

> Implementation scope: Totebox Orchestration transition Phases 1–3.
> Full plan: `.agent/plans/totebox-ppn-infrastructure-master-plan.md`
> Opened: 2026-05-08

---

## Phase 1 — Declare vocabulary (COMMAND SESSION SCOPE)

These edits happen in `~/Foundry/`, not this cluster.

- [x] **P1.1** `CLAUDE.md` §11: Master → Command Session, Task → Totebox Session, Root → eliminated
- [x] **P1.2** `AGENT.md` session roles table: same vocabulary change
- [x] **P1.3** `bin/claude-role.sh`: Command / Totebox / error-on-vendor output
- [x] **P1.4** `MANIFEST.md`: add "As a Totebox Orchestration" section `[closed: 2026-05-09 command@claude-code]`
- [x] **P1.5** Correct `systems/os-orchestration.md` user-guide article `[closed: 2026-05-12 totebox@project-editorial]`

---

## Phase 2 — Formalize manifests + SLM wiring + pairings.yaml

These edits happen in `~/Foundry/` (COMMAND scope) and this cluster (Totebox scope).

- [x] **P2.1** Update `foundry-cluster-manifest-v1` schema docs with `slm_endpoint:` field `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.2** Add `slm_endpoint: http://localhost:8011` to all 13 cluster `.agent/manifest.md` files `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.3** Create `slm/` dir in each of 13 clusters `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.3b** Create `pairings.yaml` at workspace root `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.4** Provision `clones/project-source/` `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.5** Provision `clones/project-woodfine/` `[closed: 2026-05-14 command@claude-code]`
- [x] **P2.6** Update `PROJECT-CLONES.md`: use "Totebox Archive" language, add SLM column (15 archives) `[closed: 2026-05-14 command@claude-code]`

---

## Phase 3 — Instrument tooling (TOTEBOX SESSION SCOPE — use this cluster)

Write code in `pointsav-monorepo/` on branch `cluster/project-orchestration`.

### P3.1 — bin/open-archive.sh `[closed: 2026-05-14 command@claude-code]`

Shell script at `~/Foundry/bin/open-archive.sh <archive-name>`:

```
1. Validate archive exists in clones/
2. Read clones/<archive>/.agent/manifest.md:
   - Print archive name, tetrad status (all 4 legs + status)
   - Print slm_endpoint + module_id
   - Count pending inbox messages (non-blank lines after header)
3. Check contributor tier from pairings.yaml (basic: warn if not P1 opening Command CWD)
4. Set env vars: FOUNDRY_ARCHIVE=<archive>, FOUNDRY_MODULE_ID=<module_id>
5. Exec: claude --cwd ~/Foundry/clones/<archive>/
```

### P3.2 — bin/list-archives.sh `[closed: 2026-05-14 command@claude-code]`

Shell script at `~/Foundry/bin/list-archives.sh`:

```
1. Walk clones/*/. agent/manifest.md
2. For each manifest: print cluster_name, tetrad leg statuses, inbox count
3. Columnar output, easy to scan
4. Source: PROJECT-CLONES.md or manifest files directly
```

### P3.3 — app-orchestration-command v0.0.1 (Rust)

Scaffold in `pointsav-monorepo/app-orchestration-command/`:

Endpoints (HTTP, loopback only, port 8020):
- `GET /archives` — return JSON list of all archives with tetrad status + inbox count
  Source: walk clones/*/. agent/manifest.md
- `POST /message` — route a cross-archive message
  MUST validate per-caller scope first (confused deputy defense):
  check requesting archive's module_id against pairings.yaml permissions
  Log all routing decisions to audit ledger
- `GET /personnel/<unix-user>` — return permission tier + pairing set
  Source: pairings.yaml + PersonnelArchive DataGraph (MVP: just pairings.yaml)

Implementation pattern: follow `app-orchestration-gis` structure (same codebase).
Commit on `cluster/project-orchestration` branch in this cluster's pointsav-monorepo.

### P3.4 — Deploy to deployments/orchestration-command-1/

After v0.0.1 compiles:
- Create `~/Foundry/deployments/orchestration-command-1/` with MANIFEST.md
- Copy binary, write systemd unit to `infrastructure/`
- Start service: `sudo systemctl start app-orchestration-command`
- Test: `curl http://localhost:8020/archives`

### P3.5 — Update NEXT.md

Mark Phase 1 + 2 complete; update workspace NEXT.md with Phase 3 task link.

---

## Two-VM transition (parallel track — COMMAND scope)

See plan file §"Two-VM transition" for full detail.

- [ ] **T1** WireGuard Part A: VPN peer for staging at :9200
- [ ] **T2** Provision os-mediakit node (new GCP VM)
- [ ] **T3** Transfer: rsync chain via Jennifer's Mac
- [ ] **T4** DNS cutover for 9 domains
- [ ] **T5** Remove public vhosts from os-orchestration node; update MANIFEST.md

---

## Content backlog (project-editorial scope)

- [ ] Route TOPIC/GUIDE batch: 7 drafts in `~/Foundry/.agent/drafts-outbound/` → DONE 2026-05-08
- [ ] Write `conventions/trustworthy-system.md` (COMMAND scope)
- [ ] Update user-guide article (P1.5 above)

---

## Key references

- Plan file: `.agent/plans/totebox-ppn-infrastructure-master-plan.md`
- Cluster manifest: `.agent/manifest.md`
- app-orchestration-gis reference impl: `clones/project-gis/pointsav-monorepo/app-orchestration-gis/`
