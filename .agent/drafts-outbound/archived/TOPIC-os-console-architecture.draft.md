---
schema: foundry-draft-v1
state: draft-ready-for-language-pass
originating_cluster: project-console
target_repo: pointsav/content-wiki-documentation
target_path: ./
target_filename: topic-os-console-architecture.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-06-12T00:00:00Z
authored_by: totebox@project-console
authored_with: claude-sonnet-4-6
bilingual: true
bilingual_pair: topic-os-console-architecture.es.md
references:
  - app-console-keys/src/cartridge.rs
  - os-console/src/ssh_server.rs
  - os-console/src/main.rs
  - app-console-keys/src/config.rs
  - infrastructure/systemd/console/local-console.service
  - infrastructure/systemd/console/local-pairing-server.service
research_trail:
  source_commits:
    - "app-console-keys/src/cartridge.rs — Cartridge trait definition with set_graphics_caps, flush_hyperlinks, handle_event, tick, render"
    - "os-console/src/main.rs — chassis.register() call order: PeopleCartridge/EmailCartridge/ContentCartridge/InputCartridge/SlmCartridge/SystemCartridge"
    - "os-console/src/ssh_server.rs — SSH binds hardcoded to 0.0.0.0:2222 via russh"
    - "app-console-keys/src/config.rs — ConsoleConfig::load(); ProfileConfig with slm_endpoint, people_endpoint, email_endpoint, content_endpoint"
    - "infrastructure/systemd/console/ — local-console.service, local-pairing-server.service"
  prior_drafts:
    - topic-os-console-platform.md
  citations: []
  operator_inputs:
    - "Cartridge trait set_graphics_caps receives kitty/sixel/font_size/truecolor — all four terminal capability booleans (2026-06-12)"
    - "flush_hyperlinks emits buffered OSC 8 hyperlink sequences after each draw cycle (2026-06-12)"
    - "SSH server mode binds 0.0.0.0:2222 — enabled by --features ssh-server at compile time (2026-06-12)"
    - "pairing-server binds 0.0.0.0:9201 as a separate binary from system-gateway-mba (2026-06-12)"
    - "SYS-ADR-10: F12 InputCartridge is mandatory in every os-console deployment (2026-06-12)"
  related_files:
    - topic-os-console-platform.md
    - app-console-keys/src/cartridge.rs
    - os-console/src/main.rs
notes_for_editor: |
  Architecture-focused companion to topic-os-console-platform.md.
  That draft covers: F-key map, platform targets, MBA connectivity, PDF rendering.
  This draft covers: Cartridge trait internals, terminal capability negotiation,
  hyperlink protocol, SSH server mode, systemd deployment model, configuration schema.
  Do not merge — both are needed; they serve different reader contexts.
  Refinement priorities:
    - Verify SYS-ADR-10 claim text against factory-release-engineering
    - Add specific claim-45, claim-49, claim-54 quotations when FRE copy is available
    - Apply Bloomberg-article register throughout
    - Generate .es.md pair (stub attached)
    - Target length: ~1200 words English
---

# os-console Internal Architecture

## Overview

`os-console` is a single compiled Rust binary that hosts multiple independent TUI
workspaces — cartridges — within a unified keyboard-navigation framework. This document
describes the internal design of that framework: how cartridges are defined, how the
chassis dispatches events, how terminal capabilities are negotiated, and how the binary
is deployed as a managed process.

For the F-key map, platform targets, and MBA connectivity model, see
TOPIC: os-console and app-console-keys.

---

## The Cartridge trait

Every `app-console-*` crate exposes exactly one type that implements the `Cartridge`
trait, defined in `app-console-keys`. The trait is the only interface between a
cartridge and the chassis — there are no other public APIs:

```
trait Cartridge {
    fn fkey(&self) -> FKey;           // which F-key slot this cartridge occupies
    fn title(&self) -> &str;          // displayed in the tab strip
    fn tick(&mut self);               // called once per frame; drain background channels
    fn render(&mut self, frame, area); // draw into the assigned Rect
    fn handle_event(&mut self, event) -> CartridgeAction;
    fn set_graphics_caps(&mut self, kitty, sixel, font_size, truecolor); // default no-op
    fn flush_hyperlinks(&mut self, writer);                               // default no-op
}
```

`tick()` and `render()` are called on every iteration of the event loop.
`handle_event()` is called only when a keyboard or mouse event arrives.
`set_graphics_caps()` is called once at startup, after the chassis queries the
connected terminal for its capabilities. `flush_hyperlinks()` is called after
each `render()` call, allowing cartridges to emit buffered OSC 8 hyperlink
sequences into the terminal output stream.

---

## Cartridge registration

Cartridges are registered at startup in `os-console/src/main.rs` via
`chassis.register(Box<dyn Cartridge>)`. Registration is order-independent with
respect to rendering, but the order determines tab-strip presentation when F-key
slots are not unique. Each registered cartridge must claim a distinct `FKey` slot.

The default build registers six cartridges:

| F-key | Cartridge | Backend |
|---|---|---|
| F2 | `app-console-people` | `service-people` at `people_endpoint` |
| F3 | `app-console-email` | `service-email` at `email_endpoint` |
| F4 | `app-console-content` | `service-content`, `service-slm` |
| F9 | `app-console-slm` | `service-slm` at `slm_endpoint` (Doorman) |
| F11 | `app-console-system` | `service-ppn-pairing` at `pair_endpoint` |
| F12 | `app-console-input` | `service-input` at `ingest_endpoint` |

F12 (`app-console-input`) is mandatory in every deployment per SYS-ADR-10. Omitting
it is a build constraint violation.

---

## Terminal capability negotiation

At startup, the chassis queries the connected terminal using standard escape sequences
and environment inspection:

- **Kitty graphics protocol:** detected via APC response to a probe sequence
- **Sixel:** detected via `TERM` environment and DA2 device attributes
- **Font cell size:** queried via xtwinops (CSI 16 t) when available; falls back to
  a 10×20 px estimate
- **Truecolor:** detected via `COLORTERM=truecolor` or `COLORTERM=24bit`

The resolved capabilities are passed to every registered cartridge via
`set_graphics_caps(kitty, sixel, font_size, truecolor)`. Cartridges use this to
select between 24-bit RGB colors and the named eight-color fallback palette.
The chassis never calls `set_graphics_caps()` again after initial negotiation —
capabilities are fixed for the session lifetime.

### Truecolor color conventions

When truecolor is available, cartridges use a consistent color set:

- Accent (borders, highlights): `Rgb(32, 178, 170)` — a teal close to CSS LightSeaGreen
- Selection background: `Rgb(0, 95, 135)` — a dark teal-blue
- Danger / error: `Rgb(200, 0, 0)` — deep red

When truecolor is unavailable (plain terminals, serial consoles), cartridges fall
back to named colors: Cyan for accents, DarkGray for selection backgrounds, Red for
errors. The visual hierarchy is preserved; only the precision changes.

---

## OSC 8 hyperlinks

`ContentCartridge` (F4) implements `flush_hyperlinks()`. During `render()`, it
collects URL targets from search results and citations into an internal buffer:
`pending_hyperlinks: Vec<HyperlinkTarget>`. After the ratatui draw cycle completes,
the chassis calls `flush_hyperlinks()`, which emits OSC 8 sequences:

```
OSC 8 ; params ; uri ST  (open link)
OSC 8 ; ; ST             (close link)
```

Links are only emitted when the Kitty graphics protocol is active — terminals that
support Kitty graphics also support OSC 8 reliably. The default `flush_hyperlinks()`
no-op in the trait means non-participating cartridges incur no overhead.

---

## Configuration

`app-console-keys` loads `~/.config/os-console/config.toml` via
`ConsoleConfig::load()`. If the file is absent or unparseable, a zero-configuration
default profile is used — the binary is always operable without a config file.

All endpoint fields default to localhost addresses:

| Field | Default | Service |
|---|---|---|
| `slm_endpoint` | `http://localhost:9080` | `service-slm` (Doorman) |
| `people_endpoint` | `http://127.0.0.1:9091` | `service-people` |
| `email_endpoint` | `http://127.0.0.1:9093` | `service-email` |
| `content_endpoint` | `http://127.0.0.1:9081` | `service-content` |
| `ingest_endpoint` | `http://127.0.0.1:9100` | `service-input` |
| `proof_endpoint` | `http://127.0.0.1:9092` | `service-proofreader` |
| `pair_endpoint` | `http://127.0.0.1:9201` | `pairing-server` |

The `plain_mode: bool` flag disables all Rgb colors and box-drawing characters,
producing output compatible with any vt100-class terminal. It is propagated to each
cartridge at construction.

---

## SSH server mode

Compiled with `--features ssh-server`, `os-console` starts an SSH server on
`0.0.0.0:2222` using the `russh` crate rather than launching the local TUI loop.
Remote clients connect via a standard SSH client and receive the same TUI rendered
inside the SSH session's PTY.

The SSH server and local TUI share the same cartridge codebase — the `ssh-server`
feature activates `os-console/src/ssh_server.rs` and disables `main.rs`'s local
event loop. The two modes are mutually exclusive in a single binary invocation.

---

## Systemd deployment

On the Foundry workspace VM, `os-console` is intended to run as a managed systemd
service:

**`local-console.service`** — manages the `os-console` SSH server process:
- Runs as `foundry:foundry`
- `WorkingDirectory=/opt/console/data`
- `ExecStart=/opt/console/bin/os-console`
- `MemoryMax=256M`, `CPUQuota=100%`
- `After=network-online.target local-doorman.service`
- Requires `local-pairing-server.service` to be active

**`local-pairing-server.service`** — manages the MBA pairing ceremony server:
- Runs as `foundry:foundry`
- `ExecStart=/opt/console/bin/pairing-server 0.0.0.0:9201`
- `MemoryMax=64M`, `CPUQuota=25%`
- `After=network-online.target`

Binaries are installed to `/opt/console/bin/`; persistent operator data lives in
`/opt/console/data/`. Service definitions are in
`infrastructure/systemd/console/` in the source tree.

---

## Doctrine anchors

**Claim 45** (Totebox architecture): `os-console` is the human interface of the Totebox
system, not a ring service. It operates within Totebox discipline — session locks,
archive isolation, one session per index.

**Claim 49** (terminal-first design): The keyboard-native design is a planned intended
property of the system, not a product constraint. The terminal interface is the
operator's primary interaction layer.

**Claim 54** (sovereign deployment): `os-console` is planned to be operable without
any external network dependency. All default endpoints resolve to `localhost`; the
binary starts and renders without a config file.

**SYS-ADR-10** (F12 mandatory): The Input Machine (`app-console-input`, F12) is the
mandatory ingest gate. It must be registered in every `os-console` deployment. It
enforces the Five-Field Rule and append-only audit log — the architectural invariant
that all operator-sourced text passes through a verifiable, auditable ingest path.

---

## See also

- TOPIC: os-console and app-console-keys — F-key map, platform targets, MBA connectivity
- TOPIC: Input Machine (F12) — F12 Anchor; SYS-ADR-10 full specification
- TOPIC: Machine-Based Authorization — how os-console connects to os-* peer services
- GUIDE: os-console Operator Reference — startup, keybindings, troubleshooting
- GUIDE: MBA Pairing Ceremony — pairing os-console with os-totebox and other peers
