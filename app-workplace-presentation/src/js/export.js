// Copyright (C) 2026 PointSav Digital Systems
// Licensed under the European Union Public Licence v. 1.2 (EUPL-1.2).
// See LICENCE in the project root or https://eupl.eu/1.2/en/ for the full text.

/* =============================================================================
   export.js — assemble and save the presentation as self-contained .html
   =============================================================================
   Produces a single HTML file that opens standalone in any browser and plays
   as a slideshow using the runtime from slideshow.js. The file is the product
   (CLAUDE.md hard rule 4): no external dependencies, fonts base64-embedded,
   metadata in a <meta> tag, SHA-256 seal covering the body.

   Script placement — intentional divergence from CLAUDE.md's illustrative
   layout. The inlined slideshow runtime lives at the end of <body>, not
   inside <head>, because (a) it queries the DOM on load and (b) this brings
   it inside the SHA-256 seal so tampering is detectable.

   Font embedding — inspects each text element for an explicit fontFamily,
   always includes the default Source Sans 3 when any text is present, and
   embeds only those families from window.WORKPLACE_FONT_DATA. When that
   global is absent (fonts not downloaded yet), no @font-face block is
   emitted and the file falls back to system fonts. A single console.warn
   signals the fallback.

   Save vs Save As — the existing save_file IPC always shows a dialog; there
   is no "write to known path" variant. Both menu entries therefore take the
   same path for Phase 5. See CLEANUP_LOG.md for the path-persistence item.
   ============================================================================= */

(function () {
  'use strict';

  const DEFAULT_FONT_FAMILY = 'Source Sans 3';
  const DEFAULT_FONT_STACK  =
    "'Source Sans 3', -apple-system, 'Helvetica Neue', Arial, sans-serif";

  function escapeHtml(s) {
    return String(s)
      .replace(/&/g,  '&amp;')
      .replace(/</g,  '&lt;')
      .replace(/>/g,  '&gt;')
      .replace(/"/g,  '&quot;')
      .replace(/'/g,  '&#39;');
  }

  function serializeStyle(style) {
    const out = [];
    for (const k in style) {
      if (!Object.prototype.hasOwnProperty.call(style, k)) continue;
      const cssName = k.replace(/([A-Z])/g, '-$1').toLowerCase();
      out.push(cssName + ':' + style[k]);
    }
    return out.join(';');
  }

  function serializeElement(el) {
    const pos = 'left:' + el.x + 'px;top:' + el.y +
                'px;width:' + el.width + 'px;height:' + el.height + 'px;';
    const styleStr = serializeStyle(el.style || {});
    const combined = pos + (styleStr ? styleStr + ';' : '');
    return '<div class="slide-element" data-type="' + escapeHtml(el.type) +
           '" style="' + escapeHtml(combined) + '">' +
           escapeHtml(el.content || '') +
           '</div>';
  }

  function serializeSlide(slide, index) {
    const body = slide.elements.map(serializeElement).join('');
    return '<section class="slide" data-slide="' + (index + 1) +
           '" data-layout="' + escapeHtml(slide.layout) + '">' +
           body +
           '</section>';
  }

  function collectFontFamilies(doc) {
    const set = Object.create(null);
    let hasAnyText = false;
    for (const slide of doc.slides) {
      for (const el of slide.elements) {
        if (el.type !== 'text') continue;
        hasAnyText = true;
        const family = (el.style && el.style.fontFamily) || null;
        if (family) set[family] = true;
      }
    }
    if (hasAnyText) set[DEFAULT_FONT_FAMILY] = true;
    return Object.keys(set);
  }

  function buildFontFaceCSS(doc) {
    const data = window.WORKPLACE_FONT_DATA;
    if (!data) {
      console.warn('[export] window.WORKPLACE_FONT_DATA not defined — ' +
                   'saved file will use system font fallbacks. ' +
                   'Run `make setup` to download and embed fonts.');
      return '';
    }
    const families = collectFontFamilies(doc);
    let css = '';
    for (const family of families) {
      const weights = data[family];
      if (!weights) continue;
      for (const weight in weights) {
        if (!Object.prototype.hasOwnProperty.call(weights, weight)) continue;
        const styles = weights[weight];
        for (const style in styles) {
          if (!Object.prototype.hasOwnProperty.call(styles, style)) continue;
          const b64 = styles[style];
          css += "@font-face{font-family:'" + family + "';" +
                 'font-weight:' + weight + ';' +
                 'font-style:'  + style  + ';' +
                 'src:url(data:font/woff2;base64,' + b64 + ") format('woff2');}";
        }
      }
    }
    return css;
  }

  function buildSlideLayoutCSS() {
    return [
      'html,body{margin:0;padding:0;background:#222;}',
      'body{font-family:' + DEFAULT_FONT_STACK + ';color:#000;}',
      'section.slide{width:1100px;height:850px;position:relative;' +
        'background:#fff;margin:20px auto;' +
        'box-shadow:0 0 20px rgba(0,0,0,0.4);overflow:hidden;' +
        'page-break-after:always;}',
      'section.slide[hidden]{display:none;}',
      '.slide-element{position:absolute;font-size:24pt;line-height:1.25;}',
      '@media print{html,body{background:#fff;}' +
        'section.slide{margin:0;box-shadow:none;}}',
    ].join('\n');
  }

  async function computeSHA256(text) {
    const buf  = new TextEncoder().encode(text);
    const hash = await crypto.subtle.digest('SHA-256', buf);
    const bytes = new Uint8Array(hash);
    let hex = '';
    for (let i = 0; i < bytes.length; i++) {
      const h = bytes[i].toString(16);
      hex += (h.length === 1 ? '0' + h : h);
    }
    return hex;
  }

  function getTitleFromDocument(doc) {
    for (const slide of doc.slides) {
      for (const el of slide.elements) {
        if (el.type === 'text' && el.content) {
          const line = String(el.content).split(/[\r\n]/)[0].trim();
          if (line) return line.substring(0, 120);
        }
      }
    }
    return 'Untitled Presentation';
  }

  function suggestedFilename(doc) {
    const base = (getTitleFromDocument(doc) || 'presentation')
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/^-|-$/g, '');
    return (base || 'presentation') + '.html';
  }

  async function assembleHtml(doc) {
    const title      = getTitleFromDocument(doc);
    const fontCSS    = buildFontFaceCSS(doc);
    const layoutCSS  = buildSlideLayoutCSS();
    const slideshowSource = window.PresentationSlideshow
      ? window.PresentationSlideshow.sourceAsIIFE()
      : '';
    const slidesHtml = doc.slides.map(serializeSlide).join('\n');
    const scriptTag  = '<script>' + slideshowSource + '</script>';
    const bodyInner  = slidesHtml + '\n' + scriptTag;

    const sha = await computeSHA256(bodyInner);

    const meta = {
      author:   (doc.meta && doc.meta.author)   || '',
      created:  (doc.meta && doc.meta.created)  || new Date().toISOString(),
      modified: new Date().toISOString(),
      version:  doc.version || 1,
      sha256:   sha,
    };
    const metaJson = JSON.stringify(meta);

    return '<!DOCTYPE html>\n' +
      '<html lang="en">\n' +
      '<head>\n' +
      '<meta charset="utf-8">\n' +
      '<title>' + escapeHtml(title) + '</title>\n' +
      "<meta name=\"workplace-presentation-document\" content='" +
        escapeHtml(metaJson) + "'>\n" +
      (fontCSS ? '<style>' + fontCSS + '</style>\n' : '') +
      '<style>' + layoutCSS + '</style>\n' +
      '</head>\n' +
      '<body>\n' +
      bodyInner + '\n' +
      '</body>\n' +
      '</html>\n';
  }

  async function save(doc) {
    if (!window.__TAURI__ || !window.__TAURI__.invoke) {
      console.error('[export] Tauri invoke unavailable — cannot save.');
      return null;
    }
    const html = await assembleHtml(doc);
    try {
      const path = await window.__TAURI__.invoke('save_file', {
        content:        html,
        suggested_name: suggestedFilename(doc),
      });
      if (path) {
        console.log('[export] saved:', path);
        if (window.PresentationEditor && window.PresentationEditor._state) {
          window.PresentationEditor._state.dirty = false;
        }
      }
      return path;
    } catch (err) {
      console.error('[export] save failed:', err);
      throw err;
    }
  }

  // Phase 5: Save and Save As both route to save(). The existing save_file IPC
  // always shows a dialog; there is no write-to-known-path variant. See
  // CLEANUP_LOG.md for the follow-up item on path persistence.
  window.PresentationExport = {
    save:         save,
    saveAs:       save,
    assembleHtml: assembleHtml,
  };
})();
