/**
 * saa-init.js — initialise the SAA editor surface for /edit/{slug}.
 *
 * Phase 2 Step 3: base editor — Markdown syntax highlighting + line numbers +
 *   history + explicit save button that POSTs to /edit/{slug}.
 * Phase 2 Step 4: SAA squiggles (lint diagnostics from /api/squiggle-rules).
 * Phase 2 Step 5: citation autocomplete on `[` trigger from /api/citations.
 * Phase 2 Step 6: three-keystroke ladder affordances (Tab + Cmd-K) with
 *   one-time toast on 501 — Doorman MCP wiring lands in Phase 4.
 *
 * Bundle: window.CMSAA — built out-of-tree via vendor-js/build.mjs.
 *
 * Editor state injected via window.SAA_INITIAL + window.SAA_SLUG (set by
 * the server template before this script loads). Strings are JSON-encoded
 * so multi-line markdown bodies and special characters round-trip cleanly.
 */

'use strict';

// ── Step 4: SAA squiggle rule set ────────────────────────────────────────────
// Fetched once at editor init from /api/squiggle-rules.  Cached for the
// lifetime of the page.  Each rule has shape
// { id, severity, pattern, flags, message, citation }; severity is
// 'error' | 'warning' | 'info' | 'hint' which CodeMirror lint accepts directly.
// See PHASE-2-PLAN.md §1 Step 4 + UX-DESIGN.md §5.3.
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

// ── Step 5: citation registry ─────────────────────────────────────────────────
// Fetched once at editor init from /api/citations.  Each entry has at minimum
// { id, title } plus optional { url, jurisdiction, entry_type, … }.
// The autocomplete source uses this list when the editor cursor is after `[`.
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

// CodeMirror 6 autocompletion source for citation IDs.
// Triggered when the character immediately before the cursor is `[` or when
// the cursor is inside an already-typed `[prefix` sequence.  Inserts
// `[citation-id]` and moves the cursor past the closing bracket.
function citationCompletionSource(context) {
  // Match `[` followed by zero or more non-`]` non-whitespace chars up to cursor.
  var wordBefore = context.matchBefore(/\[[^\]\s]*/);
  if (!wordBefore) {
    // Also trigger immediately after a bare `[` with nothing typed yet.
    if (context.explicit) {
      wordBefore = { from: context.pos, to: context.pos, text: '' };
    } else {
      // Check whether the character just before cursor is `[`.
      var charBefore = context.pos > 0
        ? context.state.sliceDoc(context.pos - 1, context.pos)
        : '';
      if (charBefore !== '[') return null;
      wordBefore = { from: context.pos, to: context.pos, text: '' };
    }
  }

  if (!citationRegistryFetched || citationRegistry.length === 0) return null;

  // Typed prefix is the text after the opening `[`.
  var openBracketPos = wordBefore.from;
  var prefix = context.state.sliceDoc(openBracketPos, context.pos);
  // Strip leading `[` from prefix if present.
  if (prefix.startsWith('[')) prefix = prefix.slice(1);
  var lowerPrefix = prefix.toLowerCase();

  var options = citationRegistry
    .filter(function (entry) {
      return entry.id.toLowerCase().indexOf(lowerPrefix) !== -1
        || (entry.title && entry.title.toLowerCase().indexOf(lowerPrefix) !== -1);
    })
    .map(function (entry) {
      var infoLines = [entry.title];
      if (entry.jurisdiction) infoLines.push('Jurisdiction: ' + entry.jurisdiction);
      if (entry.entry_type) infoLines.push('Type: ' + entry.entry_type);
      if (entry.url) infoLines.push(entry.url);
      return {
        label: entry.id,
        type: 'keyword',
        info: infoLines.join('\n'),
        // `apply` is called with (view, completion, from, to).
        // We replace from the opening `[` to the current cursor position,
        // inserting `[id]` in full.
        apply: function (view, _completion, _from, _to) {
          var insert = '[' + entry.id + ']';
          var replaceFrom = openBracketPos;
          var replaceTo = context.pos;
          view.dispatch({
            changes: { from: replaceFrom, to: replaceTo, insert: insert },
            selection: { anchor: replaceFrom + insert.length },
          });
        },
      };
    });

  if (options.length === 0) return null;

  // `from` is the position of `[` so CodeMirror knows where the replacement
  // starts.  The completion's `apply` function handles the full insertion.
  return {
    from: openBracketPos,
    options: options,
  };
}

// ── Step 6: Doorman stub toast ────────────────────────────────────────────────
// One-time toast notification shown when the Tab or Cmd-K path returns 501.
// Shown at most once per page load to avoid repetition.
var doormanToastShown = false;

function showDoormanToast() {
  if (doormanToastShown) return;
  doormanToastShown = true;

  var toast = document.createElement('div');
  toast.id = 'saa-doorman-toast';
  toast.setAttribute('role', 'status');
  toast.style.cssText = [
    'position:fixed',
    'bottom:1.5rem',
    'right:1.5rem',
    'background:#1a1a2e',
    'color:#e8e8f0',
    'padding:0.75rem 1.25rem',
    'border-radius:0.375rem',
    'font-size:0.875rem',
    'font-family:inherit',
    'box-shadow:0 4px 12px rgba(0,0,0,0.3)',
    'z-index:9999',
    'max-width:24rem',
    'line-height:1.4',
  ].join(';');
  toast.textContent =
    'Tab completion and Cmd-K instruction activate in Phase 4 (Doorman integration).';
  document.body.appendChild(toast);

  // Auto-dismiss after six seconds.
  setTimeout(function () {
    if (toast.parentNode) toast.parentNode.removeChild(toast);
  }, 6000);
}

// Tab key handler — requests ghost-text completion from /api/doorman/complete.
// On 501 (Phase 2): shows a one-time toast and returns false so the default
// Tab indent behaviour fires.  On 200 (Phase 4+): would render ghost text.
function handleTab(view) {
  // Defer to default indent behaviour when a selection is active or the
  // cursor is at a line start already indented (conventional editor muscle
  // memory for indentation must be preserved).
  var sel = view.state.selection.main;
  if (!sel.empty) return false;
  var line = view.state.doc.lineAt(sel.head);
  var charBeforeCursor = line.text.slice(0, sel.head - line.from);
  if (charBeforeCursor === '' || /^\s+$/.test(charBeforeCursor)) return false;

  // Gather context for the completion request.
  var contextStart = Math.max(0, sel.head - 50);
  var contextText = view.state.doc.sliceString(contextStart, sel.head);

  fetch('/api/doorman/complete', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ context: contextText }),
  })
    .then(function (resp) {
      if (resp.status === 501) {
        showDoormanToast();
      }
      // 200 path (Phase 4): render ghost text overlay — not yet implemented.
    })
    .catch(function (e) {
      console.warn('SAA doorman/complete: fetch failed', e);
    });

  // Return false: do not consume the keypress; let CodeMirror handle indent.
  return false;
}

// Cmd-K handler — opens a native prompt for an AI instruction (Phase 2 form;
// a proper modal with diff overlay lands in Phase 4).
function handleCmdK(view) {
  var sel = view.state.selection.main;
  var selectedText = sel.empty
    ? ''
    : view.state.doc.sliceString(sel.from, sel.to);

  // Use window.prompt as the Phase 2 instruction entry (Phase 4 replaces with
  // a CodeMirror panel modal matching the Cursor Cmd-K UX).
  var instruction = window.prompt(
    'AI instruction' + (selectedText ? ' (applied to selection)' : '') + ':'
  );
  if (!instruction) return true;

  fetch('/api/doorman/instruct', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ selection: selectedText, instruction: instruction }),
  })
    .then(function (resp) {
      if (resp.status === 501) {
        showDoormanToast();
      }
      // 200 path (Phase 4): render diff overlay with accept / reject per hunk.
    })
    .catch(function (e) {
      console.warn('SAA doorman/instruct: fetch failed', e);
    });

  return true;
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

// ── Step 7: collab bundle lazy-load ──────────────────────────────────────────
// When --enable-collab is set on the server, the editor page templates
// `window.WIKI_COLLAB_ENABLED = true`.  We then load cm-collab.bundle.js
// (yjs + y-codemirror.next + y-websocket) on demand so production deploys
// without the flag never pull yjs.
function loadCollabBundle() {
  return new Promise(function (resolve, reject) {
    if (typeof CMCOLLAB !== 'undefined') return resolve();
    var s = document.createElement('script');
    s.src = '/static/vendor/cm-collab.bundle.js';
    s.async = false;
    s.onload = function () { resolve(); };
    s.onerror = function () { reject(new Error('failed to load cm-collab.bundle.js')); };
    document.head.appendChild(s);
  });
}

// Construct the SAA editor — reused by both the collab-on and collab-off
// paths.  The collab branch wires a Y.Doc + WebsocketProvider + the
// y-codemirror.next yCollab() extension at editor-creation time
// (CodeMirror 6 extensions can't be cleanly bolted on after construction).
function buildSaaEditor(slot, slug, initialDoc, collabEnabled) {
  var extensions = [
    CMSAA.commands.history(),
    CMSAA.langMarkdown.markdown(),
    CMSAA.view.lineNumbers(),
    CMSAA.view.EditorView.lineWrapping,
    // Step 4 + Step 6 keymaps: history first, then doorman Tab / Cmd-K,
    // then CodeMirror defaults.  Order matters: first handler that returns
    // true wins.  handleTab intentionally returns false so default indent
    // remains available.
    CMSAA.view.keymap.of(
      CMSAA.commands.historyKeymap.concat([
        { key: 'Tab', run: handleTab },
        { key: 'Mod-k', run: handleCmdK },
      ]).concat(CMSAA.commands.defaultKeymap)
    ),
    CMSAA.lint.lintGutter(),
    CMSAA.lint.linter(makeSquiggleLinter(), { delay: 250 }),
    // Step 5 — citation autocomplete on `[` trigger.
    CMSAA.autocomplete.autocompletion({
      override: [citationCompletionSource],
      activateOnTyping: true,
    }),
  ];

  // Step 7 — collab branch.  Y.Doc + WebsocketProvider + yCollab.  The
  // initial doc seeds Y.Text only when the room is empty (so a second
  // client joining doesn't double-insert the seed).  WebsocketProvider's
  // URL = serverUrl + '/' + room, so serverUrl is the route prefix and
  // room is the slug.
  if (collabEnabled && typeof CMCOLLAB !== 'undefined') {
    var ydoc = new CMCOLLAB.yjs.Doc();
    var protocol = location.protocol === 'https:' ? 'wss:' : 'ws:';
    var wsBase = protocol + '//' + location.host + '/ws/collab';
    var provider = new CMCOLLAB.ywebsocket.WebsocketProvider(wsBase, slug, ydoc);
    var ytext = ydoc.getText('saa');
    provider.on('synced', function (isSynced) {
      if (isSynced && ytext.length === 0 && initialDoc.length > 0) {
        ytext.insert(0, initialDoc);
      }
    });
    extensions.push(CMCOLLAB.ycm.yCollab(ytext, provider.awareness));
  }

  var view = new CMSAA.view.EditorView({
    // When collab is on, leave the initial doc empty — Y.Text seeds it
    // after the synced event so we don't double-insert across clients.
    doc: collabEnabled ? '' : initialDoc,
    extensions: extensions,
    parent: slot,
  });

  // Save button — POSTs the current doc.  Works for both paths; the relay
  // keeps state in-memory only, so save is the only persistence path.
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
}

(function () {
  document.addEventListener('DOMContentLoaded', function () {
    var slot = document.getElementById('saa-editor');
    if (!slot || typeof CMSAA === 'undefined') return;
    if (typeof window.SAA_SLUG !== 'string') return;

    var slug = window.SAA_SLUG;
    var initialDoc = (typeof window.SAA_INITIAL === 'string') ? window.SAA_INITIAL : '';

    // Kick off both fetches in parallel; each extension reads from the
    // module-level cache once its respective fetch resolves.
    fetchSquiggleRules();
    fetchCitationRegistry();

    // Step 7 — collab gate.  When --enable-collab is set on the server,
    // lazy-load cm-collab.bundle.js and construct the editor with the
    // yCollab extension; otherwise build the standard editor.
    if (window.WIKI_COLLAB_ENABLED) {
      loadCollabBundle()
        .then(function () { buildSaaEditor(slot, slug, initialDoc, true); })
        .catch(function (e) {
          console.warn('SAA collab: bundle load failed; starting without collab', e);
          buildSaaEditor(slot, slug, initialDoc, false);
        });
      return;
    }
    buildSaaEditor(slot, slug, initialDoc, false);
  });
}());
