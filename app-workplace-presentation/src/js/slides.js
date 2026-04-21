// Copyright (C) 2026 PointSav Digital Systems
// Licensed under the European Union Public Licence v. 1.2 (EUPL-1.2).
// See LICENCE in the project root or https://eupl.eu/1.2/en/ for the full text.

/* =============================================================================
   slides.js — slide navigator (left pane)
   =============================================================================
   Renders the slide navigator into #slide-navigator: one thumbnail row per
   slide in the document, with the active slide highlighted. Click a row to
   jump; drag a row to reorder; right-click to open a Duplicate / Delete /
   New Slide After menu.

   Thumbnails are rendered directly from the document model — not cloned from
   the live canvas DOM. Each thumbnail is a scaled-down copy of the canvas
   DOM shape (.slide-canvas-mini > .slide-element), non-interactive, using
   the same CSS-transform pattern as the main canvas. The model is the single
   source of truth; the navigator holds no state of its own beyond a drag-in-
   progress marker and the currently-open context menu.

   Responsibilities:
     - Render navigator rows from state.document.slides on each render()
     - Handle click (setActiveSlide), right-click (context menu), and
       HTML5 drag-and-drop (reorderSlide) — all via PresentationEditor
     - Dismiss the context menu on Escape, click-outside, window blur, or
       the next contextmenu event

   Non-responsibilities:
     - Document state (owned by editor.js)
     - Slide-element rendering in the main canvas (canvas.js)
     - File I/O, save, export
   ============================================================================= */

(function () {
  'use strict';

  let navigatorEl      = null;  // #slide-navigator
  let contextMenuEl    = null;  // open .slide-context-menu, if any
  let contextOverlayEl = null;  // its click-outside catcher

  const dragState = { fromIndex: null };

  /* ─── Public render ─────────────────────────────────────────────────────── */

  function render(state) {
    if (!navigatorEl) {
      navigatorEl = document.getElementById('slide-navigator');
    }
    if (!navigatorEl) return;

    // A re-render detaches the row the menu was anchored to.
    dismissContextMenu();

    navigatorEl.innerHTML = '';
    state.document.slides.forEach(function (slide, index) {
      navigatorEl.appendChild(renderThumbnail(slide, index, state));
    });
  }

  /* ─── Thumbnail construction ────────────────────────────────────────────── */

  function renderThumbnail(slide, index, state) {
    const row = document.createElement('div');
    row.className = 'slide-thumb-row';
    row.dataset.index = String(index);
    row.draggable = true;
    if (index === state.activeSlide) row.classList.add('active');

    const label = document.createElement('div');
    label.className = 'slide-thumb-label';
    label.textContent = String(index + 1);

    const thumb = document.createElement('div');
    thumb.className = 'slide-thumb';

    const miniCanvas = document.createElement('div');
    miniCanvas.className = 'slide-canvas-mini';
    miniCanvas.dataset.slideId = slide.id;
    miniCanvas.dataset.layout  = slide.layout;

    slide.elements.forEach(function (el) {
      miniCanvas.appendChild(renderMiniElement(el));
    });

    thumb.appendChild(miniCanvas);
    row.appendChild(label);
    row.appendChild(thumb);

    row.addEventListener('click',       onClick);
    row.addEventListener('contextmenu', onContextMenu);
    row.addEventListener('dragstart',   onDragStart);
    row.addEventListener('dragover',    onDragOver);
    row.addEventListener('dragleave',   onDragLeave);
    row.addEventListener('drop',        onDrop);
    row.addEventListener('dragend',     onDragEnd);

    return row;
  }

  function renderMiniElement(el) {
    const node = document.createElement('div');
    node.className = 'slide-element';
    node.dataset.type = el.type;

    node.style.position  = 'absolute';
    node.style.left      = el.x + 'px';
    node.style.top       = el.y + 'px';
    node.style.width     = el.width + 'px';
    node.style.minHeight = el.height + 'px';

    const s = el.style || {};
    if (s.fontSize)   node.style.fontSize   = s.fontSize;
    if (s.fontWeight) node.style.fontWeight = s.fontWeight;
    if (s.color)      node.style.color      = s.color;
    if (s.textAlign)  node.style.textAlign  = s.textAlign;

    node.textContent = el.content;
    return node;
  }

  /* ─── Click — jump to slide ─────────────────────────────────────────────── */

  function onClick(e) {
    // Browsers suppress click after a native drop, but be defensive.
    if (dragState.fromIndex !== null) return;
    const index = parseInt(e.currentTarget.dataset.index, 10);
    if (!Number.isFinite(index)) return;
    window.PresentationEditor.setActiveSlide(index);
  }

  /* ─── Context menu ──────────────────────────────────────────────────────── */

  function onContextMenu(e) {
    e.preventDefault();
    const index = parseInt(e.currentTarget.dataset.index, 10);
    if (!Number.isFinite(index)) return;
    openContextMenu(e.clientX, e.clientY, index);
  }

  function openContextMenu(x, y, index) {
    dismissContextMenu();

    const slides    = window.PresentationEditor._state.document.slides;
    const canDelete = slides.length > 1;

    const overlay = document.createElement('div');
    overlay.id = 'dropdown-overlay';
    overlay.addEventListener('mousedown', dismissContextMenu);
    overlay.addEventListener('contextmenu', function (ev) {
      ev.preventDefault();
      dismissContextMenu();
    });

    const menu = document.createElement('div');
    menu.className = 'dropdown slide-context-menu';
    menu.style.left = x + 'px';
    menu.style.top  = y + 'px';

    menu.appendChild(menuButton('Duplicate', function () {
      window.PresentationEditor.duplicateSlide(index);
    }));
    menu.appendChild(menuButton('Delete', function () {
      window.PresentationEditor.deleteSlide(index);
    }, !canDelete));
    menu.appendChild(menuSeparator());
    menu.appendChild(menuButton('New Slide After', function () {
      window.PresentationEditor.addSlideAfter(index);
    }));

    document.body.appendChild(overlay);
    document.body.appendChild(menu);

    // Clamp to viewport once rendered so we can measure.
    const rect = menu.getBoundingClientRect();
    if (rect.right > window.innerWidth) {
      menu.style.left = Math.max(0, window.innerWidth - rect.width - 4) + 'px';
    }
    if (rect.bottom > window.innerHeight) {
      menu.style.top = Math.max(0, window.innerHeight - rect.height - 4) + 'px';
    }

    contextMenuEl    = menu;
    contextOverlayEl = overlay;

    window.addEventListener('keydown', onContextKeyDown);
    window.addEventListener('blur',    dismissContextMenu);
  }

  function dismissContextMenu() {
    if (contextMenuEl && contextMenuEl.parentNode) {
      contextMenuEl.parentNode.removeChild(contextMenuEl);
    }
    if (contextOverlayEl && contextOverlayEl.parentNode) {
      contextOverlayEl.parentNode.removeChild(contextOverlayEl);
    }
    contextMenuEl    = null;
    contextOverlayEl = null;
    window.removeEventListener('keydown', onContextKeyDown);
    window.removeEventListener('blur',    dismissContextMenu);
  }

  function onContextKeyDown(e) {
    if (e.key === 'Escape') {
      e.preventDefault();
      dismissContextMenu();
    }
  }

  function menuButton(label, handler, disabled) {
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.textContent = label;
    if (disabled) {
      btn.disabled = true;
    } else {
      btn.addEventListener('click', function () {
        dismissContextMenu();
        handler();
      });
    }
    return btn;
  }

  function menuSeparator() {
    const sep = document.createElement('div');
    sep.className = 'dropdown-sep';
    return sep;
  }

  /* ─── Drag and drop — native HTML5, no library ──────────────────────────── */

  function onDragStart(e) {
    const row = e.currentTarget;
    dragState.fromIndex = parseInt(row.dataset.index, 10);
    row.classList.add('dragging');
    e.dataTransfer.effectAllowed = 'move';
    // Firefox cancels the drag if no data is set on the transfer.
    try { e.dataTransfer.setData('text/plain', String(dragState.fromIndex)); }
    catch (_) { /* older browsers — ignore */ }

    dismissContextMenu();

    // Blur any focused text box so a pending edit doesn't commit mid-drag.
    const active = document.activeElement;
    if (active && active !== document.body && typeof active.blur === 'function') {
      active.blur();
    }
  }

  function onDragOver(e) {
    if (dragState.fromIndex === null) return;
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
    e.currentTarget.classList.add('dropzone');
  }

  function onDragLeave(e) {
    e.currentTarget.classList.remove('dropzone');
  }

  function onDrop(e) {
    e.preventDefault();
    const toIndex = parseInt(e.currentTarget.dataset.index, 10);
    const from    = dragState.fromIndex;
    if (from !== null && Number.isFinite(toIndex) && from !== toIndex) {
      window.PresentationEditor.reorderSlide(from, toIndex);
    }
    // Cleanup in dragend.
  }

  function onDragEnd() {
    dragState.fromIndex = null;
    if (navigatorEl) {
      navigatorEl.querySelectorAll('.dragging, .dropzone').forEach(function (n) {
        n.classList.remove('dragging');
        n.classList.remove('dropzone');
      });
    }
  }

  /* ─── Export ────────────────────────────────────────────────────────────── */

  window.PresentationNavigator = Object.freeze({
    render,
  });
})();
