// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! The glue from a document to PDF bytes: layout, then one page of content per laid-out page
//! (task F5.5). Fonts are registered from the `Fonts` of the layout, so the faces that measured
//! the text are the faces that draw it.

use crate::content::Content;
use crate::doc::PdfDoc;
use crate::nav::{validate_uri, Bookmark, Dest, Info, Link, LinkTarget, Rect};
use crate::structure::{link_texts, Builder, Mark};
use crate::text::{FontId, TextPolicy};
use moonshot_folio_core::{Emu, FolioError, Ink, Limits, Result};
use moonshot_folio_font::MissingGlyph;
use moonshot_folio_ir::{Document, Meta};
use moonshot_folio_layout::{
    headings, layout, Face, FontSlot, Fonts, HeadingRef, ItemKind, LinkAnnot, NodeId, Options,
    Page, PageTree, RectItem, TextItem, Theme,
};
use std::collections::BTreeMap;

/// Options for [`render_pdf`]. Nothing here is read from a clock or the environment.
#[derive(Clone, Debug)]
pub struct RenderOptions {
    /// Resource limits for layout and the writer.
    pub limits: Limits,
    /// What to do with a character the font cannot draw.
    pub missing_glyph: MissingGlyph,
    /// Creation date `YYYY-MM-DD`; the date of the document metadata when `None`.
    pub creation_date: Option<String>,
    /// Subject for the information dictionary.
    pub subject: Option<String>,
    /// Keywords for the information dictionary.
    pub keywords: Option<String>,
    /// Creating application; `moonshot-folio` when `None`.
    pub creator: Option<String>,
    /// Write the logical structure (tagged PDF): marked content, a structure tree, link
    /// annotations tied to it and the catalog entries assistive technology looks for. On by
    /// default; off gives a plain content stream with no tags at all.
    pub tagged: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            limits: Limits::default(),
            missing_glyph: MissingGlyph::Reject,
            creation_date: None,
            subject: None,
            keywords: None,
            creator: None,
            tagged: true,
        }
    }
}

/// Lay out `doc` and write it as a PDF. Pages, text, rectangles, links, named destinations and
/// an outline built from the headings come from the laid-out tree; the information dictionary
/// comes from the document metadata and `opts`.
///
/// # Errors
/// Whatever layout reports, plus the writer's errors (a bad URI scheme in a link, a font that
/// forbids embedding, limits).
pub fn render_pdf(
    doc: &Document,
    fonts: &Fonts<'_>,
    theme: &Theme,
    opts: &RenderOptions,
) -> Result<Vec<u8>> {
    let lay = Options {
        limits: opts.limits,
        missing_glyph: opts.missing_glyph,
    };
    let tree = layout(doc, fonts, theme, &lay)?;
    render_tree(&tree, &doc.meta, fonts, opts)
}

/// Write an already laid-out tree as a PDF. `fonts` must be the faces the tree was laid out
/// with.
///
/// # Errors
/// The writer's errors; see [`render_pdf`].
pub fn render_tree(
    tree: &PageTree,
    meta: &Meta,
    fonts: &Fonts<'_>,
    opts: &RenderOptions,
) -> Result<Vec<u8>> {
    let mut pdf = PdfDoc::new(opts.limits);
    pdf.set_info(info(meta, opts))?;
    let ids = Faces::register(&mut pdf, fonts)?;
    let policy = match opts.missing_glyph {
        MissingGlyph::Reject => TextPolicy::Reject,
        MissingGlyph::Notdef => TextPolicy::Substitute,
    };
    let mut tags = opts.tagged.then(|| Builder::new(tree));
    let texts = if opts.tagged {
        link_texts(tree)
    } else {
        BTreeMap::new()
    };
    for (pi, page) in tree.pages.iter().enumerate() {
        let content = draw_page(&mut pdf, &ids, (page, pi), policy, tags.as_mut())?;
        let links = page
            .links
            .iter()
            .map(|l| link(l, texts.get(&l.node).cloned()))
            .collect();
        pdf.add_page(page.width, page.height, &content, links)?;
    }
    if let Some(b) = tags {
        pdf.set_structure(b.finish(tree))?;
    }
    for d in &tree.dests {
        let dest = Dest {
            page: d.page,
            x: d.x,
            y: d.y,
        };
        pdf.add_named_dest(&d.name, dest)?;
    }
    let max_title = opts.limits.max_name_bytes().saturating_mul(8);
    pdf.set_outline(outline(&headings(tree), max_title))?;
    let runs = label_runs(tree);
    if runs != [(0, 1)] {
        pdf.set_page_labels(runs)?;
    }
    pdf.build()
}

fn info(meta: &Meta, opts: &RenderOptions) -> Info {
    let some = |s: &str| (!s.is_empty()).then(|| s.to_string());
    Info {
        title: some(&meta.title),
        author: some(&meta.author),
        subject: opts.subject.clone(),
        keywords: opts.keywords.clone(),
        creator: opts.creator.clone(),
        creation_date: opts.creation_date.clone().or_else(|| some(&meta.date)),
        lang: some(&meta.lang),
    }
}

/// The PDF font behind each slot.
struct Faces {
    regular: FontId,
    bold: FontId,
}

/// Same face: equal standard fonts, or TrueType fonts with the same file bytes.
fn same(a: &Face<'_>, b: &Face<'_>) -> bool {
    match (a, b) {
        (Face::Std14(x), Face::Std14(y)) => x == y,
        (Face::TrueType(x), Face::TrueType(y)) => x.data() == y.data(),
        _ => false,
    }
}

fn add_face(pdf: &mut PdfDoc, face: &Face<'_>) -> Result<FontId> {
    match face {
        Face::Std14(f) => Ok(pdf.add_std14(*f)),
        Face::TrueType(f) => pdf.add_truetype(f.data().to_vec()),
    }
}

impl Faces {
    fn register(pdf: &mut PdfDoc, fonts: &Fonts<'_>) -> Result<Self> {
        let (r, b) = (fonts.face(FontSlot::Regular), fonts.face(FontSlot::Bold));
        let regular = add_face(pdf, r)?;
        let bold = if same(r, b) {
            regular
        } else {
            add_face(pdf, b)?
        };
        Ok(Self { regular, bold })
    }

    fn id(&self, slot: FontSlot) -> FontId {
        match slot {
            FontSlot::Regular => self.regular,
            FontSlot::Bold => self.bold,
        }
    }
}

fn link(l: &LinkAnnot, contents: Option<String>) -> Link {
    Link {
        contents,
        rect: Rect {
            x: l.rect.x,
            y: l.rect.y,
            w: l.rect.w,
            h: l.rect.h,
        },
        target: match &l.target {
            moonshot_folio_layout::LinkTarget::Uri(u) => LinkTarget::Uri(u.clone()),
            moonshot_folio_layout::LinkTarget::Named(n) => LinkTarget::Named(n.clone()),
        },
    }
}

/// The content stream of one page: items in paint order. With a structure builder every item is
/// wrapped in marked content: a tagged sequence for logical content, an artifact for the rest.
fn draw_page(
    pdf: &mut PdfDoc,
    ids: &Faces,
    (page, pi): (&Page, usize),
    policy: TextPolicy,
    mut tags: Option<&mut Builder<'_>>,
) -> Result<Content> {
    let mut c = Content::new();
    let mut fill: Option<Ink> = None;
    for it in &page.items {
        let marked = match tags.as_deref_mut() {
            Some(b) => match b.classify(it) {
                Mark::Artifact(kind) => {
                    c.begin_artifact(kind)?;
                    true
                }
                Mark::Tagged(owner, tag) => {
                    let mcid = c.begin_tagged(&tag)?;
                    b.content(owner, pi, mcid);
                    true
                }
            },
            None => false,
        };
        match &it.kind {
            ItemKind::Text(t) => {
                if fill != Some(t.ink) {
                    c.set_fill(t.ink)?;
                    fill = Some(t.ink);
                }
                draw_text(pdf, &mut c, ids, t, (it.x, it.y), policy)?;
            }
            ItemKind::Rect(r) => draw_rect(&mut c, r, (it.x, it.y))?,
        }
        if marked {
            c.end_marked()?;
        }
    }
    Ok(c)
}

fn draw_text(
    pdf: &mut PdfDoc,
    c: &mut Content,
    ids: &Faces,
    t: &TextItem,
    (x, y): (Emu, Emu),
    policy: TextPolicy,
) -> Result<()> {
    let font = ids.id(t.font);
    let run = pdf.encode_text(font, &t.text, policy)?;
    c.text(t.size, x, y, &run)
}

/// A rectangle: filled, stroked, or both, inside its own graphics state.
fn draw_rect(c: &mut Content, r: &RectItem, (x, y): (Emu, Emu)) -> Result<()> {
    if r.fill.is_none() && r.stroke.is_none() {
        return Ok(());
    }
    c.save()?;
    if let Some(f) = r.fill {
        c.set_fill(f)?;
    }
    if let Some((ink, w)) = r.stroke {
        c.set_stroke(ink)?;
        c.set_line_width(w)?;
    }
    c.rect((x, y), (r.w, r.h))?;
    match (r.fill.is_some(), r.stroke.is_some()) {
        (true, true) => c.fill_stroke(),
        (true, false) => c.fill(),
        _ => c.stroke(),
    }
    c.restore()
}

/// The page label runs of a tree: a new run starts where a page's label is not the previous
/// page's label plus one (a section that restarts numbering).
fn label_runs(tree: &PageTree) -> Vec<(usize, u32)> {
    let mut runs: Vec<(usize, u32)> = Vec::new();
    let mut prev: Option<u32> = None;
    for (i, p) in tree.pages.iter().enumerate() {
        if prev.is_none_or(|q| q.checked_add(1) != Some(p.label)) {
            runs.push((i, p.label));
        }
        prev = Some(p.label);
    }
    runs
}

/// `s` cut to at most `max` bytes at a character boundary.
fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end = end.saturating_sub(1);
    }
    s.get(..end).unwrap_or("")
}

/// The bookmark tree of a heading list. A heading nests under the nearest earlier heading of a
/// lower level, so a skipped level (1 then 3) still nests, and a lower level (3 then 1) returns
/// to the top. Titles longer than `max_title` bytes are cut at a character boundary.
fn outline(hs: &[HeadingRef], max_title: usize) -> Vec<Bookmark> {
    let mut i = 0_usize;
    nest(hs, &mut i, 0, max_title)
}

fn nest(hs: &[HeadingRef], i: &mut usize, parent: u8, max_title: usize) -> Vec<Bookmark> {
    let mut out = Vec::new();
    while let Some(h) = hs.get(*i) {
        if h.level <= parent {
            break;
        }
        *i = i.saturating_add(1);
        let children = nest(hs, i, h.level, max_title);
        out.push(Bookmark {
            title: truncate(&h.title, max_title).to_string(),
            dest: Dest {
                page: h.page,
                x: h.x,
                y: h.y,
            },
            children,
        });
    }
    out
}

/// A link the writer would refuse.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BadLink {
    /// Zero-based page index.
    pub page: usize,
    /// The IR node of the link text.
    pub node: NodeId,
    /// The address as written.
    pub target: String,
    /// Why it is refused.
    pub reason: &'static str,
}

/// The external links of `tree` that [`render_tree`] would refuse (a scheme other than http,
/// https or mailto, a space or non-ASCII character, an address that is too long). A failed
/// render reports only the reason; call this to find the links.
#[must_use]
pub fn bad_links(tree: &PageTree) -> Vec<BadLink> {
    let mut out = Vec::new();
    for (pi, page) in tree.pages.iter().enumerate() {
        for l in &page.links {
            let moonshot_folio_layout::LinkTarget::Uri(u) = &l.target else {
                continue;
            };
            if let Err(e) = validate_uri(u) {
                let reason = match e {
                    FolioError::Invalid(m)
                    | FolioError::Limit(m)
                    | FolioError::Malformed(m)
                    | FolioError::Unsupported(m) => m,
                };
                out.push(BadLink {
                    page: pi,
                    node: l.node,
                    target: u.clone(),
                    reason,
                });
            }
        }
    }
    out
}
