# NEXT.md — project-bim

> **Scope: this archive only.** Cross-repo and workspace-level items live
> at `~/Foundry/NEXT.md`.

Last updated: 2026-06-09 [totebox@claude-code]

---

## Immediate — needs operator action

- [x] Install nightly BIM pipeline timer — confirmed active (foundry-bim-furniture.timer) [2026-06-05 totebox@claude-code]

## Pending Command Session

- [ ] Stage 6 promotion — app-orchestration-bim [2026-06-03 totebox@claude-code]
  - `d412d9f8` feat: DWG/RFA external links + Cargo.lock fix
  - `5acbab54` feat: /key-plans route + furniture page IFC-first
- [x] **woodfine-bim-library push to origin** — pushed by Command 2026-06-04; origin/main == local main confirmed [2026-06-05 totebox@claude-code]
- [x] **Restart bim service** — local-bim-orchestration active; healthz ok confirmed [2026-06-05 totebox@claude-code]

## Next Totebox session

- [ ] Extend `generate-key-plans.py` to Corporate Office (CO-1/2/3/4/5) and all remaining
  Key Plan categories — awaiting architect drawings for TBD-dimension entries
- [ ] Non-BIM inputs contamination: `inputs/README.md`, `inputs/spv.html`,
  `inputs/current-org-chart-html/`, `inputs/tokens-woodfine.css` — files from
  cluster-totebox-corporate-1 (org chart project); return to that archive, do NOT delete
- [ ] app-workplace-bim Wave 3 scaffold — Tauri v1.7, Phase 1 AutoCAD muscle
  memory, IfcOpenShell subprocess, EUPL-1.2 licence
- [ ] BIM_DESIGN_SYSTEM_DIR decision: woodfine-bim-library tokens (key-plans,
  spatial, etc.) are not loaded by the site — either extend token loading to
  include library_dir/tokens, or keep reading directly per-handler
- [x] Resolve archive contamination: NEXT.md, outbox, briefs README [2026-06-09 totebox@claude-code]

## BIM Objects system — deliverables in progress

- [x] Deliverable 1: Key Plans registry (`key-plans-registry.md`) — 66 entries, 18 with confirmed dimensions [2026-05-21]
- [x] Deliverable 1: PO-1/2/3 IFC files generated [2026-06-04]
- [x] Deliverable 1: Medical (M-1/2/3), Business (B-1/2/3), Laboratory (L-1/2/3), Academic (A-1/2/3), Civic (C-1/2/3) IFC files generated [2026-06-09 totebox@claude-code]
- [x] Deliverable 2: Tiles registry (`tiles/tiles-registry.md`) [2026-06-09 totebox@claude-code]
- [ ] Deliverable 2: Tiles IFC generation (tile-*.ifc files) — deferred to after tiles-registry is ratified
- [ ] Deliverable 3: Floor Plate composition matrix — Rust engine; Phase 4; deferred
- [ ] Deliverable 4: Building Width Calculator — Rust engine; Phase 4; deferred
- [ ] Corporate Office Key Plans (CO-1/2/3/4/5) — IFC blocked on TBD dimensions from architect
- [ ] Infrastructure Key Plans (N-1/2, EE-1/2, etc.) — IFC blocked on TBD dimensions
- [ ] Retail Select + Tech Industrial Key Plans — TBD dimensions from architect
