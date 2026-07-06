---
artifact: brief
schema: foundry-brief-v1
archive: project-bim
topic: key-plans-site
status: superseded
superseded_by: BRIEF-app-privategit-bim.md
created: 2026-05-22
updated: 2026-07-06
---

# Brief — Key Plans Live Site (app-orchestration-bim) — SUPERSEDED

**Superseded 2026-07-06.** This brief described the standalone
`app-orchestration-bim` / `local-bim-orchestration` service. That service was
retired 2026-07-02 (disabled, not removed, per Command's deploy message
`command-20260702-resolved-bim-woodfinegroup-com-deployed-`) and replaced by
`app-privategit-bim` on the same port/domain. `app-privategit-bim` ported the
same SVG generator convention (`render::svg::render_kp_zone_svg_from_value`,
same diagram, now at that app's `/key-plans` route). The still-valid technical
content below (SVG tier system, furniture counts, the raw-string footgun) has
been merged, with corrected paths and names, into
`BRIEF-app-privategit-bim.md`'s "Key Plans SVG diagram system" section. This
file is kept for archival record per the "briefs are permanent" rule — refer
to the successor brief for current state.

---

## Current state (as of 2026-05-22 — historical, see note above)

Key Plans catalog is live at `https://bim.woodfinegroup.com/tokens/key-plans.dtcg`.

**Binary:** `app-orchestration-bim v0.0.3` → `/usr/local/bin/app-orchestration-bim`
**Service:** `local-bim-orchestration` (systemd, port 9096, nginx reverse proxy)
**Source:** `pointsav-monorepo/app-orchestration-bim/src/main.rs` (committed; cluster/project-bim)
**DTCG data:** `pointsav-design-system/tokens/bim/key-plans.dtcg.json`
**Rust workspace:** `pointsav-monorepo/` — build with `CARGO_TARGET_DIR=/srv/foundry/cargo-target/jennifer cargo build --release -p app-orchestration-bim`
**Deploy:** `sudo systemctl stop local-bim-orchestration && sudo cp target/release/app-orchestration-bim /usr/local/bin/ && sudo systemctl start local-bim-orchestration`

## What's on the page

24 Key Plan cards across 7 categories:

| Category | cat_order | Cards | Display names |
|---|---|---|---|
| Private Office | 0 | 3 | Small / Medium / Large |
| Medical | 1 | 3 | Small / Medium / Large |
| Business | 2 | 3 | Small / Medium / Large |
| Laboratory | 3 | 3 | Small / Medium / Large |
| Academic | 4 | 3 | Small / Medium / Large |
| Civic | 5 | 3 | Small / Medium / Large |
| Corporate Office | 6 | 5 | Full Floor / Half / Third / Quarter / Eighth |

Cards ordered Small → Medium → Large within category (`size_order()`).
Corporate Office last (`cat_order` = 6).

## SVG diagram system

Each card renders `render_kp_zone_svg(z1, z2, z3, category, area_m2)`.

**Size tier:** computed from `area_m2` and `category` (see plan-bim-objects.md §SVG Diagram System for thresholds). Tier 0=Small, 1=Medium, 2=Large.

**Furniture key differences by tier:**
- Private Office: 1 / 2 / 3 desks at facade
- Medical: 2 / 4 / 6 dental chairs; 1 / 1 / 2 doctor offices
- Laboratory: 3 / 5 / 7 lab bench clusters; 1 / 2 / 2 offices
- Business: 3×3 / 4×4 / 5×5 workstation grid; 2 / 3 / 5 exec offices; 1 / 1 / 2 conference tables
- Academic: workstation bank + conf table / dual banks + oval table / theater seats + bank + round tables
- Civic: 2 / 4 / 5 offices; 1 / 2 / 2 conf rooms; court room in Large only

**Known Rust footgun:** raw string delimiter `r#"..."#` closes on first `"#` — SVG hex colors like `fill="#888"` terminate it. Use `format!()` with escaped quotes instead (current code does this correctly).

## Pending for this site

- [ ] Corporate Office SVG diagrams — currently `_ => {}` (no furniture drawn); awaiting zone depth data
- [ ] Deliverable 1b — `woodfine-bim-library/key-plans/key-plans-registry.md` standalone Markdown (data is all in plan-bim-objects.md)
- [ ] Binary ledger entry — `data/binary-ledger/app-orchestration-bim.jsonl` at Command Session
- [ ] Stage 6 — monorepo sub-clone commits not yet promoted to pointsav origin

## History

| Version | Commit | What changed |
|---|---|---|
| 0.0.1 | (pre-session) | Initial Axum server; basic token catalog |
| 0.0.2 | (prior session) | Key Plans page wired; SVG cross-section diagrams |
| 0.0.3 | 8ce0b9ba | Size-specific furniture per category and tier (2026-05-22) |
| 0.0.4 | 9b27506 | plan-bim-objects.md brief updated — site status + SVG system documented |
