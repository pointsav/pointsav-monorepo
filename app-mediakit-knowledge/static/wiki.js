'use strict';

// ===========================================================================
// 1. TOC SCROLL TRACKING
//    IntersectionObserver on h2/h3 headings — no scroll listener, no jank.
//    Topmost intersecting heading drives the active TOC highlight.
// ===========================================================================

function initTocScrollTracking() {
  const tocLinks = document.querySelectorAll('.wiki-toc-link');
  if (!tocLinks.length) return;
  const headings = Array.from(
    document.querySelectorAll('.wiki-article-body h2[id], .wiki-article-body h3[id]')
  );
  if (!headings.length) return;

  const linkMap = new Map();
  tocLinks.forEach(link => {
    const href = link.getAttribute('href');
    if (href && href.startsWith('#')) linkMap.set(href.slice(1), link);
  });

  let activeId = null;
  const headerH = (document.getElementById('wiki-header') || {}).offsetHeight || 50;

  const observer = new IntersectionObserver(entries => {
    const visible = entries
      .filter(e => e.isIntersecting)
      .sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top);
    if (!visible.length) return;
    const newId = visible[0].target.id;
    if (newId === activeId) return;
    if (activeId && linkMap.has(activeId)) linkMap.get(activeId).classList.remove('is-active');
    activeId = newId;
    if (!linkMap.has(activeId)) return;
    const el = linkMap.get(activeId);
    el.classList.add('is-active');
    const toc = document.getElementById('wiki-toc');
    if (toc) {
      const top = el.offsetTop, h = toc.clientHeight;
      if (top < toc.scrollTop || top > toc.scrollTop + h - 40)
        toc.scrollTo({ top: top - h / 2, behavior: 'smooth' });
    }
  }, { rootMargin: `-${headerH}px 0px -75% 0px`, threshold: 0 });

  headings.forEach(h => observer.observe(h));
}

// Inject per-section [edit] links next to every h2.
// Only runs when the header edit link is present (proxy for editor_enabled).
function injectSectionEditLinks() {
  const article = document.getElementById('wiki-article');
  if (!article || !document.querySelector('.wiki-header-edit')) return;
  const parts = window.location.pathname.split('/').filter(Boolean);
  if (parts.length < 2) return;
  const [category, slug] = parts;
  article.querySelectorAll('.wiki-article-body h2').forEach(h => {
    if (h.id === 'references') return;
    const span = document.createElement('span');
    span.className = 'wiki-section-edit';
    const a = document.createElement('a');
    a.href = `/edit/${category}/${slug}?section=${encodeURIComponent(h.textContent.trim())}`;
    a.textContent = 'edit';
    span.appendChild(a);
    h.insertBefore(span, h.firstChild);
  });
}

// ===========================================================================
// 2. SEARCH AUTOCOMPLETE
//    Debounced fetch to /api/search. Keyboard navigable (↑ ↓ Enter Escape).
// ===========================================================================

function initSearchAutocomplete() {
  const input       = document.getElementById('wiki-search-input');
  const suggestions = document.getElementById('wiki-search-suggestions');
  if (!input || !suggestions) return;

  let timer = null, selIdx = -1, items = [];

  input.addEventListener('input', () => {
    clearTimeout(timer);
    const q = input.value.trim();
    if (q.length < 2) { hide(); return; }
    timer = setTimeout(() =>
      fetch(`/api/search?q=${encodeURIComponent(q)}`)
        .then(r => r.ok ? r.json() : [])
        .then(data => { items = data; selIdx = -1; render(data); })
        .catch(hide)
    , 120);
  });

  input.addEventListener('keydown', e => {
    if (!items.length) return;
    if      (e.key === 'ArrowDown')              { e.preventDefault(); selIdx = Math.min(selIdx + 1, items.length - 1); hl(); }
    else if (e.key === 'ArrowUp')                { e.preventDefault(); selIdx = Math.max(selIdx - 1, -1); hl(); }
    else if (e.key === 'Enter' && selIdx >= 0)   { e.preventDefault(); window.location.href = `/${items[selIdx].slug}`; }
    else if (e.key === 'Escape')                   hide();
  });

  document.addEventListener('click', e => {
    if (!input.contains(e.target) && !suggestions.contains(e.target)) hide();
  });

  function render(data) {
    suggestions.innerHTML = '';
    if (!data.length) { hide(); return; }
    data.forEach((item, i) => {
      const a = document.createElement('a');
      a.className = 'wiki-search-suggestion';
      a.href = `/${item.slug}`;
      a.textContent = item.title || item.slug;
      a.addEventListener('mouseenter', () => { selIdx = i; hl(); });
      suggestions.appendChild(a);
    });
    suggestions.hidden = false;
  }
  function hl()   { suggestions.querySelectorAll('.wiki-search-suggestion')
    .forEach((el, i) => el.setAttribute('aria-selected', i === selIdx ? 'true' : 'false')); }
  function hide() { suggestions.hidden = true; items = []; selIdx = -1; }
}

// ===========================================================================
// 3. EDITOR — live preview, wikilink autocomplete, submit with conflict check
// ===========================================================================

function initEditor() {
  const source = document.getElementById('wiki-editor-source');
  if (!source) return;

  const preview      = document.getElementById('wiki-editor-preview');
  const summaryInput = document.getElementById('wiki-edit-summary');
  const summaryCount = document.getElementById('wiki-summary-counter');
  const submitBtn    = document.getElementById('wiki-submit-btn');
  const spinner      = document.getElementById('wiki-preview-spinner');
  const slug         = document.getElementById('wiki-edit-slug')?.textContent.trim();
  const section      = document.getElementById('wiki-edit-section')?.textContent.trim();
  const baseSha      = document.getElementById('wiki-edit-base-sha')?.textContent.trim();

  // Live preview — debounced 300ms, calls POST /api/preview
  let previewTimer = null;
  source.addEventListener('input', () => {
    clearTimeout(previewTimer);
    if (spinner) spinner.hidden = false;
    previewTimer = setTimeout(() => updatePreview(source.value), 300);
  });

  async function updatePreview(md) {
    try {
      const res = await fetch('/api/preview', {
        method: 'POST',
        headers: { 'Content-Type': 'text/plain' },
        body: md,
      });
      if (preview) preview.innerHTML = res.ok ? await res.text()
        : '<em>[Preview unavailable]</em>';
    } catch (_) {
      if (preview) preview.innerHTML = '<em>[Preview unavailable]</em>';
    }
    if (spinner) spinner.hidden = true;
  }
  if (source.value) updatePreview(source.value);

  // Edit summary validation — submit disabled until ≥ 10 characters
  function validateSummary() {
    const len = summaryInput ? summaryInput.value.trim().length : 0;
    if (summaryCount) summaryCount.textContent = `${len} / 200`;
    if (submitBtn)    submitBtn.disabled = len < 10;
  }
  if (summaryInput) { summaryInput.addEventListener('input', validateSummary); validateSummary(); }

  // Wikilink autocomplete
  initWikilinkAutocomplete(source);

  // Toolbar
  document.querySelectorAll('.wiki-toolbar-btn').forEach(btn => {
    btn.addEventListener('click', () => { applyToolbarAction(source, btn.dataset.action); source.focus(); });
  });

  // Submit
  if (!submitBtn) return;
  submitBtn.addEventListener('click', async () => {
    submitBtn.disabled = true;
    submitBtn.textContent = 'Saving…';
    const body = new URLSearchParams({
      slug:                     slug || '',
      section_heading:          section || '',
      updated_section_markdown: source.value,
      edit_summary:             summaryInput ? summaryInput.value.trim() : '',
      base_sha:                 baseSha || '',
      editor_identity:          '', // server overwrites from MBA auth token
    });
    try {
      const res = await fetch(window.location.pathname, {
        method: 'POST',
        headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
        body: body.toString(),
      });
      if (res.redirected) { window.location.href = res.url; return; }
      const html = await res.text();
      document.open(); document.write(html); document.close();
    } catch (_) {
      submitBtn.disabled = false;
      submitBtn.textContent = 'Save changes';
      alert('Save failed — check your connection and try again.');
    }
  });
}

// Wikilink autocomplete inside the editor textarea
function initWikilinkAutocomplete(ta) {
  const dropdown = document.getElementById('wiki-wikilink-suggestions');
  if (!dropdown) return;
  let active = false, searchStart = -1, selIdx = -1, items = [];

  ta.addEventListener('input', async () => {
    const pos    = ta.selectionStart;
    const before = ta.value.slice(0, pos);
    const last   = before.lastIndexOf('[[');
    if (last === -1 || before.slice(last).includes(']]')) { hide(); return; }
    const partial = before.slice(last + 2);
    if (!partial.length) { hide(); return; }
    searchStart = last; active = true;
    try {
      const res = await fetch(`/api/search?q=${encodeURIComponent(partial)}`);
      items  = res.ok ? await res.json() : [];
      selIdx = -1;
      renderDropdown(ta, items);
    } catch (_) { hide(); }
  });

  ta.addEventListener('keydown', e => {
    if (!active || !items.length) return;
    if      (e.key === 'ArrowDown') { e.preventDefault(); selIdx = Math.min(selIdx + 1, items.length - 1); hlDrop(); }
    else if (e.key === 'ArrowUp')   { e.preventDefault(); selIdx = Math.max(selIdx - 1, -1); hlDrop(); }
    else if (e.key === 'Enter' && selIdx >= 0) { e.preventDefault(); insertLink(ta, items[selIdx]); }
    else if (e.key === 'Escape')    hide();
  });

  function renderDropdown(ta, data) {
    dropdown.innerHTML = '';
    if (!data.length) { hide(); return; }
    const coords = caretCoords(ta, ta.selectionStart);
    const rect   = ta.getBoundingClientRect();
    dropdown.style.top  = `${rect.top  + coords.top  + 20}px`;
    dropdown.style.left = `${rect.left + coords.left}px`;
    data.slice(0, 8).forEach((item, i) => {
      const div = document.createElement('div');
      div.className   = 'wiki-wikilink-suggestion';
      div.textContent = item.title || item.slug;
      div.addEventListener('mousedown', e => { e.preventDefault(); insertLink(ta, item); });
      div.addEventListener('mouseenter', () => { selIdx = i; hlDrop(); });
      dropdown.appendChild(div);
    });
    dropdown.hidden = false;
  }
  function hlDrop() {
    dropdown.querySelectorAll('.wiki-wikilink-suggestion')
      .forEach((el, i) => el.classList.toggle('is-selected', i === selIdx));
  }
  function insertLink(ta, item) {
    const link   = `[[${item.slug}]]`;
    const before = ta.value.slice(0, searchStart);
    const after  = ta.value.slice(ta.selectionStart);
    ta.value     = before + link + after;
    ta.selectionStart = ta.selectionEnd = before.length + link.length;
    ta.dispatchEvent(new Event('input'));
    hide();
  }
  function hide() { active = false; items = []; selIdx = -1; dropdown.hidden = true; }
}

// Toolbar: insert Markdown syntax at cursor
function applyToolbarAction(ta, action) {
  const s   = ta.selectionStart, e = ta.selectionEnd;
  const sel = ta.value.slice(s, e);
  const pre = ta.value.slice(0, s), post = ta.value.slice(e);
  let ins = '', cur = 0;
  switch (action) {
    case 'bold':      ins = `**${sel || 'bold text'}**`;    cur = sel ? ins.length : 2; break;
    case 'italic':    ins = `*${sel  || 'italic text'}*`;   cur = sel ? ins.length : 1; break;
    case 'wikilink':  ins = `[[${sel || 'slug'}]]`;         cur = 2;                    break;
    case 'reference': {
      const n = ((ta.value.match(/\[\^(\d+)\]/g) || [])
        .map(m => parseInt(m.replace(/\D/g,''),10))
        .reduce((a, b) => Math.max(a, b), 0)) + 1;
      ins = `[^${n}]`; cur = ins.length; break;
    }
    case 'heading2': ins = `\n## ${sel || 'Section heading'}\n`; cur = ins.length; break;
    case 'heading3': ins = `\n### ${sel || 'Subsection'}\n`;     cur = ins.length; break;
    case 'table':    ins = `\n| Column 1 | Column 2 |\n|---|---|\n| Cell | Cell |\n`; cur = ins.length; break;
    default: return;
  }
  ta.value = pre + ins + post;
  ta.selectionStart = ta.selectionEnd = s + cur;
  ta.dispatchEvent(new Event('input'));
}

// Approximate caret pixel position inside a textarea (for dropdown placement)
function caretCoords(el, pos) {
  const div = document.createElement('div');
  const cs  = window.getComputedStyle(el);
  Object.assign(div.style, {
    position: 'absolute', visibility: 'hidden', overflow: 'hidden',
    whiteSpace: 'pre-wrap', wordWrap: 'break-word',
    width: cs.width, padding: cs.padding, border: cs.border,
    font: cs.font, lineHeight: cs.lineHeight,
  });
  div.textContent = el.value.substring(0, pos);
  const span = document.createElement('span');
  span.textContent = el.value.substring(pos) || '.';
  div.appendChild(span);
  document.body.appendChild(div);
  const coords = { top: span.offsetTop, left: span.offsetLeft };
  document.body.removeChild(div);
  return coords;
}

// ===========================================================================
// Boot
// ===========================================================================
document.addEventListener('DOMContentLoaded', () => {
  initTocScrollTracking();
  injectSectionEditLinks();
  initSearchAutocomplete();
  initEditor();
});
