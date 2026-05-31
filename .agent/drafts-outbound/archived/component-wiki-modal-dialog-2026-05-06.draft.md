---
schema: foundry-draft-v1
state: committed
committed_ref: 61fc430
language_protocol: DESIGN-COMPONENT
originating_cluster: project-design
target_repo: vendor/pointsav-design-system
target_path: dtcg-vault/components/wiki-modal-dialog/
target_filename: recipe.json
audience: design-system
bcsc_class: vendor-internal
authored: 2026-05-06
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 4
research_suggested_count: 0
open_questions_count: 0
research_provenance: |
  project-editorial brief 2026-05-06T18:30Z.
  Native HTML <dialog> element pattern (modern baseline).
  Focus trap + scroll lock requirements per WCAG 2.1 §2.1.2.
research_inline: true
---

# DESIGN-COMPONENT — wiki-modal-dialog

Modal dialog using native `<dialog>` element. Used for image lightbox, confirmation prompts, and extended preview panels on the wiki. Focus trapped inside dialog while open; scroll lock on `<body>`.

## Structure

```html
<dialog class="ps-modal" id="{{modal-id}}" aria-labelledby="{{modal-title-id}}">
  <div class="ps-modal__surface">
    <header class="ps-modal__header">
      <h2 class="ps-modal__title" id="{{modal-title-id}}">{{title}}</h2>
      <button type="button" class="ps-modal__close" aria-label="Close dialog">×</button>
    </header>
    <div class="ps-modal__body">{{content}}</div>
    <footer class="ps-modal__footer">
      <button type="button" class="ps-btn ps-btn--primary">{{confirm-label}}</button>
      <button type="button" class="ps-btn ps-btn--ghost" formmethod="dialog">{{cancel-label}}</button>
    </footer>
  </div>
</dialog>

<!-- Trigger (outside dialog) -->
<button type="button" class="ps-btn ps-btn--ghost"
        aria-haspopup="dialog"
        onclick="document.getElementById('{{modal-id}}').showModal()">
  {{trigger-label}}
</button>
```

## Tokens needed

- `--ps-surface-elevated` — dialog card background
- `--ps-border-subtle` — dialog border / separator
- `--ps-corner-3` — 8px border-radius on dialog surface
- `--ps-ink-primary` — dialog title + body text
- `--ps-surface-inverse` — backdrop (rgba overlay)
- `--ps-speed-4` — entrance animation (320ms)
- `--ps-ease-display` — dialog entrance easing

## ARIA

- Native `<dialog>` carries `role="dialog"` implicitly. No manual role needed.
- `aria-labelledby` links dialog to its `<h2>` title.
- `aria-modal="true"` on `<dialog>` signals that content outside the dialog is inert.
- Close button: `aria-label="Close dialog"` (icon-only × button has no visible text).
- On open: move focus to the first focusable element inside or to the dialog itself.
- On close: return focus to the element that triggered the dialog.
- `Escape` key: native `<dialog>` handles Escape → `close` event natively.

## Focus trap

Native `<dialog showModal()>` traps Tab focus automatically in supporting browsers (Chrome 98+, Firefox 98+, Safari 15.4+). No JS focus-trap library needed for this baseline.

## Scroll lock

```js
dialog.addEventListener('open', () => document.body.style.overflow = 'hidden');
dialog.addEventListener('close', () => document.body.style.overflow = '');
```

## Research trail

### Done
- Native `<dialog>` browser support verified (Chrome 98+, Firefox 98+, Safari 15.4+; covers >97% of 2026 traffic).
- Focus trap via native dialog showModal() confirmed (no polyfill needed).
- `aria-modal="true"` interaction with AT confirmed per ARIA 1.2 authoring practices.
- Escape key → `close` event native handling documented.
