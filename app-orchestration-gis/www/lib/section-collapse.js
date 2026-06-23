/* section-collapse.js — Wikipedia-mobile collapsible sections for research pages.
 *
 * Zero dependencies. Auto-wraps each section heading + its following content into a
 * disclosure (button + panel) so that on phones the page reads as a stack of tappable
 * header bars; tapping a header expands its section. On desktop every section is shown
 * and the toggles render as plain headings.
 *
 * Configure per page via <body> data attributes:
 *   data-collapse-root="<css selector>"     container holding the sections (required)
 *   data-collapse-heading="<css selector>"  heading selector (default "h2")
 *
 * Pages with neither attribute are left untouched (safe to include everywhere).
 */
(function () {
  "use strict";

  var MOBILE = window.matchMedia("(max-width: 768px)");
  var body = document.body;
  var rootSel = body.getAttribute("data-collapse-root");
  var headSel = body.getAttribute("data-collapse-heading") || "h2";
  if (!rootSel) return;
  var root = document.querySelector(rootSel);
  if (!root) return;

  // Tag the root so the shared CSS readability/table rules apply.
  root.classList.add("research-mobile");

  var idCounter = 0;
  var sections = []; // { toggle, panel }

  function buildSections() {
    var headings = Array.prototype.slice.call(root.querySelectorAll(":scope > " + headSel));
    // Fall back to descendant headings if none are direct children.
    if (!headings.length) {
      headings = Array.prototype.slice.call(root.querySelectorAll(headSel));
    }
    if (!headings.length) return;

    headings.forEach(function (heading) {
      // Collect siblings after the heading up to the next heading of the same kind.
      var panelNodes = [];
      var n = heading.nextElementSibling;
      while (n && !n.matches(headSel) && !n.classList.contains("sc-toggle")) {
        var next = n.nextElementSibling;
        // Drop redundant <hr> separators that precede the next heading.
        panelNodes.push(n);
        n = next;
      }

      var panelId = "sc-panel-" + (++idCounter);
      var btnId = "sc-btn-" + idCounter;

      // Build the toggle button, preserving the heading's id (for deep links / TOC anchors).
      var toggle = document.createElement("button");
      toggle.type = "button";
      toggle.className = "sc-toggle";
      toggle.id = btnId;
      toggle.setAttribute("aria-controls", panelId);
      if (heading.id) {
        toggle.setAttribute("data-anchor", heading.id);
      }
      var title = document.createElement("span");
      title.className = "sc-title";
      title.innerHTML = heading.innerHTML;
      var chev = document.createElement("span");
      chev.className = "sc-chevron";
      chev.setAttribute("aria-hidden", "true");
      toggle.appendChild(title);
      toggle.appendChild(chev);

      // Preserve a real heading element for the accessibility tree / document outline.
      var headingWrap = document.createElement(heading.tagName);
      if (heading.id) headingWrap.id = heading.id; // keep anchor target on a heading
      headingWrap.className = "sc-heading";
      headingWrap.style.margin = "0";
      headingWrap.appendChild(toggle);

      var panel = document.createElement("div");
      panel.className = "sc-panel";
      panel.id = panelId;
      panel.setAttribute("role", "region");
      panel.setAttribute("aria-labelledby", btnId);
      panelNodes.forEach(function (node) { panel.appendChild(node); });

      heading.parentNode.insertBefore(headingWrap, heading);
      heading.parentNode.insertBefore(panel, headingWrap.nextSibling);
      heading.parentNode.removeChild(heading);

      toggle.addEventListener("click", function () {
        setExpanded(toggle, panel, toggle.getAttribute("aria-expanded") !== "true");
      });

      sections.push({ toggle: toggle, panel: panel });
    });

    // Expand-all / collapse-all control at the top of the root (mobile only via CSS).
    if (sections.length > 1) {
      var ctl = document.createElement("div");
      ctl.className = "sc-allctl";
      var expandBtn = document.createElement("button");
      expandBtn.type = "button";
      expandBtn.textContent = "Expand all";
      expandBtn.addEventListener("click", function () {
        sections.forEach(function (s) { setExpanded(s.toggle, s.panel, true); });
      });
      var collapseBtn = document.createElement("button");
      collapseBtn.type = "button";
      collapseBtn.textContent = "Collapse all";
      collapseBtn.addEventListener("click", function () {
        sections.forEach(function (s) { setExpanded(s.toggle, s.panel, false); });
      });
      ctl.appendChild(expandBtn);
      ctl.appendChild(collapseBtn);
      root.insertBefore(ctl, root.firstChild);
    }
  }

  function setExpanded(toggle, panel, expand) {
    toggle.setAttribute("aria-expanded", expand ? "true" : "false");
    if (expand) {
      panel.hidden = false;
    } else {
      panel.hidden = true;
    }
  }

  // Apply the default open/closed state for the current viewport.
  function applyResponsiveState() {
    var collapse = MOBILE.matches;
    sections.forEach(function (s) {
      setExpanded(s.toggle, s.panel, !collapse);
    });
    // On any viewport, honour a deep-link target.
    openFromHash();
  }

  // Expand the section whose heading/anchor matches the URL hash, and scroll to it.
  function openFromHash() {
    var id = (location.hash || "").slice(1);
    if (!id) return;
    var match = null;
    sections.forEach(function (s) {
      if (s.toggle.getAttribute("data-anchor") === id ||
          s.panel.id === id) {
        match = s;
      }
    });
    if (match) {
      setExpanded(match.toggle, match.panel, true);
      // Defer scroll until layout settles.
      window.requestAnimationFrame(function () {
        match.toggle.scrollIntoView({ block: "start" });
      });
    }
  }

  function init() {
    buildSections();
    if (!sections.length) return;
    applyResponsiveState();
    // Re-apply on viewport crossing the breakpoint (rotate / resize).
    if (MOBILE.addEventListener) {
      MOBILE.addEventListener("change", applyResponsiveState);
    } else if (MOBILE.addListener) {
      MOBILE.addListener(applyResponsiveState); // Safari < 14
    }
    window.addEventListener("hashchange", openFromHash);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
