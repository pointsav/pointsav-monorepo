---
artifact: topic
schema: foundry-draft-v1
title: "OS Console Architecture"
slug: topic-os-console-architecture
status: draft
language: en
bilingual_pair_required: true
bcsc_class: internal
forbidden_terms_cleared: false
route_to: project-editorial
created: 2026-06-14
updated: 2026-06-14
research_trail:
  sources_cited: false
  claims_verified: false
  sme_review: pending
  external_review: not-required
  last_checked: 2026-06-14
---

# OS Console Architecture

The OS Console is a terminal-native operator interface for Totebox sessions. It
presents a multi-surface panel system driven by function keys, where each panel
is implemented as an independent `Cartridge` that attaches to the console chassis
at a designated F-key slot.

## Chassis and Cartridge Model

`app-console-keys` provides the chassis: a ratatui-based TUI skeleton that owns
the terminal lifecycle, event loop, and Kitty/Sixel graphics rendering pipeline.
The chassis exposes the `Cartridge` trait, which every panel crate implements:

```rust
pub trait Cartridge {
    fn title(&self) -> &str;
    fn render(&self, frame: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: &Event) -> CartridgeAction;
}
```

At runtime the chassis loads exactly one active `Cartridge` at a time. The
operator switches panels via function keys. The chassis handles QR code
generation (Ed25519 key fingerprints rendered as Kitty/Sixel inline images)
independent of panel content.

## Active Panels

### F3 — Email (`app-console-email`)

`EmailCartridge` connects to Exchange Web Services (EWS) via the `service-email`
backend. It presents three views:

- Inbox list — threaded message summaries with unread counts
- Read — full message body with attachment indicators
- Compose/send — plain-text composition with `To:` and `Subject:` fields

Plain mode (no Kitty/Sixel) is supported for terminals that lack graphics protocol
support.

### F9 — SLM (`app-console-slm`)

`SlmCartridge` renders a live health dashboard for the Doorman inference gateway.
It polls the Doorman `/health` endpoint every 10 seconds and displays:

- Tier A/B/C availability and circuit-breaker state
- Entity count from the DataGraph
- Corpus queue depth and daily cost summary

The operator may force a manual refresh with `R`.

### F11 — System (`app-console-system`)

`SystemCartridge` provides the operator panel for Totebox session management. Its
primary function in the current phase is displaying pending-pair approvals: staging
sessions awaiting Command Session sign-off before a commit is promoted. It reads
the pairing registry (`pairings.yaml`) and presents each pending item with its
archive name, commit message, and timestamp.

## Terminal Capabilities

The console detects terminal capabilities at startup:

| Feature | Detection method |
|---------|-----------------|
| Kitty graphics protocol | `TERM` / `TERM_PROGRAM` environment variables |
| Sixel fallback | `$COLORTERM` and terminal capability query |
| Plain mode | Explicit `--plain` flag or capability absent |

QR code rendering (used by `app-console-keys` for key fingerprint display) uses
Kitty inline image protocol when available and falls back to ratatui block-character
rendering otherwise.

## Workspace Membership

The console crates that are active workspace members in `pointsav-monorepo`:

| Crate | State | Notes |
|-------|-------|-------|
| `app-console-keys` | Active | Chassis; Phase 3+4 |
| `app-console-email` | Active | Phase C; EmailCartridge |
| `app-console-slm` | Active | Phase D; SlmCartridge |
| `app-console-system` | Active | Phase 4 (pairing); SystemCartridge |

Additional console surfaces (`app-console-bim`, `app-console-bookkeeper`,
`app-console-content`, `app-console-input`, `app-console-mesh`,
`app-console-minutebook`, `app-console-people`, `app-console-vault`) are at
Reserved-folder or Scaffold-coded state and are not workspace members.

## Relationship to Other Topics

- [PPN Small-Business Compute](topic-ppn-small-business-compute) — the fleet
  management surface that SystemCartridge surfaces in the operator panel
- [Software Distribution Substrate](topic-software-distribution-substrate) —
  the licensing layer that gates deployment of console binaries
