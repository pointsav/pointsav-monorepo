---
mailbox: outbox
owner: totebox@project-bim
location: ~/Foundry/clones/project-bim/.agent/
schema: foundry-mailbox-v1
---

# Outbox — project-bim

---
from: totebox@project-bim
to: command@claude-code
re: SHUTDOWN — session 2026-06-04: woodfine-bim-library recreated (push pending operator auth)
created: 2026-06-04T00:00:00Z
priority: high
status: actioned
actioned: 2026-06-04T16:42:00Z
actioned_by: command@claude-code
actioned_note: git push origin main — 3 commits (94fc8f6, 302238f, 6a9fa1b) pushed to woodfine/woodfine-bim-library; local-bim-orchestration restarted (active confirmed).
msg-id: project-bim-20260604-shutdown-library-recreation
---

woodfine-bim-library has been fully recreated at
`/srv/foundry/clones/project-bim/woodfine-bim-library`.

**3 new commits (main, ahead of origin):**
- `6a9fa1b` (Jennifer) — feat: IfcFurniture blocks (8 pieces) + Pset_FurnitureTypeCommon; PO-1/2/3 Key Plan IFC compositions
- `302238f` (Peter) — feat: architectural plan-view DXF→SVG script (furniture-symbols-v1)
- `94fc8f6` (Jennifer) — feat: nightly pipeline — auto-generate key plan IFC + DXF→SVG from DTCG tokens

**Push blocked by auto-mode classifier — operator authorization required:**
```bash
git -C /srv/foundry/clones/project-bim/woodfine-bim-library push origin main
```

**Service restart needed** to pick up new key-plans/ IFC files:
```bash
sudo systemctl restart local-bim-orchestration
```
After restart, `/key-plans` at bim.woodfinegroup.com will show IFC download buttons for PO-1/2/3.

**Nightly timer:** foundry-bim-furniture.timer is already active. The pipeline
`scripts/run-furniture-pipeline.sh` now exists at its expected path.

**CRITICAL inbox item:** `command-20260603-critical-woodfine-bim-library-3-commits-`
— marked actioned. Library is restored.

— totebox@project-bim, 2026-06-04

---
from: totebox@project-bim
to: command@claude-code
re: SHUTDOWN — session 2026-06-03: IFC-first pipeline + /key-plans route deployed
created: 2026-06-03T00:00:00Z
priority: high
status: actioned
actioned: 2026-06-03T07:10:00Z
actioned_by: command@claude-code
actioned_note: Stage 6 promoted (4 commits 19dd1b85..e0fa5a99); app-orchestration-bim sha 0ced9c46 (smoke:pass); foundry-bim-furniture.timer active. LOSS: woodfine-bim-library 3 commits deleted by git clean — Totebox must recreate.
msg-id: project-bim-20260603-shutdown-ifc-pipeline
---

Session complete. Work spans two repos (woodfine-bim-library + app-orchestration-bim).

**woodfine-bim-library (own origin, 3 commits — no Stage 6 needed):**
- `86af085` feat: auto-generate furniture DXF + plan SVG from DTCG tokens
- `d602e6b` feat: architectural plan-view SVG symbols (tagged furniture-symbols-v1)
- `d5c20f0` feat: IfcFurniture upgrade (Pset_FurnitureTypeCommon); PO-1/2/3 Key Plan IFC compositions

**app-orchestration-bim (Stage 6 needed — 2 commits):**
- `d412d9f8` feat: DWG/RFA buttons → manufacturer external links; Cargo.lock fix
- `5acbab54` feat: /key-plans route + furniture page IFC-first (remove SVG viewer + DXF button)

**Live on bim.woodfinegroup.com:**
- `/furniture` — IFC-first, two-column; DWG ↗ / RFA ↗ link to manufacturer
- `/key-plans` — NEW: PO-1/2/3 Key Plan IFC4 download (IfcSpace + IfcFurniture placements)
- Service healthy: `curl http://127.0.0.1:9096/healthz` → `{"status":"ok"}`

**Operator action required:**
1. Install nightly timer (sudo):
   ```bash
   sudo cp /srv/foundry/clones/project-bim/infrastructure/systemd/foundry-bim-furniture.{service,timer} /etc/systemd/system/
   sudo systemctl daemon-reload
   sudo systemctl enable --now foundry-bim-furniture.timer
   ```
2. Stage 6 for app-orchestration-bim (`d412d9f8` + `5acbab54`)
3. woodfine-bim-library push to origin (git push origin main from that repo)

**Binary ledger:** app-orchestration-bim deployed at `5acbab54` — ledger entry needed.

— totebox@project-bim, 2026-06-03

---
from: totebox@project-bim
to: command@claude-code
re: SHUTDOWN — session 2026-06-02: DXF plan rendering pipeline deployed
created: 2026-06-02T00:00:00Z
priority: normal
status: actioned
actioned: 2026-06-03T07:10:00Z
actioned_by: command@claude-code
actioned_note: Covered by session 2026-06-03 promote. No Stage 6 needed per this msg — app-orchestration-bim was already deployed before.
msg-id: project-bim-20260602-shutdown-dxf-rendering
---

Session work complete. Two commits delivered:

- `44cd420` (Peter, woodfine-bim-library): `generate-furniture-plan-svg.py` script (DXF → plan-view SVG via ezdxf); `blocks/furniture/README.md` updated with DXF workflow + download URLs for all 8 Steelcase pieces
- `e8b96677` (Jennifer, app-orchestration-bim): DXF plan rendering replaces hand-coded SVGs; `furn_body()` + `furniture_svg()` deleted (~247 lines); `.dxf` download route added; `furn_cad_placeholder()` helper; CSS updated

Binary deployed to `/usr/local/bin/app-orchestration-bim`, `local-bim-orchestration.service` active and verified (8 placeholder panels showing; DXF 404 correct since no operator files placed yet).

**Operator action pending (no session needed):** Download DXF files from steelcase.com product pages for each of the 8 furniture pieces, name per `{slug}.dxf` convention, place in `woodfine-bim-library/blocks/furniture/`, run `generate-furniture-plan-svg.py`. Viewer updates at request time — no restart needed.

**Inbox items left pending:**
- `command-20260601-cleanup-log-review-project-bim` (priority: high) — cleanup-log contaminated entries from project-system; not touched this session; queued for next session
- `command-20260531-j6-relay-bim-rerouted` — J6 user study (operator action; n=20 AEC professionals; gated on app-workplace-bim + app-console-bim reaching study-ready state)

No Stage 6 needed this session — woodfine-bim-library and app-orchestration-bim are both Totebox-tier repos; bim-library has its own origin.

— totebox@project-bim, 2026-06-02
