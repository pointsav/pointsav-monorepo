# DEVELOPMENT.md — Workplace✦Presentation

> Setup guide. Covers Linux Mint 22 (primary dev target) and macOS 10.13 (iMac).
> Every environment fix here also applies to the sibling workplace apps.

---

## Prerequisites

| Tool | Version | Why |
|---|---|---|
| Rust | 1.95+ | Tauri backend |
| Node.js | 20+ | Tauri CLI |
| npm | 10+ | package manager |
| Git | any recent | version control |
| ImageMagick | any recent | generate `icon-source.png` once |

Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

---

## Linux Mint 22 / Ubuntu 24.04 — first-time setup

This environment needs three fixes that do not apply to other Linux distros. All three are one-time setup and benefit the whole workplace family.

### 1. System dev packages

```bash
sudo apt update
sudo apt install -y \
  build-essential pkg-config libssl-dev \
  libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev \
  libsoup-3.0-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev \
  imagemagick
```

Note: Ubuntu 24.04 / Mint 22 removed `libwebkit2gtk-4.0-dev`. Only 4.1 is available. The shim below bridges the version gap.

### 2. webkit 4.0 → 4.1 pkg-config shim

Tauri v1 looks for `webkit2gtk-4.0.pc` and `javascriptcoregtk-4.0.pc`. The system only provides 4.1. Create shims once, benefit all workplace apps.

```bash
mkdir -p ~/.local/lib/pkgconfig

cat > ~/.local/lib/pkgconfig/webkit2gtk-4.0.pc <<'EOF'
prefix=/usr
exec_prefix=${prefix}
libdir=${prefix}/lib/x86_64-linux-gnu
includedir=${prefix}/include

Name: WebKit2GTK (4.0 compat shim)
Description: Web content engine — redirects to 4.1
Version: 2.44.0
Requires: webkit2gtk-4.1
Libs: -lwebkit2gtk-4.1
Cflags: -I${includedir}/webkitgtk-4.1
EOF

cat > ~/.local/lib/pkgconfig/javascriptcoregtk-4.0.pc <<'EOF'
prefix=/usr
exec_prefix=${prefix}
libdir=${prefix}/lib/x86_64-linux-gnu
includedir=${prefix}/include

Name: JavaScriptCoreGTK (4.0 compat shim)
Description: JavaScript engine — redirects to 4.1
Version: 2.44.0
Requires: javascriptcoregtk-4.1
Libs: -ljavascriptcoregtk-4.1
Cflags: -I${includedir}/webkitgtk-4.1
EOF
```

Add to `~/.bashrc` (or your shell's init file):

```bash
export PKG_CONFIG_PATH="$HOME/.local/lib/pkgconfig:$PKG_CONFIG_PATH"
```

Reload: `source ~/.bashrc`

Verify: `pkg-config --modversion webkit2gtk-4.0` should print `2.44.0` or similar.

### 3. Cargo workspace opt-out

The monorepo root (`pointsav-monorepo/Cargo.toml`) declares a workspace for platform crates. The `app-workplace-*` apps are not members. Each needs an empty `[workspace]` table in its `src-tauri/Cargo.toml`.

This file ships with the opt-out already in place. Do not remove it.

```toml
# At the end of src-tauri/Cargo.toml:
[workspace]
```

If the opt-out is missing, `cargo build` fails with an error about being unable to resolve the parent workspace.

---

## macOS 10.13 High Sierra — iMac

Works out of the box with Tauri v1.7. Confirmed stack:

- Install Xcode Command Line Tools: `xcode-select --install`
- Install Rust as above
- Install Node.js 20 (use `nvm` — Node 20 does not have a native macOS 10.13 installer)
- `make setup && make dev`

Do not migrate this machine to Tauri v2 — v2 requires 10.15+. See `CLEANUP_LOG.md` for the coordinated upgrade plan.

---

## First build

```bash
cd app-workplace-presentation
make setup          # npm install + download-deps.sh + embed-fonts.sh
make icons          # generates platform icons from icon-source.png
make dev            # launches the development build with hot reload
```

Frontend changes (HTML/CSS/JS in `src/`) hot-reload instantly while `make dev` is running.

Rust changes (files in `src-tauri/`) trigger a recompile — typically 10–30 seconds.

---

## Icon generation

The icon master is `src-tauri/icons/icon-source.png`, 1024×1024, PointSav gold. This is the only icon file committed to git. Derived platform formats are generated locally.

First-time icon generation (or after updating `icon-source.png`):

```bash
# If icon-source.png does not yet exist, create a gold square placeholder:
convert -size 1024x1024 xc:'#c8a96e' src-tauri/icons/icon-source.png

# Generate all platform-specific formats:
npx tauri icon src-tauri/icons/icon-source.png
```

Generated files (gitignored): `32x32.png`, `128x128.png`, `128x128@2x.png`, `icon.icns`, `icon.ico`, `Square*.png`, `StoreLogo.png`.

When a commissioned icon arrives, replace `icon-source.png` and re-run `npx tauri icon`.

---

## Font setup

Fonts are SIL Open Font Licence families, downloaded on setup, base64-encoded into `src/js/font-data.js`.

```bash
bash scripts/download-deps.sh    # downloads fonts to fonts/ and Paged.js to src/js/vendor/
bash scripts/embed-fonts.sh      # reads fonts/ and writes src/js/font-data.js
```

The `fonts/` directory and both generated files are gitignored. They are regenerated on every fresh clone via `make setup`.

---

## Troubleshooting

### "failed to select a version for the requirement tauri"

Cargo can't find a compatible Tauri version. Check `src-tauri/Cargo.toml`: `tauri = { version = "1.7", ... }`. Run `cargo update --manifest-path src-tauri/Cargo.toml`.

### "Package webkit2gtk-4.0 was not found in the pkg-config search path"

The pkg-config shim is not active. Verify:
```bash
echo $PKG_CONFIG_PATH        # should include ~/.local/lib/pkgconfig
ls ~/.local/lib/pkgconfig/   # should show webkit2gtk-4.0.pc and javascriptcoregtk-4.0.pc
```

If `PKG_CONFIG_PATH` is unset in the current shell, re-source: `source ~/.bashrc`.

### "error: current package believes it's in a workspace when it's not"

The `[workspace]` opt-out is missing from `src-tauri/Cargo.toml`. Append:

```toml
[workspace]
```

### Trailing comma error in tauri.conf.json

Tauri v1's config validator rejects trailing commas. Remove them.

### Blank window on launch with no errors

Usually a CSP violation — an inlined script or style is being blocked. Open the dev console (right-click → Inspect) and check the console tab. Adjust the CSP in `src-tauri/tauri.conf.json` only if strictly necessary; prefer fixing the violating code.

### "glob pattern did not match any file" during build

A `bundle.resources` entry in `tauri.conf.json` points to a directory that doesn't exist. Either create the directory or remove the entry.

### VS Code not installing via snap

Linux Mint disables snap by default. Install VS Code from Microsoft's apt repo:

```bash
wget -qO- https://packages.microsoft.com/keys/microsoft.asc \
  | gpg --dearmor > microsoft.gpg
sudo install -o root -g root -m 644 microsoft.gpg /etc/apt/trusted.gpg.d/
sudo sh -c 'echo "deb [arch=amd64] https://packages.microsoft.com/repos/code stable main" > /etc/apt/sources.list.d/vscode.list'
rm microsoft.gpg
sudo apt update && sudo apt install -y code
```

---

## Running with Claude Code

After first-time setup, the preferred dev workflow is Claude Code:

```bash
npm install -g @anthropic-ai/claude-code
cd /home/mathew/Foundry/factory-pointsav/pointsav-monorepo/app-workplace-presentation
claude
```

Claude Code auto-loads `CLAUDE.md` from the repo root. Use `/model` to switch between Sonnet (bulk work) and Opus (architecture decisions). Accept-Edits mode recommended after you're familiar.

The paste-in commission prompts for each phase live in `NEXT.md` and `ROADMAP.md`.
