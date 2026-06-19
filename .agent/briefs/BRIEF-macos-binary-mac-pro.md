---
artifact: brief
schema: foundry-brief-v1
brief-id: project-console-macos-binary-mac-pro
title: "os-console macOS 10.13 Intel Binary — Jennifer's Mac Pro"
status: active
owner: project-console
created: 2026-06-19
updated: 2026-06-19
---

## Context

Jennifer needs os-console running natively on her Mac Pro (macOS 10.13 High Sierra, Intel x86_64).

The iMac (Linux Mint x86_64) is working — binary at `~/bin/os-console`, pairing approved,
tunnel connects on launch.

The install flow is:
```
curl -fsSL https://software.pointsav.com/releases/os-console/install.sh | bash
```
This downloads the pre-built binary, writes `~/.config/os-console/config.toml` (with tunnel
fields), and creates a `.command` desktop launcher. The install.sh already handles
`darwin-x86_64` detection — it just needs the binary to be present on the server.

## Scope

- Build `darwin-x86_64` os-console binary on Jennifer's Mac Pro
- Deploy to software.pointsav.com as version 0.2.4
- Jennifer runs install.sh; gets working binary + config + launcher automatically

Out of scope: Stage 6 promotion (Command Session); WireGuard config changes.

## Decisions locked

**Cross-compilation is blocked.** `cargo-zigbuild` with zig 0.16.0 cannot cross-compile
`os-console` for `x86_64-apple-darwin` because:
- `aws-lc-sys` (pulled in by `reqwest`'s `rustls-tls` feature) links against
  `CoreFoundation` and `Foundation` frameworks — not bundled in zig
- `rusqlite` with `bundled` needs `libobjc` — not in zig's macOS stub set

**pdfium-render is now optional.** Made `pdfium-render` an optional dependency in
`app-console-content/Cargo.toml` with a `pdf` feature flag (committed as 0.2.4).
Linux builds continue using `--features app-console-content/pdf`.
macOS builds compile without it; `render_page()` returns an error stub.

**Linux 0.2.4 deployed.** `/var/lib/local-software/releases/os-console/0.2.4/linux-x86_64`
is live on foundry-prod. `software.pointsav.com/releases/os-console/latest/linux-x86_64`
serves 0.2.4 (verified via HTTP redirect).

**Binary must be built on the Mac Pro itself.** No other path available without the
macOS SDK. Build time ~5-10 min first run (Rust crate downloads); ~30-60 sec incremental.

## Decisions open

- None blocking Jennifer's build — the instructions below are complete.
- After deploy: verify pairing flow works end-to-end on macOS (SSH key detection,
  tunnel connect, pair POST to service-ppn-pairing port 9205).

## Work log

- 2026-06-19: Attempted cargo-zigbuild cross-compile → blocked by aws-lc-sys + rusqlite/bundled
- 2026-06-19: Made pdfium-render optional (commit c9084667, Version 0.2.4)
- 2026-06-19: Deployed linux-x86_64 0.2.4 to foundry-prod; MANIFEST.json updated
- 2026-06-19: Confirmed software.pointsav.com → 34.168.19.68 (foundry-prod); serves 0.2.4

## Carry-forward — what Jennifer needs to do

**Step 1 — On Jennifer's Mac Pro** (one-time, ~10-15 min):

```bash
# Install Xcode Command Line Tools (dialog will appear — click Install):
xcode-select --install

# Install Rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Download source from foundry-workspace:
mkdir -p ~/src
scp -r mathew@34.53.65.203:/srv/foundry/clones/project-console/pointsav-monorepo ~/src/

# Build (5-10 min first time):
cd ~/src/pointsav-monorepo
cargo build --release --bin os-console

# Send binary back for deployment:
scp target/release/os-console mathew@34.53.65.203:/tmp/darwin-x86_64-0.2.4
```

**Step 2 — deploy from foundry-workspace** (once /tmp/darwin-x86_64-0.2.4 arrives):

```bash
scp -i ~/.ssh/google_compute_engine /tmp/darwin-x86_64-0.2.4 \
    mathew@34.168.19.68:/var/lib/local-software/releases/os-console/0.2.4/darwin-x86_64
ssh -i ~/.ssh/google_compute_engine mathew@34.168.19.68 \
    "chmod +x /var/lib/local-software/releases/os-console/0.2.4/darwin-x86_64"
```

**Step 3 — Jennifer runs install.sh** (after deploy):

```bash
curl -fsSL https://software.pointsav.com/releases/os-console/install.sh | bash
```

install.sh will detect `Darwin-x86_64`, download the binary, write config.toml with
tunnel fields, and create `~/Desktop/OS Console.command` (double-click launcher).

## Notes

- SSH key detection in install.sh probes `google_compute_engine`, `id_ed25519`, `id_rsa`.
  Jennifer needs whichever key is authorized on foundry-workspace (34.53.65.203).
- First launch: macOS Gatekeeper will block `.command` file — right-click → Open → Open anyway.
- Stage 6 (promote commit c9084667) is pending — Command Session task.
