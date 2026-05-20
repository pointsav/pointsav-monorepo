---
schema: foundry-plan-v1
archive: project-proofreader
title: "os-console Platform Architecture"
created: 2026-05-20
status: active
authors: [totebox@project-proofreader, claude-sonnet-4-6]
doctrine_anchors: [claim-45, claim-49, claim-54, SYS-ADR-07, SYS-ADR-10, SYS-ADR-19]
supersedes: tui-pivot-2030.md (Phase 7 deferral section)
---

# os-console Platform Architecture

## Purpose

Consolidated architecture reference for `os-console`, `app-console-keys`, and all
`app-console-*` cartridges. Supersedes the Phase 7 chassis deferral in
`tui-pivot-2030.md`. The chassis is built first, not last.

Read this before any engineering work on os-console, app-console-keys, or any cartridge.

---

## 1. The binary: `os-console`

`os-console` is the single deployable binary for the Woodfine console interface.
Located in `pointsav-monorepo/os-console/` — Scaffold-coded, Cargo.toml present.

**Properties:**
- One process. One binary. No child process launching. No nesting.
- Runs natively on Linux Mint (NODE-IMAC-12, iMac 12.1) and macOS 13.*
- "We Own It" — all dependencies compiled in; no Docker, no dynamic plugin loading
- Feature-gated SSH server: `#[cfg(feature = "ssh-server")]` for GCE VM deployment;
  default build uses crossterm PTY for local terminal use on Linux Mint and macOS

**Type II Hypervisor framing:**
`os-console` runs ON the host OS (Linux Mint), manages `app-console-*` cartridges as
compiled-in Rust modules. The analogy: `os-console` is to its cartridges what a Type II
hypervisor is to its VMs — a host-process that manages multiple independent workloads
without being the OS itself. The cartridges are not VMs; they are compiled library crates.

---

## 2. The base chassis: `app-console-keys`

`app-console-keys` is the always-installed base chassis for `os-console`. Located in
`pointsav-monorepo/app-console-keys/` — Reserved-folder (no Cargo.toml yet; Phase 1
creates it).

**Analogy:** `app-console-keys` is to `os-console` as `service-fs` is to `os-totebox`.
It is the minimum required component. If a user only wants to connect to one Totebox
Archive with the minimal console interface, `app-console-keys` is what they need.

**What app-console-keys provides:**
- The `Cartridge` trait definition (all `app-console-*` crates implement this)
- F-key navigation framework (tab strip, active-cartridge routing, input dispatch)
- Status bar (`MBA LINK ACTIVE`, session identity, tier state, session duration)
- Profile-based connectivity (`~/.config/os-console/config.toml`)
- MBA client: manages outbound connections to os-* peers; presents link status

**What app-console-keys does NOT do:**
- It is NOT the MBA implementation. MBA lives in `system-gateway-mba` (system-* layer).
- "keys" = F-keys (keyboard function keys), NOT cryptographic keys.

---

## 3. Cartridge architecture

All `app-console-*` crates (except `app-console-keys`) are library crates implementing
the `Cartridge` trait. They are compiled into `os-console` directly — no subprocess
launches, no dynamic linking, no plugin protocols.

**Trait shape (approximate):**
```rust
pub trait Cartridge: Send + Sync {
    fn fkey(&self) -> FKey;
    fn title(&self) -> &str;
    fn render(&self, frame: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: Event) -> CartridgeAction;
    fn is_installed(&self) -> bool;
}
```

**Cartridge loading in `os-console/src/main.rs`:**
```rust
AppConsoleKeys::new()
    .cartridge(ContentCartridge::new())   // F4
    .cartridge(InputCartridge::new())     // F12
    // ... other installed cartridges
    .run()
```

Uninstalled cartridges are greyed out in the F-key tab strip. The slot exists visually;
activating a greyed slot shows "Not installed."

---

## 4. F-key map (canonical, work-in-progress)

Assignments will evolve during development. Principle: domain-clustering with
frequency ordering; F12 is immovable (SYS-ADR-10).

| F-key | Cartridge | Domain | Catalog state | Notes |
|---|---|---|---|---|
| F1 | `app-console-help` | Meta | Reserved-folder | Help overlay; never moves |
| F2 | `app-console-people` | Human | Scaffold-coded | Identity, contacts |
| F3 | `app-console-email` | Human | Scaffold-coded | Communications |
| F4 | `app-console-content` | Editorial | Scaffold-coded | Proofread + draft; Phase 0 COMPLETE |
| F5 | `app-console-minutebook` | Records | Reserved-folder | Governance, minutes |
| F6 | `app-console-bookkeeper` | Records | Reserved-folder | Financial ledger |
| F7 | `app-console-bim` | Spatial | Reserved-folder | Building information management |
| F8 | `app-console-gis` | Spatial | Not in catalog yet | Geographic information |
| F9 | `app-console-slm` | Intelligence | Not in catalog yet | SLM management, adapter marketplace |
| F10 | `app-console-mesh` | Infrastructure | Reserved-folder | PPN mesh management |
| F11 | `app-console-system` | System status | Not in catalog yet | Live os-* health, MBA pairing status |
| F12 | `app-console-input` | Boundary | Scaffold-coded | **The Anchor** — SYS-ADR-10; NEVER moves |

**Action for Command Session:** Add `app-console-gis`, `app-console-slm`,
`app-console-system` to `conventions/architecture-layer-catalog.md`.

**F10/mesh naming note:** `guide-mesh-execution.md` calls the `os-network-admin`
web interface "the F8 Terminal." That name pre-dates the os-console F-key map. In
os-console: F8=GIS, F10=mesh. The guide should be updated when `app-console-mesh`
is developed.

---

## 5. MBA peer-to-peer connectivity

Machine-Based Authorization (MBA) connects `os-console` to each `os-*` peer directly,
peer-to-peer. NOT network-layer authorization.

**os-console connects to via MBA:**
- `os-totebox` — Totebox Archive (content, people, email, files)
- `os-orchestration` — Command hub (multi-archive aggregation)
- `os-privategit` — Air-gapped source control vault
- `os-mediakit` — Media production and documentation wiki
- `os-network-admin` — PPN mesh management authority

**MBA architecture summary:**
- `system-gateway-mba` (Scaffold-coded, Cargo.toml present) — server-side verifier on each os-* peer
- `app-console-keys` — client-side pairing state; presents `MBA LINK ACTIVE`
- `pairings.yaml` — topology record (NOT a credentials store)
- SSH public key fingerprints for verification (public data; not stealable credentials)
- No central permissions database

**Critical separation:** The PointSav Private Network (PPN) carries packets but has
no access into the os-* application layer. MBA is peer-to-peer above the network layer.
See `TOPIC-pointsav-private-network.md` and plan `os-console-platform.md` §5.

---

## 6. Platform targets

| Platform | Mode | Terminal requirement | Build |
|---|---|---|---|
| Linux Mint (NODE-IMAC-12) | Primary; local crossterm PTY | Any VTE terminal; kitty recommended | Native `cargo build` |
| macOS 13.* | Primary; local crossterm PTY | kitty, iTerm2, Ghostty, WezTerm | GitHub Actions macos-14 runner |
| GCE VM | SSH server (`--features ssh-server`) | Any terminal over SSH port 2222 | Native `cargo build` |
| Linux Mint static binary | Distribution artifact | — | cargo-zigbuild (musl) |

**PDF terminal requirement:** Kitty graphics protocol (primary) + Sixel fallback.
Terminals without graphics protocol support receive an error — no text-extraction
fallback (operator decision, 2026-05-20).

---

## 7. PDF viewing

PDF rendering: `pdfium-render` (Rust bindings to Chromium pdfium) → RGB bitmap →
Kitty graphics protocol → Sixel fallback → error on unsupported terminals.

**Decision:** Pixel rendering only. Text extraction NOT in scope. See Phase 7 in
`leapfrog-2030-coding.md`.

**"We Own It" note:** pdfium is a Google/Chromium library. Accepted as external
dependency — no "We Own It" Rust PDF bitmap renderer exists at this time.
`moonshot-*` family may eventually include a sovereign PDF renderer.

---

## 8. Workspace crate layout (target state after Phase 1)

```
pointsav-monorepo/
├── os-console/                ← bin crate → the deployable binary (Scaffold-coded)
├── app-console-keys/          ← lib crate → base chassis (Reserved-folder → create in Phase 1)
├── app-console-content/       ← lib crate → F4 (Phase 0 spike → convert to lib in Phase 1)
├── app-console-input/         ← lib crate → F12 The Anchor (Scaffold-coded)
├── app-console-people/        ← lib crate → F2 (Scaffold-coded)
├── app-console-email/         ← lib crate → F3 (Scaffold-coded)
├── app-console-minutebook/    ← lib crate → F5 (Reserved-folder)
├── app-console-bookkeeper/    ← lib crate → F6 (Reserved-folder)
├── app-console-bim/           ← lib crate → F7 (Reserved-folder)
├── app-console-gis/           ← lib crate → F8 (not yet in catalog)
├── app-console-slm/           ← lib crate → F9 (not yet in catalog)
├── app-console-mesh/          ← lib crate → F10 (Reserved-folder)
├── app-console-system/        ← lib crate → F11 (not yet in catalog)
├── app-console-help/          ← lib crate → F1 (Reserved-folder)
├── system-gateway-mba/        ← lib crate → MBA server side (Scaffold-coded)
└── [additional system-*, tool-* as needed]
```

**Phase 0 refactor note:** `app-console-content/src/main.rs` (standalone spike binary)
→ `app-console-content/src/lib.rs` (exports `ContentCartridge`). The russh SSH server
code in main.rs moves to `os-console/src/main.rs` behind `#[cfg(feature = "ssh-server")]`.

---

## 9. Configuration

Profile-based at `~/.config/os-console/config.toml`:

```toml
[profile.default]
mode = "local"  # local | gce-native | tunnel | wireguard | offline

[profile.local]
# Linux Mint / macOS — crossterm PTY; no SSH server
totebox_endpoint = "http://localhost:9000"
slm_endpoint = "http://localhost:8011"

[profile.gce-native]
# On GCE VM — SSH server mode
ssh_port = 2222
totebox_endpoint = "http://localhost:9092"
slm_endpoint = "http://localhost:8011"
```

---

## 10. Critical correctness notes (common errors to avoid)

| Issue | Wrong | Correct |
|---|---|---|
| Doorman endpoint | `http://localhost:9080` | `http://localhost:8011` |
| Doorman response field | `.choices[0].message.content` | `.content` |
| Long-poll timeout | 30s | 300s on `/v1/proofread`; 30s elsewhere |
| russh async_trait | Required | Not required in russh 0.60 — native async fn |
| Chassis timing | Phase 7 (after proofreader) | Phase 1 (chassis first) |
| F10 cartridge name | `app-console-network` | `app-console-mesh` (catalog name) |
| Phase 0 spike | Is the final architecture | Is a proof-of-concept; refactor in Phase 1 |
