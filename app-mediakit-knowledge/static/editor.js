/**
 * editor.js — Editor assets for /edit/* routes only (L25).
 *
 * L25: This file must ONLY be loaded on /edit/* routes.
 * Article, home, category, and search pages load ONLY wiki.js.
 * Acceptance: article page HTML contains zero references to editor.js.
 *
 * Contents:
 *   - SAA editor (CodeMirror 6 + squiggle linting + citation autocomplete)
 *   - Merged from: saa-init.js (Phase 3 consolidation)
 *
 * Bundle: window.CMSAA — built out-of-tree via vendor-js/build.mjs.
 * Editor state injected via window.SAA_INITIAL + window.SAA_SLUG.
 */
'use strict';

// SAA squiggle rule set
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

// Citation registry
var citationRegistry = [];
var citationRegistryFetched = false;

function fetchCitationRegistry() {
  return fetch('/api/citations')
    .then(function (resp) { return resp.ok ? resp.json() : []; })
    .then(function (entries) {
      citationRegistry = entries;
      citationRegistryFetched = true;
    })
    .catch(function (e) {
      console.warn('SAA citations: registry fetch failed', e);
      citationRegistryFetched = true;
    });
}

function citationCompletionSource(context) {
  var wordBefore = context.matchBefore(/\[[^\]\s]*/);
  if (!wordBefore) {
    if (context.explicit) {
      wordBefore = { from: context.pos, to: context.pos, text: '' };
    } else {
      var charBefore = context.pos > 0 ? context.state.sliceDoc(context.pos - 1, context.pos) : '';
      if (charBefore !== '[') return null;
      wordBefore = { from: context.pos, to: context.pos, text: '' };
    }
  }
  if (!citationRegistryFetched || citationRegistry.length === 0) return null;
  var openBracketPos = wordBefore.from;
  var prefix = context.state.sliceDoc(openBracketPos, context.pos);
  if (prefix.startsWith('[')) prefix = prefix.slice(1);
  var lowerPrefix = prefix.toLowerCase();
  var options = citationRegistry
    .filter(function (e) { return e.id.toLowerCase().indexOf(lowerPrefix) !== -1 || (e.title && e.title.toLowerCase().indexOf(lowerPrefix) !== -1); })
    .map(function (entry) {
      return {
        label: entry.id, type: 'keyword',
        info: [entry.title, entry.jurisdiction ? 'Jurisdiction: ' + entry.jurisdiction : null].filter(Boolean).join('\n'),
        apply: function (view) {
          var insert = '[' + entry.id + ']';
          view.dispatch({ changes: { from: openBracketPos, to: context.pos, insert: insert }, selection: { anchor: openBracketPos + insert.length } });
        },
      };
    });
  if (options.length === 0) return null;
  return { from: openBracketPos, options: options };
}

var doormanToastShown = false;
function showDoormanToast() {
  if (doormanToastShown) return;
  doormanToastShown = true;
  var toast = document.createElement('div');
  toast.id = 'saa-doorman-toast';
  toast.setAttribute('role', 'status');
  toast.style.cssText = 'position:fixed;bottom:1.5rem;right:1.5rem;background:#1a1a2e;color:#e8e8f0;padding:0.75rem 1.25rem;border-radius:0.375rem;font-size:0.875rem;font-family:inherit;box-shadow:0 4px 12px rgba(0,0,0,0.3);z-index:9999;max-width:24rem;line-height:1.4;';
  toast.textContent = 'Tab completion and Cmd-K instruction activate in Phase 4 (Doorman integration).';
  document.body.appendChild(toast);
  setTimeout(function () { if (toast.parentNode) toast.parentNode.removeChild(toast); }, 6000);
}

function handleTab(view) {
  var sel = view.state.selection.main;
  if (!sel.empty) return false;
  var line = view.state.doc.lineAt(sel.head);
  var charBeforeCursor = line.text.slice(0, sel.head - line.from);
  if (charBeforeCursor === '' || /^\s+$/.test(charBeforeCursor)) return false;
  var contextStart = Math.max(0, sel.head - 50);
  var contextText = view.state.doc.sliceString(contextStart, sel.head);
  fetch('/api/doorman/complete', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ context: contextText }) })
    .then(function (resp) { if (resp.status === 501) showDoormanToast(); })
    .catch(function (e) { console.warn('SAA doorman/complete: fetch failed', e); });
  return false;
}

function handleCmdK(view) {
  var sel = view.state.selection.main;
  var selectedText = sel.empty ? '' : view.state.doc.sliceString(sel.from, sel.to);
  var instruction = window.prompt('AI instruction' + (selectedText ? ' (applied to selection)' : '') + ':');
  if (!instruction) return true;
  fetch('/api/doorman/instruct', { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify({ selection: selectedText, instruction: instruction }) })
    .then(function (resp) { if (resp.status === 501) showDoormanToast(); })
    .catch(function (e) { console.warn('SAA doorman/instruct: fetch failed', e); });
  return true;
}

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
        diagnostics.push({ from: m.index, to: m.index + m[0].length, severity: rule.severity, message: rule.message + '  [' + rule.citation + ']', source: rule.id });
        if (m.index === re.lastIndex) re.lastIndex++;
      }
    }
    return diagnostics;
  };
}

function buildSaaEditor(slot, slug, initialDoc) {
  var extensions = [
    CMSAA.commands.history(),
    CMSAA.langMarkdown.markdown(),
    CMSAA.view.lineNumbers(),
    CMSAA.view.EditorView.lineWrapping,
    CMSAA.view.keymap.of(CMSAA.commands.historyKeymap.concat([
      { key: 'Tab', run: handleTab },
      { key: 'Mod-k', run: handleCmdK },
    ]).concat(CMSAA.commands.defaultKeymap)),
    CMSAA.lint.lintGutter(),
    CMSAA.lint.linter(makeSquiggleLinter(), { delay: 250 }),
    CMSAA.autocomplete.autocompletion({ override: [citationCompletionSource], activateOnTyping: true }),
  ];
  var view = new CMSAA.view.EditorView({ doc: initialDoc, extensions: extensions, parent: slot });
  var saveBtn = document.getElementById('saa-save');
  var statusEl = document.getElementById('saa-status');
  function setStatus(msg, ok) {
    if (!statusEl) return;
    statusEl.textContent = msg || '';
    statusEl.className = 'saa-status' + (ok === false ? ' saa-status-error' : '');
  }
  if (saveBtn) {
    saveBtn.addEventListener('click', function () {
      var body = view.state.doc.toString();
      var userRole = (typeof window.WIKI_USER_ROLE === 'string') ? window.WIKI_USER_ROLE : 'admin';
      var summaryInput = document.getElementById('saa-summary');
      var editSummary = summaryInput ? summaryInput.value.trim() : '';
      if (userRole !== 'admin' && !editSummary) {
        var prompted = window.prompt('Describe your change (required for editors):');
        if (!prompted || !prompted.trim()) { setStatus('Edit summary required.', false); return; }
        editSummary = prompted.trim();
      }
      saveBtn.disabled = true;
      var prevLabel = saveBtn.textContent;
      saveBtn.textContent = 'Saving…';
      setStatus('');
      fetch('/edit/' + encodeURIComponent(slug), {
        method: 'POST', headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ body: body, edit_summary: editSummary }),
      }).then(function (resp) {
        if (resp.status === 202) { saveBtn.textContent = prevLabel; saveBtn.disabled = false; setStatus('Your edit has been submitted for review.', true); return; }
        if (!resp.ok) throw new Error('HTTP ' + resp.status);
        return resp.text().then(function () { saveBtn.textContent = prevLabel; saveBtn.disabled = false; setStatus('Saved at ' + new Date().toLocaleTimeString(), true); });
      }).catch(function (e) { saveBtn.textContent = prevLabel; saveBtn.disabled = false; setStatus('Save failed: ' + e.message, false); });
    });
  }
}

(function () {
  document.addEventListener('DOMContentLoaded', function () {
    var slot = document.getElementById('saa-editor');
    if (!slot || typeof CMSAA === 'undefined') return;
    if (typeof window.SAA_SLUG !== 'string') return;
    fetchSquiggleRules();
    fetchCitationRegistry();
    buildSaaEditor(slot, window.SAA_SLUG, (typeof window.SAA_INITIAL === 'string') ? window.SAA_INITIAL : '');
  });
}());
