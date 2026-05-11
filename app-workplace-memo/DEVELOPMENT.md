# Development Setup — Workplace\*Memo

This guide covers setting up a local development environment on:
- **macOS 10.13 High Sierra (Mac Pro)** — current development machine
- **Linux (Ubuntu 22.04 / Debian 12)** — production target platform

---

## Prerequisites

### macOS 10.13 (High Sierra)

> Tauri v1 is required for macOS 10.13. Tauri v2 requires macOS 10.15+.

**1. Xcode Command Line Tools**
```bash
xcode-select --install
```

**2. Homebrew**
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

**3. Rust (via rustup)**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup default stable
rustup target add x86_64-apple-darwin
```

**4. Node.js (v18 LTS — last version supporting 10.13)**

> Node.js v18 is the last release with macOS 10.13 support. v20+ requires 10.15.

```bash
# Install via nvm for version control
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
source ~/.nvm/nvm.sh
nvm install 18
nvm use 18
nvm alias default 18
```

**5. Verify**
```bash
rustc --version      # rustc 1.77.0 or later
cargo --version      # cargo 1.77.0 or later
node --version       # v18.x.x
npm --version        # 10.x.x
```

---

### Linux (Ubuntu 22.04 / Debian 12)

> Linux is the production sovereign platform. Tauri v2 is used here.

**1. System dependencies (WebKitGTK + build tools)**
```bash
sudo apt update
sudo apt install -y \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  patchelf \
  libxdo-dev
```

**2. Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup default stable
```

**3. Node.js (v20 LTS)**
```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
source ~/.nvm/nvm.sh
nvm install 20
nvm use 20
nvm alias default 20
```

---

## Install Dependencies

From the `app-workplace-memo/` directory:

```bash
npm install
```

This installs:
- `@tauri-apps/cli` — Tauri CLI (v1.x on macOS 10.13, v2.x on Linux)
- `@tauri-apps/api` — Tauri JS bindings
- Node dev dependencies

---

## Development Server

```bash
npm run tauri dev
```

This starts:
1. The Vite dev server for the frontend (hot-reload)
2. The Tauri application window (native OS window with WebView)

The application window opens automatically. File system access and print dialogs are live.

---

## Platform-Specific Notes

### macOS 10.13 — Known Limitations

| Feature | Status | Notes |
|---|---|---|
| Live pagination (Paged.js) | ✅ Works | WKWebView on 10.13 supports required CSS |
| Base64 font rendering | ✅ Works | Embedded fonts display correctly |
| Print dialogue / PDF | ✅ Works | Native macOS print sheet |
| `@page` CSS margins | ⚠️ Partial | Safari 11 (10.13) has limited `@page` support |
| Silent PDF export | ✗ Not in Phase 1 | WKWebView private API, Phase 2 |
| File System Access | ✅ Works | Via Tauri v1 `tauri::api::dialog` |

**CSS `@page` workaround for 10.13:** Until the macOS build is upgraded to Tauri v2, print margins are set via `@media print` body padding rather than `@page` margin boxes. The visual output is correct; the method differs from the Linux build.

### Linux — Production Target

All features are available on Linux. WebKitGTK provides the WebView — fully open source (LGPL), auditable, no Apple or Microsoft dependency.

---

## Build for Production

### macOS (development binary)
```bash
npm run tauri build
```
Output: `src-tauri/target/release/bundle/macos/WorkplaceMemo.app`

### Linux (sovereign production binary)
```bash
npm run tauri build
```
Output:
- `src-tauri/target/release/bundle/deb/workplace-memo_*.deb`
- `src-tauri/target/release/bundle/appimage/workplace-memo_*.AppImage`

The AppImage is a single portable file that runs on any Linux distribution without installation. This is the recommended distribution format for the initial Linux release.

---

## Running the Embed-Fonts Script

Before building a release, fonts must be embedded as base64 into the JS bundle:

```bash
./scripts/embed-fonts.sh
```

This reads all `.woff2` files from `fonts/` and writes `src/js/font-data.js` containing base64-encoded font data. This file is `.gitignore`d and regenerated at build time — font binary data does not live in the repository, only the source `.woff2` files do.

---

## Codebase Map

```
src-tauri/src/main.rs       — Rust entry point and IPC command handlers
src/index.html              — Application shell (no build step required)
src/js/editor.js            — contenteditable canvas, undo/redo, selection
src/js/toolbar.js           — Ribbon controls, execCommand wrappers
src/js/pagination.js        — Paged.js bootstrap, page break management
src/js/fonts.js             — Font panel, download, base64 registration
src/js/templates.js         — Template CSS bundles, switching logic
src/js/export.js            — HTML assembly, font embedding, print trigger
src/styles/app.css          — Application chrome (toolbar, ruler, desktop)
templates/                  — One CSS file per template
fonts/                      — .woff2 font files (tracked in git)
```

---

## IPC Commands (Rust ↔ JS)

The Rust core exposes exactly four IPC commands to the frontend:

| Command | Direction | Purpose |
|---|---|---|
| `open_file` | JS → Rust | Open native file picker, return HTML string |
| `save_file` | JS → Rust | Open native save picker, write HTML string to disk |
| `get_app_data_dir` | JS → Rust | Return path to app data directory (for font storage) |
| `read_font_file` | JS → Rust | Read a font file from app data dir, return base64 |

No other system access is exposed. The CSP blocks all external connections.

---

## Environment Variables

None required. The application is fully self-contained with no external service dependencies.

---

## Git Workflow

This package lives at:
```
pointsav-monorepo/app-workplace-memo/
```

Branch strategy follows the monorepo convention:
- `main` — stable, tagged releases only
- `dev` — active development
- `feature/[name]` — feature branches off `dev`

---

## Troubleshooting

**`webkit2gtk` not found on Linux**
```bash
sudo apt install libwebkit2gtk-4.1-dev
# If 4.1 is not available on your distro:
sudo apt install libwebkit2gtk-4.0-dev
# Then update src-tauri/Cargo.toml: tauri feature "webkit2gtk-4-0"
```

**`nvm: command not found` after install**
```bash
source ~/.bashrc   # or ~/.zshrc
```

**Tauri build fails on macOS 10.13 with linker errors**
```bash
# Ensure Xcode CLT matches macOS version
softwareupdate --list
# Install the correct CLT package for your macOS version
```

**Font panel shows no fonts**
Run the embed script first:
```bash
./scripts/embed-fonts.sh
```
