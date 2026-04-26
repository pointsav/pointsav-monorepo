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

// SAA squiggle rule set — fetched once at editor init from
// /api/squiggle-rules. Cached for the lifetime of the page. Each rule has
// shape { id, severity, pattern, flags, message, citation }; severity is
// 'error' | 'warning' | 'info' | 'hint' which CodeMirror's lint accepts
// directly. See PHASE-2-PLAN.md §1 Step 4 + UX-DESIGN.md §5.3.
var squiggleRules = [];
var squiggleRulesFetched = false;

function fetchSquiggleRules() {
  return fetch('/api/squiggle-rules')
    .then(function (resp) { return resp.ok ? resp.json() : []; })
    .then(function (rules) {
      squiggleRules = rules.map(function (r) {
        try {
          return Object.assign({}, r, { regex: new RegExp(r.pattern, r.flags || 'g') });
        } catch (e) {
          // Invalid regex on the server side — skip the rule rather than
          // crashing the linter. Surface in console for operator visibility.
          console.warn('SAA squiggle: invalid regex for rule', r.id, e);
          return null;
        }
      }).filter(Boolean);
      squiggleRulesFetched = true;
    })
    .catch(function (e) {
      console.warn('SAA squiggle: rule fetch failed', e);
      squiggleRulesFetched = true;
    });
}

// CodeMirror linter source — runs each rule's regex against the editor doc
// and emits Diagnostic[] with severity + message + citation appended.
function makeSquiggleLinter() {
  return function (view) {
    if (!squiggleRulesFetched) return [];
    var doc = view.state.doc.toString();
    var diagnostics = [];
    for (var i = 0; i < squiggleRules.length; i++) {
      var rule = squiggleRules[i];
      var re = rule.regex;
      re.lastIndex = 0;
      var m;
      while ((m = re.exec(doc)) !== null) {
        diagnostics.push({
          from: m.index,
          to: m.index + m[0].length,
          severity: rule.severity,
          message: rule.message + '  [' + rule.citation + ']',
          source: rule.id,
        });
        if (m.index === re.lastIndex) re.lastIndex++;  // avoid infinite loop on zero-width matches
      }
    }
    return diagnostics;
  };
}

(function () {
  document.addEventListener('DOMContentLoaded', function () {
    var slot = document.getElementById('saa-editor');
    if (!slot || typeof CMSAA === 'undefined') return;
    if (typeof window.SAA_SLUG !== 'string') return;

    var slug = window.SAA_SLUG;
    var initialDoc = (typeof window.SAA_INITIAL === 'string') ? window.SAA_INITIAL : '';

    // Kick off rule fetch in parallel; the lint extension reads from the
    // module-level `squiggleRules` once the fetch resolves.
    fetchSquiggleRules();

    var extensions = [
      CMSAA.commands.history(),
      CMSAA.langMarkdown.markdown(),
      CMSAA.view.lineNumbers(),
      CMSAA.view.EditorView.lineWrapping,
      CMSAA.view.keymap.of(
        CMSAA.commands.defaultKeymap.concat(CMSAA.commands.historyKeymap)
      ),
      CMSAA.lint.lintGutter(),
      CMSAA.lint.linter(makeSquiggleLinter(), { delay: 250 }),
    ];

    var view = new CMSAA.view.EditorView({
      doc: initialDoc,
      extensions: extensions,
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
