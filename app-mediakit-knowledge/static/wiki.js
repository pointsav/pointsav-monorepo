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
   * 3. Page Previews (Hover Cards)                                       *
   * ------------------------------------------------------------------ */
  function initHoverCards() {
    var card = document.createElement('div');
    card.className = 'wiki-hover-card';
    card.style.display = 'none';
    document.body.appendChild(card);

    var currentTarget = null;
    var hideTimeout = null;
    var fetchCache = {};

    var links = document.querySelectorAll('a[data-wikilink="true"]:not(.wiki-redlink)');
    links.forEach(function(link) {
      link.addEventListener('mouseenter', function(e) {
        clearTimeout(hideTimeout);
        var href = link.getAttribute('href');
        if (!href || !href.startsWith('/wiki/')) return;
        var slug = href.substring(6);
        currentTarget = link;

        var rect = link.getBoundingClientRect();
        card.style.left = Math.max(10, rect.left + window.scrollX) + 'px';
        card.style.top = rect.bottom + window.scrollY + 5 + 'px';
        
        if (fetchCache[slug]) {
          renderCard(fetchCache[slug]);
        } else {
          card.innerHTML = '<div class="hover-loading">Loading...</div>';
          card.style.display = 'block';
          
          fetch('/api/preview/' + slug)
            .then(res => res.json())
            .then(data => {
              fetchCache[slug] = data;
              if (currentTarget === link) renderCard(data);
            })
            .catch(() => {
              card.style.display = 'none';
            });
        }
      });

      link.addEventListener('mouseleave', function() {
        hideTimeout = setTimeout(function() {
          card.style.display = 'none';
          currentTarget = null;
        }, 200);
      });
    });

    card.addEventListener('mouseenter', function() {
      clearTimeout(hideTimeout);
    });

    card.addEventListener('mouseleave', function() {
      hideTimeout = setTimeout(function() {
        card.style.display = 'none';
        currentTarget = null;
      }, 200);
    });

    function renderCard(data) {
      var snip = data.snippet || "No summary available.";
      var imgHtml = data.image_url ? '<img src="' + data.image_url + '" alt="">' : '';
      card.innerHTML = imgHtml + '<strong>' + data.title + '</strong><p>' + snip + '</p>';
      card.style.display = 'block';
    }
  }

  /* ------------------------------------------------------------------ *
   * 4. Glossary Tooltips                                                 *
   * ------------------------------------------------------------------ */
  function initGlossaryTooltips() {
    var tooltip = document.createElement('div');
    tooltip.className = 'wiki-glossary-tooltip';
    tooltip.style.display = 'none';
    document.body.appendChild(tooltip);

    var terms = document.querySelectorAll('.wiki-glossary-term');
    terms.forEach(function(term) {
      term.addEventListener('mouseenter', function() {
        var defn = term.getAttribute('title');
        if (!defn) return;
        
        // Temporarily remove title to prevent native tooltip
        term.dataset.title = defn;
        term.removeAttribute('title');
        
        tooltip.textContent = defn;
        tooltip.style.display = 'block';
        
        var rect = term.getBoundingClientRect();
        // Position above the term
        tooltip.style.left = rect.left + window.scrollX + 'px';
        tooltip.style.top = (rect.top + window.scrollY - tooltip.offsetHeight - 5) + 'px';
      });

      term.addEventListener('mouseleave', function() {
        tooltip.style.display = 'none';
        if (term.dataset.title) {
          term.setAttribute('title', term.dataset.title);
        }
      });
    });
  }

  /* ------------------------------------------------------------------ *
   * 4. Mobile nav drawer toggle                                          *
   * ------------------------------------------------------------------ */

  function initMobileNav() {
    var btn     = document.getElementById('nav-toggle');
    var drawer  = document.getElementById('mobile-nav-drawer');
    var overlay = document.getElementById('mobile-nav-overlay');
    var closeBtn = document.getElementById('mobile-nav-close');

    if (!btn || !drawer || !overlay) return;

    function openNav() {
      // Close ToC drawer if open
      document.body.removeAttribute('data-toc-open');
      var tocDrawer = document.getElementById('mobile-toc-drawer');
      var tocBtn    = document.getElementById('toc-toggle-btn');
      if (tocDrawer) tocDrawer.setAttribute('aria-hidden', 'true');
      if (tocBtn)    tocBtn.setAttribute('aria-expanded', 'false');

      document.body.setAttribute('data-nav-open', 'true');
      drawer.removeAttribute('aria-hidden');
      overlay.removeAttribute('aria-hidden');
      btn.setAttribute('aria-expanded', 'true');
    }

    function closeNav() {
      document.body.removeAttribute('data-nav-open');
      drawer.setAttribute('aria-hidden', 'true');
      overlay.setAttribute('aria-hidden', 'true');
      btn.setAttribute('aria-expanded', 'false');
    }

    btn.addEventListener('click', openNav);
    overlay.addEventListener('click', closeNav);
    if (closeBtn) closeBtn.addEventListener('click', closeNav);

    document.addEventListener('keydown', function (e) {
      if (e.key === 'Escape' && document.body.hasAttribute('data-nav-open')) {
        closeNav();
      }
    });
  }

  /* ------------------------------------------------------------------ *
   * 4b. Mobile ToC drawer toggle                                         *
   * ------------------------------------------------------------------ */

  function initTocDrawer() {
    var btn      = document.getElementById('toc-toggle-btn');
    var drawer   = document.getElementById('mobile-toc-drawer');
    var overlay  = document.getElementById('mobile-nav-overlay'); // shared overlay
    var closeBtn = document.getElementById('mobile-toc-close');

    if (!btn || !drawer || !overlay) return;

    function openToc() {
      // Close nav drawer if open
      document.body.removeAttribute('data-nav-open');
      var navDrawer = document.getElementById('mobile-nav-drawer');
      var navBtn    = document.getElementById('nav-toggle');
      if (navDrawer) navDrawer.setAttribute('aria-hidden', 'true');
      if (navBtn)    navBtn.setAttribute('aria-expanded', 'false');

      document.body.setAttribute('data-toc-open', 'true');
      drawer.removeAttribute('aria-hidden');
      overlay.removeAttribute('aria-hidden');
      btn.setAttribute('aria-expanded', 'true');
    }

    function closeToc() {
      document.body.removeAttribute('data-toc-open');
      drawer.setAttribute('aria-hidden', 'true');
      overlay.setAttribute('aria-hidden', 'true');
      btn.setAttribute('aria-expanded', 'false');
    }

    btn.addEventListener('click', openToc);
    overlay.addEventListener('click', closeToc);
    if (closeBtn) closeBtn.addEventListener('click', closeToc);

    document.addEventListener('keydown', function (e) {
      if (e.key === 'Escape' && document.body.hasAttribute('data-toc-open')) {
        closeToc();
      }
    });

    // Close drawer when navigating to a section
    drawer.querySelectorAll('a').forEach(function (link) {
      link.addEventListener('click', closeToc);
    });
  }

  /* ------------------------------------------------------------------ *
   * A2: Footnote hover tooltips                                         *
   * Show the footnote body text on hover over [1] superscript links.   *
   * ------------------------------------------------------------------ */

  function initFootnoteTooltips() {
    var refs = document.querySelectorAll('sup.footnote-ref a');
    if (!refs.length) return;

    var tip = document.createElement('div');
    tip.className = 'fn-tooltip';
    tip.setAttribute('aria-hidden', 'true');
    tip.style.display = 'none';
    document.body.appendChild(tip);

    function getFootnoteText(href) {
      // href is "#fn-1"; the li id is "fn-1"
      var id = href.replace(/^#/, '');
      var li = document.getElementById(id);
      if (!li) return null;
      // Clone so we can strip the backref arrow without mutating the DOM.
      var clone = li.cloneNode(true);
      var backref = clone.querySelector('.footnote-backref');
      if (backref) backref.remove();
      return clone.textContent.trim();
    }

    function showTip(anchor, text) {
      tip.textContent = text;
      tip.style.display = 'block';
      var rect = anchor.getBoundingClientRect();
      var top = window.scrollY + rect.bottom + 6;
      var left = Math.min(window.scrollX + rect.left, window.innerWidth - 400);
      tip.style.top = top + 'px';
      tip.style.left = Math.max(8, left) + 'px';
    }

    refs.forEach(function (anchor) {
      anchor.addEventListener('mouseenter', function () {
        var text = getFootnoteText(anchor.getAttribute('href') || '');
        if (text) showTip(anchor, text);
      });
      anchor.addEventListener('mouseleave', function () {
        tip.style.display = 'none';
      });
      anchor.addEventListener('focus', function () {
        var text = getFootnoteText(anchor.getAttribute('href') || '');
        if (text) showTip(anchor, text);
      });
      anchor.addEventListener('blur', function () {
        tip.style.display = 'none';
      });
    });
  }

  /* ------------------------------------------------------------------ *
   * D2: Search autocomplete                                              *
   * Debounced dropdown fed by GET /api/complete?q={prefix}.             *
   * ------------------------------------------------------------------ */
  function initSearchAutocomplete() {
    var input = document.getElementById('header-search-q');
    var dropdown = document.getElementById('search-autocomplete-dropdown');
    if (!input || !dropdown) return;

    var debounceTimer = null;
    var activeIdx = -1;
    var items = [];

    function hideDropdown() {
      dropdown.style.display = 'none';
      dropdown.innerHTML = '';
      items = [];
      activeIdx = -1;
    }

    function renderDropdown(hits) {
      dropdown.innerHTML = '';
      items = hits;
      activeIdx = -1;
      if (!hits.length) { hideDropdown(); return; }
      hits.forEach(function(hit, i) {
        var li = document.createElement('div');
        li.className = 'ac-item';
        li.textContent = hit.title;
        li.addEventListener('mousedown', function(e) {
          e.preventDefault();
          window.location.href = '/wiki/' + hit.slug;
        });
        dropdown.appendChild(li);
      });
      dropdown.style.display = 'block';
    }

    input.addEventListener('input', function() {
      clearTimeout(debounceTimer);
      var q = input.value.trim();
      if (q.length < 2) { hideDropdown(); return; }
      debounceTimer = setTimeout(function() {
        fetch('/api/complete?q=' + encodeURIComponent(q))
          .then(function(r) { return r.json(); })
          .then(renderDropdown)
          .catch(function() { hideDropdown(); });
      }, 200);
    });

    input.addEventListener('keydown', function(e) {
      if (!items.length) return;
      var rows = dropdown.querySelectorAll('.ac-item');
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        activeIdx = Math.min(activeIdx + 1, rows.length - 1);
        rows.forEach(function(r, i) { r.classList.toggle('ac-active', i === activeIdx); });
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        activeIdx = Math.max(activeIdx - 1, 0);
        rows.forEach(function(r, i) { r.classList.toggle('ac-active', i === activeIdx); });
      } else if (e.key === 'Enter' && activeIdx >= 0) {
        e.preventDefault();
        window.location.href = '/wiki/' + items[activeIdx].slug;
      } else if (e.key === 'Escape') {
        hideDropdown();
      }
    });

    document.addEventListener('click', function(e) {
      if (!dropdown.contains(e.target) && e.target !== input) hideDropdown();
    });
  }

  /* ------------------------------------------------------------------ *
   * B3: Navbox autocollapse                                              *
   * Clicking the navbox title bar toggles visibility of navbox-content. *
   * ------------------------------------------------------------------ */
  function initNavboxes() {
    var navboxes = document.querySelectorAll('.navbox');
    navboxes.forEach(function(nb) {
      var title = nb.querySelector('.navbox-title');
      if (!title) return;
      title.addEventListener('click', function() {
        nb.classList.toggle('navbox-collapsed');
      });
    });
  }

  /* ------------------------------------------------------------------ *
   * Boot                                                                 *
   * ------------------------------------------------------------------ */

  document.addEventListener('DOMContentLoaded', function () {
    initToc();
    initDensityToggle();
    initHoverCards();
    initGlossaryTooltips();
    initMobileNav();
    initTocDrawer();
    initFootnoteTooltips();
    initNavboxes();
    initSearchAutocomplete();
  });

}());
