// Copyright (C) 2026 PointSav Digital Systems
// Licensed under the European Union Public Licence v. 1.2 (EUPL-1.2).
// See LICENCE in the project root or https://eupl.eu/1.2/en/ for the full text.

/* =============================================================================
   canvas.js — active slide renderer
   =============================================================================
   Renders the active slide into the centre pane (#canvas-area). The slide is
   drawn at its native logical dimensions (1100 x 850) and scaled with a CSS
   transform so the same pixel-coordinate model used in the data structure is
   used at runtime. The wrapper .slide-stage is sized to match the scaled
   canvas so the flex-centred canvas-area letterboxes cleanly.

   Responsibilities:
     - Render the active slide and its elements on each call to render()
     - Resize the canvas on window resize
     - Translate click coordinates from viewport space into logical space,
       then delegate to PresentationEditor.insertTextBox()
     - Commit contenteditable edits back to the element model on blur

   Non-responsibilities:
     - Document state (owned by editor.js)
     - Navigator thumbnails (Phase 3)
     - HTML source pane (Phase 4)
   ============================================================================= */

(function () {
  'use strict';

  const LOGICAL_W = 1100;
  const LOGICAL_H = 850;

  // Elements inserted by a click are sized to this default. Clamp inside the
  // canvas so a click near an edge does not spawn an element that overflows.
  const DEFAULT_TEXT_W = 300;
  const DEFAULT_TEXT_H = 60;

  let areaEl   = null;  // #canvas-area
  let stageEl  = null;  // .slide-stage
  let canvasEl = null;  // .slide-canvas
  let currentState = null;
  let resizeBound  = false;

  function render(state) {
    currentState = state;
    areaEl = document.getElementById('canvas-area');
    if (!areaEl) return;

    areaEl.innerHTML = '';

    stageEl = document.createElement('div');
    stageEl.className = 'slide-stage';

    canvasEl = document.createElement('div');
    canvasEl.className = 'slide-canvas';
    canvasEl.style.width  = LOGICAL_W + 'px';
    canvasEl.style.height = LOGICAL_H + 'px';

    const slide = state.document.slides[state.activeSlide];
    canvasEl.dataset.slideId = slide.id;
    canvasEl.dataset.layout  = slide.layout;

    slide.elements.forEach(function (el) {
      canvasEl.appendChild(renderElement(el));
    });

    canvasEl.addEventListener('mousedown', onCanvasMouseDown);

    stageEl.appendChild(canvasEl);
    areaEl.appendChild(stageEl);

    fit();

    if (!resizeBound) {
      window.addEventListener('resize', fit);
      resizeBound = true;
    }
  }

  function renderElement(el) {
    const node = document.createElement('div');
    node.className = 'slide-element';
    node.dataset.elementId = el.id;
    node.dataset.type      = el.type;
    node.contentEditable   = (el.type === 'text') ? 'true' : 'false';
    node.spellcheck        = false;

    node.style.position   = 'absolute';
    node.style.left       = el.x + 'px';
    node.style.top        = el.y + 'px';
    node.style.width      = el.width + 'px';
    node.style.minHeight  = el.height + 'px';

    const s = el.style || {};
    if (s.fontSize)   node.style.fontSize   = s.fontSize;
    if (s.fontWeight) node.style.fontWeight = s.fontWeight;
    if (s.color)      node.style.color      = s.color;
    if (s.textAlign)  node.style.textAlign  = s.textAlign;

    node.textContent = el.content;
    node.addEventListener('blur', function () { commitElement(el, node); });
    return node;
  }

  function commitElement(el, node) {
    const next = node.innerText.replace(/\n+$/, '');
    if (next !== el.content) {
      el.content = next;
      if (window.PresentationEditor && window.PresentationEditor.markDirty) {
        window.PresentationEditor.markDirty();
      }
    }
  }

  function fit() {
    if (!areaEl || !stageEl || !canvasEl) return;
    const pad = 48;
    const availW = Math.max(0, areaEl.clientWidth  - pad);
    const availH = Math.max(0, areaEl.clientHeight - pad);
    const scale  = Math.min(availW / LOGICAL_W, availH / LOGICAL_H) || 0.001;
    canvasEl.style.transformOrigin = 'top left';
    canvasEl.style.transform       = 'scale(' + scale + ')';
    stageEl.style.width  = (LOGICAL_W * scale) + 'px';
    stageEl.style.height = (LOGICAL_H * scale) + 'px';
    canvasEl.dataset.scale = String(scale);
  }

  function onCanvasMouseDown(e) {
    // Only a click on bare canvas (not on an existing element) inserts a box.
    if (e.target !== canvasEl) return;
    if (!window.PresentationEditor) return;

    const rect  = canvasEl.getBoundingClientRect();
    const scale = parseFloat(canvasEl.dataset.scale) || 1;
    const logicalX = (e.clientX - rect.left) / scale;
    const logicalY = (e.clientY - rect.top)  / scale;

    // Position so the click sits near the top-centre of the new box.
    let x = logicalX - DEFAULT_TEXT_W / 2;
    let y = logicalY - 16;
    x = Math.max(0, Math.min(x, LOGICAL_W - DEFAULT_TEXT_W));
    y = Math.max(0, Math.min(y, LOGICAL_H - DEFAULT_TEXT_H));

    window.PresentationEditor.insertTextBox(x, y);
  }

  function focusElement(elementId) {
    if (!canvasEl) return false;
    const node = canvasEl.querySelector('[data-element-id="' + elementId + '"]');
    if (!node) return false;
    node.focus();
    // Place caret at end.
    const range = document.createRange();
    range.selectNodeContents(node);
    range.collapse(false);
    const sel = window.getSelection();
    sel.removeAllRanges();
    sel.addRange(range);
    return true;
  }

  window.PresentationCanvas = Object.freeze({
    render,
    focusElement,
    LOGICAL_W,
    LOGICAL_H,
  });
})();
