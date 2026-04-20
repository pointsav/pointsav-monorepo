# CLEANUP_LOG.md — Deferred work register

> Running log of known cleanup items, deferred decisions, and technical debt.
> Append as items arise. Do not delete resolved items — mark them ✅ RESOLVED with a date.
> This is the presentation app's counterpart to the monorepo's vendor-*/moonshot-* register.

---

## Format

Each entry:

```
### [YYYY-MM-DD] Short title — STATUS
Category: architecture | dependency | UX | documentation | test | performance
Description: one paragraph explaining what and why.
Resolution target: phase/date/"when condition X holds".
```

Statuses: OPEN, IN PROGRESS, BLOCKED, RESOLVED.

---

## Seeded items

### [2026-04-19] Tauri v1 constraint — OPEN
Category: dependency
The workplace family is locked to Tauri v1.7 because the owner's iMac runs
macOS 10.13 High Sierra, and Tauri v2 requires 10.15+. When the iMac is
upgraded, all three workplace apps migrate to v2 together. This is not
piecemeal work — do not migrate this app alone. The v1→v2 diff is
dependency versions only; code is substantially identical.
Resolution target: when iMac is upgraded to 10.15 or later.

### [2026-04-19] webkit 4.0 → 4.1 pkg-config shim — OPEN (system-wide workaround in place)
Category: dependency
Ubuntu 24.04 / Linux Mint 22 removed `libwebkit2gtk-4.0-dev`; only 4.1 is
available. Tauri v1 links against 4.0. A shim is in place at
`~/.local/lib/pkgconfig/` aliasing 4.0 → 4.1 and exported via
`PKG_CONFIG_PATH`. System-wide — applies to all three workplace apps.
Removed when the project migrates to Tauri v2.
Resolution target: same as Tauri v1 constraint above.

### [2026-04-19] Icon artwork is placeholder — OPEN
Category: UX
Currently using a solid `#c8a96e` gold square as `icon-source.png`.
The family direction is a motif per app: memo = document-with-fold,
proforma = grid-of-cells, presentation = slide-stack. All in PointSav gold.
Commission real artwork when budget allows.
Resolution target: when icon artwork is commissioned.

### [2026-04-19] Syntax highlighting in code view deferred — OPEN
Category: UX
The split-screen code view ships in Phase 4 without syntax highlighting.
Decision reason: simpler, works, user sees it is just HTML. Confirmed
2026-04-19. Revisit only if user feedback indicates the plain view is
too visually dense. Candidate if commissioned: a small vendored
highlight.js with HTML language only.
Resolution target: post-Phase-7, if feedback requires it.

### [2026-04-19] US Letter landscape instead of 16:9 — ✅ RESOLVED 2026-04-19
Category: architecture
Default slide aspect ratio. PowerPoint's default is 16:9. The banker /
asset-manager audience prints everything on US Letter, so US Letter
landscape (11″ × 8.5″, 1.294:1) is the better institutional fit.
16:9 option deferred to post-ship backlog.
Resolution: ADR-PR-09 adopted. Canvas = 1100×850 logical units.

### [2026-04-19] Three ship layouts — ✅ RESOLVED 2026-04-19
Category: UX
Ship with Title / Content / Blank. Blank is the startup default — no
layout gallery appears at launch. Matches proforma's blank-canvas
discipline. Two-Column dropped to post-ship backlog along with
Comparison, Section Header, Quote, Image-with-caption.
Resolution: locked. See ROADMAP.md Phase 2.

### [2026-04-19] Default typography — ✅ RESOLVED 2026-04-19
Category: UX
Source Sans 3, 24pt body, 40pt title. Confirmed.
Resolution: locked. See ROADMAP.md Phase 2.

### [2026-04-19] Two-Column layout at ship — ✅ RESOLVED (deferred)
Category: UX
Originally proposed as one of four ship layouts. Reduced to three
(Title/Content/Blank) at decision review. Two-Column moves to
post-Phase-7 backlog. Adds back in a themed "Expand layouts"
commission along with Comparison, Section Header, Quote.
Resolution: deferred. Added to post-ship backlog.

### [2026-04-19] No undo/redo yet — OPEN
Category: UX
Phase 2–7 do not include an undo stack. This is a known gap.
Implementation plan (when commissioned): snapshot the document
state on each mutation, maintain a ring buffer of ~50 snapshots,
wire Ctrl+Z / Ctrl+Shift+Z. Consider structural sharing for memory.
Resolution target: post-Phase-7.

### [2026-04-19] Verification Surveyor throttle number — N/A (not applicable to this app)
Category: documentation
Note: the Verification Surveyor 10/day throttle is a platform-level
concept from `service-people` and does not apply to Workplace✦Presentation.
Logged here only to suppress future confusion if someone cross-references
platform docs.

### [2026-04-19] .pptx import — OPEN (explicitly deferred)
Category: feature
Importing existing PowerPoint files (.pptx) requires parsing Open XML
and is a significant undertaking. Not in the Phase 1–7 plan.
If commissioned, scope includes: Open XML parser, element translation
(PowerPoint shapes → our element model), font substitution for missing
typefaces, image extraction, master-slide flattening.
Resolution target: when commissioned. Not scheduled.

### [2026-04-19] Speaker notes pane — OPEN
Category: UX
PowerPoint users expect a speaker notes pane below the canvas. Deferred
to post-Phase-7. When commissioned: add a `notes` field per slide in
the schema; render a small pane below the canvas toggleable via
View → Notes; export notes into the saved file in a `<div class="notes">`
inside each slide section, hidden in the slideshow runtime but visible
in a "presenter view" that a future phase could add.
Resolution target: post-Phase-7.

### [2026-04-19] Presenter view — OPEN
Category: UX
Second-screen presenter view (current slide + next slide + notes + timer)
is a standard PowerPoint feature. Deferred. Requires the speaker notes
feature above as a prerequisite.
Resolution target: post-speaker-notes.

---

## How to close an item

When resolved, edit the entry in place:

```
### [2026-04-19] Icon artwork is placeholder — ✅ RESOLVED 2026-MM-DD
(… original entry text kept for history …)
Resolution: commissioned gold slide-stack motif, replaced icon-source.png
on [date]. Re-ran `npx tauri icon`. Commit: feat(branding): real icon artwork.
```

Do not delete. This file is a trail.
