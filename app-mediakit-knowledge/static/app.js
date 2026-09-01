// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

/* ============================================================================
 * app-mediakit-knowledge · Chrome behavior · P2 (winner: "Slate" + grafts)
 * ----------------------------------------------------------------------------
 * Vanilla JS, no framework. Progressive enhancement: the chrome is fully
 * usable without JS; this layer adds theme persistence and the mobile drawer.
 *
 * Provides:
 *   1. Theme toggle          — persists to localStorage['k-theme'] and sets
 *                              <html data-theme="light|dark">.
 *   2. Mobile nav drawer     — open/close, overlay click, Esc, focus trap,
 *                              scroll lock, focus restoration, aria state.
 *   3. Article TOC scroll-spy — initToc() marks the current section's link
 *                              active as the reader scrolls (IntersectionObserver).
 *   4. Search typeahead      — initSearchSuggest() debounces `/api/search-
 *                              suggest`, keyboard-navigable, `/`/Ctrl+K shortcut.
 *   5. initArticleTabs() remains a P3 STUB — its `[data-k-tabs]` mount point
 *      is not emitted by any template yet; wired here for shape only.
 *
 * Selectors match the k-* class/id manifest (deliverable D).
 * ========================================================================== */
(function () {
  "use strict";

  var STORAGE_KEY = "k-theme";
  var THEME_LIGHT = "light";
  var THEME_DARK = "dark";
  var NAV_STORAGE_KEY = "k-nav";
  var SCROLL_LOCK_CLASS = "k-scroll-lock";
  var OPEN_CLASS = "is-open";

  var root = document.documentElement;

  /* ------------------------------------------------------------------------ *
   * Small helpers
   * ------------------------------------------------------------------------ */
  function qs(sel, ctx) { return (ctx || document).querySelector(sel); }
  function qsa(sel, ctx) {
    return Array.prototype.slice.call((ctx || document).querySelectorAll(sel));
  }
  function on(el, type, fn, opts) { if (el) el.addEventListener(type, fn, opts || false); }

  function prefersDark() {
    return window.matchMedia &&
      window.matchMedia("(prefers-color-scheme: dark)").matches;
  }

  function readStoredTheme() {
    try { return window.localStorage.getItem(STORAGE_KEY); }
    catch (e) { return null; }
  }
  function writeStoredTheme(value) {
    try { window.localStorage.setItem(STORAGE_KEY, value); }
    catch (e) { /* private mode / disabled storage — non-fatal */ }
  }

  /* ------------------------------------------------------------------------ *
   * 1. Theme toggle
   * ------------------------------------------------------------------------ *
   * NOTE: to avoid a flash of the wrong theme, the integrator SHOULD also
   * inline a tiny pre-paint snippet in <head> that reads localStorage and
   * sets data-theme before first paint (see doc_head in the layout spec).
   * This module is the interactive half: it reconciles state and wires the
   * toggle button(s).
   */
  function resolveInitialTheme() {
    var stored = readStoredTheme();
    if (stored === THEME_LIGHT || stored === THEME_DARK) return stored;
    return prefersDark() ? THEME_DARK : THEME_LIGHT;
  }

  function applyTheme(theme) {
    root.setAttribute("data-theme", theme);
    var btns = qsa(".k-control--theme");
    btns.forEach(function (btn) {
      var isDark = theme === THEME_DARK;
      btn.setAttribute("aria-pressed", String(isDark));
      var next = isDark ? "light" : "dark";
      btn.setAttribute(
        "aria-label",
        "Switch to " + next + " theme (current: " + theme + ")"
      );
    });
  }

  function initTheme() {
    applyTheme(resolveInitialTheme());

    qsa(".k-control--theme").forEach(function (btn) {
      on(btn, "click", function () {
        var current = root.getAttribute("data-theme") === THEME_DARK
          ? THEME_DARK : THEME_LIGHT;
        var next = current === THEME_DARK ? THEME_LIGHT : THEME_DARK;
        applyTheme(next);
        writeStoredTheme(next);
      });
    });

    /* Follow OS changes only while the user has NOT made an explicit choice. */
    if (window.matchMedia) {
      var mq = window.matchMedia("(prefers-color-scheme: dark)");
      var onOsChange = function (e) {
        if (readStoredTheme()) return; /* explicit choice wins */
        applyTheme(e.matches ? THEME_DARK : THEME_LIGHT);
      };
      if (mq.addEventListener) mq.addEventListener("change", onOsChange);
      else if (mq.addListener) mq.addListener(onOsChange); /* Safari <14 */
    }
  }

  function readStoredNav() {
    try { return window.localStorage.getItem(NAV_STORAGE_KEY); }
    catch (e) { return null; }
  }
  function writeStoredNav(value) {
    try { window.localStorage.setItem(NAV_STORAGE_KEY, value); }
    catch (e) { /* private mode / disabled storage — non-fatal */ }
  }

  /* ------------------------------------------------------------------------ *
   * 1b. Reading-page sidebar collapse (Vector 2022 pattern)
   * ------------------------------------------------------------------------ *
   * Independent of the mobile drawer below — invisible under it (.k-sidebar
   * is already display:none at the drawer breakpoint). Collapsed is the CSS
   * default (.k-sidebar--reading .k-sidenav__browse{display:none} in
   * app.css); a reader who explicitly opens it gets that choice persisted
   * (localStorage 'k-nav') and pre-painted via the <head> guard in
   * doc_head(), mirroring initTheme's fail-open try/catch idiom above. A
   * no-op when the toggle isn't on the page (every page except a reading
   * page with a browse nav to collapse — see sidebar() in layout.rs).
   */
  function initNavCollapse() {
    var toggle = qs(".k-nav-toggle");
    if (!toggle) return;
    var setOpen = function (open) {
      if (open) root.setAttribute("data-nav", "open");
      else root.removeAttribute("data-nav");
      toggle.setAttribute("aria-expanded", open ? "true" : "false");
    };
    setOpen(readStoredNav() === "open");
    on(toggle, "click", function () {
      var open = root.getAttribute("data-nav") === "open";
      setOpen(!open);
      writeStoredNav(!open ? "open" : "collapsed");
    });
  }

  /* ------------------------------------------------------------------------ *
   * 2. Mobile nav drawer + overlay
   * ------------------------------------------------------------------------ */
  var FOCUSABLE =
    'a[href], button:not([disabled]), input:not([disabled]), ' +
    'select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

  function initDrawer() {
    var drawer = qs("#k-nav-drawer");
    var overlay = qs("#k-overlay");
    var openBtn = qs(".k-control--menu");
    var closeBtn = qs(".k-nav-drawer__close");

    if (!drawer || !openBtn) return; /* chrome without a drawer — no-op */

    var lastFocused = null;
    var keydownHandler = null;

    function getFocusable() {
      return qsa(FOCUSABLE, drawer).filter(function (el) {
        return el.offsetParent !== null || el === document.activeElement;
      });
    }

    function trapFocus(e) {
      if (e.key !== "Tab") return;
      var items = getFocusable();
      if (items.length === 0) { e.preventDefault(); return; }
      var first = items[0];
      var last = items[items.length - 1];
      if (e.shiftKey && document.activeElement === first) {
        e.preventDefault(); last.focus();
      } else if (!e.shiftKey && document.activeElement === last) {
        e.preventDefault(); first.focus();
      }
    }

    function onKeydown(e) {
      if (e.key === "Escape" || e.key === "Esc") { closeDrawer(); return; }
      trapFocus(e);
    }

    function openDrawer() {
      if (drawer.classList.contains(OPEN_CLASS)) return;
      lastFocused = document.activeElement;

      drawer.removeAttribute("hidden");
      /* force reflow so the transform transition runs from the hidden state */
      void drawer.offsetWidth;

      drawer.classList.add(OPEN_CLASS);
      if (overlay) { overlay.removeAttribute("hidden"); overlay.classList.add(OPEN_CLASS); }
      document.body.classList.add(SCROLL_LOCK_CLASS);

      openBtn.setAttribute("aria-expanded", "true");
      drawer.setAttribute("aria-hidden", "false");

      keydownHandler = onKeydown;
      document.addEventListener("keydown", keydownHandler);

      var focusables = getFocusable();
      (focusables[0] || closeBtn || drawer).focus();
    }

    function closeDrawer() {
      if (!drawer.classList.contains(OPEN_CLASS)) return;

      drawer.classList.remove(OPEN_CLASS);
      if (overlay) overlay.classList.remove(OPEN_CLASS);
      document.body.classList.remove(SCROLL_LOCK_CLASS);

      openBtn.setAttribute("aria-expanded", "false");
      drawer.setAttribute("aria-hidden", "true");

      if (keydownHandler) {
        document.removeEventListener("keydown", keydownHandler);
        keydownHandler = null;
      }

      /* hide after the transition so it stays out of the a11y/tab tree */
      var hideAfter = function () {
        if (!drawer.classList.contains(OPEN_CLASS)) {
          drawer.setAttribute("hidden", "");
          if (overlay) overlay.setAttribute("hidden", "");
        }
        drawer.removeEventListener("transitionend", hideAfter);
      };
      var reduced = window.matchMedia &&
        window.matchMedia("(prefers-reduced-motion: reduce)").matches;
      if (reduced) hideAfter();
      else drawer.addEventListener("transitionend", hideAfter);

      if (lastFocused && typeof lastFocused.focus === "function") lastFocused.focus();
      lastFocused = null;
    }

    /* initial ARIA + hidden state */
    openBtn.setAttribute("aria-expanded", "false");
    openBtn.setAttribute("aria-controls", "k-nav-drawer");
    drawer.setAttribute("aria-hidden", "true");
    if (!drawer.classList.contains(OPEN_CLASS)) drawer.setAttribute("hidden", "");
    if (overlay && !overlay.classList.contains(OPEN_CLASS)) overlay.setAttribute("hidden", "");

    on(openBtn, "click", function () {
      drawer.classList.contains(OPEN_CLASS) ? closeDrawer() : openDrawer();
    });
    on(closeBtn, "click", closeDrawer);
    on(overlay, "click", closeDrawer);

    /* close after tapping any in-drawer link (SPA-safe no-op otherwise) */
    qsa(".k-nav-link", drawer).forEach(function (link) {
      on(link, "click", closeDrawer);
    });

    /* auto-close if viewport grows past the mobile breakpoint */
    if (window.matchMedia) {
      var desktop = window.matchMedia("(min-width: 769px)");
      var onBreak = function (e) { if (e.matches) closeDrawer(); };
      if (desktop.addEventListener) desktop.addEventListener("change", onBreak);
      else if (desktop.addListener) desktop.addListener(onBreak);
    }
  }

  /* ------------------------------------------------------------------------ *
   * 3. P3 SCAFFOLDS — STUBS ONLY. Do not implement here.
   * ------------------------------------------------------------------------ *
   * These are intentionally inert. P3 ("article surface") wires them to the
   * article DOM. They are declared now so the P2 chrome ships a stable public
   * shape and the integrator can confirm the mount points exist.
   */

  /* Scroll-spy the article TOC (sidebar `.k-toc`, rendered by `toc_nav()` in
   * layout.rs — links already exist, this only tracks reading position).
   * Real UX-review finding: this was a named, already-invoked stub wired to
   * `[data-k-toc]`, an attribute the server never emits — the selector never
   * matched anything, on any page, ever. */
  function initToc() {
    var toc = qs(".k-toc");
    if (!toc) return; /* no TOC on this page (short article, or non-article page) */
    var links = qsa(".k-toc__link", toc);
    if (!links.length || !("IntersectionObserver" in window)) return;

    var headings = [];
    var linkByHref = {};
    links.forEach(function (link) {
      var id = link.getAttribute("href").slice(1);
      var heading = id && document.getElementById(id);
      if (heading) {
        headings.push(heading);
        linkByHref[id] = link;
      }
    });
    if (!headings.length) return;

    var active = null;
    function setActive(id) {
      if (id === active) return;
      if (active && linkByHref[active]) {
        linkByHref[active].classList.remove("k-toc__link--active");
        linkByHref[active].removeAttribute("aria-current");
      }
      active = id;
      if (active && linkByHref[active]) {
        linkByHref[active].classList.add("k-toc__link--active");
        linkByHref[active].setAttribute("aria-current", "location");
      }
    }

    /* A heading is "current" once it crosses the upper third of the
     * viewport (matching the sticky header) and hasn't yet reached the
     * lower third — the same "reading position" window most scroll-spy
     * implementations use, not literal full-element visibility. */
    var observer = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (entry) {
          if (entry.isIntersecting) setActive(entry.target.id);
        });
      },
      { rootMargin: "-15% 0px -70% 0px", threshold: 0 }
    );
    headings.forEach(function (h) { observer.observe(h); });
  }

  /* Search typeahead — `/api/search-suggest?q=`, debounced, keyboard-
   * navigable. Real UX-review finding: search had no suggestions at all, and
   * no `/`/Ctrl+K shortcut to reach it. Wires every `.k-search` block on the
   * page (header + mobile drawer both render one via the same template). */
  var ACTIVE_CLASS = "is-active";

  function initSearchSuggest() {
    qsa(".k-search").forEach(wireOneSearchBlock);
    on(document, "keydown", function (e) {
      var tag = document.activeElement && document.activeElement.tagName;
      var editing = tag === "INPUT" || tag === "TEXTAREA" ||
        (document.activeElement && document.activeElement.isContentEditable);
      var isShortcut = e.key === "/" ? !editing
        : (e.key === "k" || e.key === "K") && (e.metaKey || e.ctrlKey);
      if (!isShortcut) return;
      var input = qs(".k-search__input");
      if (!input) return;
      e.preventDefault();
      input.focus();
      input.select();
    });
  }

  function wireOneSearchBlock(block) {
    var input = qs(".k-search__input", block);
    var list = qs(".k-search__suggestions", block);
    if (!input || !list) return;

    var items = [];      /* current <li> elements, in display order */
    var highlighted = -1;
    var debounceTimer = null;
    var inflight = 0;    /* request-id guard against out-of-order responses */

    function close() {
      list.hidden = true;
      list.innerHTML = "";
      items = [];
      highlighted = -1;
      input.setAttribute("aria-expanded", "false");
      input.removeAttribute("aria-activedescendant");
    }

    function highlight(index) {
      if (items[highlighted]) items[highlighted].classList.remove(ACTIVE_CLASS);
      highlighted = index;
      if (items[highlighted]) {
        items[highlighted].classList.add(ACTIVE_CLASS);
        input.setAttribute("aria-activedescendant", items[highlighted].id);
      } else {
        input.removeAttribute("aria-activedescendant");
      }
    }

    function render(suggestions) {
      list.innerHTML = "";
      items = suggestions.map(function (s, i) {
        var li = document.createElement("li");
        li.id = list.id + "-opt-" + i;
        li.setAttribute("role", "option");
        li.className = "k-search__suggestion";
        var a = document.createElement("a");
        a.href = "/wiki/" + encodeURIComponent(s.slug);
        a.textContent = s.title;   /* textContent — never innerHTML on server data */
        a.tabIndex = -1;
        li.appendChild(a);
        return li;
      });
      items.forEach(function (li) { list.appendChild(li); });
      highlighted = -1;
      list.hidden = items.length === 0;
      input.setAttribute("aria-expanded", String(items.length > 0));
    }

    on(input, "input", function () {
      var q = input.value.trim();
      if (debounceTimer) clearTimeout(debounceTimer);
      if (!q) { close(); return; }
      debounceTimer = setTimeout(function () {
        var requestId = ++inflight;
        fetch("/api/search-suggest?q=" + encodeURIComponent(q))
          .then(function (r) { return r.ok ? r.json() : []; })
          .then(function (suggestions) {
            if (requestId !== inflight) return; /* a newer keystroke already fired */
            render(suggestions || []);
          })
          .catch(function () { /* network hiccup — leave the dropdown as-is */ });
      }, 150);
    });

    on(input, "keydown", function (e) {
      if (list.hidden && e.key !== "Escape") return;
      if (e.key === "ArrowDown") {
        e.preventDefault();
        highlight(Math.min(highlighted + 1, items.length - 1));
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        highlight(Math.max(highlighted - 1, -1));
      } else if (e.key === "Enter" && highlighted >= 0) {
        e.preventDefault();
        var link = items[highlighted].querySelector("a");
        if (link) window.location.href = link.href;
      } else if (e.key === "Escape") {
        close();
      }
    });

    on(document, "click", function (e) {
      if (!block.contains(e.target)) close();
    });
    on(input, "blur", function () {
      /* Deferred so a click on a suggestion (which blurs the input first)
       * still registers before the list disappears. */
      setTimeout(function () {
        if (!block.contains(document.activeElement)) close();
      }, 100);
    });
  }

  /* STUB (P3): activate the article action tabs (Read / Notes / History …).
   * Expected mount: [data-k-tabs]  with [role="tab"] children (P3 markup).
   * P3 responsibilities: roving tabindex, aria-selected, 2px underline active
   * state (no boxed fill), panel show/hide, deep-link via hash. */
  function initArticleTabs() {
    var mount = qs("[data-k-tabs]");
    if (!mount) return; /* no tabs on this page — expected during P2 */
    if (window.console && console.debug) {
      console.debug("[k] initArticleTabs: P3 stub — tab wiring not yet implemented.");
    }
    /* P3: wire keyboard + selection here. */
  }

  /* ------------------------------------------------------------------------ *
   * Article body: copy buttons on code blocks
   * ------------------------------------------------------------------------ *
   * Wraps each `.k-prose pre` in a positioned `.k-codeblock` and adds a Copy
   * button that writes the block's text to the clipboard. Progressive: absent
   * this script the <pre> still renders and scrolls normally.
   */
  function copyText(text) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      return navigator.clipboard.writeText(text);
    }
    return new Promise(function (resolve, reject) {
      try {
        var ta = document.createElement("textarea");
        ta.value = text;
        ta.setAttribute("readonly", "");
        ta.style.position = "absolute";
        ta.style.left = "-9999px";
        document.body.appendChild(ta);
        ta.select();
        document.execCommand("copy");
        document.body.removeChild(ta);
        resolve();
      } catch (e) { reject(e); }
    });
  }

  function initCodeCopy() {
    qsa(".k-prose pre").forEach(function (pre) {
      if (pre.parentNode && pre.parentNode.classList.contains("k-codeblock")) return;
      var wrap = document.createElement("div");
      wrap.className = "k-codeblock";
      pre.parentNode.insertBefore(wrap, pre);
      wrap.appendChild(pre);

      var btn = document.createElement("button");
      btn.type = "button";
      btn.className = "k-codeblock__copy";
      btn.textContent = "Copy";
      btn.setAttribute("aria-label", "Copy code to clipboard");
      wrap.appendChild(btn);

      var resetTimer = null;
      on(btn, "click", function () {
        var code = pre.querySelector("code") || pre;
        copyText(code.innerText).then(function () {
          btn.textContent = "Copied";
          btn.classList.add("is-copied");
          if (resetTimer) clearTimeout(resetTimer);
          resetTimer = setTimeout(function () {
            btn.textContent = "Copy";
            btn.classList.remove("is-copied");
          }, 1800);
        }).catch(function () {
          btn.textContent = "Press ⌘/Ctrl-C";
        });
      });
    });
  }

  /* Wrap wide tables so they scroll horizontally instead of overflowing. */
  function initTables() {
    qsa(".k-prose table").forEach(function (table) {
      if (table.parentNode && table.parentNode.classList.contains("k-table-wrap")) return;
      var wrap = document.createElement("div");
      wrap.className = "k-table-wrap";
      table.parentNode.insertBefore(wrap, table);
      wrap.appendChild(table);
    });
  }

  /* ------------------------------------------------------------------------ *
   * Bootstrap
   * ------------------------------------------------------------------------ */
  function boot() {
    initTheme();
    initNavCollapse();
    initDrawer();
    initCodeCopy();
    initTables();
    initToc();
    initSearchSuggest();
    initArticleTabs();  /* stub */
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", boot);
  } else {
    boot();
  }

  /* Expose a tiny namespace for P3 to hang later modules off, without globals. */
  window.KChrome = window.KChrome || {};
  window.KChrome.applyTheme = applyTheme;
  window.KChrome._stubs = { initToc: initToc, initArticleTabs: initArticleTabs };
})();
