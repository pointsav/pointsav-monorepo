# Slideshow Runtime — technical reference

> Reference for the developer implementing Phase 6 of Workplace✦Presentation.

---

## What it is

`src/js/slideshow.js` is a self-contained JavaScript runtime. Under 100 lines.
Zero dependencies. It accepts a deck (array of slide DOM nodes or HTML strings)
and renders them as a fullscreen presentation with keyboard control.

It is used in **two places**:

1. **Editor F5 mode** — loaded by the editor when the user presses F5. Takes the
   current in-memory deck and runs it over the editor UI.
2. **Inlined into every saved `.html` file** — `export.js` reads this file's
   contents and inlines them into a `<script>` block in the saved file. When a
   user opens a saved file in any browser, this runtime is what makes it a
   working slideshow.

See ADR-PR-04 for the rationale.

---

## Entry point

```js
startSlideshow(deck, startIndex)
```

- `deck` — array of slide HTML strings or DOM nodes
- `startIndex` — 0-based slide to start on (defaults to 0)

The runtime takes over the viewport and consumes keyboard events until the user
exits with Escape.

---

## Keyboard map

| Key | Action |
|---|---|
| ArrowRight, ArrowDown, PageDown, Space | Next slide |
| ArrowLeft, ArrowUp, PageUp | Previous slide |
| Home | First slide |
| End | Last slide |
| F | Toggle fullscreen |
| B | Toggle black screen |
| W | Toggle white screen |
| Escape | Exit slideshow |

PowerPoint conventions preserved. Do not add shortcuts not in this table without
commission.

---

## Constraints

The runtime must work when inlined into a saved `.html` file with **no other
scripts present**. This means:

- No imports. No modules. Single IIFE or plain function declarations.
- No references to editor state, Tauri IPC, or any external library.
- No dependencies on CSS beyond what the saved file itself includes.
- Must work in Firefox, Chrome, Safari, Edge — any browser with support for
  `document.documentElement.requestFullscreen()` and ES2020 syntax.

**File size budget: under 4 KB minified.** Larger runtimes bloat every saved
file. If features push the runtime over budget, ADR review required.

---

## Rendering approach

On start:
1. Create a fullscreen `<div class="slideshow-overlay">` appended to body.
2. Append slide DOM (one at a time) to that overlay. Other slides hidden with
   `display: none`.
3. On navigation: swap which slide is visible. No transitions in Phase 6
   (transitions deferred — see `CLEANUP_LOG.md`).

On exit:
1. Remove the overlay.
2. Restore keyboard event handling to the editor (no-op if loaded standalone in a
   browser).
3. Exit fullscreen if active.

---

## Black/white screen

Standard presenter feature. B shows a black overlay over the current slide
(press B again to return). W shows a white overlay. Both are useful during live
presentation to redirect audience attention.

Implementation: a single overlay div toggled between `background: black`,
`background: white`, and hidden.

---

## Dual-use discipline

When you edit `slideshow.js` in the editor project, you are editing the runtime
that will ship inside every saved file. Treat it with the same stability
discipline as a library. Every change ships to every future saved file.

When `export.js` inlines the runtime, it should inline the exact contents of
this file — no minification, no transformation. A user inspecting their saved
`.html` should see readable JavaScript. See the file-is-the-product principle.
