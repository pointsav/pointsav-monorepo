# NEXT.md — app-workplace-bim

> Last updated: 2026-05-27
> Read at session start. Update before session end.

---

## Current state

Reserved-folder. CLAUDE.md + RESEARCH.md present (bim-product-family rules apply).
No Tauri scaffold yet. State remains Reserved until Wave 3.

## Wave 3 (reserved — research phase)

This app is Wave 3. Foundation row registered 2026-05-27 to allow research
notes to accumulate. Full BIM editor specification is in:
- `.agent/rules/bim-product-family.md` — product family rules
- `RESEARCH.md` — detailed market and technical research

## When activating

- Follow bim-product-family.md Phase 1 scope (AutoCAD muscle memory)
- Tauri v1.7 + EUPL-1.2
- Add `minimumSystemVersion: "10.13"` to tauri.conf.json
- IFC operations via IfcOpenShell subprocess only (SYS-ADR-07)
- F12 rule applies: no file enters canonical/ without explicit operator action
