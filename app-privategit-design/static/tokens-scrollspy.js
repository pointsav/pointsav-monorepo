// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

// /tokens sidebar scroll-spy. Guards on DOM shape (.tg-group[id] present), not the
// URL, so this is a safe no-op everywhere except /tokens and can load globally via
// <script defer> alongside the other static scripts.
//
// root MUST be .main, not the default viewport -- .main is the real scrolling
// container (independent overflow-y:auto column, see the .layout/.sidebar/.main
// comment in portal.css), not window. Missing this makes the observer never fire.
(function () {
  var groups = document.querySelectorAll('.tg-group[id]');
  if (!groups.length) return;
  var mainEl = document.querySelector('.main');
  if (!mainEl) return;
  var sidebar = document.querySelector('nav.sidebar');
  if (!sidebar) return;

  // A single lookup pool, not two separate NodeLists keyed by class. A tier with
  // exactly one group renders as ONE combined link straight to that group -- see
  // nav-tokens.html -- so its href is "#tier-group", not "#tier"; a
  // groupLinks-only lookup for "tier-group" would miss it entirely (that link
  // lives in .doc-toc__group-heading, not .doc-toc__group-list, in the collapsed
  // case). Searching every in-page anchor link by href sidesteps the distinction.
  var allLinks = sidebar.querySelectorAll('a[href^="#"]');
  var activeGroupLink = null;

  function linkFor(id) {
    var href = '#' + id;
    for (var i = 0; i < allLinks.length; i++) {
      if (allLinks[i].getAttribute('href') === href) return allLinks[i];
    }
    return null;
  }

  // Marks only the specific group/leaf link active -- never the tier heading.
  // Matches /components' nav.html, which only ever sets .active on the leaf
  // link (see templates/nav.html). An earlier version of this function also
  // marked the enclosing tier heading active as a "breadcrumb" -- removed
  // 2026-08-05 per operator parity request: /tokens and /components now use
  // exactly one marker, never two simultaneously.
  function setActive(groupEl) {
    var link = linkFor(groupEl.id);
    if (!link || link === activeGroupLink) return;
    if (activeGroupLink) {
      activeGroupLink.classList.remove('active');
      activeGroupLink.removeAttribute('aria-current');
    }
    link.classList.add('active');
    link.setAttribute('aria-current', 'page');
    activeGroupLink = link;
  }

  // Multiple groups can sit inside the observation band at once (short groups);
  // track everything currently visible and always promote the topmost one, rather
  // than reacting to individual isIntersecting flips in isolation.
  var visible = [];

  var observer = new IntersectionObserver(function (entries) {
    entries.forEach(function (entry) {
      var idx = visible.indexOf(entry.target);
      if (entry.isIntersecting) {
        if (idx === -1) visible.push(entry.target);
      } else if (idx !== -1) {
        visible.splice(idx, 1);
      }
    });
    if (!visible.length) return;
    var top = visible[0];
    visible.forEach(function (el) {
      if (el.getBoundingClientRect().top < top.getBoundingClientRect().top) top = el;
    });
    setActive(top);
  }, {
    root: mainEl,
    rootMargin: '-10% 0px -75% 0px',
    threshold: 0,
  });

  groups.forEach(function (g) {
    observer.observe(g);
  });

  // Top/bottom-of-scroll fallbacks. The rootMargin band sits in the top quarter of
  // the viewport, so (a) at scrollTop 0 the very first group doesn't always cross
  // that band yet -- nothing observer-driven is active on initial load -- and (b)
  // once the last group's top has scrolled past the band with nothing below it to
  // push it back down, it never intersects again and nothing stays active for the
  // rest of the scroll. Force the first/last group active whenever the container
  // sits at (or within a few pixels of) either scroll extreme.
  var firstGroup = groups[0];
  var lastGroup = groups[groups.length - 1];
  function checkEdges() {
    if (mainEl.scrollTop <= 2) {
      setActive(firstGroup);
    } else if (mainEl.scrollTop + mainEl.clientHeight >= mainEl.scrollHeight - 2) {
      setActive(lastGroup);
    }
  }
  mainEl.addEventListener('scroll', checkEdges, { passive: true });
  checkEdges();
})();
