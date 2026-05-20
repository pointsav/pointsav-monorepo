---
schema: foundry-draft-v1
state: draft-ready-for-language-pass
originating_cluster: project-proofreader
target_repo: pointsav/content-wiki-documentation
target_path: ./
target_filename: topic-os-console-platform.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-20T00:00:00Z
authored_by: totebox@project-proofreader
authored_with: claude-sonnet-4-6
bilingual: true
bilingual_pair: topic-os-console-platform.es.md
references:
  - conventions/architecture-layer-catalog.md (os-console, app-console-* entries)
  - woodfine-fleet-deployment/node-console-operator/guide-command-ledger.md
  - woodfine-fleet-deployment/node-console-operator/guide-console-operations.md
  - .agent/plans/os-console-platform.md
research_trail:
  source_commits:
    - "architecture-layer-catalog.md — os-console Scaffold-coded, app-console-keys Reserved-folder"
    - "guide-command-ledger.md — F-key map, MBA LINK ACTIVE, Zero-Form architecture"
    - "guide-console-operations.md — Derivative HUD, Input Machine anchor"
  prior_drafts: []
  citations: []
  operator_inputs:
    - "os-console is the binary containing system-*, tool-*, and all app-console-* cartridges (2026-05-20)"
    - "app-console-keys is the base chassis — always installed, like service-fs for os-totebox (2026-05-20)"
    - "No nesting: flat Rust workspace, compiled-in modules, no child process launching (2026-05-20)"
    - "F-key map is a work in progress; assignments will evolve during development (2026-05-20)"
    - "PDF viewing: Kitty + Sixel pixel graphics only, no text-extraction fallback (2026-05-20)"
    - "app-console-keys name refers to F-keys, NOT cryptographic keys (2026-05-20)"
  related_files:
    - .agent/plans/os-console-platform.md
    - .agent/plans/leapfrog-2030-coding.md
    - .agent/drafts-outbound/topic-machine-based-authorization.md
notes_for_editor: |
  Comprehensive first draft. Establishes the canonical description of the os-console
  platform, app-console-keys chassis, and cartridge architecture.

  Refinement priorities:
  - Verify current catalog state of all app-console-* crates (check architecture-layer-catalog.md)
  - Generate bilingual .es.md pair
  - Apply Bloomberg-article register
  - Confirm "Type II hypervisor" framing is appropriate for public audience (operator review)
  - Confirm F-key map assignments are finalized before publication (marked WIP in this draft)
  - Target length: ~1200-1500 words English
---

# os-console and app-console-keys

## The platform

`os-console` is Woodfine's keyboard-native console interface — a single Rust binary
that provides direct access to the Totebox Archive, editorial workflows, governance
records, and infrastructure management from a terminal. It runs natively on Linux Mint
and macOS, renders full-featured TUI screens, and connects to backend `os-*` services
through Machine-Based Authorization.

The design principle is "We Own It": every component is compiled into a single binary,
no dynamic plugin loading, no subprocess launching, no nesting. The result is a
console that starts in milliseconds, responds at keyboard speed, and operates fully
offline when backend services are unavailable.

## The binary

`os-console` is the deployable artifact: one binary, one process, all cartridges
compiled in. It runs as a native application on the host operating system — Linux Mint
on NODE-IMAC-12 (iMac 12.1), or macOS 13.* on executive workstations.

The "Type II hypervisor" analogy is useful for understanding the architecture:
`os-console` runs on the host OS and manages multiple independent workloads (the
cartridges) within a single process, providing unified keyboard navigation between
them. The cartridges are not virtual machines; they are compiled Rust library crates.

An optional SSH server mode (compiled with `--features ssh-server`) enables remote
access over port 2222 for use on a GCE VM, enabling the same TUI experience over
an SSH session.

## The base chassis: app-console-keys

`app-console-keys` is the always-installed base chassis inside `os-console`. Its
relationship to `os-console` is analogous to `service-fs`'s relationship to
`os-totebox`: it is the minimum required component that must be present; everything
else is optional.

If an operator only wants the minimal console — F-key navigation and a connection to
one Totebox Archive — `app-console-keys` is what they need. Every other cartridge
is additive.

**What app-console-keys provides:**
- The `Cartridge` trait (the interface all cartridges implement)
- F-key navigation framework: the horizontal tab strip across the top of the console,
  F-key input dispatch, active-cartridge routing
- Status bar: MBA connection state, session identity, tier, session duration
- MBA client: manages connections to paired `os-*` services, presents link status
- Profile-based configuration (`~/.config/os-console/config.toml`)

**Naming note:** "keys" in `app-console-keys` refers to F-keys — keyboard function keys.
It does not refer to cryptographic keys. Machine-Based Authorization is implemented
by `system-gateway-mba`, a separate system-layer crate.

## Cartridges

Each `app-console-*` crate is a library crate that implements the `Cartridge` trait.
Cartridges are compiled into `os-console` directly — not loaded dynamically, not
launched as subprocesses. A cartridge that is not installed simply is not linked in;
its F-key slot is greyed in the tab strip.

Cartridges are optional (except `app-console-keys`). An installation that includes
only `app-console-content` (F4) and `app-console-input` (F12) is a complete, valid
os-console deployment for editorial work.

## F-key map

The console presents twelve addressable slots via F-keys. The map is organized
by domain, with less-frequently-used and more-technical cartridges at higher F-numbers.
F12 is fixed as The Anchor (see TOPIC: Input Machine).

*This map is a work in progress. Assignments will evolve during development.*

| F-key | Cartridge | Domain |
|---|---|---|
| F1 | `app-console-help` | Help overlay — universal; never moves |
| F2 | `app-console-people` | Identity and contacts |
| F3 | `app-console-email` | Communications |
| F4 | `app-console-content` | Editorial — proofread, draft, verify |
| F5 | `app-console-minutebook` | Governance — minutes, resolutions |
| F6 | `app-console-bookkeeper` | Financial ledger |
| F7 | `app-console-bim` | Building information management |
| F8 | `app-console-gis` | Geographic information |
| F9 | `app-console-slm` | AI management and adapter marketplace |
| F10 | `app-console-mesh` | PPN mesh management |
| F11 | `app-console-system` | Live os-* service health and MBA status |
| F12 | `app-console-input` | **The Anchor** — Input Machine (SYS-ADR-10) |

## The status bar

The `app-console-keys` status bar is always visible at the bottom of the console
and provides the operator with a live situational picture:

```
jennifer@woodfine | MBA LINK ACTIVE | F4: Content | Tier A | 00:04:23
```

Components:
- **Identity:** `username@tenant` — set during the MBA pairing ceremony
- **MBA state:** `MBA LINK ACTIVE` / `MBA LINK INACTIVE <reason>` / `MBA LINK PENDING`
- **Active cartridge:** current F-key slot name
- **Tier:** SLM tier in use (A = local; B = cloud burst; C = frontier API)
- **Session duration:** elapsed time since connection

## MBA connectivity

`app-console-keys` maintains outbound MBA connections to paired `os-*` services. Each
pairing is independent: the console can be active with `os-totebox` and inactive with
`os-privategit` simultaneously. The status bar shows the aggregate state.

When `MBA LINK INACTIVE`, os-console operates in local-only mode. Locally-cached
content remains accessible. Backend service requests (to `service-proofreader`,
`service-input`, `service-content`) fail gracefully with an "unavailable" state rather
than crashing.

For a complete description of the MBA mechanism, see TOPIC: Machine-Based Authorization.

## Platform targets

`os-console` runs natively on two platforms:

**Linux Mint (primary — NODE-IMAC-12):** The iMac 12.1 running Linux Mint is the
reference deployment. Any VTE-based terminal works; kitty terminal is recommended for
full graphics capabilities including PDF rendering.

**macOS 13.* (executive workstations):** Native macOS binaries built via GitHub Actions
on macos-14 runners. Compatible terminals: kitty, iTerm2, Ghostty, WezTerm.

## PDF rendering

`os-console` supports in-terminal PDF rendering using the `pdfium-render` library
(Rust bindings to Chromium's pdfium). PDF pages are rendered to RGB bitmaps and
displayed using:

- Kitty graphics protocol (primary — kitty, Ghostty, WezTerm)
- Sixel (fallback — iTerm2, mlterm)
- Error for unsupported terminals

This is pixel rendering, not text extraction. The rendered output is a faithful
reproduction of the PDF page layout, typefaces, and graphics. A compatible terminal
is required.

## Three-Ring Architecture placement

`os-console` is a client of the Three-Ring Architecture, not a ring itself. It connects
to Ring 1 and Ring 2 services through the MBA layer:

- Ring 1 services: `service-input` (via F12 Input Machine), `service-people` (via F2),
  `service-email` (via F3), `service-fs`
- Ring 2 services: `service-content` (knowledge graph), `service-search` (Tantivy)
- Ring 3 service: `service-slm` (Doorman, via Tier A/B/C routing at `http://localhost:8011`)

`os-console` does not replace any ring. It is the human interface through which an
operator instructs the rings.

## See also

- TOPIC: Machine-Based Authorization — the authorization mechanism os-console uses
  to connect to os-* services
- TOPIC: PointSav Private Network — the infrastructure layer os-console runs on
- TOPIC: Input Machine (F12) — The Anchor; mandatory ingest gate
- GUIDE: os-console Operator Reference — F-key map reference, startup, troubleshooting
- GUIDE: MBA Pairing Ceremony — establishing os-console connections to os-* peers
- `woodfine-fleet-deployment/node-console-operator/guide-command-ledger.md` — operational reference
