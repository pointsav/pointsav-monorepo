---
schema: foundry-plan-v1
archive: project-console
title: "os-console Cross-Platform Release"
created: 2026-05-30
updated: 2026-05-31
status: active
authors: [totebox@project-console, claude-sonnet-4-6]
doctrine_anchors: [SYS-ADR-07, SYS-ADR-10, SYS-ADR-19]
companion: BRIEF-os-console-platform.md
---

# os-console Cross-Platform Release

## Purpose

Track the milestone that makes `os-console` distributable as native binaries for:
- **macOS 10.13+ Intel** (High Sierra; 2017–present Intel Macs — hard floor)
- **macOS current Apple Silicon** (Monterey 11.0+ floor on ARM)
- **macOS universal** (`lipo` of the above two)
- **Linux x86_64 static** (`x86_64-unknown-linux-musl` via cargo-zigbuild)

This BRIEF governs Phase B of `BRIEF-leapfrog-2030-coding.md`. It does not
duplicate that file; it adds the release-engineering specifics.

---

## macOS 10.13 compatibility model

**Hard floor: High Sierra 10.13.x (Intel only — Apple Silicon starts at 11.0).**

The app must run on every macOS from 10.13 to current (15.x). Features degrade
gracefully — the app does not crash or hang on older terminal emulators.

| Capability | 10.13 Terminal.app | 10.13 kitty/iTerm2 | 13.0+ any terminal |
|---|---|---|---|
| Text TUI (ratatui/crossterm) | ✅ 256-color | ✅ | ✅ 24-bit |
| Kitty graphics (QR, images) | ❌ → text fallback | ✅ | ✅ |
| Sixel | ❌ → text fallback | ✅ iTerm2 | varies |
| Proofreader/Draft/SLM | ✅ full (network only) | ✅ | ✅ |
| MBA pairing | ✅ ASCII QR fallback | ✅ | ✅ |
| PDF viewer (Phase 7) | ⚠️ text-error pane | ⚠️ if pdfium supports 10.13 | ✅ |

**Principle:** On 10.13 Terminal.app, the text TUI works fully. Kitty/Sixel panels
degrade to a short message explaining the terminal requirement. No crash, no hang.

---

## Build matrix

| Artifact | Rust target | MACOSX_DEPLOYMENT_TARGET | GH Actions runner |
|---|---|---|---|
| Linux static | `x86_64-unknown-linux-musl` | N/A | ubuntu-22.04 + cargo-zigbuild |
| macOS Intel (10.13+) | `x86_64-apple-darwin` | `10.13` | macos-13 |
| macOS Apple Silicon (11.0+) | `aarch64-apple-darwin` | `11.0` | macos-14 |
| macOS universal | `lipo` of above two | — | macos-14 (post-build step) |

---

## Dependency compatibility audit

| Dep | Risk on 10.13 | Mitigation |
|---|---|---|
| `reqwest 0.12` | `native-tls` uses SecureTransport APIs that changed | Use `rustls-tls` feature; drop `native-tls` |
| `ratatui-image 0.9` | Uses terminal probe at runtime; compile-time clean | Runtime graceful degradation (already designed) |
| `pdfium-render 0.8` | Chromium pdfium binary may not target 10.13 | Audit at Phase 7; feature-gate the dependency |
| `crossterm 0.28` | Uses macOS terminal APIs | Test against 10.13 SDK; expect no issues |
| `russh 0.60` | tokio-based; no macOS-specific APIs | Fine |
| `rusqlite 0.32` (bundled) | Compiled from source | Fine |

**Action:** Audit `os-console/Cargo.toml` for `default-tls` / `native-tls` features;
switch reqwest to `rustls-tls` across all crates before adding to CI matrix.

---

## Runtime capability detection

Add `TerminalCaps` struct to `app-console-keys/src/chassis.rs` at startup:

```rust
pub struct TerminalCaps {
    pub kitty: bool,    // Kitty graphics protocol
    pub sixel: bool,    // Sixel graphics
    pub truecolor: bool, // 24-bit color
}
```

Detection happens at startup via terminal queries (in-band probing). Result stored
in `AppConsoleKeys` state and passed to cartridges via `render()` / `tick()` context.
Each cartridge uses `caps.kitty` to choose its render path — no platform compile-time
assumptions, no hard-coded `cfg(target_os)` checks for feature decisions.

---

## Phase B checklist

**B1. `rust-toolchain.toml`** at monorepo root — pin `stable` (matching `service-slm`'s 1.85).

- [ ] Create `pointsav-monorepo/rust-toolchain.toml`

**B2. `.cargo/config.toml`** — **SKIPPED** (intentional). Cargo `[env]` has no target-conditional
syntax, making the plan's proposed content a no-op. `MACOSX_DEPLOYMENT_TARGET` is set correctly
as a per-job env var in the CI workflow. No file created.

**B3. reqwest TLS audit** — **COMPLETE** (2026-05-30, commit `6f21f580`).

- [x] `app-console-keys/Cargo.toml`: `default-features = false, features = ["json", "blocking", "rustls-tls"]`
- [x] `app-console-content/Cargo.toml`: same
- [x] `app-console-input/Cargo.toml`: same
- [x] `app-console-system/Cargo.toml`: same

**B4. GitHub Actions release workflow** — **COMPLETE** (2026-05-30, commit `6f21f580`).

- [x] Trigger: `v*.*.*` annotated tag push + `workflow_dispatch` (operator confirmed)
- [x] `build-linux`: ubuntu-22.04, cargo-zigbuild, `x86_64-unknown-linux-musl`
- [x] `build-macos-intel`: macos-13, `MACOSX_DEPLOYMENT_TARGET=10.13`, `x86_64-apple-darwin`
- [x] `build-macos-arm`: macos-14, `MACOSX_DEPLOYMENT_TARGET=11.0`, `aarch64-apple-darwin`
- [x] `build-macos-universal`: macos-14, `lipo -create` of Intel + ARM
- [x] `release`: `softprops/action-gh-release@v2`, `generate_release_notes: true`, 4 artifacts

**B5. `TerminalCaps` runtime probe** — **COMPLETE** (2026-05-30, commit `6f21f580`).

- [x] `TerminalCaps { kitty, sixel, truecolor }` struct in `app-console-keys/src/chassis.rs`
- [x] Detection: `Picker::protocol_type()` (ratatui-image 9.0.0 `ProtocolType::Kitty/Sixel`) + `COLORTERM` env
- [x] Stored on `AppConsoleKeys`; populated in `run_local()` after probe; `caps()` accessor
- [x] `cargo check --workspace` exits 0

---

## Release artifact naming

```
os-console-{VERSION}-x86_64-unknown-linux-musl     # Linux static
os-console-{VERSION}-x86_64-apple-darwin           # macOS Intel
os-console-{VERSION}-aarch64-apple-darwin          # macOS Apple Silicon
os-console-{VERSION}-universal-apple-darwin        # macOS universal (lipo)
```

Versioning follows workspace `Cargo.toml` `version` field. Tag format: `v{MAJOR}.{MINOR}.{PATCH}`.
Tag `v0.1.0` will trigger the first release build after Stage 6 + canonical promote.

---

## Status

- [x] Phase B1 — rust-toolchain.toml (`6f21f580`)
- [x] Phase B2 — .cargo/config.toml (intentionally skipped — CI env handles it)
- [x] Phase B3 — reqwest TLS audit (`6f21f580`)
- [x] Phase B4 — GitHub Actions release workflow (`6f21f580`)
- [x] Phase B5 — TerminalCaps runtime probe (`6f21f580`)

**Phase B COMPLETE.** Stage 6 pending: Command to push + promote `6f21f580`.
Next: Phase C (Email cartridge F3) and Phase D (SLM cartridge F9).
