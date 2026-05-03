# Proforma Templates

This directory holds JSON template files for the Workplace Proforma application.

Each template is a standalone `.json` file conforming to the Schema 1.0 specification (see `docs/schema.md`). Users can start a new proforma from any template via File → New from Template.

## Planned templates

- `multifamily-10yr.json` — residential multifamily, 10-year hold
- `office-net-lease.json` — office single-tenant NNN lease
- `retail-strip-center.json` — retail multi-tenant

Templates are part of the application bundle (declared as a resource in `tauri.conf.json`) and are also overridable from `~/.local/share/workplace-proforma/templates/` at runtime.
