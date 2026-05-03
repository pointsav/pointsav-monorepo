# Changelog — Workplace\*Proforma

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

### Added
- Initial project scaffold
- README, DEVELOPMENT, ARCHITECTURE documentation
- EUPL v1.2 licence
- Tauri v1 configuration for macOS 10.13 development target
- Package.json with dual macOS/Linux build scripts
- Minimal Rust IPC surface: `open_file`, `save_file`, `get_app_data_dir`
- JavaScript formula engine (Phase 1 MVP) — SUM, AVERAGE, MIN, MAX, COUNT, IF, ROUND, ABS, PMT, PV, FV, NPV, IRR
- JSON schema specification (Schema 1.0 draft) — see `docs/schema.md`
- Grid with sticky label/code columns, section headers, subtotal/total emphasis
- Formula bar with live cell reference display
- Sheet tabs (Assumptions, Proforma, Returns)
- Keyboard navigation matching Excel muscle memory (arrow keys, F2, Enter, Tab, F9 recalc)
- Number formats: currency, percent, number, ratio, with adjustable decimal places
- SHA-256 audit chain — every save embeds a cryptographic digest
- CSP with `connect-src 'none'` — zero outbound network calls at runtime

---

## [0.1.0] — TBD

Phase 1 release. See README.md for full feature scope.

Phase 2 scope (tracked separately):
- IronCalc formula engine via Rust IPC
- XLSX export via rust_xlsxwriter
- Archive integration with os-totebox
- AI sidebar with three-paths pattern
