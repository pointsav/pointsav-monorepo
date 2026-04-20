// Copyright (C) 2026 PointSav Digital Systems
// Licensed under the European Union Public Licence v. 1.2 (EUPL-1.2).
// See LICENCE in the project root or https://eupl.eu/1.2/en/ for the full text.

/* =============================================================================
   schema.js — document model
   =============================================================================
   Defines the in-memory shape of a presentation document and factories for
   creating new documents, slides, and slide elements. No DOM access here; this
   module is pure data. Canvas, navigator, code view, and export all read and
   mutate the structures defined in this file.

   Document shape:
     { version, meta: { created, modified, author }, slides: [Slide] }

   Slide shape:
     { id, layout: 'title'|'content'|'blank', elements: [Element] }

   Element shape (per ROADMAP Phase 2 spec):
     { id, type: 'text'|'image'|'shape', x, y, width, height, content, style }

   Coordinates and sizes are in logical units on a 1100 x 850 canvas
   (US Letter landscape at 100 dpi). See ADR-PR-09.
   ============================================================================= */

(function () {
  'use strict';

  const LAYOUTS = Object.freeze({
    title:   { id: 'title',   name: 'Title'   },
    content: { id: 'content', name: 'Content' },
    blank:   { id: 'blank',   name: 'Blank'   },
  });

  const DEFAULT_LAYOUT = 'blank';

  const CANVAS_LOGICAL_WIDTH  = 1100;
  const CANVAS_LOGICAL_HEIGHT = 850;

  function uid(prefix) {
    if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
      return `${prefix}-${crypto.randomUUID()}`;
    }
    return `${prefix}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
  }

  function newElement(opts) {
    const o = opts || {};
    return {
      id:      uid('el'),
      type:    o.type || 'text',
      x:       typeof o.x === 'number' ? o.x : 0,
      y:       typeof o.y === 'number' ? o.y : 0,
      width:   typeof o.width  === 'number' ? o.width  : 300,
      height:  typeof o.height === 'number' ? o.height : 60,
      content: typeof o.content === 'string' ? o.content : '',
      style:   o.style ? Object.assign({}, o.style) : {},
    };
  }

  function newSlide(opts) {
    const o = opts || {};
    const layout = LAYOUTS[o.layout] ? o.layout : DEFAULT_LAYOUT;
    return {
      id:       uid('sl'),
      layout:   layout,
      elements: [],
    };
  }

  function newDocument() {
    const now = new Date().toISOString();
    return {
      version: 1,
      meta: {
        created:  now,
        modified: now,
        author:   '',
      },
      slides: [newSlide()],
    };
  }

  window.PresentationSchema = Object.freeze({
    LAYOUTS,
    DEFAULT_LAYOUT,
    CANVAS_LOGICAL_WIDTH,
    CANVAS_LOGICAL_HEIGHT,
    newElement,
    newSlide,
    newDocument,
  });
})();
