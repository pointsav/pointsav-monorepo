# Split Code View — technical reference

> Reference for the developer implementing Phase 4 of Workplace✦Presentation.

---

## What it is

The right-hand pane of the editor. Shows the raw HTML of the **active slide only**.
Toggles via View menu → Split Code View, or keyboard shortcut `Ctrl+/`.

When open: canvas and code view split the horizontal space 50/50.
When closed: canvas takes 100%.

---

## The sync model — blur-driven

Edits do not commit on every keystroke. They commit on blur.

```
Canvas pane                         Code pane
-----------                         ---------
user edits text box A
                                    [no change yet — canvas edit in flight]
user clicks outside text box A
  → text box A blurs
  → editor.js regenerates HTML of active slide
  → code pane contents replaced
                                    [code pane now shows updated HTML]

user clicks into code pane
user edits HTML text
                                    [no change yet — code edit in flight]
user clicks outside code pane
  → code pane blurs
  → editor.js parses code pane contents with DOMParser
  → if valid: active slide's element DOM replaced, canvas re-renders
  → if invalid: warning strip shown, canvas keeps last valid render
```

**Why not keystroke-driven.** Keystroke sync causes cursor thrash. The other pane
re-renders mid-typing and either steals focus or jumps the cursor. Every mature
split-pane editor (VS Code markdown preview, Word draft/print view, Excel formula
bar) uses blur or explicit trigger, not keystroke sync. See ADR-PR-03.

---

## Invalid HTML handling

When the code pane loses focus with HTML that fails to parse, the editor must:

1. Show a warning strip at the top of the code pane: yellow background, single line,
   text: *"Invalid HTML — canvas shows last valid state. Fix and click away to apply."*
2. Keep the canvas rendered from the last valid state.
3. **Not** erase or modify the user's in-progress code. They may be mid-edit.
4. When the user fixes the HTML and blurs again, strip clears and canvas updates.

The parse uses `DOMParser('text/html')`. A single root `<section class="slide">`
is expected. If the content parses but doesn't yield a valid slide structure,
treat as invalid.

---

## Scope — active slide only

The code view shows the HTML of the currently active slide only. It does not show:

- The document-level `<html>`, `<head>`, or `<body>` wrappers
- Other slides
- The embedded `<style>` or `<script>` blocks
- The document metadata `<meta>` tag

Editing those at the document level is deferred work (see `CLEANUP_LOG.md`).
Scoping to the active slide keeps the text area manageable and the mental model
clean: *one slide, one block of HTML*.

---

## Syntax highlighting

**Phase 4 ships without syntax highlighting.** The pane is a monospace text area
(font: JetBrains Mono or equivalent monospace from the OFL set, 13px, 1.4 line
height).

If user feedback indicates the plain view is too dense, the candidate library is
`highlight.js` with only the HTML language vendored in — approximately 20KB
uncompressed, MIT-licensed, works with no build step.

Do not add highlighting without explicit commission. See `NEXT.md` open decisions.

---

## The code pane is not a separate document

An important mental model: the code pane and the canvas are two views of the same
underlying element DOM. The document model in `editor.js` holds the canonical
state. Both panes read from it and write to it.

```
            editor.js document model
           /                         \
          /                           \
   canvas.js                    codeview.js
   (renders the DOM)             (renders as text)
```

Neither pane is the source of truth. The model is. When either pane blurs,
it updates the model, and the other pane re-renders from the updated model.
