# INSTALL.md — Moving the scaffold into place

> Delete this file once the scaffold is in place. It is not part of the project.

---

## Target

```
/home/mathew/Foundry/factory-pointsav/pointsav-monorepo/app-workplace-presentation/
```

The folder already exists (empty). Move everything from this scaffold into it.

---

## One-shot copy

From wherever you downloaded this scaffold:

```bash
TARGET="/home/mathew/Foundry/factory-pointsav/pointsav-monorepo/app-workplace-presentation"
SOURCE="$(pwd)"    # adjust if you're elsewhere

# Mirror everything, including hidden files (.gitignore):
cp -r "$SOURCE/." "$TARGET/"

# Confirm .gitignore made it across:
ls -la "$TARGET/" | grep gitignore
```

---

## Verify the structure

```bash
cd /home/mathew/Foundry/factory-pointsav/pointsav-monorepo/app-workplace-presentation
tree -L 2 -a -I 'node_modules|target|fonts'
```

Expected:

```
.
├── .gitignore
├── ARCHITECTURE.md
├── CHANGELOG.md
├── CLAUDE.md
├── CLEANUP_LOG.md
├── DEVELOPMENT.md
├── INSTALL.md          ← delete this after install
├── LICENCE
├── Makefile
├── NEXT.md
├── README.md
├── ROADMAP.md
├── docs
│   ├── fonts.md
│   ├── licence-header.txt
│   ├── print-pipeline.md
│   ├── slideshow-runtime.md
│   └── split-code-view.md
├── package.json
├── scripts                     ← empty, Claude Code populates in Phase 1
├── src
│   ├── js
│   │   └── vendor              ← empty, Claude Code populates in Phase 1
│   └── styles                  ← empty, Claude Code populates in Phase 1
└── src-tauri
    ├── Cargo.toml
    ├── build.rs
    ├── icons
    │   └── README.md
    ├── src
    │   └── main.rs
    └── tauri.conf.json
```

---

## Copy LICENCE body from memo

The `LICENCE` file in this scaffold is a stub pointing to memo's LICENCE. Copy
the full EUPL-1.2 text across:

```bash
cp /home/mathew/Foundry/factory-pointsav/pointsav-monorepo/app-workplace-memo/LICENCE \
   /home/mathew/Foundry/factory-pointsav/pointsav-monorepo/app-workplace-presentation/LICENCE
```

---

## Initial git commit

```bash
cd /home/mathew/Foundry/factory-pointsav/pointsav-monorepo/app-workplace-presentation
git init
git add .
git commit -m "chore(init): repository scaffold

- CLAUDE.md project memory, NEXT.md action list, ROADMAP.md 7-phase plan
- CLEANUP_LOG.md deferred work register
- ARCHITECTURE.md with 8 ADRs
- DEVELOPMENT.md setup guide (includes webkit 4.0->4.1 shim)
- README.md bilingual EN/ES per project convention
- Tauri v1.7 config with [workspace] opt-out
- Four IPC commands matching memo pattern
- EUPL-1.2 licensed"
```

---

## Start Phase 1

From the target directory:

```bash
claude
```

Claude Code auto-loads `CLAUDE.md`. Then paste the Phase 1 commission prompt
from `NEXT.md` — the block under *"Paste this into Claude Code to start Phase 1"*.

---

## Delete this file

Once the above is done:

```bash
rm INSTALL.md
git add -A && git commit -m "chore: remove INSTALL.md scaffold helper"
```

This file is not part of the project. It exists only to get the scaffold
into its home.
