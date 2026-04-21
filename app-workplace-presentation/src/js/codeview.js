// Copyright (C) 2026 PointSav Digital Systems
// Licensed under the European Union Public Licence v. 1.2 (EUPL-1.2).
// See LICENCE in the project root or https://eupl.eu/1.2/en/ for the full text.

/* =============================================================================
   codeview.js — split-screen HTML source pane (right pane)
   =============================================================================
   Manages #code-view. When open, shows the active slide's HTML as an editable
   <textarea> in a format that mirrors the Phase 5 export structure. Edits in
   either pane sync to the other on blur (not on keystroke). Invalid HTML shows
   a warning strip and discards the edit; the last valid state is preserved.

   Serialisation format (also the Phase 5 export format):
     <section class="slide" data-layout="blank">
       <div class="slide-element" data-element-id="el-…" data-type="text"
            style="position:absolute;left:…px;top:…px;…">text content</div>
     </section>

   Parsing validates a single <section> root. Any other structure is rejected.

   Responsibilities:
     - toggle()                    open/close the pane; trigger canvas refit
     - render(state)               refresh textarea from model (skips on focus)
     - notifyElementCommit(state)  called by canvas.js after a blur commit

   Non-responsibilities:
     - Document state (editor.js)
     - Canvas rendering (canvas.js)
     - Navigator thumbnails (slides.js)
     - File I/O, save, export
   ============================================================================= */

(function () {
  'use strict';

  var codeViewEl = null;
  var textareaEl = null;
  var warningEl  = null;
  var isOpen     = false;
  var _state     = null;

  /* ─── Toggle ──────────────────────────────────────────────────────────── */

  function toggle() {
    codeViewEl = codeViewEl || document.getElementById('code-view');
    if (!codeViewEl) return;
    isOpen = !isOpen;

    if (isOpen) {
      if (!textareaEl) buildPane();
      codeViewEl.classList.add('open');
      if (_state) updatePane(_state);
    } else {
      codeViewEl.classList.remove('open');
    }

    // canvas.js listens on window resize to refit the scaled canvas.
    window.dispatchEvent(new Event('resize'));
  }

  /* ─── Pane construction ───────────────────────────────────────────────── */

  function buildPane() {
    codeViewEl.innerHTML = '';

    warningEl = document.createElement('div');
    warningEl.className = 'codeview-warning hidden';
    warningEl.textContent = 'Invalid HTML — showing last valid state.';

    textareaEl = document.createElement('textarea');
    textareaEl.className = 'codeview-textarea';
    textareaEl.spellcheck = false;
    textareaEl.setAttribute('autocomplete',   'off');
    textareaEl.setAttribute('autocorrect',    'off');
    textareaEl.setAttribute('autocapitalize', 'off');
    textareaEl.addEventListener('blur', onCodeBlur);

    codeViewEl.appendChild(warningEl);
    codeViewEl.appendChild(textareaEl);
  }

  /* ─── Public render — model → code pane ──────────────────────────────── */

  function render(state) {
    _state = state;
    if (!isOpen || !textareaEl) return;
    if (document.activeElement === textareaEl) return;
    updatePane(state);
  }

  function notifyElementCommit(state) {
    _state = state;
    if (!isOpen || !textareaEl) return;
    if (document.activeElement === textareaEl) return;
    updatePane(state);
  }

  function updatePane(state) {
    var slide = state.document.slides[state.activeSlide];
    textareaEl.value = slideToHtml(slide);
    clearWarning();
  }

  /* ─── Code pane blur — code → model → canvas ──────────────────────────── */

  function onCodeBlur() {
    if (!_state) return;
    var result = parseSlideHtml(textareaEl.value);
    if (!result.valid) {
      showWarning();
      return;
    }
    clearWarning();

    var slide = _state.document.slides[_state.activeSlide];
    slide.elements = result.elements;

    if (window.PresentationEditor) window.PresentationEditor.markDirty();
    if (window.PresentationCanvas)   window.PresentationCanvas.render(_state);
    if (window.PresentationNavigator) window.PresentationNavigator.render(_state);
  }

  /* ─── Serialisation — slide model → HTML string ──────────────────────── */

  function slideToHtml(slide) {
    var inner = slide.elements.map(function (el) {
      return '  <div class="slide-element"'
        + ' data-element-id="' + escAttr(el.id)   + '"'
        + ' data-type="'       + escAttr(el.type)  + '"'
        + ' style="'           + buildInlineStyle(el) + '">'
        + escHtml(el.content)
        + '</div>';
    }).join('\n');

    return '<section class="slide" data-layout="' + escAttr(slide.layout) + '">'
      + (inner ? '\n' + inner + '\n' : '')
      + '</section>';
  }

  function buildInlineStyle(el) {
    var parts = [
      'position:absolute',
      'left:'       + el.x      + 'px',
      'top:'        + el.y      + 'px',
      'width:'      + el.width  + 'px',
      'min-height:' + el.height + 'px',
    ];
    var s = el.style || {};
    if (s.fontSize)   parts.push('font-size:'   + s.fontSize);
    if (s.fontWeight) parts.push('font-weight:' + s.fontWeight);
    if (s.color)      parts.push('color:'       + s.color);
    if (s.textAlign)  parts.push('text-align:'  + s.textAlign);
    return parts.join(';');
  }

  /* ─── Deserialisation — HTML string → element array ─────────────────── */

  function parseSlideHtml(html) {
    var doc;
    try {
      doc = new DOMParser().parseFromString(html, 'text/html');
    } catch (_) {
      return { valid: false };
    }

    // Require exactly one element-node root: the <section>.
    var meaningful = Array.prototype.filter.call(doc.body.childNodes, function (n) {
      return n.nodeType === Node.ELEMENT_NODE
          || (n.nodeType === Node.TEXT_NODE && n.textContent.trim().length > 0);
    });
    if (meaningful.length !== 1 || meaningful[0].tagName !== 'SECTION') {
      return { valid: false };
    }

    var section  = meaningful[0];
    var elNodes  = section.querySelectorAll('.slide-element');
    var elements = [];

    for (var i = 0; i < elNodes.length; i++) {
      var parsed = parseElementNode(elNodes[i]);
      if (!parsed) return { valid: false };
      elements.push(parsed);
    }

    return { valid: true, elements: elements };
  }

  function parseElementNode(node) {
    var id   = (node.dataset.elementId || '').trim();
    var type = (node.dataset.type      || 'text').trim();
    if (!id) return null;

    var cs     = node.style;
    var x      = parseFloat(cs.left)      || 0;
    var y      = parseFloat(cs.top)       || 0;
    var width  = parseFloat(cs.width)     || 300;
    var height = parseFloat(cs.minHeight) || 60;

    var style = {};
    if (cs.fontSize)   style.fontSize   = cs.fontSize;
    if (cs.fontWeight) style.fontWeight = cs.fontWeight;
    if (cs.color)      style.color      = cs.color;
    if (cs.textAlign)  style.textAlign  = cs.textAlign;

    return {
      id:      id,
      type:    type,
      x:       x,
      y:       y,
      width:   width,
      height:  height,
      content: node.textContent || '',
      style:   style,
    };
  }

  /* ─── Warning strip ──────────────────────────────────────────────────── */

  function showWarning() {
    if (warningEl) warningEl.classList.remove('hidden');
  }

  function clearWarning() {
    if (warningEl) warningEl.classList.add('hidden');
  }

  /* ─── Escape helpers ─────────────────────────────────────────────────── */

  function escHtml(s) {
    return String(s)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
  }

  function escAttr(s) {
    return String(s)
      .replace(/&/g, '&amp;')
      .replace(/"/g, '&quot;');
  }

  /* ─── Export ─────────────────────────────────────────────────────────── */

  window.PresentationCodeView = Object.freeze({
    toggle:              toggle,
    render:              render,
    notifyElementCommit: notifyElementCommit,
  });
})();
