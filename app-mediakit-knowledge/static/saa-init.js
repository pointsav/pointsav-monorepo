/**
 * saa-init.js — initialise the SAA editor surface for /edit/{slug}.
 *
 * Phase 2 Step 3 ships the BASE editor: Markdown syntax highlighting +
 * line numbers + history + an explicit save button that POSTs to /edit/{slug}.
 *
 * Phase 2 Step 4 will add SAA squiggles (lint diagnostics).
 * Phase 2 Step 5 will add citation autocomplete on `[`.
 * Phase 2 Step 6 will add the three-keystroke ladder affordances.
 *
 * Bundle: window.CMSAA — built out-of-tree via vendor-js/build.mjs.
 *
 * Editor state injected via window.SAA_INITIAL + window.SAA_SLUG (set by
 * the server template before this script loads). Strings are JSON-encoded
 * so multi-line markdown bodies and special characters round-trip cleanly.
 */

'use strict';

(function () {
  document.addEventListener('DOMContentLoaded', function () {
    var slot = document.getElementById('saa-editor');
    if (!slot || typeof CMSAA === 'undefined') return;
    if (typeof window.SAA_SLUG !== 'string') return;

    var slug = window.SAA_SLUG;
    var initialDoc = (typeof window.SAA_INITIAL === 'string') ? window.SAA_INITIAL : '';

    var view = new CMSAA.view.EditorView({
      doc: initialDoc,
      extensions: [
        CMSAA.commands.history(),
        CMSAA.langMarkdown.markdown(),
        CMSAA.view.lineNumbers(),
        CMSAA.view.EditorView.lineWrapping,
        CMSAA.view.keymap.of(
          CMSAA.commands.defaultKeymap.concat(CMSAA.commands.historyKeymap)
        ),
      ],
      parent: slot,
    });

    var saveBtn = document.getElementById('saa-save');
    var statusEl = document.getElementById('saa-status');

    function setStatus(msg, ok) {
      if (!statusEl) return;
      statusEl.textContent = msg || '';
      statusEl.className = 'saa-status' + (ok === false ? ' saa-status-error' : '');
    }

    if (saveBtn) {
      saveBtn.addEventListener('click', function () {
        saveBtn.disabled = true;
        var prevLabel = saveBtn.textContent;
        saveBtn.textContent = 'Saving…';
        setStatus('');
        var body = view.state.doc.toString();
        fetch('/edit/' + encodeURIComponent(slug), {
          method: 'POST',
          headers: { 'content-type': 'text/plain' },
          body: body,
        })
          .then(function (resp) {
            if (!resp.ok) throw new Error('HTTP ' + resp.status);
            return resp.text();
          })
          .then(function () {
            saveBtn.textContent = prevLabel;
            saveBtn.disabled = false;
            setStatus('Saved at ' + new Date().toLocaleTimeString(), true);
          })
          .catch(function (e) {
            saveBtn.textContent = prevLabel;
            saveBtn.disabled = false;
            setStatus('Save failed: ' + e.message, false);
          });
      });
    }
  });
}());
