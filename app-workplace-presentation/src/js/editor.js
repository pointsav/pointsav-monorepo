// Copyright (C) 2026 PointSav Digital Systems
// Licensed under the European Union Public Licence v. 1.2 (EUPL-1.2).
// See LICENCE in the project root or https://eupl.eu/1.2/en/ for the full text.

/* =============================================================================
   editor.js — document state, boot, and keyboard wiring
   =============================================================================
   Owns the live document, the active slide index, and the dirty flag. Boots
   on DOMContentLoaded, asks the canvas to render, and dispatches keyboard
   input to the right behaviour.

   Keyboard model (Phase 2):
     - Inside a text box (contenteditable focused):
         Enter        commits and blurs the text box (PowerPoint title-cell
                      behaviour; matches the "Enter twice → new slide"
                      cadence in the ROADMAP)
         Shift+Enter  inserts a line break as usual
         Arrow keys   move the caret as usual
         Escape       blurs the text box
     - Outside any text box:
         Enter        adds a new blank slide after the active one
         ArrowLeft    previous slide
         ArrowRight   next slide
         Escape       no-op (nothing focused to deselect)
     - Always:
         Ctrl+S       logs to console (file write wired in Phase 5)
   ============================================================================= */

(function () {
  'use strict';

  const state = {
    document: null,
    activeSlide: 0,
    dirty: false,
  };

  function start() {
    if (!window.PresentationSchema || !window.PresentationCanvas) {
      console.error('[editor] schema.js and canvas.js must load before editor.js');
      return;
    }
    state.document    = window.PresentationSchema.newDocument();
    state.activeSlide = 0;
    state.dirty       = false;

    renderAll();
    window.addEventListener('keydown', onKeyDown);
  }

  function insertTextBox(x, y) {
    const el = window.PresentationSchema.newElement({
      type:    'text',
      x:       x,
      y:       y,
      width:   300,
      height:  60,
      content: '',
      style:   { fontSize: '24pt' },
    });
    state.document.slides[state.activeSlide].elements.push(el);
    markDirty();
    renderAll();
    // Focus on next frame so the newly rendered node is in the DOM.
    requestAnimationFrame(function () {
      window.PresentationCanvas.focusElement(el.id);
    });
  }

  function addSlideAfterActive() {
    addSlideAfter(state.activeSlide);
  }

  function gotoPreviousSlide() {
    if (state.activeSlide <= 0) return;
    state.activeSlide -= 1;
    renderAll();
  }

  function gotoNextSlide() {
    if (state.activeSlide >= state.document.slides.length - 1) return;
    state.activeSlide += 1;
    renderAll();
  }

  function setActiveSlide(index) {
    const slides = state.document.slides;
    if (index < 0 || index >= slides.length) return;
    if (state.activeSlide === index) return;
    state.activeSlide = index;
    renderAll();
  }

  function addSlideAfter(index) {
    const slides = state.document.slides;
    if (index < 0 || index >= slides.length) return;
    const slide = window.PresentationSchema.newSlide();
    slides.splice(index + 1, 0, slide);
    state.activeSlide = index + 1;
    markDirty();
    renderAll();
  }

  function duplicateSlide(index) {
    const slides = state.document.slides;
    if (index < 0 || index >= slides.length) return;
    const clone = window.PresentationSchema.cloneSlide(slides[index]);
    slides.splice(index + 1, 0, clone);
    state.activeSlide = index + 1;
    markDirty();
    renderAll();
  }

  function deleteSlide(index) {
    const slides = state.document.slides;
    if (slides.length <= 1) return;
    if (index < 0 || index >= slides.length) return;
    slides.splice(index, 1);
    if (state.activeSlide > index) {
      state.activeSlide -= 1;
    } else if (state.activeSlide === index) {
      state.activeSlide = Math.min(state.activeSlide, slides.length - 1);
    }
    markDirty();
    renderAll();
  }

  function reorderSlide(from, to) {
    if (from === to) return;
    const slides = state.document.slides;
    if (from < 0 || from >= slides.length) return;
    if (to   < 0 || to   >= slides.length) return;

    const moved = slides.splice(from, 1)[0];
    slides.splice(to, 0, moved);

    // Keep activeSlide pointing at the same logical slide after the move.
    if (state.activeSlide === from) {
      state.activeSlide = to;
    } else if (from < state.activeSlide && to >= state.activeSlide) {
      state.activeSlide -= 1;
    } else if (from > state.activeSlide && to <= state.activeSlide) {
      state.activeSlide += 1;
    }

    markDirty();
    renderAll();
  }

  function renderAll() {
    window.PresentationCanvas.render(state);
    if (window.PresentationNavigator) {
      window.PresentationNavigator.render(state);
    }
    updateStatusBar();
  }

  function markDirty() {
    state.dirty = true;
  }

  function updateStatusBar() {
    const bar = document.getElementById('statusbar');
    if (!bar) return;
    const counter = bar.querySelector('[data-status="slide-count"]') || bar.firstElementChild;
    if (counter) {
      counter.textContent =
        'Slide ' + (state.activeSlide + 1) + ' of ' + state.document.slides.length;
    }
  }

  function isEditableTarget(target) {
    return !!(target && target.isContentEditable);
  }

  function onKeyDown(e) {
    // Ctrl+S is global — intercept regardless of focus.
    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && e.key.toLowerCase() === 's') {
      e.preventDefault();
      console.log('[Ctrl+S] save-to-disk wired in Phase 5; dirty =', state.dirty);
      return;
    }

    // Ctrl+M — new blank slide after active (Phase 3)
    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && e.key.toLowerCase() === 'm') {
      e.preventDefault();
      addSlideAfterActive();
      return;
    }

    // Ctrl+D — duplicate active slide (Phase 3)
    if ((e.ctrlKey || e.metaKey) && !e.shiftKey && e.key.toLowerCase() === 'd') {
      e.preventDefault();
      duplicateSlide(state.activeSlide);
      return;
    }

    const inEditable = isEditableTarget(e.target);

    if (e.key === 'Escape') {
      if (inEditable && typeof e.target.blur === 'function') {
        e.preventDefault();
        e.target.blur();
      }
      return;
    }

    if (inEditable) {
      // Enter commits (blurs) the text box. Shift+Enter falls through to the
      // default line-break behaviour.
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        e.target.blur();
      }
      return;
    }

    // Outside any editable element.
    switch (e.key) {
      case 'Enter':
        e.preventDefault();
        addSlideAfterActive();
        break;
      case 'ArrowLeft':
        e.preventDefault();
        gotoPreviousSlide();
        break;
      case 'ArrowRight':
        e.preventDefault();
        gotoNextSlide();
        break;
    }
  }

  window.PresentationEditor = {
    start,
    insertTextBox,
    setActiveSlide,
    addSlideAfter,
    duplicateSlide,
    deleteSlide,
    reorderSlide,
    markDirty,
    _state: state,  // read-only peek for debugging; do not mutate from outside
  };

  document.addEventListener('DOMContentLoaded', start);
})();
