# app-workplace-aibridge

[ 🇪🇸 Leer en Español ](./README.es.md)

The **AI section-edit bridge core** for the Workplace workbench. It lets a user
highlight a section of a document, hand **only that section** to an external AI
session, and apply the result — instead of routing an entire file through a model.

This crate is the deterministic Rust core that a Model Context Protocol (MCP) server
wraps. It composes:

- [`moonshot-docengine`](../moonshot-docengine) — snaps an arbitrary selection to its
  enclosing document section and addresses the source by byte span.
- [`moonshot-crdt`](../moonshot-crdt) — applies the AI's replacement as an undoable,
  version-bumping edit.

## Tool surface

| Tool | Method | Purpose |
|---|---|---|
| `read_selection` | `Bridge::read_selection(span)` | Snap a selection to its section; return the isolated text handed to the AI. |
| `propose_edit` | `Bridge::propose_edit(span, new)` | Preview the would-be buffer text without committing. |
| `commit_edit` | `Bridge::commit_edit(span, new)` | Apply the replacement as an undoable edit; returns the new version id. |

## SYS-ADR-07

Structured fiduciary/geometric schemas — proforma, schedule, GIS, BIM — must never be
routed through an AI layer. `Bridge` refuses those at every entry point
(`BridgeError::SchemaForbidden`). Only prose, code, and presentation content is eligible.

## Scope

This crate is the deterministic core only. The MCP wire protocol and the live
Claude-session connection are the integration layer, verified against a running
workbench (not headless). Zero runtime dependencies beyond the two sibling cores.
