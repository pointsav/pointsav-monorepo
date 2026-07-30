---
artifact: brief
status: active
topic: bim.woodfinegroup.com IFC pipeline and website state
archive: project-bim
created: 2026-06-03
owner: totebox@project-bim
---

# BRIEF — BIM Website Pipeline & Current State

## Mission

bim.woodfinegroup.com is a BIM Object library serving IFC4 files. It is
NOT a graphics site. Primary deliverables are IfcFurniture blocks and
IfcSpace Key Plan compositions for use in Bonsai / FreeCAD BIM / IfcOpenShell.

---

## Live service

- **Binary:** `/usr/local/bin/app-orchestration-bim` v0.0.2
- **Port:** 9096 (nginx proxies bim.woodfinegroup.com)
- **Service:** `local-bim-orchestration.service` — active
- **Source:** `app-orchestration-bim/src/main.rs` in this archive
- **Build target:** `/srv/foundry/cargo-target/jennifer/release/app-orchestration-bim`

---

## woodfine-bim-library state (2026-06-03)

Repo at `/srv/foundry/clones/project-bim/woodfine-bim-library/`
Tag: `furniture-symbols-v1` = baseline architectural SVG symbols (fallback)

### blocks/furniture/ (8 pieces)
- 8 × `.ifc` — `IfcFurniture` + `Pset_FurnitureTypeCommon` (ISO 16739-1:2018)
- 8 × `.dxf` — generated footprints from DTCG tokens (for CAD download)
- 8 × `-plan.svg` — architectural line-work symbols (web viewer)

### key-plans/ (3 files)
- `private-office-1.ifc` — PO-1, 30.19 m², 9 furniture placements
- `private-office-2.ifc` — PO-2, 43.20 m², 11 furniture placements
- `private-office-3.ifc` — PO-3, 63.64 m², 12 furniture placements
- Each: IfcSpace + IfcFurniture × N via IfcLocalPlacement → IfcAxis2Placement2D

### scripts/
- `generate-furniture-ifc.py` — tokens → IfcFurniture blocks
- `generate-furniture-dxf.py` — tokens → DXF footprints
- `generate-furniture-symbols-svg.py` — tokens → architectural SVG symbols
- `generate-key-plan-ifc.py` — tokens → Key Plan IFC compositions (PO-1/2/3)
- `run-furniture-pipeline.sh` — runs all 3 steps in sequence

---

## Site routes (app-orchestration-bim)

| Route | What it serves |
|---|---|
| `/furniture` | 8 furniture BIM Objects — IFC download, DWG/RFA → manufacturer |
| `/furniture/download/{slug}.ifc` | Individual IfcFurniture block |
| `/furniture/download/bundle.zip` | All 8 IFC + DWG + RFA files |
| `/key-plans` | PO-1/2/3 Key Plan IFC compositions with download |
| `/key-plans/download/{slug}.ifc` | Individual Key Plan IFC file |
| `/tokens/key-plans.dtcg` | Raw DTCG token viewer (existing, unchanged) |

**Removed this session:** SVG plan viewer column, DXF download button on /furniture.

---

## Token architecture

- **BIM_DESIGN_SYSTEM_DIR** = `pointsav-design-system/` (tokens served via /tokens.json)
- **BIM_LIBRARY_DIR** = `woodfine-bim-library/` (IFC files, Key Plans, SVGs)
- `key-plans.dtcg.json` is in `woodfine-bim-library/tokens/bim/` — read directly by
  `key_plans_handler()` via `library_dir/tokens/bim/key-plans.dtcg.json`
- `interior.dtcg.json` (furniture) is in design-system — read via `state.tokens["interior"]`

---

## Nightly pipeline (auto)

`foundry-bim-furniture.timer` at 03:00 — **unit files in infrastructure/systemd/**
**NOT YET INSTALLED** — requires operator sudo action:
```bash
sudo cp infrastructure/systemd/foundry-bim-furniture.{service,timer} /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now foundry-bim-furniture.timer
```

---

## Pending — Stage 6 promotion

app-orchestration-bim commits on this archive are NOT yet in canonical
`pointsav-monorepo`. Two commits need Stage 6 from Command Session:
- `d412d9f8` — DWG/RFA external links + Cargo.lock fix
- `5acbab54` — /key-plans route + furniture page IFC-first

woodfine-bim-library is a Totebox-tier repo with its own origin — Stage 6
not applicable; commits go direct via git push.

---

## Next session priorities

1. Install nightly timer (operator sudo)
2. Stage 6 for app-orchestration-bim (Command Session)
3. Add more Key Plan categories (other than PO-1/2/3) to generate-key-plan-ifc.py
4. app-workplace-bim scaffold (Wave 3 — separate session)
5. DTCG contamination cleanup (NEXT.md, briefs, manifest all from other archives)


---

## We Own It — Dependency Tier Table

Tier assignments per [we-own-it-principle](../../../conventions/we-own-it-principle.md).

| Component | Tier | Notes |
|---|---|---|
| app-orchestration-bim | Tier 1 — Ours | BIM website pipeline; Rust/axum; all routes; Apache 2.0 |
| woodfine-bim-library | Tier 1 — Ours | IFC content vault; BIM library data; own git origin |
| generate-key-plan-ifc.py | Tier 1 — Ours | IFC floor plan generator; Python pipeline script |
| IfcOpenShell | Tier 3 — Vendored auditable | LGPL-3.0; IFC file parsing; Python dep; no runtime call-home |
| CodeMirror 6 | Tier 4 — Tooling only | MIT; in-browser editor for raw IFC/JSON edits; Bridge → moonshot-code-editor (open decision per project-design) |
| Python runtime | Tier 4 — Tooling only | Pipeline scripting only; not on runtime product path |
