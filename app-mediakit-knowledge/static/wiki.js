/**
 * wiki.js — Phase 1.1 minimal interactivity.
 *
 * Two responsibilities only:
 *   1. TOC collapse toggle (Vector 2022 sticky TOC pattern).
 *   2. Reader density preference (Off / Exceptions only / All),
 *      persisted to localStorage. No machinery honours the setting
 *      until Phase 7; this script maintains the UI state only.
 *
 * No external dependencies. No module bundler. Loaded with `defer`
 * so HTML renders without it. Progressively enhances — if this script
 * fails to load, the page remains fully readable.
 *
 * Bloomberg article standard: plain English, no marketing copy.
 */

'use strict';

(function () {

  /* ------------------------------------------------------------------ *
   * 1. TOC collapse toggle                                               *
   * ------------------------------------------------------------------ */

  var STORAGE_KEY_TOC = 'wiki-toc-expanded';

  function initToc() {
    var toc     = document.getElementById('wiki-toc');
    var toggle  = document.getElementById('toc-toggle');
    var list    = document.getElementById('toc-list');

    if (!toc || !toggle || !list) return;

    // Restore saved state (default: expanded).
    var saved = localStorage.getItem(STORAGE_KEY_TOC);
    var expanded = saved === null ? true : saved === 'true';
    applyTocState(toc, toggle, expanded);

    toggle.addEventListener('click', function () {
      expanded = !expanded;
      localStorage.setItem(STORAGE_KEY_TOC, String(expanded));
      applyTocState(toc, toggle, expanded);
    });
  }

  function applyTocState(toc, toggle, expanded) {
    toggle.setAttribute('aria-expanded', String(expanded));
    toggle.textContent = expanded ? '[hide]' : '[show]';
    if (expanded) {
      toc.classList.remove('toc-collapsed');
    } else {
      toc.classList.add('toc-collapsed');
    }
  }

  /* ------------------------------------------------------------------ *
   * 2. Reader density toggle                                             *
   *                                                                     *
   * Three states:                                                        *
   *   off        — no IVC marks (pure reading experience)               *
   *   exceptions — neutral marks visible; coloured marks prominent      *
   *                (default; operationalises the TLS-padlock lesson)    *
   *   all        — shows verified marks too; for auditors, power-users  *
   *                                                                     *
   * The setting is stored as a string in localStorage. No CSS class     *
   * is applied to the body yet because no IVC marks exist in Phase 1.1. *
   * Phase 7 will read this key and apply the appropriate class.         *
   * ------------------------------------------------------------------ */

  var STORAGE_KEY_DENSITY = 'wiki-citation-density';
  var DENSITY_DEFAULT     = 'exceptions';

  function initDensityToggle() {
    var btnOff        = document.getElementById('density-off');
    var btnExceptions = document.getElementById('density-exceptions');
    var btnAll        = document.getElementById('density-all');

    if (!btnOff || !btnExceptions || !btnAll) return;

    var current = localStorage.getItem(STORAGE_KEY_DENSITY) || DENSITY_DEFAULT;
    applyDensity(current, btnOff, btnExceptions, btnAll);

    btnOff.addEventListener('click', function () {
      setDensity('off', btnOff, btnExceptions, btnAll);
    });
    btnExceptions.addEventListener('click', function () {
      setDensity('exceptions', btnOff, btnExceptions, btnAll);
    });
    btnAll.addEventListener('click', function () {
      setDensity('all', btnOff, btnExceptions, btnAll);
    });
  }

  function setDensity(value, btnOff, btnExceptions, btnAll) {
    localStorage.setItem(STORAGE_KEY_DENSITY, value);
    applyDensity(value, btnOff, btnExceptions, btnAll);
    // Phase 7: document.body.dataset.citationDensity = value;
  }

  function applyDensity(value, btnOff, btnExceptions, btnAll) {
    var ACTIVE = 'density-btn-active';
    btnOff.classList.toggle(ACTIVE, value === 'off');
    btnExceptions.classList.toggle(ACTIVE, value === 'exceptions');
    btnAll.classList.toggle(ACTIVE, value === 'all');
  }

  /* ------------------------------------------------------------------ *
   * Boot                                                                 *
   * ------------------------------------------------------------------ */

  document.addEventListener('DOMContentLoaded', function () {
    initToc();
    initDensityToggle();
  });

}());
