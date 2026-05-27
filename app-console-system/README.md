# app-console-system

F11 System Cartridge for os-console — operator panel for pending connection-request approvals.

## Overview

Provides the `SystemCartridge` implementing the `Cartridge` trait. Polls
`GET /v1/pair/pending` every five seconds and presents pending connection requests
in a list. Operator actions:

| Key | Action |
|-----|--------|
| ↑ / k | Move selection up |
| ↓ / j | Move selection down |
| Enter | Approve selected request |
| D | Deny selected request |
| R | Manual refresh |

A badge count is surfaced to the chassis status bar when requests are pending.

## Dependencies

- `app-console-keys` — Cartridge trait and FKey
- `reqwest` (blocking) — HTTP calls to the pairing server
