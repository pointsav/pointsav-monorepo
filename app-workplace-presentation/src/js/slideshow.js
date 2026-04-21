// Copyright (C) 2026 PointSav Digital Systems
// Licensed under the European Union Public Licence v. 1.2 (EUPL-1.2).
// See LICENCE in the project root or https://eupl.eu/1.2/en/ for the full text.

/* =============================================================================
   slideshow.js — self-contained slideshow runtime
   =============================================================================
   Two consumers:
     1. export.js inlines this file's runtime body (as an IIFE) into every
        saved .html file, so the artefact plays standalone in any browser
        with no other scripts present.
     2. (Phase 6) editor.js will call PresentationSlideshow.run() when F5
        is pressed to enter fullscreen slideshow mode over the in-memory deck.

   Phase 5 scope per NEXT.md: arrow keys navigate, F toggles fullscreen,
   Escape exits fullscreen. Phase 6 will add PageUp/PageDown/Space, B/W
   screen toggles, and the editor F5 entry point.

   Source-of-truth pattern: the runtime is defined once as a named function.
   sourceAsIIFE() returns its body wrapped as a self-calling block, so the
   exporter never has to duplicate this code. Function.prototype.toString()
   preserves source verbatim on WebKit (the only engine we ship on).
   ============================================================================= */

(function () {
  'use strict';

  function runtime() {
    var slides = document.querySelectorAll('section.slide');
    if (!slides.length) return;

    var current = 0;

    function show(i) {
      current = Math.max(0, Math.min(slides.length - 1, i));
      for (var k = 0; k < slides.length; k++) {
        slides[k].hidden = (k !== current);
      }
    }

    function toggleFullscreen() {
      if (!document.fullscreenElement) {
        if (document.documentElement.requestFullscreen) {
          document.documentElement.requestFullscreen();
        }
      } else if (document.exitFullscreen) {
        document.exitFullscreen();
      }
    }

    document.addEventListener('keydown', function (e) {
      switch (e.key) {
        case 'ArrowRight':
        case 'ArrowDown':
          e.preventDefault();
          show(current + 1);
          break;
        case 'ArrowLeft':
        case 'ArrowUp':
          e.preventDefault();
          show(current - 1);
          break;
        case 'f':
        case 'F':
          e.preventDefault();
          toggleFullscreen();
          break;
        case 'Escape':
          if (document.fullscreenElement && document.exitFullscreen) {
            document.exitFullscreen();
          }
          break;
      }
    });

    show(0);
  }

  window.PresentationSlideshow = {
    run: runtime,
    // Returns the runtime's source text wrapped as a self-executing IIFE,
    // suitable for inlining into the saved .html file's <script> block.
    sourceAsIIFE: function () {
      return '(' + runtime.toString() + ')();';
    },
  };
})();
