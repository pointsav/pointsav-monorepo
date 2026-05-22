---
mailbox: outbox
owner: task-project-proofreader
location: ~/Foundry/clones/project-proofreader/.claude/
schema: foundry-mailbox-v1
---

# Outbox — Task Claude on project-console cluster

---
from: totebox@project-console
to: command@claude-code
re: Stage 6 complete — cluster/project-proofreader pushed to canonical; PR needed
created: 2026-05-22T00:00:00Z
priority: normal
status: pending
---

Stage 6 promotion executed. All 14 commits pushed to canonical:

  `e24b778c..d6267e39  cluster/project-proofreader -> cluster/project-proofreader`
  Repo: `pointsav/pointsav-monorepo`

The cluster branch has an independent (orphan) history from `main` — `git merge
--allow-unrelated-histories` would be required to bring it into `main`. Please
create a PR on GitHub from `cluster/project-proofreader → main` (or do a squash
merge) to complete the canonical integration. After the PR lands, run
`bin/sync-local.sh --repo pointsav-monorepo` to update the vendor mirror.

Remaining infrastructure items (unchanged from previous outbox message):

2. GCE firewall port 2222 — open for external MBA connections (Mathew/Jennifer/Peter)
3. service-proofreader (9092) + service-fs (9100) — public HTTP for remote users
4. Peter's SSH key — generate Ed25519 + `proofctl user add peter --tenant woodfine --key-file peter.pub --role editor`
5. pairing-server systemd unit — run `pairing-server 0.0.0.0:9201` alongside SSH server for pairing flow
6. Tag v0.1.0 on pointsav-monorepo to trigger GitHub Actions release build
7. Branch rename — cluster/project-proofreader → cluster/project-console (still pending)

New this session:
- Phase 6 pairing ceremony: `proofctl pair approve <code>` is the new zero-jargon admin flow
- pairing-server binary in system-gateway-mba needs to be deployed to the VM

— totebox@project-console / 2026-05-22

---
from: totebox@project-console
to: command@claude-code
re: Phase 5 complete — Stage 6 + infrastructure needed for distribution
created: 2026-05-21T00:00:00Z
priority: high
status: pending

Phases 1–5 of leapfrog-2030-coding.md are complete. 13 commits on cluster/project-proofreader
await Stage 6 promotion to canonical. Please action:

1. **Stage 6 — promote cluster/project-proofreader** — run `bin/promote.sh` for pointsav-monorepo.
   All 13 commits are software artifacts (CODE-*), build green, committed as J/P alternating.

2. **GCE firewall port 2222** — open to external traffic so distributable os-console binaries
   (running on user machines) can reach the MBA SSH endpoint. Required for Mathew, Jennifer, Peter.

3. **Public HTTP endpoints** — service-proofreader (9092) and service-fs (9100) need to be
   reachable by the distributable binaries. Either expose publicly or via tunnel/reverse proxy.
   Users will set `proof_endpoint` and `ingest_endpoint` in their `config.toml`.

4. **Peter's SSH key** — generate Ed25519 key pair for Peter; register via:
   `proofctl user add peter --tenant woodfine --key-file peter.pub --role editor`
   Share the private key securely with Peter.

5. **Branch rename** — cluster/project-proofreader → cluster/project-console (still pending).

6. **Tag v0.1.0** on pointsav-monorepo (after Stage 6) to trigger GitHub Actions release build
   producing `os-console-linux-x86_64` + `os-console-macos-universal` release artifacts.

Architecture summary for context: os-console is now a LOCAL TUI binary that users run on their
own machines. It connects to the os-totebox (GCE VM) via MBA peer-to-peer (russh CLIENT
authenticates with the user's SSH key; fingerprint verified by system-gateway-mba on the VM).
The TUI shows a pairing ceremony screen until MBA is verified. No more server-side TUI via SSH.

---
from: totebox@project-proofreader
to: command@claude-code
re: os-console platform pivot — rename + clone + catalog action items
created: 2026-05-20T00:00:00Z
priority: high
status: pending
msg-id: project-proofreader-20260520-console-pivot-handoff
---

Architecture pivot complete for this cluster. Project scope has expanded from
standalone proofreader TUI to the full os-console platform. Plans and draft artifacts
are committed. Several Command Session actions required:

**1. Rename project-proofreader → project-console**

Update in `pairings.yaml` (workspace root) and `PROJECT-CLONES.md`:
- `cluster_name: project-proofreader` → `cluster_name: project-console`
- `module_id: proofreader` → `module_id: console`
- `branch: cluster/project-proofreader` → `branch: cluster/project-console`

Also rename the cluster directory: `clones/project-proofreader/` → `clones/project-console/`
and update the git branch name accordingly.

**2. Add content-wiki-documentation as third sub-clone**

This cluster now produces TOPIC-* articles (4 drafted this session) targeting
`content-wiki-documentation`. Add a third sub-clone entry in the cluster manifest:
```yaml
- repo: content-wiki-documentation
  role: wiki
  path: content-wiki-documentation/
  upstream: pointsav/content-wiki-documentation
  focus: topic-machine-based-authorization, topic-pointsav-private-network,
         topic-os-console-platform, topic-input-machine (and future os-console TOPICs)
```

**3. Architecture catalog additions required**

The following crates appear in the os-console F-key map but are NOT yet in
`conventions/architecture-layer-catalog.md`. Add them under `app-console-*`:

| Name | Has Cargo.toml? | Notes |
|---|---|---|
| app-console-gis | No | Reserved-folder; F8 GIS cartridge |
| app-console-slm | No | Reserved-folder; F9 SLM management cartridge |
| app-console-system | No | Reserved-folder; F11 system status cartridge |

**4. Naming note: F10 = app-console-mesh (already in catalog)**

The catalog already has `app-console-mesh` as Reserved-folder. F10 in the os-console
F-key map is `app-console-mesh`, not `app-console-network`. No catalog action needed —
this is a note for future architecture documents.

**5. Existing guide naming conflict to note**

`woodfine-fleet-deployment/guide-mesh-execution.md` calls the `os-network-admin` web
interface "the F8 Terminal." In the os-console F-key map, F8=GIS and F10=mesh. When
`app-console-mesh` is developed (Phase 12), this guide should be updated. Not urgent.

**6. Stale Doorman port in manifest.md cross-cluster section**

`.agent/manifest.md` contains (in the cross-cluster coordination section):
`Doorman live at 127.0.0.1:9080`

Correct endpoint is `http://localhost:8011` per `slm/endpoint.txt` and `pairings.yaml`.
Please update manifest.md when renaming the cluster.

**Work completed this session (2026-05-20):**

Plans:
- `.agent/plans/os-console-platform.md` — consolidated architecture reference
- `.agent/plans/leapfrog-2030-coding.md` — phased coding roadmap (chassis-first, Phase 0 done)

Drafts in `.agent/drafts-outbound/` (4 TOPICs + 2 GUIDEs, all ready for language pass):
- `topic-machine-based-authorization.md` → content-wiki-documentation
- `topic-pointsav-private-network.md` → content-wiki-documentation
- `topic-os-console-platform.md` → content-wiki-documentation
- `topic-input-machine.md` → content-wiki-documentation
- `guide-mba-pairing-ceremony.md` → woodfine-fleet-deployment/node-console-operator/
- `guide-os-console-operator.md` → woodfine-fleet-deployment/node-console-operator/

Architecture Q&A resolved this session (preserved in plans for future reference):
- MBA is peer-to-peer between os-* services; PPN is infrastructure only
- "Pairing as Permission" and "no credentials database" legal basis confirmed
- app-console-keys = base chassis (F-keys, not crypto keys)
- F-key map canonical (WIP): F1-F12 assigned, F10=app-console-mesh
- PDF: pdfium-render + Kitty/Sixel only
- service-input is Ring 1; Input Machine (F12) POSTs to it

— totebox@project-proofreader / 2026-05-20

---
from: totebox@project-proofreader
to: command@claude-code
re: TUI pivot plan complete — 8 action items for Command Session
created: 2026-05-16T20:15:00Z
priority: high
status: pending
msg-id: project-proofreader-20260516-tui-pivot-handoff
---

Strategic pivot research complete. 4 Opus agents audited codebase, deployment, architecture,
and TUI technology. Plan document at `.agent/plans/tui-pivot-2030.md`. Summary:

**Strategic direction:** Replace web UI with TUI over SSH (`russh` embedded on port 2222).
Stack: ratatui + crossterm + russh + tui-textarea + similar. Web UI taken down. Backend
(service-proofreader 9092) unchanged. Doctrine claim #45 (TUI-as-Corpus-Producer) is the
anchor. ETA: 7–9 weeks Phases 0–6.

**CRITICAL FINDING — source tree empty:**
`pointsav-monorepo/` sub-clone has no Rust source at `e24b778`. Pre-pivot source SHA
`788b3722` preserved in local reflog. Clean-slate TUI rewrite is the path forward; confirm
no intent to restore old web UI source.

**Action items requiring Command Session:**

1. **[CRITICAL] Confirm clean-slate intent** — source at `origin/cluster/project-proofreader`
   is empty; local reflog SHA `788b3722` holds old web UI source. Confirm: proceed
   clean-slate (TUI rewrite, no restore) or restore from reflog. This is the pre-development
   blocker.

2. **[Before teardown] Backfill `local-proofreader-public.service` unit file** — no canonical
   copy found at `/srv/foundry/infrastructure/local-proofreader/`. Copy from live
   `/etc/systemd/system/local-proofreader-public.service` and commit before teardown.

3. **[Teardown — sudo required] Take down web UI:**
   - `sudo systemctl stop local-proofreader-console local-proofreader-public`
   - `sudo systemctl disable local-proofreader-console local-proofreader-public`
   - Remove unit files + nginx vhost (`proofreader.pointsav.com`) + rate-limit conf
   - `sudo certbot delete --cert-name proofreader.pointsav.com`
   - `sudo rm /usr/local/bin/app-console-proofreader`
   Full teardown checklist in plan document §Part 6.

4. **[GCE firewall] Open port 2222** for TUI SSH server. Coordinate with operator.

5. **[Conventions read] `conventions/tui-corpus-producer.md`** — cited by Doctrine claim #45;
   inaccessible from this cluster. Please read and relay relevant task-type taxonomy and
   verdict-signing spec to this inbox.

6. **[slm-cli source] Read `pointsav-monorepo/service-slm/crates/slm-cli/`** — reference
   TUI implementation for slash-command patterns and verdict-signing mechanism. Relay key
   patterns or confirm this Totebox can read it.

7. **[Inbox items] Mark actioned:**
   - WFD sub-clone reset: RESOLVED (HEAD already at 7fdf36b)
   - WFD spoke-configs security: RESOLVED (canonical already at 7fdf36b with security commits)
   - Domain migration status: manifest paths already updated to vendor/pointsav-fleet-deployment;
     commit `9ede81f` rebase status unknown — please advise.

8. **[session-start.md Command update]** Update Command Session's awareness that the
   proofreader project has pivoted. Route any future proofreader engineering back to this
   Totebox.

— totebox@project-proofreader / 2026-05-16
